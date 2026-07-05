use crate::helpers::{TestApp, spawn_app};

async fn nuke_coins_table(app: &TestApp) {
    sqlx::query!("DROP TABLE coins").execute(&app.pool).await.unwrap();
}

#[tokio::test]
async fn get_coin_returns_404_when_fetching_a_non_existent_coin() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_coin().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "The provided id is invalid");
}

#[tokio::test]
async fn get_coin_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coins_table(&app).await;

    // Act
    let response = app.get_coin().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch coin");
}

#[tokio::test]
async fn search_coins_returns_400_when_search_query_is_invalid() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.search_coins(" ").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 400);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Invalid search query: ' '");
}

#[tokio::test]
async fn search_coins_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coins_table(&app).await;

    // Act
    let response = app.search_coins("coin").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch coins");
}
