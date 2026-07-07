use std::error::Error;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
struct ApiError {
    status: u16,
    reason: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}

pub trait ApiErrorResponse: Error {
    fn status(&self) -> StatusCode;

    fn reason(&self) -> String {
        self.to_string()
    }
}

pub(crate) fn response<E: ApiErrorResponse>(error: &E) -> Response {
    ApiError {
        status: error.status().as_u16(),
        reason: error.reason(),
    }
    .into_response()
}

/// Iterates over the whole chain of errors that led to `e`, printing each `source()`.
pub(crate) fn error_chain_fmt(e: &impl std::error::Error, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{e}")?;

    let mut current = e.source();
    let mut level = 0;

    if current.is_some() {
        writeln!(f, "Caused by:")?;
    }

    while let Some(cause) = current {
        writeln!(f, "\t{level}: {cause}")?;

        current = cause.source();
        level += 1;
    }

    Ok(())
}
