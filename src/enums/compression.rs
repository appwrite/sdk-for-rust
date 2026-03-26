use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Compression {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "zstd")]
    Zstd,
}

impl Compression {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Compression::None => "none",
            Compression::Gzip => "gzip",
            Compression::Zstd => "zstd",
        }
    }
}

impl std::fmt::Display for Compression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
