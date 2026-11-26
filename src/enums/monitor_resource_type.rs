use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorResourceType {
    #[serde(rename = "project")]
    #[default]
    Project,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "bucket")]
    Bucket,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "dedicatedDatabase")]
    DedicatedDatabase,
    #[serde(rename = "organization")]
    Organization,
}

impl MonitorResourceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorResourceType::Project => "project",
            MonitorResourceType::Database => "database",
            MonitorResourceType::Bucket => "bucket",
            MonitorResourceType::Function => "function",
            MonitorResourceType::Site => "site",
            MonitorResourceType::DedicatedDatabase => "dedicatedDatabase",
            MonitorResourceType::Organization => "organization",
        }
    }
}

impl std::fmt::Display for MonitorResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
