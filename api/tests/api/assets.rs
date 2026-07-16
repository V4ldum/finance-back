use crate::{
    cash_assets::insert_cash_asset,
    coin_assets::insert_coin_asset,
    helpers::{TestApp, composition, name, possessed, purity, spawn_app, unit_value, unit_weight},
    raw_assets::insert_raw_asset,
};

async fn nuke_raw_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE raw_assets").execute(&app.pool).await.unwrap();
}

#[tokio::test]
async fn get_assets_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_raw_assets_table(&app).await;

    // Act
    let response = app.get_assets().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch raw assets");
}

#[tokio::test]
async fn get_assets_returns_the_correct_data() {
    // Arrange
    let app = spawn_app().await;

    let raw_asset_name = name();
    let cash_asset_name = name();
    let coin_asset_name = name();

    insert_raw_asset(
        &app,
        &raw_asset_name,
        possessed(),
        unit_weight(),
        &composition(),
        purity(),
    )
    .await;
    insert_cash_asset(&app, &cash_asset_name, possessed(), unit_value()).await;
    insert_coin_asset(&app, &coin_asset_name, possessed()).await;

    // Act
    let response = app.get_assets().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    dbg!(&json_response);
    assert_eq!(json_response["raw_assets"][0]["name"], raw_asset_name);
    assert_eq!(json_response["cash_assets"][0]["name"], cash_asset_name);
    assert_eq!(json_response["coin_assets"][0]["coin_data"]["name"], coin_asset_name);
}
