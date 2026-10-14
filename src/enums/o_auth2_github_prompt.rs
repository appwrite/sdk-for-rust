use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2GithubPrompt {
    #[serde(rename = "select_account")]
    #[default]
    SelectAccount,
}

impl OAuth2GithubPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2GithubPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for OAuth2GithubPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
