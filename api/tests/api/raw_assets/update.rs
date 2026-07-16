use serde_json::json;

use crate::{
    helpers::{composition, name, possessed, purity, spawn_app, unit_weight},
    raw_assets::{insert_raw_asset, nuke_raw_assets_table},
};

#[tokio::test]
async fn update_raw_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            json!({
                "name": "",
            }),
            "Invalid asset name: ''",
            "name was incorrect",
        ),
        (
            json!({
                "possessed": -1,
            }),
            "possessed must be >= 1",
            "possessed was negative",
        ),
        (
            json!({
                "unit_weight": -20,
            }),
            "unit_weight must be >= 0",
            "unit_weight was negative",
        ),
    ];

    for (invalid_body, reason, error_message) in test_cases {
        // Act
        let response = app.patch_raw_asset(1, &invalid_body).await;

        // Assert
        let status = response.status().as_u16();
        assert_eq!(
            status, 400,
            "The API did not fail with 400 Bad Request when {error_message}"
        );

        let json_response = response.json::<serde_json::Value>().await.unwrap();
        assert_eq!(json_response["status"], status);
        assert_eq!(json_response["reason"], reason);
    }
}

#[tokio::test]
async fn update_raw_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_raw_assets_table(&app).await;

    // Act
    let response = app.patch_raw_asset(1, &json!({})).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to update raw asset");
}

#[tokio::test]
async fn update_raw_asset_returns_404_when_id_is_not_in_database() {
    // Arrange
    let app = spawn_app().await;
    // No data in db by default

    // Act
    let response = app.patch_raw_asset(1, &json!({})).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "The provided id is invalid");
}

#[tokio::test]
async fn update_raw_asset_updates_the_asset() {
    // Arrange
    let app = spawn_app().await;

    let original_name = name();
    let possessed = possessed();
    let unit_weight = unit_weight();
    let composition = composition();
    let purity = purity();
    insert_raw_asset(&app, &original_name, possessed, unit_weight, &composition, purity).await;

    let new_name = name();

    // Act
    let response = app
        .patch_raw_asset(
            1,
            &json!({
                "name": new_name,
            }),
        )
        .await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 204);

    let saved = sqlx::query!("SELECT name, possessed, unit_weight, composition, purity FROM raw_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch raw_assets");

    assert_eq!(saved.name, new_name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_weight, unit_weight);
    assert_eq!(saved.composition, composition);
    assert_eq!(saved.purity, purity);
}
