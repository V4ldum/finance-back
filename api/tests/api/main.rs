use crate::helpers::spawn_app;

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

#[tokio::test]
async fn unaccent_extension_is_available() {
    // Arrange
    let app = spawn_app().await;

    // Act
    // fails with "no such function: unaccent" if the extension isn't loaded
    let unaccented: String = sqlx::query_scalar("SELECT unaccent('éàù')")
        .fetch_one(&app.pool)
        .await
        .expect("unaccent extension function should be registered");

    // Assert
    assert_eq!(unaccented, "eau");
}
