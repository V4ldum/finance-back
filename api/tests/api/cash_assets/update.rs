use serde_json::json;

use crate::{
    cash_assets::{insert_cash_asset, nuke_cash_assets_table},
    helpers::{name, possessed, spawn_app, unit_value},
};

#[tokio::test]
async fn update_cash_asset_returns_400_when_data_is_invalid() {
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
                "unit_value": -20,
            }),
            "unit_value must be >= 0",
            "unit_value was negative",
        ),
    ];

    for (invalid_body, reason, error_message) in test_cases {
        // Act
        let response = app.patch_cash_asset(1, &invalid_body).await;

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
async fn update_cash_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_cash_assets_table(&app).await;

    // Act
    let response = app.patch_cash_asset(1, &json!({})).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to update cash asset");
}

#[tokio::test]
async fn update_cash_asset_returns_404_when_id_is_not_in_database() {
    // Arrange
    let app = spawn_app().await;
    // No data in db by default

    // Act
    let response = app.patch_cash_asset(1, &json!({})).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "The provided id is invalid");
}

#[tokio::test]
async fn update_cash_asset_updates_the_asset() {
    // Arrange
    let app = spawn_app().await;

    let original_name = name();
    let possessed = possessed();
    let unit_value = unit_value();
    insert_cash_asset(&app, &original_name, possessed, unit_value).await;

    let new_name = name();

    // Act
    let response = app
        .patch_cash_asset(
            1,
            &json!({
                "name": new_name,
            }),
        )
        .await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 204);

    let saved = sqlx::query!("SELECT name, possessed, unit_value FROM cash_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch cash_assets");

    assert_eq!(saved.name, new_name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_value, unit_value);
}
