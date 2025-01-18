use std::error::Error;

use crate::database::tables::coin::Coin;
use crate::database::tables::coin_image::CoinImage;
use crate::database::Database;

impl Database {
    pub async fn search_coin(&self, query: &str) -> Result<Vec<Coin>, Box<dyn Error>> {
        let result = sqlx::query_as!(Coin, "SELECT * FROM coins WHERE instr(name, $1) > 0", query)
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
