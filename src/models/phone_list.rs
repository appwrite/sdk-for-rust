//! PhoneList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Phones List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct PhoneList {
    /// Total number of phones that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of phones.
    #[serde(rename = "phones")]
    pub phones: Vec<crate::models::Phone>,
}

impl PhoneList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get phones
    pub fn phones(&self) -> &Vec<crate::models::Phone> {
        &self.phones
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_list_creation() {
        let _model = <PhoneList as Default>::default();
        let _ = _model.total();
        let _ = _model.phones();
    }

    #[test]
    fn test_phone_list_serialization() {
        let model = <PhoneList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<PhoneList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
