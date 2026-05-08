//! Proxy service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Proxy Service allows you to configure actions for your domains beyond
/// DNS configuration.
#[derive(Debug, Clone)]
pub struct Proxy {
    client: Client,
}

impl Proxy {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the proxy rules. You can use the query params to filter
    /// your results.
    pub async fn list_rules(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ProxyRuleList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/proxy/rules".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new proxy rule for serving Appwrite's API on custom domain.
    /// 
    /// Rule ID is automatically generated as MD5 hash of a rule domain for
    /// performance purposes.
    pub async fn create_api_rule(
        &self,
        domain: impl Into<String>,
    ) -> crate::error::Result<crate::models::ProxyRule> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), json!(domain.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/proxy/rules/api".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new proxy rule for executing Appwrite Function on custom domain.
    /// 
    /// Rule ID is automatically generated as MD5 hash of a rule domain for
    /// performance purposes.
    pub async fn create_function_rule(
        &self,
        domain: impl Into<String>,
        function_id: impl Into<String>,
        branch: Option<&str>,
    ) -> crate::error::Result<crate::models::ProxyRule> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), json!(domain.into()));
        params.insert("functionId".to_string(), json!(function_id.into()));
        if let Some(value) = branch {
            params.insert("branch".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/proxy/rules/function".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new proxy rule for to redirect from custom domain to another
    /// domain.
    /// 
    /// Rule ID is automatically generated as MD5 hash of a rule domain for
    /// performance purposes.
    pub async fn create_redirect_rule(
        &self,
        domain: impl Into<String>,
        url: impl Into<String>,
        status_code: crate::enums::StatusCode,
        resource_id: impl Into<String>,
        resource_type: crate::enums::ProxyResourceType,
    ) -> crate::error::Result<crate::models::ProxyRule> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), json!(domain.into()));
        params.insert("url".to_string(), json!(url.into()));
        params.insert("statusCode".to_string(), json!(status_code));
        params.insert("resourceId".to_string(), json!(resource_id.into()));
        params.insert("resourceType".to_string(), json!(resource_type));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/proxy/rules/redirect".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new proxy rule for serving Appwrite Site on custom domain.
    /// 
    /// Rule ID is automatically generated as MD5 hash of a rule domain for
    /// performance purposes.
    pub async fn create_site_rule(
        &self,
        domain: impl Into<String>,
        site_id: impl Into<String>,
        branch: Option<&str>,
    ) -> crate::error::Result<crate::models::ProxyRule> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), json!(domain.into()));
        params.insert("siteId".to_string(), json!(site_id.into()));
        if let Some(value) = branch {
            params.insert("branch".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/proxy/rules/site".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a proxy rule by its unique ID.
    pub async fn get_rule(
        &self,
        rule_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::ProxyRule> {
        let params = HashMap::new();

        let path = "/proxy/rules/{ruleId}".to_string().replace("{ruleId}", &rule_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a proxy rule by its unique ID.
    pub async fn delete_rule(
        &self,
        rule_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/proxy/rules/{ruleId}".to_string().replace("{ruleId}", &rule_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// If not succeeded yet, retry verification process of a proxy rule domain.
    /// This endpoint triggers domain verification by checking DNS records. If
    /// verification is successful, a TLS certificate will be automatically
    /// provisioned for the domain asynchronously in the background.
    pub async fn update_rule_status(
        &self,
        rule_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::ProxyRule> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/proxy/rules/{ruleId}/status".to_string().replace("{ruleId}", &rule_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Proxy {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_creation() {
        let client = Client::new();
        let service = Proxy::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
