use std::error;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::types::{Agent, FactionSymbol};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    ApiSuccess(ApiSuccess<T>),
    ApiErrored(ApiErrorResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: u32,
    pub data: Option<serde_json::Value>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl error::Error for ApiError {}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSuccess<T> {
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

