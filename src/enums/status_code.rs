use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum StatusCode {
    #[serde(rename = "301")]
    #[default]
    MovedPermanently301,
    #[serde(rename = "302")]
    Found302,
    #[serde(rename = "307")]
    TemporaryRedirect307,
    #[serde(rename = "308")]
    PermanentRedirect308,
}

impl StatusCode {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::MovedPermanently301 => "301",
            StatusCode::Found302 => "302",
            StatusCode::TemporaryRedirect307 => "307",
            StatusCode::PermanentRedirect308 => "308",
        }
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
