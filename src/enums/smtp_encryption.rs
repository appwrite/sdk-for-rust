use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SmtpEncryption {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "ssl")]
    Ssl,
    #[serde(rename = "tls")]
    Tls,
}

impl SmtpEncryption {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            SmtpEncryption::None => "none",
            SmtpEncryption::Ssl => "ssl",
            SmtpEncryption::Tls => "tls",
        }
    }
}

impl std::fmt::Display for SmtpEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
