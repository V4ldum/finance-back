use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct SP500Price {
    pub data: SP500PriceData,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceData {
    #[serde(rename = "GetBarchartQuotes")]
    pub quote: SP500PriceQuote,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceQuote {
    #[serde(rename = "results", deserialize_with = "deserialize_results")]
    pub result: SP500PriceResult,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResult {
    // pub name: String,
    #[serde(rename = "lastPrice")]
    pub last_price: f64, // this is in USD
}

fn deserialize_results<'de, D>(deserializer: D) -> Result<SP500PriceResult, D::Error>
where
    D: Deserializer<'de>,
{
    let result_vec = Vec::<SP500PriceResult>::deserialize(deserializer)?;
    if result_vec.len() != 1 {
        return Err(serde::de::Error::custom(format!(
            "Expected a single SP500PriceResult, found {}",
            result_vec.len()
        )));
    }

    Ok(result_vec.into_iter().next().expect("There should be one element here"))
}
