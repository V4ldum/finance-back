use axum::{middleware, Router};
use axum::http::Method;
use axum::routing::get;
use middleware::from_fn_with_state;
use tower_http::cors::{Any, CorsLayer};

use crate::middleware::check_api_key::check_api_key;
use crate::routes::health_check::health_check;
use crate::routes::trade_values::{get_all_trade_values, get_one_trade_value};
use crate::state::AppState;

mod health_check;
mod trade_values;

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/trade_values", get(get_all_trade_values))
        .route("/trade_values/:query", get(get_one_trade_value))
        .route_layer(from_fn_with_state(state.clone(), check_api_key))
        .with_state(state)
        .route("/health", get(health_check))
        .layer(cors)
}
