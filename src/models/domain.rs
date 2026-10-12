//! Domain model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Domain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Domain {
    /// Domain ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Domain creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Domain update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Domain name.
    #[serde(rename = "domain")]
    #[serde(default = "default_domain")]
    pub domain: String,
    /// Domain registrar (e.g. "appwrite" or "third_party").
    #[serde(rename = "registrar")]
    #[serde(default = "default_registrar")]
    pub registrar: String,
    /// Nameservers setting. "Appwrite" or empty string.
    #[serde(rename = "nameservers")]
    #[serde(default = "default_nameservers")]
    pub nameservers: String,
    /// Domain expiry date in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
    /// Domain renewal date in ISO 8601 format.
    #[serde(rename = "renewal")]
    #[serde(default = "default_renewal")]
    pub renewal: String,
    /// If set to true, the domain will automatically renew.
    #[serde(rename = "autoRenewal")]
    #[serde(default = "default_auto_renewal")]
    pub auto_renewal: bool,
    /// Renewal price (in cents).
    #[serde(rename = "renewalPrice")]
    #[serde(default = "default_renewal_price")]
    pub renewal_price: i64,
    /// Transfer status for domains being transferred in. Null when the domain is
    /// not being transferred.
    #[serde(rename = "transferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<crate::enums::DomainTransferStatusEnum>,
    /// Team ID.
    #[serde(rename = "teamId")]
    #[serde(default = "default_team_id")]
    pub team_id: String,
    /// Dns records
    #[serde(rename = "dnsRecords")]
    pub dns_records: Vec<crate::models::DnsRecord>,
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

fn default_domain() -> String {
    "".to_string()
}

fn default_registrar() -> String {
    "".to_string()
}

fn default_nameservers() -> String {
    "".to_string()
}

fn default_expire() -> String {
    "".to_string()
}

fn default_renewal() -> String {
    "".to_string()
}

fn default_auto_renewal() -> bool {
    false
}

fn default_renewal_price() -> i64 {
    0
}

fn default_team_id() -> String {
    "".to_string()
}

impl Domain {
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

    /// Get domain
    pub fn domain(&self) -> &String {
        &self.domain
    }

    /// Get registrar
    pub fn registrar(&self) -> &String {
        &self.registrar
    }

    /// Get nameservers
    pub fn nameservers(&self) -> &String {
        &self.nameservers
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get renewal
    pub fn renewal(&self) -> &String {
        &self.renewal
    }

    /// Get auto_renewal
    pub fn auto_renewal(&self) -> &bool {
        &self.auto_renewal
    }

    /// Get renewal_price
    pub fn renewal_price(&self) -> &i64 {
        &self.renewal_price
    }

    /// Set transfer_status
    pub fn set_transfer_status(
        mut self,
        transfer_status: crate::enums::DomainTransferStatusEnum,
    ) -> Self {
        self.transfer_status = Some(transfer_status);
        self
    }

    /// Get transfer_status
    pub fn transfer_status(&self) -> Option<&crate::enums::DomainTransferStatusEnum> {
        self.transfer_status.as_ref()
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
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
    fn test_domain_creation() {
        let _model = <Domain as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.domain();
        let _ = _model.registrar();
        let _ = _model.nameservers();
        let _ = _model.expire();
        let _ = _model.renewal();
        let _ = _model.auto_renewal();
        let _ = _model.renewal_price();
        let _ = _model.team_id();
        let _ = _model.dns_records();
    }

    #[test]
    fn test_domain_serialization() {
        let model = <Domain as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Domain, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
