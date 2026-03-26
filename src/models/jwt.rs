//! Jwt model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Jwt {
    /// JWT encoded string.
    #[serde(rename = "jwt")]
    pub jwt: String,
}

impl Jwt {
    /// Get jwt
    pub fn jwt(&self) -> &String {
        &self.jwt
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_creation() {
        let _model = <Jwt as Default>::default();
        let _ = _model.jwt();
    }

    #[test]
    fn test_jwt_serialization() {
        let model = <Jwt as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Jwt, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
