use serde::{Serialize, Deserialize};
use crate::types::system;

pub type FactionSymbol = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: FactionSymbol,
    pub name: String,
    pub description: String,
    pub headquarters: system::Coordinates,
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

