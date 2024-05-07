use serde::Serialize;

#[derive(Serialize)]
pub struct CoinDataDto {
    pub id: i32,
    pub numista_id: String,
    pub name: String,
    pub weight: f64,
    pub size: f64,
    pub thickness: Option<f64>,
    pub min_year: String,
    pub max_year: Option<String>,
    pub composition: String,
    pub purity: i32,
    pub obverse: Option<CoinSideDataDto>,
    pub reverse: Option<CoinSideDataDto>,
    pub edge: Option<CoinSideDataDto>,
}

#[derive(Serialize)]
pub struct CoinSideDataDto {
    pub image_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub lettering: Option<String>,
    pub description: Option<String>,
    pub copyright: Option<String>,
}
