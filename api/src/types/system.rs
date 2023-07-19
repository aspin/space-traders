use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::types::{FactionReference, ShipSymbol, Trait};

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
    pub traits: Vec<Trait>,
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

pub type SectorSymbol = String;
pub type SystemSymbol = String;
pub type WaypointSymbol = Coordinates;
pub type MarketGoodSymbol = String;

#[derive(Debug)]
pub enum SystemError {
    InvalidCoordinates
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::InvalidCoordinates => write!(f, "{}", "coordinates are improperly formatted")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Coordinates {
    _sector: String,
    _system: String,
    _waypoint: String,
}

impl TryFrom<String> for Coordinates {
    type Error = SystemError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Coordinates::new(value.as_str())
    }
}

impl Into<String> for Coordinates {
    fn into(self) -> String {
        self._waypoint
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.waypoint())
    }
}

impl Coordinates {
    pub fn new(s: &str) -> Result<Self, SystemError> {
        let parts: Vec<&str> = s.split("-").collect();
        match parts[..] {
            [sector, system, waypoint] => Ok(Coordinates {
                _sector: sector.to_string(),
                _system: format!("{}-{}", sector, system),
                _waypoint: format!("{}-{}-{}", sector, system, waypoint),
            }),
            _ => Err(SystemError::InvalidCoordinates)
        }
    }

    pub fn sector(&self) -> &str {
        self._sector.as_str()
    }

    pub fn system(&self) -> &str {
        self._system.as_str()
    }

    pub fn waypoint(&self) -> &str {
        self._waypoint.as_str()
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;
    use crate::types::system::Coordinates;

    #[derive(Deserialize)]
    struct Object {
        pub coordinates: Coordinates,
    }

    #[test]
    fn test_serialize() {
        let expected = String::from("X1-DF55-20250Z");
        let c = Coordinates::new(expected.as_str()).unwrap();
        match serde_json::to_string(&c) {
            Ok(s) => {
                assert_eq!(format!("\"{}\"", expected), s);
            }
            Err(e) => assert!(false, "could not serialize: {:?}", e)
        }
    }

    #[test]
    fn test_deserialize() {
        let serialized = json!({
            "coordinates": "X1-DF55-20250Z"
        }).to_string();

        match serde_json::from_str::<Object>(serialized.as_str()) {
            Ok(o) => {
                assert_eq!(o.coordinates.sector(), "X1");
                assert_eq!(o.coordinates.system(), "X1-DF55");
                assert_eq!(o.coordinates.waypoint(), "X1-DF55-20250Z");
            }
            Err(e) => assert!(false, "could not deserialize: {:?}", e)
        }
    }
}