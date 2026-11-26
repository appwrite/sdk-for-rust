//! VideoRendition model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video rendition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoRendition {
    /// Rendition ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Rendition creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Rendition update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Video ID.
    #[serde(rename = "videoId")]
    #[serde(default = "default_video_id")]
    pub video_id: String,
    /// Video profile ID this rendition was encoded against.
    #[serde(rename = "profileId")]
    #[serde(default = "default_profile_id")]
    pub profile_id: String,
    /// Rendition name, derived from its dimensions and bitrate.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Transcoding start time in ISO 8601 format.
    #[serde(rename = "startedAt")]
    #[serde(default = "default_started_at")]
    pub started_at: String,
    /// Transcoding end time in ISO 8601 format.
    #[serde(rename = "endedAt")]
    #[serde(default = "default_ended_at")]
    pub ended_at: String,
    /// Rendition width in pixels.
    #[serde(rename = "width")]
    #[serde(default = "default_width")]
    pub width: i64,
    /// Rendition height in pixels.
    #[serde(rename = "height")]
    #[serde(default = "default_height")]
    pub height: i64,
    /// Video bitrate in bits per second.
    #[serde(rename = "videoBitRate")]
    #[serde(default = "default_video_bit_rate")]
    pub video_bit_rate: i64,
    /// Audio bitrate in bits per second.
    #[serde(rename = "audioBitRate")]
    #[serde(default = "default_audio_bit_rate")]
    pub audio_bit_rate: i64,
    /// Longest segment duration in seconds.
    #[serde(rename = "targetDuration")]
    #[serde(default = "default_target_duration")]
    pub target_duration: f64,
    /// Transcoding status: one of `pending`, `started`, `ended`, `ready`, `error`
    /// or `aborted`.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: crate::enums::VideoRenditionStatus,
    /// Why the rendition failed or was aborted. Empty unless `status` is `error`
    /// or `aborted`.
    #[serde(rename = "error")]
    #[serde(default = "default_error")]
    pub error: String,
    /// Transcoding progress as a percentage, from 0 to 100.
    #[serde(rename = "progress")]
    #[serde(default = "default_progress")]
    pub progress: i64,
    /// Streaming output format: `hls`, `dash`, or `cmaf`.
    #[serde(rename = "output")]
    #[serde(default = "default_output")]
    pub output: crate::enums::VideoOutput,
    /// Video encode codec snapshotted from the profile at create time: one of
    /// `h264`, `hevc`, or `vp9`.
    #[serde(rename = "codec")]
    #[serde(default = "default_codec")]
    pub codec: crate::enums::VideoCodecId,
    /// Stored rendition size in bytes.
    #[serde(rename = "size")]
    #[serde(default = "default_size")]
    pub size: i64,
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

fn default_video_id() -> String {
    "".to_string()
}

fn default_profile_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_started_at() -> String {
    "".to_string()
}

fn default_ended_at() -> String {
    "".to_string()
}

fn default_width() -> i64 {
    0
}

fn default_height() -> i64 {
    0
}

fn default_video_bit_rate() -> i64 {
    0
}

fn default_audio_bit_rate() -> i64 {
    0
}

fn default_target_duration() -> f64 {
    0.0
}

fn default_status() -> crate::enums::VideoRenditionStatus {
    serde_json::from_value(serde_json::json!("pending")).unwrap()
}

fn default_error() -> String {
    "".to_string()
}

fn default_progress() -> i64 {
    0
}

fn default_output() -> crate::enums::VideoOutput {
    serde_json::from_value(serde_json::json!("hls")).unwrap()
}

fn default_codec() -> crate::enums::VideoCodecId {
    serde_json::from_value(serde_json::json!("h264")).unwrap()
}

fn default_size() -> i64 {
    0
}

impl VideoRendition {
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

    /// Get video_id
    pub fn video_id(&self) -> &String {
        &self.video_id
    }

    /// Get profile_id
    pub fn profile_id(&self) -> &String {
        &self.profile_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get started_at
    pub fn started_at(&self) -> &String {
        &self.started_at
    }

    /// Get ended_at
    pub fn ended_at(&self) -> &String {
        &self.ended_at
    }

    /// Get width
    pub fn width(&self) -> &i64 {
        &self.width
    }

    /// Get height
    pub fn height(&self) -> &i64 {
        &self.height
    }

    /// Get video_bit_rate
    pub fn video_bit_rate(&self) -> &i64 {
        &self.video_bit_rate
    }

    /// Get audio_bit_rate
    pub fn audio_bit_rate(&self) -> &i64 {
        &self.audio_bit_rate
    }

    /// Get target_duration
    pub fn target_duration(&self) -> &f64 {
        &self.target_duration
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::VideoRenditionStatus {
        &self.status
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
    }

    /// Get progress
    pub fn progress(&self) -> &i64 {
        &self.progress
    }

    /// Get output
    pub fn output(&self) -> &crate::enums::VideoOutput {
        &self.output
    }

    /// Get codec
    pub fn codec(&self) -> &crate::enums::VideoCodecId {
        &self.codec
    }

    /// Get size
    pub fn size(&self) -> &i64 {
        &self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_rendition_creation() {
        let _model = <VideoRendition as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.video_id();
        let _ = _model.profile_id();
        let _ = _model.name();
        let _ = _model.started_at();
        let _ = _model.ended_at();
        let _ = _model.width();
        let _ = _model.height();
        let _ = _model.video_bit_rate();
        let _ = _model.audio_bit_rate();
        let _ = _model.target_duration();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.progress();
        let _ = _model.output();
        let _ = _model.codec();
        let _ = _model.size();
    }

    #[test]
    fn test_video_rendition_serialization() {
        let model = <VideoRendition as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoRendition, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
