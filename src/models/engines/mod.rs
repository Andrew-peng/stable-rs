use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Engine {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")] 
    pub engine_type: String,
}
