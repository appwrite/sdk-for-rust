//! Oauth2 service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The OAuth2 service allows you to authorize apps and issue standards-based
/// OAuth2 and OpenID Connect tokens.
#[derive(Debug, Clone)]
pub struct Oauth2 {
    client: Client,
}

impl Oauth2 {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Approve an OAuth2 grant after the user gives consent. Returns the
    /// `redirectUrl` the end user should be sent to. The consent screen may
    /// optionally pass enriched `authorization_details` to record the concrete
    /// resources the user selected. You can pass Accept header of
    /// `application/json` to receive a JSON response instead of a redirect.
    pub async fn approve(
        &self,
        grant_id: impl Into<String>,
        authorization_details: Option<&str>,
        scope: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2Approve> {
        let mut params = HashMap::new();
        params.insert("grant_id".to_string(), json!(grant_id.into()));
        if let Some(value) = authorization_details {
            params.insert("authorization_details".to_string(), json!(value));
        }
        if let Some(value) = scope {
            params.insert("scope".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/approve".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Begin the OAuth2 authorization flow. When called without a session, the
    /// user is redirected to the consent screen without grant ID. When called with
    /// a session, the redirect URL includes param for grant ID. You can pass
    /// Accept header of `application/json` to receive a JSON response instead of a
    /// redirect.
    #[allow(clippy::too_many_arguments)]
    pub async fn authorize(
        &self,
        client_id: Option<&str>,
        redirect_uri: Option<&str>,
        response_type: Option<&str>,
        scope: Option<&str>,
        state: Option<&str>,
        nonce: Option<&str>,
        code_challenge: Option<&str>,
        code_challenge_method: Option<&str>,
        prompt: Option<&str>,
        max_age: Option<i64>,
        authorization_details: Option<&str>,
        resource: Option<&str>,
        audience: Option<&str>,
        request_uri: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2Authorize> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("client_id".to_string(), json!(value));
        }
        if let Some(value) = redirect_uri {
            params.insert("redirect_uri".to_string(), json!(value));
        }
        if let Some(value) = response_type {
            params.insert("response_type".to_string(), json!(value));
        }
        if let Some(value) = scope {
            params.insert("scope".to_string(), json!(value));
        }
        if let Some(value) = state {
            params.insert("state".to_string(), json!(value));
        }
        if let Some(value) = nonce {
            params.insert("nonce".to_string(), json!(value));
        }
        if let Some(value) = code_challenge {
            params.insert("code_challenge".to_string(), json!(value));
        }
        if let Some(value) = code_challenge_method {
            params.insert("code_challenge_method".to_string(), json!(value));
        }
        if let Some(value) = prompt {
            params.insert("prompt".to_string(), json!(value));
        }
        if let Some(value) = max_age {
            params.insert("max_age".to_string(), json!(value));
        }
        if let Some(value) = authorization_details {
            params.insert("authorization_details".to_string(), json!(value));
        }
        if let Some(value) = resource {
            params.insert("resource".to_string(), json!(value));
        }
        if let Some(value) = audience {
            params.insert("audience".to_string(), json!(value));
        }
        if let Some(value) = request_uri {
            params.insert("request_uri".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/authorize".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Begin the OAuth2 authorization flow. When called without a session, the
    /// user is redirected to the consent screen without grant ID. When called with
    /// a session, the redirect URL includes param for grant ID. You can pass
    /// Accept header of `application/json` to receive a JSON response instead of a
    /// redirect.
    #[allow(clippy::too_many_arguments)]
    pub async fn authorize_post(
        &self,
        client_id: Option<&str>,
        redirect_uri: Option<&str>,
        response_type: Option<&str>,
        scope: Option<&str>,
        state: Option<&str>,
        nonce: Option<&str>,
        code_challenge: Option<&str>,
        code_challenge_method: Option<&str>,
        prompt: Option<&str>,
        max_age: Option<i64>,
        authorization_details: Option<&str>,
        resource: Option<&str>,
        audience: Option<&str>,
        request_uri: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2Authorize> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("client_id".to_string(), json!(value));
        }
        if let Some(value) = redirect_uri {
            params.insert("redirect_uri".to_string(), json!(value));
        }
        if let Some(value) = response_type {
            params.insert("response_type".to_string(), json!(value));
        }
        if let Some(value) = scope {
            params.insert("scope".to_string(), json!(value));
        }
        if let Some(value) = state {
            params.insert("state".to_string(), json!(value));
        }
        if let Some(value) = nonce {
            params.insert("nonce".to_string(), json!(value));
        }
        if let Some(value) = code_challenge {
            params.insert("code_challenge".to_string(), json!(value));
        }
        if let Some(value) = code_challenge_method {
            params.insert("code_challenge_method".to_string(), json!(value));
        }
        if let Some(value) = prompt {
            params.insert("prompt".to_string(), json!(value));
        }
        if let Some(value) = max_age {
            params.insert("max_age".to_string(), json!(value));
        }
        if let Some(value) = authorization_details {
            params.insert("authorization_details".to_string(), json!(value));
        }
        if let Some(value) = resource {
            params.insert("resource".to_string(), json!(value));
        }
        if let Some(value) = audience {
            params.insert("audience".to_string(), json!(value));
        }
        if let Some(value) = request_uri {
            params.insert("request_uri".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/authorize".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Start the OAuth2 Device Authorization Grant. Returns the device code, user
    /// code, verification URL, expiration, and polling interval.
    pub async fn create_device_authorization(
        &self,
        client_id: Option<&str>,
        scope: Option<&str>,
        authorization_details: Option<&str>,
        resource: Option<&str>,
        audience: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2DeviceAuthorization> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("client_id".to_string(), json!(value));
        }
        if let Some(value) = scope {
            params.insert("scope".to_string(), json!(value));
        }
        if let Some(value) = authorization_details {
            params.insert("authorization_details".to_string(), json!(value));
        }
        if let Some(value) = resource {
            params.insert("resource".to_string(), json!(value));
        }
        if let Some(value) = audience {
            params.insert("audience".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/device_authorization".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Exchange a device flow user code for an OAuth2 grant. The authenticated
    /// user is bound to the pending grant. Pass the returned grant ID to the get
    /// grant endpoint to render the consent screen, then to the approve or reject
    /// endpoint to complete the flow.
    pub async fn create_grant(
        &self,
        user_code: impl Into<String>,
    ) -> crate::error::Result<crate::models::Oauth2Grant> {
        let mut params = HashMap::new();
        params.insert("user_code".to_string(), json!(user_code.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/grants".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get an OAuth2 grant by its ID. Used by the consent screen to display the
    /// details of the authorization the user is being asked to approve. A grant
    /// can only be read by the user it belongs to, or by server SDK.
    pub async fn get_grant(
        &self,
        grant_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Oauth2Grant> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/grants/{grant_id}".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default()).replace("{grant_id}", &grant_id.into().to_string());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// List the organizations the OAuth2 access token can access. Resolves the
    /// token's `organization` authorization details, expanding the `*` wildcard
    /// into the concrete set of organizations the user can see.
    pub async fn list_organizations(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        search: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2OrganizationList> {
        let mut params = HashMap::new();
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/organizations".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Store an OAuth2 authorization request server-side and receive a short-lived
    /// request_uri handle for the authorize endpoint.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_par(
        &self,
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
        response_type: impl Into<String>,
        scope: Option<&str>,
        state: Option<&str>,
        nonce: Option<&str>,
        code_challenge: Option<&str>,
        code_challenge_method: Option<&str>,
        prompt: Option<&str>,
        max_age: Option<i64>,
        authorization_details: Option<&str>,
        resource: Option<&str>,
        audience: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2PAR> {
        let mut params = HashMap::new();
        params.insert("client_id".to_string(), json!(client_id.into()));
        params.insert("redirect_uri".to_string(), json!(redirect_uri.into()));
        params.insert("response_type".to_string(), json!(response_type.into()));
        if let Some(value) = scope {
            params.insert("scope".to_string(), json!(value));
        }
        if let Some(value) = state {
            params.insert("state".to_string(), json!(value));
        }
        if let Some(value) = nonce {
            params.insert("nonce".to_string(), json!(value));
        }
        if let Some(value) = code_challenge {
            params.insert("code_challenge".to_string(), json!(value));
        }
        if let Some(value) = code_challenge_method {
            params.insert("code_challenge_method".to_string(), json!(value));
        }
        if let Some(value) = prompt {
            params.insert("prompt".to_string(), json!(value));
        }
        if let Some(value) = max_age {
            params.insert("max_age".to_string(), json!(value));
        }
        if let Some(value) = authorization_details {
            params.insert("authorization_details".to_string(), json!(value));
        }
        if let Some(value) = resource {
            params.insert("resource".to_string(), json!(value));
        }
        if let Some(value) = audience {
            params.insert("audience".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/par".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// List the projects the OAuth2 access token can access. Resolves the token's
    /// `project` authorization details, expanding the `*` wildcard into the
    /// concrete set of projects the user can see.
    pub async fn list_projects(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        search: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2ProjectList> {
        let mut params = HashMap::new();
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/projects".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Reject an OAuth2 grant when the user denies consent. Returns the
    /// `redirectUrl` the end user should be sent to with an `access_denied` error.
    /// You can pass Accept header of `application/json` to receive a JSON response
    /// instead of a redirect.
    pub async fn reject(
        &self,
        grant_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Oauth2Reject> {
        let mut params = HashMap::new();
        params.insert("grant_id".to_string(), json!(grant_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/reject".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Revoke an OAuth2 access token or refresh token.
    pub async fn revoke(
        &self,
        token: impl Into<String>,
        token_type_hint: Option<&str>,
        client_id: Option<&str>,
        client_secret: Option<&str>,
    ) -> crate::error::Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("token".to_string(), json!(token.into()));
        if let Some(value) = token_type_hint {
            params.insert("token_type_hint".to_string(), json!(value));
        }
        if let Some(value) = client_id {
            params.insert("client_id".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("client_secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/revoke".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Exchange an OAuth2 authorization code, refresh token, or device code for
    /// access and refresh tokens.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_token(
        &self,
        grant_type: impl Into<String>,
        code: Option<&str>,
        refresh_token: Option<&str>,
        device_code: Option<&str>,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        code_verifier: Option<&str>,
        redirect_uri: Option<&str>,
        resource: Option<&str>,
        audience: Option<&str>,
    ) -> crate::error::Result<crate::models::Oauth2Token> {
        let mut params = HashMap::new();
        params.insert("grant_type".to_string(), json!(grant_type.into()));
        if let Some(value) = code {
            params.insert("code".to_string(), json!(value));
        }
        if let Some(value) = refresh_token {
            params.insert("refresh_token".to_string(), json!(value));
        }
        if let Some(value) = device_code {
            params.insert("device_code".to_string(), json!(value));
        }
        if let Some(value) = client_id {
            params.insert("client_id".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("client_secret".to_string(), json!(value));
        }
        if let Some(value) = code_verifier {
            params.insert("code_verifier".to_string(), json!(value));
        }
        if let Some(value) = redirect_uri {
            params.insert("redirect_uri".to_string(), json!(value));
        }
        if let Some(value) = resource {
            params.insert("resource".to_string(), json!(value));
        }
        if let Some(value) = audience {
            params.insert("audience".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/oauth2/{project_id}/token".to_string().replace("{project_id}", &self.client.get_headers().get("x-appwrite-project").cloned().unwrap_or_default());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Oauth2 {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_creation() {
        let client = Client::new();
        let service = Oauth2::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
