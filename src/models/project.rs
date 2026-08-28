//! Project model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Project {
    /// Project ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Project creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Project update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Project name.
    #[serde(rename = "name")]
    pub name: String,
    /// Project team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// Project region
    #[serde(rename = "region")]
    pub region: String,
    /// Deprecated since 1.9.5: List of dev keys.
    #[serde(rename = "devKeys")]
    pub dev_keys: Vec<crate::models::DevKey>,
    /// Status for custom SMTP
    #[serde(rename = "smtpEnabled")]
    pub smtp_enabled: bool,
    /// SMTP sender name
    #[serde(rename = "smtpSenderName")]
    pub smtp_sender_name: String,
    /// SMTP sender email
    #[serde(rename = "smtpSenderEmail")]
    pub smtp_sender_email: String,
    /// SMTP reply to name
    #[serde(rename = "smtpReplyToName")]
    pub smtp_reply_to_name: String,
    /// SMTP reply to email
    #[serde(rename = "smtpReplyToEmail")]
    pub smtp_reply_to_email: String,
    /// SMTP server host name
    #[serde(rename = "smtpHost")]
    pub smtp_host: String,
    /// SMTP server port
    #[serde(rename = "smtpPort")]
    pub smtp_port: i64,
    /// SMTP server username
    #[serde(rename = "smtpUsername")]
    pub smtp_username: String,
    /// SMTP server password. This property is write-only and always returned
    /// empty.
    #[serde(rename = "smtpPassword")]
    pub smtp_password: String,
    /// SMTP server secure protocol
    #[serde(rename = "smtpSecure")]
    pub smtp_secure: String,
    /// Number of times the ping was received for this project.
    #[serde(rename = "pingCount")]
    pub ping_count: i64,
    /// Last ping datetime in ISO 8601 format.
    #[serde(rename = "pingedAt")]
    pub pinged_at: String,
    /// Labels for the project.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// Project status
    #[serde(rename = "status")]
    pub status: String,
    /// Stage progress (completed or skipped) with timestamps and actor types,
    /// keyed by stage id.
    #[serde(rename = "onboarding")]
    pub onboarding: serde_json::Value,
    /// List of auth methods.
    #[serde(rename = "authMethods")]
    pub auth_methods: Vec<crate::models::ProjectAuthMethod>,
    /// List of services.
    #[serde(rename = "services")]
    pub services: Vec<crate::models::ProjectService>,
    /// List of protocols.
    #[serde(rename = "protocols")]
    pub protocols: Vec<crate::models::ProjectProtocol>,
    /// Project blocks information
    #[serde(rename = "blocks")]
    pub blocks: Vec<crate::models::Block>,
    /// Last time the project was accessed via console. Used with plan's
    /// projectInactivityDays to determine if project is paused.
    #[serde(rename = "consoleAccessedAt")]
    pub console_accessed_at: String,
    /// Whether WAF enforcement is enabled for the project.
    #[serde(rename = "wafEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_enabled: Option<bool>,
    /// Billing limits reached
    #[serde(rename = "billingLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_limits: Option<crate::models::BillingLimits>,
    /// OAuth2 server status
    #[serde(rename = "oAuth2ServerEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_enabled: Option<bool>,
    /// OAuth2 server authorization URL
    #[serde(rename = "oAuth2ServerAuthorizationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_authorization_url: Option<String>,
    /// OAuth2 server allowed scopes
    #[serde(rename = "oAuth2ServerScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_scopes: Option<Vec<String>>,
    /// OAuth2 server scopes used when an authorization request omits the scope
    /// parameter
    #[serde(rename = "oAuth2ServerDefaultScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_default_scopes: Option<Vec<String>>,
    /// Scopes an application may request when installed on a team
    #[serde(rename = "oAuth2ServerInstallationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_installation_scopes: Option<Vec<String>>,
    /// OAuth2 server accepted RFC 9396 authorization_details types
    #[serde(rename = "oAuth2ServerAuthorizationDetailsTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_authorization_details_types: Option<Vec<String>>,
    /// OAuth2 server access token duration in seconds for confidential clients
    #[serde(rename = "oAuth2ServerAccessTokenDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_access_token_duration: Option<i64>,
    /// OAuth2 server refresh token duration in seconds for confidential clients
    #[serde(rename = "oAuth2ServerRefreshTokenDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_refresh_token_duration: Option<i64>,
    /// OAuth2 server access token duration in seconds for public clients (SPAs,
    /// mobile, native)
    #[serde(rename = "oAuth2ServerPublicAccessTokenDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_public_access_token_duration: Option<i64>,
    /// OAuth2 server refresh token duration in seconds for public clients (SPAs,
    /// mobile, native)
    #[serde(rename = "oAuth2ServerPublicRefreshTokenDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_public_refresh_token_duration: Option<i64>,
    /// OAuth2 server access token duration in seconds for app installation access
    /// tokens
    #[serde(rename = "oAuth2ServerInstallationAccessTokenDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_installation_access_token_duration: Option<i64>,
    /// When enabled, PKCE is required for confidential clients (server-side flows
    /// using client_secret). PKCE is always required for public clients regardless
    /// of this setting.
    #[serde(rename = "oAuth2ServerConfidentialPkce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_confidential_pkce: Option<bool>,
    /// URL to your application page where users enter the device flow user code.
    /// Empty when the Device Authorization Grant is not configured.
    #[serde(rename = "oAuth2ServerVerificationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_verification_url: Option<String>,
    /// Number of characters in the device flow user code, excluding the formatting
    /// separator.
    #[serde(rename = "oAuth2ServerUserCodeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_user_code_length: Option<i64>,
    /// Character set for device flow user codes: `numeric`, `alphabetic`, or
    /// `alphanumeric`.
    #[serde(rename = "oAuth2ServerUserCodeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_user_code_format: Option<String>,
    /// Lifetime in seconds of device flow device codes and user codes.
    #[serde(rename = "oAuth2ServerDeviceCodeDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_device_code_duration: Option<i64>,
    /// OAuth2 server discovery URL
    #[serde(rename = "oAuth2ServerDiscoveryUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_server_discovery_url: Option<String>,
}

