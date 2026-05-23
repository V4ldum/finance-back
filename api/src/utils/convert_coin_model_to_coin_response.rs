use sqlx::SqlitePool;

use crate::model::coin::Coin;
use crate::model::coin_image::CoinImage;
use crate::utils::api_error::APIError;
use crate::utils::dto::coins_dto::{CoinDataDto, CoinSideDataDto};

pub(crate) async fn convert_coin_model_to_coin_response(
    coin: Coin,
    pool: &SqlitePool,
) -> Result<CoinDataDto, APIError> {
    let obverse = if let Some(obverse) = coin.obverse {
        match sqlx::query_as!(CoinImage, "SELECT * FROM coin_images WHERE id = $1", obverse)
            .fetch_optional(pool)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to execute query: {e:?}");
                return Err(APIError::database_error());
            }
        }
    } else {
        None
    };
    let reverse = if let Some(reverse) = coin.reverse {
        match sqlx::query_as!(CoinImage, "SELECT * FROM coin_images WHERE id = $1", reverse)
            .fetch_optional(pool)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to execute query: {e:?}");
                return Err(APIError::database_error());
            }
        }
    } else {
        None
    };
    let edge = if let Some(edge) = coin.edge {
        match sqlx::query_as!(CoinImage, "SELECT * FROM coin_images WHERE id = $1", edge)
            .fetch_optional(pool)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to execute query: {e:?}");
                return Err(APIError::database_error());
            }
        }
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
