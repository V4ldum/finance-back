use std::error::Error;

use reqwest::Client;

use sp500_price::SP500Price;

mod sp500_price;

pub async fn get_sp500_price() -> Result<SP500Price, Box<dyn Error>> {
    const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/114.0.5735.99 Mobile/15E148 Safari/604.1";
    let symbol = "^GSPC";

    let result = Client::builder()
        .user_agent(USER_AGENT)
        .build()?
        .get(format!("https://query2.finance.yahoo.com/v8/finance/chart/{}", symbol))
        .send()
        .await?;

    serde_json::from_str::<SP500Price>(&result.text().await?).map_err(|err| err.into())
}
