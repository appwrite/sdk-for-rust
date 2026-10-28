use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OnDuplicate {
    #[serde(rename = "fail")]
    #[default]
    Fail,
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "overwrite")]
    Overwrite,
}

impl OnDuplicate {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OnDuplicate::Fail => "fail",
            OnDuplicate::Skip => "skip",
            OnDuplicate::Overwrite => "overwrite",
        }
    }
}

impl std::fmt::Display for OnDuplicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
