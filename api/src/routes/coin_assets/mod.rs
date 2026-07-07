use crate::model::coin_asset::CoinAsset;
use anyhow::Result;
use sqlx::SqlitePool;

mod create;
mod delete;
mod get;
mod update;

pub(crate) use create::create_coin_asset;
pub(crate) use delete::delete_coin_asset;
pub(crate) use get::get_coin_asset;
pub(crate) use update::update_coin_asset;

#[tracing::instrument(skip_all)]
async fn query_coin_asset(pool: &SqlitePool, coin_id: i64, user_id: i64) -> Result<Option<CoinAsset>> {
    let coin_asset = sqlx::query_as!(
        CoinAsset,
        "SELECT * FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
        coin_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(coin_asset)
}
