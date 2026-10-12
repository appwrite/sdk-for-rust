//! WafRuleChallenge model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// WafRuleChallenge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct WafRuleChallenge {
    /// Rule ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// WAF rule creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// WAF rule last update time in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Human friendly rule name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Optional description for the rule.
    #[serde(rename = "description")]
    #[serde(default = "default_description")]
    pub description: String,
    /// Team ID.
    #[serde(rename = "teamId")]
    #[serde(default = "default_team_id")]
    pub team_id: String,
    /// Project ID.
    #[serde(rename = "projectId")]
    #[serde(default = "default_project_id")]
    pub project_id: String,
    /// Resource type the rule is scoped to.
    #[serde(rename = "resourceType")]
    #[serde(default = "default_resource_type")]
    pub resource_type: String,
    /// Resource identifier. Empty for API-wide rules.
    #[serde(rename = "resourceId")]
    #[serde(default = "default_resource_id")]
    pub resource_id: String,
    /// Action performed when the rule matches.
    #[serde(rename = "action")]
    pub action: crate::enums::WafRuleAction,
    /// Evaluation priority. Lower values execute earlier.
    #[serde(rename = "priority")]
    #[serde(default = "default_priority")]
    pub priority: i64,
    /// Whether the rule is active.
    #[serde(rename = "enabled")]
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// List of conditions evaluated for this rule.
    #[serde(rename = "conditions")]
    #[serde(default = "default_conditions")]
    pub conditions: Vec<serde_json::Value>,
    /// Action specific configuration.
    #[serde(rename = "config")]
    #[serde(default = "default_config")]
    pub config: serde_json::Value,
    /// Challenge type enforced when the rule matches.
    #[serde(rename = "challengeType")]
    #[serde(default = "default_challenge_type")]
    pub challenge_type: String,
    /// Challenge difficulty from 1 (easiest) to 5 (hardest) enforced when the rule
    /// matches.
    #[serde(rename = "difficulty")]
    #[serde(default = "default_difficulty")]
    pub difficulty: i64,
    /// Seconds a visitor stays cleared after passing the challenge before being
    /// challenged again.
    #[serde(rename = "ttl")]
    #[serde(default = "default_ttl")]
    pub ttl: i64,
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

fn default_description() -> String {
    "".to_string()
}

fn default_team_id() -> String {
    "".to_string()
}

fn default_project_id() -> String {
    "".to_string()
}

fn default_resource_type() -> String {
    "api".to_string()
}

fn default_resource_id() -> String {
    "".to_string()
}

fn default_priority() -> i64 {
    0
}

fn default_enabled() -> bool {
    true
}

fn default_conditions() -> Vec<serde_json::Value> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_config() -> serde_json::Value {
    serde_json::json!({})
}

fn default_challenge_type() -> String {
    "compute".to_string()
}

fn default_difficulty() -> i64 {
    3
}

fn default_ttl() -> i64 {
    1800
}

impl WafRuleChallenge {
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

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get action
    pub fn action(&self) -> &crate::enums::WafRuleAction {
        &self.action
    }

    /// Get priority
    pub fn priority(&self) -> &i64 {
        &self.priority
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get conditions
    pub fn conditions(&self) -> &Vec<serde_json::Value> {
        &self.conditions
    }

    /// Get config
    pub fn config(&self) -> &serde_json::Value {
        &self.config
    }

    /// Get challenge_type
    pub fn challenge_type(&self) -> &String {
        &self.challenge_type
    }

    /// Get difficulty
    pub fn difficulty(&self) -> &i64 {
        &self.difficulty
    }

    /// Get ttl
    pub fn ttl(&self) -> &i64 {
        &self.ttl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waf_rule_challenge_creation() {
        let _model = <WafRuleChallenge as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.description();
        let _ = _model.team_id();
        let _ = _model.project_id();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.action();
        let _ = _model.priority();
        let _ = _model.enabled();
        let _ = _model.conditions();
        let _ = _model.config();
        let _ = _model.challenge_type();
        let _ = _model.difficulty();
        let _ = _model.ttl();
    }

    #[test]
    fn test_waf_rule_challenge_serialization() {
        let model = <WafRuleChallenge as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<WafRuleChallenge, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
