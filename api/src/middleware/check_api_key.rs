use axum::Extension;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::database::prelude::ApiKeys;
use crate::utils::error::APIError;

pub async fn check_api_key(
    Extension(db): Extension<DatabaseConnection>,
    request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    let Some(key) = headers.get("X-API-KEY") else {
        return APIError::no_api_key().into_response();
    };
    let Ok(key) = key.to_str() else {
        return APIError::bad_api_key().into_response();
    };

    let Ok(found_key) = ApiKeys::find_by_id(key).one(&db).await else {
        return APIError::database_error().into_response();
    };

    if found_key.is_none() {
        return APIError::bad_api_key().into_response();
    }

    next.run(request).await
}
