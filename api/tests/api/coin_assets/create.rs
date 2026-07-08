use serde_json::json;

use crate::{
    coin_assets::{insert_coin, nuke_coin_assets_table},
    helpers::{fake_id, possessed, spawn_app},
};

#[tokio::test]
async fn create_coin_asset_returns_201_for_valid_data() {
    // Arrange
    let app = spawn_app().await;

    let coin_id = fake_id();
    let json = json!({
        "coin_id": coin_id,
        "possessed": possessed(),
    });
    insert_coin(&app, coin_id).await;

    // Act
    let response = app.post_coin_asset(&json).await;

    // Assert
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn create_coin_asset_persists_the_asset() {
    // Arrange
    let app = spawn_app().await;

    let coin_id = fake_id();
    let possessed = possessed();

    let json = json!({
        "coin_id": coin_id,
        "possessed": possessed,
    });
    insert_coin(&app, coin_id).await;

    // Act
    app.post_coin_asset(&json).await;

    // Assert
    let saved = sqlx::query!("SELECT coin_id, possessed FROM coin_assets")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch coin_assets");

    assert_eq!(saved.coin_id, coin_id);
    assert_eq!(saved.possessed, possessed);
}

#[tokio::test]
async fn create_coin_asset_returns_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            json!({
                "possessed": possessed(),
            }),
            "missing coin_id",
        ),
        (
            json!({
                "coin_id": fake_id(),
            }),
            "missing possessed",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_coin_asset(&invalid_body).await;

        // Assert
        let status = response.status().as_u16();
        assert_eq!(
            status, 422,
            "The API did not fail with 422 Bad Request when the payload was {error_message}"
        );
    }
}

#[tokio::test]
async fn create_coin_asset_returns_400_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let json = json!({
        "coin_id": fake_id(),
        "possessed": 0,
    });

    // Act
    let response = app.post_coin_asset(&json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 400,);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "possessed must be >= 1");
}

#[tokio::test]
async fn create_coin_asset_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_coin_assets_table(&app).await;
    let json = json!({
        "coin_id": fake_id(),
        "possessed": possessed(),
    });

    // Act
    let response = app.post_coin_asset(&json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to insert coin asset");
}

#[tokio::test]
async fn create_coin_asset_returns_409_when_adding_a_coin_you_already_possess() {
    // Arrange
    let app = spawn_app().await;
    let coin_id = fake_id();
    let json = json!({
        "coin_id": coin_id,
        "possessed": possessed(),
    });
    insert_coin(&app, coin_id).await;

    // Act
    let _ = app.post_coin_asset(&json).await; // Insert twice
    let response = app.post_coin_asset(&json).await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 409);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(
        json_response["reason"],
        format!("You already possess coin_id {coin_id}")
    );
}
