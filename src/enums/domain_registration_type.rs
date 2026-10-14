use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DomainRegistrationType {
    #[serde(rename = "new")]
    #[default]
    New,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "renewal")]
    Renewal,
    #[serde(rename = "trade")]
    Trade,
}

impl DomainRegistrationType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DomainRegistrationType::New => "new",
            DomainRegistrationType::Transfer => "transfer",
            DomainRegistrationType::Renewal => "renewal",
            DomainRegistrationType::Trade => "trade",
        }
    }
}

impl std::fmt::Display for DomainRegistrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
