use axum::response::{IntoResponse, Response};

pub async fn health_check() -> Response {
    "API is working!".into_response()
}
