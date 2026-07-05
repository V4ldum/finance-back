mod get_all;
mod get_one;

pub(crate) use get_all::get_all_prices;
pub(crate) use get_one::get_one_price;

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Price {
    price: f64,
    last_update: String,
}
