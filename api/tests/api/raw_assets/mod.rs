use crate::helpers::TestApp;

mod create;
mod delete;
mod get;
mod update;

async fn nuke_raw_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE raw_assets").execute(&app.pool).await.unwrap();
}
