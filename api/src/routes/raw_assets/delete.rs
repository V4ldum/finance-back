use anyhow::{Context, Result};
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::SqlitePool;

use crate::domain::AuthenticatedUserId;

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user_id
    ),
    err(Debug)
)]
pub(crate) async fn delete_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Result<StatusCode, DeleteRawAssetError> {
    delete_raw_asset_(&pool, user_id, id)
        .await
        .context("Failed to delete raw asset")?;

    Ok(StatusCode::NO_CONTENT)
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn delete_raw_asset_(pool: &SqlitePool, user_id: i64, asset_id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM raw_assets WHERE id = $1 AND user_id = $2",
        asset_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum DeleteRawAssetError {
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
