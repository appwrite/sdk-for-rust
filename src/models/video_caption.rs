//! VideoCaption model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video caption
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoCaption {
    /// Caption ID.
    #[serde(rename = "$id")]
    #[serde(default = "default_id")]
    pub id: String,
    /// Caption creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    #[serde(default = "default_created_at")]
    pub created_at: String,
    /// Caption update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    #[serde(default = "default_updated_at")]
    pub updated_at: String,
    /// Video ID.
    #[serde(rename = "videoId")]
    #[serde(default = "default_video_id")]
    pub video_id: String,
    /// Storage bucket ID holding the caption file.
    #[serde(rename = "bucketId")]
    #[serde(default = "default_bucket_id")]
    pub bucket_id: String,
    /// Caption file ID.
    #[serde(rename = "fileId")]
    #[serde(default = "default_file_id")]
    pub file_id: String,
    /// Caption display name.
    #[serde(rename = "name")]
    #[serde(default = "default_name")]
    pub name: String,
    /// Caption ISO 639-2 language code.
    #[serde(rename = "language")]
    #[serde(default = "default_language")]
    pub language: String,
    /// Is this the default caption track?
    #[serde(rename = "default")]
    #[serde(default = "default_default")]
    pub default: bool,
    /// Was this track auto-extracted from the source container? Extracted tracks
    /// have no backing file; extraction runs once per video, so a deleted
    /// extracted track is not re-created.
    #[serde(rename = "embedded")]
    #[serde(default = "default_embedded")]
    pub embedded: bool,
    /// Longest segment duration in seconds.
    #[serde(rename = "targetDuration")]
    #[serde(default = "default_target_duration")]
    pub target_duration: f64,
    /// Packaging status: one of `pending`, `started`, `ready`, `error` or
    /// `aborted`.
    #[serde(rename = "status")]
    #[serde(default = "default_status")]
    pub status: crate::enums::VideoCaptionStatus,
    /// Why the caption failed or was aborted. Empty unless `status` is `error` or
    /// `aborted`.
    #[serde(rename = "error")]
    #[serde(default = "default_error")]
    pub error: String,
    /// Stored caption size in bytes.
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

fn default_bucket_id() -> String {
    "".to_string()
}

fn default_file_id() -> String {
    "".to_string()
}

fn default_name() -> String {
    "".to_string()
}

fn default_language() -> String {
    "".to_string()
}

fn default_default() -> bool {
    false
}

fn default_embedded() -> bool {
    false
}

fn default_target_duration() -> f64 {
    0.0
}

fn default_status() -> crate::enums::VideoCaptionStatus {
    serde_json::from_value(serde_json::json!("pending")).unwrap()
}

fn default_error() -> String {
    "".to_string()
}

fn default_size() -> i64 {
    0
}

impl VideoCaption {
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

    /// Get language
    pub fn language(&self) -> &String {
        &self.language
    }

    /// Get default
    pub fn default(&self) -> &bool {
        &self.default
    }

    /// Get embedded
    pub fn embedded(&self) -> &bool {
        &self.embedded
    }

    /// Get target_duration
    pub fn target_duration(&self) -> &f64 {
        &self.target_duration
    }

    /// Get status
    pub fn status(&self) -> &crate::enums::VideoCaptionStatus {
        &self.status
    }

    /// Get error
    pub fn error(&self) -> &String {
        &self.error
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
    fn test_video_caption_creation() {
        let _model = <VideoCaption as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.video_id();
        let _ = _model.bucket_id();
        let _ = _model.file_id();
        let _ = _model.name();
        let _ = _model.language();
        let _ = _model.default();
        let _ = _model.embedded();
        let _ = _model.target_duration();
        let _ = _model.status();
        let _ = _model.error();
        let _ = _model.size();
    }

    #[test]
    fn test_video_caption_serialization() {
        let model = <VideoCaption as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoCaption, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
