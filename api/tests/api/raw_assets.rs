use serde_json::json;

use crate::helpers::{composition, name, possessed, purity, spawn_app, unit_weight};

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
            "purity was outside of the 1..9999 range",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_raw_asset(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when {error_message}"
        );
    }
}
