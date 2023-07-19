use crate::{error, SpaceTradersApi, types};

impl SpaceTradersApi {
    pub async fn list_systems(&self, limit: usize) -> error::Result<Vec<types::System>> {
        // there any many systems, and the rate limit tends to be exceeded
        return self.get_limit("systems", limit).await;
    }
}