use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MessagingProviderType {
    #[serde(rename = "email")]
    #[default]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "push")]
    Push,
}

impl MessagingProviderType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MessagingProviderType::Email => "email",
            MessagingProviderType::Sms => "sms",
            MessagingProviderType::Push => "push",
        }
    }
}

impl std::fmt::Display for MessagingProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
