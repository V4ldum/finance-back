use serde::Serialize;

use crate::utils::dto::coins_dto::CoinDataDto;

#[derive(Serialize)]
pub struct AssetsDto {
    pub raw_assets: Vec<RawAssetsDto>,
    pub cash_assets: Vec<CashAssetsDto>,
    pub coin_assets: Vec<CoinAssetsDto>,
}

#[derive(Serialize)]
pub struct RawAssetsDto {
    pub id: i64,
    pub name: String,
    pub possessed: i64,
    pub unit_weight: i64,
    pub composition: String,
    pub purity: i64,
}

#[derive(Serialize)]
pub struct CashAssetsDto {
    pub id: i64,
    pub name: String,
    pub possessed: i64,
    pub unit_value: i64,
}

#[derive(Serialize)]
pub struct CoinAssetsDto {
    pub possessed: i64,
    pub coin_data: CoinDataDto,
}
