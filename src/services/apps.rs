//! Apps service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Apps service allows you to manage OAuth2 applications, their keys,
/// secrets, scopes, and installations.
#[derive(Debug, Clone)]
pub struct Apps {
    client: Client,
}

impl Apps {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List applications.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AppsList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new application.
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        app_id: impl Into<String>,
        name: impl Into<String>,
        redirect_uris: impl IntoIterator<Item = impl Into<String>>,
        description: Option<&str>,
        client_uri: Option<&str>,
        logo_uri: Option<&str>,
        privacy_policy_url: Option<&str>,
        terms_url: Option<&str>,
        contacts: Option<Vec<String>>,
        tagline: Option<&str>,
        tags: Option<Vec<String>>,
        images: Option<Vec<String>>,
        support_url: Option<&str>,
        data_deletion_url: Option<&str>,
        post_logout_redirect_uris: Option<Vec<String>>,
        enabled: Option<bool>,
        r#type: Option<&str>,
        device_flow: Option<bool>,
        team_id: Option<&str>,
    ) -> crate::error::Result<crate::models::App> {
        let mut params = HashMap::new();
        params.insert("appId".to_string(), json!(app_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = client_uri {
            params.insert("clientUri".to_string(), json!(value));
        }
        if let Some(value) = logo_uri {
            params.insert("logoUri".to_string(), json!(value));
        }
        if let Some(value) = privacy_policy_url {
            params.insert("privacyPolicyUrl".to_string(), json!(value));
        }
        if let Some(value) = terms_url {
            params.insert("termsUrl".to_string(), json!(value));
        }
        if let Some(value) = contacts {
            params.insert("contacts".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = tagline {
            params.insert("tagline".to_string(), json!(value));
        }
        if let Some(value) = tags {
            params.insert("tags".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = images {
            params.insert("images".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = support_url {
            params.insert("supportUrl".to_string(), json!(value));
        }
        if let Some(value) = data_deletion_url {
            params.insert("dataDeletionUrl".to_string(), json!(value));
        }
        params.insert("redirectUris".to_string(), json!(redirect_uris.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = post_logout_redirect_uris {
            params.insert("postLogoutRedirectUris".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        if let Some(value) = device_flow {
            params.insert("deviceFlow".to_string(), json!(value));
        }
        if let Some(value) = team_id {
            params.insert("teamId".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// List scopes an application can request when installed on a team.
    pub async fn list_installation_scopes(
        &self,
    ) -> crate::error::Result<crate::models::AppScopeList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/scopes/installations".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// List scopes an application can request during the OAuth2 flow.
    pub async fn list_o_auth2_scopes(
        &self,
    ) -> crate::error::Result<crate::models::AppScopeList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/scopes/oauth2".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Get an application by its unique ID.
    pub async fn get(
        &self,
        app_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::App> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Update an application by its unique ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        app_id: impl Into<String>,
        name: impl Into<String>,
        description: Option<&str>,
        client_uri: Option<&str>,
        logo_uri: Option<&str>,
        privacy_policy_url: Option<&str>,
        terms_url: Option<&str>,
        contacts: Option<Vec<String>>,
        tagline: Option<&str>,
        tags: Option<Vec<String>>,
        images: Option<Vec<String>>,
        support_url: Option<&str>,
        data_deletion_url: Option<&str>,
        enabled: Option<bool>,
        redirect_uris: Option<Vec<String>>,
        post_logout_redirect_uris: Option<Vec<String>>,
        r#type: Option<&str>,
        device_flow: Option<bool>,
        installation_scopes: Option<Vec<String>>,
        installation_redirect_url: Option<&str>,
    ) -> crate::error::Result<crate::models::App> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = client_uri {
            params.insert("clientUri".to_string(), json!(value));
        }
        if let Some(value) = logo_uri {
            params.insert("logoUri".to_string(), json!(value));
        }
        if let Some(value) = privacy_policy_url {
            params.insert("privacyPolicyUrl".to_string(), json!(value));
        }
        if let Some(value) = terms_url {
            params.insert("termsUrl".to_string(), json!(value));
        }
        if let Some(value) = contacts {
            params.insert("contacts".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = tagline {
            params.insert("tagline".to_string(), json!(value));
        }
        if let Some(value) = tags {
            params.insert("tags".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = images {
            params.insert("images".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = support_url {
            params.insert("supportUrl".to_string(), json!(value));
        }
        if let Some(value) = data_deletion_url {
            params.insert("dataDeletionUrl".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = redirect_uris {
            params.insert("redirectUris".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = post_logout_redirect_uris {
            params.insert("postLogoutRedirectUris".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        if let Some(value) = device_flow {
            params.insert("deviceFlow".to_string(), json!(value));
        }
        if let Some(value) = installation_scopes {
            params.insert("installationScopes".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = installation_redirect_url {
            params.insert("installationRedirectUrl".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an application by its unique ID.
    pub async fn delete(
        &self,
        app_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// List installations of an application. Requires an app key sent in the
    /// `X-Appwrite-Key` header alongside the `X-Appwrite-App` header, or a caller
    /// with update access to the app.
    pub async fn list_installations(
        &self,
        app_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AppInstallationList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/installations".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Get an installation of an application by its unique ID. Requires an app key
    /// sent in the `X-Appwrite-Key` header alongside the `X-Appwrite-App` header,
    /// or a caller with update access to the app.
    pub async fn get_installation(
        &self,
        app_id: impl Into<String>,
        installation_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AppInstallation> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/installations/{installationId}".to_string().replace("{appId}", &app_id.into().to_string()).replace("{installationId}", &installation_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an installation of an application by its unique ID. Requires a
    /// caller with update access to the app. Previously issued installation access
    /// tokens are revoked.
    pub async fn delete_installation(
        &self,
        app_id: impl Into<String>,
        installation_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/installations/{installationId}".to_string().replace("{appId}", &app_id.into().to_string()).replace("{installationId}", &installation_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Create a token for an installation of an application. Requires an app key
    /// sent in the `X-Appwrite-Key` header alongside the `X-Appwrite-App` header,
    /// or a caller with update access to the app. The returned token carries the
    /// scopes and authorization details granted to the installation, and can be
    /// used as an `Authorization: Bearer` header everywhere OAuth2 access tokens
    /// are accepted. Multiple tokens can be active for the same installation at
    /// once; each token stays valid until it expires or the installation is
    /// updated or deleted.
    pub async fn create_installation_token(
        &self,
        app_id: impl Into<String>,
        installation_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Oauth2Token> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/installations/{installationId}/tokens".to_string().replace("{appId}", &app_id.into().to_string()).replace("{installationId}", &installation_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// List app keys for an application.
    pub async fn list_keys(
        &self,
        app_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AppKeyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/keys".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new app key for an application. App keys carry no scopes; send one
    /// in the `X-Appwrite-Key` header alongside the `X-Appwrite-App` header to
    /// list the application's installations and create installation access tokens.
    pub async fn create_key(
        &self,
        app_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AppKey> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/keys".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get an app key by its unique ID.
    pub async fn get_key(
        &self,
        app_id: impl Into<String>,
        key_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AppKey> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/keys/{keyId}".to_string().replace("{appId}", &app_id.into().to_string()).replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an app key by its unique ID.
    pub async fn delete_key(
        &self,
        app_id: impl Into<String>,
        key_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/keys/{keyId}".to_string().replace("{appId}", &app_id.into().to_string()).replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the labels of an application. Labels are read-only for clients; only
    /// a server SDK using a project API key can set them. Replaces the previous
    /// labels.
    pub async fn update_labels(
        &self,
        app_id: impl Into<String>,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<crate::models::App> {
        let mut params = HashMap::new();
        params.insert("labels".to_string(), json!(labels.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/labels".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// List client secrets for an application.
    pub async fn list_secrets(
        &self,
        app_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AppSecretList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/secrets".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new client secret for an application.
    pub async fn create_secret(
        &self,
        app_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AppSecretPlaintext> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/secrets".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get an application client secret by its unique ID.
    pub async fn get_secret(
        &self,
        app_id: impl Into<String>,
        secret_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AppSecret> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/secrets/{secretId}".to_string().replace("{appId}", &app_id.into().to_string()).replace("{secretId}", &secret_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an application client secret by its unique ID.
    pub async fn delete_secret(
        &self,
        app_id: impl Into<String>,
        secret_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/secrets/{secretId}".to_string().replace("{appId}", &app_id.into().to_string()).replace("{secretId}", &secret_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Transfer an application to another team by its unique ID.
    pub async fn update_team(
        &self,
        app_id: impl Into<String>,
        team_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::App> {
        let mut params = HashMap::new();
        params.insert("teamId".to_string(), json!(team_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/team".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Revoke all tokens for an application by its unique ID.
    pub async fn delete_tokens(
        &self,
        app_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/apps/{appId}/tokens".to_string().replace("{appId}", &app_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Apps {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apps_creation() {
        let client = Client::new();
        let service = Apps::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
