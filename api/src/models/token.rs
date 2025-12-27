use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenClaims {
    pub typ: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum CreateToken {
    #[serde(rename_all = "camelCase")]
    Login { username: String, password: String },
    #[serde(rename_all = "camelCase")]
    Refresh { refresh_token: String },
}
