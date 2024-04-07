use serde::Serialize;

#[derive(Serialize)]
pub struct TradeValues {
    pub gold: TradeValue,
    pub silver: TradeValue,
    pub sp_500: TradeValue,
}
#[derive(Serialize)]
pub struct TradeValue {
    pub price: f64,
    pub last_update: String,
}
