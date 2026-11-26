//! Video model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Video {
    /// Video ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Video creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Video update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Video permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    #[serde(default = "default_permissions")]
    pub permissions: Vec<String>,
    /// Storage bucket ID holding the source file.
    #[serde(rename = "bucketId")]
    #[serde(default = "default_bucket_id")]
    pub bucket_id: String,
    /// Source file ID.
    #[serde(rename = "fileId")]
    #[serde(default = "default_file_id")]
    pub file_id: String,
    /// Video name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Source file size in bytes.
    #[serde(rename = "size")]
    #[serde(default = "default_size")]
    pub size: i64,
    /// Container format.
    #[serde(rename = "format")]
    #[serde(default = "default_format")]
    pub format: String,
    /// Video duration in milliseconds.
    #[serde(rename = "duration")]
    #[serde(default = "default_duration")]
    pub duration: i64,
    /// Video width in pixels.
    #[serde(rename = "width")]
    #[serde(default = "default_width")]
    pub width: i64,
    /// Video height in pixels.
    #[serde(rename = "height")]
    #[serde(default = "default_height")]
    pub height: i64,
    /// Video aspect ratio.
    #[serde(rename = "aspectRatio")]
    #[serde(default = "default_aspect_ratio")]
    pub aspect_ratio: String,
    /// Video codec.
    #[serde(rename = "videoCodec")]
    #[serde(default = "default_video_codec")]
    pub video_codec: String,
    /// Video format.
    #[serde(rename = "videoFormat")]
    #[serde(default = "default_video_format")]
    pub video_format: String,
    /// Video format profile.
    #[serde(rename = "videoFormatProfile")]
    #[serde(default = "default_video_format_profile")]
    pub video_format_profile: String,
    /// Video bitrate in bits per second.
    #[serde(rename = "videoBitRate")]
    #[serde(default = "default_video_bit_rate")]
    pub video_bit_rate: i64,
    /// Video frame rate in frames per second.
    #[serde(rename = "videoFrameRate")]
    #[serde(default = "default_video_frame_rate")]
    pub video_frame_rate: f64,
    /// Video frame rate mode.
    #[serde(rename = "videoFrameRateMode")]
    #[serde(default = "default_video_frame_rate_mode")]
    pub video_frame_rate_mode: String,
    /// Audio codec.
    #[serde(rename = "audioCodec")]
    #[serde(default = "default_audio_codec")]
    pub audio_codec: String,
    /// Audio format.
    #[serde(rename = "audioFormat")]
    #[serde(default = "default_audio_format")]
    pub audio_format: String,
    /// Audio bitrate in bits per second.
    #[serde(rename = "audioBitRate")]
    #[serde(default = "default_audio_bit_rate")]
    pub audio_bit_rate: i64,
    /// Audio sample rate in hertz.
    #[serde(rename = "audioSampleRate")]
    #[serde(default = "default_audio_sample_rate")]
    pub audio_sample_rate: i64,
    /// Thumbnail timeline job status: one of `pending`, `started`, `ready`,
    /// `skipped` (source has no video track) or `error`.
    #[serde(rename = "timelineStatus")]
    #[serde(default = "default_timeline_status")]
    pub timeline_status: crate::enums::VideoTimelineStatus,
    /// Why the timeline job failed. Empty unless `timelineStatus` is `error`.
    #[serde(rename = "timelineError")]
    #[serde(default = "default_timeline_error")]
    pub timeline_error: String,
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

fn default_permissions() -> Vec<String> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

fn default_bucket_id() -> String {
    "".to_string()
}

fn default_file_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_size() -> i64 {
    0
}

fn default_format() -> String {
    "".to_string()
}

fn default_duration() -> i64 {
    0
}

fn default_width() -> i64 {
    0
}

fn default_height() -> i64 {
    0
}

fn default_aspect_ratio() -> String {
    "".to_string()
}

fn default_video_codec() -> String {
    "".to_string()
}

