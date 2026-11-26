//! VideoProfileList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Video Profiles List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct VideoProfileList {
    /// Total number of profiles that matched your query.
    #[serde(rename = "total")]
    #[serde(default = "default_total")]
    pub total: i64,
    /// List of profiles.
    #[serde(rename = "profiles")]
    #[serde(default = "default_profiles")]
    pub profiles: Vec<crate::models::VideoProfile>,
}

fn default_total() -> i64 {
    0
}

fn default_profiles() -> Vec<crate::models::VideoProfile> {
    serde_json::from_value(serde_json::json!([])).unwrap()
}

impl VideoProfileList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get profiles
    pub fn profiles(&self) -> &Vec<crate::models::VideoProfile> {
        &self.profiles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_profile_list_creation() {
        let _model = <VideoProfileList as Default>::default();
        let _ = _model.total();
        let _ = _model.profiles();
    }

    #[test]
    fn test_video_profile_list_serialization() {
        let model = <VideoProfileList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<VideoProfileList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
