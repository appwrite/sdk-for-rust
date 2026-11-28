use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnalyticsDimension {
    #[serde(rename = "country")]
    #[default]
    Country,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "city")]
    City,
    #[serde(rename = "browser")]
    Browser,
    #[serde(rename = "browserVersion")]
    BrowserVersion,
    #[serde(rename = "operatingSystem")]
    OperatingSystem,
    #[serde(rename = "operatingSystemVersion")]
    OperatingSystemVersion,
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "screenSize")]
    ScreenSize,
    #[serde(rename = "referrerSource")]
    ReferrerSource,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "utmSource")]
    UtmSource,
    #[serde(rename = "utmMedium")]
    UtmMedium,
    #[serde(rename = "utmCampaign")]
    UtmCampaign,
    #[serde(rename = "utmContent")]
    UtmContent,
    #[serde(rename = "utmTerm")]
    UtmTerm,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "hostname")]
    Hostname,
    #[serde(rename = "entryPage")]
    EntryPage,
    #[serde(rename = "exitPage")]
    ExitPage,
    #[serde(rename = "trafficType")]
    TrafficType,
    #[serde(rename = "botName")]
    BotName,
    #[serde(rename = "botCategory")]
    BotCategory,
    #[serde(rename = "userId")]
    UserId,
    #[serde(rename = "eventName")]
    EventName,
    #[serde(rename = "props")]
    Props,
}

impl AnalyticsDimension {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AnalyticsDimension::Country => "country",
            AnalyticsDimension::Region => "region",
            AnalyticsDimension::City => "city",
            AnalyticsDimension::Browser => "browser",
            AnalyticsDimension::BrowserVersion => "browserVersion",
            AnalyticsDimension::OperatingSystem => "operatingSystem",
            AnalyticsDimension::OperatingSystemVersion => "operatingSystemVersion",
            AnalyticsDimension::Device => "device",
            AnalyticsDimension::ScreenSize => "screenSize",
            AnalyticsDimension::ReferrerSource => "referrerSource",
            AnalyticsDimension::Channel => "channel",
            AnalyticsDimension::UtmSource => "utmSource",
            AnalyticsDimension::UtmMedium => "utmMedium",
            AnalyticsDimension::UtmCampaign => "utmCampaign",
            AnalyticsDimension::UtmContent => "utmContent",
            AnalyticsDimension::UtmTerm => "utmTerm",
            AnalyticsDimension::Page => "page",
            AnalyticsDimension::Hostname => "hostname",
            AnalyticsDimension::EntryPage => "entryPage",
            AnalyticsDimension::ExitPage => "exitPage",
            AnalyticsDimension::TrafficType => "trafficType",
            AnalyticsDimension::BotName => "botName",
            AnalyticsDimension::BotCategory => "botCategory",
            AnalyticsDimension::UserId => "userId",
            AnalyticsDimension::EventName => "eventName",
            AnalyticsDimension::Props => "props",
        }
    }
}

impl std::fmt::Display for AnalyticsDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
