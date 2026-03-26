//! AlgoScryptModified model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AlgoScryptModified
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AlgoScryptModified {
    /// Algo type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Salt used to compute hash.
    #[serde(rename = "salt")]
    pub salt: String,
    /// Separator used to compute hash.
    #[serde(rename = "saltSeparator")]
    pub salt_separator: String,
    /// Key used to compute hash.
    #[serde(rename = "signerKey")]
    pub signer_key: String,
}

impl AlgoScryptModified {
    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get salt
    pub fn salt(&self) -> &String {
        &self.salt
    }

    /// Get salt_separator
    pub fn salt_separator(&self) -> &String {
        &self.salt_separator
    }

    /// Get signer_key
    pub fn signer_key(&self) -> &String {
        &self.signer_key
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_scrypt_modified_creation() {
        let _model = <AlgoScryptModified as Default>::default();
        let _ = _model.r#type();
        let _ = _model.salt();
        let _ = _model.salt_separator();
        let _ = _model.signer_key();
    }

    #[test]
    fn test_algo_scrypt_modified_serialization() {
        let model = <AlgoScryptModified as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AlgoScryptModified, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
