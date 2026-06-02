use anyhow::Result;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::domain::AssetTradeValue;
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

#[tracing::instrument(
    name = "get one trade value",
    skip_all,
    fields(
        query = %query
    )
)]
pub(crate) async fn get_one_trade_value(Path(query): Path<String>, State(pool): State<SqlitePool>) -> Response {
    let trade_value = match AssetTradeValue::parse(query) {
        Ok(trade_value) => trade_value,
        Err(err) => {
            return APIError::unknown_query(&err.to_string()).into_response();
        }
    };

    match get_price(&pool, &trade_value).await {
        Ok(Some(value)) => Json(TradeValue {
            price: value.value,
            last_update: value.date.to_string(),
        })
        .into_response(),
        Ok(None) => {
            // The prices table is expected to always contain a row for every known query key
            tracing::warn!(
                "No price row found for query key {}. This should not happen",
                trade_value.as_ref()
            );
            APIError::database_error().into_response()
        }
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "get price", skip_all)]
async fn get_price(pool: &SqlitePool, trade_value: &AssetTradeValue) -> Result<Option<Price>> {
    let trade_value_key = trade_value.as_ref();

    let price = sqlx::query_as!(Price, "SELECT * FROM prices WHERE name = $1", trade_value_key)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(price)
}

#[tracing::instrument(name = "get all trade values", skip_all)]
pub(crate) async fn get_all_trade_values(State(pool): State<SqlitePool>) -> Response {
    match get_all_prices(&pool).await {
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
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "get all prices", skip_all)]
async fn get_all_prices(pool: &SqlitePool) -> Result<Vec<Price>> {
    let prices = sqlx::query_as!(Price, "SELECT * FROM prices")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(prices)
}
