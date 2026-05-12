use crate::database::Database;
use crate::database::tables::coin_asset::CoinAsset;
use std::error::Error;

impl Database {
    pub(crate) async fn get_coin_assets(&self, id_user: i64) -> Result<Vec<CoinAsset>, Box<dyn Error>> {
        let coin_assets = sqlx::query_as!(CoinAsset, "SELECT * FROM coin_assets WHERE user_id = $1", id_user)
            .fetch_all(&self.db)
            .await?;

        Ok(coin_assets)
    }

    pub(crate) async fn find_coin_asset(
        &self,
        coin_id: i64,
        user_id: i64,
    ) -> Result<Option<CoinAsset>, Box<dyn Error>> {
        let coin_asset = sqlx::query_as!(
            CoinAsset,
            "SELECT * FROM coin_assets WHERE coin_id = $1 AND user_id = $2",
            coin_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(coin_asset)
    }

    pub(crate) async fn add_coin_asset(&self, coin_id: i64, user_id: i64, possessed: i64) -> Result<(), sqlx::Error> {
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

    pub(crate) async fn update_coin_asset(
        &self,
        coin_id: i64,
        user_id: i64,
        possessed: i64,
    ) -> Result<(), Box<dyn Error>> {
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

    pub(crate) async fn delete_coin_asset(&self, coin_id: i64, user_id: i64) -> Result<(), Box<dyn Error>> {
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
