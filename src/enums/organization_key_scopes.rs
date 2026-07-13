use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OrganizationKeyScopes {
    #[serde(rename = "projects.read")]
    #[default]
    ProjectsRead,
    #[serde(rename = "projects.write")]
    ProjectsWrite,
    #[serde(rename = "devKeys.read")]
    DevKeysRead,
    #[serde(rename = "devKeys.write")]
    DevKeysWrite,
    #[serde(rename = "organization.keys.read")]
    OrganizationKeysRead,
    #[serde(rename = "organization.keys.write")]
    OrganizationKeysWrite,
    #[serde(rename = "organization.memberships.read")]
    OrganizationMembershipsRead,
    #[serde(rename = "organization.memberships.write")]
    OrganizationMembershipsWrite,
    #[serde(rename = "organization.read")]
    OrganizationRead,
    #[serde(rename = "organization.write")]
    OrganizationWrite,
    #[serde(rename = "domains.read")]
    DomainsRead,
    #[serde(rename = "domains.write")]
    DomainsWrite,
    #[serde(rename = "keys.read")]
    KeysRead,
    #[serde(rename = "keys.write")]
    KeysWrite,
}

impl OrganizationKeyScopes {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OrganizationKeyScopes::ProjectsRead => "projects.read",
            OrganizationKeyScopes::ProjectsWrite => "projects.write",
            OrganizationKeyScopes::DevKeysRead => "devKeys.read",
            OrganizationKeyScopes::DevKeysWrite => "devKeys.write",
            OrganizationKeyScopes::OrganizationKeysRead => "organization.keys.read",
            OrganizationKeyScopes::OrganizationKeysWrite => "organization.keys.write",
            OrganizationKeyScopes::OrganizationMembershipsRead => "organization.memberships.read",
            OrganizationKeyScopes::OrganizationMembershipsWrite => "organization.memberships.write",
            OrganizationKeyScopes::OrganizationRead => "organization.read",
            OrganizationKeyScopes::OrganizationWrite => "organization.write",
            OrganizationKeyScopes::DomainsRead => "domains.read",
            OrganizationKeyScopes::DomainsWrite => "domains.write",
            OrganizationKeyScopes::KeysRead => "keys.read",
            OrganizationKeyScopes::KeysWrite => "keys.write",
        }
    }
}

impl std::fmt::Display for OrganizationKeyScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
