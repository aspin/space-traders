use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::types::{FactionReference, MarketGoodSymbol, SectorSymbol, ShipSymbol, SystemSymbol, WaypointSymbol, WaypointTraitSymbol};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub symbol: SystemSymbol,
    pub sector_symbol: SectorSymbol,
    #[serde(rename = "type")]
    pub system_type: SystemType,
    pub x: i64,
    pub y: i64,
    pub waypoints: Vec<WaypointReference>,
    pub factions: Vec<FactionReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointReference {
    pub symbol: WaypointSymbol,
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    #[serde(flatten)]
    pub reference: WaypointReference,
    pub system_symbol: SystemSymbol,
    pub orbitals: Vec<Orbital>,
    pub faction: Option<FactionReference>,
    pub traits: Vec<WaypointTrait>,
    pub chart: Option<Chart>,
}

impl Waypoint {
    pub fn is_market(&self) -> bool {
        self.traits.iter().find(|t| t.symbol == "MARKETPLACE".to_string()).is_some()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orbital {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaypointTrait {
    pub symbol: WaypointTraitSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub waypoint_symbol: Option<WaypointSymbol>,
    pub submitted_by: String,
    pub submitted_on: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: WaypointSymbol,
    pub imports: Vec<MarketGood>,
    pub exports: Vec<MarketGood>,
    pub exchange: Vec<MarketGood>,
    pub transactions: Vec<Transaction>,
    pub trade_goods: Vec<MarketTradeGood>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketGood {
    pub symbol: MarketGoodSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub waypoint_symbol: WaypointSymbol,
    pub ship_symbol: ShipSymbol,
    pub trade_symbol: MarketGoodSymbol,
    #[serde(rename = "type")]
    pub trade_type: TradeType,
    pub units: u64,
    pub price_per_unit: u64,
    pub total_price: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTradeGood {
    pub symbol: MarketGoodSymbol,
    pub trade_volume: u64,
    pub supply: SupplyType,
    pub purchase_price: u64,
    pub sell_price: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemType {
    NeutronStar,
    RedStar,
    OrangeStar,
    BlueStar,
    YoungStar,
    WhiteDwarf,
    BlackHole,
    Hypergiant,
    Nebula,
    Unstable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Nebula,
    DebrisField,
    GravityWell,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeType {
    Purchase,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupplyType {
    Scarce,
    Limited,
    Moderate,
    Abundant,
}
