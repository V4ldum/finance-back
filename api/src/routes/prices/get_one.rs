use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::SqlitePool;

use crate::domain::AssetPrice;
use crate::model::price::PriceDb;
use crate::routes::prices::Price;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

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
) -> Result<Json<Price>, GetPriceError> {
    let price = AssetPrice::parse(&query).map_err(GetPriceError::UnknownPrice)?;

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

    Ok(Json(Price {
        price: value.value,
        last_update: value.date.to_string(),
    }))
}

#[tracing::instrument(skip_all)]
async fn query_price(pool: &SqlitePool, price: &AssetPrice) -> Result<Option<PriceDb>> {
    let price_key = price.as_ref();

    let price = sqlx::query_as!(PriceDb, "SELECT * FROM prices WHERE name = $1", price_key)
        .fetch_optional(pool)
        .await?;

    Ok(price)
}

#[derive(thiserror::Error)]
pub(crate) enum GetPriceError {
    #[error("{0}")]
    UnknownPrice(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for GetPriceError {
    fn status(&self) -> StatusCode {
        match self {
            GetPriceError::UnknownPrice(_) => StatusCode::NOT_FOUND,
            GetPriceError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for GetPriceError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for GetPriceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
