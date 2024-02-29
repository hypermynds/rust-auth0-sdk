use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::{EncryptionKey, JwtConfiguration, OidcLogoutConfig, SigningKey};

/// Represents a client as returned from various APIs.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Client {
    /// ID of this client.
    pub client_id: Option<String>,
    /// Name of the tenant this client belongs to.
    #[serde(default)]
    pub tenant: Option<String>,
    /// Name of this client.
    #[serde(default)]
    pub name: Option<String>,
    /// Free text description of this client.
    #[serde(default)]
    pub description: Option<String>,
    /// Whether this is your global 'All Applications' client representing legacy tenant settings
    /// (true) or a regular client (false).
    #[serde(default)]
    pub global: Option<bool>,
    /// Client secret.
    pub client_secret: Option<String>,
    /// Type of client used to determine which settings are applicable.
    pub app_type: Option<String>,
    /// URL of the logo to display for this client.
    #[serde(default)]
    pub logo_uri: Option<String>,
    /// Whether this client a first party client.
    #[serde(default)]
    pub is_first_party: Option<bool>,
    /// Whether this client conforms to strict OIDC specifications.
    #[serde(default)]
    pub oidc_conformant: Option<bool>,
    /// List of URLs whitelisted for Auth0 to use as a callback to the client after authentication.
    #[serde(default)]
    pub callbacks: Vec<String>,
    /// List of URLs allowed to make requests from JavaScript to Auth0 API (typically used with
    /// CORS).
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    /// List of allowed origins for use with Cross-Origin Authentication, Device Flow, and web
    /// message response mode.
    #[serde(default)]
    pub web_origins: Vec<String>,
    /// List of audiences/realms for SAML protocol.
    #[serde(default)]
    pub client_aliases: Vec<String>,
    /// List of allow clients and API ids that are allowed to make delegation requests. Empty means
    /// all all your clients are allowed.
    #[serde(default)]
    pub allowed_clients: Vec<String>,
    /// List of URLs that are valid to redirect to after logout from Auth0. Wildcards are allowed
    /// for subdomains.
    #[serde(default)]
    pub allowed_logout_urls: Vec<String>,
    /// Configuration for OIDC backchannel logout.
    #[serde(default)]
    pub oidc_logout: Option<OidcLogoutConfig>,
    /// List of grant types supported for this application.
    #[serde(default)]
    pub grant_types: Vec<String>,
    /// Configuration related to JWTs for the client.
    #[serde(default)]
    pub jwt_configuration: Option<JwtConfiguration>,
    /// Signing certificates associated with this client.
    #[serde(default)]
    pub signing_keys: Vec<SigningKey>,
    /// Encryption used for WsFed responses with this client.
    #[serde(default)]
    pub encryption_key: Option<EncryptionKey>,
    /// Applies only to SSO clients and determines whether Auth0 will handle Single Sign On (true)
    /// or whether the Identity Provider will (false).
    #[serde(default)]
    pub sso: Option<bool>,
    /// Whether Single Sign On is disabled.
    pub sso_disabled: Option<bool>,
    /// Whether this client can be used to make cross-origin authentication requests.
    #[serde(default)]
    pub cross_origin_authentication: Option<bool>,
    /// URL of the location in your site where the cross origin verification takes place for the
    /// cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted
    /// login page.
    #[serde(default)]
    pub cross_origin_loc: Option<String>,
    /// Whether a custom login page is to be used (true) or the default provided login page
    /// (false).
    #[serde(default)]
    pub custom_login_page_on: Option<bool>,
    /// The content (HTML, CSS, JS) of the custom login page.
    #[serde(default)]
    pub custom_login_page: Option<String>,
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    #[serde(default)]
    pub custom_login_page_preview: Option<String>,
    /// HTML form template to be used for WS-Federation.
    #[serde(default)]
    pub form_template: Option<String>,
    /// Addons enabled for this client and their associated configurations.
    #[serde(default)]
    pub addons: Option<JsonValue>,
    /// Defines the requested authentication method for the token endpoint.
    pub token_endpoint_auth_method: Option<String>,
    /// Metadata associated with the client.
    #[serde(default)]
    pub client_metadata: Option<JsonValue>,
    /// Additional configuration for native mobile apps.
    #[serde(default)]
    pub mobile: Option<JsonValue>,
    /// Initiate login uri, must be https.
    #[serde(default)]
    pub initiate_login_uri: Option<String>,
    /// Configure native social settings.
    #[serde(default)]
    pub native_social_login: Option<JsonValue>,
    /// Refresh token configuration.
    #[serde(default)]
    pub refresh_token: Option<JsonValue>,
    /// Defines how to proceed during an authentication transaction with regards an organization.
    #[serde(default)]
    pub organization_usage: Option<String>,
    /// Defines how to proceed during an authentication transaction when `organization_usage` is
    /// `require`.
    #[serde(default)]
    pub organization_require_behavior: Option<String>,
    /// Defines client authentication methods.
    #[serde(default)]
    pub client_authentication_methods: Option<JsonValue>,
    /// Makes the use of Pushed Authorization Requests mandatory for this client.
    #[serde(default)]
    pub require_pushed_authorization_requests: Option<bool>,
    /// Custom configuration for Access Tokens.
    #[serde(default)]
    pub access_token: Option<JsonValue>,
    /// JWT-secured Authorization Requests (JAR) settings.
    #[serde(default)]
    pub signed_request_object: Option<JsonValue>,
    /// Defines the compliance level for this client, which may restrict it's capabilities.
    #[serde(default)]
    pub compliance_level: Option<String>,
}
