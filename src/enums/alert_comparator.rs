use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AlertComparator {
    #[serde(rename = "greaterThan")]
    #[default]
    GreaterThan,
    #[serde(rename = "greaterThanEqual")]
    GreaterThanEqual,
    #[serde(rename = "lessThan")]
    LessThan,
    #[serde(rename = "lessThanEqual")]
    LessThanEqual,
}

impl AlertComparator {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AlertComparator::GreaterThan => "greaterThan",
            AlertComparator::GreaterThanEqual => "greaterThanEqual",
            AlertComparator::LessThan => "lessThan",
            AlertComparator::LessThanEqual => "lessThanEqual",
        }
    }
}

impl std::fmt::Display for AlertComparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
