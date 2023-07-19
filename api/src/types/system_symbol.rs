use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

pub type MarketGoodSymbol = String;
pub type WaypointTraitSymbol = String;

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

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SectorSymbol {
    _sector: String,
}

impl TryFrom<String> for SectorSymbol {
    type Error = SystemError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SectorSymbol::new(value.as_str())
    }
}

impl Into<String> for SectorSymbol {
    fn into(self) -> String {
        self._sector
    }
}

impl Display for SectorSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.sector())
    }
}

impl Debug for SectorSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.sector())
    }
}

impl SectorSymbol {
    pub fn new(s: &str) -> Result<Self, SystemError> {
        let parts: Vec<&str> = s.split("-").collect();
        match parts[..] {
            [sector] => Ok(SectorSymbol {
                _sector: sector.to_string(),
            }),
            _ => Err(SystemError::InvalidCoordinates)
        }
    }

    pub fn sector(&self) -> &str {
        self._sector.as_str()
    }
}


#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SystemSymbol {
    _sector: String,
    _system: String,
}

impl TryFrom<String> for SystemSymbol {
    type Error = SystemError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SystemSymbol::new(value.as_str())
    }
}

impl Into<String> for SystemSymbol {
    fn into(self) -> String {
        self._system
    }
}

impl Display for SystemSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.system())
    }
}

impl Debug for SystemSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.system())
    }
}

impl SystemSymbol {
    pub fn new(s: &str) -> Result<Self, SystemError> {
        let parts: Vec<&str> = s.split("-").collect();
        match parts[..] {
            [sector, system] => Ok(SystemSymbol {
                _sector: sector.to_string(),
                _system: format!("{}-{}", sector, system),
            }),
            _ => Err(SystemError::InvalidCoordinates)
        }
    }

    pub fn sector(&self) -> &str {
        self._sector.as_str()
    }

    pub fn sector_symbol(&self) -> SectorSymbol {
        SectorSymbol::new(self.sector()).unwrap()
    }

    pub fn system(&self) -> &str {
        self._system.as_str()
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WaypointSymbol {
    _sector: String,
    _system: String,
    _waypoint: String,
}

impl TryFrom<String> for WaypointSymbol {
    type Error = SystemError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        WaypointSymbol::new(value.as_str())
    }
}

impl Into<String> for WaypointSymbol {
    fn into(self) -> String {
        self._waypoint
    }
}

impl Display for WaypointSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.waypoint())
    }
}

impl Debug for WaypointSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.waypoint())
    }
}

impl WaypointSymbol {
    pub fn new(s: &str) -> Result<Self, SystemError> {
        let parts: Vec<&str> = s.split("-").collect();
        match parts[..] {
            [sector, system, waypoint] => Ok(WaypointSymbol {
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

    pub fn sector_symbol(&self) -> SectorSymbol {
        SectorSymbol::new(self.sector()).unwrap()
    }

    pub fn system(&self) -> &str {
        self._system.as_str()
    }

    pub fn system_symbol(&self) -> SystemSymbol {
        SystemSymbol::new(self._system.as_str()).unwrap()
    }

    pub fn waypoint(&self) -> &str {
        self._waypoint.as_str()
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;
    use crate::types::WaypointSymbol;

    #[derive(Deserialize)]
    struct Object {
        pub waypoint: WaypointSymbol,
    }

    #[test]
    fn test_serialize() {
        let expected = String::from("X1-DF55-20250Z");
        let c = WaypointSymbol::new(expected.as_str()).unwrap();
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
            "waypoint": "X1-DF55-20250Z"
        }).to_string();

        match serde_json::from_str::<Object>(serialized.as_str()) {
            Ok(o) => {
                assert_eq!(o.waypoint.sector(), "X1");
                assert_eq!(o.waypoint.system(), "X1-DF55");
                assert_eq!(o.waypoint.waypoint(), "X1-DF55-20250Z");

                let system_symbol = o.waypoint.system_symbol();
                assert_eq!(system_symbol.system(), "X1-DF55");

                let sector_symbol = o.waypoint.sector_symbol();
                assert_eq!(sector_symbol.sector(), "X1");
                assert_eq!(sector_symbol, system_symbol.sector_symbol());
            }
            Err(e) => assert!(false, "could not deserialize: {:?}", e)
        }
    }
}
