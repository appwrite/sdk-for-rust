//! DnsRecord model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// DNSRecord
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DnsRecord {
    /// DNS Record ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// DNS Record creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// DNS Record update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// DNS record type (e.g. A, CNAME, MX).
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub r#type: String,
    /// Record name relative to the domain, or `@` for the domain itself.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Value of the record (IP address, domain, etc.).
    #[serde(rename = "value")]
    #[serde(default = "default_value")]
    pub value: String,
    /// Time to live (in seconds).
    #[serde(rename = "ttl")]
    #[serde(default = "default_ttl")]
    pub ttl: i64,
    /// Record priority (commonly used for MX).
    #[serde(rename = "priority")]
    #[serde(default = "default_priority")]
    pub priority: i64,
    /// Whether this record is locked (read-only).
    #[serde(rename = "lock")]
    #[serde(default = "default_lock")]
    pub lock: bool,
    /// Record weight (used for SRV records).
    #[serde(rename = "weight")]
    #[serde(default = "default_weight")]
    pub weight: i64,
    /// Target port (used for SRV records).
    #[serde(rename = "port")]
    #[serde(default = "default_port")]
    pub port: i64,
    /// Comment for the DNS record.
    #[serde(rename = "comment")]
    #[serde(default = "default_comment")]
    pub comment: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_type() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_value() -> String {
    "".to_string()
}

fn default_ttl() -> i64 {
    3600
}

fn default_priority() -> i64 {
    0
}

fn default_lock() -> bool {
    false
}

fn default_weight() -> i64 {
    0
}

fn default_port() -> i64 {
    0
}

fn default_comment() -> String {
    "".to_string()
}

impl DnsRecord {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get value
    pub fn value(&self) -> &String {
        &self.value
    }

    /// Get ttl
    pub fn ttl(&self) -> &i64 {
        &self.ttl
    }

    /// Get priority
    pub fn priority(&self) -> &i64 {
        &self.priority
    }

    /// Get lock
    pub fn lock(&self) -> &bool {
        &self.lock
    }

    /// Get weight
    pub fn weight(&self) -> &i64 {
        &self.weight
    }

    /// Get port
    pub fn port(&self) -> &i64 {
        &self.port
    }

    /// Get comment
    pub fn comment(&self) -> &String {
        &self.comment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_record_creation() {
        let _model = <DnsRecord as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.r#type();
        let _ = _model.name();
        let _ = _model.value();
        let _ = _model.ttl();
        let _ = _model.priority();
        let _ = _model.lock();
        let _ = _model.weight();
        let _ = _model.port();
        let _ = _model.comment();
    }

    #[test]
    fn test_dns_record_serialization() {
        let model = <DnsRecord as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DnsRecord, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
