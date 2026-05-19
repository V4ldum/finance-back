use crate::utils::api_error::APIError;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use sqlx::SqlitePool;

#[derive(Clone, Copy)]
pub(crate) struct AuthenticatedUserId(pub i64);

pub(crate) async fn check_api_key(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    let Some(key) = headers.get("X-API-KEY") else {
        return APIError::no_api_key().into_response();
    };
    let Ok(key) = key.to_str() else {
        return APIError::bad_api_key().into_response();
    };

    match sqlx::query!("SELECT id FROM users WHERE api_key = $1", key)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(user)) => {
            request.extensions_mut().insert(AuthenticatedUserId(user.id));
            next.run(request).await
        }
        Ok(None) => APIError::bad_api_key().into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}
