use chrono::NaiveDate;
use serde::Serialize;

mod get_all;
mod get_one;

pub(crate) use get_all::get_all_prices;
pub(crate) use get_one::get_one_price;

/***** DATABASE *****/

pub(crate) struct Price {
    pub(crate) name: String,
    pub(crate) value: f64,
    pub(crate) date: NaiveDate,
}

/***** RESPONSE *****/

#[derive(Serialize)]
pub(crate) struct PriceResponse {
    price: f64,
    last_update: String,
}

impl From<&Price> for PriceResponse {
    fn from(price: &Price) -> Self {
        Self {
            price: price.value,
            last_update: price.date.to_string(),
        }
    }
}
