use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::middleware::AuthenticatedUserId;
use crate::routes::raw_assets::query_raw_asset;
use crate::utils::dto::assets_dto::RawAssetsDto;

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user.id()
    ),
    err(Debug)
)]
pub(crate) async fn get_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
) -> Result<Json<RawAssetsDto>, GetRawAssetError> {
    let asset = query_raw_asset(&pool, id, user.id())
        .await
        .context("Failed to fetch raw asset")?
        .ok_or(GetRawAssetError::InvalidId)?;

    Ok(Json(RawAssetsDto {
        id: asset.id,
        name: asset.name,
        possessed: asset.possessed,
        unit_weight: asset.unit_weight,
        composition: asset.composition,
        purity: asset.purity,
    }))
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetRawAssetError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
