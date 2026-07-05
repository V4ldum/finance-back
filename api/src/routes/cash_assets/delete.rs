use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::SqlitePool;

use crate::middleware::auth::AuthenticatedUserId;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

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

#[derive(thiserror::Error)]
pub(crate) enum DeleteCashAssetError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for DeleteCashAssetError {
    fn status(&self) -> StatusCode {
        match self {
            DeleteCashAssetError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for DeleteCashAssetError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for DeleteCashAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
