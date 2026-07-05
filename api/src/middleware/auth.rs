use std::fmt::Debug;

use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};
use anyhow::{Context, Result};
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use sqlx::SqlitePool;

#[derive(Clone, Copy)]
pub(crate) struct AuthenticatedUserId(pub i64);

#[tracing::instrument(skip_all)]
pub(crate) async fn check_api_key(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response<Body>, CheckApiKeyError> {
    let key = headers
        .get("X-API-Key")
        .ok_or(CheckApiKeyError::NoApiKey)?
        .to_str()
        .map_err(|_| CheckApiKeyError::InvalidApiKey)?;

    let user_id = fetch_user_id(&pool, key)
        .await
        .context("Failed to fetch api key")?
        .ok_or(CheckApiKeyError::InvalidApiKey)?;

    request.extensions_mut().insert(AuthenticatedUserId(user_id));
    Ok(next.run(request).await)
}

async fn fetch_user_id(pool: &SqlitePool, key: &str) -> Result<Option<i64>> {
    let user = sqlx::query!("SELECT id FROM users WHERE api_key = $1", key)
        .fetch_optional(pool)
        .await?;

    Ok(user.map(|u| u.id))
}

#[derive(thiserror::Error)]
pub(crate) enum CheckApiKeyError {
    #[error("X-API-KEY header not provided")]
    NoApiKey,
    #[error("Invalid X-API-KEY provided")]
    InvalidApiKey,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for CheckApiKeyError {
    fn status(&self) -> StatusCode {
        match self {
            CheckApiKeyError::NoApiKey => StatusCode::UNAUTHORIZED,
            CheckApiKeyError::InvalidApiKey => StatusCode::UNAUTHORIZED,
            CheckApiKeyError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for CheckApiKeyError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for CheckApiKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
