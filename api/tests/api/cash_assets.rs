use fake::{Fake, faker::lorem::en::Sentence};
use serde_json::json;

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_cash_asset_returns_201_for_valid_data() {
    // Arrange
    let app = spawn_app().await;

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_value = (1..1000).fake::<i64>();

    // Act
    let json = json!({
        "name": name,
        "possessed": possessed,
        "unit_value": unit_value,
    });
    let response = app.post_cash_asset(&json).await;

    // Assert
    assert_eq!(response.status().as_u16(), 201);

    let saved = sqlx::query!("SELECT name, possessed, unit_value FROM cash_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch cash_assets");

    assert_eq!(saved.name, name);
    assert_eq!(saved.possessed, possessed);
    assert_eq!(saved.unit_value, unit_value);
}

#[tokio::test]
async fn create_cash_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_value = (1..1000).fake::<i64>();

    let test_cases = vec![
        (
            json!({
                "possessed": possessed,
                "unit_value": unit_value,
            }),
            "missing name",
        ),
        (
            json!({
                "name": name,
                "unit_value": unit_value,
            }),
            "missing possessed",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
            }),
            "missing unit_value",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_cash_asset(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            422,
            "The API did not fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}

#[tokio::test]
async fn create_cash_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;

    let name = Sentence(1..3).fake::<String>();
    let possessed = (1..1000).fake::<i64>();
    let unit_value = (1..1000).fake::<i64>();

    let test_cases = vec![
        (
            json!({
                "name": "",
                "possessed": possessed,
                "unit_value": unit_value,
            }),
            "name was incorrect",
        ),
        (
            json!({
                "name": name,
                "possessed": -1,
                "unit_value": unit_value,
            }),
            "possessed was negative",
        ),
        (
            json!({
                "name": name,
                "possessed": possessed,
                "unit_value": -20,
            }),
            "unit_value was negative",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_cash_asset(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when {error_message}"
        );
    }
}
