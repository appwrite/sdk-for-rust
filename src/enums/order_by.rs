use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OrderBy {
    #[serde(rename = "asc")]
    #[default]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl OrderBy {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OrderBy::Asc => "asc",
            OrderBy::Desc => "desc",
        }
    }
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
