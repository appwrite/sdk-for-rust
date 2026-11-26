use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MonitorResult {
    #[serde(rename = "ok")]
    #[default]
    Ok,
    #[serde(rename = "dns_failed")]
    DnsFailed,
    #[serde(rename = "connect_refused")]
    ConnectRefused,
    #[serde(rename = "connect_timeout")]
    ConnectTimeout,
    #[serde(rename = "connect_failed")]
    ConnectFailed,
    #[serde(rename = "tls_failed")]
    TlsFailed,
    #[serde(rename = "tls_expiring")]
    TlsExpiring,
    #[serde(rename = "status_mismatch")]
    StatusMismatch,
    #[serde(rename = "answer_mismatch")]
    AnswerMismatch,
    #[serde(rename = "smtp_rejected")]
    SmtpRejected,
    #[serde(rename = "target_denied")]
    TargetDenied,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "rule_matched")]
    RuleMatched,
}

impl MonitorResult {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            MonitorResult::Ok => "ok",
            MonitorResult::DnsFailed => "dns_failed",
            MonitorResult::ConnectRefused => "connect_refused",
            MonitorResult::ConnectTimeout => "connect_timeout",
            MonitorResult::ConnectFailed => "connect_failed",
            MonitorResult::TlsFailed => "tls_failed",
            MonitorResult::TlsExpiring => "tls_expiring",
            MonitorResult::StatusMismatch => "status_mismatch",
            MonitorResult::AnswerMismatch => "answer_mismatch",
            MonitorResult::SmtpRejected => "smtp_rejected",
            MonitorResult::TargetDenied => "target_denied",
            MonitorResult::Timeout => "timeout",
            MonitorResult::RuleMatched => "rule_matched",
        }
    }
}

impl std::fmt::Display for MonitorResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
