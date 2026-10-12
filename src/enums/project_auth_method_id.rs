use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectAuthMethodId {
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
    #[serde(rename = "passkey")]
    Passkey,
}

impl ProjectAuthMethodId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectAuthMethodId::EmailPassword => "email-password",
            ProjectAuthMethodId::MagicUrl => "magic-url",
            ProjectAuthMethodId::EmailOtp => "email-otp",
            ProjectAuthMethodId::Anonymous => "anonymous",
            ProjectAuthMethodId::Invites => "invites",
            ProjectAuthMethodId::Jwt => "jwt",
            ProjectAuthMethodId::Phone => "phone",
            ProjectAuthMethodId::Passkey => "passkey",
        }
    }
}

impl std::fmt::Display for ProjectAuthMethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
