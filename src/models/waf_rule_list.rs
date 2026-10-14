//! WafRuleList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// WAF rule list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct WafRuleList {
    /// Total number of rules that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of rules.
    #[serde(rename = "rules")]
    #[serde(default = "default_rules")]
    pub rules: Vec<serde_json::Value>,
}

fn default_total() -> i64 {
    0
}

fn default_rules() -> Vec<serde_json::Value> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl WafRuleList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get rules
    pub fn rules(&self) -> &Vec<serde_json::Value> {
        &self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waf_rule_list_creation() {
        let _model = <WafRuleList as Default>::default();
        let _ = _model.total();
        let _ = _model.rules();
    }

    #[test]
    fn test_waf_rule_list_serialization() {
        let model = <WafRuleList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<WafRuleList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
