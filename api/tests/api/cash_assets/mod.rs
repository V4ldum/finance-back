use serde_json::json;

use crate::helpers::TestApp;

mod create;
mod delete;
mod get;
mod update;

async fn nuke_cash_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE cash_assets").execute(&app.pool).await.unwrap();
}

pub(crate) async fn insert_cash_asset(app: &TestApp, name: &str, possessed: i64, unit_value: i64) {
    app.post_cash_asset(&json!({
        "name": name,
        "possessed": possessed,
        "unit_value": unit_value,
    }))
    .await;
}
