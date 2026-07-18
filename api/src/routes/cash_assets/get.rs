use anyhow::{Context, Result};
use axum::Extension;
use axum::extract::{Path, State};
use sqlx::SqlitePool;

use crate::Json;
use crate::middleware::AuthenticatedUserId;
use crate::routes::cash_assets::{CashAssetResponse, query_cash_asset};

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
) -> Result<Json<CashAssetResponse>, GetCashAssetError> {
    let asset = query_cash_asset(&pool, id, user.id())
        .await
        .context("Failed to fetch cash asset")?
        .ok_or(GetCashAssetError::InvalidId)?;

    Ok(Json(CashAssetResponse {
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
