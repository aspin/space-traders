use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use reqwest;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: u32,
    pub data: Option<HashMap<String, String>>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl error::Error for ApiError {}

#[derive(Debug)]
pub struct DecodeError {
    pub message: String,
    pub error: serde_json::Error,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not decode {}: {}", self.message, self.error)
    }
}

impl error::Error for DecodeError {}


#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    DecodeError(DecodeError),
    HttpError(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ApiError(e) => write!(f, "{}", e),
            Error::HttpError(e) => write!(f, "{}", e),
            Error::DecodeError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ApiError(e) => Some(e),
            Error::HttpError(e) => Some(e),
            Error::DecodeError(e) => Some(e),
        }
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::ApiError(e)
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Self {
        Error::DecodeError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HttpError(e)
    }
}
