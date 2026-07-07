use anyhow::{Context, Result};
use axum::Json;
use axum::extract::State;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::model::price::PriceDb;
use crate::routes::prices::PriceDto;

/***** ENDPOINT *****/

#[tracing::instrument(skip_all, err(Debug))]
pub(crate) async fn get_all_prices(State(pool): State<SqlitePool>) -> Result<Json<PricesDto>, GetPricesError> {
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

    Ok(Json(PricesDto {
        gold: PriceDto {
            price: gold_value.value,
            last_update: gold_value.date.to_string(),
        },
        silver: PriceDto {
            price: silver_value.value,
            last_update: silver_value.date.to_string(),
        },
        sp_500: PriceDto {
            price: sp_value.value,
            last_update: sp_value.date.to_string(),
        },
    }))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_prices(pool: &SqlitePool) -> Result<Vec<PriceDb>> {
    let prices = sqlx::query_as!(PriceDb, "SELECT * FROM prices").fetch_all(pool).await?;

    Ok(prices)
}

/***** DATA TRANSFER OBJECTS *****/

#[derive(Serialize)]
pub(crate) struct PricesDto {
    gold: PriceDto,
    silver: PriceDto,
    sp_500: PriceDto,
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
