//! Analytics service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Analytics {
    client: Client,
}

impl Analytics {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List analytics properties for the current project.
    pub async fn list_properties(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AnalyticsPropertyList> {
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

        let path = "/analytics/properties".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new analytics property to track a website or application.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_property(
        &self,
        property_id: impl Into<String>,
        name: impl Into<String>,
        domain: Option<&str>,
        enabled: Option<bool>,
        allowed_origins: Option<Vec<String>>,
        identify_users: Option<bool>,
        r#type: Option<crate::enums::AnalyticsPropertyType>,
    ) -> crate::error::Result<crate::models::AnalyticsProperty> {
        let mut params = HashMap::new();
        params.insert("propertyId".to_string(), json!(property_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = allowed_origins {
            params.insert(
                "allowedOrigins".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = identify_users {
            params.insert("identifyUsers".to_string(), json!(value));
        }
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/analytics/properties".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get an analytics property by ID.
    pub async fn get_property(
        &self,
        property_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AnalyticsProperty> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an analytics property. Only the attributes you pass are changed;
    /// omitted attributes keep their current value.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_property(
        &self,
        property_id: impl Into<String>,
        name: Option<&str>,
        domain: Option<&str>,
        enabled: Option<bool>,
        allowed_origins: Option<Vec<String>>,
        identify_users: Option<bool>,
        r#type: Option<crate::enums::AnalyticsPropertyType>,
    ) -> crate::error::Result<crate::models::AnalyticsProperty> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        if let Some(value) = allowed_origins {
            params.insert(
                "allowedOrigins".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = identify_users {
            params.insert("identifyUsers".to_string(), json!(value));
        }
        if let Some(value) = r#type {
            params.insert("type".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete an analytics property along with every event and session collected
    /// for it. This cannot be undone.
    pub async fn delete_property(
        &self,
        property_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// List the annotations of an analytics property. Filter on `date` to fetch
    /// the annotations inside a chart's time range.
    pub async fn list_annotations(
        &self,
        property_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AnalyticsAnnotationList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/annotations"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create an annotation that marks a point in time on an analytics property's
    /// charts.
    pub async fn create_annotation(
        &self,
        property_id: impl Into<String>,
        annotation_id: impl Into<String>,
        date: impl Into<String>,
        content: impl Into<String>,
    ) -> crate::error::Result<crate::models::AnalyticsAnnotation> {
        let mut params = HashMap::new();
        params.insert("annotationId".to_string(), json!(annotation_id.into()));
        params.insert("date".to_string(), json!(date.into()));
        params.insert("content".to_string(), json!(content.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/annotations"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get an analytics annotation by ID.
    pub async fn get_annotation(
        &self,
        property_id: impl Into<String>,
        annotation_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::AnalyticsAnnotation> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let annotation_id = annotation_id.into();
        if annotation_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"annotationId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/annotations/{annotationId}"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id))
            .replace("{annotationId}", &Client::encode_path(&annotation_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an analytics annotation. Only the attributes you pass are changed;
    /// omitted attributes keep their current value.
    pub async fn update_annotation(
        &self,
        property_id: impl Into<String>,
        annotation_id: impl Into<String>,
        date: Option<&str>,
        content: Option<&str>,
    ) -> crate::error::Result<crate::models::AnalyticsAnnotation> {
        let mut params = HashMap::new();
        if let Some(value) = date {
            params.insert("date".to_string(), json!(value));
        }
        if let Some(value) = content {
            params.insert("content".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let annotation_id = annotation_id.into();
        if annotation_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"annotationId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/annotations/{annotationId}"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id))
            .replace("{annotationId}", &Client::encode_path(&annotation_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete an analytics annotation.
    pub async fn delete_annotation(
        &self,
        property_id: impl Into<String>,
        annotation_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let annotation_id = annotation_id.into();
        if annotation_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"annotationId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/annotations/{annotationId}"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id))
            .replace("{annotationId}", &Client::encode_path(&annotation_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Send a tracking event from a browser, native app, or server-side SDK.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_event(
        &self,
        property_id: impl Into<String>,
        name: impl Into<String>,
        url: impl Into<String>,
        domain: Option<&str>,
        referrer: Option<&str>,
        screen_width: Option<i64>,
        session_hash: Option<&str>,
        scroll_depth: Option<i64>,
        engagement_time: Option<i64>,
        props: Option<Vec<String>>,
        user_id: Option<&str>,
        ip: Option<&str>,
        user_agent: Option<&str>,
        interactive: Option<bool>,
    ) -> crate::error::Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("url".to_string(), json!(url.into()));
        if let Some(value) = domain {
            params.insert("domain".to_string(), json!(value));
        }
        if let Some(value) = referrer {
            params.insert("referrer".to_string(), json!(value));
        }
        if let Some(value) = screen_width {
            params.insert("screenWidth".to_string(), json!(value));
        }
        if let Some(value) = session_hash {
            params.insert("sessionHash".to_string(), json!(value));
        }
        if let Some(value) = scroll_depth {
            params.insert("scrollDepth".to_string(), json!(value));
        }
        if let Some(value) = engagement_time {
            params.insert("engagementTime".to_string(), json!(value));
        }
        if let Some(value) = props {
            params.insert(
                "props".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = user_id {
            params.insert("userId".to_string(), json!(value));
        }
        if let Some(value) = ip {
            params.insert("ip".to_string(), json!(value));
        }
        if let Some(value) = user_agent {
            params.insert("userAgent".to_string(), json!(value));
        }
        if let Some(value) = interactive {
            params.insert("interactive".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/events"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Read analytics metrics (visitors, sessions, pageviews, events, bounceRate,
    /// …) for a property over a date range.
    ///
    /// **Three response shapes**, chosen by `dimensions[]` and `interval`:
    /// - Neither: one row aggregating the whole window, with `value` and `date`
    /// null. Only this shape carries `visits`, `bounceRate`, `visitDuration`,
    /// `viewsPerVisit`, `scrollDepth` and `engagementTime`.
    /// - `dimensions[]`: one row per dimension value, ranked by visitors, with
    /// `value` set and `date` null.
    /// - `interval`: one row per time bucket in chronological order, with `date`
    /// set and `value` null.
    ///
    /// `pageviews` is carried by the flat aggregate and by `interval`, but not by
    /// a breakdown.
    ///
    /// `userId` and `users` cover identified traffic only: events sent with an API
    /// key's `userId`, or from a signed-in session on a property with
    /// `identifyUsers` on. A `userId` breakdown leaves out anonymous events;
    /// returned values are the project's user IDs.
    ///
    /// Custom event props: `dimensions[]=props` ranks the prop keys seen in the
    /// window, and adding `propKey` ranks the values of that one key. `queries[]`
    /// filters on a prop's value with the `props.<key>` attribute, e.g.
    /// `equal("props.plan", ["pro"])`.
    ///
    /// Combining `dimensions[]` with `interval` is not supported yet. `queries[]`
    /// filters the underlying events using standard Utopia query syntax.
    #[allow(clippy::too_many_arguments)]
    pub async fn list_metrics(
        &self,
        property_id: impl Into<String>,
        queries: Option<Vec<String>>,
        interval: Option<crate::enums::AnalyticsInterval>,
        dimensions: Option<Vec<crate::enums::AnalyticsDimension>>,
        prop_key: Option<&str>,
        date_range: Option<&str>,
        start_at: Option<&str>,
        end_at: Option<&str>,
        limit: Option<i64>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::AnalyticsMetricList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = dimensions {
            params.insert("dimensions".to_string(), json!(value));
        }
        if let Some(value) = prop_key {
            params.insert("propKey".to_string(), json!(value));
        }
        if let Some(value) = date_range {
            params.insert("dateRange".to_string(), json!(value));
        }
        if let Some(value) = start_at {
            params.insert("startAt".to_string(), json!(value));
        }
        if let Some(value) = end_at {
            params.insert("endAt".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let property_id = property_id.into();
        if property_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"propertyId\"",
                None,
                String::new(),
            ));
        }

        let path = "/analytics/properties/{propertyId}/metrics"
            .to_string()
            .replace("{propertyId}", &Client::encode_path(&property_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Analytics {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_creation() {
        let client = Client::new();
        let service = Analytics::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
