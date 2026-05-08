use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProxyRuleStatus {
    #[serde(rename = "unverified")]
    #[default]
    Unverified,
    #[serde(rename = "verifying")]
    Verifying,
    #[serde(rename = "verified")]
    Verified,
}

impl ProxyRuleStatus {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProxyRuleStatus::Unverified => "unverified",
            ProxyRuleStatus::Verifying => "verifying",
            ProxyRuleStatus::Verified => "verified",
        }
    }
}

impl std::fmt::Display for ProxyRuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
