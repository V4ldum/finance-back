use axum::response::{IntoResponse, Response};

pub async fn health_check() -> Response {
    "OK".into_response()
}
