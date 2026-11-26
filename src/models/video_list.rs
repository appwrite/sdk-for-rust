//! VideoList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Videos List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoList {
    /// Total number of videos that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of videos.
    #[serde(rename = "videos")]
    #[serde(default = "default_videos")]
    pub videos: Vec<crate::models::Video>,
}

fn default_total() -> i64 {
    0
}

fn default_videos() -> Vec<crate::models::Video> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl VideoList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get videos
    pub fn videos(&self) -> &Vec<crate::models::Video> {
        &self.videos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_list_creation() {
        let _model = <VideoList as Default>::default();
        let _ = _model.total();
        let _ = _model.videos();
    }

    #[test]
    fn test_video_list_serialization() {
        let model = <VideoList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
