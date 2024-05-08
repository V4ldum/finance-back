use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::database::Database;
use crate::util::api_error::APIError;

pub async fn check_api_key(
    State(database): State<Database>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let Some(key) = headers.get("X-API-KEY") else {
        return APIError::no_api_key().into_response();
    };
    let Ok(key) = key.to_str() else {
        return APIError::bad_api_key().into_response();
    };

    let Ok(found_key) = database.get_user(key).await else {
        return APIError::database_error().into_response();
    };

    if found_key.is_none() {
        return APIError::bad_api_key().into_response();
    }

    next.run(request).await
}
