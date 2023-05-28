use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TextPrompt {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
}
