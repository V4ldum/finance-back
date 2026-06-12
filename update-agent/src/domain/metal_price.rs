use serde::{Deserialize, Deserializer};

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
    // this is in EUR per troy ounces
    pub bid: f64,
    //pub change: f64,
    //#[serde(rename = "changePercentage")]
    //pub change_percentage: f64,
}

impl MetalPrice {
    pub fn price(&self) -> f64 {
        self.data.quote.result.bid
    }
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

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use claims::{assert_err, assert_ok};
    use fake::Fake;

    use super::{MetalPrice, MetalPriceData, MetalPriceQuote, MetalPriceResult, deserialize_results};

    #[test]
    fn price_returns_the_bid() {
        let bid = (500.0..3000.0).fake();
        let metal_price = MetalPrice {
            data: MetalPriceData {
                quote: MetalPriceQuote {
                    result: MetalPriceResult { bid },
                },
            },
        };

        assert_relative_eq!(metal_price.price(), bid);
    }

    #[test]
    fn deserialize_results_parses_a_single_result() {
        let bid: f64 = (500.0..3000.0).fake();
        let json = serde_json::json!([{ "bid": bid }]);
        let deserialized = deserialize_results(json);

        let deserialized = assert_ok!(deserialized);
        assert_relative_eq!(deserialized.bid, bid);
    }

    #[test]
    fn deserialize_results_rejects_no_result() {
        let json = serde_json::json!([]);
        let deserialized = deserialize_results(json);
        assert_err!(deserialized);
    }

    #[test]
    fn deserialize_results_rejects_multiple_results() {
        let bid: f64 = (500.0..3000.0).fake();
        let json = serde_json::json!([{ "bid": bid }, { "bid": bid }]);
        let deserialized = deserialize_results(json);
        assert_err!(deserialized);
    }
}
