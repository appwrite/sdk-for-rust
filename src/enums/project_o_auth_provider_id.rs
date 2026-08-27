use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuthProviderId {
    #[serde(rename = "amazon")]
    #[default]
    Amazon,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "appwrite")]
    Appwrite,
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
    #[serde(rename = "huggingface")]
    Huggingface,
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
}

impl ProjectOAuthProviderId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuthProviderId::Amazon => "amazon",
            ProjectOAuthProviderId::Apple => "apple",
            ProjectOAuthProviderId::Appwrite => "appwrite",
            ProjectOAuthProviderId::Auth0 => "auth0",
            ProjectOAuthProviderId::Authentik => "authentik",
            ProjectOAuthProviderId::Autodesk => "autodesk",
            ProjectOAuthProviderId::Bitbucket => "bitbucket",
            ProjectOAuthProviderId::Bitly => "bitly",
            ProjectOAuthProviderId::Box => "box",
            ProjectOAuthProviderId::Dailymotion => "dailymotion",
            ProjectOAuthProviderId::Discord => "discord",
            ProjectOAuthProviderId::Disqus => "disqus",
            ProjectOAuthProviderId::Dropbox => "dropbox",
            ProjectOAuthProviderId::Etsy => "etsy",
            ProjectOAuthProviderId::Facebook => "facebook",
            ProjectOAuthProviderId::Figma => "figma",
            ProjectOAuthProviderId::Fusionauth => "fusionauth",
            ProjectOAuthProviderId::Github => "github",
            ProjectOAuthProviderId::Gitlab => "gitlab",
            ProjectOAuthProviderId::Google => "google",
            ProjectOAuthProviderId::Huggingface => "huggingface",
            ProjectOAuthProviderId::Keycloak => "keycloak",
            ProjectOAuthProviderId::Kick => "kick",
            ProjectOAuthProviderId::Linkedin => "linkedin",
            ProjectOAuthProviderId::Microsoft => "microsoft",
            ProjectOAuthProviderId::Notion => "notion",
            ProjectOAuthProviderId::Oidc => "oidc",
            ProjectOAuthProviderId::Okta => "okta",
            ProjectOAuthProviderId::Paypal => "paypal",
            ProjectOAuthProviderId::PaypalSandbox => "paypalSandbox",
            ProjectOAuthProviderId::Podio => "podio",
            ProjectOAuthProviderId::Salesforce => "salesforce",
            ProjectOAuthProviderId::Slack => "slack",
            ProjectOAuthProviderId::Spotify => "spotify",
            ProjectOAuthProviderId::Stripe => "stripe",
            ProjectOAuthProviderId::Tradeshift => "tradeshift",
            ProjectOAuthProviderId::TradeshiftBox => "tradeshiftBox",
            ProjectOAuthProviderId::Twitch => "twitch",
            ProjectOAuthProviderId::Wordpress => "wordpress",
            ProjectOAuthProviderId::X => "x",
            ProjectOAuthProviderId::Yahoo => "yahoo",
            ProjectOAuthProviderId::Yammer => "yammer",
            ProjectOAuthProviderId::Yandex => "yandex",
            ProjectOAuthProviderId::Zoho => "zoho",
            ProjectOAuthProviderId::Zoom => "zoom",
        }
    }
}

impl std::fmt::Display for ProjectOAuthProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
