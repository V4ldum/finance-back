use anyhow::{Context, Result};
use axum::extract::State;
use axum::{Extension, Json};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::middleware::AuthenticatedUserId;
use crate::routes::cash_assets::{CashAsset, CashAssetResponse};
use crate::routes::coin_assets::{CoinAssetResponse, CoinAssetRow};
use crate::routes::raw_assets::{RawAsset, RawAssetResponse};

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        user_id = %user.id()
    ),
    err(Debug),
)]
pub(crate) async fn get_assets(
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
) -> Result<Json<AssetsResponse>, GetAssetsError> {
    // Query Raw Assets
    let raw_assets = query_raw_assets(&pool, user.id())
        .await
        .context("Failed to fetch raw assets")?;

    // Query Cash Assets
    let cash_assets = query_cash_assets(&pool, user.id())
        .await
        .context("Failed to fetch cash assets")?;

    // Query Coin Assets
    let coin_assets = query_coin_assets(&pool, user.id())
        .await
        .context("Failed to fetch coin assets")?;

    Ok(Json(AssetsResponse {
        raw_assets: raw_assets
            .into_iter()
            .map(|asset| RawAssetResponse {
                id: asset.id,
                name: asset.name,
                possessed: asset.possessed,
                unit_weight: asset.unit_weight,
                composition: asset.composition,
                purity: asset.purity,
            })
            .collect(),
        cash_assets: cash_assets
            .into_iter()
            .map(|asset| CashAssetResponse {
                id: asset.id,
                name: asset.name,
                possessed: asset.possessed,
                unit_value: asset.unit_value,
            })
            .collect(),
        coin_assets: coin_assets.into_iter().map(Into::into).collect(),
    }))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_raw_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<RawAsset>> {
    let assets = sqlx::query_as!(
        RawAsset,
        "SELECT id, name, possessed, unit_weight, composition, purity FROM raw_assets WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(assets)
}

#[tracing::instrument(skip_all)]
async fn query_cash_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<CashAsset>> {
    let assets = sqlx::query_as!(
        CashAsset,
        "SELECT id, name, possessed, unit_value FROM cash_assets WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(assets)
}

#[tracing::instrument(skip_all)]
async fn query_coin_assets(pool: &SqlitePool, user_id: i64) -> Result<Vec<CoinAssetRow>> {
    // INNER JOIN coins: an orphan coin_asset can't exist (FK) and, if it did,
    // would simply be dropped rather than 500 the whole request. Joined image
    // columns get `AS "..?"` to force Option<T> (LEFT JOIN nullability).
    let rows = sqlx::query_as!(
        CoinAssetRow,
        r#"
            SELECT
                ca.possessed,
                c.id, c.numista_id, c.name, c.weight, c.size, c.thickness,
                c.min_year, c.max_year, c.composition, c.purity,
                c.obverse, c.reverse, c.edge,
                o.image_url     AS "o_image_url?",
                o.thumbnail_url AS "o_thumbnail_url?",
                o.lettering     AS "o_lettering?",
                o.description   AS "o_description?",
                o.copyright     AS "o_copyright?",
                r.image_url     AS "r_image_url?",
                r.thumbnail_url AS "r_thumbnail_url?",
                r.lettering     AS "r_lettering?",
                r.description   AS "r_description?",
                r.copyright     AS "r_copyright?",
                e.image_url     AS "e_image_url?",
                e.thumbnail_url AS "e_thumbnail_url?",
                e.lettering     AS "e_lettering?",
                e.description   AS "e_description?",
                e.copyright     AS "e_copyright?"
            FROM coin_assets ca
            JOIN coins c ON c.id = ca.coin_id
            LEFT JOIN coin_images o ON o.id = c.obverse
            LEFT JOIN coin_images r ON r.id = c.reverse
            LEFT JOIN coin_images e ON e.id = c.edge
            WHERE ca.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/***** RESPONSE *****/

#[allow(clippy::struct_field_names)]
#[derive(Serialize)]
pub(crate) struct AssetsResponse {
    pub(crate) raw_assets: Vec<RawAssetResponse>,
    pub(crate) cash_assets: Vec<CashAssetResponse>,
    pub(crate) coin_assets: Vec<CoinAssetResponse>,
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetAssetsError {
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
