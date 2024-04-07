use crate::database::Database;

mod currencies_values;
mod database;
mod precious_metals_values;
mod stocks_values;

#[tokio::main]
async fn main() {
    // TODO requery a certain amount of time in case of error

    // Query values from the interwebs
    let gold_price = precious_metals_values::get_metal_price("AU").await;
    let silver_price = precious_metals_values::get_metal_price("AG").await;
    let sp500_price = stocks_values::get_sp500_price().await;

    // Save them in local database
    let database = Database::build().unwrap();

    match gold_price {
        Ok(gold_price) => {
            let gold_result = database.update_value("Gold", gold_price.data.quote.result.bid);

            if let Err(err) = gold_result {
                eprintln!("An error occurred updating gold price : {err}");
            }
        }
        Err(err) => eprintln!("{err:#?}"),
    }

    match silver_price {
        Ok(silver_price) => {
            let silver_result = database.update_value("Silver", silver_price.data.quote.result.bid);

            if let Err(err) = silver_result {
                eprintln!("An error occurred updating silver price : {err}");
            }
        }
        Err(err) => eprintln!("{err:#?}"),
    }

    match sp500_price {
        Ok(sp500_price) => {
            let currencies_price = currencies_values::get_usd_to_eur_exchange_rate().await;

            match currencies_price {
                Ok(currencies_price) => {
                    let sp_result = database.update_value(
                        "SP500",
                        // EUR = USD / Rate, SP500 quote is in USD
                        sp500_price.data.quote.result.last_price
                            / currencies_price.data.quote.result.last_price,
                    );
                    if let Err(err) = sp_result {
                        eprintln!("An error occurred updating SP500 price : {err}");
                    }
                }
                Err(err) => eprintln!("{err:#?}"),
            }
        }
        Err(err) => eprintln!("{err:#?}"),
    }
}
