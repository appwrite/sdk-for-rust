use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DomainTransferStatusEnum {
    #[serde(rename = "transferrable")]
    #[default]
    Transferrable,
    #[serde(rename = "not_transferrable")]
    NotTransferrable,
    #[serde(rename = "pending_owner")]
    PendingOwner,
    #[serde(rename = "pending_admin")]
    PendingAdmin,
    #[serde(rename = "pending_registry")]
    PendingRegistry,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "service_unavailable")]
    ServiceUnavailable,
}

impl DomainTransferStatusEnum {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            DomainTransferStatusEnum::Transferrable => "transferrable",
            DomainTransferStatusEnum::NotTransferrable => "not_transferrable",
            DomainTransferStatusEnum::PendingOwner => "pending_owner",
            DomainTransferStatusEnum::PendingAdmin => "pending_admin",
            DomainTransferStatusEnum::PendingRegistry => "pending_registry",
            DomainTransferStatusEnum::Completed => "completed",
            DomainTransferStatusEnum::Cancelled => "cancelled",
            DomainTransferStatusEnum::ServiceUnavailable => "service_unavailable",
        }
    }
}

impl std::fmt::Display for DomainTransferStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
