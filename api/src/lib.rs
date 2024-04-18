use sea_orm::DatabaseConnection;

use crate::routes::router;

mod database;
mod middleware;
mod routes;

mod utils;

pub async fn run(db: DatabaseConnection) {
    let router = router(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("The listener should be able to bind to this port");

    axum::serve(listener, router)
        .await
        .expect("The server should launch successfully");
}
