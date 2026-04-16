use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProtocolId {
    #[serde(rename = "rest")]
    #[default]
    Rest,
    #[serde(rename = "graphql")]
    Graphql,
    #[serde(rename = "websocket")]
    Websocket,
}

impl ProtocolId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProtocolId::Rest => "rest",
            ProtocolId::Graphql => "graphql",
            ProtocolId::Websocket => "websocket",
        }
    }
}

impl std::fmt::Display for ProtocolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
