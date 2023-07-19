use std::collections::HashMap;
use std::time::Duration;
use crate::{error, SpaceTradersApi, types};

#[derive(Debug)]
pub struct ApiManager {
    pub api: SpaceTradersApi,
    factions: HashMap<types::FactionSymbol, types::Faction>,
    user_agent: Option<types::Agent>,
}

impl ApiManager {
    pub async fn load_from_env(call_sign: &str, faction: types::FactionSymbol) -> error::Result<Self> {
        let api = match std::env::var("AUTH_TOKEN") {
            Ok(auth_token) => {
                SpaceTradersApi::new(&auth_token)
            }
            Err(_) => {
                SpaceTradersApi::register(call_sign, faction).await?
            }
        };

        let mut m = ApiManager {
            api,
            factions: HashMap::new(),
            user_agent: None,
        };

        m.hydrate().await?;
        Ok(m)
    }

    pub fn faction_symbols(&self) -> Vec<types::FactionSymbol> {
        self.factions.keys().cloned().collect()
    }

    async fn hydrate(&mut self) -> error::Result<()> {
        self.user_agent = Some(self.api.get_agent().await?);

        for faction in self.api.list_factions().await?.into_iter() {
            self.factions.insert(faction.symbol.clone(), faction);
        }
        Ok(())
    }

    pub async fn find_market_waypoints(&self, limit: usize) -> error::Result<Vec<types::WaypointSymbol>> {
        let mut market_waypoints = Vec::<types::WaypointSymbol>::new();

        let mut page = 1;
        while market_waypoints.len() < limit {
            for system in self.api.list_systems(Some(page), Some(20)).await? {
                for waypoint in self.api.list_system_waypoints(system.symbol.clone(), None, None).await? {
                    if waypoint.is_market() {
                        market_waypoints.push(waypoint.reference.symbol.clone())
                    }

                    if market_waypoints.len() >= limit {
                        return Ok(market_waypoints);
                    }
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
                println!("checked system {}, {} markets found", system.symbol, market_waypoints.len());
            }
            page += 1;
        }

        Ok(market_waypoints)
    }
}