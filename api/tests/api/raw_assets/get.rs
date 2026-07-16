use crate::{
    helpers::{composition, name, possessed, purity, spawn_app, unit_weight},
    raw_assets::{insert_raw_asset, nuke_raw_assets_table},
};

#[tokio::test]
async fn get_raw_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_raw_assets_table(&app).await;

    // Act
    let response = app.get_raw_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch raw asset");
}

#[tokio::test]
async fn get_raw_asset_returns_404_when_id_is_not_in_database() {
    // Arrange
    let app = spawn_app().await;
    // No data in db by default

    // Act
    let response = app.get_raw_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "The provided id is invalid");
}

#[tokio::test]
async fn get_raw_asset_returns_the_correct_asset() {
    // Arrange
    let app = spawn_app().await;
    let name = name();
    insert_raw_asset(&app, &name, possessed(), unit_weight(), &composition(), purity()).await;

    // Act
    let response = app.get_raw_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["name"], name);
}
