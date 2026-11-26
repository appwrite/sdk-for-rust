//! VideoCaptionList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video Captions List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoCaptionList {
    /// Total number of captions that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of captions.
    #[serde(rename = "captions")]
    #[serde(default = "default_captions")]
    pub captions: Vec<crate::models::VideoCaption>,
}

fn default_total() -> i64 {
    0
}

fn default_captions() -> Vec<crate::models::VideoCaption> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl VideoCaptionList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get captions
    pub fn captions(&self) -> &Vec<crate::models::VideoCaption> {
        &self.captions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_caption_list_creation() {
        let _model = <VideoCaptionList as Default>::default();
        let _ = _model.total();
        let _ = _model.captions();
    }

    #[test]
    fn test_video_caption_list_serialization() {
        let model = <VideoCaptionList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoCaptionList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
