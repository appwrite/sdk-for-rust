use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProxyRuleDeploymentResourceType {
    #[serde(rename = "function")]
    #[default]
    Function,
    #[serde(rename = "site")]
    Site,
}

impl ProxyRuleDeploymentResourceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProxyRuleDeploymentResourceType::Function => "function",
            ProxyRuleDeploymentResourceType::Site => "site",
        }
    }
}

impl std::fmt::Display for ProxyRuleDeploymentResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
