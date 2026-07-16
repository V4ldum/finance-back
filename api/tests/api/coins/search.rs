use claims::assert_some;

use crate::{
    coins::{insert_coin_by_name, nuke_coins_table},
    helpers::{name, spawn_app},
};

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
    let response = app.search_coins("coins").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch coins");
}

#[tokio::test]
async fn search_coins_returns_the_correct_coins() {
    // Arrange
    let app = spawn_app().await;

    let coin_name = name();
    let coin_name_variant1 = format!("{coin_name}{}", name());
    let coin_name_variant2 = format!("{coin_name}{}", name());
    insert_coin_by_name(&app, &coin_name_variant1).await;
    insert_coin_by_name(&app, &coin_name_variant2).await;
    insert_coin_by_name(&app, &name()).await;

    // Act
    let response = app.search_coins(&coin_name).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    let array = assert_some!(json_response.as_array());
    assert_eq!(array.len(), 2);
    assert!(array.iter().any(|v| v["name"] == coin_name_variant1));
    assert!(array.iter().any(|v| v["name"] == coin_name_variant2));
}