impl Project {
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

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get region
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Get dev_keys
    pub fn dev_keys(&self) -> &Vec<crate::models::DevKey> {
        &self.dev_keys
    }

    /// Get smtp_enabled
    pub fn smtp_enabled(&self) -> &bool {
        &self.smtp_enabled
    }

    /// Get smtp_sender_name
    pub fn smtp_sender_name(&self) -> &String {
        &self.smtp_sender_name
    }

    /// Get smtp_sender_email
    pub fn smtp_sender_email(&self) -> &String {
        &self.smtp_sender_email
    }

    /// Get smtp_reply_to_name
    pub fn smtp_reply_to_name(&self) -> &String {
        &self.smtp_reply_to_name
    }

    /// Get smtp_reply_to_email
    pub fn smtp_reply_to_email(&self) -> &String {
        &self.smtp_reply_to_email
    }

    /// Get smtp_host
    pub fn smtp_host(&self) -> &String {
        &self.smtp_host
    }

    /// Get smtp_port
    pub fn smtp_port(&self) -> &i64 {
        &self.smtp_port
    }

    /// Get smtp_username
    pub fn smtp_username(&self) -> &String {
        &self.smtp_username
    }

    /// Get smtp_password
    pub fn smtp_password(&self) -> &String {
        &self.smtp_password
    }

    /// Get smtp_secure
    pub fn smtp_secure(&self) -> &String {
        &self.smtp_secure
    }

    /// Get ping_count
    pub fn ping_count(&self) -> &i64 {
        &self.ping_count
    }

    /// Get pinged_at
    pub fn pinged_at(&self) -> &String {
        &self.pinged_at
    }

    /// Get labels
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get onboarding
    pub fn onboarding(&self) -> &serde_json::Value {
        &self.onboarding
    }

    /// Get auth_methods
    pub fn auth_methods(&self) -> &Vec<crate::models::ProjectAuthMethod> {
        &self.auth_methods
    }

    /// Get services
    pub fn services(&self) -> &Vec<crate::models::ProjectService> {
        &self.services
    }

    /// Get protocols
    pub fn protocols(&self) -> &Vec<crate::models::ProjectProtocol> {
        &self.protocols
    }

    /// Get blocks
    pub fn blocks(&self) -> &Vec<crate::models::Block> {
        &self.blocks
    }

    /// Get console_accessed_at
    pub fn console_accessed_at(&self) -> &String {
        &self.console_accessed_at
    }

    /// Set waf_enabled
    pub fn set_waf_enabled(mut self, waf_enabled: bool) -> Self {
        self.waf_enabled = Some(waf_enabled);
        self
    }

    /// Get waf_enabled
    pub fn waf_enabled(&self) -> Option<&bool> {
        self.waf_enabled.as_ref()
    }

    /// Set billing_limits
    pub fn set_billing_limits(mut self, billing_limits: crate::models::BillingLimits) -> Self {
        self.billing_limits = Some(billing_limits);
        self
    }

    /// Get billing_limits
    pub fn billing_limits(&self) -> Option<&crate::models::BillingLimits> {
        self.billing_limits.as_ref()
    }

    /// Set o_auth2_server_enabled
    pub fn set_o_auth2_server_enabled(mut self, o_auth2_server_enabled: bool) -> Self {
        self.o_auth2_server_enabled = Some(o_auth2_server_enabled);
        self
    }

