//! User model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct User {
    /// User ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// User creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// User update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// User name.
    #[serde(rename = "name")]
    pub name: String,
    /// Hashed user password.
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Password hashing algorithm.
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Password hashing algorithm configuration.
    #[serde(rename = "hashOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_options: Option<serde_json::Value>,
    /// User registration date in ISO 8601 format.
    #[serde(rename = "registration")]
    pub registration: String,
    /// User status. Pass `true` for enabled and `false` for disabled.
    #[serde(rename = "status")]
    pub status: bool,
    /// Labels for the user.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// Password update time in ISO 8601 format.
    #[serde(rename = "passwordUpdate")]
    pub password_update: String,
    /// User email address.
    #[serde(rename = "email")]
    pub email: String,
    /// User phone number in E.164 format.
    #[serde(rename = "phone")]
    pub phone: String,
    /// Email verification status.
    #[serde(rename = "emailVerification")]
    pub email_verification: bool,
    /// Phone verification status.
    #[serde(rename = "phoneVerification")]
    pub phone_verification: bool,
    /// Multi factor authentication status.
    #[serde(rename = "mfa")]
    pub mfa: bool,
    /// User preferences as a key-value object
    #[serde(rename = "prefs")]
    pub prefs: crate::models::Preferences,
    /// A user-owned message receiver. A single user may have multiple e.g. emails,
    /// phones, and a browser. Each target is registered with a single provider.
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::Target>,
    /// Most recent access date in ISO 8601 format. This attribute is only updated
    /// again after 24 hours.
    #[serde(rename = "accessedAt")]
    pub accessed_at: String,
    /// Whether the user can impersonate other users.
    #[serde(rename = "impersonator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impersonator: Option<bool>,
    /// ID of the original actor performing the impersonation. Present only when
    /// the current request is impersonating another user. Internal audit logs
    /// attribute the action to this user, while the impersonated target is
    /// recorded only in internal audit payload data.
    #[serde(rename = "impersonatorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impersonator_user_id: Option<String>,
}

impl User {
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

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Set password
    pub fn set_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    /// Get password
    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Set hash
    pub fn set_hash(mut self, hash: String) -> Self {
        self.hash = Some(hash);
        self
    }

    /// Get hash
    pub fn hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    /// Set hash_options
    pub fn set_hash_options(mut self, hash_options: serde_json::Value) -> Self {
        self.hash_options = Some(hash_options);
        self
    }

    /// Get hash_options
    pub fn hash_options(&self) -> Option<&serde_json::Value> {
        self.hash_options.as_ref()
    }

    /// Get registration
    pub fn registration(&self) -> &String {
        &self.registration
    }

    /// Get status
    pub fn status(&self) -> &bool {
        &self.status
    }

    /// Get labels
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Get password_update
    pub fn password_update(&self) -> &String {
        &self.password_update
    }

    /// Get email
    pub fn email(&self) -> &String {
        &self.email
    }

    /// Get phone
    pub fn phone(&self) -> &String {
        &self.phone
    }

    /// Get email_verification
    pub fn email_verification(&self) -> &bool {
        &self.email_verification
    }

    /// Get phone_verification
    pub fn phone_verification(&self) -> &bool {
        &self.phone_verification
    }

    /// Get mfa
    pub fn mfa(&self) -> &bool {
        &self.mfa
    }

    /// Get prefs
    pub fn prefs(&self) -> &crate::models::Preferences {
        &self.prefs
    }

    /// Get targets
    pub fn targets(&self) -> &Vec<crate::models::Target> {
        &self.targets
    }

    /// Get accessed_at
    pub fn accessed_at(&self) -> &String {
        &self.accessed_at
    }

    /// Set impersonator
    pub fn set_impersonator(mut self, impersonator: bool) -> Self {
        self.impersonator = Some(impersonator);
        self
    }

    /// Get impersonator
    pub fn impersonator(&self) -> Option<&bool> {
        self.impersonator.as_ref()
    }

    /// Set impersonator_user_id
    pub fn set_impersonator_user_id(mut self, impersonator_user_id: String) -> Self {
        self.impersonator_user_id = Some(impersonator_user_id);
        self
    }

    /// Get impersonator_user_id
    pub fn impersonator_user_id(&self) -> Option<&String> {
        self.impersonator_user_id.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let _model = <User as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.registration();
        let _ = _model.status();
        let _ = _model.labels();
        let _ = _model.password_update();
        let _ = _model.email();
        let _ = _model.phone();
        let _ = _model.email_verification();
        let _ = _model.phone_verification();
        let _ = _model.mfa();
        let _ = _model.prefs();
        let _ = _model.targets();
        let _ = _model.accessed_at();
    }

    #[test]
    fn test_user_serialization() {
        let model = <User as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<User, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
