use serde_json::json;

use crate::{coins::insert_coin_with_name, helpers::TestApp};

mod create;
mod delete;
mod get;
mod update;

async fn nuke_coin_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE coin_assets").execute(&app.pool).await.unwrap();
}

pub(crate) async fn insert_coin_asset(app: &TestApp, name: &str, possessed: i64) {
    insert_coin_with_name(app, name).await;
    app.post_coin_asset(&json!({
        "coin_id": 1,
        "possessed": possessed,
    }))
    .await;
}
