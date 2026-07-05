use serde_json::json;

use crate::helpers::{TestApp, composition, name, possessed, purity, spawn_app, unit_weight};

async fn nuke_raw_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE raw_assets").execute(&app.pool).await.unwrap();
}

#[tokio::test]
async fn create_raw_asset_returns_201_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let json = json!({
        "name": name(),
        "possessed": possessed(),
        "unit_weight": unit_weight(),
        "composition": composition(),
        "purity": purity(),
    });

    // Act
    let response = app.post_raw_asset(&json).await;

    // Assert
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn create_raw_asset_persists_the_asset() {
    // Arrange
    let app = spawn_app().await;

    let name = name();
    let possessed = possessed();
    let unit_weight = unit_weight();
    let composition = composition();
    let purity = purity();

    let json = json!({
        "name": name,
        "possessed": possessed,
        "unit_weight": unit_weight,
        "composition": composition,
        "purity": purity,
    });

    // Act
    app.post_raw_asset(&json).await;

    // Assert
    let saved = sqlx::query!("SELECT name, possessed, unit_weight, composition, purity FROM raw_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch raw_assets");

    assert_eq!(saved.name, name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_weight, unit_weight);
    assert_eq!(saved.composition, composition);
    assert_eq!(saved.purity, purity);
}

#[tokio::test]
async fn create_raw_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;

    let test_cases = vec![
        (
            json!({
                "possessed": possessed(),
                "unit_weight": unit_weight(),
                "composition": composition(),
                "purity": purity(),
            }),
            "missing name",
        ),
        (
            json!({
                "name": name(),
                "unit_weight": unit_weight(),
                "composition": composition(),
                "purity": purity(),
            }),
            "missing possessed",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "composition": composition(),
                "purity": purity(),
            }),
            "missing unit_weight",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "unit_weight": unit_weight(),
                "purity": purity(),
            }),
            "missing composition",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "unit_weight": unit_weight(),
                "composition": composition(),
            }),
            "missing purity",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_raw_asset(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            422,
            "The API did not fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}

#[tokio::test]
async fn create_raw_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;

    let test_cases = vec![
        (
            json!({
                "name": "",
                "possessed": possessed(),
                "unit_weight": unit_weight(),
                "composition": composition(),
                "purity": purity(),
            }),
            "Invalid asset name: ''",
            "name was incorrect",
        ),
        (
            json!({
                "name": name(),
                "possessed": -1,
                "unit_weight": unit_weight(),
                "composition": composition(),
                "purity": purity(),
            }),
            "possessed must be >= 1",
            "possessed was negative",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "unit_weight": -20,
                "composition": composition(),
                "purity": purity(),
            }),
            "unit_weight must be >= 0",
            "unit_weight was negative",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "unit_weight": unit_weight(),
                "composition": "Incorrect",
                "purity": purity(),
            }),
            "composition can either be \"GOLD\" or \"SILVER\"",
            "composition was incorrect",
        ),
        (
            json!({
                "name": name(),
                "possessed": possessed(),
                "unit_weight": unit_weight(),
                "composition": composition(),
                "purity": 10000,
            }),
            "purity must be between 1 and 9999",
            "purity was outside of the 1..9999 range",
        ),
    ];

    for (invalid_body, reason, error_message) in test_cases {
        // Act
        let response = app.post_raw_asset(&invalid_body).await;

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
async fn create_raw_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_raw_assets_table(&app).await;
    let json = json!({
        "name": name(),
        "possessed": possessed(),
        "unit_weight": unit_weight(),
        "composition": composition(),
        "purity": purity(),
    });

    // Act
    let response = app.post_raw_asset(&json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to insert raw asset");
}

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
    assert_eq!(json_response["reason"], "Failed to fetch raw asset");
}

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
