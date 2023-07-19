use crate::{error, SpaceTradersApi, types};

impl SpaceTradersApi {
    pub async fn list_systems(&self, limit: Option<usize>) -> error::Result<Vec<types::System>> {
        // there any many systems, and the rate limit tends to be exceeded
        self.get_limit("systems", limit).await
    }

    pub async fn get_system(&self, system_symbol: types::SystemSymbol) -> error::Result<types::System> {
        self.get_one(format!("systems/{}", system_symbol).as_str()).await
    }

    pub async fn list_system_waypoints(&self, system_symbol: types::SystemSymbol, limit: Option<usize>) -> error::Result<Vec<types::Waypoint>> {
        self.get_limit(format!("systems/{}/waypoints", system_symbol).as_str(), limit).await
    }

    // TODO: should only really require waypoint symbol
    pub async fn get_waypoint(&self, system_symbol: types::SystemSymbol, waypoint_symbol: types::WaypointSymbol) -> error::Result<types::Waypoint> {
        self.get_one(format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol).as_str()).await
    }
}