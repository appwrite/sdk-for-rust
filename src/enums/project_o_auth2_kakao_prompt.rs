use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2KakaoPrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "select_account")]
    SelectAccount,
}

impl ProjectOAuth2KakaoPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2KakaoPrompt::None => "none",
            ProjectOAuth2KakaoPrompt::Login => "login",
            ProjectOAuth2KakaoPrompt::Create => "create",
            ProjectOAuth2KakaoPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2KakaoPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
