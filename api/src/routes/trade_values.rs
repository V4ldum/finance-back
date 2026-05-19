use axum::Json;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::model::price::Price;
use crate::utils::api_error::APIError;

#[derive(Serialize)]
pub(super) struct TradeValues {
    gold: TradeValue,
    silver: TradeValue,
    sp_500: TradeValue,
}
#[derive(Serialize)]
pub(super) struct TradeValue {
    price: f64,
    last_update: String,
}

pub(crate) async fn get_one_trade_value(Path(query): Path<String>, State(pool): State<SqlitePool>) -> Response {
    let query_key = match query.as_str() {
        "gold" => "Gold",
        "silver" => "Silver",
        "sp500" => "SP500",
        _ => {
            return APIError::unknown_query(&query).into_response();
        }
    };

    match sqlx::query_as!(Price, "SELECT * FROM prices WHERE name = $1", query_key)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(value)) => Json(TradeValue {
            price: value.value,
            last_update: value.date.to_string(),
        })
        .into_response(),
        Ok(None) => {
            // The prices table is expected to always contain a row for every known query key
            log::warn!("No price row found for query key {query_key}. This should not happen");
            APIError::database_error().into_response()
        }
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}

pub(crate) async fn get_all_trade_values(State(pool): State<SqlitePool>) -> Response {
    match sqlx::query_as!(Price, "SELECT * FROM prices").fetch_all(&pool).await {
        Ok(prices) => {
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
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}
