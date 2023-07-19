use space_traders_api::SpaceTradersApi;
use space_traders_api::types::{FactionSymbol, WaypointSymbol};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = match std::env::var("AUTH_TOKEN") {
        Ok(auth_token) => {
            println!("auth token found! instantiating API");
            SpaceTradersApi::new(&auth_token)
        }
        Err(_) => {
            println!("auth token not found! registering new");
            SpaceTradersApi::register("bear", FactionSymbol::from("COSMIC")).await?
        }
    };

    // println!("{:?}", api.list_systems(Some(20)).await?);
    // println!("{:?}", api.get_system("X1-JF24".to_string()).await?);
    // println!("{:?}", api.list_system_waypoints("X1-JF24".to_string(), None).await?);
    // println!("{:?}", api.get_waypoint(WaypointSymbol::new("X1-JF24-77691C").unwrap()).await?);
    // println!("{:?}", api.get_market(WaypointSymbol::new("X1-JF24-06790Z").unwrap()).await?);
    // println!("{:?}", api.get_shipyard(WaypointSymbol::new("X1-JF24-73757X").unwrap()).await?);
    println!("{:?}", api.get_jump_gate(WaypointSymbol::new("X1-JF24-00189Z").unwrap()).await?);
    Ok(())
}