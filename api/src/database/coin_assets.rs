use crate::database::tables::coin_asset::CoinAsset;
use crate::database::Database;
use std::error::Error;

impl Database {
    pub async fn get_coin_assets(&self, id_user: i64) -> Result<Vec<CoinAsset>, Box<dyn Error>> {
        let coin_assets = sqlx::query!("SELECT * FROM coin_assets WHERE user_id = $1", id_user)
            .fetch_all(&self.db)
            .await?;

        Ok(coin_assets
            .into_iter()
            .map(|record| CoinAsset {
                coin_id: record.coin_id,
                user_id: record.user_id,
                possessed: record.possessed,
            })
            .collect())
    }

    pub async fn find_coin_asset(&self, coin_id: i64, user_id: i64) -> Result<Option<CoinAsset>, Box<dyn Error>> {
        let coin_asset = sqlx::query!(
            "SELECT * FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
            coin_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(coin_asset.map(|record| CoinAsset {
            coin_id: record.coin_id,
            user_id: record.user_id,
            possessed: record.possessed,
        }))
    }

    pub async fn add_coin_asset(&self, coin_id: i64, user_id: i64, possessed: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO coin_assets (coin_id, user_id, possessed)
            VALUES ($1, $2, $3)
            "#,
            coin_id,
            user_id,
            possessed
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn update_coin_asset(&self, coin_id: i64, user_id: i64, possessed: i64) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            "UPDATE coin_assets SET possessed = $1 WHERE coin_id = $2 AND user_id = $3",
            possessed,
            coin_id,
            user_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn delete_coin_asset(&self, coin_id: i64, user_id: i64) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            "DELETE FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
            coin_id,
            user_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
