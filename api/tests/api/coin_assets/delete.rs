use claims::assert_none;

use crate::{
    coin_assets::{insert_coin_asset, nuke_coin_assets_table},
    helpers::{name, possessed, spawn_app},
};

#[tokio::test]
async fn delete_coin_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coin_assets_table(&app).await;

    // Act
    let response = app.delete_coin_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to delete coin asset");
}

#[tokio::test]
async fn delete_coin_asset_deletes_the_data() {
    // Arrange
    let app = spawn_app().await;
    insert_coin_asset(&app, &name(), possessed()).await;

    // Act
    let response = app.delete_coin_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 204);

    let saved = sqlx::query!("SELECT possessed FROM coin_assets")
        .fetch_optional(&app.pool)
        .await
        .expect("Failed to fetch coin_assets");
    assert_none!(saved);
}
