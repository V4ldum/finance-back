use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct CurrenciesPrice {
    pub data: CurrenciesPriceData,
}

#[derive(Deserialize, Debug)]
pub struct CurrenciesPriceData {
    #[serde(rename = "GetBarchartFuturesByExchange")]
    pub quote: CurrenciesPriceQuote,
}

#[derive(Deserialize, Debug)]
pub struct CurrenciesPriceQuote {
    #[serde(rename = "results", deserialize_with = "deserialize_results")]
    pub result: CurrenciesPriceResult,
}

#[derive(Deserialize, Debug)]
pub struct CurrenciesPriceResult {
    pub name: String,
    //pub symbol: String,
    #[serde(rename = "lastPrice")]
    pub last_price: f64,
}

fn deserialize_results<'de, D>(deserializer: D) -> Result<CurrenciesPriceResult, D::Error>
where
    D: Deserializer<'de>,
{
    let result_vec = Vec::<CurrenciesPriceResult>::deserialize(deserializer)?;
    // Filter to only show Euro FX
    let result_vec: Vec<_> = result_vec
        .into_iter()
        .filter(|e| e.name.contains("Euro FX") && !e.name.contains("Pit"))
        .collect();

    if result_vec.is_empty() {
        return Err(serde::de::Error::custom(format!(
            "Expected a CurrenciesPriceResult, found {}",
            result_vec.len()
        )));
    }

    // First element should always be the most recent value for the currency
    Ok(result_vec
        .into_iter()
        .next()
        .expect("There should be one element here"))
}
