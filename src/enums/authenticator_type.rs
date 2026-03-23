use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AuthenticatorType {
    #[serde(rename = "totp")]
    #[default]
    Totp,
}

impl AuthenticatorType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AuthenticatorType::Totp => "totp",
        }
    }
}

impl std::fmt::Display for AuthenticatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
