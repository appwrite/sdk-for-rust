use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectLDAPEncryption {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "ldaps")]
    Ldaps,
    #[serde(rename = "starttls")]
    Starttls,
}

impl ProjectLDAPEncryption {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectLDAPEncryption::None => "none",
            ProjectLDAPEncryption::Ldaps => "ldaps",
            ProjectLDAPEncryption::Starttls => "starttls",
        }
    }
}

impl std::fmt::Display for ProjectLDAPEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
