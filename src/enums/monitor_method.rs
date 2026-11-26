use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorMethod {
    #[serde(rename = "GET")]
    #[default]
    GET,
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "OPTIONS")]
    OPTIONS,
}

impl MonitorMethod {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorMethod::GET => "GET",
            MonitorMethod::HEAD => "HEAD",
            MonitorMethod::POST => "POST",
            MonitorMethod::PUT => "PUT",
            MonitorMethod::PATCH => "PATCH",
            MonitorMethod::DELETE => "DELETE",
            MonitorMethod::OPTIONS => "OPTIONS",
        }
    }
}

impl std::fmt::Display for MonitorMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
