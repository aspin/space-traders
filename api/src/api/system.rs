use crate::{error, SpaceTradersApi, types};

impl SpaceTradersApi {
    pub async fn list_systems(&self, page: Option<u32>, limit: Option<usize>) -> error::Result<Vec<types::System>> {
        // there any many systems, and the rate limit tends to be exceeded
        self.get_limit("systems", page, limit).await
    }

    pub async fn get_system(&self, system_symbol: types::SystemSymbol) -> error::Result<types::System> {
        self.get_one(format!("systems/{}", system_symbol).as_str()).await
    }

    pub async fn list_system_waypoints(&self, system_symbol: types::SystemSymbol, page: Option<u32>, limit: Option<usize>) -> error::Result<Vec<types::Waypoint>> {
        self.get_limit(format!("systems/{}/waypoints", system_symbol).as_str(), page, limit).await
    }

    pub async fn get_waypoint(&self, waypoint_symbol: types::WaypointSymbol) -> error::Result<types::Waypoint> {
        self.get_one(format!("systems/{}/waypoints/{}", waypoint_symbol.system(), waypoint_symbol).as_str()).await
    }

    pub async fn get_market(&self, waypoint_symbol: types::WaypointSymbol) -> error::Result<types::Market> {
        self.get_one(format!("systems/{}/waypoints/{}/market", waypoint_symbol.system(), waypoint_symbol).as_str()).await
    }

    pub async fn get_shipyard(&self, waypoint_symbol: types::WaypointSymbol) -> error::Result<types::Shipyard> {
        self.get_one(format!("systems/{}/waypoints/{}/shipyard", waypoint_symbol.system(), waypoint_symbol).as_str()).await
    }

    pub async fn get_jump_gate(&self, waypoint_symbol: types::WaypointSymbol) -> error::Result<types::JumpGate> {
        self.get_one(format!("systems/{}/waypoints/{}/jump-gate", waypoint_symbol.system(), waypoint_symbol).as_str()).await
    }
}