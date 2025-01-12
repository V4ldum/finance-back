use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::database::Database;
use crate::utils::api_error::APIError;
use crate::utils::convert_coin_model_to_coin_response::convert_coin_model_to_coin_response;
use crate::utils::dto::assets_dto::{AssetsDto, CashAssetsDto, CoinAssetsDto, RawAssetsDto};

pub async fn get_assets(State(database): State<Database>, headers: HeaderMap) -> Response {
    let key = headers
        .get("X-API-KEY")
        .expect("The key was confirmed present by the middleware")
        .to_str()
        .expect("The key was confirmed properly formatted by the middleware");

    let Ok(Some(user)) = database.get_user(key).await else {
        return APIError::database_error().into_response();
    };

    let Ok(raw_assets) = database.get_raw_assets(user.id).await else {
        return APIError::database_error().into_response();
    };

    let Ok(cash_assets) = database.get_cash_assets(user.id).await else {
        return APIError::database_error().into_response();
    };

    let Ok(coin_assets_models) = database.get_coin_assets(user.id).await else {
        return APIError::database_error().into_response();
    };

    let mut coins_assets = Vec::with_capacity(coin_assets_models.len());

    for coin_asset_model in coin_assets_models.into_iter() {
        let Ok(coin_model) = database.find_coin(coin_asset_model.coin_id).await else {
            return APIError::database_error().into_response();
        };
        let coin_model = coin_model.expect("coin_id coming from the database should be correct");

        let coin_assets = convert_coin_model_to_coin_response(coin_model, &database).await;

        match coin_assets {
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
