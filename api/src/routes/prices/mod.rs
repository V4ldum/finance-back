use serde::Serialize;

use crate::model::price::PriceDb;

mod get_all;
mod get_one;

pub(crate) use get_all::get_all_prices;
pub(crate) use get_one::get_one_price;

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct PriceResponse {
    price: f64,
    last_update: String,
}

impl From<&PriceDb> for PriceResponse {
    fn from(price: &PriceDb) -> Self {
        Self {
            price: price.value,
            last_update: price.date.to_string(),
        }
    }
}
