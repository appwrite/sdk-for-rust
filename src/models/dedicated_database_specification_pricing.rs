//! DedicatedDatabaseSpecificationPricing model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// SpecificationPricing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DedicatedDatabaseSpecificationPricing {
    /// Price per GB of storage above the included amount, per month, in USD.
    #[serde(rename = "storageOverageRate")]
    pub storage_overage_rate: f64,
    /// Price per GB of bandwidth above the included amount, per month, in USD.
    #[serde(rename = "bandwidthOverageRate")]
    pub bandwidth_overage_rate: f64,
    /// High availability replica price as a fraction of the specification cost.
    #[serde(rename = "replicaRate")]
    pub replica_rate: f64,
    /// Point-in-time recovery price as a fraction of the specification cost.
    #[serde(rename = "pitrRate")]
    pub pitr_rate: f64,
}

impl DedicatedDatabaseSpecificationPricing {
    /// Get storage_overage_rate
    pub fn storage_overage_rate(&self) -> &f64 {
        &self.storage_overage_rate
    }

    /// Get bandwidth_overage_rate
    pub fn bandwidth_overage_rate(&self) -> &f64 {
        &self.bandwidth_overage_rate
    }

    /// Get replica_rate
    pub fn replica_rate(&self) -> &f64 {
        &self.replica_rate
    }

    /// Get pitr_rate
    pub fn pitr_rate(&self) -> &f64 {
        &self.pitr_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedicated_database_specification_pricing_creation() {
        let _model = <DedicatedDatabaseSpecificationPricing as Default>::default();
        let _ = _model.storage_overage_rate();
        let _ = _model.bandwidth_overage_rate();
        let _ = _model.replica_rate();
        let _ = _model.pitr_rate();
    }

    #[test]
    fn test_dedicated_database_specification_pricing_serialization() {
        let model = <DedicatedDatabaseSpecificationPricing as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DedicatedDatabaseSpecificationPricing, _> =
            serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
