use crate::client::Client;
use crate::errors::StabilityError;

use crate::models::Engine;

pub struct EngineApi {
    client: Client,
}

impl EngineApi {

    pub fn new(client: &Client) -> EngineApi {
        EngineApi {
            client: client.to_owned(),
        }
    }

    #[maybe_async::maybe_async]
    pub async fn engines(&self) -> Result<Vec<Engine>, StabilityError> {
        Ok(self
            .client
            .get("https://api.stability.ai/v1/engines/list")
            .send()
            .await?
            .json::<Vec<Engine>>()
            .await?)
    }
}