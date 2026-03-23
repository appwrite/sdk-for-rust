use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Browser {
    #[serde(rename = "aa")]
    #[default]
    AvantBrowser,
    #[serde(rename = "an")]
    AndroidWebViewBeta,
    #[serde(rename = "ch")]
    GoogleChrome,
    #[serde(rename = "ci")]
    GoogleChromeIOS,
    #[serde(rename = "cm")]
    GoogleChromeMobile,
    #[serde(rename = "cr")]
    Chromium,
    #[serde(rename = "ff")]
    MozillaFirefox,
    #[serde(rename = "sf")]
    Safari,
    #[serde(rename = "mf")]
    MobileSafari,
    #[serde(rename = "ps")]
    MicrosoftEdge,
    #[serde(rename = "oi")]
    MicrosoftEdgeIOS,
    #[serde(rename = "om")]
    OperaMini,
    #[serde(rename = "op")]
    Opera,
    #[serde(rename = "on")]
    OperaNext,
}

impl Browser {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Browser::AvantBrowser => "aa",
            Browser::AndroidWebViewBeta => "an",
            Browser::GoogleChrome => "ch",
            Browser::GoogleChromeIOS => "ci",
            Browser::GoogleChromeMobile => "cm",
            Browser::Chromium => "cr",
            Browser::MozillaFirefox => "ff",
            Browser::Safari => "sf",
            Browser::MobileSafari => "mf",
            Browser::MicrosoftEdge => "ps",
            Browser::MicrosoftEdgeIOS => "oi",
            Browser::OperaMini => "om",
            Browser::Opera => "op",
            Browser::OperaNext => "on",
        }
    }
}

impl std::fmt::Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
