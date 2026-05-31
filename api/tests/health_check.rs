use api::{
    Configuration,
    telemetry::{SubscriberConfig, get_subscriber, init_subscriber},
};
use serde_json::json;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{str::FromStr, sync::LazyLock};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use uuid::Uuid;

struct TestApp {
    address: String,
    pool: SqlitePool,
}

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
        .expect("Failed to insert into database");

    pool
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

async fn spawn_app() -> TestApp {
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

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn create_cash_asset_returns_201_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let json = json!({
        "name": "20 €",
        "possessed": 1,
        "unit_value": 20,
    });
    let response = client
        .post(format!("{}/assets/cash", app.address))
        .header("X-API-KEY", "123")
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 201);

    let saved = sqlx::query!("SELECT name, possessed, unit_value FROM cash_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch cash_assets");
    assert_eq!(saved.name, "20 €");
    assert_eq!(saved.possessed, 1);
    assert_eq!(saved.unit_value, 20);
}

#[tokio::test]
async fn create_cash_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        (
            json!({
                "possessed": 1,
                "unit_value": 20,
            }),
            "missing name",
        ),
        (
            json!({
                "name": "20 €",
                "unit_value": 20,
            }),
            "missing possessed",
        ),
        (
            json!({
                "name": "20 €",
                "possessed": 1,
            }),
            "missing unit_value",
        ),
        (
            json!({
                "unit_value": 20,
            }),
            "missing name and possessed",
        ),
        (
            json!({
                "possessed": 1,
            }),
            "missing name and unit_value",
        ),
        (
            json!({
                "name": "20 €",
            }),
            "missing possessed and unit_value",
        ),
        (json!({}), "missing name, possessed and unit_value"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/assets/cash", app.address))
            .header("X-API-KEY", "123")
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            response.status().as_u16(),
            422,
            "The API did not fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}

#[tokio::test]
async fn create_cash_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        (
            json!({
                "name": " ",
                "possessed": 1,
                "unit_value": 20,
            }),
            "name was empty",
        ),
        (
            json!({
                "name": "20 €",
                "possessed": -1,
                "unit_value": 20,
            }),
            "possessed was negative",
        ),
        (
            json!({
                "name": "20 €",
                "possessed": 0,
                "unit_value": 20,
            }),
            "possessed was zero",
        ),
        (
            json!({
                "name": "20 €",
                "possessed": 1,
                "unit_value": -20,
            }),
            "unit_value was negative",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/assets/cash", app.address))
            .header("X-API-KEY", "123")
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when {error_message}"
        );
    }
}
