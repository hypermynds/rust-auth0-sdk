use serde::{Deserialize, Serialize};

/// Configuration for OIDC backchannel logout.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OidcLogoutConfig {
    /// List of URLs that are valid to call back from Auth0 for OIDC backchannel logout.
    #[serde(default)]
    pub backchannel_logout_urls: Vec<String>,
}
