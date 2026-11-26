//! VideoCodecList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video Codecs List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoCodecList {
    /// Total number of codecs that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of codecs.
    #[serde(rename = "codecs")]
    #[serde(default = "default_codecs")]
    pub codecs: Vec<crate::models::VideoCodec>,
}

fn default_total() -> i64 {
    0
}

fn default_codecs() -> Vec<crate::models::VideoCodec> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl VideoCodecList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get codecs
    pub fn codecs(&self) -> &Vec<crate::models::VideoCodec> {
        &self.codecs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_list_creation() {
        let _model = <VideoCodecList as Default>::default();
        let _ = _model.total();
        let _ = _model.codecs();
    }

    #[test]
    fn test_video_codec_list_serialization() {
        let model = <VideoCodecList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoCodecList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
