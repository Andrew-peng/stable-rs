use crate::client::Client;
use crate::errors::StabilityError;
use crate::images::ImageList;
use crate::options::GenerationOptions;

use crate::models::{Image, TextPrompt};

use serde_json::json;
use serde_json::Value;

pub struct GenerationApi {
    client: Client,
}

impl GenerationApi {

    pub fn new(client: &Client) -> GenerationApi {
        GenerationApi {
            client: client.to_owned(),
        }
    }

    #[maybe_async::maybe_async]
    pub async fn text_to_image(&self, engine_id: &str, prompts: Vec<TextPrompt>, options: &GenerationOptions) -> Result<ImageList, StabilityError> {
        let options_json = serde_json::to_value(options)?;

        // Construct the request body JSON payload
        let mut request_body = json!({
            "text_prompts": prompts,
        });
    
        // Merge the `options_json` object into the `request_body`
        if let Value::Object(options_obj) = options_json {
            request_body.as_object_mut().unwrap().extend(options_obj);
        }
    
        let res = self
            .client
            .post(format!("https://api.stability.ai/v1/generation/{}/text-to-image", engine_id))
            .json(&request_body)
            .send()
            .await?
            .json::<ImageList>()
            .await?;
        Ok(res)
    }

    #[maybe_async::maybe_async]
    pub async fn str_text_to_image(&self, engine_id: &str, prompt: &str, options: &GenerationOptions) -> Result<Image, StabilityError> {
        let options_json = serde_json::to_value(options)?;

        // Construct the request body JSON payload
        let mut request_body = json!({
            "text_prompts": vec![TextPrompt {
                text: prompt.to_string(),
                weight: None,
            }],
        });
    
        // Merge the `options_json` object into the `request_body`
        if let Value::Object(options_obj) = options_json {
            request_body.as_object_mut().unwrap().extend(options_obj);
        }
        Ok(self
            .client
            .post(format!("https://api.stability.ai/v1/generation/{}/text-to-image", engine_id))
            .json(&request_body)
            .send()
            .await?
            .json::<ImageList>()
            .await?.artifacts[0].clone())
    }
}
