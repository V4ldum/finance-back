use anyhow::{Context, Result};
use axum::extract::State;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::domain::CoinSearchQuery;
use crate::routes::coins::{CoinResponse, CoinRow};
use crate::{Json, Query};

/***** REQUEST *****/

#[derive(Deserialize)]
pub(crate) struct QueryParams {
    q: String,
}

/***** ENDPOINT *****/

#[tracing::instrument(
    skip_all,
    fields(
        query = %query.q
    ),
    err(Debug)
)]
pub(crate) async fn search_coins(
    Query(query): Query<QueryParams>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<CoinResponse>>, SearchCoinsError> {
    let query = CoinSearchQuery::parse(query.q).map_err(SearchCoinsError::ValidationError)?;

    let rows = query_coins(&pool, query).await.context("Failed to fetch coins")?;

    let response = rows.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_coins(pool: &SqlitePool, query: CoinSearchQuery) -> Result<Vec<CoinRow>> {
    fn convert_to_fts5_format(query: &CoinSearchQuery) -> String {
        let query = query.as_ref();
        let mut formatted = String::new();

        for word in query.split_whitespace() {
            // Skip words that don't contain any alphanumeric characters
            if !word.chars().any(char::is_alphanumeric) {
                continue;
            }

            // Add a space between words
            if !formatted.is_empty() {
                formatted.push(' ');
            }

            formatted.push('"');
            for character in word.chars() {
                // Escape double quotes
                if character == '"' {
                    formatted.push('"');
                }
                formatted.push(character);
            }
            formatted.push_str("\"*");
        }

        // Output something in the form of "word1"* "word2"* "word3"*
        formatted
    }
    let fts5_formatted = convert_to_fts5_format(&query);

    let coins = sqlx::query_as!(
        CoinRow,
        r"
            SELECT
                c.id, c.numista_id, c.name, c.weight, c.size, c.thickness,
                c.min_year, c.max_year, c.composition, c.purity,
                c.obverse, c.reverse, c.edge,
                o.image_url     AS o_image_url,
                o.thumbnail_url AS o_thumbnail_url,
                o.lettering     AS o_lettering,
                o.description   AS o_description,
                o.copyright     AS o_copyright,
                r.image_url     AS r_image_url,
                r.thumbnail_url AS r_thumbnail_url,
                r.lettering     AS r_lettering,
                r.description   AS r_description,
                r.copyright     AS r_copyright,
                e.image_url     AS e_image_url,
                e.thumbnail_url AS e_thumbnail_url,
                e.lettering     AS e_lettering,
                e.description   AS e_description,
                e.copyright     AS e_copyright
            FROM coins c
            JOIN coins_fts ON coins_fts.rowid = c.id
            LEFT JOIN coin_images o ON o.id = c.obverse
            LEFT JOIN coin_images r ON r.id = c.reverse
            LEFT JOIN coin_images e ON e.id = c.edge
            WHERE coins_fts MATCH $1
            ORDER BY bm25(coins_fts)
        ",
        fts5_formatted
    )
    .fetch_all(pool)
    .await?;

    Ok(coins)
}

/***** ERRORS *****/

#[derive(thiserror::Error, api_error_derive::ApiError)]
pub(crate) enum SearchCoinsError {
    #[error("{0}")]
    #[status(BAD_REQUEST)]
    ValidationError(String),
    #[error(transparent)]
    #[status(INTERNAL_SERVER_ERROR)]
    UnexpectedError(#[from] anyhow::Error),
}
