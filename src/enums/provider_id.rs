use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProviderId {
    #[serde(rename = "amazon")]
    #[default]
    Amazon,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "auth0")]
    Auth0,
    #[serde(rename = "authentik")]
    Authentik,
    #[serde(rename = "autodesk")]
    Autodesk,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "bitly")]
    Bitly,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "dailymotion")]
    Dailymotion,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "disqus")]
    Disqus,
    #[serde(rename = "dropbox")]
    Dropbox,
    #[serde(rename = "etsy")]
    Etsy,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "figma")]
    Figma,
    #[serde(rename = "fusionauth")]
    Fusionauth,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "keycloak")]
    Keycloak,
    #[serde(rename = "kick")]
    Kick,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "notion")]
    Notion,
    #[serde(rename = "oidc")]
    Oidc,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "paypalSandbox")]
    PaypalSandbox,
    #[serde(rename = "podio")]
    Podio,
    #[serde(rename = "salesforce")]
    Salesforce,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "tradeshift")]
    Tradeshift,
    #[serde(rename = "tradeshiftBox")]
    TradeshiftBox,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "wordpress")]
    Wordpress,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "yahoo")]
    Yahoo,
    #[serde(rename = "yammer")]
    Yammer,
    #[serde(rename = "yandex")]
    Yandex,
    #[serde(rename = "zoho")]
    Zoho,
    #[serde(rename = "zoom")]
    Zoom,
    #[serde(rename = "mock")]
    Mock,
    #[serde(rename = "mock-unverified")]
    MockUnverified,
    #[serde(rename = "githubImagine")]
    GithubImagine,
    #[serde(rename = "googleImagine")]
    GoogleImagine,
}

impl ProviderId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProviderId::Amazon => "amazon",
            ProviderId::Apple => "apple",
            ProviderId::Auth0 => "auth0",
            ProviderId::Authentik => "authentik",
            ProviderId::Autodesk => "autodesk",
            ProviderId::Bitbucket => "bitbucket",
            ProviderId::Bitly => "bitly",
            ProviderId::Box => "box",
            ProviderId::Dailymotion => "dailymotion",
            ProviderId::Discord => "discord",
            ProviderId::Disqus => "disqus",
            ProviderId::Dropbox => "dropbox",
            ProviderId::Etsy => "etsy",
            ProviderId::Facebook => "facebook",
            ProviderId::Figma => "figma",
            ProviderId::Fusionauth => "fusionauth",
            ProviderId::Github => "github",
            ProviderId::Gitlab => "gitlab",
            ProviderId::Google => "google",
            ProviderId::Keycloak => "keycloak",
            ProviderId::Kick => "kick",
            ProviderId::Linkedin => "linkedin",
            ProviderId::Microsoft => "microsoft",
            ProviderId::Notion => "notion",
            ProviderId::Oidc => "oidc",
            ProviderId::Okta => "okta",
            ProviderId::Paypal => "paypal",
            ProviderId::PaypalSandbox => "paypalSandbox",
            ProviderId::Podio => "podio",
            ProviderId::Salesforce => "salesforce",
            ProviderId::Slack => "slack",
            ProviderId::Spotify => "spotify",
            ProviderId::Stripe => "stripe",
            ProviderId::Tradeshift => "tradeshift",
            ProviderId::TradeshiftBox => "tradeshiftBox",
            ProviderId::Twitch => "twitch",
            ProviderId::Wordpress => "wordpress",
            ProviderId::X => "x",
            ProviderId::Yahoo => "yahoo",
            ProviderId::Yammer => "yammer",
            ProviderId::Yandex => "yandex",
            ProviderId::Zoho => "zoho",
            ProviderId::Zoom => "zoom",
            ProviderId::Mock => "mock",
            ProviderId::MockUnverified => "mock-unverified",
            ProviderId::GithubImagine => "githubImagine",
            ProviderId::GoogleImagine => "googleImagine",
        }
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
