use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AuthenticationFactor {
    #[serde(rename = "email")]
    #[default]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "recoverycode")]
    Recoverycode,
}

impl AuthenticationFactor {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AuthenticationFactor::Email => "email",
            AuthenticationFactor::Phone => "phone",
            AuthenticationFactor::Totp => "totp",
            AuthenticationFactor::Recoverycode => "recoverycode",
        }
    }
}

impl std::fmt::Display for AuthenticationFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
