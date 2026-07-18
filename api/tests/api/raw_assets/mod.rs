use serde_json::json;

use crate::helpers::TestApp;

mod create;
mod delete;
mod get;
mod update;

async fn nuke_raw_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE raw_assets").execute(&app.pool).await.unwrap();
}

pub(crate) async fn insert_raw_asset(
    app: &TestApp,
    name: &str,
    possessed: i64,
    unit_weight: i64,
    composition: &str,
    purity: i64,
) {
    app.post_raw_asset(&json!({
        "name": name,
        "possessed": possessed,
        "unit_weight": unit_weight,
        "composition": composition,
        "purity": purity,
    }))
    .await;
}
