use anyhow::{Context, Result};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::middleware::AuthenticatedUserId;
use crate::routes::coin_assets::{CoinAssetResponse, CoinAssetRow};

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        id = %id,
        user_id = %user.id()
    ),
    err(Debug)
)]
pub(crate) async fn get_coin_asset(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthenticatedUserId>,
) -> Result<Json<CoinAssetResponse>, GetCoinAssetError> {
    let row = query_coin_asset_row(&pool, id, user.id())
        .await
        .context("Failed to fetch coin asset")?
        .ok_or(GetCoinAssetError::InvalidId)?;

    Ok(Json(row.into()))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_coin_asset_row(pool: &SqlitePool, coin_id: i64, user_id: i64) -> Result<Option<CoinAssetRow>> {
    // INNER JOIN coins: an orphan coin_asset can't exist (FK). Joined image
    // columns get `AS "..?"` to force Option<T> (LEFT JOIN nullability).
    let row = sqlx::query_as!(
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
            WHERE ca.coin_id = $1 AND ca.user_id = $2
        "#,
        coin_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetCoinAssetError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
