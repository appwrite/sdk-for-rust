//! Oauth2Project model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// OAuth2 Project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Oauth2Project {
    /// Project ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Region ID the project is deployed in.
    #[serde(rename = "region")]
    pub region: String,
    /// API endpoint of the region the project is deployed in. Empty when the
    /// region has no public hostname configured.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
}

impl Oauth2Project {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get region
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Get endpoint
    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_project_creation() {
        let _model = <Oauth2Project as Default>::default();
        let _ = _model.id();
        let _ = _model.region();
        let _ = _model.endpoint();
    }

    #[test]
    fn test_oauth2_project_serialization() {
        let model = <Oauth2Project as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Oauth2Project, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
