use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::database::Database;
use crate::util::api_error::APIError;
use crate::util::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;

#[derive(Deserialize)]
pub struct QueryParams {
    q: String,
}

pub async fn search_coin(
    Query(query): Query<QueryParams>,
    State(database): State<Database>,
) -> Response {
    let Ok(coins) = database.search_coin(&query.q).await else {
        return APIError::database_error().into_response();
    };

    let mut coins_response = Vec::with_capacity(coins.len());

    for coin in coins.into_iter() {
        let coin_response = convert_coin_model_to_coin_response(coin, &database).await;

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

pub async fn get_coin(Path(id): Path<String>, State(database): State<Database>) -> Response {
    let Ok(id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(coin) = database.find_coin(id).await else {
        return APIError::database_error().into_response();
    };

    let Some(coin) = coin else {
        return APIError::bad_id(&id.to_string()).into_response();
    };

    let coin_response = convert_coin_model_to_coin_response(coin, &database).await;

    match coin_response {
        Ok(coin_response) => Json(coin_response).into_response(),
        Err(error) => error.into_response(),
    }
}
