use serde::{Deserialize, Serialize};

/// The access token received as a response of authentication flow.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceCode {
    /// The unique code for the device. When the user visits the `verification_uri` in their
    /// browser-based device, this code will be bound to their session.
    pub device_code: String,
    /// The code that the user should input at the `verification_uri` to authorize the device.
    pub user_code: String,
    /// The URL the user should visit to authorize the device.
    pub verification_uri: String,
    /// The complete URL the user should visit to authorize the device. Your app can use this value
    /// to embed the `user_code` in the URL, if you so choose.
    pub verification_uri_complete: String,
    /// The lifetime (in seconds) of the `device_code` and `user_code`.
    pub expires_in: usize,
    /// The interval (in seconds) at which the app should poll the token URL to request a token.
    pub interval: usize,
}
