use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::raw_asset::RawAsset;
use crate::utils::api_error::APIError;
use crate::utils::dto::assets_dto::RawAssetsDto;

#[tracing::instrument(
    name = "get raw asset",
    skip_all,
    fields(
        user_id = %user_id,
        asset_id = %id
    )
)]
pub(crate) async fn get_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match query_raw_asset(&pool, user_id, id).await {
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
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "query raw asset", skip_all)]
async fn query_raw_asset(pool: &SqlitePool, user_id: i64, id: i64) -> Result<Option<RawAsset>> {
    let raw_asset = sqlx::query_as!(
        RawAsset,
        "SELECT * FROM raw_assets WHERE user_id = $1 AND id = $2",
        user_id,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(raw_asset)
}

#[derive(Deserialize)]
pub(super) struct CreateRawAssetRequest {
    name: String,
    possessed: i64,
    unit_weight: i64,
    composition: String,
    purity: i64,
}

#[tracing::instrument(
    name = "create raw asset",
    skip_all,
    fields(
        user_id = %user_id,
        asset_name = %request.name,
        possessed = %request.possessed,
        unit_weight = %request.unit_weight,
        composition = %request.composition,
        purity = %request.purity
    )
)]
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

    match add_raw_asset(&pool, user_id, &request).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "add raw asset", skip_all)]
async fn add_raw_asset(pool: &SqlitePool, user_id: i64, request: &CreateRawAssetRequest) -> Result<()> {
    sqlx::query!(
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}

#[derive(Deserialize)]
pub(super) struct UpdateRawAssetRequest {
    name: Option<String>,
    possessed: Option<i64>,
    unit_weight: Option<i64>,
    composition: Option<String>,
    purity: Option<i64>,
}

#[tracing::instrument(
    name = "update raw asset",
    skip_all,
    fields(
        user_id = %user_id,
        asset_id = %id,
        asset_name = ?request.name,
        possessed = ?request.possessed,
        unit_weight = ?request.unit_weight,
        composition = ?request.composition,
        purity = ?request.purity
    )
)]
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

    if (request.name.is_some()
        || request.possessed.is_some()
        || request.unit_weight.is_some()
        || request.composition.is_some()
        || request.purity.is_some())
        && update_a_raw_asset(&pool, user_id, id, &request).await.is_err()
    {
        return APIError::database_error().into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

#[tracing::instrument(name = "update a raw asset", skip_all)]
async fn update_a_raw_asset(
    pool: &SqlitePool,
    user_id: i64,
    asset_id: i64,
    request: &UpdateRawAssetRequest,
) -> Result<()> {
    let request_name = request.name.as_ref();
    let request_possessed = request.possessed.as_ref();
    let request_unit_weight = request.unit_weight.as_ref();
    let request_composition = request.composition.as_ref();
    let request_purity = request.purity.as_ref();

    // COALESCE writes the first non-null argument in the pair
    sqlx::query!(
        r#"
            UPDATE raw_assets
            SET name = COALESCE($1, name),
                possessed = COALESCE($2, possessed),
                unit_weight = COALESCE($3, unit_weight),
                composition = COALESCE($4, composition),
                purity = COALESCE($5, purity)
            WHERE id = $6 AND user_id = $7
        "#,
        request_name,
        request_possessed,
        request_unit_weight,
        request_composition,
        request_purity,
        asset_id,
        user_id,
    )
    .execute(pool)
    .await
    .inspect_err(|e| tracing::error!("Failed to execute query: {e:?}"))?;

    Ok(())
}

#[tracing::instrument(
    name = "delete raw asset",
    skip_all,
    fields(
        user_id = %user_id,
        asset_id = %id
    )
)]
pub(crate) async fn delete_raw_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match delete_a_raw_asset(&pool, user_id, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "delete a raw asset", skip_all)]
async fn delete_a_raw_asset(pool: &SqlitePool, user_id: i64, asset_id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM raw_assets WHERE id = $1 AND user_id = $2",
        asset_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}
