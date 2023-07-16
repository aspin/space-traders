use space_traders_api::SpaceTradersApi;
use space_traders_api::types::FactionSymbol;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = match std::env::var("AUTH_TOKEN") {
        Ok(auth_token) => {
            println!("auth token found! instantiating API");
            SpaceTradersApi::new(&auth_token)
        },
        Err(_) => {
            println!("auth token not found! registering new");
            SpaceTradersApi::register("bear", FactionSymbol::Cosmic).await?
        }
    };

    println!("{:?}", api.agent_data().await?);
    return Ok(());
}