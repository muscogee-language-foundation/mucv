use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct FormData {
    pub team_id: String,
    pub team_domain: String,
    pub enterprise_id: String,
    pub enterprise_name: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
}

#[derive(Serialize)]
pub struct Text {
    pub text: String,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
}

#[derive(Serialize)]
pub struct Response {
    pub response_type: String,
    pub replace_original: bool,
    pub text: Text,
}
