use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::{
    AssetComposition, AssetName, AssetPossessed, AssetPurity, AssetUnitWeight, CreateRawAsset, UpdateRawAsset,
};
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

impl TryFrom<CreateRawAssetRequest> for CreateRawAsset {
    type Error = anyhow::Error;

    fn try_from(value: CreateRawAssetRequest) -> Result<Self> {
        let name = AssetName::parse(value.name)?;
        let possessed = AssetPossessed::parse(value.possessed)?;
        let unit_weight = AssetUnitWeight::parse(value.unit_weight)?;
        let composition = AssetComposition::parse(value.composition)?;
        let purity = AssetPurity::parse(value.purity)?;

        Ok(Self {
            name,
            possessed,
            unit_weight,
            composition,
            purity,
        })
    }
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
    let create_raw_asset: CreateRawAsset = match request.try_into() {
        Ok(create_raw_asset) => create_raw_asset,
        Err(err) => return APIError::invalid_value(&err.to_string()).into_response(),
    };

    match add_raw_asset(&pool, user_id, &create_raw_asset).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "add raw asset", skip_all)]
async fn add_raw_asset(pool: &SqlitePool, user_id: i64, raw_asset: &CreateRawAsset) -> Result<()> {
    let raw_asset_name = raw_asset.name.as_ref();
    let raw_asset_possessed = raw_asset.possessed.as_ref();
    let raw_asset_unit_weight = raw_asset.unit_weight.as_ref();
    let raw_asset_composition = raw_asset.composition.as_ref();
    let raw_asset_purity = raw_asset.purity.as_ref();

    sqlx::query!(
        r#"
            INSERT INTO raw_assets (name, possessed, unit_weight, composition, purity, user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        raw_asset_name,
        raw_asset_possessed,
        raw_asset_unit_weight,
        raw_asset_composition,
        raw_asset_purity,
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

impl TryFrom<UpdateRawAssetRequest> for UpdateRawAsset {
    type Error = anyhow::Error;

    fn try_from(value: UpdateRawAssetRequest) -> Result<Self> {
        let name = value.name.map(AssetName::parse).transpose()?;
        let possessed = value.possessed.map(AssetPossessed::parse).transpose()?;
        let unit_weight = value.unit_weight.map(AssetUnitWeight::parse).transpose()?;
        let composition = value.composition.map(AssetComposition::parse).transpose()?;
        let purity = value.purity.map(AssetPurity::parse).transpose()?;

        Ok(Self {
            name,
            possessed,
            unit_weight,
            composition,
            purity,
        })
    }
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
    let update_raw_asset: UpdateRawAsset = match request.try_into() {
        Ok(update_raw_asset) => update_raw_asset,
        Err(err) => return APIError::invalid_value(&err.to_string()).into_response(),
    };

    if (update_raw_asset.name.is_some()
        || update_raw_asset.possessed.is_some()
        || update_raw_asset.unit_weight.is_some()
        || update_raw_asset.composition.is_some()
        || update_raw_asset.purity.is_some())
        && update_a_raw_asset(&pool, user_id, id, &update_raw_asset).await.is_err()
    {
        return APIError::database_error().into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

#[tracing::instrument(name = "update a raw asset", skip_all)]
async fn update_a_raw_asset(pool: &SqlitePool, user_id: i64, asset_id: i64, raw_asset: &UpdateRawAsset) -> Result<()> {
    let raw_asset_name = raw_asset.name.as_ref().map(|n| n.as_ref());
    let raw_asset_possessed = raw_asset.possessed.as_ref().map(|p| p.as_ref());
    let raw_asset_unit_weight = raw_asset.unit_weight.as_ref().map(|w| w.as_ref());
    let raw_asset_composition = raw_asset.composition.as_ref().map(|c| c.as_ref());
    let raw_asset_purity = raw_asset.purity.as_ref().map(|p| p.as_ref());

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
        raw_asset_name,
        raw_asset_possessed,
        raw_asset_unit_weight,
        raw_asset_composition,
        raw_asset_purity,
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
