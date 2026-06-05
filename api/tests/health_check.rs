use api::{
    Configuration,
    telemetry::{SubscriberConfig, get_subscriber, init_subscriber},
};
use fake::{Fake, faker::lorem::en::Sentence};
use reqwest::Method;
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
        .expect("Failed to insert user into database");

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
async fn auth_middleware_returns_401_for_missing_api_key() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/trade_values", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn auth_middleware_returns_401_for_invalid_api_key() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/trade_values", app.address))
        .header("X-API-KEY", "invalid")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn auth_middleware_protects_all_routes() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let protected_routes = [
        (Method::GET, "/trade_values"),
        (Method::GET, "/trade_values/gold"),
        (Method::GET, "/coins/search"),
        (Method::GET, "/coins/1"),
        (Method::GET, "/assets"),
        (Method::POST, "/assets/coin"),
        (Method::GET, "/assets/coin/1"),
        (Method::PATCH, "/assets/coin/1"),
        (Method::DELETE, "/assets/coin/1"),
        (Method::POST, "/assets/raw"),
        (Method::GET, "/assets/raw/1"),
        (Method::PATCH, "/assets/raw/1"),
        (Method::DELETE, "/assets/raw/1"),
        (Method::POST, "/assets/cash"),
        (Method::GET, "/assets/cash/1"),
        (Method::PATCH, "/assets/cash/1"),
        (Method::DELETE, "/assets/cash/1"),
    ];

    for (method, route) in protected_routes {
        // Act
        let request = match method {
            Method::GET => client.get(format!("{}{}", app.address, route)),
            Method::POST => client.post(format!("{}{}", app.address, route)),
            Method::PATCH => client.patch(format!("{}{}", app.address, route)),
            Method::DELETE => client.delete(format!("{}{}", app.address, route)),
            _ => panic!("Unsupported method"),
        };
        let response = request.send().await.expect("Failed to execute request");

        // Assert
        assert_eq!(response.status().as_u16(), 401);
    }
}

#[tokio::test]
async fn create_cash_asset_returns_201_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_value = (1..1000).fake::<i64>();

    // Act
    let json = json!({
        "name": name,
        "possessed": possessed,
        "unit_value": unit_value,
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

    assert_eq!(saved.name, name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_value, unit_value);
}

#[tokio::test]
async fn create_cash_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_value = (1..1000).fake::<i64>();

    let test_cases = vec![
        (
            json!({
                "possessed": possessed,
                "unit_value": unit_value,
            }),
            "missing name",
        ),
        (
            json!({
                "name": name,
                "unit_value": unit_value,
            }),
            "missing possessed",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
            }),
            "missing unit_value",
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

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_value = (1..1000).fake::<i64>();

    let test_cases = vec![
        (
            json!({
                "name": "",
                "possessed": possessed,
                "unit_value": unit_value,
            }),
            "name was incorrect",
        ),
        (
            json!({
                "name": name,
                "possessed": -1,
                "unit_value": unit_value,
            }),
            "possessed was negative",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
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

#[tokio::test]
async fn create_raw_asset_returns_201_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_weight = (1..1000).fake::<i64>();
    let composition = ["GOLD", "SILVER"][(0..2).fake::<usize>()];
    let purity = (1..=9999).fake::<i64>();

    // Act
    let json = json!({
        "name": name,
        "possessed": possessed,
        "unit_weight": unit_weight,
        "composition": composition,
        "purity": purity,
    });
    let response = client
        .post(format!("{}/assets/raw", app.address))
        .header("X-API-KEY", "123")
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 201);

    let saved = sqlx::query!("SELECT name, possessed, unit_weight, composition, purity FROM raw_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch raw_assets");

    assert_eq!(saved.name, name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_weight, unit_weight);
    assert_eq!(saved.composition, composition);
    assert_eq!(saved.purity, purity);
}

#[tokio::test]
async fn create_raw_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_weight = (1..1000).fake::<i64>();
    let composition = ["GOLD", "SILVER"][(0..2).fake::<usize>()];
    let purity = (1..=9999).fake::<i64>();

    let test_cases = vec![
        (
            json!({
                "possessed": possessed,
                "unit_weight": unit_weight,
                "composition": composition,
                "purity": purity,
            }),
            "missing name",
        ),
        (
            json!({
                "name": name,
                "unit_weight": unit_weight,
                "composition": composition,
                "purity": purity,
            }),
            "missing possessed",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "composition": composition,
                "purity": purity,
            }),
            "missing unit_weight",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "unit_weight": unit_weight,
                "purity": purity,
            }),
            "missing composition",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "unit_weight": unit_weight,
                "composition": composition,
            }),
            "missing purity",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/assets/raw", app.address))
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
async fn create_raw_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_weight = (1..1000).fake::<i64>();
    let composition = ["GOLD", "SILVER"][(0..2).fake::<usize>()];
    let purity = (1..=9999).fake::<i64>();

    let test_cases = vec![
        (
            json!({
                "name": "",
                "possessed": possessed,
                "unit_weight": unit_weight,
                "composition": composition,
                "purity": purity,
            }),
            "name was incorrect",
        ),
        (
            json!({
                "name": name,
                "possessed": -1,
                "unit_weight": unit_weight,
                "composition": composition,
                "purity": purity,
            }),
            "possessed was negative",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "unit_weight": -20,
                "composition": composition,
                "purity": purity,
            }),
            "unit_weight was negative",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "unit_weight": unit_weight,
                "composition": "Incorrect",
                "purity": purity,
            }),
            "composition was incorrect",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "unit_weight": unit_weight,
                "composition": composition,
                "purity": 10000,
            }),
            "purity was outside of the 1..9999 range",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/assets/raw", app.address))
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
