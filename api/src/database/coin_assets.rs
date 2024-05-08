use std::error::Error;

use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::database::Database;
use crate::database::generated::coin_assets;
use crate::database::generated::coin_assets::Model as CoinAssetsModel;
use crate::database::generated::prelude::CoinAssets;

impl Database {
    pub async fn get_coin_assets(
        &self,
        id_user: i32,
    ) -> Result<Vec<CoinAssetsModel>, Box<dyn Error>> {
        let coin_assets = CoinAssets::find()
            .filter(coin_assets::Column::UserId.eq(id_user))
            .all(&self.db)
            .await?;

        Ok(coin_assets)
    }

    pub async fn find_coin_asset(
        &self,
        coin_id: i32,
        user_id: i32,
    ) -> Result<Option<CoinAssetsModel>, Box<dyn Error>> {
        let coin_asset = CoinAssets::find_by_id((coin_id, user_id))
            .one(&self.db)
            .await?;

        Ok(coin_asset)
    }

    pub async fn add_coin_asset(
        &self,
        coin_id: i32,
        user_id: i32,
        possessed: i32,
    ) -> Result<(), DbErr> {
        let add_coin_asset = coin_assets::ActiveModel {
            coin_id: Set(coin_id),
            user_id: Set(user_id),
            possessed: Set(possessed),
        };

        CoinAssets::insert(add_coin_asset).exec(&self.db).await?;

        Ok(())
    }
}
