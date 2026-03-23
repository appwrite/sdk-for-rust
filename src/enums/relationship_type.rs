use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RelationshipType {
    #[serde(rename = "oneToOne")]
    #[default]
    OneToOne,
    #[serde(rename = "manyToOne")]
    ManyToOne,
    #[serde(rename = "manyToMany")]
    ManyToMany,
    #[serde(rename = "oneToMany")]
    OneToMany,
}

impl RelationshipType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            RelationshipType::OneToOne => "oneToOne",
            RelationshipType::ManyToOne => "manyToOne",
            RelationshipType::ManyToMany => "manyToMany",
            RelationshipType::OneToMany => "oneToMany",
        }
    }
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
