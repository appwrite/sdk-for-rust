//! VideoRenditionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video Renditions List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoRenditionList {
    /// Total number of renditions that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of renditions.
    #[serde(rename = "renditions")]
    #[serde(default = "default_renditions")]
    pub renditions: Vec<crate::models::VideoRendition>,
}

fn default_total() -> i64 {
    0
}

fn default_renditions() -> Vec<crate::models::VideoRendition> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl VideoRenditionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get renditions
    pub fn renditions(&self) -> &Vec<crate::models::VideoRendition> {
        &self.renditions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_rendition_list_creation() {
        let _model = <VideoRenditionList as Default>::default();
        let _ = _model.total();
        let _ = _model.renditions();
    }

    #[test]
    fn test_video_rendition_list_serialization() {
        let model = <VideoRenditionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoRenditionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
