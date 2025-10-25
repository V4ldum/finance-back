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
    pub close: Vec<f64>,
}
