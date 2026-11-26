//! EscalationList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Escalations List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EscalationList {
    /// Total number of escalations that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of escalations.
    #[serde(rename = "escalations")]
    #[serde(default = "default_escalations")]
    pub escalations: Vec<crate::models::Escalation>,
}

fn default_total() -> i64 {
    0
}

fn default_escalations() -> Vec<crate::models::Escalation> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl EscalationList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get escalations
    pub fn escalations(&self) -> &Vec<crate::models::Escalation> {
        &self.escalations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation_list_creation() {
        let _model = <EscalationList as Default>::default();
        let _ = _model.total();
        let _ = _model.escalations();
    }

    #[test]
    fn test_escalation_list_serialization() {
        let model = <EscalationList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<EscalationList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
