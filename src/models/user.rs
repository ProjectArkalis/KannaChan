use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    #[serde(rename(deserialize = "tokenType"), alias = "token_type")]
    pub token_type: String,
    #[serde(rename(deserialize = "accessToken"), alias = "access_token")]
    pub access_token: String,
    #[serde(rename(deserialize = "expiresIn"), alias = "expires_in")]
    pub expires_in: i32,
    #[serde(rename(deserialize = "refreshToken"), alias = "refresh_token")]
    pub refresh_token: String
}