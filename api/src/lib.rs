use crate::route::router;
use crate::state::AppState;

pub mod database;
mod middleware;
mod route;

pub mod state;
mod util;

pub async fn run(state: AppState) {
    let router = router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("The listener should be able to bind to this port");

    axum::serve(listener, router)
        .await
        .expect("The server should launch successfully");
}
