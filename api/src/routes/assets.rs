use crate::middleware::auth::AuthenticatedUserId;
use crate::model::cash_asset::CashAsset;
use crate::model::coin::Coin;
use crate::model::coin_asset::CoinAsset;
use crate::model::raw_asset::RawAsset;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::assets_dto::{AssetsDto, CashAssetsDto, CoinAssetsDto, RawAssetsDto};
use anyhow::{Context, Result};
use axum::extract::State;
use axum::{Extension, Json};
use sqlx::SqlitePool;

#[tracing::instrument(
    skip_all,
    fields(
        user_id = %user_id
    ),
    err(Debug),
)]
pub(crate) async fn get_assets(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Result<Json<AssetsDto>, GetAssetsError> {
    // Query Raw Assets
    let raw_assets = query_raw_assets(&pool, user_id)
        .await
        .context("Failed to fetch raw assets")?;

    // Query Cash Assets
    let cash_assets = query_cash_assets(&pool, user_id)
        .await
        .context("Failed to fetch cash assets")?;

    // Query Coin Assets
    let coin_assets = query_coin_assets(&pool, user_id)
        .await
        .context("Failed to fetch coin assets")?;

    let mut coins = Vec::with_capacity(coin_assets.len());
    for coin_asset in coin_assets {
        let coin_model = query_coin(&pool, coin_asset.coin_id)
            .await
            .context("Failed to fetch coin")?
            // There should not be any orphan coin_assets so this should not happen
            .ok_or_else(|| {
                GetAssetsError::UnexpectedError(anyhow::anyhow!(
                    "Coin associated with coin_asset not found, this should not happen"
                ))
            })?;

        let coin_data = convert_coin_model_to_coin_response(coin_model, &pool)
            .await
            .context("Failed to convert coin model to coin response")?;

        coins.push(CoinAssetsDto {
            possessed: coin_asset.possessed,
            coin_data,
        });
    }

    Ok(Json(AssetsDto {
        raw_assets: raw_assets
            .into_iter()
            .map(|asset| RawAssetsDto {
                id: asset.id,
                name: asset.name,
                possessed: asset.possessed,
                unit_weight: asset.unit_weight,
                composition: asset.composition,
                purity: asset.purity,
            })
            .collect(),
        cash_assets: cash_assets
            .into_iter()
            .map(|asset| CashAssetsDto {
                id: asset.id,
                name: asset.name,
                possessed: asset.possessed,
                unit_value: asset.unit_value,
            })
            .collect(),
        coin_assets: coins,
    }))
}

#[tracing::instrument(skip_all)]
async fn query_raw_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<RawAsset>> {
    let assets = sqlx::query_as!(RawAsset, "SELECT * FROM raw_assets WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await?;

    Ok(assets)
}

#[tracing::instrument(skip_all)]
async fn query_cash_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<CashAsset>> {
    let assets = sqlx::query_as!(CashAsset, "SELECT * FROM cash_assets WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await?;

    Ok(assets)
}

#[tracing::instrument(skip_all)]
async fn query_coin_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<CoinAsset>> {
    let assets = sqlx::query_as!(CoinAsset, "SELECT * FROM coin_assets WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await?;

    Ok(assets)
}

#[tracing::instrument(skip_all)]
async fn query_coin(pool: &SqlitePool, coin_id: i64) -> Result<Option<Coin>> {
    let coin = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", coin_id)
        .fetch_optional(pool)
        .await?;

    Ok(coin)
}

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetAssetsError {
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
