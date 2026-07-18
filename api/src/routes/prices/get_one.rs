use anyhow::{Context, Result};
use axum::extract::{Path, State};
use sqlx::SqlitePool;

use crate::Json;
use crate::domain::AssetPrice;
use crate::routes::prices::{Price, PriceResponse};

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        query = %query
    ),
    err(Debug),
)]
pub(crate) async fn get_one_price(
    Path(query): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<PriceResponse>, GetPriceError> {
    let price = AssetPrice::parse(&query).map_err(GetPriceError::UnknownPriceKey)?;

    let value = query_price(&pool, &price)
        .await
        .context("Failed to fetch price")?
        // The prices table is expected to always contain a row for every known query key
        .ok_or_else(|| {
            GetPriceError::UnexpectedError(anyhow::anyhow!(
                "No price found for query key {}. This should not happen",
                price.as_ref()
            ))
        })?;

    Ok(Json(PriceResponse::from(&value)))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_price(pool: &SqlitePool, price: &AssetPrice) -> Result<Option<Price>> {
    let price_key = price.as_ref();

    let price = sqlx::query_as!(Price, "SELECT * FROM prices WHERE name = $1", price_key)
        .fetch_optional(pool)
        .await?;

    Ok(price)
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetPriceError {
    #[error("{0}")]
    #[status(NOT_FOUND)]
    UnknownPriceKey(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
