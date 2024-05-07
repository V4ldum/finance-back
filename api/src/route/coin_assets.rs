use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::database::Database;
use crate::util::api_error::APIError;
use crate::util::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::util::dto::assets_dto::CoinAssetsDto;

pub async fn get_coin_assets(
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

    let Ok(coin_id) = id.parse::<i32>() else {
        return APIError::bad_id(&id).into_response();
    };

    let Ok(coin) = database.find_coin_asset(coin_id, user.id).await else {
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
