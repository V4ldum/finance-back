use std::error::Error;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::Database;
use crate::database::generated::coin_images::Model as CoinImagesModel;
use crate::database::generated::coins;
use crate::database::generated::coins::Model as CoinsModel;
use crate::database::generated::prelude::{CoinImages, Coins};

impl Database {
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
}
