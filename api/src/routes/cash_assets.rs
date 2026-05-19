use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::cash_asset::CashAsset;
use crate::utils::api_error::APIError;
use crate::utils::dto::assets_dto::CashAssetsDto;

pub(crate) async fn get_cash_asset(
    Path(cash_asset_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match sqlx::query_as!(
        CashAsset,
        "SELECT * FROM cash_assets WHERE id = $1 AND user_id = $2",
        cash_asset_id,
        user_id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(asset)) => Json(CashAssetsDto {
            id: asset.id,
            name: asset.name,
            possessed: asset.possessed,
            unit_value: asset.unit_value,
        })
        .into_response(),
        Ok(None) => APIError::bad_id(&cash_asset_id.to_string()).into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct CreateCashAssetRequest {
    name: String,
    possessed: i64,
    unit_value: i64,
}

pub(crate) async fn create_cash_asset(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateCashAssetRequest>,
) -> Response {
    if request.possessed < 1 {
        return APIError::invalid_value("possessed must be >= 1").into_response();
    }

    if request.unit_value < 0 {
        return APIError::invalid_value("unit_value must be >= 0").into_response();
    }

    match sqlx::query!(
        r#"
            INSERT INTO cash_assets (name, possessed, unit_value, user_id)
            VALUES ($1, $2, $3, $4)
            "#,
        request.name,
        request.possessed,
        request.unit_value,
        user_id
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct UpdateCashAssetRequest {
    name: Option<String>,
    possessed: Option<i64>,
    unit_value: Option<i64>,
}

pub(crate) async fn update_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateCashAssetRequest>,
) -> Response {
    if matches!(request.possessed, Some(p) if p < 1) {
        return APIError::invalid_value("possessed must be >= 1").into_response();
    }

    if matches!(request.unit_value, Some(v) if v < 0) {
        return APIError::invalid_value("unit_value must be >= 0").into_response();
    }

    match (request.name, request.possessed, request.unit_value) {
        (None, None, None) => (), // No update necessary
        (name, possessed, unit_value) => {
            let mut query: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE cash_assets SET ");
            let mut and = false;

            if let Some(name) = name {
                query.push("name = ");
                query.push_bind(name);
                and = true;
            }
            if let Some(possessed) = possessed {
                if and {
                    query.push(", ");
                }
                query.push("possessed = ");
                query.push_bind(possessed);
                and = true;
            }
            if let Some(unit_value) = unit_value {
                if and {
                    query.push(", ");
                }
                query.push("unit_value = ");
                query.push_bind(unit_value);
            }
            query.push(" WHERE id = ");
            query.push_bind(id);
            query.push(" AND user_id = ");
            query.push_bind(user_id);

            match query.build().execute(&pool).await {
                Ok(_) => (),
                Err(e) => {
                    log::error!("{e}");
                    return APIError::database_error().into_response();
                }
            }
        }
    }

    StatusCode::NO_CONTENT.into_response()
}

pub(crate) async fn delete_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match sqlx::query!("DELETE FROM cash_assets WHERE id = $1 AND user_id = $2", id, user_id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}
