use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::json;

use crate::domain::{currency_price::EURUSDExchangeRate, metal_price::MetalPrice, stock_price::SP500Price};

const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/114.0.5735.99 Mobile/15E148 Safari/604.1";
const EUR_CURRENCY_SYMBOL: &str = "EUR";
const GOLD_SYMBOL: &str = "AU";
const SILVER_SYMBOL: &str = "AG";
const SP500_SYMBOL: &str = "^GSPC";

pub struct UpdateAgent {
    metal: String,
    sp500: String,
    exchange_rate: String,
}

impl UpdateAgent {
    pub fn new() -> Self {
        Self {
            metal: "https://kdb-gw.prod.kitco.com/".to_string(),
            sp500: format!("https://query2.finance.yahoo.com/v8/finance/chart/{SP500_SYMBOL}"),
            exchange_rate: "https://query1.finance.yahoo.com/v8/finance/chart/EURUSD=X".to_string(),
        }
    }

    pub async fn get_gold_price(&self) -> Result<MetalPrice> {
        self.get_metal_price(EUR_CURRENCY_SYMBOL, GOLD_SYMBOL).await
    }

    pub async fn get_silver_price(&self) -> Result<MetalPrice> {
        self.get_metal_price(EUR_CURRENCY_SYMBOL, SILVER_SYMBOL).await
    }

    pub async fn get_sp500_price(&self) -> Result<SP500Price> {
        let result = Client::builder()
            .user_agent(USER_AGENT)
            .build()?
            .get(&self.sp500)
            .send()
            .await?;

        serde_json::from_str::<SP500Price>(&result.text().await?).map_err(|err| err.into())
    }

    async fn get_metal_price(&self, fiat_currency_symbol: &str, metal_symbol: &str) -> Result<MetalPrice> {
        let timestamp = Utc::now().timestamp();

        let json_body = json!({
            "query": r"fragment MetalFragment on Metal { name results { ...MetalQuoteFragment } } fragment MetalQuoteFragment on Quote { bid change changePercentage } query MetalQuote( $symbol: String! $currency: String! $timestamp: Int ) { GetMetalQuote( symbol: $symbol currency: $currency timestamp: $timestamp ) { ...MetalFragment } }",
            "variables": {
                "symbol": metal_symbol,
                "currency": fiat_currency_symbol,
                "timestamp": timestamp,
            },
        });

        let result = Client::new().post(&self.metal).json(&json_body).send().await?;

        serde_json::from_str::<MetalPrice>(&result.text().await?).map_err(|err| err.into())
    }

    pub async fn get_usd_to_eur_exchange_rate(&self) -> Result<EURUSDExchangeRate> {
        let result = Client::builder()
            .user_agent(USER_AGENT)
            .build()?
            .get(&self.exchange_rate)
            .send()
            .await?;

        serde_json::from_str::<EURUSDExchangeRate>(&result.text().await?).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use claims::{assert_ok, assert_some};
    use fake::Fake;
    use serde_json::{Value, json};
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use crate::update_agent::{EUR_CURRENCY_SYMBOL, GOLD_SYMBOL, SILVER_SYMBOL, USER_AGENT, UpdateAgent};

    async fn mock_server(http_method: &str, body: Value) -> MockServer {
        let mock_server = MockServer::start().await;

        Mock::given(method(http_method))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(&mock_server)
            .await;

        mock_server
    }

    fn update_agent(mock_url: &str) -> UpdateAgent {
        let mut update_agent = UpdateAgent::new();

        // override URLs to mock server
        update_agent.metal = mock_url.to_string();
        update_agent.sp500 = mock_url.to_string();
        update_agent.exchange_rate = mock_url.to_string();

        update_agent
    }

    fn metal_price_body(bid: f64) -> Value {
        json!({ "data": { "GetMetalQuote": { "results": [{ "bid": bid }] } } })
    }

    // Both the SP500 and the exchange rate endpoints share this Yahoo chart shape
    fn sp500_body(close: f64) -> Value {
        json!({ "chart": { "result": [{ "indicators": { "quote": [{ "close": [close] }] } }] } })
    }

    #[tokio::test]
    async fn get_gold_price_fires_to_url() {
        // Arrange
        let bid = (2000.0..5000.0).fake::<f64>();
        let mock_server = mock_server("POST", metal_price_body(bid)).await;
        let update_agent = update_agent(&mock_server.uri());

        // Act
        let result = update_agent.get_gold_price().await;

        // Assert
        let price = assert_ok!(result);
        assert_relative_eq!(price.price(), bid);
    }

    #[tokio::test]
    async fn get_silver_price_fires_to_url() {
        // Arrange
        let bid = (20.0..100.0).fake::<f64>();
        let mock_server = mock_server("POST", metal_price_body(bid)).await;
        let update_agent = update_agent(&mock_server.uri());

        // Act
        let result = update_agent.get_silver_price().await;

        // Assert
        let price = assert_ok!(result);
        assert_relative_eq!(price.price(), bid);
    }

    #[tokio::test]
    async fn get_metal_price_gold_fires_to_url() {
        // Arrange
        let bid = (2000.0..5000.0).fake::<f64>();
        let mock_server = mock_server("POST", metal_price_body(bid)).await;
        let update_agent = update_agent(&mock_server.uri());

        // Act
        let result = update_agent.get_metal_price(EUR_CURRENCY_SYMBOL, GOLD_SYMBOL).await;

        // Assert
        let price = assert_ok!(result);
        assert_relative_eq!(price.price(), bid);
    }

    #[tokio::test]
    async fn get_metal_price_silver_fires_to_url() {
        // Arrange
        let bid = (20.0..100.0).fake::<f64>();
        let mock_server = mock_server("POST", metal_price_body(bid)).await;
        let update_agent = update_agent(&mock_server.uri());

        // Act
        let result = update_agent.get_metal_price(EUR_CURRENCY_SYMBOL, SILVER_SYMBOL).await;

        // Assert
        let price = assert_ok!(result);
        assert_relative_eq!(price.price(), bid);
    }

    #[tokio::test]
    async fn get_sp500_price_fires_to_url_with_proper_user_agent() {
        // Arrange
        let close = (7000.0..10000.0).fake::<f64>();
        let mock_server = mock_server("GET", sp500_body(close)).await;
        let update_agent = update_agent(&mock_server.uri());

        // Act
        let result = update_agent.get_sp500_price().await;

        // Assert
        let price = assert_ok!(result);
        assert_relative_eq!(assert_some!(price.price()), close);

        let requests = mock_server
            .received_requests()
            .await
            .expect("Failed to retrieve received requests");
        let headers = &requests[0].headers;

        assert_eq!(headers["user-agent"], USER_AGENT);
    }

    #[tokio::test]
    async fn get_usd_to_eur_exchange_rate_fires_to_url_with_proper_user_agent() {
        // Arrange
        let rate = (0.5..1.5).fake::<f64>();
        let mock_server = mock_server("GET", sp500_body(rate)).await;
        let update_agent = update_agent(&mock_server.uri());

        // Act
        let result = update_agent.get_usd_to_eur_exchange_rate().await;

        // Assert
        let exchange_rate = assert_ok!(result);
        assert_relative_eq!(assert_some!(exchange_rate.exchange_rate()), rate);

        let requests = mock_server
            .received_requests()
            .await
            .expect("Failed to retrieve received requests");
        let headers = &requests[0].headers;

        assert_eq!(headers["user-agent"], USER_AGENT);
    }
}
