//! DomainTransferStatus model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// domainTransferStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DomainTransferStatus {
    /// Transfer status.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: crate::enums::DomainTransferStatusEnum,
    /// Additional transfer status information.
    #[serde(rename = "reason")]
    #[serde(default = "default_reason")]
    pub reason: String,
    /// Transfer status timestamp in ISO 8601 format.
    #[serde(rename = "timestamp")]
    #[serde(default = "default_timestamp")]
    pub timestamp: String,
}

fn default_status() -> crate::enums::DomainTransferStatusEnum {
    serde_json::from_value(serde_json::json!("pending_registry")).unwrap()
}

fn default_reason() -> String {
    "".to_string()
}

fn default_timestamp() -> String {
    "".to_string()
}

impl DomainTransferStatus {
    /// Get status
    pub fn status(&self) -> &crate::enums::DomainTransferStatusEnum {
        &self.status
    }

    /// Get reason
    pub fn reason(&self) -> &String {
        &self.reason
    }

    /// Get timestamp
    pub fn timestamp(&self) -> &String {
        &self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_transfer_status_creation() {
        let _model = <DomainTransferStatus as Default>::default();
        let _ = _model.status();
        let _ = _model.reason();
        let _ = _model.timestamp();
    }

    #[test]
    fn test_domain_transfer_status_serialization() {
        let model = <DomainTransferStatus as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DomainTransferStatus, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
