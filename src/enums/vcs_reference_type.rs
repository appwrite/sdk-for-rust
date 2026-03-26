use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum VCSReferenceType {
    #[serde(rename = "branch")]
    #[default]
    Branch,
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "tag")]
    Tag,
}

impl VCSReferenceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            VCSReferenceType::Branch => "branch",
            VCSReferenceType::Commit => "commit",
            VCSReferenceType::Tag => "tag",
        }
    }
}

impl std::fmt::Display for VCSReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
