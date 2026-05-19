use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::model::coin::Coin;
use crate::utils::api_error::APIError;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;

#[derive(Deserialize)]
pub(super) struct QueryParams {
    q: String,
}

pub(crate) async fn search_coin(Query(query): Query<QueryParams>, State(pool): State<SqlitePool>) -> Response {
    // TODO migrate back to query_as! macro then comptime extension is merged :
    // - https://github.com/launchbadge/sqlx/issues/3330
    // - https://github.com/launchbadge/sqlx/pull/3713
    let coins: Vec<Coin> =
        match sqlx::query_as("SELECT * FROM coins WHERE instr(UNACCENT(LOWER(name)), UNACCENT(LOWER(?))) > 0")
            .bind(query.q)
            .fetch_all(&pool)
            .await
        {
            Ok(coins) => coins,
            Err(e) => {
                log::error!("{e}");
                return APIError::database_error().into_response();
            }
        };

    let mut coins_response = Vec::with_capacity(coins.len());
    for coin in coins.into_iter() {
        let coin_response = convert_coin_model_to_coin_response(coin, &pool).await;

        match coin_response {
            Ok(coin_response) => {
                coins_response.push(coin_response);
            }
            Err(error) => {
                return error.into_response();
            }
        };
    }

    Json(coins_response).into_response()
}

pub(crate) async fn get_coin(Path(id): Path<i64>, State(pool): State<SqlitePool>) -> Response {
    match sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(coin)) => match convert_coin_model_to_coin_response(coin, &pool).await {
            Ok(coin_response) => Json(coin_response).into_response(),
            Err(error) => error.into_response(),
        },
        Ok(None) => APIError::bad_id(&id.to_string()).into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}
