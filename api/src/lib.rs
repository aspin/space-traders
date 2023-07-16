use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::types::{AgentData, FactionSymbol, RegistrationRequest, RegistrationResult};
pub use crate::error::Result;

pub mod types;
pub mod error;

pub fn run() {
    println!("space traders api");
}


const BASE_URL: &str = "https://api.spacetraders.io/v2";

#[derive(Debug)]
pub struct SpaceTradersApi {
    client: reqwest::Client,
    token: String,
}

impl SpaceTradersApi {
    pub fn new(auth_token: &str) -> Self {
        SpaceTradersApi {
            client: reqwest::Client::new(),
            token: String::from(auth_token),
        }
    }

    async fn post<T: Serialize + ?Sized, R: DeserializeOwned>(&self, path: &str, request: &T) -> Result<R> {
        self.handle_response(
            self.client.post(format!("{}/{}", BASE_URL, path))
                .json(&request)
                .send()
                .await?
        ).await
    }

    async fn post_empty<R: DeserializeOwned>(&self, path: &str) -> Result<R> {
        self.handle_response(
            self.client.post(format!("{}/{}", BASE_URL, path))
                .send()
                .await?
        ).await
    }

    async fn handle_response<R: DeserializeOwned>(&self, response: reqwest::Response) -> Result<R> {
        response.json::<R>().await.map_err(error::Error::from)
    }

    pub async fn register(call_sign: &str, faction: FactionSymbol) -> Result<SpaceTradersApi> {
        let mut api = SpaceTradersApi::new("");
        let registration_data = api.post::<RegistrationRequest, RegistrationResult>("register", &RegistrationRequest {
            symbol: String::from(call_sign),
            faction,
        }).await?;

        api.token = registration_data.data.token;
        Ok(api)
    }

    pub async fn agent_data(&self) -> Result<AgentData> {
        return self.post_empty::<AgentData>("my/agent").await;
    }
}

