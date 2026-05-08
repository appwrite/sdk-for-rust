//! Project service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Project service allows you to manage all the projects in your Appwrite
/// server.
#[derive(Debug, Clone)]
pub struct Project {
    client: Client,
}

impl Project {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Delete a project.
    pub async fn delete(
        &self,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project".to_string();

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update properties of a specific auth method. Use this endpoint to enable or
    /// disable a method in your project.
    pub async fn update_auth_method(
        &self,
        method_id: crate::enums::MethodId,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/auth-methods/{methodId}".to_string().replace("{methodId}", &method_id.to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all API keys from the current project.
    pub async fn list_keys(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::KeyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/keys".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new API key. It's recommended to have multiple API keys with
    /// strict scopes for separate functions within your project.
    /// 
    /// You can also create an ephemeral API key if you need a short-lived key
    /// instead.
    pub async fn create_key(
        &self,
        key_id: impl Into<String>,
        name: impl Into<String>,
        scopes: Vec<crate::enums::Scopes>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::Key> {
        let mut params = HashMap::new();
        params.insert("keyId".to_string(), json!(key_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("scopes".to_string(), json!(scopes));
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new ephemeral API key. It's recommended to have multiple API keys
    /// with strict scopes for separate functions within your project.
    /// 
    /// You can also create a standard API key if you need a longer-lived key
    /// instead.
    pub async fn create_ephemeral_key(
        &self,
        scopes: Vec<crate::enums::Scopes>,
        duration: i64,
    ) -> crate::error::Result<crate::models::EphemeralKey> {
        let mut params = HashMap::new();
        params.insert("scopes".to_string(), json!(scopes));
        params.insert("duration".to_string(), json!(duration));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys/ephemeral".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a key by its unique ID.
    pub async fn get_key(
        &self,
        key_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Key> {
        let params = HashMap::new();

        let path = "/project/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a key by its unique ID. Use this endpoint to update the name,
    /// scopes, or expiration time of an API key.
    pub async fn update_key(
        &self,
        key_id: impl Into<String>,
        name: impl Into<String>,
        scopes: Vec<crate::enums::Scopes>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::Key> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("scopes".to_string(), json!(scopes));
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a key by its unique ID. Once deleted, the key can no longer be used
    /// to authenticate API calls.
    pub async fn delete_key(
        &self,
        key_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project labels. Labels can be used to easily filter projects in
    /// an organization.
    pub async fn update_labels(
        &self,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("labels".to_string(), json!(labels.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/labels".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all mock phones in the project. This endpoint returns an
    /// array of all mock phones and their OTPs.
    pub async fn list_mock_phones(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::MockNumberList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/mock-phones".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new mock phone for your project. Use this endpoint to register a
    /// mock phone number and its sign-in OTP for your testers.
    pub async fn create_mock_phone(
        &self,
        number: impl Into<String>,
        otp: impl Into<String>,
    ) -> crate::error::Result<crate::models::MockNumber> {
        let mut params = HashMap::new();
        params.insert("number".to_string(), json!(number.into()));
        params.insert("otp".to_string(), json!(otp.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/mock-phones".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a mock phone by its unique number. This endpoint returns the mock
    /// phone's OTP.
    pub async fn get_mock_phone(
        &self,
        number: impl Into<String>,
    ) -> crate::error::Result<crate::models::MockNumber> {
        let params = HashMap::new();

        let path = "/project/mock-phones/{number}".to_string().replace("{number}", &number.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a mock phone by its unique number. Use this endpoint to update the
    /// mock phone's OTP.
    pub async fn update_mock_phone(
        &self,
        number: impl Into<String>,
        otp: impl Into<String>,
    ) -> crate::error::Result<crate::models::MockNumber> {
        let mut params = HashMap::new();
        params.insert("otp".to_string(), json!(otp.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/mock-phones/{number}".to_string().replace("{number}", &number.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a mock phone by its unique number. This endpoint removes the mock
    /// phone and its OTP configuration from the project.
    pub async fn delete_mock_phone(
        &self,
        number: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/mock-phones/{number}".to_string().replace("{number}", &number.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all OAuth2 providers supported by the server, along with the
    /// project's configuration for each. Credential fields are write-only and
    /// always returned empty.
    pub async fn list_o_auth2_providers(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2ProviderList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/oauth2".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a single OAuth2 provider configuration. Credential fields (client
    /// secret, p8 file, key/team IDs) are write-only and always returned empty.
    pub async fn get_o_auth2_provider(
        &self,
        provider_id: crate::enums::ProviderId,
    ) -> crate::error::Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("providerId".to_string(), json!(provider_id));

        let path = "/project/oauth2/:provider".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update the project OAuth2 Amazon configuration.
    pub async fn update_o_auth2_amazon(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Amazon> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/amazon".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Apple configuration.
    pub async fn update_o_auth2_apple(
        &self,
        service_id: Option<&str>,
        key_id: Option<&str>,
        team_id: Option<&str>,
        p8_file: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Apple> {
        let mut params = HashMap::new();
        if let Some(value) = service_id {
            params.insert("serviceId".to_string(), json!(value));
        }
        if let Some(value) = key_id {
            params.insert("keyId".to_string(), json!(value));
        }
        if let Some(value) = team_id {
            params.insert("teamId".to_string(), json!(value));
        }
        if let Some(value) = p8_file {
            params.insert("p8File".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/apple".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Auth0 configuration.
    pub async fn update_o_auth2_auth0(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        endpoint: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Auth0> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = endpoint {
            params.insert("endpoint".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/auth0".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Authentik configuration.
    pub async fn update_o_auth2_authentik(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        endpoint: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Authentik> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = endpoint {
            params.insert("endpoint".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/authentik".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Autodesk configuration.
    pub async fn update_o_auth2_autodesk(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Autodesk> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/autodesk".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Bitbucket configuration.
    pub async fn update_o_auth2_bitbucket(
        &self,
        key: Option<&str>,
        secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Bitbucket> {
        let mut params = HashMap::new();
        if let Some(value) = key {
            params.insert("key".to_string(), json!(value));
        }
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/bitbucket".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Bitly configuration.
    pub async fn update_o_auth2_bitly(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Bitly> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/bitly".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Box configuration.
    pub async fn update_o_auth2_box(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Box> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/box".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Dailymotion configuration.
    pub async fn update_o_auth2_dailymotion(
        &self,
        api_key: Option<&str>,
        api_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Dailymotion> {
        let mut params = HashMap::new();
        if let Some(value) = api_key {
            params.insert("apiKey".to_string(), json!(value));
        }
        if let Some(value) = api_secret {
            params.insert("apiSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/dailymotion".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Discord configuration.
    pub async fn update_o_auth2_discord(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Discord> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/discord".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Disqus configuration.
    pub async fn update_o_auth2_disqus(
        &self,
        public_key: Option<&str>,
        secret_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Disqus> {
        let mut params = HashMap::new();
        if let Some(value) = public_key {
            params.insert("publicKey".to_string(), json!(value));
        }
        if let Some(value) = secret_key {
            params.insert("secretKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/disqus".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Dropbox configuration.
    pub async fn update_o_auth2_dropbox(
        &self,
        app_key: Option<&str>,
        app_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Dropbox> {
        let mut params = HashMap::new();
        if let Some(value) = app_key {
            params.insert("appKey".to_string(), json!(value));
        }
        if let Some(value) = app_secret {
            params.insert("appSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/dropbox".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Etsy configuration.
    pub async fn update_o_auth2_etsy(
        &self,
        key_string: Option<&str>,
        shared_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Etsy> {
        let mut params = HashMap::new();
        if let Some(value) = key_string {
            params.insert("keyString".to_string(), json!(value));
        }
        if let Some(value) = shared_secret {
            params.insert("sharedSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/etsy".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Facebook configuration.
    pub async fn update_o_auth2_facebook(
        &self,
        app_id: Option<&str>,
        app_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Facebook> {
        let mut params = HashMap::new();
        if let Some(value) = app_id {
            params.insert("appId".to_string(), json!(value));
        }
        if let Some(value) = app_secret {
            params.insert("appSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/facebook".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Figma configuration.
    pub async fn update_o_auth2_figma(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Figma> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/figma".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 FusionAuth configuration.
    pub async fn update_o_auth2_fusion_auth(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        endpoint: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2FusionAuth> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = endpoint {
            params.insert("endpoint".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/fusionauth".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 GitHub configuration.
    pub async fn update_o_auth2_git_hub(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Github> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/github".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Gitlab configuration.
    pub async fn update_o_auth2_gitlab(
        &self,
        application_id: Option<&str>,
        secret: Option<&str>,
        endpoint: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Gitlab> {
        let mut params = HashMap::new();
        if let Some(value) = application_id {
            params.insert("applicationId".to_string(), json!(value));
        }
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        if let Some(value) = endpoint {
            params.insert("endpoint".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/gitlab".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Google configuration.
    pub async fn update_o_auth2_google(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Google> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/google".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Keycloak configuration.
    pub async fn update_o_auth2_keycloak(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        endpoint: Option<&str>,
        realm_name: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Keycloak> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = endpoint {
            params.insert("endpoint".to_string(), json!(value));
        }
        if let Some(value) = realm_name {
            params.insert("realmName".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/keycloak".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Kick configuration.
    pub async fn update_o_auth2_kick(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Kick> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/kick".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Linkedin configuration.
    pub async fn update_o_auth2_linkedin(
        &self,
        client_id: Option<&str>,
        primary_client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Linkedin> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = primary_client_secret {
            params.insert("primaryClientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/linkedin".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Microsoft configuration.
    pub async fn update_o_auth2_microsoft(
        &self,
        application_id: Option<&str>,
        application_secret: Option<&str>,
        tenant: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Microsoft> {
        let mut params = HashMap::new();
        if let Some(value) = application_id {
            params.insert("applicationId".to_string(), json!(value));
        }
        if let Some(value) = application_secret {
            params.insert("applicationSecret".to_string(), json!(value));
        }
        if let Some(value) = tenant {
            params.insert("tenant".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/microsoft".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Notion configuration.
    pub async fn update_o_auth2_notion(
        &self,
        oauth_client_id: Option<&str>,
        oauth_client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Notion> {
        let mut params = HashMap::new();
        if let Some(value) = oauth_client_id {
            params.insert("oauthClientId".to_string(), json!(value));
        }
        if let Some(value) = oauth_client_secret {
            params.insert("oauthClientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/notion".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Oidc configuration.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_o_auth2_oidc(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        well_known_url: Option<&str>,
        authorization_url: Option<&str>,
        token_url: Option<&str>,
        user_info_url: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Oidc> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = well_known_url {
            params.insert("wellKnownURL".to_string(), json!(value));
        }
        if let Some(value) = authorization_url {
            params.insert("authorizationURL".to_string(), json!(value));
        }
        if let Some(value) = token_url {
            params.insert("tokenURL".to_string(), json!(value));
        }
        if let Some(value) = user_info_url {
            params.insert("userInfoURL".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/oidc".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Okta configuration.
    pub async fn update_o_auth2_okta(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        domain: Option<&str>,
        authorization_server_id: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Okta> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = authorization_server_id {
            params.insert("authorizationServerId".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/okta".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Paypal configuration.
    pub async fn update_o_auth2_paypal(
        &self,
        client_id: Option<&str>,
        secret_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Paypal> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = secret_key {
            params.insert("secretKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/paypal".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 PaypalSandbox configuration.
    pub async fn update_o_auth2_paypal_sandbox(
        &self,
        client_id: Option<&str>,
        secret_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Paypal> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = secret_key {
            params.insert("secretKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/paypalSandbox".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Podio configuration.
    pub async fn update_o_auth2_podio(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Podio> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/podio".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Salesforce configuration.
    pub async fn update_o_auth2_salesforce(
        &self,
        customer_key: Option<&str>,
        customer_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Salesforce> {
        let mut params = HashMap::new();
        if let Some(value) = customer_key {
            params.insert("customerKey".to_string(), json!(value));
        }
        if let Some(value) = customer_secret {
            params.insert("customerSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/salesforce".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Slack configuration.
    pub async fn update_o_auth2_slack(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Slack> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/slack".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Spotify configuration.
    pub async fn update_o_auth2_spotify(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Spotify> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/spotify".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Stripe configuration.
    pub async fn update_o_auth2_stripe(
        &self,
        client_id: Option<&str>,
        api_secret_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Stripe> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = api_secret_key {
            params.insert("apiSecretKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/stripe".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Tradeshift configuration.
    pub async fn update_o_auth2_tradeshift(
        &self,
        oauth2_client_id: Option<&str>,
        oauth2_client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Tradeshift> {
        let mut params = HashMap::new();
        if let Some(value) = oauth2_client_id {
            params.insert("oauth2ClientId".to_string(), json!(value));
        }
        if let Some(value) = oauth2_client_secret {
            params.insert("oauth2ClientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/tradeshift".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Tradeshift Sandbox configuration.
    pub async fn update_o_auth2_tradeshift_sandbox(
        &self,
        oauth2_client_id: Option<&str>,
        oauth2_client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Tradeshift> {
        let mut params = HashMap::new();
        if let Some(value) = oauth2_client_id {
            params.insert("oauth2ClientId".to_string(), json!(value));
        }
        if let Some(value) = oauth2_client_secret {
            params.insert("oauth2ClientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/tradeshiftBox".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Twitch configuration.
    pub async fn update_o_auth2_twitch(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Twitch> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/twitch".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 WordPress configuration.
    pub async fn update_o_auth2_word_press(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2WordPress> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/wordpress".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 X configuration.
    pub async fn update_o_auth2_x(
        &self,
        customer_key: Option<&str>,
        secret_key: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2X> {
        let mut params = HashMap::new();
        if let Some(value) = customer_key {
            params.insert("customerKey".to_string(), json!(value));
        }
        if let Some(value) = secret_key {
            params.insert("secretKey".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/x".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Yahoo configuration.
    pub async fn update_o_auth2_yahoo(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Yahoo> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/yahoo".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Yandex configuration.
    pub async fn update_o_auth2_yandex(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Yandex> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/yandex".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Zoho configuration.
    pub async fn update_o_auth2_zoho(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Zoho> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/zoho".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project OAuth2 Zoom configuration.
    pub async fn update_o_auth2_zoom(
        &self,
        client_id: Option<&str>,
        client_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::OAuth2Zoom> {
        let mut params = HashMap::new();
        if let Some(value) = client_id {
            params.insert("clientId".to_string(), json!(value));
        }
        if let Some(value) = client_secret {
            params.insert("clientSecret".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/oauth2/zoom".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all platforms in the project. This endpoint returns an array
    /// of all platforms and their configurations.
    pub async fn list_platforms(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::PlatformList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/platforms".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Android platform for your project. Use this endpoint to
    /// register a new Android platform where your users will run your application
    /// which will interact with the Appwrite API.
    pub async fn create_android_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        application_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformAndroid> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("applicationId".to_string(), json!(application_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/android".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an Android platform by its unique ID. Use this endpoint to update
    /// the platform's name or application ID.
    pub async fn update_android_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        application_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformAndroid> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("applicationId".to_string(), json!(application_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/android/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Apple platform for your project. Use this endpoint to register
    /// a new Apple platform where your users will run your application which will
    /// interact with the Appwrite API.
    pub async fn create_apple_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        bundle_identifier: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformApple> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("bundleIdentifier".to_string(), json!(bundle_identifier.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/apple".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an Apple platform by its unique ID. Use this endpoint to update the
    /// platform's name or bundle identifier.
    pub async fn update_apple_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        bundle_identifier: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformApple> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("bundleIdentifier".to_string(), json!(bundle_identifier.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/apple/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Linux platform for your project. Use this endpoint to register
    /// a new Linux platform where your users will run your application which will
    /// interact with the Appwrite API.
    pub async fn create_linux_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformLinux> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageName".to_string(), json!(package_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/linux".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Linux platform by its unique ID. Use this endpoint to update the
    /// platform's name or package name.
    pub async fn update_linux_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformLinux> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageName".to_string(), json!(package_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/linux/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new web platform for your project. Use this endpoint to register a
    /// new platform where your users will run your application which will interact
    /// with the Appwrite API.
    pub async fn create_web_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        hostname: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWeb> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("hostname".to_string(), json!(hostname.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/web".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a web platform by its unique ID. Use this endpoint to update the
    /// platform's name or hostname.
    pub async fn update_web_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        hostname: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWeb> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("hostname".to_string(), json!(hostname.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/web/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Windows platform for your project. Use this endpoint to
    /// register a new Windows platform where your users will run your application
    /// which will interact with the Appwrite API.
    pub async fn create_windows_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_identifier_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWindows> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageIdentifierName".to_string(), json!(package_identifier_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/windows".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Windows platform by its unique ID. Use this endpoint to update the
    /// platform's name or package identifier name.
    pub async fn update_windows_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_identifier_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWindows> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageIdentifierName".to_string(), json!(package_identifier_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/windows/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Get a platform by its unique ID. This endpoint returns the platform's
    /// details, including its name, type, and key configurations.
    pub async fn get_platform(
        &self,
        platform_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();

        let path = "/project/platforms/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a platform by its unique ID. This endpoint removes the platform and
    /// all its configurations from the project.
    pub async fn delete_platform(
        &self,
        platform_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all project policies and their current configuration.
    pub async fn list_policies(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::PolicyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/policies".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Updating this policy allows you to control if team members can see other
    /// members information. When enabled, all team members can see ID, name,
    /// email, phone number, and MFA status of other members..
    pub async fn update_membership_privacy_policy(
        &self,
        user_id: Option<bool>,
        user_email: Option<bool>,
        user_phone: Option<bool>,
        user_name: Option<bool>,
        user_mfa: Option<bool>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = user_id {
            params.insert("userId".to_string(), json!(value));
        }
        if let Some(value) = user_email {
            params.insert("userEmail".to_string(), json!(value));
        }
        if let Some(value) = user_phone {
            params.insert("userPhone".to_string(), json!(value));
        }
        if let Some(value) = user_name {
            params.insert("userName".to_string(), json!(value));
        }
        if let Some(value) = user_mfa {
            params.insert("userMFA".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/membership-privacy".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if new passwords are checked
    /// against most common passwords dictionary. When enabled, and user changes
    /// their password, password must not be contained in the dictionary.
    pub async fn update_password_dictionary_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/password-dictionary".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updates one of password strength policies. Based on total length
    /// configured, previous password hashes are stored, and users cannot choose a
    /// new password that is already stored in the passwird history list, when
    /// updating an user password, or setting new one through password recovery.
    /// 
    /// Keep in mind, while password history policy is disabled, the history is not
    /// being stored. Enabling the policy will not have any history on existing
    /// users, and it will only start to collect and enforce the policy on password
    /// changes since the policy is enabled.
    pub async fn update_password_history_policy(
        &self,
        total: Option<i64>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/password-history".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if password strength is checked
    /// against personal data. When enabled, and user sets or changes their
    /// password, the password must not contain user ID, name, email or phone
    /// number.
    pub async fn update_password_personal_data_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/password-personal-data".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if email alert is sent upon
    /// session creation. When enabled, and user signs into their account, they
    /// will be sent an email notification. There is an exception, the first
    /// session after a new sign up does not trigger an alert, even if the policy
    /// is enabled.
    pub async fn update_session_alert_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-alert".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update maximum duration how long sessions created within a project should
    /// stay active for.
    pub async fn update_session_duration_policy(
        &self,
        duration: i64,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("duration".to_string(), json!(duration));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-duration".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if existing sessions should be
    /// invalidated when a password of a user is changed. When enabled, and user
    /// changes their password, they will be logged out of all their devices.
    pub async fn update_session_invalidation_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-invalidation".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the maximum number of sessions allowed per user. When the limit is
    /// hit, the oldest session will be deleted to make room for new one.
    pub async fn update_session_limit_policy(
        &self,
        total: Option<i64>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-limit".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the maximum number of users in the project. When the limit is hit or
    /// amount of existing users already exceeded the limit, all users remain
    /// active, but new user sign up will be prohibited.
    pub async fn update_user_limit_policy(
        &self,
        total: Option<i64>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/user-limit".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a policy by its unique ID. This endpoint returns the current
    /// configuration for the requested project policy.
    pub async fn get_policy(
        &self,
        policy_id: crate::enums::PolicyId,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();

        let path = "/project/policies/{policyId}".to_string().replace("{policyId}", &policy_id.to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update properties of a specific protocol. Use this endpoint to enable or
    /// disable a protocol in your project.
    pub async fn update_protocol(
        &self,
        protocol_id: crate::enums::ProtocolId,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/protocols/{protocolId}".to_string().replace("{protocolId}", &protocol_id.to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update properties of a specific service. Use this endpoint to enable or
    /// disable a service in your project.
    pub async fn update_service(
        &self,
        service_id: crate::enums::ServiceId,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/services/{serviceId}".to_string().replace("{serviceId}", &service_id.to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the SMTP configuration for your project. Use this endpoint to
    /// configure your project's SMTP provider with your custom settings for
    /// sending transactional emails.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_smtp(
        &self,
        host: Option<&str>,
        port: Option<i64>,
        username: Option<&str>,
        password: Option<&str>,
        sender_email: Option<&str>,
        sender_name: Option<&str>,
        reply_to_email: Option<&str>,
        reply_to_name: Option<&str>,
        secure: Option<crate::enums::Secure>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = host {
            params.insert("host".to_string(), json!(value));
        }
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        if let Some(value) = username {
            params.insert("username".to_string(), json!(value));
        }
        if let Some(value) = password {
            params.insert("password".to_string(), json!(value));
        }
        if let Some(value) = sender_email {
            params.insert("senderEmail".to_string(), json!(value));
        }
        if let Some(value) = sender_name {
            params.insert("senderName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = secure {
            params.insert("secure".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/smtp".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Send a test email to verify SMTP configuration.
    pub async fn create_smtp_test(
        &self,
        emails: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<()> {
        let mut params = HashMap::new();
        params.insert("emails".to_string(), json!(emails.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/smtp/tests".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all custom email templates configured for the project. This
    /// endpoint returns an array of all configured email templates and their
    /// locales.
    pub async fn list_email_templates(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::EmailTemplateList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/templates/email".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a custom email template for the specified locale and type. Use this
    /// endpoint to modify the content of your email templates.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_email_template(
        &self,
        template_id: crate::enums::EmailTemplateType,
        locale: Option<crate::enums::EmailTemplateLocale>,
        subject: Option<&str>,
        message: Option<&str>,
        sender_name: Option<&str>,
        sender_email: Option<&str>,
        reply_to_email: Option<&str>,
        reply_to_name: Option<&str>,
    ) -> crate::error::Result<crate::models::EmailTemplate> {
        let mut params = HashMap::new();
        params.insert("templateId".to_string(), json!(template_id));
        if let Some(value) = locale {
            params.insert("locale".to_string(), json!(value));
        }
        if let Some(value) = subject {
            params.insert("subject".to_string(), json!(value));
        }
        if let Some(value) = message {
            params.insert("message".to_string(), json!(value));
        }
        if let Some(value) = sender_name {
            params.insert("senderName".to_string(), json!(value));
        }
        if let Some(value) = sender_email {
            params.insert("senderEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/templates/email".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a custom email template for the specified locale and type. This
    /// endpoint returns the template content, subject, and other configuration
    /// details.
    pub async fn get_email_template(
        &self,
        template_id: crate::enums::EmailTemplateType,
        locale: Option<crate::enums::EmailTemplateLocale>,
    ) -> crate::error::Result<crate::models::EmailTemplate> {
        let mut params = HashMap::new();
        if let Some(value) = locale {
            params.insert("locale".to_string(), json!(value));
        }

        let path = "/project/templates/email/{templateId}".to_string().replace("{templateId}", &template_id.to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a list of all project environment variables.
    pub async fn list_variables(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::VariableList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/variables".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new project environment variable. These variables can be accessed
    /// by all functions and sites in the project.
    pub async fn create_variable(
        &self,
        variable_id: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
        secret: Option<bool>,
    ) -> crate::error::Result<crate::models::Variable> {
        let mut params = HashMap::new();
        params.insert("variableId".to_string(), json!(variable_id.into()));
        params.insert("key".to_string(), json!(key.into()));
        params.insert("value".to_string(), json!(value.into()));
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/variables".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a variable by its unique ID.
    pub async fn get_variable(
        &self,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Variable> {
        let params = HashMap::new();

        let path = "/project/variables/{variableId}".to_string().replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update variable by its unique ID.
    pub async fn update_variable(
        &self,
        variable_id: impl Into<String>,
        key: Option<&str>,
        value: Option<&str>,
        secret: Option<bool>,
    ) -> crate::error::Result<crate::models::Variable> {
        let mut params = HashMap::new();
        if let Some(value) = key {
            params.insert("key".to_string(), json!(value));
        }
        if let Some(value) = value {
            params.insert("value".to_string(), json!(value));
        }
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/variables/{variableId}".to_string().replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a variable by its unique ID.
    pub async fn delete_variable(
        &self,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/variables/{variableId}".to_string().replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Project {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let client = Client::new();
        let service = Project::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
