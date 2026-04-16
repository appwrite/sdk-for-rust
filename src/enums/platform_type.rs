use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PlatformType {
    #[serde(rename = "windows")]
    #[default]
    Windows,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "web")]
    Web,
}

impl PlatformType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            PlatformType::Windows => "windows",
            PlatformType::Apple => "apple",
            PlatformType::Android => "android",
            PlatformType::Linux => "linux",
            PlatformType::Web => "web",
        }
    }
}

impl std::fmt::Display for PlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
