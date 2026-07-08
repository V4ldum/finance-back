use anyhow::{Context, Result};
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::SqlitePool;

use crate::middleware::AuthenticatedUserId;

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user.id()
    ),
    err(Debug)
)]
pub(crate) async fn delete_coin_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
) -> Result<StatusCode, DeleteCoinAssetError> {
    delete_coin_asset_(&pool, user.id(), id)
        .await
        .context("Failed to delete coin asset")?;

    Ok(StatusCode::NO_CONTENT)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn delete_coin_asset_(pool: &SqlitePool, user_id: i64, coin_id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum DeleteCoinAssetError {
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
