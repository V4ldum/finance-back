use fake::Fake;
use serde_json::json;

use crate::helpers::{TestApp, composition, fake_id, name, possessed, purity, unit_weight};

mod create;
mod delete;
mod get;
mod update;

async fn nuke_coin_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE coin_assets").execute(&app.pool).await.unwrap();
}

async fn insert_coin(app: &TestApp, id: i64) {
    let numista_id = fake_id();
    let name = name();
    let weight = unit_weight();
    let size = (1..100).fake::<i64>();
    let min_year = (1900..2020).fake::<i64>().to_string();
    let composition = composition();
    let purity = purity();
    sqlx::query!(
        "
        INSERT INTO coins(id, numista_id, name, weight, size, min_year, composition, purity)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8)
        ",
        id,
        numista_id,
        name,
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

async fn insert_coin_asset(app: &TestApp) {
    insert_coin(app, 1).await;
    app.post_coin_asset(&json!({
        "coin_id": 1,
        "possessed": possessed(),
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
