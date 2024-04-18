use axum::{Extension, middleware, Router};
use axum::http::Method;
use axum::routing::get;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::middleware::check_api_key::check_api_key;
use crate::routes::health_check::health_check;
use crate::routes::trade_values::{trade_values, trade_values_with_param};

mod health_check;
mod trade_values;

pub fn router(db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/trade_values", get(trade_values))
        .route("/trade_values/:query", get(trade_values_with_param))
        .route_layer(middleware::from_fn(check_api_key))
        .layer(Extension(db))
        .route("/health", get(health_check))
        .layer(cors)
}
