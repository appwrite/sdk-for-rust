//! ProxyRuleList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Rule List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ProxyRuleList {
    /// Total number of rules that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of rules.
    #[serde(rename = "rules")]
    #[serde(default = "default_rules")]
    pub rules: Vec<crate::models::ProxyRule>,
}

fn default_total() -> i64 {
    0
}

fn default_rules() -> Vec<crate::models::ProxyRule> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl ProxyRuleList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get rules
    pub fn rules(&self) -> &Vec<crate::models::ProxyRule> {
        &self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_rule_list_creation() {
        let _model = <ProxyRuleList as Default>::default();
        let _ = _model.total();
        let _ = _model.rules();
    }

    #[test]
    fn test_proxy_rule_list_serialization() {
        let model = <ProxyRuleList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ProxyRuleList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
