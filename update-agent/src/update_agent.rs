use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::json;

use crate::domain::{currency_price::EURUSDExchangeRate, metal_price::MetalPrice, stock_price::SP500Price};

const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/114.0.5735.99 Mobile/15E148 Safari/604.1";
const EUR_CURRENCY_SYMBOL: &str = "EUR";
const GOLD_SYMBOL: &str = "AU";
const SILVER_SYMBOL: &str = "AG";
const SP500_SYMBOL: &str = "^GSPC";

pub struct UpdateAgent {}

impl UpdateAgent {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_gold_price(&self) -> Result<MetalPrice> {
        self.get_metal_price(EUR_CURRENCY_SYMBOL, GOLD_SYMBOL).await
    }

    pub async fn get_silver_price(&self) -> Result<MetalPrice> {
        self.get_metal_price(EUR_CURRENCY_SYMBOL, SILVER_SYMBOL).await
    }

    pub async fn get_sp500_price(&self) -> Result<SP500Price> {
        let result = Client::builder()
            .user_agent(USER_AGENT)
            .build()?
            .get(format!(
                "https://query2.finance.yahoo.com/v8/finance/chart/{SP500_SYMBOL}"
            ))
            .send()
            .await?;

        serde_json::from_str::<SP500Price>(&result.text().await?).map_err(|err| err.into())
    }

    async fn get_metal_price(&self, fiat_currency_symbol: &str, metal_symbol: &str) -> Result<MetalPrice> {
        let timestamp = Utc::now().timestamp();

        let json_body = json!({
            "query": r"fragment MetalFragment on Metal { name results { ...MetalQuoteFragment } } fragment MetalQuoteFragment on Quote { bid change changePercentage } query MetalQuote( $symbol: String! $currency: String! $timestamp: Int ) { GetMetalQuote( symbol: $symbol currency: $currency timestamp: $timestamp ) { ...MetalFragment } }",
            "variables": {
                "symbol": metal_symbol,
                "currency": fiat_currency_symbol,
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

    pub async fn get_usd_to_eur_exchange_rate(&self) -> Result<EURUSDExchangeRate> {
        let result = Client::builder()
            .user_agent(USER_AGENT)
            .build()?
            .get("https://query1.finance.yahoo.com/v8/finance/chart/EURUSD=X")
            .send()
            .await?;

        serde_json::from_str::<EURUSDExchangeRate>(&result.text().await?).map_err(|err| err.into())
    }
}
