use axum::http::HeaderMap;

use crate::database::Database;

use super::api_error::APIError;

pub async fn get_user_id_from_headers(
    headers: &HeaderMap,
    database: &Database,
) -> Result<i32, APIError> {
    let key = headers
        .get("X-API-KEY")
        .expect("The key was confirmed present by the middleware")
        .to_str()
        .expect("The key was confirmed properly formatted by the middleware");

    let Ok(Some(user)) = database.get_user(key).await else {
        return Err(APIError::database_error());
    };

    Ok(user.id)
}
