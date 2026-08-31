use crate::helpers::spawn_app;

#[tokio::test]
async fn live_check_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_live().await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn ready_check_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_ready().await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn live_check_doesnt_fail_if_database_is_unavailable() {
    // Arrange
    let app = spawn_app().await;
    sqlx::query("DROP TABLE _sqlx_migrations")
        .execute(&app.pool)
        .await
        .unwrap();

    // Act
    let response = app.get_live().await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn ready_check_fails_if_database_is_unavailable() {
    // Arrange
    let app = spawn_app().await;
    sqlx::query("DROP TABLE _sqlx_migrations")
        .execute(&app.pool)
        .await
        .unwrap();

    // Act
    let response = app.get_ready().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 503);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to reach the database");
}
