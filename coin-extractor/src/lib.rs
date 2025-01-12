use std::error::Error;

use crate::coin_query::{CoinQuery, CoinQuerySide};
use crate::program_parameters::ProgramParameters;
use reqwest::Client;
use sqlx::SqliteConnection;

mod coin_query;
pub mod program_parameters;

pub async fn run(mut params: ProgramParameters) -> Result<(), Box<dyn Error>> {
    let response = Client::new()
        .get(format!(
            "https://api.numista.com/api/v3/types/{}?lang=fr",
            params.coin_id
        ))
        .header("Numista-API-Key", params.api_key)
        .send()
        .await?;

    let coin = serde_json::from_str::<CoinQuery>(&response.text().await?)?;

    if coin.watermark.is_some() {
        panic!("FOUND WATERMARK FOR {}", coin.title);
    }

    insert_in_database(coin, params.coin_id, &mut params.db).await?;

    Ok(())
}

async fn insert_in_database(coin: CoinQuery, numista_id: u32, db: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    let obverse_id = match coin.obverse {
        Some(obverse) => {
            let id = insert_coin_side(obverse, db).await?;
            Some(id)
        }
        None => None,
    };
    let reverse_id = match coin.reverse {
        Some(reverse) => {
            let id = insert_coin_side(reverse, db).await?;
            Some(id)
        }
        None => None,
    };
    let edge_id = match coin.edge {
        Some(edge) => {
            let id = insert_coin_side(edge, db).await?;
            Some(id)
        }
        None => None,
    };
    // let watermark_id = match coin.watermark {
    //     Some(watermark) => {
    //         let model = insert_coin_side(watermark, &db).await?;
    //         Some(id)
    //     },
    //     None => None,
    // };

    sqlx::query!(
        r#"
        INSERT INTO coins (numista_id, name, weight, size, thickness, min_year, max_year, composition, purity, obverse, reverse, edge)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        numista_id,
        coin.title,
        coin.weight,
        coin.size,
        coin.thickness,
        coin.min_year,
        coin.max_year,
        coin.composition.composition,
        coin.composition.purity,
        obverse_id,
        reverse_id,
        edge_id
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn insert_coin_side(side: CoinQuerySide, db: &mut SqliteConnection) -> Result<i64, Box<dyn Error>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO coin_images (image_url, thumbnail_url, lettering, description, copyright)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        side.picture,
        side.thumbnail,
        side.lettering,
        side.description,
        side.picture_copyright
    )
    .fetch_one(db)
    .await?;

    Ok(result.id)
}
