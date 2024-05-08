use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;

use crate::database::Database;
use crate::util::api_error::APIError;
use crate::util::dto::assets_dto::CashAssetsDto;
use crate::util::get_user_id_from_headers::get_user_id_from_headers;

pub async fn get_cash_asset(
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

    let Ok(cash_asset_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(asset) = database.find_cash_asset(cash_asset_id, user_id).await else {
        return APIError::database_error().into_response();
    };

    let Some(asset) = asset else {
        return APIError::bad_id(&cash_asset_id.to_string()).into_response();
    };

    Json(CashAssetsDto {
        id: asset.id,
        name: asset.name,
        possessed: asset.possessed,
        unit_value: asset.unit_value,
    })
    .into_response()
}

#[derive(Deserialize)]
pub struct CreateCashAssetRequest {
    name: String,
    possessed: i32,
    unit_value: i32,
}

pub async fn create_cash_asset(
    State(database): State<Database>,
    headers: HeaderMap,
    Json(request): Json<CreateCashAssetRequest>,
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

    if request.unit_value < 0 {
        return APIError::invalid_value("unit_value must be > 0").into_response();
    }

    let Ok(_) = database
        .add_cash_asset(request.name, request.possessed, request.unit_value, user_id)
        .await
    else {
        return APIError::database_error().into_response();
    };

    StatusCode::CREATED.into_response()
}

pub async fn update_cash_asset(
    Path(id): Path<String>,
    State(database): State<Database>,
    headers: HeaderMap,
) -> Response {
    todo!()
}

pub async fn delete_cash_asset(
    Path(id): Path<String>,
    State(database): State<Database>,
    headers: HeaderMap,
) -> Response {
    todo!()
}
