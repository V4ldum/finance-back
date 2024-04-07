use std::error::Error;

use reqwest::Client;
use serde_json::json;

use crate::currencies_values::currencies_price::CurrenciesPrice;

mod currencies_price;

pub async fn get_usd_to_eur_exchange_rate() -> Result<CurrenciesPrice, Box<dyn Error>> {
    let json_body = json!({
        "query": r"query BarchartsFuturesByExchange( $exchange: String!, $category: String! ) { GetBarchartFuturesByExchange( exchange: $exchange, category: $category ) {  results { name, symbol, lastPrice, } } }",
        "variables": {
            "category": "Currencies",
            "exchange": "CME",
            "name": "Euro FX",
        },
    });

    let result = Client::new()
        .post("https://kitco-gcdn-prod.stellate.sh/")
        .json(&json_body)
        .send()
        .await?;

    serde_json::from_str::<CurrenciesPrice>(&result.text().await?).map_err(|err| err.into())
}
