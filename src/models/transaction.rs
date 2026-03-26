//! Transaction model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Transaction {
    /// Transaction ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Transaction creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Transaction update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Current status of the transaction. One of: pending, committing, committed,
    /// rolled_back, failed.
    #[serde(rename = "status")]
    pub status: String,
    /// Number of operations in the transaction.
    #[serde(rename = "operations")]
    pub operations: i64,
    /// Expiration time in ISO 8601 format.
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

impl Transaction {
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

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get operations
    pub fn operations(&self) -> &i64 {
        &self.operations
    }

    /// Get expires_at
    pub fn expires_at(&self) -> &String {
        &self.expires_at
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let _model = <Transaction as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.status();
        let _ = _model.operations();
        let _ = _model.expires_at();
    }

    #[test]
    fn test_transaction_serialization() {
        let model = <Transaction as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Transaction, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
