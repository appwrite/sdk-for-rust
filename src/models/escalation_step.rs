//! EscalationStep model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Escalation Step
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EscalationStep {
    /// Seconds to wait before notifying this step: after the alert fires for the
    /// first step, after the previous step for the others.
    #[serde(rename = "delay")]
    #[serde(default = "default_delay")]
    pub delay: i64,
    /// Organization roles whose confirmed members this step notifies.
    #[serde(rename = "roles")]
    #[serde(default = "default_roles")]
    pub roles: Vec<String>,
    /// IDs of the organization's members this step notifies, whatever their role.
    #[serde(rename = "userIds")]
    #[serde(default = "default_user_ids")]
    pub user_ids: Vec<String>,
}

fn default_delay() -> i64 {
    0
}

fn default_roles() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_user_ids() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl EscalationStep {
    /// Get delay
    pub fn delay(&self) -> &i64 {
        &self.delay
    }

    /// Get roles
    pub fn roles(&self) -> &Vec<String> {
        &self.roles
    }

    /// Get user_ids
    pub fn user_ids(&self) -> &Vec<String> {
        &self.user_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation_step_creation() {
        let _model = <EscalationStep as Default>::default();
        let _ = _model.delay();
        let _ = _model.roles();
        let _ = _model.user_ids();
    }

    #[test]
    fn test_escalation_step_serialization() {
        let model = <EscalationStep as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<EscalationStep, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
