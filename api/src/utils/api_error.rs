use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct APIError {
    code: StatusCode,
    reason: String,
}

#[derive(Serialize)]
struct ApiErrorResponse {
    reason: String,
}

impl APIError {
    pub fn database_error() -> Self {
        APIError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            reason: String::from("Database error"),
        }
    }
    pub fn unknown_query(query: &str) -> Self {
        APIError {
            code: StatusCode::BAD_REQUEST,
            reason: format!("Unknown query : {query}"),
        }
    }
    pub fn no_api_key() -> Self {
        APIError {
            code: StatusCode::BAD_REQUEST,
            reason: String::from(r#""key" parameter is required"#),
        }
    }
    pub fn bad_api_key() -> Self {
        APIError {
            code: StatusCode::UNAUTHORIZED,
            reason: String::from("Invalid API Key"),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (
            self.code,
            Json(ApiErrorResponse {
                reason: self.reason,
            }),
        )
            .into_response()
    }
}