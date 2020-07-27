use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text: String,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub response_type: String,
    pub text: String,
}
