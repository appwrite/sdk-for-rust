use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TemplateReferenceType {
    #[serde(rename = "commit")]
    #[default]
    Commit,
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
}

impl TemplateReferenceType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            TemplateReferenceType::Commit => "commit",
            TemplateReferenceType::Branch => "branch",
            TemplateReferenceType::Tag => "tag",
        }
    }
}

impl std::fmt::Display for TemplateReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
