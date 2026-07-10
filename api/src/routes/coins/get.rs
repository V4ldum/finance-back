use anyhow::{Context, Result};
use axum::Json;
use axum::extract::{Path, State};
use sqlx::SqlitePool;

use crate::routes::coins::{CoinResponse, CoinRow};

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        id = %id
    ),
    err(Debug),
)]
pub(crate) async fn get_coin(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<CoinResponse>, GetCoinError> {
    let row = query_coin(&pool, id)
        .await
        .context("Failed to fetch coin")?
        .ok_or(GetCoinError::InvalidId)?;

    Ok(Json(row.into()))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_coin(pool: &SqlitePool, id: i64) -> Result<Option<CoinRow>> {
    // Joined image columns get `AS "..?"`: a LEFT JOIN can yield all-NULL rows,
    // which sqlx cannot infer, so `?` forces each into Option<T>.
    let coin = sqlx::query_as!(
        CoinRow,
        r#"
            SELECT
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
            FROM coins c
            LEFT JOIN coin_images o ON o.id = c.obverse
            LEFT JOIN coin_images r ON r.id = c.reverse
            LEFT JOIN coin_images e ON e.id = c.edge
            WHERE c.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(coin)
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum GetCoinError {
    #[error("The provided id is invalid")]
    #[status(NOT_FOUND)]
    InvalidId,
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
