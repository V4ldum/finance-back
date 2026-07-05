use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::CoinSearchQuery;
use crate::model::coin::Coin;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::coins_dto::CoinDataDto;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

#[derive(Deserialize)]
pub(crate) struct QueryParams {
    q: String,
}

#[tracing::instrument(
    skip_all,
    fields(
        query = %query.q
    ),
    err(Debug)
)]
pub(crate) async fn search_coins(
    Query(query): Query<QueryParams>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<CoinDataDto>>, SearchCoinsError> {
    let query = CoinSearchQuery::parse(query.q).map_err(SearchCoinsError::ValidationError)?;

    let coins = query_coins(&pool, query).await.context("Failed to fetch coins")?;

    let mut response = Vec::with_capacity(coins.len());
    for coin in coins {
        let coin = convert_coin_model_to_coin_response(coin, &pool)
            .await
            .context("Failed to convert coin model to coin response")?;

        response.push(coin);
    }

    Ok(Json(response))
}

#[tracing::instrument(skip_all)]
async fn query_coins(pool: &SqlitePool, query: CoinSearchQuery) -> Result<Vec<Coin>> {
    // TODO migrate back to query_as! macro then comptime extension is merged :
    // - https://github.com/launchbadge/sqlx/issues/3330
    // - https://github.com/launchbadge/sqlx/pull/3713
    let coins = sqlx::query_as("SELECT * FROM coins WHERE instr(UNACCENT(LOWER(name)), UNACCENT(LOWER(?))) > 0")
        .bind(query.as_ref())
        .fetch_all(pool)
        .await?;

    Ok(coins)
}

#[derive(thiserror::Error)]
pub(crate) enum SearchCoinsError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for SearchCoinsError {
    fn status(&self) -> StatusCode {
        match self {
            SearchCoinsError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SearchCoinsError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for SearchCoinsError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for SearchCoinsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
