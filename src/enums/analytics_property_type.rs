use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnalyticsPropertyType {
    #[serde(rename = "web")]
    #[default]
    Web,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "agent")]
    Agent,
}

impl AnalyticsPropertyType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AnalyticsPropertyType::Web => "web",
            AnalyticsPropertyType::Mobile => "mobile",
            AnalyticsPropertyType::Agent => "agent",
        }
    }
}

impl std::fmt::Display for AnalyticsPropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
