use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub authorization_code: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
