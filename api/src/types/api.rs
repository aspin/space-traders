use serde::{Serialize, Deserialize};
use crate::types::{Agent, FactionSymbol};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<ApiMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMeta {
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationRequest {
    pub symbol: String,
    pub faction: FactionSymbol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationData {
    pub token: String,
    pub agent: Agent,
}

