use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::{AssetComposition, AssetName, AssetPossessed, AssetPurity, AssetUnitWeight, UpdateRawAsset};
use crate::middleware::AuthenticatedUserId;
use crate::model::raw_asset::RawAsset;
use crate::routes::raw_assets::query_raw_asset;

/***** REQUEST *****/

#[derive(Deserialize)]
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
        user_id = %user.id(),
        name = ?request.name,
        possessed = ?request.possessed,
        unit_weight = ?request.unit_weight,
        composition = ?request.composition,
        purity = ?request.purity
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

    let asset = query_raw_asset(&pool, id, user.id())
        .await
        .context("Failed to fetch raw asset")?
        .ok_or(UpdateRawAssetError::InvalidId)?;

    // Only write if a provided field actually differs from the stored value
    if has_changes(&update_raw_asset, &asset) {
        update_raw_asset_(&pool, user.id(), id, &update_raw_asset)
            .await
            .context("Failed to update raw asset")?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/***** HELPERS *****/

fn has_changes(update: &UpdateRawAsset, current: &RawAsset) -> bool {
    update.name.as_ref().is_some_and(|v| v.as_ref() != current.name)
        || update
            .possessed
            .as_ref()
            .is_some_and(|v| *v.as_ref() != current.possessed)
        || update
            .unit_weight
            .as_ref()
            .is_some_and(|v| *v.as_ref() != current.unit_weight)
        || update
            .composition
            .as_ref()
            .is_some_and(|v| v.as_ref() != current.composition)
        || update.purity.as_ref().is_some_and(|v| *v.as_ref() != current.purity)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn update_raw_asset_(pool: &SqlitePool, user_id: i64, asset_id: i64, raw_asset: &UpdateRawAsset) -> Result<()> {
    let raw_asset_name = raw_asset.name.as_ref().map(|n| n.as_ref());
    let raw_asset_possessed = raw_asset.possessed.as_ref().map(|p| p.as_ref());
    let raw_asset_unit_weight = raw_asset.unit_weight.as_ref().map(|w| w.as_ref());
    let raw_asset_composition = raw_asset.composition.as_ref().map(|c| c.as_ref());
    let raw_asset_purity = raw_asset.purity.as_ref().map(|p| p.as_ref());

    // COALESCE writes the first non-null argument in the pair
    sqlx::query!(
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

    Ok(())
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
