use std::error::Error;

use chrono::Utc;
use reqwest::Client;
use serde_json::json;

use sp500_price::SP500Price;

mod sp500_price;

pub async fn get_sp500_price() -> Result<SP500Price, Box<dyn Error>> {
    let symbol = "$SPX";
    let timestamp = Utc::now().timestamp();

    let json_body = json!({
        "query": r"query BarChartsQuotes( $timestamp: Int!, $symbols: String! ) { GetBarchartQuotes(symbols: $symbols, timestamp: $timestamp) { results, { name, lastPrice, } } }",
        "variables": {
            "symbols": symbol,
            "timestamp": timestamp,
        },
    });

    let result = Client::new()
        .post("https://kitco-gcdn-prod.stellate.sh/")
        .json(&json_body)
        .send()
        .await?;

    serde_json::from_str::<SP500Price>(&result.text().await?).map_err(|err| err.into())
}
