use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WafRuleAction {
    #[serde(rename = "bypass")]
    #[default]
    Bypass,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "rateLimit")]
    RateLimit,
    #[serde(rename = "redirect")]
    Redirect,
}

impl WafRuleAction {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            WafRuleAction::Bypass => "bypass",
            WafRuleAction::Deny => "deny",
            WafRuleAction::Challenge => "challenge",
            WafRuleAction::RateLimit => "rateLimit",
            WafRuleAction::Redirect => "redirect",
        }
    }
}

impl std::fmt::Display for WafRuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
