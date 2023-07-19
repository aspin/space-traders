use space_traders_api::SpaceTradersApi;
use space_traders_api::types::FactionSymbol;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = match std::env::var("AUTH_TOKEN") {
        Ok(auth_token) => {
            println!("auth token found! instantiating API");
            SpaceTradersApi::new(&auth_token)
        }
        Err(_) => {
            println!("auth token not found! registering new");
            SpaceTradersApi::register("bear", FactionSymbol::from("COSMIC")).await?
        }
    };

    // api.hydrate().await?;
    println!("factions available: {:?}", api.faction_symbols());
    // println!("{:?}", api.list_systems(Some(20)).await?);
    // println!("{:?}", api.get_system("X1-JF24".to_string()).await?);
    println!("{:?}", api.list_system_waypoints("X1-JF24".to_string(), None).await?);
    return Ok(());
}