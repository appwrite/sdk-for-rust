use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PasswordHash {
    #[serde(rename = "sha1")]
    #[default]
    Sha1,
    #[serde(rename = "sha224")]
    Sha224,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha384")]
    Sha384,
    #[serde(rename = "sha512/224")]
    Sha512224,
    #[serde(rename = "sha512/256")]
    Sha512256,
    #[serde(rename = "sha512")]
    Sha512,
    #[serde(rename = "sha3-224")]
    Sha3224,
    #[serde(rename = "sha3-256")]
    Sha3256,
    #[serde(rename = "sha3-384")]
    Sha3384,
    #[serde(rename = "sha3-512")]
    Sha3512,
}

impl PasswordHash {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            PasswordHash::Sha1 => "sha1",
            PasswordHash::Sha224 => "sha224",
            PasswordHash::Sha256 => "sha256",
            PasswordHash::Sha384 => "sha384",
            PasswordHash::Sha512224 => "sha512/224",
            PasswordHash::Sha512256 => "sha512/256",
            PasswordHash::Sha512 => "sha512",
            PasswordHash::Sha3224 => "sha3-224",
            PasswordHash::Sha3256 => "sha3-256",
            PasswordHash::Sha3384 => "sha3-384",
            PasswordHash::Sha3512 => "sha3-512",
        }
    }
}

impl std::fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
