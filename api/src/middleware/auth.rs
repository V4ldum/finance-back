use anyhow::{Context, Result};
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use sqlx::SqlitePool;
use std::fmt::Debug;

use crate::domain::AuthenticatedUserId;

/***** ENDPOINT *****/

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

/***** DATABASE *****/

async fn fetch_user_id(pool: &SqlitePool, key: &str) -> Result<Option<i64>> {
    let user = sqlx::query!("SELECT id FROM users WHERE api_key = $1", key)
        .fetch_optional(pool)
        .await?;

    Ok(user.map(|u| u.id))
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum CheckApiKeyError {
    #[error("X-API-KEY header not provided")]
    #[status(UNAUTHORIZED)]
    NoApiKey,
    #[error("Invalid X-API-KEY provided")]
    #[status(UNAUTHORIZED)]
    InvalidApiKey,
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
