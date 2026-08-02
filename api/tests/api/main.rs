use sqlx::SqlitePool;

use crate::{coins::insert_coin_with_name, helpers::spawn_app};

mod assets;
mod auth_middleware;
mod cash_assets;
mod coin_assets;
mod coins;
mod health_check;
mod helpers;
mod prices;
mod raw_assets;

#[tokio::test]
async fn pool_enforces_foreign_keys() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let fk_on: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
        .fetch_one(&app.pool)
        .await
        .unwrap();

    // Assert
    assert_eq!(fk_on, 1, "foreign key enforcement must be on");
}

async fn drop_coins_fts_triggers(pool: &SqlitePool) {
    sqlx::query!("DROP TRIGGER insert_coins_fts")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query!("DROP TRIGGER update_coins_fts")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query!("DROP TRIGGER delete_coins_fts")
        .execute(pool)
        .await
        .unwrap();
}

async fn backfill_coins_fts(pool: &SqlitePool) {
    sqlx::query!("INSERT INTO coins_fts(coins_fts) VALUES('rebuild')")
        .execute(pool)
        .await
        .unwrap();
}

async fn query_coins_fts(pool: &SqlitePool, query: &str) -> Vec<String> {
    sqlx::query!(
        r#"SELECT name AS "name!: String" FROM coins_fts WHERE coins_fts MATCH $1"#,
        query
    )
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.name)
    .collect()
}

#[tokio::test]
#[ignore = "backfill has been confirmed and is already done in production"]
async fn coins_fts_backfill_worked() {
    // Arrange
    let app = spawn_app().await;
    // Drop the trigger so it doesn't automatically populate the vtable
    drop_coins_fts_triggers(&app.pool).await;

    // Act
    // Migration is run on an empty database so nothing is backfilled
    // We manually trigger an insert and a backfill to confirm it works
    insert_coin_with_name(&app, "Silver Krugerrand").await;
    backfill_coins_fts(&app.pool).await;
    let result = query_coins_fts(&app.pool, "Silver").await;

    // Assert
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Silver Krugerrand");
}

#[tokio::test]
async fn coins_fts_updates_on_insert() {
    // Arrange
    let app = spawn_app().await;

    // Act
    insert_coin_with_name(&app, "Silver Krugerrand").await;

    // Assert
    let result = query_coins_fts(&app.pool, "Silver").await;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Silver Krugerrand");
}

#[tokio::test]
async fn coins_fts_updates_on_update() {
    // Arrange
    let app = spawn_app().await;
    insert_coin_with_name(&app, "Silver Krugerrand").await;

    // Act
    sqlx::query!("UPDATE coins SET name = 'Gold Krugerrand' WHERE name = 'Silver Krugerrand'")
        .execute(&app.pool)
        .await
        .unwrap();

    // Assert
    let result = query_coins_fts(&app.pool, "Gold").await;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Gold Krugerrand");

    let result = query_coins_fts(&app.pool, "Silver").await;
    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn coins_fts_updates_on_delete() {
    // Arrange
    let app = spawn_app().await;
    insert_coin_with_name(&app, "Silver Krugerrand").await;

    // Act
    sqlx::query!("DELETE FROM coins WHERE name = 'Silver Krugerrand'")
        .execute(&app.pool)
        .await
        .unwrap();

    // Assert
    let result = query_coins_fts(&app.pool, "Silver").await;
    assert_eq!(result.len(), 0);
}
