use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SP500Price {
    pub chart: SP500PriceChart,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceChart {
    pub result: Vec<SP500PriceResult>,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResult {
    pub indicators: SP500PriceResultIndicator,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResultIndicator {
    pub quote: Vec<SP500PriceResultIndicatorQuote>,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResultIndicatorQuote {
    pub close: Vec<Option<f64>>,
}

impl SP500Price {
    pub fn price(&self) -> Option<f64> {
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
