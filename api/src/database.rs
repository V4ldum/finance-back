use std::error::Error;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::generated::cash_assets::Model as CashAssetsModel;
use crate::database::generated::coin_assets::Model as CoinAssetsModel;
use crate::database::generated::coin_images::Model as CoinImagesModel;
use crate::database::generated::coins::Model as CoinsModel;
use crate::database::generated::prelude::{
    CashAssets, CoinAssets, CoinImages, Coins, Prices, RawAssets, Users,
};
use crate::database::generated::prices::Model as PriceModel;
use crate::database::generated::raw_assets::Model as RawAssetsModel;
use crate::database::generated::users::Model as UsersModel;
use crate::database::generated::{cash_assets, coin_assets, coins, raw_assets, users};

pub(crate) mod generated;

#[derive(Clone)]
pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub fn new(db: DatabaseConnection) -> Self {
        Database { db }
    }

    pub async fn get_user(&self, key: &str) -> Result<Option<UsersModel>, Box<dyn Error>> {
        let result = Users::find()
            .filter(users::Column::ApiKey.eq(key))
            .one(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn find_one_trade_value(
        &self,
        query: &str,
    ) -> Result<Option<PriceModel>, Box<dyn Error>> {
        let result = Prices::find_by_id(query).one(&self.db).await?;

        Ok(result)
    }

    pub async fn find_all_trade_values(&self) -> Result<Vec<PriceModel>, Box<dyn Error>> {
        let result = Prices::find().all(&self.db).await?;

        Ok(result)
    }

    pub async fn search_coin(&self, query: &str) -> Result<Vec<CoinsModel>, Box<dyn Error>> {
        let result = Coins::find()
            .filter(coins::Column::Name.contains(query))
            .all(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn get_coin_side(
        &self,
        side_id: i32,
    ) -> Result<Option<CoinImagesModel>, Box<dyn Error>> {
        let result = CoinImages::find_by_id(side_id).one(&self.db).await?;

        Ok(result)
    }

    pub async fn find_coin(&self, id: i32) -> Result<Option<CoinsModel>, Box<dyn Error>> {
        let result = Coins::find_by_id(id).one(&self.db).await?;

        Ok(result)
    }

    pub async fn get_raw_assets(
        &self,
        id_user: i32,
    ) -> Result<Vec<RawAssetsModel>, Box<dyn Error>> {
        let raw_assets = RawAssets::find()
            .filter(raw_assets::Column::IdUser.eq(id_user))
            .all(&self.db)
            .await?;

        Ok(raw_assets)
    }

    pub async fn get_cash_assets(
        &self,
        id_user: i32,
    ) -> Result<Vec<CashAssetsModel>, Box<dyn Error>> {
        let cash_assets = CashAssets::find()
            .filter(cash_assets::Column::IdUser.eq(id_user))
            .all(&self.db)
            .await?;

        Ok(cash_assets)
    }

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

    pub async fn find_raw_asset(
        &self,
        asset_id: i32,
        user_id: i32,
    ) -> Result<Option<RawAssetsModel>, Box<dyn Error>> {
        let asset = RawAssets::find_by_id(asset_id)
            .filter(raw_assets::Column::IdUser.eq(user_id))
            .one(&self.db)
            .await?;

        Ok(asset)
    }

    pub async fn find_cash_asset(
        &self,
        asset_id: i32,
        user_id: i32,
    ) -> Result<Option<CashAssetsModel>, Box<dyn Error>> {
        let asset = CashAssets::find_by_id(asset_id)
            .filter(cash_assets::Column::IdUser.eq(user_id))
            .one(&self.db)
            .await?;

        Ok(asset)
    }
}
