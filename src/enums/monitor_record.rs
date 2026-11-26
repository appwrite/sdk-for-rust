use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorRecord {
    #[serde(rename = "A")]
    #[default]
    A,
    #[serde(rename = "AAAA")]
    AAAA,
    #[serde(rename = "CNAME")]
    CNAME,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "TXT")]
    TXT,
    #[serde(rename = "NS")]
    NS,
}

impl MonitorRecord {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorRecord::A => "A",
            MonitorRecord::AAAA => "AAAA",
            MonitorRecord::CNAME => "CNAME",
            MonitorRecord::MX => "MX",
            MonitorRecord::TXT => "TXT",
            MonitorRecord::NS => "NS",
        }
    }
}

impl std::fmt::Display for MonitorRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
