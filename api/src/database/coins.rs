use std::error::Error;

use crate::database::tables::coin::Coin;
use crate::database::tables::coin_image::CoinImage;
use crate::database::Database;

impl Database {
    pub async fn search_coin(&self, query: &str) -> Result<Vec<Coin>, Box<dyn Error>> {
        // TODO migrate back to query_as! macro then comptime extension is merged :
        // - https://github.com/launchbadge/sqlx/issues/3330
        // - https://github.com/launchbadge/sqlx/pull/3713
        let result = sqlx::query_as("SELECT * FROM coins WHERE instr(UNACCENT(LOWER(name)), UNACCENT(LOWER(?))) > 0")
            .bind(query)
            .fetch_all(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn get_coin_side(&self, side_id: i64) -> Result<Option<CoinImage>, Box<dyn Error>> {
        let result = sqlx::query_as!(CoinImage, "SELECT * FROM coin_images WHERE id = $1", side_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn find_coin(&self, id: i64) -> Result<Option<Coin>, Box<dyn Error>> {
        let result = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE id = $1", id)
            .fetch_optional(&self.db)
            .await?;

        Ok(result)
    }
}
