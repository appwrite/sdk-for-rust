use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ImageGravity {
    #[serde(rename = "center")]
    #[default]
    Center,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom-right")]
    BottomRight,
}

impl ImageGravity {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ImageGravity::Center => "center",
            ImageGravity::TopLeft => "top-left",
            ImageGravity::Top => "top",
            ImageGravity::TopRight => "top-right",
            ImageGravity::Left => "left",
            ImageGravity::Right => "right",
            ImageGravity::BottomLeft => "bottom-left",
            ImageGravity::Bottom => "bottom",
            ImageGravity::BottomRight => "bottom-right",
        }
    }
}

impl std::fmt::Display for ImageGravity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
