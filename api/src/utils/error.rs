use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct APIError {
    reason: String,
}

impl APIError {
    pub fn database_error() -> (StatusCode, Json<Self>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(APIError {
                reason: String::from("Database error"),
            }),
        )
    }
    pub fn unknown_query(query: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(APIError {
                reason: format!("Unknown query : {query}"),
            }),
        )
    }
    pub fn no_api_key() -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(APIError {
                reason: String::from(r#""key" parameter is required"#),
            }),
        )
    }
    pub fn bad_api_key() -> (StatusCode, Json<Self>) {
        (
            StatusCode::UNAUTHORIZED,
            Json(APIError {
                reason: String::from("Invalid API Key"),
            }),
        )
    }
}
