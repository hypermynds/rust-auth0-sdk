use serde::{Deserialize, Serialize};

/// The client's encryption key
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptionKey {
    /// Encryption Public RSA Key.
    pub r#pub: String,
    /// Encryption certificate for public key in X.590 (.CER) format.
    pub cert: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    #[serde(default)]
    pub subject: Option<String>,
}
