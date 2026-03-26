use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ImageFormat {
    #[serde(rename = "jpg")]
    #[default]
    Jpg,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "heic")]
    Heic,
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "gif")]
    Gif,
}

impl ImageFormat {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ImageFormat::Jpg => "jpg",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Png => "png",
            ImageFormat::Webp => "webp",
            ImageFormat::Heic => "heic",
            ImageFormat::Avif => "avif",
            ImageFormat::Gif => "gif",
        }
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
