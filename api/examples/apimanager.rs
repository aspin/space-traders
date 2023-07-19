use space_traders_api::ApiManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = ApiManager::load_from_env("bear", "COSMIC".into()).await?;

    println!("{:?}", manager.find_market_waypoints(1).await?);
    Ok(())
}