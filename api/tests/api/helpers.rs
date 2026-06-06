use std::{str::FromStr, sync::LazyLock};

use api::{
    Configuration,
    telemetry::{SubscriberConfig, get_subscriber, init_subscriber},
};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: SqlitePool,
}

// Ensure the telemetry stack is only initialized once
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = LevelFilter::INFO;
    let subscriber_name = "test".to_string();

    // If the environment variable TEST_LOG is set, output tracing to stdout, otherwise don't output it
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(SubscriberConfig {
            service: subscriber_name,
            json_filter: default_filter_level,
            json_sink: std::io::sink,
            text_filter: default_filter_level,
            text_sink: std::io::stdout,
        });
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(SubscriberConfig {
            service: subscriber_name,
            json_filter: default_filter_level,
            json_sink: std::io::sink,
            text_filter: default_filter_level,
            text_sink: std::io::sink,
        });
        init_subscriber(subscriber);
    };
});

async fn configure_database(config: &Configuration) -> SqlitePool {
    // Create a connection pool to the database
    let options = SqliteConnectOptions::from_str(&config.database_url)
        .expect("Failed to create SqliteConnectOptions")
        .foreign_keys(true);
    let pool = SqlitePool::connect_with(options)
        .await
        .expect("Failed to connect to database");

    // Migrate the database
    sqlx::migrate!().run(&pool).await.expect("Failed to migrate database");

    // Insert a test user into the database
    sqlx::query!("INSERT OR IGNORE INTO users(api_key) VALUES ('123')")
        .execute(&pool)
        .await
        .expect("Failed to insert user into database");

    pool
}

pub async fn spawn_app() -> TestApp {
    // Setup telemetry
    LazyLock::force(&TRACING);

    // Bind to a random port and get the address
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    // Create the configuration and set up the database
    let configuration = Configuration {
        database_url: format!("sqlite:file:memdb-{}?mode=memory&cache=shared", Uuid::new_v4()),
        application_host: "127.0.0.1".to_string(),
        application_port: port,
    };
    let pool = configure_database(&configuration).await;

    // Run the server in the background
    let server = api::run(listener, pool.clone()).expect("Failed to bind address");
    tokio::spawn(server.into_future());

    TestApp { address, pool }
}
