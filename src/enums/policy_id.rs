use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PolicyId {
    #[serde(rename = "password-dictionary")]
    #[default]
    PasswordDictionary,
    #[serde(rename = "password-history")]
    PasswordHistory,
    #[serde(rename = "password-personal-data")]
    PasswordPersonalData,
    #[serde(rename = "session-alert")]
    SessionAlert,
    #[serde(rename = "session-duration")]
    SessionDuration,
    #[serde(rename = "session-invalidation")]
    SessionInvalidation,
    #[serde(rename = "session-limit")]
    SessionLimit,
    #[serde(rename = "user-limit")]
    UserLimit,
    #[serde(rename = "membership-privacy")]
    MembershipPrivacy,
}

impl PolicyId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            PolicyId::PasswordDictionary => "password-dictionary",
            PolicyId::PasswordHistory => "password-history",
            PolicyId::PasswordPersonalData => "password-personal-data",
            PolicyId::SessionAlert => "session-alert",
            PolicyId::SessionDuration => "session-duration",
            PolicyId::SessionInvalidation => "session-invalidation",
            PolicyId::SessionLimit => "session-limit",
            PolicyId::UserLimit => "user-limit",
            PolicyId::MembershipPrivacy => "membership-privacy",
        }
    }
}

impl std::fmt::Display for PolicyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
