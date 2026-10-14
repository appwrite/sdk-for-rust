use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectEmailTemplateId {
    #[serde(rename = "verification")]
    #[default]
    Verification,
    #[serde(rename = "magicSession")]
    MagicSession,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "invitation")]
    Invitation,
    #[serde(rename = "mfaChallenge")]
    MfaChallenge,
    #[serde(rename = "sessionAlert")]
    SessionAlert,
    #[serde(rename = "otpSession")]
    OtpSession,
    #[serde(rename = "otpVerification")]
    OtpVerification,
    #[serde(rename = "otpRecovery")]
    OtpRecovery,
}

impl ProjectEmailTemplateId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectEmailTemplateId::Verification => "verification",
            ProjectEmailTemplateId::MagicSession => "magicSession",
            ProjectEmailTemplateId::Recovery => "recovery",
            ProjectEmailTemplateId::Invitation => "invitation",
            ProjectEmailTemplateId::MfaChallenge => "mfaChallenge",
            ProjectEmailTemplateId::SessionAlert => "sessionAlert",
            ProjectEmailTemplateId::OtpSession => "otpSession",
            ProjectEmailTemplateId::OtpVerification => "otpVerification",
            ProjectEmailTemplateId::OtpRecovery => "otpRecovery",
        }
    }
}

impl std::fmt::Display for ProjectEmailTemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
