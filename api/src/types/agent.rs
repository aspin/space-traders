use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: u64,
}
