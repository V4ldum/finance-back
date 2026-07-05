use crate::helpers::{TestApp, spawn_app};

//async fn insert_raw_asset(app: &TestApp) {
//    app.post_raw_asset(&json!({
//        "name": name(),
//        "possessed": possessed(),
//        "unit_weight": unit_weight(),
//        "composition": composition(),
//        "purity": purity(),
//    }))
//    .await;
//}

//async fn insert_cash_asset(app: &TestApp) {
//    app.post_cash_asset(&json!({
//        "name": name(),
//        "possessed": possessed(),
//        "unit_value": unit_value(),
//    }))
//    .await;
//}

//async fn insert_coin_asset(app: &TestApp) {
//    app.post_cash_asset(&json!({
//        "name": name(),
//        "possessed": possessed(),
//        "unit_value": unit_value(),
//    }))
//    .await;
//}

async fn nuke_raw_assets_table(app: &TestApp) {
    sqlx::query!("DROP TABLE raw_assets").execute(&app.pool).await.unwrap();
}

//#[tokio::test]
//async fn get_assets_returns_400_when_data_is_invalid() {
//    // Arrange
//    let app = spawn_app().await;
//    let raw_asset = insert_raw_asset(&app).await;
//    let cash_asset = insert_cash_asset(&app).await;
//    let coin_asset = insert_coin_asset(&app).await;
//
//    // Act
//    let response: Response = app.get_assets().await;
//
//    // Assert
//    assert_eq!(response.status().as_u16(), 200);
//    todo!("check si on retrouve les assets dans la réponse");
//}

#[tokio::test]
async fn get_assets_fails_and_returns_500_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    nuke_raw_assets_table(&app).await;

    // Act
    let response = app.get_assets().await;

    // Assert
    let status = response.status().as_u16();
    assert_eq!(status, 500);

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json_response["status"], status);
    assert_eq!(json_response["reason"], "Failed to fetch raw assets");
}
