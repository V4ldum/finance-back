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
