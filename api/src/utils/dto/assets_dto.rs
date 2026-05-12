use serde::Serialize;

use crate::utils::dto::coins_dto::CoinDataDto;

#[derive(Serialize)]
pub(crate) struct AssetsDto {
    pub(crate) raw_assets: Vec<RawAssetsDto>,
    pub(crate) cash_assets: Vec<CashAssetsDto>,
    pub(crate) coin_assets: Vec<CoinAssetsDto>,
}

#[derive(Serialize)]
pub(crate) struct RawAssetsDto {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_weight: i64,
    pub(crate) composition: String,
    pub(crate) purity: i64,
}

#[derive(Serialize)]
pub(crate) struct CashAssetsDto {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_value: i64,
}

#[derive(Serialize)]
pub(crate) struct CoinAssetsDto {
    pub(crate) possessed: i64,
    pub(crate) coin_data: CoinDataDto,
}
