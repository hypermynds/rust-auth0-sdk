use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;

use super::Identity;

/// Represents a user as returned from various APIs.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    /// ID of the user which can be used when interacting with other APIs.
    pub user_id: String,
    /// Email address of this user.
    pub email: Option<String>,
    /// Whether this email address is verified.
    pub email_verified: Option<bool>,
    /// Username of this user.
    pub username: Option<String>,
    /// Phone number for this user when using SMS connections.
    pub phone_number: Option<String>,
    /// Whether this phone number has been verified
    #[serde(default)]
    pub phone_verified: Option<bool>,
    /// Date and time when this user was created.
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
    /// Date and time when this user was last updated/modified.
    #[serde(with = "time::serde::iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
    /// Array of user identity objects when accounts are linked.
    #[serde(default)]
    pub identities: Vec<Identity>,
    /// User metadata to which this user has read-only access.
    pub app_metadata: Option<JsonValue>,
    /// User metadata to which this user has read/write access.
    pub user_metadata: Option<JsonValue>,
    /// URL to picture, photo, or avatar of this user.
    pub picture: Option<String>,
    /// Name of this user.
    pub name: Option<String>,
    /// Preferred nickname or alias of this user.
    pub nickname: Option<String>,
    /// List of multi-factor authentication providers with which this user has enrolled.
    #[serde(default)]
    pub multifactor: Vec<String>,
    /// Last IP address from which this user logged in.
    pub last_ip: Option<String>,
    /// Last date and time this user logged in.
    #[serde(with = "time::serde::iso8601::option")]
    pub last_login: Option<OffsetDateTime>,
    /// Total number of logins this user has performed.
    pub logins_count: usize,
    /// Whether this user was blocked by an administrator.
    pub blocked: Option<bool>,
    /// Given name/first name/forename of this user.
    pub given_name: Option<String>,
    /// Family name/last name/surname of this user.
    pub family_name: Option<String>,
}
