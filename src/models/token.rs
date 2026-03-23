//! Token model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Token {
    /// Token ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Token secret key. This will return an empty string unless the response is
    /// returned using an API key or as part of a webhook payload.
    #[serde(rename = "secret")]
    pub secret: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    pub expire: String,
    /// Security phrase of a token. Empty if security phrase was not requested when
    /// creating a token. It includes randomly generated phrase which is also sent
    /// in the external resource such as email.
    #[serde(rename = "phrase")]
    pub phrase: String,
}

impl Token {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get phrase
    pub fn phrase(&self) -> &String {
        &self.phrase
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let _model = <Token as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.user_id();
        let _ = _model.secret();
        let _ = _model.expire();
        let _ = _model.phrase();
    }

    #[test]
    fn test_token_serialization() {
        let model = <Token as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Token, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
