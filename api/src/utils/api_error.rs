use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub(crate) struct APIError {
    code: StatusCode,
    reason: String,
}

#[derive(Serialize)]
struct ApiErrorResponse {
    reason: String,
}

impl APIError {
    pub(crate) fn database_error() -> Self {
        APIError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            reason: String::from("Database error"),
        }
    }
    pub(crate) fn unknown_query(query: &str) -> Self {
        APIError {
            code: StatusCode::BAD_REQUEST,
            reason: format!("Unknown query: {query}"),
        }
    }
    pub(crate) fn no_api_key() -> Self {
        APIError {
            code: StatusCode::UNAUTHORIZED,
            reason: String::from(r#""key" parameter is required"#),
        }
    }
    pub(crate) fn bad_api_key() -> Self {
        APIError {
            code: StatusCode::UNAUTHORIZED,
            reason: String::from("Invalid API Key"),
        }
    }
    pub(crate) fn bad_id(id: &str) -> Self {
        APIError {
            code: StatusCode::NOT_FOUND,
            reason: format!("The provided id is invalid: {id}"),
        }
    }
    pub(crate) fn invalid_value(reason: &str) -> Self {
        APIError {
            code: StatusCode::BAD_REQUEST,
            reason: format!("Invalid value: {reason}"),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (self.code, Json(ApiErrorResponse { reason: self.reason })).into_response()
    }
}
