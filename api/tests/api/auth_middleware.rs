use reqwest::Method;

use crate::helpers::{TestApp, spawn_app};

async fn nuke_users_table(app: &TestApp) {
    sqlx::query!("ALTER TABLE users RENAME COLUMN api_key TO dropped")
        .execute(&app.pool)
        .await
        .expect("Failed to rename api_key column");
}

#[tokio::test]
async fn auth_middleware_returns_401_for_missing_api_key() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_auth_middleware(None).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 401);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "X-API-KEY header not provided");
}

#[tokio::test]
async fn auth_middleware_returns_401_for_invalid_api_key() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_auth_middleware(Some("invalid")).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 401);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Invalid X-API-KEY provided");
}

#[tokio::test]
async fn auth_middleware_returns_401_for_non_ascii_api_key() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_auth_middleware(Some("無効")).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 401);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Invalid X-API-KEY provided");
}

#[tokio::test]
async fn auth_middleware_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_users_table(&app).await;

    // Act
    let response = app.get_auth_middleware(Some("123")).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch api key");
}

#[tokio::test]
async fn auth_middleware_protects_all_routes() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let protected_routes: Vec<(&[Method], &str)> = vec![
        (&[Method::GET], "/prices"),
        (&[Method::GET], "/prices/gold"),
        (&[Method::GET], "/coins/search"),
        (&[Method::GET], "/coins/1"),
        (&[Method::GET], "/assets"),
        (&[Method::POST], "/assets/coin"),
        (&[Method::GET, Method::PATCH, Method::DELETE], "/assets/coin/1"),
        (&[Method::POST], "/assets/raw"),
        (&[Method::GET, Method::PATCH, Method::DELETE], "/assets/raw/1"),
        (&[Method::POST], "/assets/cash"),
        (&[Method::GET, Method::PATCH, Method::DELETE], "/assets/cash/1"),
    ];

    for (methods, route) in protected_routes {
        for method in methods {
            // Act
            let response = client
                .request(method.clone(), format!("{}{}", app.address, route))
                .send()
                .await
                .expect("Failed to execute request");

            // Assert
            assert_eq!(response.status().as_u16(), 401);
        }
    }
}
