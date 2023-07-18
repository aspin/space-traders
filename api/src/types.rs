use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<ApiMeta>,
}

// TODO: implement using this pages
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMeta {
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

pub type FactionSymbol = String;
pub type Coordinates = String;

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
pub struct RegistrationData {
    pub token: String,
    pub agent: Agent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: Coordinates,
    pub credits: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: FactionSymbol,
    pub name: String,
    pub description: String,
    pub headquarters: Coordinates,
    pub traits: Vec<Trait>,
    pub is_recruiting: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    pub faction_symbol: FactionSymbol,

    #[serde(rename = "type")]
    pub contract_type: ContractType,
}