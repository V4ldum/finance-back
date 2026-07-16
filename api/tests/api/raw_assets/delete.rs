use claims::assert_none;

use crate::{
    helpers::{composition, name, possessed, purity, spawn_app, unit_weight},
    raw_assets::{insert_raw_asset, nuke_raw_assets_table},
};

#[tokio::test]
async fn delete_raw_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_raw_assets_table(&app).await;

    // Act
    let response = app.delete_raw_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to delete raw asset");
}

#[tokio::test]
async fn delete_raw_asset_deletes_the_data() {
    // Arrange
    let app = spawn_app().await;
    insert_raw_asset(&app, &name(), possessed(), unit_weight(), &composition(), purity()).await;

    // Act
    let response = app.delete_raw_asset(1).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 204);

    let saved = sqlx::query!("SELECT name FROM raw_assets")
        .fetch_optional(&app.pool)
        .await
        .expect("Failed to fetch raw_assets");
    assert_none!(saved);
}
