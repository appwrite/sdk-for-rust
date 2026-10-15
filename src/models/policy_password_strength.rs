//! PolicyPasswordStrength model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Policy Password Strength
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PolicyPasswordStrength {
    /// Policy ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Minimum password length required for user passwords.
    #[serde(rename = "min")]
    #[serde(default = "default_min")]
    pub min: i64,
    /// Whether passwords must include at least one uppercase letter.
    #[serde(rename = "uppercase")]
    #[serde(default = "default_uppercase")]
    pub uppercase: bool,
    /// Whether passwords must include at least one lowercase letter.
    #[serde(rename = "lowercase")]
    #[serde(default = "default_lowercase")]
    pub lowercase: bool,
    /// Whether passwords must include at least one number.
    #[serde(rename = "number")]
    #[serde(default = "default_number")]
    pub number: bool,
    /// Whether passwords must include at least one symbol.
    #[serde(rename = "symbols")]
    #[serde(default = "default_symbols")]
    pub symbols: bool,
}

fn default_id() -> String {
    "".to_string()
}

fn default_min() -> i64 {
    8
}

fn default_uppercase() -> bool {
    false
}

fn default_lowercase() -> bool {
    false
}

fn default_number() -> bool {
    false
}

fn default_symbols() -> bool {
    false
}

impl PolicyPasswordStrength {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get min
    pub fn min(&self) -> &i64 {
        &self.min
    }

    /// Get uppercase
    pub fn uppercase(&self) -> &bool {
        &self.uppercase
    }

    /// Get lowercase
    pub fn lowercase(&self) -> &bool {
        &self.lowercase
    }

    /// Get number
    pub fn number(&self) -> &bool {
        &self.number
    }

    /// Get symbols
    pub fn symbols(&self) -> &bool {
        &self.symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_password_strength_creation() {
        let _model = <PolicyPasswordStrength as Default>::default();
        let _ = _model.id();
        let _ = _model.min();
        let _ = _model.uppercase();
        let _ = _model.lowercase();
        let _ = _model.number();
        let _ = _model.symbols();
    }

    #[test]
    fn test_policy_password_strength_serialization() {
        let model = <PolicyPasswordStrength as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PolicyPasswordStrength, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
