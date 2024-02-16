use serde::{Deserialize, Serialize};

/// SigningKey used for signing tokens.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SigningKey {
    /// Signing certificate public key and chain in PKCS#7 (.P7B) format.
    pub pkcs7: String,
    /// Signing certificate public key in X.590 (.CER) format.
    pub cert: String,
    /// Subject name for this certificate in the format `/CN={domain}`.
    #[serde(default)]
    pub subject: Option<String>,
}
