use crate::domain::AssetPossessed;
use crate::middleware::auth::AuthenticatedUserId;
use crate::model::coin_asset::CoinAsset;
use crate::routes::coin_assets::query_coin_asset;
use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub(crate) struct UpdateCoinAssetRequest {
    possessed: i64,
}

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user_id,
        possessed = %request.possessed
    ),
    err(Debug)
)]
pub(crate) async fn update_coin_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<UpdateCoinAssetRequest>,
) -> Result<StatusCode, UpdateCoinAssetError> {
    let asset_possessed = AssetPossessed::parse(request.possessed).map_err(UpdateCoinAssetError::ValidationError)?;

    let coin_asset = query_coin_asset(&pool, id, user_id)
        .await
        .context("Failed to fetch coin asset")?
        .ok_or(UpdateCoinAssetError::InvalidId)?;

    // Only write if the provided value actually differs from the stored one
    if has_changes(asset_possessed, &coin_asset) {
        update_coin_asset_(&pool, user_id, id, asset_possessed)
            .await
            .context("Failed to update coin asset")?;
    }

    Ok(StatusCode::NO_CONTENT)
}

fn has_changes(possessed: AssetPossessed, current: &CoinAsset) -> bool {
    *possessed.as_ref() != current.possessed
}

#[tracing::instrument(skip_all)]
async fn update_coin_asset_(pool: &SqlitePool, user_id: i64, coin_id: i64, possessed: AssetPossessed) -> Result<()> {
    let possessed = possessed.as_ref();

    sqlx::query!(
        "UPDATE coin_assets SET possessed = $1 WHERE coin_id = $2 AND user_id = $3",
        possessed,
        coin_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum UpdateCoinAssetError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error("{0}")]
    #[status(BAD_REQUEST)]
    ValidationError(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
