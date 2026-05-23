use std::time::Duration;

use self::cash_assets::{create_cash_asset, get_cash_asset};
use self::coin_assets::{create_coin_asset, get_coin_asset};
use self::raw_assets::{create_raw_asset, get_raw_asset};
use crate::middleware::check_api_key::check_api_key;
use crate::routes::assets::get_assets;
use crate::routes::cash_assets::{delete_cash_asset, update_cash_asset};
use crate::routes::coin_assets::{delete_coin_asset, update_coin_asset};
use crate::routes::coins::{get_coin, search_coin};
use crate::routes::health_check::health_check;
use crate::routes::raw_assets::{delete_raw_asset, update_raw_asset};
use crate::routes::trade_values::{get_all_trade_values, get_one_trade_value};
use axum::http::{Method, Request, Response};
use axum::routing::{get, post};
use axum::{Router, middleware};
use middleware::from_fn_with_state;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;
use uuid::Uuid;

mod assets;
mod cash_assets;
mod coin_assets;
mod coins;
mod health_check;
mod raw_assets;
mod trade_values;

pub(crate) fn router(pool: SqlitePool) -> Router {
    // CORS Middleware
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    // Telemetry Middleware
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            tracing::info_span!(
                "request",
                request_id = %Uuid::new_v4(),
                method = %req.method(),
                uri = %req.uri(),
            )
        })
        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            tracing::info!(
                latency_ms = latency.as_millis() as u64,
                status = response.status().as_u16(),
                "finished processing request"
            );
        });

    Router::new()
        .route("/trade_values", get(get_all_trade_values))
        .route("/trade_values/{query}", get(get_one_trade_value))
        .route("/coins/search", get(search_coin))
        .route("/coins/{id}", get(get_coin))
        .route("/assets", get(get_assets))
        .route("/assets/coin", post(create_coin_asset))
        .route(
            "/assets/coin/{id}",
            get(get_coin_asset).patch(update_coin_asset).delete(delete_coin_asset),
        )
        .route("/assets/raw", post(create_raw_asset))
        .route(
            "/assets/raw/{id}",
            get(get_raw_asset).patch(update_raw_asset).delete(delete_raw_asset),
        )
        .route("/assets/cash", post(create_cash_asset))
        .route(
            "/assets/cash/{id}",
            get(get_cash_asset).patch(update_cash_asset).delete(delete_cash_asset),
        )
        .route_layer(from_fn_with_state(pool.clone(), check_api_key))
        .with_state(pool)
        .route("/health", get(health_check))
        .layer(cors)
        .layer(trace_layer)
}
