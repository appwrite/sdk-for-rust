//! DnsRecordsList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// DNS records list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DnsRecordsList {
    /// Total number of dnsRecords that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of dnsRecords.
    #[serde(rename = "dnsRecords")]
    #[serde(default = "default_dns_records")]
    pub dns_records: Vec<crate::models::DnsRecord>,
}

fn default_total() -> i64 {
    0
}

fn default_dns_records() -> Vec<crate::models::DnsRecord> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl DnsRecordsList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get dns_records
    pub fn dns_records(&self) -> &Vec<crate::models::DnsRecord> {
        &self.dns_records
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_records_list_creation() {
        let _model = <DnsRecordsList as Default>::default();
        let _ = _model.total();
        let _ = _model.dns_records();
    }

    #[test]
    fn test_dns_records_list_serialization() {
        let model = <DnsRecordsList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DnsRecordsList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
