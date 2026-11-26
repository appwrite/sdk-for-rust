//! VideoCodec model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video codec
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoCodec {
    /// Codec ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Codec display name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Streaming output formats this codec may be packaged into.
    #[serde(rename = "outputs")]
    #[serde(default = "default_outputs")]
    pub outputs: Vec<crate::enums::VideoOutput>,
}

fn default_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_outputs() -> Vec<crate::enums::VideoOutput> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl VideoCodec {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get outputs
    pub fn outputs(&self) -> &Vec<crate::enums::VideoOutput> {
        &self.outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_creation() {
        let _model = <VideoCodec as Default>::default();
        let _ = _model.id();
        let _ = _model.name();
        let _ = _model.outputs();
    }

    #[test]
    fn test_video_codec_serialization() {
        let model = <VideoCodec as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoCodec, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
