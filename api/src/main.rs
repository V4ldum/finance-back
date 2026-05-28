use anyhow::Result;
use api::get_configuration;
use api::run;
use api::telemetry::SubscriberConfig;
use api::telemetry::get_subscriber;
use api::telemetry::init_subscriber;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup telemetry
    let subscriber = get_subscriber(SubscriberConfig {
        service: "finance".into(),
        json_filter: LevelFilter::INFO,
        json_sink: std::io::stdout,
        text_filter: LevelFilter::WARN,
        text_sink: std::io::stderr,
    });
    init_subscriber(subscriber);

    // Read configuration from environment variables
    let configuration = get_configuration().expect("Failed to read configuration");

    // Setup the database connection
    let options = SqliteConnectOptions::from_str(&configuration.database_url)?
        .extension("libsqlite3_unaccent")
        .foreign_keys(true);
    let pool = SqlitePool::connect_with(options).await?;

    // Automatically migrate the database
    sqlx::migrate!().run(&pool).await?;

    // Bind the listener to the IP and port
    let address = format!("{}:{}", configuration.application_host, configuration.application_port);
    tracing::info!("Serving {address}");
    let listener = TcpListener::bind(&address)
        .await
        .expect("The listener should be able to bind to this port");

    // Serve the API
    run(listener, pool)?.await?;

    Ok(())
}
