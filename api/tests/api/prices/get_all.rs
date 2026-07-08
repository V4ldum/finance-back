use crate::{
    helpers::spawn_app,
    prices::{nuke_prices_table, remove_value_from},
};

#[tokio::test]
async fn get_prices_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_prices_table(&app).await;

    // Act
    let response = app.get_prices().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch prices");
}

#[tokio::test]
async fn get_prices_returns_500_when_fetching_a_price_that_should_exist() {
    // Arrange
    let app = spawn_app().await;
    remove_value_from(&app, "Gold").await;

    // Act
    let response = app.get_prices().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(
        json_response["reason"],
        "No price found for Gold. This should not happen"
    );
}
