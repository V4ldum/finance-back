use reqwest::Method;

use crate::helpers::spawn_app;

#[tokio::test]
async fn auth_middleware_returns_401_for_missing_api_key() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_auth_middleware(None).await;

    // Assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn auth_middleware_returns_401_for_invalid_api_key() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_auth_middleware(Some("invalid")).await;

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
