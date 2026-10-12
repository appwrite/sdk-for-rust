//! Session model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Session {
    /// Session ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Session creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Session update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// User ID.
    #[serde(rename = "userId")]
    #[serde(default = "default_user_id")]
    pub user_id: String,
    /// Session expiration date in ISO 8601 format.
    #[serde(rename = "expire")]
    #[serde(default = "default_expire")]
    pub expire: String,
    /// Session Provider.
    #[serde(rename = "provider")]
    #[serde(default = "default_provider")]
    pub provider: String,
    /// Session Provider User ID.
    #[serde(rename = "providerUid")]
    #[serde(default = "default_provider_uid")]
    pub provider_uid: String,
    /// Session Provider Access Token.
    #[serde(rename = "providerAccessToken")]
    #[serde(default = "default_provider_access_token")]
    pub provider_access_token: String,
    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry")]
    #[serde(default = "default_provider_access_token_expiry")]
    pub provider_access_token_expiry: String,
    /// Session Provider Refresh Token.
    #[serde(rename = "providerRefreshToken")]
    #[serde(default = "default_provider_refresh_token")]
    pub provider_refresh_token: String,
    /// IP in use when the session was created.
    #[serde(rename = "ip")]
    #[serde(default = "default_ip")]
    pub ip: String,
    /// Operating system code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode")]
    #[serde(default = "default_os_code")]
    pub os_code: String,
    /// Operating system name.
    #[serde(rename = "osName")]
    #[serde(default = "default_os_name")]
    pub os_name: String,
    /// Operating system version.
    #[serde(rename = "osVersion")]
    #[serde(default = "default_os_version")]
    pub os_version: String,
    /// Client type.
    #[serde(rename = "clientType")]
    #[serde(default = "default_client_type")]
    pub client_type: String,
    /// Client code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode")]
    #[serde(default = "default_client_code")]
    pub client_code: String,
    /// Client name.
    #[serde(rename = "clientName")]
    #[serde(default = "default_client_name")]
    pub client_name: String,
    /// Client version.
    #[serde(rename = "clientVersion")]
    #[serde(default = "default_client_version")]
    pub client_version: String,
    /// Client engine name.
    #[serde(rename = "clientEngine")]
    #[serde(default = "default_client_engine")]
    pub client_engine: String,
    /// Client engine name.
    #[serde(rename = "clientEngineVersion")]
    #[serde(default = "default_client_engine_version")]
    pub client_engine_version: String,
    /// Device name.
    #[serde(rename = "deviceName")]
    #[serde(default = "default_device_name")]
    pub device_name: String,
    /// Device brand name.
    #[serde(rename = "deviceBrand")]
    #[serde(default = "default_device_brand")]
    pub device_brand: String,
    /// Device model name.
    #[serde(rename = "deviceModel")]
    #[serde(default = "default_device_model")]
    pub device_model: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    #[serde(default = "default_country_code")]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName")]
    #[serde(default = "default_country_name")]
    pub country_name: String,
    /// Returns true if this the current user session.
    #[serde(rename = "current")]
    #[serde(default = "default_current")]
    pub current: bool,
    /// Returns a list of active session factors.
    #[serde(rename = "factors")]
    #[serde(default = "default_factors")]
    pub factors: Vec<String>,
    /// Secret used to authenticate the user. Only included if the request was made
    /// with an API key
    #[serde(rename = "secret")]
    #[serde(default = "default_secret")]
    pub secret: String,
    /// Most recent date in ISO 8601 format when the session successfully passed
    /// MFA challenge.
    #[serde(rename = "mfaUpdatedAt")]
    #[serde(default = "default_mfa_updated_at")]
    pub mfa_updated_at: String,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_user_id() -> String {
    "".to_string()
}

fn default_expire() -> String {
    "".to_string()
}

fn default_provider() -> String {
    "".to_string()
}

fn default_provider_uid() -> String {
    "".to_string()
}

fn default_provider_access_token() -> String {
    "".to_string()
}

fn default_provider_access_token_expiry() -> String {
    "".to_string()
}

fn default_provider_refresh_token() -> String {
    "".to_string()
}

fn default_ip() -> String {
    "".to_string()
}

fn default_os_code() -> String {
    "".to_string()
}

fn default_os_name() -> String {
    "".to_string()
}

fn default_os_version() -> String {
    "".to_string()
}

fn default_client_type() -> String {
    "".to_string()
}

fn default_client_code() -> String {
    "".to_string()
}

fn default_client_name() -> String {
    "".to_string()
}

fn default_client_version() -> String {
    "".to_string()
}

fn default_client_engine() -> String {
    "".to_string()
}

fn default_client_engine_version() -> String {
    "".to_string()
}

fn default_device_name() -> String {
    "".to_string()
}

fn default_device_brand() -> String {
    "".to_string()
}

fn default_device_model() -> String {
    "".to_string()
}

fn default_country_code() -> String {
    "".to_string()
}

fn default_country_name() -> String {
    "".to_string()
}

fn default_current() -> bool {
    false
}

fn default_factors() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_secret() -> String {
    "".to_string()
}

fn default_mfa_updated_at() -> String {
    "".to_string()
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
