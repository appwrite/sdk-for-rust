//! Escalation model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Escalation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Escalation {
    /// Escalation ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Escalation creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Escalation update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Escalation name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Who is notified of a firing alert, in order, until a member acknowledges
    /// it.
    #[serde(rename = "steps")]
    #[serde(default = "default_steps")]
    pub steps: Vec<crate::models::EscalationStep>,
    /// How many more times the steps run from the first, a monitor's cooldown
    /// after the last step, while nobody has acknowledged the alert.
    #[serde(rename = "repeat")]
    #[serde(default = "default_repeat")]
    pub repeat: i64,
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

fn default_name() -> String {
    "".to_string()
}

fn default_steps() -> Vec<crate::models::EscalationStep> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_repeat() -> i64 {
    0
}

impl Escalation {
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

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get steps
    pub fn steps(&self) -> &Vec<crate::models::EscalationStep> {
        &self.steps
    }

    /// Get repeat
    pub fn repeat(&self) -> &i64 {
        &self.repeat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation_creation() {
        let _model = <Escalation as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.steps();
        let _ = _model.repeat();
    }

    #[test]
    fn test_escalation_serialization() {
        let model = <Escalation as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Escalation, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
