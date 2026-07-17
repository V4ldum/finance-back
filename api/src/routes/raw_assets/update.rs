use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::{AssetComposition, AssetName, AssetPossessed, AssetPurity, AssetUnitWeight, UpdateRawAsset};
use crate::middleware::AuthenticatedUserId;

/***** REQUEST *****/

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateRawAssetRequest {
    name: Option<String>,
    possessed: Option<i64>,
    unit_weight: Option<i64>,
    composition: Option<String>,
    purity: Option<i64>,
}

impl TryFrom<UpdateRawAssetRequest> for UpdateRawAsset {
    type Error = String;

    fn try_from(value: UpdateRawAssetRequest) -> Result<Self, String> {
        let name = value.name.map(AssetName::parse).transpose()?;
        let possessed = value.possessed.map(AssetPossessed::parse).transpose()?;
        let unit_weight = value.unit_weight.map(AssetUnitWeight::parse).transpose()?;
        let composition = value.composition.as_deref().map(AssetComposition::parse).transpose()?;
        let purity = value.purity.map(AssetPurity::parse).transpose()?;

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
        id = %id,
        user_id = %user.id()
    ),
    err(Debug)
)]
pub(crate) async fn update_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateRawAssetRequest>,
) -> Result<StatusCode, UpdateRawAssetError> {
    let update_raw_asset: UpdateRawAsset = request.try_into().map_err(UpdateRawAssetError::ValidationError)?;

    let rows_affected = update_raw_asset_(&pool, user.id(), id, &update_raw_asset)
        .await
        .context("Failed to update raw asset")?;

    // No row matched id + user_id, so the asset does not exist for this user
    if rows_affected == 0 {
        return Err(UpdateRawAssetError::InvalidId);
    }

    Ok(StatusCode::NO_CONTENT)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn update_raw_asset_(pool: &SqlitePool, user_id: i64, asset_id: i64, raw_asset: &UpdateRawAsset) -> Result<u64> {
    let raw_asset_name = raw_asset.name.as_ref().map(|n| n.as_ref());
    let raw_asset_possessed = raw_asset.possessed.as_ref().map(|p| p.as_ref());
    let raw_asset_unit_weight = raw_asset.unit_weight.as_ref().map(|w| w.as_ref());
    let raw_asset_composition = raw_asset.composition.as_ref().map(|c| c.as_ref());
    let raw_asset_purity = raw_asset.purity.as_ref().map(|p| p.as_ref());

    // COALESCE writes the first non-null argument in the pair
    let result = sqlx::query!(
        r#"
            UPDATE raw_assets
            SET name = COALESCE($1, name),
                possessed = COALESCE($2, possessed),
                unit_weight = COALESCE($3, unit_weight),
                composition = COALESCE($4, composition),
                purity = COALESCE($5, purity)
            WHERE id = $6 AND user_id = $7
        "#,
        raw_asset_name,
        raw_asset_possessed,
        raw_asset_unit_weight,
        raw_asset_composition,
        raw_asset_purity,
        asset_id,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum UpdateRawAssetError {
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
