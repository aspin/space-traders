use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FactionSymbol {
    Cosmic
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Procurement
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationRequest {
    pub symbol: String,
    pub faction: FactionSymbol,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationResult {
    pub data: AgentData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentData {
    pub token: String,
    pub agent: Agent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    pub faction_symbol: FactionSymbol,

    #[serde(rename = "type")]
    pub contract_type: ContractType,
}