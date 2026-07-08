use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::{AssetName, AssetPossessed, AssetUnitValue, UpdateCashAsset};
use crate::middleware::AuthenticatedUserId;
use crate::model::cash_asset::CashAsset;
use crate::routes::cash_assets::query_cash_asset;

/***** REQUEST *****/

#[derive(Deserialize)]
pub(crate) struct UpdateCashAssetRequest {
    name: Option<String>,
    possessed: Option<i64>,
    unit_value: Option<i64>,
}

impl TryFrom<UpdateCashAssetRequest> for UpdateCashAsset {
    type Error = String;

    fn try_from(value: UpdateCashAssetRequest) -> Result<Self, String> {
        let name = value.name.map(AssetName::parse).transpose()?;
        let possessed = value.possessed.map(AssetPossessed::parse).transpose()?;
        let unit_value = value.unit_value.map(AssetUnitValue::parse).transpose()?;

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
        id = %id,
        user_id = %user.id(),
        name = ?request.name,
        possessed = ?request.possessed,
        unit_value = ?request.unit_value,
    ),
    err(Debug)
)]
pub(crate) async fn update_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateCashAssetRequest>,
) -> Result<StatusCode, UpdateCashAssetError> {
    let update_cash_asset: UpdateCashAsset = request.try_into().map_err(UpdateCashAssetError::ValidationError)?;

    let asset = query_cash_asset(&pool, id, user.id())
        .await
        .context("Failed to fetch cash asset")?
        .ok_or(UpdateCashAssetError::InvalidId)?;

    // Only write if a provided field actually differs from the stored value
    if has_changes(&update_cash_asset, &asset) {
        update_cash_asset_(&pool, user.id(), id, &update_cash_asset)
            .await
            .context("Failed to update cash asset")?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/***** HELPERS *****/
fn has_changes(update: &UpdateCashAsset, current: &CashAsset) -> bool {
    update.name.as_ref().is_some_and(|v| v.as_ref() != current.name)
        || update
            .possessed
            .as_ref()
            .is_some_and(|v| *v.as_ref() != current.possessed)
        || update
            .unit_value
            .as_ref()
            .is_some_and(|v| *v.as_ref() != current.unit_value)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn update_cash_asset_(
    pool: &SqlitePool,
    user_id: i64,
    asset_id: i64,
    cash_asset: &UpdateCashAsset,
) -> Result<()> {
    let cash_asset_name = cash_asset.name.as_ref().map(|n| n.as_ref());
    let cash_asset_possessed = cash_asset.possessed.as_ref().map(|p| p.as_ref());
    let cash_asset_unit_value = cash_asset.unit_value.as_ref().map(|v| v.as_ref());

    // COALESCE writes the first non-null argument in the pair
    sqlx::query!(
        r#"
            UPDATE cash_assets
            SET name = COALESCE($1, name),
                possessed = COALESCE($2, possessed),
                unit_value = COALESCE($3, unit_value)
            WHERE id = $4 AND user_id = $5
        "#,
        cash_asset_name,
        cash_asset_possessed,
        cash_asset_unit_value,
        asset_id,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum UpdateCashAssetError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error("{0}")]
    #[status(BAD_REQUEST)]
    ValidationError(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
