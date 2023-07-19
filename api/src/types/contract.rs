use serde::{Serialize, Deserialize};
use crate::types::FactionSymbol;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Procurement
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    pub faction_symbol: FactionSymbol,

    #[serde(rename = "type")]
    pub contract_type: ContractType,
}
