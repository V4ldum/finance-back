use serde::Serialize;

use crate::routes::coins::{CoinResponse, CoinRow};

mod create;
mod delete;
mod get;
mod update;

pub(crate) use create::create_coin_asset;
pub(crate) use delete::delete_coin_asset;
pub(crate) use get::get_coin_asset;
pub(crate) use update::update_coin_asset;

/***** DATABASE *****/

pub(crate) struct CoinAssetRow {
    pub(crate) possessed: i64,
    pub(crate) id: i64,
    pub(crate) numista_id: String,
    pub(crate) name: String,
    pub(crate) weight: f64,
    pub(crate) size: f64,
    pub(crate) thickness: Option<f64>,
    pub(crate) min_year: String,
    pub(crate) max_year: Option<String>,
    pub(crate) composition: String,
    pub(crate) purity: i64,
    // FKs
    pub(crate) obverse: Option<i64>,
    pub(crate) reverse: Option<i64>,
    pub(crate) edge: Option<i64>,
    // obverse
    pub(crate) o_image_url: Option<String>,
    pub(crate) o_thumbnail_url: Option<String>,
    pub(crate) o_lettering: Option<String>,
    pub(crate) o_description: Option<String>,
    pub(crate) o_copyright: Option<String>,
    // reverse
    pub(crate) r_image_url: Option<String>,
    pub(crate) r_thumbnail_url: Option<String>,
    pub(crate) r_lettering: Option<String>,
    pub(crate) r_description: Option<String>,
    pub(crate) r_copyright: Option<String>,
    // edge
    pub(crate) e_image_url: Option<String>,
    pub(crate) e_thumbnail_url: Option<String>,
    pub(crate) e_lettering: Option<String>,
    pub(crate) e_description: Option<String>,
    pub(crate) e_copyright: Option<String>,
}

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct CoinAssetResponse {
    pub(crate) possessed: i64,
    pub(crate) coin_data: CoinResponse,
}

impl From<CoinAssetRow> for CoinAssetResponse {
    fn from(row: CoinAssetRow) -> Self {
        Self {
            possessed: row.possessed,
            coin_data: CoinRow {
                id: row.id,
                numista_id: row.numista_id,
                name: row.name,
                weight: row.weight,
                size: row.size,
                thickness: row.thickness,
                min_year: row.min_year,
                max_year: row.max_year,
                composition: row.composition,
                purity: row.purity,
                // FKs
                obverse: row.obverse,
                reverse: row.reverse,
                edge: row.edge,
                // obverse
                o_image_url: row.o_image_url,
                o_thumbnail_url: row.o_thumbnail_url,
                o_lettering: row.o_lettering,
                o_description: row.o_description,
                o_copyright: row.o_copyright,
                // reverse
                r_image_url: row.r_image_url,
                r_thumbnail_url: row.r_thumbnail_url,
                r_lettering: row.r_lettering,
                r_description: row.r_description,
                r_copyright: row.r_copyright,
                // edge
                e_image_url: row.e_image_url,
                e_thumbnail_url: row.e_thumbnail_url,
                e_lettering: row.e_lettering,
                e_description: row.e_description,
                e_copyright: row.e_copyright,
            }
            .into(),
        }
    }
}
