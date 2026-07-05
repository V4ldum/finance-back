use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;
use sqlx::error::ErrorKind;

use crate::domain::{AssetPossessed, CreateCoinAsset};
use crate::middleware::auth::AuthenticatedUserId;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

#[derive(Deserialize)]
pub(crate) struct CreateCoinAssetRequest {
    coin_id: i64,
    possessed: i64,
}

impl TryFrom<CreateCoinAssetRequest> for CreateCoinAsset {
    type Error = String;

    fn try_from(value: CreateCoinAssetRequest) -> Result<Self, String> {
        let possessed = AssetPossessed::parse(value.possessed)?;

        Ok(Self {
            coin_id: value.coin_id,
            possessed,
        })
    }
}

#[tracing::instrument(
    skip_all,
    fields(
        id = %request.coin_id,
        user_id = %user_id,
        possessed = %request.possessed,
    ),
    err(Debug)
)]
pub(crate) async fn create_coin_asset(
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
    Json(request): Json<CreateCoinAssetRequest>,
) -> Result<StatusCode, CreateCoinAssetError> {
    let create_coin_asset: CreateCoinAsset = request.try_into().map_err(CreateCoinAssetError::ValidationError)?;

    insert_coin_asset(&pool, user_id, &create_coin_asset)
        .await
        .context("Failed to insert coin asset")
        .map_err(|e| {
            if let Some(sqlx::Error::Database(db_error)) = e.downcast_ref::<sqlx::Error>()
                && db_error.kind() == ErrorKind::UniqueViolation
            {
                CreateCoinAssetError::DuplicateCoin(create_coin_asset.coin_id)
            } else {
                CreateCoinAssetError::UnexpectedError(e)
            }
        })?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(skip_all)]
async fn insert_coin_asset(pool: &SqlitePool, user_id: i64, coin_asset: &CreateCoinAsset) -> Result<()> {
    let coin_asset_possessed = coin_asset.possessed.as_ref();

    sqlx::query!(
        r#"
            INSERT INTO coin_assets (coin_id, user_id, possessed)
            VALUES ($1, $2, $3)
        "#,
        coin_asset.coin_id,
        user_id,
        coin_asset_possessed
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(thiserror::Error)]
pub(crate) enum CreateCoinAssetError {
    #[error("{0}")]
    ValidationError(String),
    #[error("You already possess coin_id {0}")]
    DuplicateCoin(i64),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for CreateCoinAssetError {
    fn status(&self) -> StatusCode {
        match self {
            CreateCoinAssetError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreateCoinAssetError::DuplicateCoin(_) => StatusCode::CONFLICT,
            CreateCoinAssetError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for CreateCoinAssetError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for CreateCoinAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
