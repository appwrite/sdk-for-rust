//! Session model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Session {
    /// Session ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Session creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Session update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Session expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    pub expire: String,
    /// Session Provider.
    #[serde(rename = "provider")]
    pub provider: String,
    /// Session Provider User ID.
    #[serde(rename = "providerUid")]
    pub provider_uid: String,
    /// Session Provider Access Token.
    #[serde(rename = "providerAccessToken")]
    pub provider_access_token: String,
    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry")]
    pub provider_access_token_expiry: String,
    /// Session Provider Refresh Token.
    #[serde(rename = "providerRefreshToken")]
    pub provider_refresh_token: String,
    /// IP in use when the session was created.
    #[serde(rename = "ip")]
    pub ip: String,
    /// Operating system code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode")]
    pub os_code: String,
    /// Operating system name.
    #[serde(rename = "osName")]
    pub os_name: String,
    /// Operating system version.
    #[serde(rename = "osVersion")]
    pub os_version: String,
    /// Client type.
    #[serde(rename = "clientType")]
    pub client_type: String,
    /// Client code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode")]
    pub client_code: String,
    /// Client name.
    #[serde(rename = "clientName")]
    pub client_name: String,
    /// Client version.
    #[serde(rename = "clientVersion")]
    pub client_version: String,
    /// Client engine name.
    #[serde(rename = "clientEngine")]
    pub client_engine: String,
    /// Client engine name.
    #[serde(rename = "clientEngineVersion")]
    pub client_engine_version: String,
    /// Device name.
    #[serde(rename = "deviceName")]
    pub device_name: String,
    /// Device brand name.
    #[serde(rename = "deviceBrand")]
    pub device_brand: String,
    /// Device model name.
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName")]
    pub country_name: String,
    /// Returns true if this the current user session.
    #[serde(rename = "current")]
    pub current: bool,
    /// Returns a list of active session factors.
    #[serde(rename = "factors")]
    pub factors: Vec<String>,
    /// Secret used to authenticate the user. Only included if the request was made
    /// with an API key
    #[serde(rename = "secret")]
    pub secret: String,
    /// Most recent date in ISO 8601 format when the session successfully passed
    /// MFA challenge.
    #[serde(rename = "mfaUpdatedAt")]
    pub mfa_updated_at: String,
}

impl Session {
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

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get expire
    pub fn expire(&self) -> &String {
        &self.expire
    }

    /// Get provider
    pub fn provider(&self) -> &String {
        &self.provider
    }

    /// Get provider_uid
    pub fn provider_uid(&self) -> &String {
        &self.provider_uid
    }

    /// Get provider_access_token
    pub fn provider_access_token(&self) -> &String {
        &self.provider_access_token
    }

    /// Get provider_access_token_expiry
    pub fn provider_access_token_expiry(&self) -> &String {
        &self.provider_access_token_expiry
    }

    /// Get provider_refresh_token
    pub fn provider_refresh_token(&self) -> &String {
        &self.provider_refresh_token
    }

    /// Get ip
    pub fn ip(&self) -> &String {
        &self.ip
    }

    /// Get os_code
    pub fn os_code(&self) -> &String {
        &self.os_code
    }

    /// Get os_name
    pub fn os_name(&self) -> &String {
        &self.os_name
    }

    /// Get os_version
    pub fn os_version(&self) -> &String {
        &self.os_version
    }

    /// Get client_type
    pub fn client_type(&self) -> &String {
        &self.client_type
    }

    /// Get client_code
    pub fn client_code(&self) -> &String {
        &self.client_code
    }

    /// Get client_name
    pub fn client_name(&self) -> &String {
        &self.client_name
    }

    /// Get client_version
    pub fn client_version(&self) -> &String {
        &self.client_version
    }

    /// Get client_engine
    pub fn client_engine(&self) -> &String {
        &self.client_engine
    }

    /// Get client_engine_version
    pub fn client_engine_version(&self) -> &String {
        &self.client_engine_version
    }

    /// Get device_name
    pub fn device_name(&self) -> &String {
        &self.device_name
    }

    /// Get device_brand
    pub fn device_brand(&self) -> &String {
        &self.device_brand
    }

    /// Get device_model
    pub fn device_model(&self) -> &String {
        &self.device_model
    }

    /// Get country_code
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// Get country_name
    pub fn country_name(&self) -> &String {
        &self.country_name
    }

    /// Get current
    pub fn current(&self) -> &bool {
        &self.current
    }

    /// Get factors
    pub fn factors(&self) -> &Vec<String> {
        &self.factors
    }

    /// Get secret
    pub fn secret(&self) -> &String {
        &self.secret
    }

    /// Get mfa_updated_at
    pub fn mfa_updated_at(&self) -> &String {
        &self.mfa_updated_at
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let _model = <Session as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.user_id();
        let _ = _model.expire();
        let _ = _model.provider();
        let _ = _model.provider_uid();
        let _ = _model.provider_access_token();
        let _ = _model.provider_access_token_expiry();
        let _ = _model.provider_refresh_token();
        let _ = _model.ip();
        let _ = _model.os_code();
        let _ = _model.os_name();
        let _ = _model.os_version();
        let _ = _model.client_type();
        let _ = _model.client_code();
        let _ = _model.client_name();
        let _ = _model.client_version();
        let _ = _model.client_engine();
        let _ = _model.client_engine_version();
        let _ = _model.device_name();
        let _ = _model.device_brand();
        let _ = _model.device_model();
        let _ = _model.country_code();
        let _ = _model.country_name();
        let _ = _model.current();
        let _ = _model.factors();
        let _ = _model.secret();
        let _ = _model.mfa_updated_at();
    }

    #[test]
    fn test_session_serialization() {
        let model = <Session as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Session, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
