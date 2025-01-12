use crate::database::tables::raw_asset::RawAsset;
use crate::database::Database;
use sqlx::{Execute, QueryBuilder, Sqlite};
use std::error::Error;

impl Database {
    pub async fn get_raw_assets(&self, id_user: i64) -> Result<Vec<RawAsset>, Box<dyn Error>> {
        let raw_assets = sqlx::query!("SELECT * FROM raw_assets WHERE id_user = $1", id_user)
            .fetch_all(&self.db)
            .await?;

        Ok(raw_assets
            .into_iter()
            .map(|record| RawAsset {
                id: record.id,
                name: record.name,
                possessed: record.possessed,
                unit_weight: record.unit_weight,
                composition: record.composition,
                purity: record.purity,
                id_user: record.id_user,
            })
            .collect())
    }

    pub async fn find_raw_asset(&self, asset_id: i64, user_id: i64) -> Result<Option<RawAsset>, Box<dyn Error>> {
        let asset = sqlx::query!(
            "SELECT * FROM raw_assets WHERE id_user = $1 AND id = $2",
            user_id,
            asset_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(asset.map(|record| RawAsset {
            id: record.id,
            name: record.name,
            possessed: record.possessed,
            unit_weight: record.unit_weight,
            composition: record.composition,
            purity: record.purity,
            id_user: record.id_user,
        }))
    }

    pub async fn add_raw_asset(
        &self,
        name: String,
        possessed: i64,
        unit_weight: i64,
        composition: String,
        purity: i64,
        user_id: i64,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
            INSERT INTO raw_assets (name, possessed, unit_weight, composition, purity, id_user)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            name,
            possessed,
            unit_weight,
            composition,
            purity,
            user_id,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_raw_asset(
        &self,
        id: i64,
        user_id: i64,
        name: Option<String>,
        possessed: Option<i64>,
        unit_weight: Option<i64>,
        composition: Option<String>,
        purity: Option<i64>,
    ) -> Result<(), Box<dyn Error>> {
        match (name, possessed, unit_weight, composition, purity) {
            (None, None, None, None, None) => return Ok(()), // No update necessary
            (name, possessed, unit_weight, composition, purity) => {
                let mut query: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE raw_assets SET ");
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
                if let Some(unit_weight) = unit_weight {
                    if and {
                        query.push(", ");
                    }
                    query.push("unit_weight = ");
                    query.push_bind(unit_weight);
                    and = true;
                }
                if let Some(composition) = composition {
                    if and {
                        query.push(", ");
                    }
                    query.push("composition = ");
                    query.push_bind(composition);
                    and = true;
                }
                if let Some(purity) = purity {
                    if and {
                        query.push(", ");
                    }
                    query.push("purity = ");
                    query.push_bind(purity);
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

    pub async fn delete_raw_asset(&self, id: i64, user_id: i64) -> Result<(), Box<dyn Error>> {
        sqlx::query!("DELETE FROM raw_assets WHERE id = $1 AND id_user = $2", id, user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