    /// Get o_auth2_server_enabled
    pub fn o_auth2_server_enabled(&self) -> Option<&bool> {
        self.o_auth2_server_enabled.as_ref()
    }

    /// Set o_auth2_server_authorization_url
    pub fn set_o_auth2_server_authorization_url(
        mut self,
        o_auth2_server_authorization_url: String,
    ) -> Self {
        self.o_auth2_server_authorization_url = Some(o_auth2_server_authorization_url);
        self
    }

    /// Get o_auth2_server_authorization_url
    pub fn o_auth2_server_authorization_url(&self) -> Option<&String> {
        self.o_auth2_server_authorization_url.as_ref()
    }

    /// Set o_auth2_server_scopes
    pub fn set_o_auth2_server_scopes(mut self, o_auth2_server_scopes: Vec<String>) -> Self {
        self.o_auth2_server_scopes = Some(o_auth2_server_scopes);
        self
    }

    /// Get o_auth2_server_scopes
    pub fn o_auth2_server_scopes(&self) -> Option<&Vec<String>> {
        self.o_auth2_server_scopes.as_ref()
    }

    /// Set o_auth2_server_default_scopes
    pub fn set_o_auth2_server_default_scopes(
        mut self,
        o_auth2_server_default_scopes: Vec<String>,
    ) -> Self {
        self.o_auth2_server_default_scopes = Some(o_auth2_server_default_scopes);
        self
    }

    /// Get o_auth2_server_default_scopes
    pub fn o_auth2_server_default_scopes(&self) -> Option<&Vec<String>> {
        self.o_auth2_server_default_scopes.as_ref()
    }

    /// Set o_auth2_server_installation_scopes
    pub fn set_o_auth2_server_installation_scopes(
        mut self,
        o_auth2_server_installation_scopes: Vec<String>,
    ) -> Self {
        self.o_auth2_server_installation_scopes = Some(o_auth2_server_installation_scopes);
        self
    }

    /// Get o_auth2_server_installation_scopes
    pub fn o_auth2_server_installation_scopes(&self) -> Option<&Vec<String>> {
        self.o_auth2_server_installation_scopes.as_ref()
    }

    /// Set o_auth2_server_authorization_details_types
    pub fn set_o_auth2_server_authorization_details_types(
        mut self,
        o_auth2_server_authorization_details_types: Vec<String>,
    ) -> Self {
        self.o_auth2_server_authorization_details_types =
            Some(o_auth2_server_authorization_details_types);
        self
    }

    /// Get o_auth2_server_authorization_details_types
    pub fn o_auth2_server_authorization_details_types(&self) -> Option<&Vec<String>> {
        self.o_auth2_server_authorization_details_types.as_ref()
    }

    /// Set o_auth2_server_access_token_duration
    pub fn set_o_auth2_server_access_token_duration(
        mut self,
        o_auth2_server_access_token_duration: i64,
    ) -> Self {
        self.o_auth2_server_access_token_duration = Some(o_auth2_server_access_token_duration);
        self
    }

    /// Get o_auth2_server_access_token_duration
    pub fn o_auth2_server_access_token_duration(&self) -> Option<&i64> {
        self.o_auth2_server_access_token_duration.as_ref()
    }

    /// Set o_auth2_server_refresh_token_duration
    pub fn set_o_auth2_server_refresh_token_duration(
        mut self,
        o_auth2_server_refresh_token_duration: i64,
    ) -> Self {
        self.o_auth2_server_refresh_token_duration = Some(o_auth2_server_refresh_token_duration);
        self
    }

    /// Get o_auth2_server_refresh_token_duration
    pub fn o_auth2_server_refresh_token_duration(&self) -> Option<&i64> {
        self.o_auth2_server_refresh_token_duration.as_ref()
    }

    /// Set o_auth2_server_public_access_token_duration
    pub fn set_o_auth2_server_public_access_token_duration(
        mut self,
        o_auth2_server_public_access_token_duration: i64,
    ) -> Self {
        self.o_auth2_server_public_access_token_duration =
            Some(o_auth2_server_public_access_token_duration);
        self
    }

    /// Get o_auth2_server_public_access_token_duration
    pub fn o_auth2_server_public_access_token_duration(&self) -> Option<&i64> {
        self.o_auth2_server_public_access_token_duration.as_ref()
    }

    /// Set o_auth2_server_public_refresh_token_duration
    pub fn set_o_auth2_server_public_refresh_token_duration(
        mut self,
        o_auth2_server_public_refresh_token_duration: i64,
    ) -> Self {
        self.o_auth2_server_public_refresh_token_duration =
            Some(o_auth2_server_public_refresh_token_duration);
        self
    }

