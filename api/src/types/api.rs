use std::error as stderror;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::types::{Agent, FactionSymbol};
use crate::error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<'a> {
    #[serde(borrow)]
    ApiSuccess(ApiSuccess<'a>),
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

impl stderror::Error for ApiError {}


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSuccess<'a> {
    #[serde(borrow)]
    pub data: &'a serde_json::value::RawValue,
    pub meta: Option<ApiMeta>,
}

impl <'a, T> TryInto<ApiMessage<T>> for ApiSuccess<'a> {
    type Error = error::Error;

    fn try_into(self) -> Result<ApiMessage<T>, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct ApiMessage<T> {
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

