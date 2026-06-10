use std::error::Error;

use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Deserializer};
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct MetalPrice {
    pub data: MetalPriceData,
}

#[derive(Deserialize, Debug)]
pub struct MetalPriceData {
    #[serde(rename = "GetMetalQuote")]
    pub quote: MetalPriceQuote,
}

#[derive(Deserialize, Debug)]
pub struct MetalPriceQuote {
    //pub name: String,
    #[serde(rename = "results", deserialize_with = "deserialize_results")]
    pub result: MetalPriceResult,
}

#[derive(Deserialize, Debug)]
pub struct MetalPriceResult {
    pub bid: f64, // this is in EUR per troy ounces
                  //pub change: f64,
                  //#[serde(rename = "changePercentage")]
                  //pub change_percentage: f64,
}

fn deserialize_results<'de, D>(deserializer: D) -> Result<MetalPriceResult, D::Error>
where
    D: Deserializer<'de>,
{
    let result_vec = Vec::<MetalPriceResult>::deserialize(deserializer)?;
    if result_vec.len() != 1 {
        return Err(serde::de::Error::custom(format!(
            "Expected a single MetalPriceResult, found {}",
            result_vec.len()
        )));
    }
    Ok(result_vec.into_iter().next().expect("There should be one element here"))
}

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
        .post("https://kdb-gw.prod.kitco.com/")
        .json(&json_body)
        .send()
        .await?;

    serde_json::from_str::<MetalPrice>(&result.text().await?).map_err(|err| err.into())
}
