use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::SqlitePool;

use crate::model::coin::Coin;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::coins_dto::CoinDataDto;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

#[tracing::instrument(
    skip_all,
    fields(
        id = %id
    ),
    err(Debug),
)]
pub(crate) async fn get_coin(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<CoinDataDto>, GetCoinError> {
    let coin = query_coin(&pool, id)
        .await
        .context("Failed to fetch coin")?
        .ok_or(GetCoinError::InvalidId)?;

    let coin = convert_coin_model_to_coin_response(coin, &pool)
        .await
        .context("Failed to convert coin model to coin response")?;

    Ok(Json(coin))
}

#[tracing::instrument(skip_all)]
async fn query_coin(pool: &SqlitePool, id: i64) -> Result<Option<Coin>> {
    let coin = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;

    Ok(coin)
}

#[derive(thiserror::Error)]
pub(crate) enum GetCoinError {
    #[error("The provided id is invalid")]
    InvalidId,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for GetCoinError {
    fn status(&self) -> StatusCode {
        match self {
            GetCoinError::InvalidId => StatusCode::NOT_FOUND,
            GetCoinError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for GetCoinError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for GetCoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
