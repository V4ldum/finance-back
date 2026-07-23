use std::time::Duration;

use axum::http::{Method, Request, Response};
use axum::routing::{get, post};
use axum::{Router, middleware};
use axum_extra::routing::RouterExt;
use middleware::from_fn_with_state;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;
use uuid::Uuid;

use crate::middleware::auth::check_api_key;
use crate::routes::{assets, cash_assets, coin_assets, coins, health_check, prices, raw_assets};

pub(crate) fn router(pool: SqlitePool) -> Router {
    // CORS Middleware
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    // Telemetry Middleware
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            // Don't trace the healthcheck endpoint
            if req.uri().path().trim_end_matches('/') == "/health" {
                return tracing::Span::none();
            }
            tracing::info_span!(
                "request",
                request_id = %Uuid::new_v4(),
                method = %req.method(),
                uri = %req.uri(),
            )
        })
        .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
            // Don't trace a disabled span
            if span.is_disabled() {
                return;
            }
            tracing::info!(
                latency_ms = latency.as_millis(),
                status = response.status().as_u16(),
                "finished processing request"
            );
        })
        .on_failure(());

    Router::new()
        .route_with_tsr("/prices", get(prices::get_all_prices))
        .route_with_tsr("/prices/{query}", get(prices::get_one_price))
        .route_with_tsr("/coins/search", get(coins::search_coins))
        .route_with_tsr("/coins/{id}", get(coins::get_coin))
        .route_with_tsr("/assets", get(assets::get_assets))
        .route_with_tsr("/assets/coin", post(coin_assets::create_coin_asset))
        .route_with_tsr(
            "/assets/coin/{id}",
            get(coin_assets::get_coin_asset)
                .patch(coin_assets::update_coin_asset)
                .delete(coin_assets::delete_coin_asset),
        )
        .route_with_tsr("/assets/raw", post(raw_assets::create_raw_asset))
        .route_with_tsr(
            "/assets/raw/{id}",
            get(raw_assets::get_raw_asset)
                .patch(raw_assets::update_raw_asset)
                .delete(raw_assets::delete_raw_asset),
        )
        .route_with_tsr("/assets/cash", post(cash_assets::create_cash_asset))
        .route_with_tsr(
            "/assets/cash/{id}",
            get(cash_assets::get_cash_asset)
                .patch(cash_assets::update_cash_asset)
                .delete(cash_assets::delete_cash_asset),
        )
        // Anything above needs authentication
        .route_layer(from_fn_with_state(pool.clone(), check_api_key))
        // Anything above can use the state
        .with_state(pool)
        .route_with_tsr("/health", get(health_check::health_check))
        .layer(cors)
        .layer(trace_layer)
}
