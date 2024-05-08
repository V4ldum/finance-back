use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DbErr;
use serde::Deserialize;

use crate::database::Database;
use crate::util::api_error::APIError;
use crate::util::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::util::dto::assets_dto::CoinAssetsDto;
use crate::util::get_user_id_from_headers::get_user_id_from_headers;

pub async fn get_coin_asset(
    Path(id): Path<String>,
    State(database): State<Database>,
    headers: HeaderMap,
) -> Response {
    let user_id = match get_user_id_from_headers(&headers, &database).await {
        Ok(user_id) => user_id,
        Err(err) => {
            return err.into_response();
        }
    };

    let Ok(coin_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(coin) = database.find_coin_asset(coin_id, user_id).await else {
        return APIError::database_error().into_response();
    };

    let Some(coin_asset) = coin else {
        return APIError::bad_id(&coin_id.to_string()).into_response();
    };

    let Ok(Some(coin_data)) = database.find_coin(coin_asset.coin_id).await else {
        return APIError::database_error().into_response();
    };

    let coin_data = convert_coin_model_to_coin_response(coin_data, &database).await;

    let coin_data = match coin_data {
        Ok(coin_data) => coin_data,
        Err(error) => {
            return error.into_response();
        }
    };

    Json(CoinAssetsDto {
        possessed: coin_asset.possessed,
        coin_data,
    })
    .into_response()
}

#[derive(Deserialize)]
pub struct CreateCoinAssetRequest {
    coin_id: i32,
    possessed: i32,
}

pub async fn create_coin_asset(
    State(database): State<Database>,
    headers: HeaderMap,
    Json(request): Json<CreateCoinAssetRequest>,
) -> Response {
    let user_id = match get_user_id_from_headers(&headers, &database).await {
        Ok(user_id) => user_id,
        Err(err) => {
            return err.into_response();
        }
    };

    if request.possessed < 1 {
        return APIError::invalid_value("possessed must be > 1").into_response();
    }

    let result = database
        .add_coin_asset(request.coin_id, user_id, request.possessed)
        .await;

    if result.is_err() {
        return match result.err().unwrap() {
            DbErr::Exec(_) => {
                APIError::invalid_value(&format!("you already possess coin_id {}", request.coin_id))
            }
            _ => APIError::database_error(),
        }
        .into_response();
    }

    StatusCode::CREATED.into_response()
}

#[derive(Deserialize)]
pub struct UpdateCoinAssetRequest {
    possessed: i32,
}

pub async fn update_coin_asset(
    Path(id): Path<String>,
    State(database): State<Database>,
    headers: HeaderMap,
    Json(request): Json<UpdateCoinAssetRequest>,
) -> Response {
    let user_id = match get_user_id_from_headers(&headers, &database).await {
        Ok(user_id) => user_id,
        Err(err) => {
            return err.into_response();
        }
    };

    let Ok(coin_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(coin_asset) = database.find_coin_asset(coin_id, user_id).await else {
        return APIError::database_error().into_response();
    };

    let Some(coin_asset) = coin_asset else {
        return APIError::bad_id(&coin_id.to_string()).into_response();
    };

    if coin_asset.possessed != request.possessed {
        let Ok(_) = database
            .update_coin_asset(coin_id, user_id, request.possessed)
            .await
        else {
            return APIError::database_error().into_response();
        };
    }

    StatusCode::NO_CONTENT.into_response()
}

pub async fn delete_coin_asset(
    Path(id): Path<String>,
    State(database): State<Database>,
    headers: HeaderMap,
) -> Response {
    let user_id = match get_user_id_from_headers(&headers, &database).await {
        Ok(user_id) => user_id,
        Err(err) => {
            return err.into_response();
        }
    };

    let Ok(coin_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(_) = database.delete_coin_asset(coin_id, user_id).await else {
        return APIError::database_error().into_response();
    };

    StatusCode::NO_CONTENT.into_response()
}
