use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TemplateReferenceType {
    #[serde(rename = "branch")]
    #[default]
    Branch,
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "tag")]
    Tag,
}

impl TemplateReferenceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            TemplateReferenceType::Branch => "branch",
            TemplateReferenceType::Commit => "commit",
            TemplateReferenceType::Tag => "tag",
        }
    }
}

impl std::fmt::Display for TemplateReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
