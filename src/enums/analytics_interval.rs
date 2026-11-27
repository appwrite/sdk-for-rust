use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnalyticsInterval {
    #[serde(rename = "1h")]
    #[default]
    OneHour,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1m")]
    OneMonth,
}

impl AnalyticsInterval {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AnalyticsInterval::OneHour => "1h",
            AnalyticsInterval::OneDay => "1d",
            AnalyticsInterval::OneWeek => "1w",
            AnalyticsInterval::OneMonth => "1m",
        }
    }
}

impl std::fmt::Display for AnalyticsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
