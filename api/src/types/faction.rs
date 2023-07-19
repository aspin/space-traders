use serde::{Deserialize, Serialize};

use crate::types::system_symbol;

pub type FactionSymbol = String;
pub type FactionTraitSymbol = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: FactionSymbol,
    pub name: String,
    pub description: String,
    pub headquarters: system_symbol::WaypointSymbol,
    pub traits: Vec<FactionTrait>,
    pub is_recruiting: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionReference {
    pub symbol: FactionSymbol,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionTrait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

