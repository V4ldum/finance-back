mod configuration;
mod domain;
mod middleware;
mod model;
mod routes;
pub mod telemetry;
mod utils;

use std::str::FromStr;

pub use configuration::Configuration;
pub use configuration::get_configuration;
use sqlx::sqlite::SqliteConnectOptions;

use crate::routes::router;
use anyhow::Result;
use axum::Router;
use axum::serve::Serve;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

type Server = Serve<TcpListener, Router, Router>;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Configuration) -> Result<Self> {
        // Setup the database connection
        let pool = get_connection_pool(&configuration.database_url, &configuration.sqlite_extension).await?;

        // Bind the listener to the IP and port
        let address = format!("{}:{}", configuration.application_host, configuration.application_port);
        tracing::info!("Serving {address}");
        let listener = TcpListener::bind(&address).await?;
        let port = listener.local_addr().expect("Failed to get local address").port();

        // Serve the API
        let server = run(listener, pool);

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await?;
        Ok(())
    }
}

pub async fn get_connection_pool(database_url: &str, extension: &str) -> Result<SqlitePool> {
    // Setup the database connection
    let options = SqliteConnectOptions::from_str(database_url)?
        .extension(extension.to_owned())
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePool::connect_with(options).await?;

    // Automatically migrate the database
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

fn run(listener: TcpListener, pool: SqlitePool) -> Server {
    let router = router(pool);
    axum::serve(listener, router)
}
