use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2GitHubPrompt {
    #[serde(rename = "select_account")]
    #[default]
    SelectAccount,
}

impl ProjectOAuth2GitHubPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2GitHubPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2GitHubPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
