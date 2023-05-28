use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub role: String,
    pub is_default: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub email: String,
    pub id: String,
    pub organizations: Vec<Organization>,
    pub profile_picture: Option<String>,
}