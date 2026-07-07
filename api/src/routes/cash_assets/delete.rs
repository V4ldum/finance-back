use crate::middleware::auth::AuthenticatedUserId;
use anyhow::{Context, Result};
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::SqlitePool;

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user_id,
    ),
    err(Debug)
)]
pub(crate) async fn delete_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Result<StatusCode, DeleteCashAssetError> {
    delete_cash_asset_(&pool, id, user_id)
        .await
        .context("Failed to delete cash asset")?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(skip_all)]
async fn delete_cash_asset_(pool: &SqlitePool, asset_id: i64, user_id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM cash_assets WHERE id = $1 AND user_id = $2",
        asset_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum DeleteCashAssetError {
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
