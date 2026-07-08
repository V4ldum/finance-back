use crate::helpers::TestApp;

mod get;
mod search;

async fn nuke_coins_table(app: &TestApp) {
    sqlx::query!("DROP TABLE coins").execute(&app.pool).await.unwrap();
}
