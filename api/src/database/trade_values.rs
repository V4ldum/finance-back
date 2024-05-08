use std::error::Error;

use sea_orm::EntityTrait;

use crate::database::Database;
use crate::database::generated::prelude::Prices;
use crate::database::generated::prices::Model as PricesModel;

impl Database {
    pub async fn find_one_trade_value(
        &self,
        query: &str,
    ) -> Result<Option<PricesModel>, Box<dyn Error>> {
        let result = Prices::find_by_id(query).one(&self.db).await?;

        Ok(result)
    }

    pub async fn find_all_trade_values(&self) -> Result<Vec<PricesModel>, Box<dyn Error>> {
        let result = Prices::find().all(&self.db).await?;

        Ok(result)
    }
}
