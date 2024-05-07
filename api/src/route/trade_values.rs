use axum::extract::{Path, State};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::database::Database;
use crate::util::api_error::APIError;

#[derive(Serialize)]
pub struct TradeValues {
    pub gold: TradeValue,
    pub silver: TradeValue,
    pub sp_500: TradeValue,
}
#[derive(Serialize)]
pub struct TradeValue {
    pub price: f64,
    pub last_update: String,
}

pub async fn get_one_trade_value(
    Path(query): Path<String>,
    State(database): State<Database>,
) -> Response {
    let query_key = match query.as_str() {
        "gold" => "Gold",
        "silver" => "Silver",
        "sp500" => "SP500",
        _ => {
            return APIError::unknown_query(&query).into_response();
        }
    };

    let Ok(value) = database.find_one_trade_value(query_key).await else {
        return APIError::database_error().into_response();
    };

    match value {
        Some(value) => Json(TradeValue {
            price: value.value,
            last_update: value.date.to_string(),
        })
        .into_response(),
        None => APIError::database_error().into_response(),
    }
}

pub async fn get_all_trade_values(State(database): State<Database>) -> Response {
    let Ok(prices) = database.find_all_trade_values().await else {
        return APIError::database_error().into_response();
    };

    let gold_value = prices
        .iter()
        .find(|v| v.name == "Gold")
        .expect("This value must be found");
    let silver_value = prices
        .iter()
        .find(|v| v.name == "Silver")
        .expect("This value must be found");
    let sp_value = prices
        .iter()
        .find(|v| v.name == "SP500")
        .expect("This value must be found");

    Json(TradeValues {
        gold: TradeValue {
            price: gold_value.value,
            last_update: gold_value.date.to_string(),
        },
        silver: TradeValue {
            price: silver_value.value,
            last_update: silver_value.date.to_string(),
        },
        sp_500: TradeValue {
            price: sp_value.value,
            last_update: sp_value.date.to_string(),
        },
    })
    .into_response()
}
