use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::middleware::AuthenticatedUserId;
use crate::routes::cash_assets::query_cash_asset;
use crate::utils::dto::assets_dto::CashAssetsDto;

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user.id()
    ),
    err(Debug)
)]
pub(crate) async fn get_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
) -> Result<Json<CashAssetsDto>, GetCashAssetError> {
    let asset = query_cash_asset(&pool, id, user.id())
        .await
        .context("Failed to fetch cash asset")?
        .ok_or(GetCashAssetError::InvalidId)?;

    Ok(Json(CashAssetsDto {
        id: asset.id,
        name: asset.name,
        possessed: asset.possessed,
        unit_value: asset.unit_value,
    }))
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetCashAssetError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