    /// Get o_auth2_server_public_refresh_token_duration
    pub fn o_auth2_server_public_refresh_token_duration(&self) -> Option<&i64> {
        self.o_auth2_server_public_refresh_token_duration.as_ref()
    }

    /// Set o_auth2_server_installation_access_token_duration
    pub fn set_o_auth2_server_installation_access_token_duration(
        mut self,
        o_auth2_server_installation_access_token_duration: i64,
    ) -> Self {
        self.o_auth2_server_installation_access_token_duration =
            Some(o_auth2_server_installation_access_token_duration);
        self
    }

    /// Get o_auth2_server_installation_access_token_duration
    pub fn o_auth2_server_installation_access_token_duration(&self) -> Option<&i64> {
        self.o_auth2_server_installation_access_token_duration
            .as_ref()
    }

    /// Set o_auth2_server_confidential_pkce
    pub fn set_o_auth2_server_confidential_pkce(
        mut self,
        o_auth2_server_confidential_pkce: bool,
    ) -> Self {
        self.o_auth2_server_confidential_pkce = Some(o_auth2_server_confidential_pkce);
        self
    }

    /// Get o_auth2_server_confidential_pkce
    pub fn o_auth2_server_confidential_pkce(&self) -> Option<&bool> {
        self.o_auth2_server_confidential_pkce.as_ref()
    }

    /// Set o_auth2_server_verification_url
    pub fn set_o_auth2_server_verification_url(
        mut self,
        o_auth2_server_verification_url: String,
    ) -> Self {
        self.o_auth2_server_verification_url = Some(o_auth2_server_verification_url);
        self
    }

    /// Get o_auth2_server_verification_url
    pub fn o_auth2_server_verification_url(&self) -> Option<&String> {
        self.o_auth2_server_verification_url.as_ref()
    }

    /// Set o_auth2_server_user_code_length
    pub fn set_o_auth2_server_user_code_length(
        mut self,
        o_auth2_server_user_code_length: i64,
    ) -> Self {
        self.o_auth2_server_user_code_length = Some(o_auth2_server_user_code_length);
        self
    }

    /// Get o_auth2_server_user_code_length
    pub fn o_auth2_server_user_code_length(&self) -> Option<&i64> {
        self.o_auth2_server_user_code_length.as_ref()
    }

    /// Set o_auth2_server_user_code_format
    pub fn set_o_auth2_server_user_code_format(
        mut self,
        o_auth2_server_user_code_format: String,
    ) -> Self {
        self.o_auth2_server_user_code_format = Some(o_auth2_server_user_code_format);
        self
    }

    /// Get o_auth2_server_user_code_format
    pub fn o_auth2_server_user_code_format(&self) -> Option<&String> {
        self.o_auth2_server_user_code_format.as_ref()
    }

    /// Set o_auth2_server_device_code_duration
    pub fn set_o_auth2_server_device_code_duration(
        mut self,
        o_auth2_server_device_code_duration: i64,
    ) -> Self {
        self.o_auth2_server_device_code_duration = Some(o_auth2_server_device_code_duration);
        self
    }

    /// Get o_auth2_server_device_code_duration
    pub fn o_auth2_server_device_code_duration(&self) -> Option<&i64> {
        self.o_auth2_server_device_code_duration.as_ref()
    }

    /// Set o_auth2_server_discovery_url
    pub fn set_o_auth2_server_discovery_url(
        mut self,
        o_auth2_server_discovery_url: String,
    ) -> Self {
        self.o_auth2_server_discovery_url = Some(o_auth2_server_discovery_url);
        self
    }

    /// Get o_auth2_server_discovery_url
    pub fn o_auth2_server_discovery_url(&self) -> Option<&String> {
        self.o_auth2_server_discovery_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let _model = <Project as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.team_id();
        let _ = _model.region();
        let _ = _model.dev_keys();
        let _ = _model.smtp_enabled();
        let _ = _model.smtp_sender_name();
        let _ = _model.smtp_sender_email();
        let _ = _model.smtp_reply_to_name();
        let _ = _model.smtp_reply_to_email();
        let _ = _model.smtp_host();
        let _ = _model.smtp_port();
        let _ = _model.smtp_username();
        let _ = _model.smtp_password();
        let _ = _model.smtp_secure();
        let _ = _model.ping_count();
        let _ = _model.pinged_at();
        let _ = _model.labels();
        let _ = _model.status();
        let _ = _model.onboarding();
        let _ = _model.auth_methods();
        let _ = _model.services();
        let _ = _model.protocols();
        let _ = _model.blocks();
        let _ = _model.console_accessed_at();
    }

    #[test]
    fn test_project_serialization() {
        let model = <Project as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Project, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
