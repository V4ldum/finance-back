use crate::database::tables::cash_asset::CashAsset;
use crate::database::Database;
use sqlx::{Execute, QueryBuilder, Sqlite};
use std::error::Error;

impl Database {
    pub async fn get_cash_assets(&self, id_user: i64) -> Result<Vec<CashAsset>, Box<dyn Error>> {
        let cash_assets = sqlx::query_as!(CashAsset, "SELECT * FROM cash_assets WHERE id_user == $1", id_user)
            .fetch_all(&self.db)
            .await?;

        Ok(cash_assets)
    }

    pub async fn find_cash_asset(&self, asset_id: i64, user_id: i64) -> Result<Option<CashAsset>, Box<dyn Error>> {
        let asset = sqlx::query_as!(
            CashAsset,
            "SELECT * FROM cash_assets WHERE id = $1 AND id_user = $2",
            asset_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(asset)
    }

    pub async fn add_cash_asset(
        &self,
        name: String,
        possessed: i64,
        unit_value: i64,
        user_id: i64,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
            INSERT INTO cash_assets (name, possessed, unit_value, id_user)
            VALUES ($1, $2, $3, $4)
            "#,
            name,
            possessed,
            unit_value,
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn update_cash_asset(
        &self,
        id: i64,
        user_id: i64,
        name: Option<String>,
        possessed: Option<i64>,
        unit_value: Option<i64>,
    ) -> Result<(), Box<dyn Error>> {
        match (name, possessed, unit_value) {
            (None, None, None) => return Ok(()), // No update necessary
            (name, possessed, unit_value) => {
                let mut query: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE cash_assets SET ");
                let mut and = false;

                if let Some(name) = name {
                    query.push("name = ");
                    query.push_bind(name);
                    and = true;
                }
                if let Some(possessed) = possessed {
                    if and {
                        query.push(", ");
                    }
                    query.push("possessed = ");
                    query.push_bind(possessed);
                    and = true;
                }
                if let Some(unit_value) = unit_value {
                    if and {
                        query.push(", ");
                    }
                    query.push("unit_value = ");
                    query.push_bind(unit_value);
                }
                query.push(" WHERE id = ");
                query.push_bind(id);
                query.push(" AND id_user = ");
                query.push_bind(user_id);

                sqlx::query(query.build().sql()).execute(&self.db).await?;
            }
        }

        Ok(())
    }

    pub async fn delete_cash_asset(&self, id: i64, user_id: i64) -> Result<(), Box<dyn Error>> {
        sqlx::query!("DELETE FROM cash_assets WHERE id = $1 AND id_user = $2", id, user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
