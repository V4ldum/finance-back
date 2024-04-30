use axum::extract::{Query, State};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::utils::api_error::APIError;

#[derive(Deserialize)]
pub struct QueryParams {
    q: String,
}

#[derive(Serialize)]
struct CoinResponse {
    id: i32,
    numista_id: String,
    name: String,
    weight: f64,
    size: f64,
    thickness: Option<f64>,
    min_year: String,
    max_year: Option<String>,
    composition: String,
    purity: i32,
    obverse: Option<CoinSideResponse>,
    reverse: Option<CoinSideResponse>,
    edge: Option<CoinSideResponse>,
}

#[derive(Serialize)]
struct CoinSideResponse {
    pub image_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub lettering: Option<String>,
    pub description: Option<String>,
    pub copyright: Option<String>,
}

pub async fn search_coin(
    Query(query): Query<QueryParams>,
    State(database): State<Database>,
) -> Response {
    let Ok(coins) = database.search_coin(&query.q).await else {
        return APIError::database_error().into_response();
    };

    let mut coins_response = Vec::with_capacity(coins.len());

    for coin in coins.into_iter() {
        let obverse = if let Some(obverse) = coin.obverse {
            let Ok(result) = database.get_coin_side(obverse).await else {
                return APIError::database_error().into_response();
            };

            result
        } else {
            None
        };
        let reverse = if let Some(reverse) = coin.reverse {
            let Ok(result) = database.get_coin_side(reverse).await else {
                return APIError::database_error().into_response();
            };

            result
        } else {
            None
        };
        let edge = if let Some(edge) = coin.edge {
            let Ok(result) = database.get_coin_side(edge).await else {
                return APIError::database_error().into_response();
            };

            result
        } else {
            None
        };

        let coin_response = CoinResponse {
            id: coin.id,
            numista_id: coin.numista_id,
            name: coin.name,
            weight: coin.weight,
            size: coin.size,
            thickness: coin.thickness,
            min_year: coin.min_year,
            max_year: coin.max_year,
            composition: coin.composition,
            purity: coin.purity,
            obverse: obverse.map(|obverse| CoinSideResponse {
                image_url: obverse.image_url,
                thumbnail_url: obverse.thumbnail_url,
                lettering: obverse.lettering,
                description: obverse.description,
                copyright: obverse.copyright,
            }),
            reverse: reverse.map(|reverse| CoinSideResponse {
                image_url: reverse.image_url,
                thumbnail_url: reverse.thumbnail_url,
                lettering: reverse.lettering,
                description: reverse.description,
                copyright: reverse.copyright,
            }),
            edge: edge.map(|edge| CoinSideResponse {
                image_url: edge.image_url,
                thumbnail_url: edge.thumbnail_url,
                lettering: edge.lettering,
                description: edge.description,
                copyright: edge.copyright,
            }),
        };

        coins_response.push(coin_response);
    }

    Json(coins_response).into_response()
}
