use anyhow::Result;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::CoinSearchQuery;
use crate::model::coin::Coin;
use crate::utils::api_error::APIError;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;

#[derive(Deserialize)]
pub(super) struct QueryParams {
    q: String,
}

#[tracing::instrument(
    name = "search a coin",
    skip_all,
    fields(
        query = %query.q
    )
)]
pub(crate) async fn search_coin(Query(query): Query<QueryParams>, State(pool): State<SqlitePool>) -> Response {
    let query = match CoinSearchQuery::parse(query.q) {
        Ok(query) => query,
        Err(err) => return APIError::invalid_value(&err.to_string()).into_response(),
    };

    let coins = match query_coins(&pool, query).await {
        Ok(coins) => coins,
        Err(_) => return APIError::database_error().into_response(),
    };

    let mut coins_response = Vec::with_capacity(coins.len());
    for coin in coins.into_iter() {
        let coin_response = convert_coin_model_to_coin_response(coin, &pool).await;

        match coin_response {
            Ok(coin_response) => coins_response.push(coin_response),
            Err(error) => return error.into_response(),
        };
    }

    Json(coins_response).into_response()
}

#[tracing::instrument(name = "query coins", skip_all)]
async fn query_coins(pool: &SqlitePool, query: CoinSearchQuery) -> Result<Vec<Coin>> {
    // TODO migrate back to query_as! macro then comptime extension is merged :
    // - https://github.com/launchbadge/sqlx/issues/3330
    // - https://github.com/launchbadge/sqlx/pull/3713
    let coins = sqlx::query_as("SELECT * FROM coins WHERE instr(UNACCENT(LOWER(name)), UNACCENT(LOWER(?))) > 0")
        .bind(query.as_ref())
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(coins)
}

#[tracing::instrument(
    name = "get a coin",
    skip_all,
    fields(
        id = %id
    )
)]
pub(crate) async fn get_coin(Path(id): Path<i64>, State(pool): State<SqlitePool>) -> Response {
    match query_coin(&pool, id).await {
        Ok(Some(coin)) => match convert_coin_model_to_coin_response(coin, &pool).await {
            Ok(coin_response) => Json(coin_response).into_response(),
            Err(error) => error.into_response(),
        },
        Ok(None) => APIError::bad_id(&id.to_string()).into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "query coin", skip_all)]
async fn query_coin(pool: &SqlitePool, id: i64) -> Result<Option<Coin>> {
    let coin = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(coin)
}
