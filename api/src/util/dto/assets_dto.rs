use serde::Serialize;

use crate::util::dto::coins_dto::CoinDataDto;

#[derive(Serialize)]
pub struct AssetsDto {
    pub raw_assets: Vec<RawAssetsDto>,
    pub cash_assets: Vec<CashAssetsDto>,
    pub coin_assets: Vec<CoinAssetsDto>,
}

#[derive(Serialize)]
pub struct RawAssetsDto {
    pub id: i32,
    pub name: String,
    pub possessed: i32,
    pub unit_weight: i32,
    pub composition: String,
    pub purity: i32,
}

#[derive(Serialize)]
pub struct CashAssetsDto {
    pub id: i32,
    pub name: String,
    pub possessed: i32,
    pub unit_value: i32,
}

#[derive(Serialize)]
pub struct CoinAssetsDto {
    pub possessed: i32,
    pub coin_data: CoinDataDto,
}
