use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Framework {
    #[serde(rename = "analog")]
    #[default]
    Analog,
    #[serde(rename = "angular")]
    Angular,
    #[serde(rename = "nextjs")]
    Nextjs,
    #[serde(rename = "react")]
    React,
    #[serde(rename = "nuxt")]
    Nuxt,
    #[serde(rename = "vue")]
    Vue,
    #[serde(rename = "sveltekit")]
    Sveltekit,
    #[serde(rename = "astro")]
    Astro,
    #[serde(rename = "tanstack-start")]
    TanstackStart,
    #[serde(rename = "remix")]
    Remix,
    #[serde(rename = "lynx")]
    Lynx,
    #[serde(rename = "flutter")]
    Flutter,
    #[serde(rename = "react-native")]
    ReactNative,
    #[serde(rename = "vite")]
    Vite,
    #[serde(rename = "other")]
    Other,
}

impl Framework {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Framework::Analog => "analog",
            Framework::Angular => "angular",
            Framework::Nextjs => "nextjs",
            Framework::React => "react",
            Framework::Nuxt => "nuxt",
            Framework::Vue => "vue",
            Framework::Sveltekit => "sveltekit",
            Framework::Astro => "astro",
            Framework::TanstackStart => "tanstack-start",
            Framework::Remix => "remix",
            Framework::Lynx => "lynx",
            Framework::Flutter => "flutter",
            Framework::ReactNative => "react-native",
            Framework::Vite => "vite",
            Framework::Other => "other",
        }
    }
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
