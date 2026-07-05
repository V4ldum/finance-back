use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::middleware::auth::AuthenticatedUserId;
use crate::model::coin::Coin;
use crate::routes::coin_assets::query_coin_asset;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::assets_dto::CoinAssetsDto;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user_id
    ),
    err(Debug)
)]
pub(crate) async fn get_coin_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Result<Json<CoinAssetsDto>, GetCoinAssetError> {
    let coin_asset = query_coin_asset(&pool, id, user_id)
        .await
        .context("Failed to fetch coin asset")?
        .ok_or(GetCoinAssetError::InvalidId)?;

    let coin_data = query_coin(&pool, id)
        .await
        .context("Failed to fetch coin")?
        // There should not be any orphan coin_assets so this should not happen
        .ok_or_else(|| {
            GetCoinAssetError::UnexpectedError(anyhow::anyhow!(
                "Coin associated with coin_asset not found, this should not happen"
            ))
        })?;

    let coin_data = convert_coin_model_to_coin_response(coin_data, &pool)
        .await
        .context("Failed to convert coin model to coin response")?;

    Ok(Json(CoinAssetsDto {
        possessed: coin_asset.possessed,
        coin_data,
    }))
}

#[tracing::instrument(skip_all)]
async fn query_coin(pool: &SqlitePool, coin_id: i64) -> Result<Option<Coin>> {
    let coin = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", coin_id)
        .fetch_optional(pool)
        .await?;

    Ok(coin)
}

#[derive(thiserror::Error)]
pub(crate) enum GetCoinAssetError {
    #[error("The provided id is invalid")]
    InvalidId,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for GetCoinAssetError {
    fn status(&self) -> StatusCode {
        match self {
            GetCoinAssetError::InvalidId => StatusCode::NOT_FOUND,
            GetCoinAssetError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for GetCoinAssetError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for GetCoinAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
