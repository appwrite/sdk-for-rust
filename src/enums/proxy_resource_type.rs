use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProxyResourceType {
    #[serde(rename = "site")]
    #[default]
    Site,
    #[serde(rename = "function")]
    Function,
}

impl ProxyResourceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProxyResourceType::Site => "site",
            ProxyResourceType::Function => "function",
        }
    }
}

impl std::fmt::Display for ProxyResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
