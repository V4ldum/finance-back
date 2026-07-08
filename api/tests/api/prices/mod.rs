use crate::helpers::TestApp;

mod get_all;
mod get_one;

async fn nuke_prices_table(app: &TestApp) {
    sqlx::query!("DROP TABLE prices").execute(&app.pool).await.unwrap();
}

async fn remove_value_from(app: &TestApp, name: &str) {
    sqlx::query!("DELETE FROM prices WHERE name = $1", name)
        .execute(&app.pool)
        .await
        .unwrap();
}
