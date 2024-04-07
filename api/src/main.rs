use axum::{Router, routing::get};

use finance_api::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/trade_values", get(trade_values))
        .route("/trade_values/:value", get(trade_value));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("The listener should be able to bind to this port");
    axum::serve(listener, app)
        .await
        .expect("The server should launch successfully");
}
