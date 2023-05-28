use crate::client::Client;
use crate::errors::StabilityError;

use crate::models::{Account, Balance};

pub struct UserApi {
    client: Client,
}

impl UserApi {

    pub fn new(client: &Client) -> UserApi {
        UserApi {
            client: client.to_owned(),
        }
    }

    #[maybe_async::maybe_async]
    pub async fn account(&self) -> Result<Account, StabilityError> {
        Ok(self
            .client
            .get("https://api.stability.ai/v1/user/account")
            .send()
            .await?
            .json::<Account>()
            .await?)
    }

    #[maybe_async::maybe_async]
    pub async fn balance(&self) -> Result<Balance, StabilityError> {
        Ok(self
            .client
            .get("https://api.stability.ai/v1/user/balance")
            .send()
            .await?
            .json::<Balance>()
            .await?)
    }
}