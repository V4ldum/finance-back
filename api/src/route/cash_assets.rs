use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::database::Database;
use crate::util::api_error::APIError;
use crate::util::dto::assets_dto::CashAssetsDto;

pub async fn get_cash_assets(
    Path(id): Path<String>,
    State(database): State<Database>,
    headers: HeaderMap,
) -> Response {
    let key = headers
        .get("X-API-KEY")
        .expect("The key was confirmed present by the middleware")
        .to_str()
        .expect("The key was confirmed properly formatted by the middleware");

    let Ok(Some(user)) = database.get_user(key).await else {
        return APIError::database_error().into_response();
    };

    let Ok(cash_asset_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(asset) = database.find_cash_asset(cash_asset_id, user.id).await else {
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
