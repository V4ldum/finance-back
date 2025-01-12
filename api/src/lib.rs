use crate::state::AppState;

mod middleware;

mod database;
mod routes;
pub mod state;
mod utils;

pub use crate::database::Database;
use crate::routes::router;

pub async fn run(state: AppState) {
    let router = router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("The listener should be able to bind to this port");

    axum::serve(listener, router)
        .await
        .expect("The server should launch successfully");
}
