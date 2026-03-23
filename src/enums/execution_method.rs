use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ExecutionMethod {
    #[serde(rename = "GET")]
    #[default]
    GET,
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
    #[serde(rename = "HEAD")]
    HEAD,
}

impl ExecutionMethod {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ExecutionMethod::GET => "GET",
            ExecutionMethod::POST => "POST",
            ExecutionMethod::PUT => "PUT",
            ExecutionMethod::PATCH => "PATCH",
            ExecutionMethod::DELETE => "DELETE",
            ExecutionMethod::OPTIONS => "OPTIONS",
            ExecutionMethod::HEAD => "HEAD",
        }
    }
}

impl std::fmt::Display for ExecutionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
