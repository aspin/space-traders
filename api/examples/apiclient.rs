use space_traders_api::SpaceTradersApi;
use space_traders_api::types::FactionSymbol;


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
    // println!("{:?}", api.get_waypoint("X1-JF24".to_string(), "X1-JF24-77691C".to_string()).await?);
    println!("{:?}", api.get_market("X1-JF24".to_string(), "X1-JF24-06790Z".to_string()).await?);
    Ok(())
}