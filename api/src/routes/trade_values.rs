use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::prelude::Prices;
use crate::utils::error::APIError;

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

pub async fn trade_values_with_param(
    Path(query): Path<String>,
    Extension(db): Extension<DatabaseConnection>,
) -> Response {
    let query_key = match query.as_str() {
        "gold" => "Gold",
        "silver" => "Silver",
        "sp500" => "SP500",
        _ => {
            return APIError::unknown_query(&query).into_response();
        }
    };

    let Ok(value) = Prices::find_by_id(query_key).one(&db).await else {
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

pub async fn trade_values(Extension(db): Extension<DatabaseConnection>) -> Response {
    let Ok(prices) = Prices::find().all(&db).await else {
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
