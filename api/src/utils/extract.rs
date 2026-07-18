use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::utils::errors::error_response;

/// Drop-in replacement for [`axum::Json`] whose extraction failures render
/// as our `ApiError` envelope.
pub(crate) struct Json<T>(pub(crate) T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(value)) => Ok(Self(value)),
            Err(rejection) => Err(reject(rejection.status(), &rejection.body_text())),
        }
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

/// Drop-in replacement for [`axum::extract::Query`] with the same rejection handling.
pub(crate) struct Query<T>(pub(crate) T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Query(value)) => Ok(Self(value)),
            Err(rejection) => Err(reject(rejection.status(), &rejection.body_text())),
        }
    }
}

/// Re-wrap an extractor rejection as an `ApiError`, keeping Axum's status.
///
/// Serde's message carries a volatile `" at line X column Y"` suffix that depends
/// on the exact payload; strip it so the reason is stable and readable.
fn reject(status: StatusCode, text: &str) -> Response {
    let reason = match text.split_once(" at line ") {
        Some((head, _)) => head.trim_end(),
        None => text.trim_end(),
    };
    error_response(status, reason)
}
