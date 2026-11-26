use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VideoOutput {
    #[serde(rename = "hls")]
    #[default]
    Hls,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "cmaf")]
    Cmaf,
}

impl VideoOutput {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VideoOutput::Hls => "hls",
            VideoOutput::Dash => "dash",
            VideoOutput::Cmaf => "cmaf",
        }
    }
}

impl std::fmt::Display for VideoOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
