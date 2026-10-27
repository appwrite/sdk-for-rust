//! Waf service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Waf {
    client: Client,
}

impl Waf {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List WAF rules for the current project.
    pub async fn list_rules(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::WafRuleList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/waf/rules".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a bypass WAF rule. Conditions can match request attributes including
    /// `ip` (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`, `path`,
    /// `host`, `country`, `continent`, `headers.<name>`, `query.<key>`,
    /// `queryKeys`, `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`,
    /// `osVersion`, `browser`, and `browserVersion`. Conditions on `city` and
    /// `state` require the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_bypass_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: impl Into<String>,
        name: impl Into<String>,
        resource_id: Option<&str>,
        description: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleBypass> {
        let mut params = HashMap::new();
        params.insert("ruleId".to_string(), json!(rule_id.into()));
        params.insert("resourceType".to_string(), json!(resource_type.into()));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/waf/rules/bypass".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a bypass WAF rule. Conditions can match request attributes including
    /// `ip` (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`, `path`,
    /// `host`, `country`, `continent`, `headers.<name>`, `query.<key>`,
    /// `queryKeys`, `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`,
    /// `osVersion`, `browser`, and `browserVersion`. Conditions on `city` and
    /// `state` require the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_bypass_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        name: Option<&str>,
        description: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleBypass> {
        let mut params = HashMap::new();
        if let Some(value) = resource_type {
            params.insert("resourceType".to_string(), json!(value));
        }
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/bypass/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a challenge WAF rule. Use `difficulty` (1 easiest to 5 hardest) to
    /// tune the client-side proof-of-work cost, and `ttl` to control how long, in
    /// seconds, a visitor stays cleared after passing the challenge before being
    /// challenged again. Conditions can match request attributes including `ip`
    /// (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`, `path`, `host`,
    /// `country`, `continent`, `headers.<name>`, `query.<key>`, `queryKeys`,
    /// `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`, `osVersion`,
    /// `browser`, and `browserVersion`. Conditions on `city` and `state` require
    /// the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_challenge_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: impl Into<String>,
        name: impl Into<String>,
        resource_id: Option<&str>,
        description: Option<&str>,
        challenge_type: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
        difficulty: Option<i64>,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::WafRuleChallenge> {
        let mut params = HashMap::new();
        params.insert("ruleId".to_string(), json!(rule_id.into()));
        params.insert("resourceType".to_string(), json!(resource_type.into()));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = challenge_type {
            params.insert("challengeType".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = difficulty {
            params.insert("difficulty".to_string(), json!(value));
        }
        if let Some(value) = ttl {
            params.insert("ttl".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/waf/rules/challenge".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a challenge WAF rule. Use `difficulty` (1 easiest to 5 hardest) to
    /// tune the client-side proof-of-work cost, and `ttl` to control how long, in
    /// seconds, a visitor stays cleared after passing the challenge before being
    /// challenged again. Conditions can match request attributes including `ip`
    /// (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`, `path`, `host`,
    /// `country`, `continent`, `headers.<name>`, `query.<key>`, `queryKeys`,
    /// `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`, `osVersion`,
    /// `browser`, and `browserVersion`. Conditions on `city` and `state` require
    /// the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_challenge_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        name: Option<&str>,
        description: Option<&str>,
        challenge_type: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
        difficulty: Option<i64>,
        ttl: Option<i64>,
    ) -> crate::error::Result<crate::models::WafRuleChallenge> {
        let mut params = HashMap::new();
        if let Some(value) = resource_type {
            params.insert("resourceType".to_string(), json!(value));
        }
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = challenge_type {
            params.insert("challengeType".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = difficulty {
            params.insert("difficulty".to_string(), json!(value));
        }
        if let Some(value) = ttl {
            params.insert("ttl".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/challenge/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a deny WAF rule. Conditions can match request attributes including
    /// `ip` (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`, `path`,
    /// `host`, `country`, `continent`, `headers.<name>`, `query.<key>`,
    /// `queryKeys`, `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`,
    /// `osVersion`, `browser`, and `browserVersion`. Conditions on `city` and
    /// `state` require the premium Geo DB addon. `sessionAge` and `mfaAge` hold
    /// the seconds since the request's session was created and since its last MFA
    /// challenge. Both are null for requests without a session, such as guests and
    /// API keys, while a request authenticated with the `x-appwrite-jwt` header
    /// carries the age of the session its token is bound to; on `functions` rules
    /// the session comes from the `x-appwrite-user-jwt` header, and `mfaAge` is
    /// null when the session never completed MFA. A deny matching on `mfaAge`
    /// returns `user_challenge_required` and one matching on `sessionAge` returns
    /// `user_reauthentication_required`, both with status 401, so the user can
    /// sign in again and retry. To also deny requests without a session, combine
    /// `isNull` and `greaterThan` in an `or` condition.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_deny_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: impl Into<String>,
        name: impl Into<String>,
        resource_id: Option<&str>,
        description: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleDeny> {
        let mut params = HashMap::new();
        params.insert("ruleId".to_string(), json!(rule_id.into()));
        params.insert("resourceType".to_string(), json!(resource_type.into()));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/waf/rules/deny".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a deny WAF rule. Conditions can match request attributes including
    /// `ip` (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`, `path`,
    /// `host`, `country`, `continent`, `headers.<name>`, `query.<key>`,
    /// `queryKeys`, `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`,
    /// `osVersion`, `browser`, and `browserVersion`. Conditions on `city` and
    /// `state` require the premium Geo DB addon. `sessionAge` and `mfaAge` hold
    /// the seconds since the request's session was created and since its last MFA
    /// challenge. Both are null for requests without a session, such as guests and
    /// API keys, while a request authenticated with the `x-appwrite-jwt` header
    /// carries the age of the session its token is bound to; on `functions` rules
    /// the session comes from the `x-appwrite-user-jwt` header, and `mfaAge` is
    /// null when the session never completed MFA. A deny matching on `mfaAge`
    /// returns `user_challenge_required` and one matching on `sessionAge` returns
    /// `user_reauthentication_required`, both with status 401, so the user can
    /// sign in again and retry. To also deny requests without a session, combine
    /// `isNull` and `greaterThan` in an `or` condition.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_deny_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        name: Option<&str>,
        description: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleDeny> {
        let mut params = HashMap::new();
        if let Some(value) = resource_type {
            params.insert("resourceType".to_string(), json!(value));
        }
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/deny/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a rate limit WAF rule. Use `key` to choose the counter: `ip` limits
    /// per client IP, while `userId` limits per authenticated user (requests
    /// without an authenticated user skip `userId` rules). Conditions can match
    /// request attributes including `ip` (plain IPs or CIDR blocks like
    /// `10.0.0.0/8`), `method`, `path`, `host`, `country`, `continent`,
    /// `headers.<name>`, `query.<key>`, `queryKeys`, `accept`, `acceptLanguage`,
    /// `cookie`, `userAgent`, `os`, `osVersion`, `browser`, and `browserVersion`.
    /// Conditions on `city` and `state` require the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_rate_limit_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: impl Into<String>,
        name: impl Into<String>,
        limit: i64,
        interval: i64,
        resource_id: Option<&str>,
        description: Option<&str>,
        key: Option<&str>,
        strategy: Option<&str>,
        max_bucket_size: Option<i64>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleRateLimit> {
        let mut params = HashMap::new();
        params.insert("ruleId".to_string(), json!(rule_id.into()));
        params.insert("resourceType".to_string(), json!(resource_type.into()));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        params.insert("limit".to_string(), json!(limit));
        params.insert("interval".to_string(), json!(interval));
        if let Some(value) = key {
            params.insert("key".to_string(), json!(value));
        }
        if let Some(value) = strategy {
            params.insert("strategy".to_string(), json!(value));
        }
        if let Some(value) = max_bucket_size {
            params.insert("maxBucketSize".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/waf/rules/rate-limit".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a rate limit WAF rule. Use `key` to choose the counter: `ip` limits
    /// per client IP, while `userId` limits per authenticated user (requests
    /// without an authenticated user skip `userId` rules). Conditions can match
    /// request attributes including `ip` (plain IPs or CIDR blocks like
    /// `10.0.0.0/8`), `method`, `path`, `host`, `country`, `continent`,
    /// `headers.<name>`, `query.<key>`, `queryKeys`, `accept`, `acceptLanguage`,
    /// `cookie`, `userAgent`, `os`, `osVersion`, `browser`, and `browserVersion`.
    /// Conditions on `city` and `state` require the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_rate_limit_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        name: Option<&str>,
        description: Option<&str>,
        limit: Option<i64>,
        interval: Option<i64>,
        key: Option<&str>,
        max_bucket_size: Option<i64>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleRateLimit> {
        let mut params = HashMap::new();
        if let Some(value) = resource_type {
            params.insert("resourceType".to_string(), json!(value));
        }
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = key {
            params.insert("key".to_string(), json!(value));
        }
        if let Some(value) = max_bucket_size {
            params.insert("maxBucketSize".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/rate-limit/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a redirect WAF rule. Conditions can match request attributes
    /// including `ip` (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`,
    /// `path`, `host`, `country`, `continent`, `headers.<name>`, `query.<key>`,
    /// `queryKeys`, `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`,
    /// `osVersion`, `browser`, and `browserVersion`. Conditions on `city` and
    /// `state` require the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_redirect_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: impl Into<String>,
        name: impl Into<String>,
        location: impl Into<String>,
        status_code: i64,
        resource_id: Option<&str>,
        description: Option<&str>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleRedirect> {
        let mut params = HashMap::new();
        params.insert("ruleId".to_string(), json!(rule_id.into()));
        params.insert("resourceType".to_string(), json!(resource_type.into()));
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        params.insert("location".to_string(), json!(location.into()));
        params.insert("statusCode".to_string(), json!(status_code));
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/waf/rules/redirect".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update a redirect WAF rule. Conditions can match request attributes
    /// including `ip` (plain IPs or CIDR blocks like `10.0.0.0/8`), `method`,
    /// `path`, `host`, `country`, `continent`, `headers.<name>`, `query.<key>`,
    /// `queryKeys`, `accept`, `acceptLanguage`, `cookie`, `userAgent`, `os`,
    /// `osVersion`, `browser`, and `browserVersion`. Conditions on `city` and
    /// `state` require the premium Geo DB addon.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_redirect_rule(
        &self,
        rule_id: impl Into<String>,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        name: Option<&str>,
        description: Option<&str>,
        location: Option<&str>,
        status_code: Option<i64>,
        priority: Option<i64>,
        enabled: Option<bool>,
        conditions: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::WafRuleRedirect> {
        let mut params = HashMap::new();
        if let Some(value) = resource_type {
            params.insert("resourceType".to_string(), json!(value));
        }
        if let Some(value) = resource_id {
            params.insert("resourceId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = description {
            params.insert("description".to_string(), json!(value));
        }
        if let Some(value) = location {
            params.insert("location".to_string(), json!(value));
        }
        if let Some(value) = status_code {
            params.insert("statusCode".to_string(), json!(value));
        }
        if let Some(value) = priority {
            params.insert("priority".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = conditions {
            params.insert(
                "conditions".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/redirect/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a WAF rule by its ID.
    pub async fn get_rule(
        &self,
        rule_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a WAF rule.
    pub async fn delete_rule(&self, rule_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let rule_id = rule_id.into();
        if rule_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"ruleId\"",
                None,
                String::new(),
            ));
        }

        let path = "/waf/rules/{ruleId}"
            .to_string()
            .replace("{ruleId}", &Client::encode_path(&rule_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Waf {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waf_creation() {
        let client = Client::new();
        let service = Waf::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
