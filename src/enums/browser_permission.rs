use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BrowserPermission {
    #[serde(rename = "geolocation")]
    #[default]
    Geolocation,
    #[serde(rename = "camera")]
    Camera,
    #[serde(rename = "microphone")]
    Microphone,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "clipboard-read")]
    ClipboardRead,
    #[serde(rename = "clipboard-write")]
    ClipboardWrite,
    #[serde(rename = "payment-handler")]
    PaymentHandler,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "ambient-light-sensor")]
    AmbientLightSensor,
    #[serde(rename = "background-sync")]
    BackgroundSync,
    #[serde(rename = "persistent-storage")]
    PersistentStorage,
    #[serde(rename = "screen-wake-lock")]
    ScreenWakeLock,
    #[serde(rename = "web-share")]
    WebShare,
    #[serde(rename = "xr-spatial-tracking")]
    XrSpatialTracking,
}

impl BrowserPermission {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            BrowserPermission::Geolocation => "geolocation",
            BrowserPermission::Camera => "camera",
            BrowserPermission::Microphone => "microphone",
            BrowserPermission::Notifications => "notifications",
            BrowserPermission::Midi => "midi",
            BrowserPermission::Push => "push",
            BrowserPermission::ClipboardRead => "clipboard-read",
            BrowserPermission::ClipboardWrite => "clipboard-write",
            BrowserPermission::PaymentHandler => "payment-handler",
            BrowserPermission::Usb => "usb",
            BrowserPermission::Bluetooth => "bluetooth",
            BrowserPermission::Accelerometer => "accelerometer",
            BrowserPermission::Gyroscope => "gyroscope",
            BrowserPermission::Magnetometer => "magnetometer",
            BrowserPermission::AmbientLightSensor => "ambient-light-sensor",
            BrowserPermission::BackgroundSync => "background-sync",
            BrowserPermission::PersistentStorage => "persistent-storage",
            BrowserPermission::ScreenWakeLock => "screen-wake-lock",
            BrowserPermission::WebShare => "web-share",
            BrowserPermission::XrSpatialTracking => "xr-spatial-tracking",
        }
    }
}

impl std::fmt::Display for BrowserPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
