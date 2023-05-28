use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub base64: String,
    #[serde(rename = "finishReason")] 
    pub finish_reason: String,
    pub seed: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageList {
    pub artifacts: Vec<Image>,
}

