use std::error::Error;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::generated::api_keys::Model as ApiKeyModel;
use crate::database::generated::coin_images::Model as CoinImagesModel;
use crate::database::generated::coins;
use crate::database::generated::coins::Model as CoinsModel;
use crate::database::generated::prelude::{ApiKeys, CoinImages, Coins, Prices};
use crate::database::generated::prices::Model as PriceModel;

mod generated;

#[derive(Clone)]
pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub fn new(db: DatabaseConnection) -> Self {
        Database { db }
    }

    pub async fn find_api_key(&self, key: &str) -> Result<Option<ApiKeyModel>, Box<dyn Error>> {
        let result = ApiKeys::find_by_id(key).one(&self.db).await?;

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
}
