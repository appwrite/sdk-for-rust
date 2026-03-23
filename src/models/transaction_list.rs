//! TransactionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Transaction List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TransactionList {
    /// Total number of transactions that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of transactions.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::Transaction>,
}

impl TransactionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get transactions
    pub fn transactions(&self) -> &Vec<crate::models::Transaction> {
        &self.transactions
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_list_creation() {
        let _model = <TransactionList as Default>::default();
        let _ = _model.total();
        let _ = _model.transactions();
    }

    #[test]
    fn test_transaction_list_serialization() {
        let model = <TransactionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<TransactionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
