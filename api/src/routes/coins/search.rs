use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::CoinSearchQuery;
use crate::model::coin::Coin;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::coins_dto::CoinDataDto;

/***** REQUEST *****/

#[derive(Deserialize)]
pub(crate) struct QueryParams {
    q: String,
}

/***** ENDPOINT *****/

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

/***** DATABASE *****/

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

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum SearchCoinsError {
    #[error("{0}")]
    #[status(BAD_REQUEST)]
    ValidationError(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
