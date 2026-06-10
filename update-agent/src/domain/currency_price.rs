use std::error::Error;

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRate {
    pub chart: EURUSDExchangeRateChart,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateChart {
    pub result: Vec<EURUSDExchangeRateResult>,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateResult {
    pub indicators: EURUSDExchangeRateResultIndicator,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateResultIndicator {
    pub quote: Vec<EURUSDExchangeRateQuote>,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateQuote {
    pub close: Vec<Option<f64>>,
}

pub async fn get_usd_to_eur_exchange_rate() -> Result<EURUSDExchangeRate, Box<dyn Error>> {
    const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/114.0.5735.99 Mobile/15E148 Safari/604.1";

    let result = Client::builder()
        .user_agent(USER_AGENT)
        .build()?
        .get("https://query1.finance.yahoo.com/v8/finance/chart/EURUSD=X")
        .send()
        .await?;

    serde_json::from_str::<EURUSDExchangeRate>(&result.text().await?).map_err(|err| err.into())
}
