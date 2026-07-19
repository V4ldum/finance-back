use anyhow::{Context, Result};
use axum::extract::State;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::Json;
use crate::routes::prices::{Price, PriceResponse};

/***** ENDPOINT *****/

#[tracing::instrument(skip_all, err(Debug))]
pub(crate) async fn get_all_prices(State(pool): State<SqlitePool>) -> Result<Json<PricesResponse>, GetPricesError> {
    let prices = query_prices(&pool).await.context("Failed to fetch prices")?;

    let price_of = |name: &str| {
        prices
            .iter()
            .find(|v| v.name == name)
            .map(PriceResponse::from)
            .ok_or_else(|| GetPricesError::ValueNotFound(name.to_string()))
    };

    Ok(Json(PricesResponse {
        gold: price_of("Gold")?,
        silver: price_of("Silver")?,
        sp_500: price_of("SP500")?,
    }))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_prices(pool: &SqlitePool) -> Result<Vec<Price>> {
    let prices = sqlx::query_as!(Price, r#"SELECT name, value, date AS "date: _" FROM prices"#)
        .fetch_all(pool)
        .await?;

    Ok(prices)
}

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct PricesResponse {
    gold: PriceResponse,
    silver: PriceResponse,
    sp_500: PriceResponse,
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetPricesError {
    #[error("No price found for {0}. This should not happen")]
    #[status(INTERNAL_SERVER_ERROR)]
    ValueNotFound(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
