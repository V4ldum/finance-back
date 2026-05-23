use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::cash_asset::CashAsset;
use crate::model::coin::Coin;
use crate::model::coin_asset::CoinAsset;
use crate::model::raw_asset::RawAsset;
use crate::utils::api_error::APIError;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::assets_dto::{AssetsDto, CashAssetsDto, CoinAssetsDto, RawAssetsDto};
use anyhow::Result;

#[tracing::instrument(
    name = "get all assets",
    skip_all,
    fields(
        user_id = %user_id
    )
)]
pub(crate) async fn get_assets(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    // Query Raw Assets
    let raw_assets = match get_raw_assets(&pool, user_id).await {
        Ok(raw_assets) => raw_assets,
        Err(_) => return APIError::database_error().into_response(),
    };

    // Query Cash Assets
    let cash_assets = match get_cash_assets(&pool, user_id).await {
        Ok(cash_assets) => cash_assets,
        Err(_) => return APIError::database_error().into_response(),
    };

    // Query Coin Assets
    let coin_assets_models = match get_coin_assets(&pool, user_id).await {
        Ok(coin_assets_models) => coin_assets_models,
        Err(_) => return APIError::database_error().into_response(),
    };

    let mut coins_assets = Vec::with_capacity(coin_assets_models.len());
    for coin_asset_model in coin_assets_models.into_iter() {
        match get_coin(&pool, coin_asset_model.coin_id).await {
            Ok(Some(coin_model)) => {
                match convert_coin_model_to_coin_response(coin_model, &pool).await {
                    Ok(coin_data) => {
                        coins_assets.push(CoinAssetsDto {
                            possessed: coin_asset_model.possessed,
                            coin_data,
                        });
                    }
                    Err(error) => {
                        return error.into_response();
                    }
                };
            }
            Ok(None) => {
                // There should not be any orphan coin_assets so this should not happen
                tracing::warn!("Coin associated with coin_asset not found, this should not happen");
                return APIError::database_error().into_response();
            }
            Err(_) => return APIError::database_error().into_response(),
        };
    }

    Json(AssetsDto {
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
        coin_assets: coins_assets,
    })
    .into_response()
}

#[tracing::instrument(name = "get raw assets", skip_all)]
async fn get_raw_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<RawAsset>> {
    let assets = sqlx::query_as!(RawAsset, "SELECT * FROM raw_assets WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(assets)
}

#[tracing::instrument(name = "get cash assets", skip_all)]
async fn get_cash_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<CashAsset>> {
    let assets = sqlx::query_as!(CashAsset, "SELECT * FROM cash_assets WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(assets)
}

#[tracing::instrument(name = "get coin assets", skip_all)]
async fn get_coin_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<CoinAsset>> {
    let assets = sqlx::query_as!(CoinAsset, "SELECT * FROM coin_assets WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(assets)
}

#[tracing::instrument(name = "get coin", skip_all)]
async fn get_coin(pool: &SqlitePool, coin_id: i64) -> Result<Option<Coin>> {
    let coin = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", coin_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(coin)
}
