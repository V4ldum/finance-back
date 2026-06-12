use crate::database::Database;
use crate::update_agent::UpdateAgent;
use anyhow::Result;

mod database;
mod domain;
mod update_agent;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO re-query a certain amount of time in case of error

    // Query values from the interwebs
    let update_agent = UpdateAgent::new();

    let gold = update_agent.get_gold_price().await;
    let silver = update_agent.get_silver_price().await;
    let sp500 = update_agent.get_sp500_price().await;

    // Save them in local db
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    let database = Database::build(&database_url).await?;

    match gold {
        Ok(gold) => {
            let gold_result = database.update_gold_price(gold.price()).await;

            if let Err(err) = gold_result {
                eprintln!("An error occurred updating gold price : {err:?}");
            }
        }
        Err(err) => eprintln!("An error occurred with Gold: {err:?}"),
    }

    match silver {
        Ok(silver) => {
            let silver_result = database.update_silver_price(silver.price()).await;

            if let Err(err) = silver_result {
                eprintln!("An error occurred updating silver price : {err:?}");
            }
        }
        Err(err) => eprintln!("An error occurred with Silver: {err:?}"),
    }

    match sp500 {
        Ok(sp500) => {
            let usd_to_eur = update_agent.get_usd_to_eur_exchange_rate().await;

            match usd_to_eur {
                Ok(usd_to_eur) => {
                    let close = sp500.price().expect("Failed to find a close value");
                    let exchange_rate = usd_to_eur.exchange_rate().expect("Failed to find exchange rate");

                    // EUR = USD / Rate, SP500 quote is in USD
                    let sp_result = database.update_sp500_price(close / exchange_rate).await;
                    if let Err(err) = sp_result {
                        eprintln!("An error occurred updating SP500 price : {err:?}");
                    }
                }
                Err(err) => eprintln!("{err:?}"),
            }
        }
        Err(err) => eprintln!("An error occurred with SP500: {err:?}"),
    }

    Ok(())
}
