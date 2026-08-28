//! Advisor service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Advisor service surfaces actionable reports about your project
/// resources, with CTA descriptors for one-click remediation in the console.
#[derive(Debug, Clone)]
pub struct Advisor {
    client: Client,
}

impl Advisor {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the project's analyzer reports. You can use the query
    /// params to filter your results.
    pub async fn list_reports(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::ReportList> {
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

        let path = "/reports".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get an analyzer report by its unique ID. The response includes the report's
    /// metadata and the nested insights it produced.
    pub async fn get_report(
        &self,
        report_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Report> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/reports/{reportId}"
            .to_string()
            .replace("{reportId}", &report_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete an analyzer report by its unique ID. Nested insights and CTA
    /// metadata are removed asynchronously by the deletes worker.
    pub async fn delete_report(&self, report_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/reports/{reportId}"
            .to_string()
            .replace("{reportId}", &report_id.into().to_string());

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// List the insights produced under a single analyzer report. You can use the
    /// query params to filter your results further.
    pub async fn list_insights(
        &self,
        report_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::InsightList> {
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

        let path = "/reports/{reportId}/insights"
            .to_string()
            .replace("{reportId}", &report_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get an insight by its unique ID, scoped to its parent report.
    pub async fn get_insight(
        &self,
        report_id: impl Into<String>,
        insight_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Insight> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/reports/{reportId}/insights/{insightId}"
            .to_string()
            .replace("{reportId}", &report_id.into().to_string())
            .replace("{insightId}", &insight_id.into().to_string());

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Advisor {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advisor_creation() {
        let client = Client::new();
        let service = Advisor::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
