use std::error::Error;

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::database::generated::api_keys::Model as ApiKeyModel;
use crate::database::generated::prelude::ApiKeys;
use crate::database::generated::prelude::Prices;
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
}
