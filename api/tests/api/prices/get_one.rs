use crate::{
    helpers::spawn_app,
    prices::{nuke_prices_table, remove_value_from},
};

#[tokio::test]
async fn get_price_returns_404_when_fetching_a_non_existent_price() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_price("paladium").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 404);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(
        json_response["reason"],
        "price can either be \"gold\" or \"silver\" or \"sp500\""
    );
}

#[tokio::test]
async fn get_price_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_prices_table(&app).await;

    // Act
    let response = app.get_price("gold").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch price");
}

#[tokio::test]
async fn get_price_returns_500_when_fetching_a_price_that_should_exist() {
    // Arrange
    let app = spawn_app().await;
    remove_value_from(&app, "Gold").await;

    // Act
    let response = app.get_price("gold").await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(
        json_response["reason"],
        "No price found for query key Gold. This should not happen"
    );
}
