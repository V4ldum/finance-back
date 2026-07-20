use axum::response::{IntoResponse, Response};

/***** ENDPOINT *****/

#[tracing::instrument(skip_all)]
pub(crate) async fn health_check() -> Response {
    tracing::error!("test err");
    tracing::warn!("test warn");
    "OK".into_response()
}
