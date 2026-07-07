use crate::model::coin::Coin;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::coins_dto::CoinDataDto;
use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Path, State};
use sqlx::SqlitePool;

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

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetCoinError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
