use fake::Fake;

use crate::helpers::{TestApp, composition, fake_id, name, purity, unit_weight};

mod get;
mod search;

async fn nuke_coins_table(app: &TestApp) {
    sqlx::query!("DROP TABLE coins").execute(&app.pool).await.unwrap();
}

pub(crate) async fn insert_coin_with_name(app: &TestApp, name: &str) {
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

pub(crate) async fn insert_coin_with_id(app: &TestApp, id: i64) {
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
