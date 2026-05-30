use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::domain::{AssetName, AssetPossessed, AssetUnitValue, NewCashAsset};
use crate::middleware::check_api_key::AuthenticatedUserId;
use crate::model::cash_asset::CashAsset;
use crate::utils::api_error::APIError;
use crate::utils::dto::assets_dto::CashAssetsDto;

#[tracing::instrument(
    name = "get cash asset",
    skip_all,
    fields(
        id = %cash_asset_id,
        user_id = %user_id
    )
)]
pub(crate) async fn get_cash_asset(
    Path(cash_asset_id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match query_cash_asset(&pool, cash_asset_id, user_id).await {
        Ok(Some(asset)) => Json(CashAssetsDto {
            id: asset.id,
            name: asset.name,
            possessed: asset.possessed,
            unit_value: asset.unit_value,
        })
        .into_response(),
        Ok(None) => APIError::bad_id(&cash_asset_id.to_string()).into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "query cash asset", skip_all)]
async fn query_cash_asset(pool: &SqlitePool, asset_id: i64, user_id: i64) -> Result<Option<CashAsset>> {
    let cash_asset = sqlx::query_as!(
        CashAsset,
        "SELECT * FROM cash_assets WHERE id = $1 AND user_id = $2",
        asset_id,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(cash_asset)
}

#[derive(Deserialize)]
pub(super) struct CreateCashAssetRequest {
    name: String,
    possessed: i64,
    unit_value: i64,
}

impl TryFrom<CreateCashAssetRequest> for NewCashAsset {
    type Error = anyhow::Error;

    fn try_from(value: CreateCashAssetRequest) -> Result<NewCashAsset> {
        let name = AssetName::parse(value.name)?;
        let possessed = AssetPossessed::parse(value.possessed)?;
        let unit_value = AssetUnitValue::parse(value.unit_value)?;

        Ok(NewCashAsset {
            name,
            possessed,
            unit_value,
        })
    }
}

#[tracing::instrument(
    name = "create cash asset",
    skip_all,
    fields(
        user_id = %user_id,
        asset_name = %request.name,
        possessed = %request.possessed,
        unit_value = %request.unit_value
    )
)]
pub(crate) async fn create_cash_asset(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateCashAssetRequest>,
) -> Response {
    let new_cash_asset: NewCashAsset = match request.try_into() {
        Ok(new_cash_asset) => new_cash_asset,
        Err(err) => return APIError::invalid_value(&err.to_string()).into_response(),
    };

    match create_a_cash_asset(&pool, user_id, &new_cash_asset).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "create a cash asset", skip_all)]
async fn create_a_cash_asset(pool: &SqlitePool, user_id: i64, cash_asset: &NewCashAsset) -> Result<()> {
    let cash_asset_name = cash_asset.name.as_ref();
    let cash_asset_possessed = cash_asset.possessed.as_ref();
    let cash_asset_unit_value = cash_asset.unit_value.as_ref();

    sqlx::query!(
        r#"
            INSERT INTO cash_assets (name, possessed, unit_value, user_id)
            VALUES ($1, $2, $3, $4)
        "#,
        cash_asset_name,
        cash_asset_possessed,
        cash_asset_unit_value,
        user_id
    )
    .execute(pool)
    .await
    .inspect_err(|e| tracing::error!("Failed to execute query: {e:?}"))?;

    Ok(())
}

#[derive(Deserialize)]
pub(super) struct UpdateCashAssetRequest {
    name: Option<String>,
    possessed: Option<i64>,
    unit_value: Option<i64>,
}

#[tracing::instrument(
    name = "update cash asset",
    skip_all,
    fields(
        id = %id,
        user_id = %user_id,
        asset_name = ?request.name,
        possessed = ?request.possessed,
        unit_value = ?request.unit_value,
    )
)]
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

    match (request.name.as_deref(), request.possessed, request.unit_value) {
        (None, None, None) => (), // No update necessary
        (_, _, _) => {
            if update_a_cash_asset(&pool, user_id, id, &request).await.is_err() {
                return APIError::database_error().into_response();
            }
        }
    }

    StatusCode::NO_CONTENT.into_response()
}

#[tracing::instrument(name = "update a cash asset", skip_all)]
async fn update_a_cash_asset(
    pool: &SqlitePool,
    user_id: i64,
    asset_id: i64,
    request: &UpdateCashAssetRequest,
) -> Result<()> {
    let mut query: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE cash_assets SET ");
    let mut and = false;

    if let Some(name) = request.name.as_deref() {
        query.push("name = ");
        query.push_bind(name);
        and = true;
    }
    if let Some(possessed) = request.possessed {
        if and {
            query.push(", ");
        }
        query.push("possessed = ");
        query.push_bind(possessed);
        and = true;
    }
    if let Some(unit_value) = request.unit_value {
        if and {
            query.push(", ");
        }
        query.push("unit_value = ");
        query.push_bind(unit_value);
    }
    query.push(" WHERE id = ");
    query.push_bind(asset_id);
    query.push(" AND user_id = ");
    query.push_bind(user_id);

    query.build().execute(pool).await.map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}

#[tracing::instrument(
    name = "delete cash asset",
    skip_all,
    fields(
        id = %id,
        user_id = %user_id,
    )
)]
pub(crate) async fn delete_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Response {
    match delete_a_cash_asset(&pool, id, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => APIError::database_error().into_response(),
    }
}

#[tracing::instrument(name = "delete a cash asset", skip_all)]
async fn delete_a_cash_asset(pool: &SqlitePool, asset_id: i64, user_id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM cash_assets WHERE id = $1 AND user_id = $2",
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
