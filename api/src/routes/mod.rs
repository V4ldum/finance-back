use crate::middleware::auth::check_api_key;
use axum::http::{Method, Request, Response};
use axum::routing::{get, post};
use axum::{Router, middleware};
use middleware::from_fn_with_state;
use sqlx::SqlitePool;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;
use uuid::Uuid;

mod assets;
mod cash_assets;
mod coin_assets;
mod coins;
mod health_check;
mod prices;
mod raw_assets;

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
                latency_ms = latency.as_millis(),
                status = response.status().as_u16(),
                "finished processing request"
            );
        })
        .on_failure(());

    Router::new()
        .route("/prices", get(prices::get_all_prices))
        .route("/prices/{query}", get(prices::get_one_price))
        .route("/coins/search", get(coins::search_coins))
        .route("/coins/{id}", get(coins::get_coin))
        .route("/assets", get(assets::get_assets))
        .route("/assets/coin", post(coin_assets::create_coin_asset))
        .route(
            "/assets/coin/{id}",
            get(coin_assets::get_coin_asset)
                .patch(coin_assets::update_coin_asset)
                .delete(coin_assets::delete_coin_asset),
        )
        .route("/assets/raw", post(raw_assets::create_raw_asset))
        .route(
            "/assets/raw/{id}",
            get(raw_assets::get_raw_asset)
                .patch(raw_assets::update_raw_asset)
                .delete(raw_assets::delete_raw_asset),
        )
        .route("/assets/cash", post(cash_assets::create_cash_asset))
        .route(
            "/assets/cash/{id}",
            get(cash_assets::get_cash_asset)
                .patch(cash_assets::update_cash_asset)
                .delete(cash_assets::delete_cash_asset),
        )
        // Anything above needs authentication
        .route_layer(from_fn_with_state(pool.clone(), check_api_key))
        // Anything above can use the state
        .with_state(pool)
        .route("/health", get(health_check::health_check))
        .layer(cors)
        .layer(trace_layer)
}
