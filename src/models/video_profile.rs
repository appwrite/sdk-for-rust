//! VideoProfile model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video profile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoProfile {
    /// Video profile ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Video profile creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Video profile update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Video profile name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Video encode codec: one of `h264`, `hevc`, or `vp9`.
    #[serde(rename = "codec")]
    #[serde(default = "default_codec")]
    pub codec: crate::enums::VideoCodecId,
    /// Target video bitrate in bits per second.
    #[serde(rename = "videoBitRate")]
    #[serde(default = "default_video_bit_rate")]
    pub video_bit_rate: i64,
    /// Target audio bitrate in bits per second.
    #[serde(rename = "audioBitRate")]
    #[serde(default = "default_audio_bit_rate")]
    pub audio_bit_rate: i64,
    /// Target video width in pixels.
    #[serde(rename = "width")]
    #[serde(default = "default_width")]
    pub width: i64,
    /// Target video height in pixels.
    #[serde(rename = "height")]
    #[serde(default = "default_height")]
    pub height: i64,
}

fn default_id() -> String {
    "".to_string()
}

fn default_created_at() -> String {
    "".to_string()
}

fn default_updated_at() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_codec() -> crate::enums::VideoCodecId {
    serde_json::from_value(serde_json::json!("h264")).unwrap()
}

fn default_video_bit_rate() -> i64 {
    0
}

fn default_audio_bit_rate() -> i64 {
    0
}

fn default_width() -> i64 {
    0
}

fn default_height() -> i64 {
    0
}

impl VideoProfile {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get codec
    pub fn codec(&self) -> &crate::enums::VideoCodecId {
        &self.codec
    }

    /// Get video_bit_rate
    pub fn video_bit_rate(&self) -> &i64 {
        &self.video_bit_rate
    }

    /// Get audio_bit_rate
    pub fn audio_bit_rate(&self) -> &i64 {
        &self.audio_bit_rate
    }

    /// Get width
    pub fn width(&self) -> &i64 {
        &self.width
    }

    /// Get height
    pub fn height(&self) -> &i64 {
        &self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_profile_creation() {
        let _model = <VideoProfile as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.codec();
        let _ = _model.video_bit_rate();
        let _ = _model.audio_bit_rate();
        let _ = _model.width();
        let _ = _model.height();
    }

    #[test]
    fn test_video_profile_serialization() {
        let model = <VideoProfile as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoProfile, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
