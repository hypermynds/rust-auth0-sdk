use serde::{Deserialize, Serialize};

/// The access token received as a response of authentication flow.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessToken {
    /// The access token.
    pub access_token: String,
    /// The token type of the access token.
    pub token_type: String,
    /// The duration in secs that the access token is valid.
    pub expires_in: u64,
    /// The refresh token, available with the `offline_access` scope.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// The user's ID Token.
    #[serde(default)]
    pub id_token: Option<String>,
}
