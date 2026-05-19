use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;
use sqlx::error::ErrorKind;

use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::coin::Coin;
use crate::model::coin_asset::CoinAsset;
use crate::utils::api_error::APIError;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::assets_dto::CoinAssetsDto;

pub(crate) async fn get_coin_asset(
    Path(coin_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    let coin_asset = match sqlx::query_as!(
        CoinAsset,
        "SELECT * FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(coin)) => coin,
        Ok(None) => return APIError::bad_id(&coin_id.to_string()).into_response(),
        Err(e) => {
            log::error!("{e}");
            return APIError::database_error().into_response();
        }
    };

    match sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", coin_id)
        .fetch_optional(&pool)
        .await
    {
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
            log::warn!("Coin associated with coin_asset not found, this should not happen");
            APIError::database_error().into_response()
        }
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct CreateCoinAssetRequest {
    coin_id: i64,
    possessed: i64,
}

pub(crate) async fn create_coin_asset(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateCoinAssetRequest>,
) -> Response {
    if request.possessed < 1 {
        return APIError::invalid_value("possessed must be >= 1").into_response();
    }

    match sqlx::query!(
        r#"
            INSERT INTO coin_assets (coin_id, user_id, possessed)
            VALUES ($1, $2, $3)
        "#,
        request.coin_id,
        user_id,
        request.possessed
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => match e {
            sqlx::Error::Database(db_error) if db_error.kind() == ErrorKind::UniqueViolation => {
                APIError::invalid_value(&format!("you already possess coin_id {}", request.coin_id))
            }
            e => {
                log::error!("{e}");
                APIError::database_error()
            }
        }
        .into_response(),
    }
}

#[derive(Deserialize)]
pub(super) struct UpdateCoinAssetRequest {
    possessed: i64,
}

pub(crate) async fn update_coin_asset(
    Path(coin_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateCoinAssetRequest>,
) -> Response {
    let coin_asset = match sqlx::query_as!(
        CoinAsset,
        "SELECT * FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(coin_asset)) => coin_asset,
        Ok(None) => return APIError::bad_id(&coin_id.to_string()).into_response(),
        Err(e) => {
            log::error!("{e}");
            return APIError::database_error().into_response();
        }
    };

    if coin_asset.possessed != request.possessed {
        match sqlx::query!(
            "UPDATE coin_assets SET possessed = $1 WHERE coin_id = $2 AND user_id = $3",
            request.possessed,
            coin_id,
            user_id
        )
        .execute(&pool)
        .await
        {
            Ok(_) => (), // no-op
            Err(e) => {
                log::error!("{e}");
                return APIError::database_error().into_response();
            }
        };
    }

    StatusCode::NO_CONTENT.into_response()
}

pub(crate) async fn delete_coin_asset(
    Path(coin_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match sqlx::query!(
        "DELETE FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}
