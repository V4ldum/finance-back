use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::{AssetComposition, AssetName, AssetPossessed, AssetPurity, AssetUnitWeight, CreateRawAsset};
use crate::middleware::AuthenticatedUserId;

/***** REQUEST *****/

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CreateRawAssetRequest {
    name: String,
    possessed: i64,
    unit_weight: i64,
    composition: String,
    purity: i64,
}

impl TryFrom<CreateRawAssetRequest> for CreateRawAsset {
    type Error = String;

    fn try_from(value: CreateRawAssetRequest) -> Result<Self, String> {
        let name = AssetName::parse(value.name)?;
        let possessed = AssetPossessed::parse(value.possessed)?;
        let unit_weight = AssetUnitWeight::parse(value.unit_weight)?;
        let composition = AssetComposition::parse(&value.composition)?;
        let purity = AssetPurity::parse(value.purity)?;

        Ok(Self {
            name,
            possessed,
            unit_weight,
            composition,
            purity,
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
        unit_weight = %request.unit_weight,
        composition = %request.composition,
        purity = %request.purity
    ),
    err(Debug)
)]
pub(crate) async fn create_raw_asset(
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateRawAssetRequest>,
) -> Result<StatusCode, CreateRawAssetError> {
    let create_raw_asset: CreateRawAsset = request.try_into().map_err(CreateRawAssetError::ValidationError)?;

    insert_raw_asset(&pool, user.id(), &create_raw_asset)
        .await
        .context("Failed to insert raw asset")?;

    Ok(StatusCode::CREATED)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn insert_raw_asset(pool: &SqlitePool, user_id: i64, raw_asset: &CreateRawAsset) -> Result<()> {
    let raw_asset_name = raw_asset.name.as_ref();
    let raw_asset_possessed = raw_asset.possessed.as_ref();
    let raw_asset_unit_weight = raw_asset.unit_weight.as_ref();
    let raw_asset_composition = raw_asset.composition.as_ref();
    let raw_asset_purity = raw_asset.purity.as_ref();

    sqlx::query!(
        r#"
            INSERT INTO raw_assets (name, possessed, unit_weight, composition, purity, user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        raw_asset_name,
        raw_asset_possessed,
        raw_asset_unit_weight,
        raw_asset_composition,
        raw_asset_purity,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum CreateRawAssetError {
    #[error("{0}")]
    #[status(BAD_REQUEST)]
    ValidationError(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
