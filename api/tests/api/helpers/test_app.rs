use sha3::Digest;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct TestUser {
    pub api_key: String,
}

impl TestUser {
    pub(super) fn generate() -> Self {
        Self {
            api_key: Uuid::new_v4().to_string(),
        }
    }

    pub(super) async fn store(&self, pool: &SqlitePool) {
        let password_hash = sha3::Sha3_256::digest(self.api_key.as_bytes());
        let password_hash = hex::encode(password_hash);

        sqlx::query!("INSERT OR IGNORE INTO users(api_key) VALUES ($1)", password_hash)
            .execute(pool)
            .await
            .expect("Failed to insert user into database");
    }
}

pub struct TestApp {
    pub address: String,
    pub pool: SqlitePool,
    pub test_user: TestUser,
}

impl TestApp {
    pub async fn get_healthcheck(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/health", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_auth_middleware(&self, api_key: Option<&str>) -> reqwest::Response {
        let mut client = reqwest::Client::new().get(format!("{}/assets", self.address));

        if let Some(api_key) = api_key {
            client = client.header("X-API-KEY", api_key);
        }

        client.send().await.expect("Failed to execute request")
    }

    pub async fn post_raw_asset(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/assets/raw", self.address))
            .header("X-API-KEY", &self.test_user.api_key)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_raw_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .delete(format!("{}/assets/raw/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn patch_raw_asset(&self, id: i64, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/assets/raw/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_raw_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets/raw/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_cash_asset(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/assets/cash", self.address))
            .header("X-API-KEY", &self.test_user.api_key)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_cash_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .delete(format!("{}/assets/cash/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn patch_cash_asset(&self, id: i64, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/assets/cash/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_cash_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets/cash/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_coin_asset(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/assets/coin", self.address))
            .header("X-API-KEY", &self.test_user.api_key)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_coin_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .delete(format!("{}/assets/coin/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn patch_coin_asset(&self, id: i64, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/assets/coin/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_coin_asset(&self, id: i64) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets/coin/{}", self.address, id))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_price(&self, name: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/prices/{}", self.address, name))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_prices(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/prices", self.address))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_assets(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/assets", self.address))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_coin(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/coins/1", self.address))
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn search_coins(&self, query: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{}/coins/search", self.address))
            .query(&[("q", query)])
            .header("X-API-KEY", &self.test_user.api_key)
            .send()
            .await
            .expect("Failed to execute request")
    }
}
