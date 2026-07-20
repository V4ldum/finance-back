use axum::response::{IntoResponse, Response};

/***** ENDPOINT *****/

#[tracing::instrument(skip_all)]
pub(crate) async fn health_check() -> Response {
    "OK".into_response()
}
