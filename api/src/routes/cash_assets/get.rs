use std::fmt::Debug;

use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::middleware::auth::AuthenticatedUserId;
use crate::routes::cash_assets::query_cash_asset;
use crate::utils::dto::assets_dto::CashAssetsDto;
use crate::utils::errors::{ApiErrorResponse, error_chain_fmt, response};

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user_id
    ),
    err(Debug)
)]
pub(crate) async fn get_cash_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(AuthenticatedUserId(user_id)): Extension<AuthenticatedUserId>,
) -> Result<Json<CashAssetsDto>, GetCashAssetError> {
    let asset = query_cash_asset(&pool, id, user_id)
        .await
        .context("Failed to fetch cash asset")?
        .ok_or(GetCashAssetError::InvalidId)?;

    Ok(Json(CashAssetsDto {
        id: asset.id,
        name: asset.name,
        possessed: asset.possessed,
        unit_value: asset.unit_value,
    }))
}

#[derive(thiserror::Error)]
pub(crate) enum GetCashAssetError {
    #[error("The provided id is invalid")]
    InvalidId,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ApiErrorResponse for GetCashAssetError {
    fn status(&self) -> StatusCode {
        match self {
            GetCashAssetError::InvalidId => StatusCode::NOT_FOUND,
            GetCashAssetError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn reason(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for GetCashAssetError {
    fn into_response(self) -> Response {
        response(&self)
    }
}

impl Debug for GetCashAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
