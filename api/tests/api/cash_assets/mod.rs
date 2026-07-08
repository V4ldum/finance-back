use crate::helpers::TestApp;

mod create;
mod delete;
mod get;
mod update;

async fn nuke_cash_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE cash_assets").execute(&app.pool).await.unwrap();
}
