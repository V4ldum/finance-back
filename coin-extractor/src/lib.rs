use std::error::Error;

use reqwest::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use sea_orm::ActiveValue::Set;

use crate::coin_query::{CoinQuery, CoinQuerySide};
use crate::generated::coin_images::{
    ActiveModel as CoinImagesActiveModel, Model as CoinImagesModel,
};
use crate::generated::coins::ActiveModel as CoinsActiveModel;
use crate::program_parameters::ProgramParameters;

mod coin_query;
mod generated;
pub mod program_parameters;

pub async fn run(params: ProgramParameters) -> Result<(), Box<dyn Error>> {
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

    insert_in_database(coin, params.coin_id, params.db).await?;

    Ok(())
}

async fn insert_in_database(
    coin: CoinQuery,
    numista_id: u32,
    db: DatabaseConnection,
) -> Result<(), Box<dyn Error>> {
    let obverse_id = match coin.obverse {
        Some(obverse) => {
            let model = insert_coin_side(obverse, &db).await?;
            Some(model.id)
        }
        None => None,
    };
    let reverse_id = match coin.reverse {
        Some(reverse) => {
            let model = insert_coin_side(reverse, &db).await?;
            Some(model.id)
        }
        None => None,
    };
    let edge_id = match coin.edge {
        Some(edge) => {
            let model = insert_coin_side(edge, &db).await?;
            Some(model.id)
        }
        None => None,
    };
    // let watermark_id = match coin.watermark {
    //     Some(watermark) => {
    //         let model = insert_coin_side(watermark, &db).await?;
    //         Some(model.id)
    //     },
    //     None => None,
    // };

    CoinsActiveModel {
        numista_id: Set(numista_id.to_string()),
        name: Set(coin.title),
        weight: Set(coin.weight),
        size: Set(coin.size),
        thickness: Set(coin.thickness),
        min_year: Set(coin.min_year),
        max_year: Set(coin.max_year),
        composition: Set(coin.composition.composition),
        purity: Set(coin.composition.purity),
        obverse: Set(obverse_id),
        reverse: Set(reverse_id),
        edge: Set(edge_id),
        //watermark: Set(watermark_id),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(())
}

async fn insert_coin_side(
    side: CoinQuerySide,
    db: &DatabaseConnection,
) -> Result<CoinImagesModel, Box<dyn Error>> {
    let result = CoinImagesActiveModel {
        image_url: Set(side.picture),
        thumbnail_url: Set(side.thumbnail),
        lettering: Set(side.lettering),
        description: Set(side.description),
        copyright: Set(side.picture_copyright),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(result)
}
