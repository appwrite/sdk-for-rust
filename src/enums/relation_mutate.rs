use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RelationMutate {
    #[serde(rename = "cascade")]
    #[default]
    Cascade,
    #[serde(rename = "restrict")]
    Restrict,
    #[serde(rename = "setNull")]
    SetNull,
}

impl RelationMutate {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            RelationMutate::Cascade => "cascade",
            RelationMutate::Restrict => "restrict",
            RelationMutate::SetNull => "setNull",
        }
    }
}

impl std::fmt::Display for RelationMutate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
