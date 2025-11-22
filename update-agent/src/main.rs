use crate::database::Database;
use crate::values::{currencies_values, precious_metals_values, stocks_values};

mod database;
mod values;

#[tokio::main]
async fn main() {
    // TODO re-query a certain amount of time in case of error

    // Query values from the interwebs
    let gold_price = precious_metals_values::get_metal_price("AU").await;
    let silver_price = precious_metals_values::get_metal_price("AG").await;
    let sp500_price = stocks_values::get_sp500_price().await;

    // Save them in local db
    let database = Database::build().await.expect("Failed to build the database");

    match gold_price {
        Ok(gold_price) => {
            let gold_result = database.update_value("Gold", gold_price.data.quote.result.bid).await;

            if let Err(err) = gold_result {
                eprintln!("An error occurred updating gold price : {err}");
            }
        }
        Err(err) => eprintln!("An error occurred with Gold: {err:#?}"),
    }

    match silver_price {
        Ok(silver_price) => {
            let silver_result = database
                .update_value("Silver", silver_price.data.quote.result.bid)
                .await;

            if let Err(err) = silver_result {
                eprintln!("An error occurred updating silver price : {err}");
            }
        }
        Err(err) => eprintln!("An error occurred with Silver: {err:#?}"),
    }

    match sp500_price {
        Ok(sp500_price) => {
            let currencies_price = currencies_values::get_usd_to_eur_exchange_rate().await;

            match currencies_price {
                Ok(currencies_price) => {
                    let close_value = sp500_price
                        .chart
                        .result
                        .first()
                        .expect("Failed to find SP500PriceResult")
                        .indicators
                        .quote
                        .first()
                        .expect("Failed to find SP500PriceResultIndicatorQuote")
                        .close
                        .iter()
                        .filter(|item| item.is_some())
                        .last()
                        .expect("Failed to find a value in SP500PriceResultIndicatorQuote")
                        .expect("We filtered out None");
                    let change_rate = currencies_price
                        .chart
                        .result
                        .first()
                        .expect("Failed to find EURUSDExchangeRateResult")
                        .indicators
                        .quote
                        .first()
                        .expect("Failed to find EURUSDExchangeRateResultIndicatorQuote")
                        .close
                        .iter()
                        .filter(|item| item.is_some())
                        .last()
                        .expect("Failed to find a value in EURUSDExchangeRateResultIndicatorQuote")
                        .expect("There should be a value since we filtered all None");

                    let sp_result = database
                        .update_value(
                            "SP500",
                            // EUR = USD / Rate, SP500 quote is in USD
                            close_value / change_rate,
                        )
                        .await;
                    if let Err(err) = sp_result {
                        eprintln!("An error occurred updating SP500 price : {err}");
                    }
                }
                Err(err) => eprintln!("{err:#?}"),
            }
        }
        Err(err) => eprintln!("An error occurred with SP500: {err:#?}"),
    }
}
