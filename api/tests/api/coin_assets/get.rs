use crate::{
    coin_assets::{insert_coin_asset, nuke_coin_assets_table},
    helpers::{name, possessed, spawn_app},
};

#[tokio::test]
async fn get_coin_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coin_assets_table(&app).await;

    // Act
    let response = app.get_coin_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch coin asset");
}

#[tokio::test]
async fn get_coin_asset_returns_404_when_id_is_not_in_database() {
    // Arrange
    let app = spawn_app().await;
    // No data in db by default

    // Act
    let response = app.get_coin_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "The provided id is invalid");
}

#[tokio::test]
async fn get_coin_asset_returns_the_correct_asset() {
    // Arrange
    let app = spawn_app().await;
    let name = name();
    insert_coin_asset(&app, &name, possessed()).await;

    // Act
    let response = app.get_coin_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["coin_data"]["name"], name);
}
