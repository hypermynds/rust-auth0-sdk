use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;

use super::Identity;

/// Represents a user as returned from various APIs.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    /// ID of the user which can be used when interacting with other APIs.
    pub user_id: Option<String>,
    /// Email address of this user.
    #[serde(default)]
    pub email: Option<String>,
    /// Whether this email address is verified.
    #[serde(default)]
    pub email_verified: Option<bool>,
    /// Username of this user.
    #[serde(default)]
    pub username: Option<String>,
    /// Phone number for this user when using SMS connections.
    #[serde(default)]
    pub phone_number: Option<String>,
    /// Whether this phone number has been verified
    #[serde(default)]
    pub phone_verified: Option<bool>,
    /// Date and time when this user was created.
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub created_at: Option<OffsetDateTime>,
    /// Date and time when this user was last updated/modified.
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub updated_at: Option<OffsetDateTime>,
    /// Array of user identity objects when accounts are linked.
    #[serde(default)]
    pub identities: Vec<Identity>,
    /// User metadata to which this user has read-only access.
    #[serde(default)]
    pub app_metadata: Option<JsonValue>,
    /// User metadata to which this user has read/write access.
    #[serde(default)]
    pub user_metadata: Option<JsonValue>,
    /// URL to picture, photo, or avatar of this user.
    #[serde(default)]
    pub picture: Option<String>,
    /// Name of this user.
    #[serde(default)]
    pub name: Option<String>,
    /// Preferred nickname or alias of this user.
    #[serde(default)]
    pub nickname: Option<String>,
    /// List of multi-factor authentication providers with which this user has enrolled.
    #[serde(default)]
    pub multifactor: Vec<String>,
    /// Last IP address from which this user logged in.
    #[serde(default)]
    pub last_ip: Option<String>,
    /// Last date and time this user logged in.
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub last_login: Option<OffsetDateTime>,
    /// Lasta date and time this user reset their password.
    #[serde(with = "time::serde::rfc3339::option", default)]
    pub last_password_reset: Option<OffsetDateTime>,
    /// Total number of logins this user has performed.
    pub logins_count: Option<usize>,
    /// Whether this user was blocked by an administrator.
    #[serde(default)]
    pub blocked: Option<bool>,
    /// Given name/first name/forename of this user.
    #[serde(default)]
    pub given_name: Option<String>,
    /// Family name/last name/surname of this user.
    #[serde(default)]
    pub family_name: Option<String>,
}
