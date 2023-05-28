#[derive(Debug, Clone, Default)]
pub struct Auth {
    pub user_agent: String,
    pub api_key: String,
    pub organization_id: Option<String>,
    pub client_id: Option<String>,
    pub client_version: Option<String>,
}

impl Auth {
    pub fn new(user_agent: &str, api_key: &str) -> Auth {
        Auth {
            user_agent: user_agent.to_owned(),
            api_key: api_key.to_owned(),
            ..Auth::default()
        }
    }
}
