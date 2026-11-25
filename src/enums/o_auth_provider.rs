use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuthProvider {
    #[serde(rename = "amazon")]
    #[default]
    Amazon,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "appwrite")]
    Appwrite,
    #[serde(rename = "atlassian")]
    Atlassian,
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
    #[serde(rename = "cloudflare")]
    Cloudflare,
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
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "kakao")]
    Kakao,
    #[serde(rename = "keycloak")]
    Keycloak,
    #[serde(rename = "kick")]
    Kick,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "naver")]
    Naver,
    #[serde(rename = "netlify")]
    Netlify,
    #[serde(rename = "notion")]
    Notion,
    #[serde(rename = "oidc")]
    Oidc,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "patreon")]
    Patreon,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "paypalSandbox")]
    PaypalSandbox,
    #[serde(rename = "podio")]
    Podio,
    #[serde(rename = "railway")]
    Railway,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "resend")]
    Resend,
    #[serde(rename = "roblox")]
    Roblox,
    #[serde(rename = "salesforce")]
    Salesforce,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "strava")]
    Strava,
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "tiktok")]
    Tiktok,
    #[serde(rename = "tradeshift")]
    Tradeshift,
    #[serde(rename = "tradeshiftBox")]
    TradeshiftBox,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "vercel")]
    Vercel,
    #[serde(rename = "vipps")]
    Vipps,
    #[serde(rename = "vippsSandbox")]
    VippsSandbox,
    #[serde(rename = "vk")]
    Vk,
    #[serde(rename = "webflow")]
    Webflow,
    #[serde(rename = "wordpress")]
    Wordpress,
    #[serde(rename = "workos")]
    Workos,
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

impl OAuthProvider {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuthProvider::Amazon => "amazon",
            OAuthProvider::Apple => "apple",
            OAuthProvider::Appwrite => "appwrite",
            OAuthProvider::Atlassian => "atlassian",
            OAuthProvider::Auth0 => "auth0",
            OAuthProvider::Authentik => "authentik",
            OAuthProvider::Autodesk => "autodesk",
            OAuthProvider::Bitbucket => "bitbucket",
            OAuthProvider::Bitly => "bitly",
            OAuthProvider::Box => "box",
            OAuthProvider::Cloudflare => "cloudflare",
            OAuthProvider::Dailymotion => "dailymotion",
            OAuthProvider::Discord => "discord",
            OAuthProvider::Disqus => "disqus",
            OAuthProvider::Dropbox => "dropbox",
            OAuthProvider::Etsy => "etsy",
            OAuthProvider::Facebook => "facebook",
            OAuthProvider::Figma => "figma",
            OAuthProvider::Fusionauth => "fusionauth",
            OAuthProvider::Github => "github",
            OAuthProvider::Gitlab => "gitlab",
            OAuthProvider::Google => "google",
            OAuthProvider::Huggingface => "huggingface",
            OAuthProvider::Instagram => "instagram",
            OAuthProvider::Kakao => "kakao",
            OAuthProvider::Keycloak => "keycloak",
            OAuthProvider::Kick => "kick",
            OAuthProvider::Line => "line",
            OAuthProvider::Linear => "linear",
            OAuthProvider::Linkedin => "linkedin",
            OAuthProvider::Microsoft => "microsoft",
            OAuthProvider::Naver => "naver",
            OAuthProvider::Netlify => "netlify",
            OAuthProvider::Notion => "notion",
            OAuthProvider::Oidc => "oidc",
            OAuthProvider::Okta => "okta",
            OAuthProvider::Patreon => "patreon",
            OAuthProvider::Paypal => "paypal",
            OAuthProvider::PaypalSandbox => "paypalSandbox",
            OAuthProvider::Podio => "podio",
            OAuthProvider::Railway => "railway",
            OAuthProvider::Reddit => "reddit",
            OAuthProvider::Resend => "resend",
            OAuthProvider::Roblox => "roblox",
            OAuthProvider::Salesforce => "salesforce",
            OAuthProvider::Slack => "slack",
            OAuthProvider::Spotify => "spotify",
            OAuthProvider::Strava => "strava",
            OAuthProvider::Stripe => "stripe",
            OAuthProvider::Tiktok => "tiktok",
            OAuthProvider::Tradeshift => "tradeshift",
            OAuthProvider::TradeshiftBox => "tradeshiftBox",
            OAuthProvider::Twitch => "twitch",
            OAuthProvider::Vercel => "vercel",
            OAuthProvider::Vipps => "vipps",
            OAuthProvider::VippsSandbox => "vippsSandbox",
            OAuthProvider::Vk => "vk",
            OAuthProvider::Webflow => "webflow",
            OAuthProvider::Wordpress => "wordpress",
            OAuthProvider::Workos => "workos",
            OAuthProvider::X => "x",
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
