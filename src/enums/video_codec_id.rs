use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VideoCodecId {
    #[serde(rename = "h264")]
    #[default]
    H264,
    #[serde(rename = "hevc")]
    Hevc,
    #[serde(rename = "vp9")]
    Vp9,
}

impl VideoCodecId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VideoCodecId::H264 => "h264",
            VideoCodecId::Hevc => "hevc",
            VideoCodecId::Vp9 => "vp9",
        }
    }
}

impl std::fmt::Display for VideoCodecId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
