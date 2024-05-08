use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::database::Database;
use crate::util::api_error::APIError;
use crate::util::dto::assets_dto::RawAssetsDto;
use crate::util::get_user_id_from_headers::get_user_id_from_headers;

pub async fn get_raw_asset(
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

    let Ok(raw_asset_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(asset) = database.find_raw_asset(raw_asset_id, user_id).await else {
        return APIError::database_error().into_response();
    };

    let Some(asset) = asset else {
        return APIError::bad_id(&raw_asset_id.to_string()).into_response();
    };

    Json(RawAssetsDto {
        id: asset.id,
        name: asset.name,
        possessed: asset.possessed,
        unit_weight: asset.unit_weight,
        composition: asset.composition,
        purity: asset.purity,
    })
    .into_response()
}

#[derive(Deserialize)]
pub struct CreateRawAssetRequest {
    name: String,
    possessed: i32,
    unit_weight: i32,
    composition: String,
    purity: i32,
}

pub async fn create_raw_asset(
    State(database): State<Database>,
    headers: HeaderMap,
    Json(request): Json<CreateRawAssetRequest>,
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

    if request.unit_weight < 0 {
        return APIError::invalid_value("unit_value must be > 0").into_response();
    }

    if request.composition != "GOLD" && request.composition != "SILVER" {
        return APIError::invalid_value("composition can either be \"GOLD\" or \"SILVER\"")
            .into_response();
    }

    if request.purity > 9999 || request.purity < 1 {
        return APIError::invalid_value("purity must be between 1 and 9999").into_response();
    }

    let Ok(_) = database
        .add_raw_asset(
            request.name,
            request.possessed,
            request.unit_weight,
            request.composition,
            request.purity,
            user_id,
        )
        .await
    else {
        return APIError::database_error().into_response();
    };

    StatusCode::CREATED.into_response()
}
