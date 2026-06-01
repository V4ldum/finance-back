use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;
use sqlx::error::ErrorKind;

use crate::domain::{AssetPossessed, CreateCoinAsset};
use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::coin::Coin;
use crate::model::coin_asset::CoinAsset;
use crate::utils::api_error::APIError;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::assets_dto::CoinAssetsDto;

#[tracing::instrument(
    name = "get coin asset",
    skip_all,
    fields(
        id = %coin_id,
        user_id = %user_id
    )
)]
pub(crate) async fn get_coin_asset(
    Path(coin_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    let coin_asset = match query_coin_asset(&pool, coin_id, user_id).await {
        Ok(Some(coin)) => coin,
        Ok(None) => return APIError::bad_id(&coin_id.to_string()).into_response(),
        Err(_) => return APIError::database_error().into_response(),
    };

    match query_coin(&pool, coin_id).await {
        Ok(Some(coin_data)) => match convert_coin_model_to_coin_response(coin_data, &pool).await {
            Ok(coin_data) => Json(CoinAssetsDto {
                possessed: coin_asset.possessed,
                coin_data,
            })
            .into_response(),
            Err(e) => e.into_response(),
        },
        Ok(None) => {
            // There should not be any orphan coin_assets so this should not happen
            tracing::warn!("Coin associated with coin_asset not found, this should not happen");
            APIError::database_error().into_response()
        }
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "query coin asset", skip_all)]
async fn query_coin_asset(pool: &SqlitePool, coin_id: i64, user_id: i64) -> Result<Option<CoinAsset>> {
    let coin_asset = sqlx::query_as!(
        CoinAsset,
        "SELECT * FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(coin_asset)
}

#[tracing::instrument(name = "query coin", skip_all)]
async fn query_coin(pool: &SqlitePool, coin_id: i64) -> Result<Option<Coin>> {
    let coin = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", coin_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

    Ok(coin)
}

#[derive(Deserialize)]
pub(super) struct CreateCoinAssetRequest {
    coin_id: i64,
    possessed: i64,
}

impl TryFrom<CreateCoinAssetRequest> for CreateCoinAsset {
    type Error = anyhow::Error;

    fn try_from(value: CreateCoinAssetRequest) -> Result<Self> {
        let possessed = AssetPossessed::parse(value.possessed)?;

        Ok(Self {
            coin_id: value.coin_id,
            possessed,
        })
    }
}

#[tracing::instrument(
    name = "create coin asset",
    skip_all,
    fields(
        user_id = %user_id,
        coin_id = %request.coin_id,
        possessed = %request.possessed,
    )
)]
pub(crate) async fn create_coin_asset(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateCoinAssetRequest>,
) -> Response {
    let create_coin_asset: CreateCoinAsset = match request.try_into() {
        Ok(create_coin_asset) => create_coin_asset,
        Err(err) => return APIError::invalid_value(&err.to_string()).into_response(),
    };

    match create_a_coin_asset(&pool, user_id, &create_coin_asset).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => match e.downcast::<sqlx::Error>() {
            Ok(sqlx::Error::Database(db_error)) if db_error.kind() == ErrorKind::UniqueViolation => {
                APIError::invalid_value(&format!("you already possess coin_id {}", create_coin_asset.coin_id))
            }
            _ => APIError::database_error(),
        }
        .into_response(),
    }
}

#[tracing::instrument(name = "create a coin asset", skip_all)]
async fn create_a_coin_asset(pool: &SqlitePool, user_id: i64, coin_asset: &CreateCoinAsset) -> Result<()> {
    let coin_asset_possessed = coin_asset.possessed.as_ref();

    sqlx::query!(
        r#"
            INSERT INTO coin_assets (coin_id, user_id, possessed)
            VALUES ($1, $2, $3)
        "#,
        coin_asset.coin_id,
        user_id,
        coin_asset_possessed
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}

#[derive(Deserialize)]
pub(super) struct UpdateCoinAssetRequest {
    possessed: i64,
}

#[tracing::instrument(
    name = "update coin asset",
    skip_all,
    fields(
        id = %coin_id,
        user_id = %user_id,
        possessed = %request.possessed
    )
)]
pub(crate) async fn update_coin_asset(
    Path(coin_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateCoinAssetRequest>,
) -> Response {
    let asset_possessed = match AssetPossessed::parse(request.possessed) {
        Ok(asset_possessed) => asset_possessed,
        Err(err) => return APIError::invalid_value(&err.to_string()).into_response(),
    };

    let coin_asset = match query_coin_asset(&pool, coin_id, user_id).await {
        Ok(Some(coin_asset)) => coin_asset,
        Ok(None) => return APIError::bad_id(&coin_id.to_string()).into_response(),
        Err(_) => return APIError::database_error().into_response(),
    };

    if coin_asset.possessed != *asset_possessed.as_ref()
        && update_a_coin_asset(&pool, user_id, coin_id, asset_possessed)
            .await
            .is_err()
    {
        return APIError::database_error().into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

#[tracing::instrument(name = "update a coin asset", skip_all)]
async fn update_a_coin_asset(pool: &SqlitePool, user_id: i64, coin_id: i64, possessed: AssetPossessed) -> Result<()> {
    let possessed = possessed.as_ref();

    sqlx::query!(
        "UPDATE coin_assets SET possessed = $1 WHERE coin_id = $2 AND user_id = $3",
        possessed,
        coin_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}

#[tracing::instrument(
    name = "delete coin asset",
    skip_all,
    fields(
        coin_id = %coin_id,
        user_id = %user_id
    )
)]
pub(crate) async fn delete_coin_asset(
    Path(coin_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match delete_a_coin_asset(&pool, user_id, coin_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "delete a coin asset", skip_all)]
async fn delete_a_coin_asset(pool: &SqlitePool, user_id: i64, coin_id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}
