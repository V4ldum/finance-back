use axum::http::Method;
use axum::routing::get;
use axum::{middleware, Router};
use middleware::from_fn_with_state;
use tower_http::cors::{Any, CorsLayer};

use crate::middleware::check_api_key::check_api_key;
use crate::route::assets::get_assets;
use crate::route::cash_assets::get_cash_assets;
use crate::route::coin_assets::get_coin_assets;
use crate::route::coins::{get_coin, search_coin};
use crate::route::health_check::health_check;
use crate::route::raw_assets::get_raw_assets;
use crate::route::trade_values::{get_all_trade_values, get_one_trade_value};
use crate::state::AppState;

mod assets;
mod cash_assets;
mod coin_assets;
mod coins;
mod health_check;
mod raw_assets;
mod trade_values;

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/trade_values", get(get_all_trade_values))
        .route("/trade_values/:query", get(get_one_trade_value))
        .route("/coins/search", get(search_coin))
        .route("/coins/:id", get(get_coin))
        .route("/assets", get(get_assets))
        .route("/assets/coin/:id", get(get_coin_assets))
        .route("/assets/raw/:id", get(get_raw_assets))
        .route("/assets/cash/:id", get(get_cash_assets))
        .route_layer(from_fn_with_state(state.clone(), check_api_key))
        .with_state(state)
        .route("/health", get(health_check))
        .layer(cors)
}
