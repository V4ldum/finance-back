mod database;
mod middleware;
mod routes;
pub mod state;
mod utils;

pub use crate::database::Database;

use crate::routes::router;
use crate::state::AppState;
use log::info;

pub async fn run(state: AppState) {
    const IP: &str = "0.0.0.0";
    const PORT: &str = "7878";

    let router = router(state);
    let listener = tokio::net::TcpListener::bind(format!("{IP}:{PORT}"))
        .await
        .expect("The listener should be able to bind to this port");

    info!("Serving {IP}:{PORT}");
    axum::serve(listener, router)
        .await
        .expect("The server should launch successfully");
}
