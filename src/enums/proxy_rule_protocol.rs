use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProxyRuleProtocol {
    #[serde(rename = "http")]
    #[default]
    Http,
    #[serde(rename = "smtp")]
    Smtp,
}

impl ProxyRuleProtocol {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProxyRuleProtocol::Http => "http",
            ProxyRuleProtocol::Smtp => "smtp",
        }
    }
}

impl std::fmt::Display for ProxyRuleProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
