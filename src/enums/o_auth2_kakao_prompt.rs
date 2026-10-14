use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2KakaoPrompt {
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

impl OAuth2KakaoPrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2KakaoPrompt::None => "none",
            OAuth2KakaoPrompt::Login => "login",
            OAuth2KakaoPrompt::Create => "create",
            OAuth2KakaoPrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for OAuth2KakaoPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
