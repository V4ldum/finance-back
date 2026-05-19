use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::raw_asset::RawAsset;
use crate::utils::api_error::APIError;
use crate::utils::dto::assets_dto::RawAssetsDto;

pub(crate) async fn get_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match sqlx::query_as!(
        RawAsset,
        "SELECT * FROM raw_assets WHERE user_id = $1 AND id = $2",
        user_id,
        id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(asset)) => Json(RawAssetsDto {
            id: asset.id,
            name: asset.name,
            possessed: asset.possessed,
            unit_weight: asset.unit_weight,
            composition: asset.composition,
            purity: asset.purity,
        })
        .into_response(),
        Ok(None) => APIError::bad_id(&id.to_string()).into_response(),
        Err(e) => {
            log::error!("{e}");
            APIError::database_error().into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct CreateRawAssetRequest {
    name: String,
    possessed: i64,
    unit_weight: i64,
    composition: String,
    purity: i64,
}

pub(crate) async fn create_raw_asset(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateRawAssetRequest>,
) -> Response {
    if request.possessed < 1 {
        return APIError::invalid_value("possessed must be >= 1").into_response();
    }

    if request.unit_weight < 0 {
        return APIError::invalid_value("unit_weight must be >= 0").into_response();
    }

    if request.composition != "GOLD" && request.composition != "SILVER" {
        return APIError::invalid_value("composition can either be \"GOLD\" or \"SILVER\"").into_response();
    }

    if request.purity > 9999 || request.purity < 1 {
        return APIError::invalid_value("purity must be between 1 and 9999").into_response();
    }

    match sqlx::query!(
        r#"
            INSERT INTO raw_assets (name, possessed, unit_weight, composition, purity, user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        request.name,
        request.possessed,
        request.unit_weight,
        request.composition,
        request.purity,
        user_id,
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
pub(super) struct UpdateRawAssetRequest {
    name: Option<String>,
    possessed: Option<i64>,
    unit_weight: Option<i64>,
    composition: Option<String>,
    purity: Option<i64>,
}

pub(crate) async fn update_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateRawAssetRequest>,
) -> Response {
    if matches!(request.possessed, Some(p) if p < 1) {
        return APIError::invalid_value("possessed must be >= 1").into_response();
    }

    if matches!(request.unit_weight, Some(w) if w < 0) {
        return APIError::invalid_value("unit_weight must be >= 0").into_response();
    }

    if matches!(request.composition.as_deref(), Some(c) if c != "GOLD" && c != "SILVER") {
        return APIError::invalid_value("composition can either be \"GOLD\" or \"SILVER\"").into_response();
    }

    if matches!(request.purity, Some(p) if !(1..=9999).contains(&p)) {
        return APIError::invalid_value("purity must be between 1 and 9999").into_response();
    }

    match (
        request.name,
        request.possessed,
        request.unit_weight,
        request.composition,
        request.purity,
    ) {
        (None, None, None, None, None) => (), // No update necessary
        (name, possessed, unit_weight, composition, purity) => {
            let mut query: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE raw_assets SET ");
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
            if let Some(unit_weight) = unit_weight {
                if and {
                    query.push(", ");
                }
                query.push("unit_weight = ");
                query.push_bind(unit_weight);
                and = true;
            }
            if let Some(composition) = composition {
                if and {
                    query.push(", ");
                }
                query.push("composition = ");
                query.push_bind(composition);
                and = true;
            }
            if let Some(purity) = purity {
                if and {
                    query.push(", ");
                }
                query.push("purity = ");
                query.push_bind(purity);
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
            };
        }
    };

    StatusCode::NO_CONTENT.into_response()
}

pub(crate) async fn delete_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match sqlx::query!("DELETE FROM raw_assets WHERE id = $1 AND user_id = $2", id, user_id)
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
