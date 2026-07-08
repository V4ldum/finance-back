use crate::{coins::nuke_coins_table, helpers::spawn_app};

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
