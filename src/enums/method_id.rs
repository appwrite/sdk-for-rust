use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MethodId {
    #[serde(rename = "email-password")]
    #[default]
    EmailPassword,
    #[serde(rename = "magic-url")]
    MagicUrl,
    #[serde(rename = "email-otp")]
    EmailOtp,
    #[serde(rename = "anonymous")]
    Anonymous,
    #[serde(rename = "invites")]
    Invites,
    #[serde(rename = "jwt")]
    Jwt,
    #[serde(rename = "phone")]
    Phone,
}

impl MethodId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MethodId::EmailPassword => "email-password",
            MethodId::MagicUrl => "magic-url",
            MethodId::EmailOtp => "email-otp",
            MethodId::Anonymous => "anonymous",
            MethodId::Invites => "invites",
            MethodId::Jwt => "jwt",
            MethodId::Phone => "phone",
        }
    }
}

impl std::fmt::Display for MethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
