//! DedicatedDatabasePITRWindows model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// PITRWindows
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabasePITRWindows {
    /// Earliest available recovery point.
    #[serde(rename = "earliest")]
    #[serde(default = "default_earliest")]
    pub earliest: String,
    /// Latest available recovery point.
    #[serde(rename = "latest")]
    #[serde(default = "default_latest")]
    pub latest: String,
}

fn default_earliest() -> String {
    "".to_string()
}

fn default_latest() -> String {
    "".to_string()
}

impl DedicatedDatabasePITRWindows {
    /// Get earliest
    pub fn earliest(&self) -> &String {
        &self.earliest
    }

    /// Get latest
    pub fn latest(&self) -> &String {
        &self.latest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_pitr_windows_creation() {
        let _model = <DedicatedDatabasePITRWindows as Default>::default();
        let _ = _model.earliest();
        let _ = _model.latest();
    }

    #[test]
    fn test_dedicated_database_pitr_windows_serialization() {
        let model = <DedicatedDatabasePITRWindows as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabasePITRWindows, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
