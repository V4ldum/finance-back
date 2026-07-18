use anyhow::{Context, Result};
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::Json;
use crate::domain::AssetPossessed;
use crate::middleware::AuthenticatedUserId;

/***** REQUEST *****/

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateCoinAssetRequest {
    possessed: i64,
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
pub(crate) async fn update_coin_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateCoinAssetRequest>,
) -> Result<StatusCode, UpdateCoinAssetError> {
    let asset_possessed = AssetPossessed::parse(request.possessed).map_err(UpdateCoinAssetError::ValidationError)?;

    let rows_affected = update_coin_asset_(&pool, user.id(), id, asset_possessed)
        .await
        .context("Failed to update coin asset")?;

    // No row matched coin_id + user_id, so the asset does not exist for this user
    if rows_affected == 0 {
        return Err(UpdateCoinAssetError::InvalidId);
    }

    Ok(StatusCode::NO_CONTENT)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn update_coin_asset_(pool: &SqlitePool, user_id: i64, coin_id: i64, possessed: AssetPossessed) -> Result<u64> {
    let possessed = possessed.as_ref();

    let result = sqlx::query!(
        "UPDATE coin_assets SET possessed = $1 WHERE coin_id = $2 AND user_id = $3",
        possessed,
        coin_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum UpdateCoinAssetError {
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
