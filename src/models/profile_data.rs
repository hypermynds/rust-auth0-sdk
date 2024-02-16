use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Additional profile information for linked identities.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileData {
    /// Email address of this user.
    #[serde(default)]
    pub email: Option<String>,
    /// Whether this email address is verified.
    #[serde(default)]
    pub email_verified: Option<bool>,
    /// Name of this user.
    #[serde(default)]
    pub name: Option<String>,
    /// Username of this user.
    #[serde(default)]
    pub username: Option<String>,
    /// Given name/first name/forename of this user.
    #[serde(default)]
    pub given_name: Option<String>,
    /// Phone number for this user.
    #[serde(default)]
    pub phone_number: Option<String>,
    /// Whether this phone number is verified.
    #[serde(default)]
    pub phone_verified: Option<bool>,
    /// Family name/last name/surname of this user.
    #[serde(default)]
    pub family_name: Option<String>,
    /// Other values.
    #[serde(flatten, default)]
    pub values: HashMap<String, JsonValue>,
}
