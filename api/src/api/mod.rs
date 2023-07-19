mod system;

use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::types;
use crate::error::{DecodeError, Result, Error, ApiErrorResponse};

const BASE_URL: &str = "https://api.spacetraders.io/v2";
const MAX_PAGE_LIMIT: u32 = 20;

#[derive(Debug)]
pub struct SpaceTradersApi {
    client: reqwest::Client,
    token: String,
    factions: HashMap<types::FactionSymbol, types::Faction>,
    user_agent: Option<types::Agent>,
}

impl SpaceTradersApi {
    pub fn new(auth_token: &str) -> Self {
        SpaceTradersApi {
            client: reqwest::Client::new(),
            token: String::from(auth_token),
            factions: HashMap::new(),
            user_agent: None,
        }
    }

    fn authorization(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub fn faction_symbols(&self) -> Vec<types::FactionSymbol> {
        self.factions.keys().cloned().collect()
    }

    async fn get<R: DeserializeOwned>(&self, path: &str) -> Result<types::ApiResponse<R>> {
        self.handle_response(
            self.client.get(format!("{}/{}", BASE_URL, path))
                .header(reqwest::header::AUTHORIZATION, &self.authorization())
                .send()
                .await?
        ).await
    }

    async fn get_one<R: DeserializeOwned>(&self, path: &str) -> Result<R> {
        self.get::<R>(path).await.map(|result| result.data)
    }

    async fn get_limit<R: DeserializeOwned>(&self, path: &str, limit: Option<usize>) -> Result<Vec<R>> {
        let limit = match limit {
            Some(l) => l,
            None => usize::MAX
        };

        let response = self.get::<Vec<R>>(
            paginate_path(path, 1, MAX_PAGE_LIMIT).as_str()
        ).await?;

        let mut results = response.data;
        if let Some(meta) = response.meta {
            let mut page: u32 = 2;
            let pages: u32 = meta.total / meta.limit + 1;
            while page < pages && results.len() < limit {
                let next = self.get_one::<Vec<R>>(
                    paginate_path(path, page.into(), MAX_PAGE_LIMIT).as_str()
                ).await?;

                results.extend(next);
                page += 1;
            }
        }
        results.truncate(limit);
        Ok(results)
    }

    async fn get_all<R: DeserializeOwned>(&self, path: &str) -> Result<Vec<R>> {
        self.get_limit(path, None).await
    }

    async fn post<T: Serialize + ?Sized, R: DeserializeOwned>(&self, path: &str, request: &T) -> Result<R> {
        self.handle_response(
            self.client.post(format!("{}/{}", BASE_URL, path))
                .header(reqwest::header::AUTHORIZATION, &self.authorization())
                .json(&request)
                .send()
                .await?
        ).await.map(|response| response.data)
    }

    async fn handle_response<R: DeserializeOwned>(&self, response: reqwest::Response) -> Result<types::ApiResponse<R>> {
        let response_text = response.text().await.map_err(Error::from)?;

        match serde_json::from_str::<types::ApiResponse<R>>(&response_text) {
            Ok(v) => Ok(v),
            Err(e) => {
                match serde_json::from_str::<ApiErrorResponse>(&response_text) {
                    Ok(v) => Err(v.error.into()),
                    Err(_) => Err(Error::DecodeError(DecodeError { message: response_text, error: e }.into()))
                }
            }
        }
    }

    pub async fn register(call_sign: &str, faction: types::FactionSymbol) -> Result<SpaceTradersApi> {
        let mut api = SpaceTradersApi::new("");
        let registration_data: types::RegistrationData = api.post("register", &types::RegistrationRequest {
            symbol: String::from(call_sign),
            faction,
        }).await?;

        api.token = registration_data.token;
        Ok(api)
    }

    pub async fn hydrate(&mut self) -> Result<()> {
        self.user_agent = Some(self.fetch_agent().await?);

        for faction in self.fetch_factions().await?.into_iter() {
            self.factions.insert(faction.symbol.clone(), faction);
        }
        Ok(())
    }

    pub async fn fetch_agent(&self) -> Result<types::Agent> {
        return self.get_one("my/agent").await;
    }

    pub async fn fetch_factions(&self) -> Result<Vec<types::Faction>> {
        return self.get_all("factions").await;
    }

    pub async fn fetch_contracts(&self) -> Result<Vec<types::Contract>> {
        return self.get_all("my/contracts").await;
    }
}

fn paginate_path(path: &str, page: u32, limit: u32) -> String {
    let mut limit = limit;
    if limit > MAX_PAGE_LIMIT {
        limit = MAX_PAGE_LIMIT;
    }

    format!("{}?page={}&limit={}", path, page, limit)
}

