use std::error::Error;

use chrono::Utc;
use reqwest::Client;
use serde_json::json;

pub use metal_price::*;

mod metal_price;

pub async fn get_metal_price(metal_symbol: &str) -> Result<MetalPrice, Box<dyn Error>> {
    let currency = "EUR";
    let timestamp = Utc::now().timestamp();

    let json_body = json!({
        "query": r"fragment MetalFragment on Metal { name results { ...MetalQuoteFragment } } fragment MetalQuoteFragment on Quote { bid change changePercentage } query MetalQuote( $symbol: String! $currency: String! $timestamp: Int ) { GetMetalQuote( symbol: $symbol currency: $currency timestamp: $timestamp ) { ...MetalFragment } }",
        "variables": {
            "symbol": metal_symbol,
            "currency": currency,
            "timestamp": timestamp,
        },
    });

    let result = Client::new()
        .post("https://kitco-gcdn-prod.stellate.sh/")
        .json(&json_body)
        .send()
        .await?;

    serde_json::from_str::<MetalPrice>(&result.text().await?).map_err(|err| err.into())
}
