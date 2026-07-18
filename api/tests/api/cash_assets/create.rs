use serde_json::json;

use crate::{
    cash_assets::nuke_cash_assets_table,
    helpers::{name, possessed, spawn_app, unit_value},
};

#[tokio::test]
async fn create_cash_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            json!({
                "possessed": possessed(),
                "unit_value": unit_value(),
            }),
            "missing field `name`",
            "missing name",
        ),
        (
            json!({
                "name": name(),
                "unit_value": unit_value(),
            }),
            "missing field `possessed`",
            "missing possessed",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
            }),
            "missing field `unit_value`",
            "missing unit_value",
        ),
    ];

    for (invalid_body, missing_field, error_message) in test_cases {
        // Act
        let response = app.post_cash_asset(&invalid_body).await;

        // Assert
        let status = response.status().as_u16();
        assert_eq!(
            status, 422,
            "The API did not fail with 422 Unprocessable Entity when the payload was {error_message}"
        );

        let json_response = response.json::<serde_json::Value>().await.unwrap();
        assert_eq!(json_response["status"], status);
        assert_eq!(
            json_response["reason"],
            format!("Failed to deserialize the JSON body into the target type: {missing_field}")
        );
    }
}

#[tokio::test]
async fn create_cash_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            json!({
                "name": "",
                "possessed": possessed(),
                "unit_value": unit_value(),
            }),
            "Invalid asset name: ''",
            "name was incorrect",
        ),
        (
            json!({
                "name": name(),
                "possessed": -1,
                "unit_value": unit_value(),
            }),
            "possessed must be >= 1",
            "possessed was negative",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "unit_value": -20,
            }),
            "unit_value must be >= 0",
            "unit_value was negative",
        ),
    ];

    for (invalid_body, reason, error_message) in test_cases {
        // Act
        let response = app.post_cash_asset(&invalid_body).await;

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
async fn create_cash_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_cash_assets_table(&app).await;
    let json = json!({
        "name": name(),
        "possessed": possessed(),
        "unit_value": unit_value(),
    });

    // Act
    let response = app.post_cash_asset(&json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to insert cash asset");
}

#[tokio::test]
async fn create_cash_asset_persists_the_asset() {
    // Arrange
    let app = spawn_app().await;

    let name = name();
    let possessed = possessed();
    let unit_value = unit_value();

    let json = json!({
        "name": name,
        "possessed": possessed,
        "unit_value": unit_value,
    });

    // Act
    let response = app.post_cash_asset(&json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 201);
    //
    let saved = sqlx::query!("SELECT name, possessed, unit_value FROM cash_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch cash_assets");

    assert_eq!(saved.name, name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_value, unit_value);
}