fn default_video_format() -> String {
    "".to_string()
}

fn default_video_format_profile() -> String {
    "".to_string()
}

fn default_video_bit_rate() -> i64 {
    0
}

fn default_video_frame_rate() -> f64 {
    0.0
}

fn default_video_frame_rate_mode() -> String {
    "".to_string()
}

fn default_audio_codec() -> String {
    "".to_string()
}

fn default_audio_format() -> String {
    "".to_string()
}

fn default_audio_bit_rate() -> i64 {
    0
}

fn default_audio_sample_rate() -> i64 {
    0
}

fn default_timeline_status() -> crate::enums::VideoTimelineStatus {
    serde_json::from_value(serde_json::json!("pending")).unwrap()
}

fn default_timeline_error() -> String {
    "".to_string()
}

impl Video {
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

    /// Get permissions
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    /// Get bucket_id
    pub fn bucket_id(&self) -> &String {
        &self.bucket_id
    }

    /// Get file_id
    pub fn file_id(&self) -> &String {
        &self.file_id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get size
    pub fn size(&self) -> &i64 {
        &self.size
    }

    /// Get format
    pub fn format(&self) -> &String {
        &self.format
    }

    /// Get duration
    pub fn duration(&self) -> &i64 {
        &self.duration
    }

    /// Get width
    pub fn width(&self) -> &i64 {
        &self.width
    }

    /// Get height
    pub fn height(&self) -> &i64 {
        &self.height
    }

    /// Get aspect_ratio
    pub fn aspect_ratio(&self) -> &String {
        &self.aspect_ratio
    }

    /// Get video_codec
    pub fn video_codec(&self) -> &String {
        &self.video_codec
    }

    /// Get video_format
    pub fn video_format(&self) -> &String {
        &self.video_format
    }

    /// Get video_format_profile
    pub fn video_format_profile(&self) -> &String {
        &self.video_format_profile
    }

    /// Get video_bit_rate
    pub fn video_bit_rate(&self) -> &i64 {
        &self.video_bit_rate
    }

    /// Get video_frame_rate
    pub fn video_frame_rate(&self) -> &f64 {
        &self.video_frame_rate
    }

    /// Get video_frame_rate_mode
    pub fn video_frame_rate_mode(&self) -> &String {
        &self.video_frame_rate_mode
    }

    /// Get audio_codec
    pub fn audio_codec(&self) -> &String {
        &self.audio_codec
    }

    /// Get audio_format
    pub fn audio_format(&self) -> &String {
        &self.audio_format
    }

    /// Get audio_bit_rate
    pub fn audio_bit_rate(&self) -> &i64 {
        &self.audio_bit_rate
    }

    /// Get audio_sample_rate
    pub fn audio_sample_rate(&self) -> &i64 {
        &self.audio_sample_rate
    }

    /// Get timeline_status
    pub fn timeline_status(&self) -> &crate::enums::VideoTimelineStatus {
        &self.timeline_status
    }

    /// Get timeline_error
    pub fn timeline_error(&self) -> &String {
        &self.timeline_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_creation() {
        let _model = <Video as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.permissions();
        let _ = _model.bucket_id();
        let _ = _model.file_id();
        let _ = _model.name();
        let _ = _model.size();
        let _ = _model.format();
        let _ = _model.duration();
        let _ = _model.width();
        let _ = _model.height();
        let _ = _model.aspect_ratio();
        let _ = _model.video_codec();
        let _ = _model.video_format();
        let _ = _model.video_format_profile();
        let _ = _model.video_bit_rate();
        let _ = _model.video_frame_rate();
        let _ = _model.video_frame_rate_mode();
        let _ = _model.audio_codec();
        let _ = _model.audio_format();
        let _ = _model.audio_bit_rate();
        let _ = _model.audio_sample_rate();
        let _ = _model.timeline_status();
        let _ = _model.timeline_error();
    }

    #[test]
    fn test_video_serialization() {
        let model = <Video as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Video, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
