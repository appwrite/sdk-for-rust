//! App model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// App
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct App {
    /// App ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// App creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// App update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Application name.
    #[serde(rename = "name")]
    pub name: String,
    /// Application description shown to users during OAuth2 consent.
    #[serde(rename = "description")]
    pub description: String,
    /// Application homepage URL shown to users during OAuth2 consent.
    #[serde(rename = "clientUri")]
    pub client_uri: String,
    /// Application logo URL shown to users during OAuth2 consent.
    #[serde(rename = "logoUri")]
    pub logo_uri: String,
    /// Application privacy policy URL shown to users during OAuth2 consent.
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: String,
    /// Application terms of service URL shown to users during OAuth2 consent.
    #[serde(rename = "termsUrl")]
    pub terms_url: String,
    /// Application support or security contact emails.
    #[serde(rename = "contacts")]
    pub contacts: Vec<String>,
    /// Application tagline shown to users during OAuth2 consent.
    #[serde(rename = "tagline")]
    pub tagline: String,
    /// Application tags shown to users during OAuth2 consent.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Application labels. Read-only for clients; only a server SDK using a
    /// project API key can update them.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// Application image URLs shown to users during OAuth2 consent.
    #[serde(rename = "images")]
    pub images: Vec<String>,
    /// Application support URL shown to users during OAuth2 consent.
    #[serde(rename = "supportUrl")]
    pub support_url: String,
    /// Application data deletion URL shown to users during OAuth2 consent.
    #[serde(rename = "dataDeletionUrl")]
    pub data_deletion_url: String,
    /// List of authorized redirect URIs. These URIs can be used to redirect users
    /// after they authenticate.
    #[serde(rename = "redirectUris")]
    pub redirect_uris: Vec<String>,
    /// List of authorized post-logout redirect URIs for OpenID Connect
    /// RP-Initiated Logout. The logout endpoint only redirects users to URIs in
    /// this list after ending their session.
    #[serde(rename = "postLogoutRedirectUris")]
    pub post_logout_redirect_uris: Vec<String>,
    /// Whether the app is enabled or not.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// OAuth2 client type. `public` for SPAs, mobile, and native apps that cannot
    /// keep a client secret (PKCE required); `confidential` for server-side
    /// clients that authenticate with a client secret.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Whether this client may use the OAuth2 Device Authorization Grant (RFC
    /// 8628).
    #[serde(rename = "deviceFlow")]
    pub device_flow: bool,
    /// ID of team that owns the application, if owned by team. Otherwise, user ID
    /// will be used.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// ID of user who owns the application, if owned by user. Otherwise, team ID
    /// will be used.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Scopes the application requests when installed on a team.
    /// Organization-level and project-level scopes only.
    #[serde(rename = "installationScopes")]
    pub installation_scopes: Vec<String>,
    /// URL users are redirected to after creating or updating an installation of
    /// this application. Empty for no redirect.
    #[serde(rename = "installationRedirectUrl")]
    pub installation_redirect_url: String,
    /// List of application secrets.
    #[serde(rename = "secrets")]
    pub secrets: Vec<crate::models::AppSecret>,
}

impl App {
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

    /// Get client_uri
    pub fn client_uri(&self) -> &String {
        &self.client_uri
    }

    /// Get logo_uri
    pub fn logo_uri(&self) -> &String {
        &self.logo_uri
    }

    /// Get privacy_policy_url
    pub fn privacy_policy_url(&self) -> &String {
        &self.privacy_policy_url
    }

    /// Get terms_url
    pub fn terms_url(&self) -> &String {
        &self.terms_url
    }

    /// Get contacts
    pub fn contacts(&self) -> &Vec<String> {
        &self.contacts
    }

    /// Get tagline
    pub fn tagline(&self) -> &String {
        &self.tagline
    }

    /// Get tags
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    /// Get labels
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Get images
    pub fn images(&self) -> &Vec<String> {
        &self.images
    }

    /// Get support_url
    pub fn support_url(&self) -> &String {
        &self.support_url
    }

    /// Get data_deletion_url
    pub fn data_deletion_url(&self) -> &String {
        &self.data_deletion_url
    }

    /// Get redirect_uris
    pub fn redirect_uris(&self) -> &Vec<String> {
        &self.redirect_uris
    }

    /// Get post_logout_redirect_uris
    pub fn post_logout_redirect_uris(&self) -> &Vec<String> {
        &self.post_logout_redirect_uris
    }

    /// Get enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Get r#type
    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    /// Get device_flow
    pub fn device_flow(&self) -> &bool {
        &self.device_flow
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get user_id
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get installation_scopes
    pub fn installation_scopes(&self) -> &Vec<String> {
        &self.installation_scopes
    }

    /// Get installation_redirect_url
    pub fn installation_redirect_url(&self) -> &String {
        &self.installation_redirect_url
    }

    /// Get secrets
    pub fn secrets(&self) -> &Vec<crate::models::AppSecret> {
        &self.secrets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let _model = <App as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.description();
        let _ = _model.client_uri();
        let _ = _model.logo_uri();
        let _ = _model.privacy_policy_url();
        let _ = _model.terms_url();
        let _ = _model.contacts();
        let _ = _model.tagline();
        let _ = _model.tags();
        let _ = _model.labels();
        let _ = _model.images();
        let _ = _model.support_url();
        let _ = _model.data_deletion_url();
        let _ = _model.redirect_uris();
        let _ = _model.post_logout_redirect_uris();
        let _ = _model.enabled();
        let _ = _model.r#type();
        let _ = _model.device_flow();
        let _ = _model.team_id();
        let _ = _model.user_id();
        let _ = _model.installation_scopes();
        let _ = _model.installation_redirect_url();
        let _ = _model.secrets();
    }

    #[test]
    fn test_app_serialization() {
        let model = <App as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<App, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
