//! AnalyticsMetric model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// AnalyticsMetric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct AnalyticsMetric {
    /// Dimension value this row covers. Null when no dimension was requested,
    /// empty when the dimension could not be derived for those events.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Start of the time bucket this row covers, in ISO 8601. Null when no
    /// interval was requested.
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Unique visitors in the requested range.
    #[serde(rename = "visitors")]
    #[serde(default = "default_visitors")]
    pub visitors: i64,
    /// Unique sessions in the requested range.
    #[serde(rename = "sessions")]
    #[serde(default = "default_sessions")]
    pub sessions: i64,
    /// Total page and screen view events (events named "pageview" or
    /// "screen_view") in the requested range. The flat aggregate and the time
    /// series compute it; null on breakdown rows.
    #[serde(rename = "pageviews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageviews: Option<i64>,
    /// Total events in the requested range.
    #[serde(rename = "events")]
    #[serde(default = "default_events")]
    pub events: i64,
    /// Unique identified users in the requested range. Anonymous events are not
    /// counted. Null on entry and exit page breakdown rows.
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<i64>,
    /// Total sessions with activity in the requested range. Only the flat
    /// aggregate computes it; null on breakdown and time-series rows.
    #[serde(rename = "visits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visits: Option<i64>,
    /// Share of bounced visits, as a percentage; a visit is a bounce unless it
    /// records two or more page or screen views, at least one interaction, or ten
    /// seconds or more of activity. An interaction is any event that is neither a
    /// view nor automatic measurement such as "app_foregrounded",
    /// "engagement_time" or "scroll_depth". Only the flat aggregate computes it;
    /// null on breakdown and time-series rows.
    #[serde(rename = "bounceRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_rate: Option<f64>,
    /// Average session length in seconds, measured first event to last event.
    /// Single-event sessions count as 0. Only the flat aggregate computes it; null
    /// on breakdown and time-series rows.
    #[serde(rename = "visitDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_duration: Option<f64>,
    /// Average pageviews per session (pageviews divided by visits). Only the flat
    /// aggregate computes it; null on breakdown and time-series rows.
    #[serde(rename = "viewsPerVisit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub views_per_visit: Option<f64>,
    /// Average scroll depth across events, as a percentage. Only the flat
    /// aggregate computes it; null on breakdown and time-series rows.
    #[serde(rename = "scrollDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_depth: Option<f64>,
    /// Average engaged time per session in seconds, counting only foreground time.
    /// Sessions that reported no engagement are excluded. Only the flat aggregate
    /// computes it; null on breakdown and time-series rows.
    #[serde(rename = "engagementTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement_time: Option<f64>,
}

fn default_visitors() -> i64 {
    0
}

fn default_sessions() -> i64 {
    0
}

fn default_events() -> i64 {
    0
}

impl AnalyticsMetric {
    /// Set value
    pub fn set_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Get value
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    /// Set date
    pub fn set_date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    /// Get date
    pub fn date(&self) -> Option<&String> {
        self.date.as_ref()
    }

    /// Get visitors
    pub fn visitors(&self) -> &i64 {
        &self.visitors
    }

    /// Get sessions
    pub fn sessions(&self) -> &i64 {
        &self.sessions
    }

    /// Set pageviews
    pub fn set_pageviews(mut self, pageviews: i64) -> Self {
        self.pageviews = Some(pageviews);
        self
    }

    /// Get pageviews
    pub fn pageviews(&self) -> Option<&i64> {
        self.pageviews.as_ref()
    }

    /// Get events
    pub fn events(&self) -> &i64 {
        &self.events
    }

    /// Set users
    pub fn set_users(mut self, users: i64) -> Self {
        self.users = Some(users);
        self
    }

    /// Get users
    pub fn users(&self) -> Option<&i64> {
        self.users.as_ref()
    }

    /// Set visits
    pub fn set_visits(mut self, visits: i64) -> Self {
        self.visits = Some(visits);
        self
    }

    /// Get visits
    pub fn visits(&self) -> Option<&i64> {
        self.visits.as_ref()
    }

    /// Set bounce_rate
    pub fn set_bounce_rate(mut self, bounce_rate: f64) -> Self {
        self.bounce_rate = Some(bounce_rate);
        self
    }

    /// Get bounce_rate
    pub fn bounce_rate(&self) -> Option<&f64> {
        self.bounce_rate.as_ref()
    }

    /// Set visit_duration
    pub fn set_visit_duration(mut self, visit_duration: f64) -> Self {
        self.visit_duration = Some(visit_duration);
        self
    }

    /// Get visit_duration
    pub fn visit_duration(&self) -> Option<&f64> {
        self.visit_duration.as_ref()
    }

    /// Set views_per_visit
    pub fn set_views_per_visit(mut self, views_per_visit: f64) -> Self {
        self.views_per_visit = Some(views_per_visit);
        self
    }

    /// Get views_per_visit
    pub fn views_per_visit(&self) -> Option<&f64> {
        self.views_per_visit.as_ref()
    }

    /// Set scroll_depth
    pub fn set_scroll_depth(mut self, scroll_depth: f64) -> Self {
        self.scroll_depth = Some(scroll_depth);
        self
    }

    /// Get scroll_depth
    pub fn scroll_depth(&self) -> Option<&f64> {
        self.scroll_depth.as_ref()
    }

    /// Set engagement_time
    pub fn set_engagement_time(mut self, engagement_time: f64) -> Self {
        self.engagement_time = Some(engagement_time);
        self
    }

    /// Get engagement_time
    pub fn engagement_time(&self) -> Option<&f64> {
        self.engagement_time.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_metric_creation() {
        let _model = <AnalyticsMetric as Default>::default();
        let _ = _model.visitors();
        let _ = _model.sessions();
        let _ = _model.events();
    }

    #[test]
    fn test_analytics_metric_serialization() {
        let model = <AnalyticsMetric as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<AnalyticsMetric, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
