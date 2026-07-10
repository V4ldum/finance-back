use anyhow::Result;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::model::raw_asset::RawAsset;

mod create;
mod delete;
mod get;
mod update;

pub(crate) use create::create_raw_asset;
pub(crate) use delete::delete_raw_asset;
pub(crate) use get::get_raw_asset;
pub(crate) use update::update_raw_asset;

/***** DATABASE *****/

#[tracing::instrument(skip_all)]
async fn query_raw_asset(pool: &SqlitePool, asset_id: i64, user_id: i64) -> Result<Option<RawAsset>> {
    let raw_asset = sqlx::query_as!(
        RawAsset,
        "SELECT * FROM raw_assets WHERE id = $1 AND user_id = $2",
        asset_id,
        user_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(raw_asset)
}

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct RawAssetResponse {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_weight: i64,
    pub(crate) composition: String,
    pub(crate) purity: i64,
}
