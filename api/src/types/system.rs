use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::types::{AgentSymbol, FactionReference, FactionSymbol, MarketGoodSymbol, SectorSymbol, ShipEngine, ShipFrame, ShipModule, ShipMount, ShipReactor, ShipSymbol, SystemSymbol, WaypointSymbol, WaypointTraitSymbol};

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

    pub fn is_shipyard(&self) -> bool {
        self.traits.iter().find(|t| t.symbol == "SHIPYARD".to_string()).is_some()
    }

    pub fn is_jump_gate(&self) -> bool {
        self.reference.waypoint_type == WaypointType::JumpGate
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
    pub transactions: Vec<MarketTransaction>,
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
pub struct MarketTransaction {
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
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    pub symbol: WaypointSymbol,
    pub ship_types: Vec<ShipTypeReference>,
    pub transactions: Vec<ShipTransaction>,
    pub ships: Vec<Ship>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipTypeReference {
    #[serde(rename = "type")]
    pub ship_type: ShipType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipTransaction {
    pub waypoint_symbol: WaypointSymbol,
    pub ship_symbol: ShipSymbol,
    pub price: u64,
    pub agent_symbol: AgentSymbol,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    #[serde(rename = "type")]
    pub ship_type: ShipType,
    pub name: String,
    pub description: String,
    pub purchase_price: u64,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JumpGate {
    pub jump_range: i64,
    pub faction_symbol: FactionSymbol,
    pub connected_systems: Vec<ConnectedSystem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedSystem {
    pub symbol: SystemSymbol,
    pub sector_symbol: SectorSymbol,
    #[serde(rename = "type")]
    pub system_type: SystemType,
    pub faction_symbol: FactionSymbol,
    pub x: i64,
    pub y: i64,
    pub distance: i64,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    ShipProbe,
    ShipMiningDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
}
