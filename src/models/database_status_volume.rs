//! DatabaseStatusVolume model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Volume
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseStatusVolume {
    /// Mount path of the volume.
    #[serde(rename = "path")]
    pub path: String,
    /// Percentage of storage used.
    #[serde(rename = "usedPercent")]
    pub used_percent: String,
    /// Available storage space.
    #[serde(rename = "available")]
    pub available: String,
    /// Whether the volume is mounted.
    #[serde(rename = "mounted")]
    pub mounted: bool,
}

impl DatabaseStatusVolume {
    /// Get path
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Get used_percent
    pub fn used_percent(&self) -> &String {
        &self.used_percent
    }

    /// Get available
    pub fn available(&self) -> &String {
        &self.available
    }

    /// Get mounted
    pub fn mounted(&self) -> &bool {
        &self.mounted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_status_volume_creation() {
        let _model = <DatabaseStatusVolume as Default>::default();
        let _ = _model.path();
        let _ = _model.used_percent();
        let _ = _model.available();
        let _ = _model.mounted();
    }

    #[test]
    fn test_database_status_volume_serialization() {
        let model = <DatabaseStatusVolume as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseStatusVolume, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
