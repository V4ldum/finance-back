use std::error::Error;

use crate::database::tables::price::Price;
use crate::database::Database;

impl Database {
    pub async fn find_one_trade_value(&self, query: &str) -> Result<Option<Price>, Box<dyn Error>> {
        let result = sqlx::query_as!(Price, "SELECT * FROM prices WHERE name = $1", query)
            .fetch_optional(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn find_all_trade_values(&self) -> Result<Vec<Price>, Box<dyn Error>> {
        let result = sqlx::query_as!(Price, "SELECT * FROM prices")
            .fetch_all(&self.db)
            .await?;

        Ok(result)
    }
}
