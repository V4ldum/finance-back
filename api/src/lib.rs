mod database;
mod middleware;
mod routes;
pub mod state;
mod utils;

pub use crate::database::Database;

use crate::routes::router;
use crate::state::AppState;
use anyhow::Result;
use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;

pub fn run(state: AppState, listener: TcpListener) -> Result<Serve<TcpListener, Router, Router>> {
    let router = router(state);
    let server = axum::serve(listener, router);

    Ok(server)
}
