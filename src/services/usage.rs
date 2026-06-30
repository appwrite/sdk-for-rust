//! Usage service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Usage {
    client: Client,
}

impl Usage {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Aggregate usage event metrics. `metrics[]` (1-10) is required; the response
    /// always contains one entry per requested metric, each with its own
    /// `points[]` time series.
    /// 
    /// **Two response shapes**:
    /// - Omit `interval` for a flat top-N table — one point per dimension
    /// combination, no time axis. Useful for "top 10 paths by bandwidth in the
    /// last 7 days".
    /// - Pass `interval` (`1m`, `15m`, `30m`, `1h`, `1d`) for a time series —
    /// one point per (time bucket × dimension combination).
    /// 
    /// `dimensions[]` breaks each point down by one or more attributes (service,
    /// path, status, country, …). `queries[]` filters the underlying events
    /// using the standard Utopia query syntax — `equal("path",
    /// ["/v1/storage/files"])`, `equal("resource", ["bucket"])`,
    /// `equal("resourceId", ["abc123"])`, `startsWith("path", ["/v1/storage"])`,
    /// `equal("status", ["200", "201"])`, `isNotNull("resourceId")`. Supported
    /// attributes: see `queries[]` param. Supported methods: `equal`, `notEqual`,
    /// `contains`, `startsWith`, `endsWith`, `isNull`, `isNotNull`. Pass multiple
    /// metrics to render stacked charts in one round-trip.
    /// `orderBy=value`+`orderDir=desc`+`limit=N` returns the top-N by aggregated
    /// value. When `startAt` is omitted, the default window adapts to `interval`
    /// (or 7d when interval is omitted).
    #[allow(clippy::too_many_arguments)]
    pub async fn list_events(
        &self,
        metrics: impl IntoIterator<Item = impl Into<String>>,
        queries: Option<Vec<String>>,
        interval: Option<&str>,
        dimensions: Option<Vec<String>>,
        start_at: Option<&str>,
        end_at: Option<&str>,
        order_by: Option<&str>,
        order_dir: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> crate::error::Result<crate::models::UsageEventList> {
        let mut params = HashMap::new();
        params.insert("metrics".to_string(), json!(metrics.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = dimensions {
            params.insert("dimensions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = start_at {
            params.insert("startAt".to_string(), json!(value));
        }
        if let Some(value) = end_at {
            params.insert("endAt".to_string(), json!(value));
        }
        if let Some(value) = order_by {
            params.insert("orderBy".to_string(), json!(value));
        }
        if let Some(value) = order_dir {
            params.insert("orderDir".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/usage/events".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

    /// Aggregate usage gauge snapshots. Gauges are point-in-time values (storage
    /// totals, resource counts, …); each point carries the latest snapshot in
    /// its interval via `argMax(value, time)`. `metrics[]` (1-10) is required; the
    /// response always contains one entry per requested metric, each with its own
    /// `points[]` time series.
    /// 
    /// **Two response shapes**:
    /// - Omit `interval` for a flat top-N table — `argMax(value, time)` per
    /// dimension combination over the whole window, no time axis. Useful for "top
    /// 10 resources by current storage".
    /// - Pass `interval` (`1m`, `15m`, `30m`, `1h`, `1d`) for a time series —
    /// one snapshot per (time bucket × dimension combination).
    /// 
    /// `dimensions[]` breaks each point down further. Supported on gauges:
    /// `resourceId`, `teamId`, `service`, `resource`. `service` and `resource`
    /// enable per-service / per-resource-type panels (e.g. storage-by-service:
    /// group `files.storage`, `deployments.storage`, `builds.storage`,
    /// `databases.storage` by `service`). `queries[]` filters the underlying rows
    /// using the standard Utopia query syntax — `equal("resource", ["bucket"])`,
    /// `equal("resourceId", ["abc123"])`, `equal("teamId", ["team_x"])`,
    /// `isNotNull("teamId")`. Supported attributes: see `queries[]` param.
    /// Supported methods: `equal`, `notEqual`, `isNull`, `isNotNull`. Pass
    /// multiple metrics to render stacked charts in one round-trip.
    /// `orderBy=value`+`orderDir=desc`+`limit=N` returns the top-N. When `startAt`
    /// is omitted, the default window adapts to interval (or 7d when interval is
    /// omitted).
    #[allow(clippy::too_many_arguments)]
    pub async fn list_gauges(
        &self,
        metrics: impl IntoIterator<Item = impl Into<String>>,
        queries: Option<Vec<String>>,
        interval: Option<&str>,
        dimensions: Option<Vec<String>>,
        start_at: Option<&str>,
        end_at: Option<&str>,
        order_by: Option<&str>,
        order_dir: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> crate::error::Result<crate::models::UsageGaugeList> {
        let mut params = HashMap::new();
        params.insert("metrics".to_string(), json!(metrics.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = interval {
            params.insert("interval".to_string(), json!(value));
        }
        if let Some(value) = dimensions {
            params.insert("dimensions".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = start_at {
            params.insert("startAt".to_string(), json!(value));
        }
        if let Some(value) = end_at {
            params.insert("endAt".to_string(), json!(value));
        }
        if let Some(value) = order_by {
            params.insert("orderBy".to_string(), json!(value));
        }
        if let Some(value) = order_dir {
            params.insert("orderDir".to_string(), json!(value));
        }
        if let Some(value) = limit {
            params.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = offset {
            params.insert("offset".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/usage/gauges".to_string();

        self.client.call(Method::GET, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Usage {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_creation() {
        let client = Client::new();
        let service = Usage::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
