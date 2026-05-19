mod configuration;
mod middleware;
mod model;
mod routes;
mod utils;

pub use configuration::Configuration;
pub use configuration::get_configuration;

use crate::routes::router;
use anyhow::Result;
use axum::Router;
use axum::serve::Serve;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

pub fn run(listener: TcpListener, pool: SqlitePool) -> Result<Serve<TcpListener, Router, Router>> {
    let router = router(pool);
    let server = axum::serve(listener, router);

    Ok(server)
}
