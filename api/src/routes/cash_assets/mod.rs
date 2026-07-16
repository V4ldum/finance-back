use anyhow::Result;
use serde::Serialize;
use sqlx::SqlitePool;

mod create;
mod delete;
mod get;
mod update;

pub(crate) use create::create_cash_asset;
pub(crate) use delete::delete_cash_asset;
pub(crate) use get::get_cash_asset;
pub(crate) use update::update_cash_asset;

/***** DATABASE *****/

pub(crate) struct CashAsset {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_value: i64,
}

#[tracing::instrument(skip_all)]
async fn query_cash_asset(pool: &SqlitePool, asset_id: i64, user_id: i64) -> Result<Option<CashAsset>> {
    let cash_asset = sqlx::query_as!(
        CashAsset,
        "SELECT id, name, possessed, unit_value FROM cash_assets WHERE id = $1 AND user_id = $2",
        asset_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(cash_asset)
}

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct CashAssetResponse {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_value: i64,
}
