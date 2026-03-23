use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuthProvider {
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
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "google")]
    Google,
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
}

impl OAuthProvider {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuthProvider::Amazon => "amazon",
            OAuthProvider::Apple => "apple",
            OAuthProvider::Auth0 => "auth0",
            OAuthProvider::Authentik => "authentik",
            OAuthProvider::Autodesk => "autodesk",
            OAuthProvider::Bitbucket => "bitbucket",
            OAuthProvider::Bitly => "bitly",
            OAuthProvider::Box => "box",
            OAuthProvider::Dailymotion => "dailymotion",
            OAuthProvider::Discord => "discord",
            OAuthProvider::Disqus => "disqus",
            OAuthProvider::Dropbox => "dropbox",
            OAuthProvider::Etsy => "etsy",
            OAuthProvider::Facebook => "facebook",
            OAuthProvider::Figma => "figma",
            OAuthProvider::Github => "github",
            OAuthProvider::Gitlab => "gitlab",
            OAuthProvider::Google => "google",
            OAuthProvider::Linkedin => "linkedin",
            OAuthProvider::Microsoft => "microsoft",
            OAuthProvider::Notion => "notion",
            OAuthProvider::Oidc => "oidc",
            OAuthProvider::Okta => "okta",
            OAuthProvider::Paypal => "paypal",
            OAuthProvider::PaypalSandbox => "paypalSandbox",
            OAuthProvider::Podio => "podio",
            OAuthProvider::Salesforce => "salesforce",
            OAuthProvider::Slack => "slack",
            OAuthProvider::Spotify => "spotify",
            OAuthProvider::Stripe => "stripe",
            OAuthProvider::Tradeshift => "tradeshift",
            OAuthProvider::TradeshiftBox => "tradeshiftBox",
            OAuthProvider::Twitch => "twitch",
            OAuthProvider::Wordpress => "wordpress",
            OAuthProvider::Yahoo => "yahoo",
            OAuthProvider::Yammer => "yammer",
            OAuthProvider::Yandex => "yandex",
            OAuthProvider::Zoho => "zoho",
            OAuthProvider::Zoom => "zoom",
        }
    }
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
