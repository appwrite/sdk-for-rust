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
    /// Project description.
    #[serde(rename = "description")]
    pub description: String,
    /// Project team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// Project logo file ID.
    #[serde(rename = "logo")]
    pub logo: String,
    /// Project website URL.
    #[serde(rename = "url")]
    pub url: String,
    /// Company legal name.
    #[serde(rename = "legalName")]
    pub legal_name: String,
    /// Country code in [ISO 3166-1](http://en.wikipedia.org/wiki/ISO_3166-1)
    /// two-character format.
    #[serde(rename = "legalCountry")]
    pub legal_country: String,
    /// State name.
    #[serde(rename = "legalState")]
    pub legal_state: String,
    /// City name.
    #[serde(rename = "legalCity")]
    pub legal_city: String,
    /// Company Address.
    #[serde(rename = "legalAddress")]
    pub legal_address: String,
    /// Company Tax ID.
    #[serde(rename = "legalTaxId")]
    pub legal_tax_id: String,
    /// Session duration in seconds.
    #[serde(rename = "authDuration")]
    pub auth_duration: i64,
    /// Max users allowed. 0 is unlimited.
    #[serde(rename = "authLimit")]
    pub auth_limit: i64,
    /// Max sessions allowed per user. 100 maximum.
    #[serde(rename = "authSessionsLimit")]
    pub auth_sessions_limit: i64,
    /// Max allowed passwords in the history list per user. Max passwords limit
    /// allowed in history is 20. Use 0 for disabling password history.
    #[serde(rename = "authPasswordHistory")]
    pub auth_password_history: i64,
    /// Whether or not to check user's password against most commonly used
    /// passwords.
    #[serde(rename = "authPasswordDictionary")]
    pub auth_password_dictionary: bool,
    /// Whether or not to check the user password for similarity with their
    /// personal data.
    #[serde(rename = "authPersonalDataCheck")]
    pub auth_personal_data_check: bool,
    /// Whether or not to disallow disposable email addresses during signup and
    /// email updates.
    #[serde(rename = "authDisposableEmails")]
    pub auth_disposable_emails: bool,
    /// Whether or not to require canonical email addresses during signup and email
    /// updates.
    #[serde(rename = "authCanonicalEmails")]
    pub auth_canonical_emails: bool,
    /// Whether or not to disallow free email addresses during signup and email
    /// updates.
    #[serde(rename = "authFreeEmails")]
    pub auth_free_emails: bool,
    /// An array of mock numbers and their corresponding verification codes (OTPs).
    #[serde(rename = "authMockNumbers")]
    pub auth_mock_numbers: Vec<crate::models::MockNumber>,
    /// Whether or not to send session alert emails to users.
    #[serde(rename = "authSessionAlerts")]
    pub auth_session_alerts: bool,
    /// Whether or not to show user names in the teams membership response.
    #[serde(rename = "authMembershipsUserName")]
    pub auth_memberships_user_name: bool,
    /// Whether or not to show user emails in the teams membership response.
    #[serde(rename = "authMembershipsUserEmail")]
    pub auth_memberships_user_email: bool,
    /// Whether or not to show user MFA status in the teams membership response.
    #[serde(rename = "authMembershipsMfa")]
    pub auth_memberships_mfa: bool,
    /// Whether or not all existing sessions should be invalidated on password
    /// change
    #[serde(rename = "authInvalidateSessions")]
    pub auth_invalidate_sessions: bool,
    /// List of Auth Providers.
    #[serde(rename = "oAuthProviders")]
    pub o_auth_providers: Vec<crate::models::AuthProvider>,
    /// List of Platforms.
    #[serde(rename = "platforms")]
    pub platforms: Vec<serde_json::Value>,
    /// List of Webhooks.
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<crate::models::Webhook>,
    /// List of API Keys.
    #[serde(rename = "keys")]
    pub keys: Vec<crate::models::Key>,
    /// List of dev keys.
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
    /// SMTP reply to email
    #[serde(rename = "smtpReplyTo")]
    pub smtp_reply_to: String,
    /// SMTP server host name
    #[serde(rename = "smtpHost")]
    pub smtp_host: String,
    /// SMTP server port
    #[serde(rename = "smtpPort")]
    pub smtp_port: i64,
    /// SMTP server username
    #[serde(rename = "smtpUsername")]
    pub smtp_username: String,
    /// SMTP server password
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
    /// Email/Password auth method status
    #[serde(rename = "authEmailPassword")]
    pub auth_email_password: bool,
    /// Magic URL auth method status
    #[serde(rename = "authUsersAuthMagicURL")]
    pub auth_users_auth_magic_url: bool,
    /// Email (OTP) auth method status
    #[serde(rename = "authEmailOtp")]
    pub auth_email_otp: bool,
    /// Anonymous auth method status
    #[serde(rename = "authAnonymous")]
    pub auth_anonymous: bool,
    /// Invites auth method status
    #[serde(rename = "authInvites")]
    pub auth_invites: bool,
    /// JWT auth method status
    #[serde(rename = "authJWT")]
    pub auth_jwt: bool,
    /// Phone auth method status
    #[serde(rename = "authPhone")]
    pub auth_phone: bool,
    /// Account service status
    #[serde(rename = "serviceStatusForAccount")]
    pub service_status_for_account: bool,
    /// Avatars service status
    #[serde(rename = "serviceStatusForAvatars")]
    pub service_status_for_avatars: bool,
    /// Databases (legacy) service status
    #[serde(rename = "serviceStatusForDatabases")]
    pub service_status_for_databases: bool,
    /// TablesDB service status
    #[serde(rename = "serviceStatusForTablesdb")]
    pub service_status_for_tablesdb: bool,
    /// Locale service status
    #[serde(rename = "serviceStatusForLocale")]
    pub service_status_for_locale: bool,
    /// Health service status
    #[serde(rename = "serviceStatusForHealth")]
    pub service_status_for_health: bool,
    /// Project service status
    #[serde(rename = "serviceStatusForProject")]
    pub service_status_for_project: bool,
    /// Storage service status
    #[serde(rename = "serviceStatusForStorage")]
    pub service_status_for_storage: bool,
    /// Teams service status
    #[serde(rename = "serviceStatusForTeams")]
    pub service_status_for_teams: bool,
    /// Users service status
    #[serde(rename = "serviceStatusForUsers")]
    pub service_status_for_users: bool,
    /// VCS service status
    #[serde(rename = "serviceStatusForVcs")]
    pub service_status_for_vcs: bool,
    /// Sites service status
    #[serde(rename = "serviceStatusForSites")]
    pub service_status_for_sites: bool,
    /// Functions service status
    #[serde(rename = "serviceStatusForFunctions")]
    pub service_status_for_functions: bool,
    /// Proxy service status
    #[serde(rename = "serviceStatusForProxy")]
    pub service_status_for_proxy: bool,
    /// GraphQL service status
    #[serde(rename = "serviceStatusForGraphql")]
    pub service_status_for_graphql: bool,
    /// Migrations service status
    #[serde(rename = "serviceStatusForMigrations")]
    pub service_status_for_migrations: bool,
    /// Messaging service status
    #[serde(rename = "serviceStatusForMessaging")]
    pub service_status_for_messaging: bool,
    /// REST protocol status
    #[serde(rename = "protocolStatusForRest")]
    pub protocol_status_for_rest: bool,
    /// GraphQL protocol status
    #[serde(rename = "protocolStatusForGraphql")]
    pub protocol_status_for_graphql: bool,
    /// Websocket protocol status
    #[serde(rename = "protocolStatusForWebsocket")]
    pub protocol_status_for_websocket: bool,
    /// Project region
    #[serde(rename = "region")]
    pub region: String,
    /// Billing limits reached
    #[serde(rename = "billingLimits")]
    pub billing_limits: crate::models::BillingLimits,
    /// Project blocks information
    #[serde(rename = "blocks")]
    pub blocks: Vec<crate::models::Block>,
    /// Last time the project was accessed via console. Used with plan's
    /// projectInactivityDays to determine if project is paused.
    #[serde(rename = "consoleAccessedAt")]
    pub console_accessed_at: String,
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

    /// Get description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get logo
    pub fn logo(&self) -> &String {
        &self.logo
    }

    /// Get url
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Get legal_name
    pub fn legal_name(&self) -> &String {
        &self.legal_name
    }

    /// Get legal_country
    pub fn legal_country(&self) -> &String {
        &self.legal_country
    }

    /// Get legal_state
    pub fn legal_state(&self) -> &String {
        &self.legal_state
    }

    /// Get legal_city
    pub fn legal_city(&self) -> &String {
        &self.legal_city
    }

    /// Get legal_address
    pub fn legal_address(&self) -> &String {
        &self.legal_address
    }

    /// Get legal_tax_id
    pub fn legal_tax_id(&self) -> &String {
        &self.legal_tax_id
    }

    /// Get auth_duration
    pub fn auth_duration(&self) -> &i64 {
        &self.auth_duration
    }

    /// Get auth_limit
    pub fn auth_limit(&self) -> &i64 {
        &self.auth_limit
    }

    /// Get auth_sessions_limit
    pub fn auth_sessions_limit(&self) -> &i64 {
        &self.auth_sessions_limit
    }

    /// Get auth_password_history
    pub fn auth_password_history(&self) -> &i64 {
        &self.auth_password_history
    }

    /// Get auth_password_dictionary
    pub fn auth_password_dictionary(&self) -> &bool {
        &self.auth_password_dictionary
    }

    /// Get auth_personal_data_check
    pub fn auth_personal_data_check(&self) -> &bool {
        &self.auth_personal_data_check
    }

    /// Get auth_disposable_emails
    pub fn auth_disposable_emails(&self) -> &bool {
        &self.auth_disposable_emails
    }

    /// Get auth_canonical_emails
    pub fn auth_canonical_emails(&self) -> &bool {
        &self.auth_canonical_emails
    }

    /// Get auth_free_emails
    pub fn auth_free_emails(&self) -> &bool {
        &self.auth_free_emails
    }

    /// Get auth_mock_numbers
    pub fn auth_mock_numbers(&self) -> &Vec<crate::models::MockNumber> {
        &self.auth_mock_numbers
    }

    /// Get auth_session_alerts
    pub fn auth_session_alerts(&self) -> &bool {
        &self.auth_session_alerts
    }

    /// Get auth_memberships_user_name
    pub fn auth_memberships_user_name(&self) -> &bool {
        &self.auth_memberships_user_name
    }

    /// Get auth_memberships_user_email
    pub fn auth_memberships_user_email(&self) -> &bool {
        &self.auth_memberships_user_email
    }

    /// Get auth_memberships_mfa
    pub fn auth_memberships_mfa(&self) -> &bool {
        &self.auth_memberships_mfa
    }

    /// Get auth_invalidate_sessions
    pub fn auth_invalidate_sessions(&self) -> &bool {
        &self.auth_invalidate_sessions
    }

    /// Get o_auth_providers
    pub fn o_auth_providers(&self) -> &Vec<crate::models::AuthProvider> {
        &self.o_auth_providers
    }

    /// Get platforms
    pub fn platforms(&self) -> &Vec<serde_json::Value> {
        &self.platforms
    }

    /// Get webhooks
    pub fn webhooks(&self) -> &Vec<crate::models::Webhook> {
        &self.webhooks
    }

    /// Get keys
    pub fn keys(&self) -> &Vec<crate::models::Key> {
        &self.keys
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

    /// Get smtp_reply_to
    pub fn smtp_reply_to(&self) -> &String {
        &self.smtp_reply_to
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

    /// Get auth_email_password
    pub fn auth_email_password(&self) -> &bool {
        &self.auth_email_password
    }

    /// Get auth_users_auth_magic_url
    pub fn auth_users_auth_magic_url(&self) -> &bool {
        &self.auth_users_auth_magic_url
    }

    /// Get auth_email_otp
    pub fn auth_email_otp(&self) -> &bool {
        &self.auth_email_otp
    }

    /// Get auth_anonymous
    pub fn auth_anonymous(&self) -> &bool {
        &self.auth_anonymous
    }

    /// Get auth_invites
    pub fn auth_invites(&self) -> &bool {
        &self.auth_invites
    }

    /// Get auth_jwt
    pub fn auth_jwt(&self) -> &bool {
        &self.auth_jwt
    }

    /// Get auth_phone
    pub fn auth_phone(&self) -> &bool {
        &self.auth_phone
    }

    /// Get service_status_for_account
    pub fn service_status_for_account(&self) -> &bool {
        &self.service_status_for_account
    }

    /// Get service_status_for_avatars
    pub fn service_status_for_avatars(&self) -> &bool {
        &self.service_status_for_avatars
    }

    /// Get service_status_for_databases
    pub fn service_status_for_databases(&self) -> &bool {
        &self.service_status_for_databases
    }

    /// Get service_status_for_tablesdb
    pub fn service_status_for_tablesdb(&self) -> &bool {
        &self.service_status_for_tablesdb
    }

    /// Get service_status_for_locale
    pub fn service_status_for_locale(&self) -> &bool {
        &self.service_status_for_locale
    }

    /// Get service_status_for_health
    pub fn service_status_for_health(&self) -> &bool {
        &self.service_status_for_health
    }

    /// Get service_status_for_project
    pub fn service_status_for_project(&self) -> &bool {
        &self.service_status_for_project
    }

    /// Get service_status_for_storage
    pub fn service_status_for_storage(&self) -> &bool {
        &self.service_status_for_storage
    }

    /// Get service_status_for_teams
    pub fn service_status_for_teams(&self) -> &bool {
        &self.service_status_for_teams
    }

    /// Get service_status_for_users
    pub fn service_status_for_users(&self) -> &bool {
        &self.service_status_for_users
    }

    /// Get service_status_for_vcs
    pub fn service_status_for_vcs(&self) -> &bool {
        &self.service_status_for_vcs
    }

    /// Get service_status_for_sites
    pub fn service_status_for_sites(&self) -> &bool {
        &self.service_status_for_sites
    }

    /// Get service_status_for_functions
    pub fn service_status_for_functions(&self) -> &bool {
        &self.service_status_for_functions
    }

    /// Get service_status_for_proxy
    pub fn service_status_for_proxy(&self) -> &bool {
        &self.service_status_for_proxy
    }

    /// Get service_status_for_graphql
    pub fn service_status_for_graphql(&self) -> &bool {
        &self.service_status_for_graphql
    }

    /// Get service_status_for_migrations
    pub fn service_status_for_migrations(&self) -> &bool {
        &self.service_status_for_migrations
    }

    /// Get service_status_for_messaging
    pub fn service_status_for_messaging(&self) -> &bool {
        &self.service_status_for_messaging
    }

    /// Get protocol_status_for_rest
    pub fn protocol_status_for_rest(&self) -> &bool {
        &self.protocol_status_for_rest
    }

    /// Get protocol_status_for_graphql
    pub fn protocol_status_for_graphql(&self) -> &bool {
        &self.protocol_status_for_graphql
    }

    /// Get protocol_status_for_websocket
    pub fn protocol_status_for_websocket(&self) -> &bool {
        &self.protocol_status_for_websocket
    }

    /// Get region
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Get billing_limits
    pub fn billing_limits(&self) -> &crate::models::BillingLimits {
        &self.billing_limits
    }

    /// Get blocks
    pub fn blocks(&self) -> &Vec<crate::models::Block> {
        &self.blocks
    }

    /// Get console_accessed_at
    pub fn console_accessed_at(&self) -> &String {
        &self.console_accessed_at
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
        let _ = _model.description();
        let _ = _model.team_id();
        let _ = _model.logo();
        let _ = _model.url();
        let _ = _model.legal_name();
        let _ = _model.legal_country();
        let _ = _model.legal_state();
        let _ = _model.legal_city();
        let _ = _model.legal_address();
        let _ = _model.legal_tax_id();
        let _ = _model.auth_duration();
        let _ = _model.auth_limit();
        let _ = _model.auth_sessions_limit();
        let _ = _model.auth_password_history();
        let _ = _model.auth_password_dictionary();
        let _ = _model.auth_personal_data_check();
        let _ = _model.auth_disposable_emails();
        let _ = _model.auth_canonical_emails();
        let _ = _model.auth_free_emails();
        let _ = _model.auth_mock_numbers();
        let _ = _model.auth_session_alerts();
        let _ = _model.auth_memberships_user_name();
        let _ = _model.auth_memberships_user_email();
        let _ = _model.auth_memberships_mfa();
        let _ = _model.auth_invalidate_sessions();
        let _ = _model.o_auth_providers();
        let _ = _model.platforms();
        let _ = _model.webhooks();
        let _ = _model.keys();
        let _ = _model.dev_keys();
        let _ = _model.smtp_enabled();
        let _ = _model.smtp_sender_name();
        let _ = _model.smtp_sender_email();
        let _ = _model.smtp_reply_to();
        let _ = _model.smtp_host();
        let _ = _model.smtp_port();
        let _ = _model.smtp_username();
        let _ = _model.smtp_password();
        let _ = _model.smtp_secure();
        let _ = _model.ping_count();
        let _ = _model.pinged_at();
        let _ = _model.labels();
        let _ = _model.status();
        let _ = _model.auth_email_password();
        let _ = _model.auth_users_auth_magic_url();
        let _ = _model.auth_email_otp();
        let _ = _model.auth_anonymous();
        let _ = _model.auth_invites();
        let _ = _model.auth_jwt();
        let _ = _model.auth_phone();
        let _ = _model.service_status_for_account();
        let _ = _model.service_status_for_avatars();
        let _ = _model.service_status_for_databases();
        let _ = _model.service_status_for_tablesdb();
        let _ = _model.service_status_for_locale();
        let _ = _model.service_status_for_health();
        let _ = _model.service_status_for_project();
        let _ = _model.service_status_for_storage();
        let _ = _model.service_status_for_teams();
        let _ = _model.service_status_for_users();
        let _ = _model.service_status_for_vcs();
        let _ = _model.service_status_for_sites();
        let _ = _model.service_status_for_functions();
        let _ = _model.service_status_for_proxy();
        let _ = _model.service_status_for_graphql();
        let _ = _model.service_status_for_migrations();
        let _ = _model.service_status_for_messaging();
        let _ = _model.protocol_status_for_rest();
        let _ = _model.protocol_status_for_graphql();
        let _ = _model.protocol_status_for_websocket();
        let _ = _model.region();
        let _ = _model.billing_limits();
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
