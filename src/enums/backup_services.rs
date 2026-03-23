use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BackupServices {
    #[serde(rename = "databases")]
    #[default]
    Databases,
    #[serde(rename = "functions")]
    Functions,
    #[serde(rename = "storage")]
    Storage,
}

impl BackupServices {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            BackupServices::Databases => "databases",
            BackupServices::Functions => "functions",
            BackupServices::Storage => "storage",
        }
    }
}

impl std::fmt::Display for BackupServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
