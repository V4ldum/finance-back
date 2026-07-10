use serde::Serialize;

mod get;
mod search;

pub(crate) use get::get_coin;
pub(crate) use search::search_coins;

/***** DATABASE *****/

// TODO remove the FromRow derive once the comptime UNACCENT extension is merged
// Check search_coins for details
#[derive(sqlx::FromRow)]
pub(crate) struct CoinRow {
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

/// The single place a `CoinResponse` is built from coin columns. A side exists iff
/// the coin holds its FK; the joined image columns are then guaranteed present by
/// referential integrity.
impl From<CoinRow> for CoinResponse {
    fn from(row: CoinRow) -> Self {
        Self {
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
            obverse: row.obverse.map(|_| CoinSideResponse {
                image_url: row.o_image_url,
                thumbnail_url: row.o_thumbnail_url,
                lettering: row.o_lettering,
                description: row.o_description,
                copyright: row.o_copyright,
            }),
            reverse: row.reverse.map(|_| CoinSideResponse {
                image_url: row.r_image_url,
                thumbnail_url: row.r_thumbnail_url,
                lettering: row.r_lettering,
                description: row.r_description,
                copyright: row.r_copyright,
            }),
            edge: row.edge.map(|_| CoinSideResponse {
                image_url: row.e_image_url,
                thumbnail_url: row.e_thumbnail_url,
                lettering: row.e_lettering,
                description: row.e_description,
                copyright: row.e_copyright,
            }),
        }
    }
}

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct CoinResponse {
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
    pub(crate) obverse: Option<CoinSideResponse>,
    pub(crate) reverse: Option<CoinSideResponse>,
    pub(crate) edge: Option<CoinSideResponse>,
}

#[derive(Serialize)]
pub(crate) struct CoinSideResponse {
    pub(crate) image_url: Option<String>,
    pub(crate) thumbnail_url: Option<String>,
    pub(crate) lettering: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) copyright: Option<String>,
}
