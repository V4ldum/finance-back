use chrono::Utc;

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

async fn insert_prices(app: &TestApp, gold: f64, silver: f64, sp: f64) {
    let date = Utc::now().format("%Y-%m-%d").to_string();

    sqlx::query!(
        r#"
            INSERT OR IGNORE INTO prices
            VALUES ('Gold', $1, $4),
                   ('Silver', $2, $4),
                   ('SP500', $3, $4)
            "#,
        gold,
        silver,
        sp,
        date,
    )
    .execute(&app.pool)
    .await
    .expect("Failed to insert user into database");
}
