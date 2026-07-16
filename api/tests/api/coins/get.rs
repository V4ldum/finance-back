use crate::{
    coins::{insert_coin_with_name, nuke_coins_table},
    helpers::{name, spawn_app},
};

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
async fn get_coin_returns_the_correct_coin() {
    // Arrange
    let app = spawn_app().await;

    let coin_name = name();
    insert_coin_with_name(&app, &coin_name).await;

    // Act
    let response = app.get_coin().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["id"], 1);
    assert_eq!(json_response["name"], coin_name);
}
