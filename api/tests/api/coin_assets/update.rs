use serde_json::json;

use crate::{
    coin_assets::nuke_coin_assets_table,
    helpers::{fake_id, spawn_app},
};

#[tokio::test]
async fn update_coin_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    // No data in db by default

    // Act
    let response = app.patch_coin_asset(fake_id(), &json!({})).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 422);
}

#[tokio::test]
async fn update_coin_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let json = json!({
        "possessed": -1,
    });

    // Act
    let response = app.patch_coin_asset(1, &json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 400,);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "possessed must be >= 1");
}

#[tokio::test]
async fn update_coin_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coin_assets_table(&app).await;
    let json = json!({
        "possessed": 1,
    });

    // Act
    let response = app.patch_coin_asset(1, &json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch coin asset");
}

#[tokio::test]
async fn update_coin_asset_returns_404_when_id_is_not_in_database() {
    // Arrange
    let app = spawn_app().await;
    let json = json!({
        "possessed": 1,
    });
    // No data in db by default

    // Act
    let response = app.patch_coin_asset(1, &json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "The provided id is invalid");
}
