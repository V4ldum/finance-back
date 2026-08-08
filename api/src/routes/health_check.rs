use anyhow::{Context, Result};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use sqlx::SqlitePool;

/***** ENDPOINT *****/

#[tracing::instrument(skip_all, err(Debug))]
pub(crate) async fn health_check(State(pool): State<SqlitePool>) -> Result<Response, HealthCheckError> {
    ping_database(&pool).await.context("Failed to reach the database")?;

    Ok("OK".into_response())
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn ping_database(pool: &SqlitePool) -> Result<()> {
    // Test database connectivity
    sqlx::query("SELECT 1 FROM _sqlx_migrations LIMIT 1")
        .fetch_optional(pool)
        .await?;

    Ok(())
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum HealthCheckError {
    #[error(transparent)]
    #[status(SERVICE_UNAVAILABLE)]
    UnexpectedError(#[from] anyhow::Error),
}
