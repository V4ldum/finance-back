use std::error::Error;

use crate::database::tables::coin::Coin;
use crate::database::tables::coin_image::CoinImage;
use crate::database::Database;

impl Database {
    pub async fn search_coin(&self, query: &str) -> Result<Vec<Coin>, Box<dyn Error>> {
        let result = sqlx::query!("SELECT * FROM coins WHERE instr(name, $1) > 0", query)
            .fetch_all(&self.db)
            .await?;

        Ok(result
            .into_iter()
            .map(|record| Coin {
                id: record.id,
                numista_id: record.numista_id,
                name: record.name,
                weight: record.weight,
                size: record.size,
                thickness: record.thickness,
                min_year: record.min_year,
                max_year: record.max_year,
                composition: record.composition,
                purity: record.purity,
                obverse_id: record.obverse,
                reverse_id: record.reverse,
                edge_id: record.edge,
            })
            .collect())
    }

    pub async fn get_coin_side(&self, side_id: i64) -> Result<Option<CoinImage>, Box<dyn Error>> {
        let result = sqlx::query!("SELECT * FROM coin_images WHERE id = $1", side_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.map(|record| CoinImage {
            id: record.id,
            image_url: record.image_url,
            thumbnail_url: record.thumbnail_url,
            lettering: record.lettering,
            description: record.description,
            copyright: record.copyright,
        }))
    }

    pub async fn find_coin(&self, id: i64) -> Result<Option<Coin>, Box<dyn Error>> {
        let result = sqlx::query!("SELECT * FROM coins WHERE id = $1", id)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.map(|record| Coin {
            id: record.id,
            numista_id: record.numista_id,
            name: record.name,
            weight: record.weight,
            size: record.size,
            thickness: record.thickness,
            min_year: record.min_year,
            max_year: record.max_year,
            composition: record.composition,
            purity: record.purity,
            obverse_id: record.obverse,
            reverse_id: record.reverse,
            edge_id: record.edge,
        }))
    }
}
