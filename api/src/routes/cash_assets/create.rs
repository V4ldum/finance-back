use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::{AssetName, AssetPossessed, AssetUnitValue, CreateCashAsset};
use crate::middleware::AuthenticatedUserId;

/***** REQUEST *****/

#[derive(Deserialize)]
pub(crate) struct CreateCashAssetRequest {
    name: String,
    possessed: i64,
    unit_value: i64,
}

impl TryFrom<CreateCashAssetRequest> for CreateCashAsset {
    type Error = String;

    fn try_from(value: CreateCashAssetRequest) -> Result<CreateCashAsset, String> {
        let name = AssetName::parse(value.name)?;
        let possessed = AssetPossessed::parse(value.possessed)?;
        let unit_value = AssetUnitValue::parse(value.unit_value)?;

        Ok(Self {
            name,
            possessed,
            unit_value,
        })
    }
}

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        user_id = %user.id(),
        name = %request.name,
        possessed = %request.possessed,
        unit_value = %request.unit_value
    ),
    err(Debug)
)]
pub(crate) async fn create_cash_asset(
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateCashAssetRequest>,
) -> Result<StatusCode, CreateCashAssetError> {
    let create_cash_asset: CreateCashAsset = request.try_into().map_err(CreateCashAssetError::ValidationError)?;

    insert_cash_asset(&pool, user.id(), &create_cash_asset)
        .await
        .context("Failed to insert cash asset")?;

    Ok(StatusCode::CREATED)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn insert_cash_asset(pool: &SqlitePool, user_id: i64, cash_asset: &CreateCashAsset) -> Result<()> {
    let cash_asset_name = cash_asset.name.as_ref();
    let cash_asset_possessed = cash_asset.possessed.as_ref();
    let cash_asset_unit_value = cash_asset.unit_value.as_ref();

    sqlx::query!(
        r#"
            INSERT INTO cash_assets (name, possessed, unit_value, user_id)
            VALUES ($1, $2, $3, $4)
        "#,
        cash_asset_name,
        cash_asset_possessed,
        cash_asset_unit_value,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum CreateCashAssetError {
    #[error("{0}")]
    #[status(BAD_REQUEST)]
    ValidationError(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
