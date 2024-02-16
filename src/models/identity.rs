use serde::{Deserialize, Serialize};

use super::ProfileData;

/// Describes a 3rd party account for a given user.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Identity {
    /// Name of the connection containing this identity.
    pub connection: String,
    /// Unique identifier of the user user for this identity.
    pub user_id: String,
    /// The type of identity provider
    pub provider: String,
    /// Whether this identity is from a social provider.
    #[serde(rename = "isSocial", default)]
    pub is_social: Option<bool>,
    /// IDP access token returned only if scope read:user_idp_tokens is defined.
    #[serde(default)]
    pub access_token: Option<String>,
    /// IDP access token secret returned only if scope read:user_idp_tokens is defined.
    #[serde(default)]
    pub access_token_secret: Option<String>,
    /// IDP refresh token returned only if scope read:user_idp_tokens is defined.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// Contains additional profile information for linked identities.
    #[serde(rename = "profileData", default)]
    pub profile_data: Option<ProfileData>,
}
