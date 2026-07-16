use fake::Fake;
use serde_json::json;

use crate::helpers::{TestApp, composition, fake_id, name, possessed, purity, spawn_app, unit_value, unit_weight};

async fn insert_raw_asset_by_name(app: &TestApp, name: &str) {
    app.post_raw_asset(&json!({
        "name": name,
        "possessed": possessed(),
        "unit_weight": unit_weight(),
        "composition": composition(),
        "purity": purity(),
    }))
    .await;
}

async fn insert_cash_asset_by_name(app: &TestApp, name: &str) {
    app.post_cash_asset(&json!({
        "name": name,
        "possessed": possessed(),
        "unit_value": unit_value(),
    }))
    .await;
}

async fn insert_coin_by_name(app: &TestApp, name: &str) {
    let numista_id = fake_id();
    let weight = unit_weight();
    let size = (1..100).fake::<i64>();
    let min_year = (1900..2020).fake::<i64>().to_string();
    let composition = composition();
    let purity = purity();
    sqlx::query!(
        "
        INSERT INTO coins(name, numista_id, weight, size, min_year, composition, purity)
        VALUES($1, $2, $3, $4, $5, $6, $7)
        ",
        name,
        numista_id,
        weight,
        size,
        min_year,
        composition,
        purity
    )
    .execute(&app.pool)
    .await
    .unwrap();
}

async fn insert_coin_asset_by_name(app: &TestApp, name: &str) {
    insert_coin_by_name(app, name).await;
    app.post_coin_asset(&json!({
        "coin_id": 1,
        "possessed": possessed(),
    }))
    .await;
}

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

    insert_raw_asset_by_name(&app, &raw_asset_name).await;
    insert_cash_asset_by_name(&app, &cash_asset_name).await;
    insert_coin_asset_by_name(&app, &coin_asset_name).await;

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
