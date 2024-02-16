use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Configuration related to JWTs for the client.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfiguration {
    /// Number of seconds the JWT will be valid for.
    #[serde(default)]
    pub lifetime_in_seconds: Option<usize>,
    /// Whether the client secret is base64 encoded.
    #[serde(default)]
    pub secret_encoded: Option<bool>,
    /// Configuration related to id token claims for the client.
    pub scopes: JsonValue,
    /// Algorithm used to sign JWTs. Can be HS256 or RS256. PS256 available via addon.
    pub alg: String,
}
