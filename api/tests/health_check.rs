use api::{Database, state::AppState};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}

async fn spawn_app() -> String {
    let database = SqlitePool::connect_with(
        SqliteConnectOptions::from_str("../test.db")
            .expect("Failed to create SqliteConnectOptions")
            .create_if_missing(true),
    )
    .await
    .expect("Failed to connect to database");

    let state = AppState {
        database: Database::new(database),
    };

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = api::run(state, listener).expect("Failed to bind address");
    tokio::spawn(server.into_future());

    format!("http://127.0.0.1:{port}")
}
