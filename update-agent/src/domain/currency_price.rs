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

impl EURUSDExchangeRate {
    pub fn exchange_rate(&self) -> Option<f64> {
        self.chart
            .result
            .first()?
            .indicators
            .quote
            .first()?
            .close
            .iter()
            .rfind(|item| item.is_some())?
            .to_owned()
    }
}
