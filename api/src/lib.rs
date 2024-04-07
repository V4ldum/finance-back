use axum::{extract, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use serde::Deserialize;

use crate::database::Database;
use crate::error::APIError;
use crate::trade_values::*;

mod database;
mod error;
mod trade_values;
type APIResult<T> = Result<Json<T>, (StatusCode, Json<APIError>)>;

fn check_api_key(
    key: &Option<String>,
    database: &Database,
) -> Result<(), (StatusCode, Json<APIError>)> {
    if let Some(key) = key {
        let Ok(is_valid) = database.check_api_key(key) else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(APIError::database_error()),
            ));
        };

        if !is_valid {
            return Err((StatusCode::UNAUTHORIZED, Json(APIError::bad_api_key())));
        }
    } else {
        return Err((StatusCode::BAD_REQUEST, Json(APIError::no_api_key())));
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct ApiKeyParam {
    pub key: Option<String>,
}

pub async fn health_check() -> &'static str {
    "API is working!"
}

pub async fn trade_values(params: extract::Query<ApiKeyParam>) -> APIResult<TradeValues> {
    let Ok(database) = Database::build() else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(APIError::database_error()),
        ));
    };

    check_api_key(&params.key, &database)?;

    let mut prices = database.query_trade_values().unwrap();

    let gold_value = prices.remove("Gold").expect("This value must be found");
    let silver_value = prices.remove("Silver").expect("This value must be found");
    let sp_value = prices.remove("SP500").expect("This value must be found");

    Ok(Json(TradeValues {
        gold: gold_value,
        silver: silver_value,
        sp_500: sp_value,
    }))
}

pub async fn trade_value(
    Path(key): Path<String>,
    params: extract::Query<ApiKeyParam>,
) -> APIResult<TradeValue> {
    let Ok(database) = Database::build() else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(APIError::database_error()),
        ));
    };

    check_api_key(&params.key, &database)?;

    let value = match key.as_str() {
        "gold" => database.query_trade_value("Gold"),
        "silver" => database.query_trade_value("Silver"),
        "sp500" => database.query_trade_value("SP500"),
        _ => return Err((StatusCode::BAD_REQUEST, Json(APIError::unknown_query(&key)))),
    };

    if let Ok(value) = value {
        Ok(Json(value))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(APIError::database_error()),
        ))
    }
}
