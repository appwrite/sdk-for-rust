//! ReportList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Reports List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ReportList {
    /// Total number of reports that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of reports.
    #[serde(rename = "reports")]
    pub reports: Vec<crate::models::Report>,
}

impl ReportList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get reports
    pub fn reports(&self) -> &Vec<crate::models::Report> {
        &self.reports
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_list_creation() {
        let _model = <ReportList as Default>::default();
        let _ = _model.total();
        let _ = _model.reports();
    }

    #[test]
    fn test_report_list_serialization() {
        let model = <ReportList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<ReportList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
