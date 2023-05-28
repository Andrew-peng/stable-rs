use reqwest::header;
use errors::StabilityError;

mod auth;
mod client;
mod errors;
mod options;

use auth::Auth;
use client::Client;

mod models;
pub use models::*;

mod api;
pub use api::*;

pub use options::*;
pub struct Stability {
    auth: Auth,
    client: Client,
}

impl Stability {
    pub fn new(user_agent: &str, api_key: &str) -> Stability {
        Stability {
            auth: Auth::new(user_agent, api_key),
            client: Client::new(),
        }
    }

    pub fn set_organization(&mut self, organization_id: &str) {
        self.auth.organization_id = Some(organization_id.to_owned());
    }

    pub fn set_client_id(&mut self, client_id: &str) {
        self.auth.client_id = Some(client_id.to_owned());
    }

    pub fn init_client(&mut self) -> Result<(), StabilityError>{
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/json").unwrap(),);
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", self.auth.api_key)).unwrap()); 
        headers.insert(header::USER_AGENT, header::HeaderValue::from_str(&self.auth.user_agent[..]).unwrap()); 
        
        if let Some(organization_id) = &self.auth.organization_id {
            let header_value = header::HeaderValue::from_str(&organization_id.as_ref())?;
            headers.insert("Organization", header_value);
        }

        if let Some(client_id) = &self.auth.client_id {
            let header_value = header::HeaderValue::from_str(client_id.as_ref())?;
            headers.insert("Stability-Client-ID", header_value);
        }

        if let Some(client_version) = &self.auth.client_version {
            let header_value = header::HeaderValue::from_str(client_version.as_ref())?;
            headers.insert("Stability-Client-Version", header_value);
        }

        self.client =  Client::builder().default_headers(headers).build().unwrap();
        Ok(())
    }

    // v1/user
    pub fn user(&self) -> Result<UserApi, errors::StabilityError> {
        Ok(UserApi::new(&self.client))
    }

    // v1/engines
    pub fn engines(&self) -> Result<EngineApi, errors::StabilityError> {
        Ok(EngineApi::new(&self.client))
    }

    // v1/generation
    pub fn generations(&self) -> Result<GenerationApi, errors::StabilityError> {
        Ok(GenerationApi::new(&self.client))
    }
}
