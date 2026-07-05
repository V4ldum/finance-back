use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::model::price::PriceDb;
use crate::routes::prices::Price;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

#[derive(Serialize)]
pub(crate) struct Prices {
    gold: Price,
    silver: Price,
    sp_500: Price,
}

#[tracing::instrument(skip_all, err(Debug))]
pub(crate) async fn get_all_prices(State(pool): State<SqlitePool>) -> Result<Json<Prices>, GetPricesError> {
    let prices = query_prices(&pool).await.context("Failed to fetch prices")?;

    let gold_value = prices
        .iter()
        .find(|v| v.name == "Gold")
        .ok_or_else(|| GetPricesError::ValueNotFound("Gold".to_string()))?;
    let silver_value = prices
        .iter()
        .find(|v| v.name == "Silver")
        .ok_or_else(|| GetPricesError::ValueNotFound("Silver".to_string()))?;
    let sp_value = prices
        .iter()
        .find(|v| v.name == "SP500")
        .ok_or_else(|| GetPricesError::ValueNotFound("SP500".to_string()))?;

    Ok(Json(Prices {
        gold: Price {
            price: gold_value.value,
            last_update: gold_value.date.to_string(),
        },
        silver: Price {
            price: silver_value.value,
            last_update: silver_value.date.to_string(),
        },
        sp_500: Price {
            price: sp_value.value,
            last_update: sp_value.date.to_string(),
        },
    }))
}

#[tracing::instrument(skip_all)]
async fn query_prices(pool: &SqlitePool) -> Result<Vec<PriceDb>> {
    let prices = sqlx::query_as!(PriceDb, "SELECT * FROM prices").fetch_all(pool).await?;

    Ok(prices)
}

#[derive(thiserror::Error)]
pub(crate) enum GetPricesError {
    #[error("No price found for {0}. This should not happen")]
    ValueNotFound(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for GetPricesError {
    fn status(&self) -> StatusCode {
        match self {
            GetPricesError::ValueNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GetPricesError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for GetPricesError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for GetPricesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
