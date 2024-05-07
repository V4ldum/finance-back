use crate::database::Database;
use crate::database::generated::coins::Model as CoinsModel;
use crate::util::api_error::APIError;
use crate::util::dto::coins_dto::{CoinDataDto, CoinSideDataDto};

pub async fn convert_coin_model_to_coin_response(
    coin: CoinsModel,
    database: &Database,
) -> Result<CoinDataDto, APIError> {
    let obverse = if let Some(obverse) = coin.obverse {
        let Ok(result) = database.get_coin_side(obverse).await else {
            return Err(APIError::database_error());
        };

        result
    } else {
        None
    };
    let reverse = if let Some(reverse) = coin.reverse {
        let Ok(result) = database.get_coin_side(reverse).await else {
            return Err(APIError::database_error());
        };

        result
    } else {
        None
    };
    let edge = if let Some(edge) = coin.edge {
        let Ok(result) = database.get_coin_side(edge).await else {
            return Err(APIError::database_error());
        };

        result
    } else {
        None
    };

    Ok(CoinDataDto {
        id: coin.id,
        numista_id: coin.numista_id,
        name: coin.name,
        weight: coin.weight,
        size: coin.size,
        thickness: coin.thickness,
        min_year: coin.min_year,
        max_year: coin.max_year,
        composition: coin.composition,
        purity: coin.purity,
        obverse: obverse.map(|obverse| CoinSideDataDto {
            image_url: obverse.image_url,
            thumbnail_url: obverse.thumbnail_url,
            lettering: obverse.lettering,
            description: obverse.description,
            copyright: obverse.copyright,
        }),
        reverse: reverse.map(|reverse| CoinSideDataDto {
            image_url: reverse.image_url,
            thumbnail_url: reverse.thumbnail_url,
            lettering: reverse.lettering,
            description: reverse.description,
            copyright: reverse.copyright,
        }),
        edge: edge.map(|edge| CoinSideDataDto {
            image_url: edge.image_url,
            thumbnail_url: edge.thumbnail_url,
            lettering: edge.lettering,
            description: edge.description,
            copyright: edge.copyright,
        }),
    })
}
