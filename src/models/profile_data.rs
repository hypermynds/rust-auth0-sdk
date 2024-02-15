use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Additional profile information for linked identities.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileData {
    /// Email address of this user.
    pub email: Option<String>,
    /// Whether this email address is verified.
    pub email_verified: Option<bool>,
    /// Name of this user.
    pub name: Option<String>,
    /// Username of this user.
    pub username: Option<String>,
    /// Given name/first name/forename of this user.
    pub given_name: Option<String>,
    /// Phone number for this user.
    pub phone_number: Option<String>,
    /// Whether this phone number is verified.
    pub phone_verified: Option<bool>,
    /// Family name/last name/surname of this user.
    pub family_name: Option<String>,
    /// Other values.
    #[serde(flatten)]
    pub values: HashMap<String, JsonValue>,
}
