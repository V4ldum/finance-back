use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct CoinDataDto {
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
    pub(crate) obverse: Option<CoinSideDataDto>,
    pub(crate) reverse: Option<CoinSideDataDto>,
    pub(crate) edge: Option<CoinSideDataDto>,
}

#[derive(Serialize)]
pub(crate) struct CoinSideDataDto {
    pub(crate) image_url: Option<String>,
    pub(crate) thumbnail_url: Option<String>,
    pub(crate) lettering: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) copyright: Option<String>,
}
