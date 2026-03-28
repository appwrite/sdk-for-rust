//! Avatars service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Avatars service aims to help you complete everyday tasks related to
/// your app image, icons, and avatars.
#[derive(Debug, Clone)]
pub struct Avatars {
    client: Client,
}

impl Avatars {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// You can use this endpoint to show different browser icons to your users.
    /// The code argument receives the browser code as it appears in your user [GET
    /// /account/sessions](https://appwrite.io/docs/references/cloud/client-web/account#getSessions)
    /// endpoint. Use width, height and quality arguments to change the output
    /// settings.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn get_browser(
        &self,
        code: crate::enums::Browser,
        width: Option<i64>,
        height: Option<i64>,
        quality: Option<i64>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = quality {
            params.insert("quality".to_string(), json!(value));
        }

        let path = "/avatars/browsers/{code}".to_string().replace("{code}", &code.to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// The credit card endpoint will return you the icon of the credit card
    /// provider you need. Use width, height and quality arguments to change the
    /// output settings.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn get_credit_card(
        &self,
        code: crate::enums::CreditCard,
        width: Option<i64>,
        height: Option<i64>,
        quality: Option<i64>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = quality {
            params.insert("quality".to_string(), json!(value));
        }

        let path = "/avatars/credit-cards/{code}".to_string().replace("{code}", &code.to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Use this endpoint to fetch the favorite icon (AKA favicon) of any remote
    /// website URL.
    /// 
    /// This endpoint does not follow HTTP redirects.
    pub async fn get_favicon(
        &self,
        url: impl Into<String>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        params.insert("url".to_string(), json!(url.into()));

        let path = "/avatars/favicon".to_string();

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// You can use this endpoint to show different country flags icons to your
    /// users. The code argument receives the 2 letter country code. Use width,
    /// height and quality arguments to change the output settings. Country codes
    /// follow the [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) standard.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn get_flag(
        &self,
        code: crate::enums::Flag,
        width: Option<i64>,
        height: Option<i64>,
        quality: Option<i64>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = quality {
            params.insert("quality".to_string(), json!(value));
        }

        let path = "/avatars/flags/{code}".to_string().replace("{code}", &code.to_string());

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Use this endpoint to fetch a remote image URL and crop it to any image size
    /// you want. This endpoint is very useful if you need to crop and display
    /// remote images in your app or in case you want to make sure a 3rd party
    /// image is properly served using a TLS protocol.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 400x400px.
    /// 
    /// This endpoint does not follow HTTP redirects.
    pub async fn get_image(
        &self,
        url: impl Into<String>,
        width: Option<i64>,
        height: Option<i64>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        params.insert("url".to_string(), json!(url.into()));
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }

        let path = "/avatars/image".to_string();

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Use this endpoint to show your user initials avatar icon on your website or
    /// app. By default, this route will try to print your logged-in user name or
    /// email initials. You can also overwrite the user name if you pass the 'name'
    /// parameter. If no name is given and no user is logged, an empty avatar will
    /// be returned.
    /// 
    /// You can use the color and background params to change the avatar colors. By
    /// default, a random theme will be selected. The random theme will persist for
    /// the user's initials when reloading the same theme will always return for
    /// the same initials.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn get_initials(
        &self,
        name: Option<&str>,
        width: Option<i64>,
        height: Option<i64>,
        background: Option<&str>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = background {
            params.insert("background".to_string(), json!(value));
        }

        let path = "/avatars/initials".to_string();

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Converts a given plain text to a QR code image. You can use the query
    /// parameters to change the size and style of the resulting image.
    pub async fn get_qr(
        &self,
        text: impl Into<String>,
        size: Option<i64>,
        margin: Option<i64>,
        download: Option<bool>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        params.insert("text".to_string(), json!(text.into()));
        if let Some(value) = size {
            params.insert("size".to_string(), json!(value));
        }
        if let Some(value) = margin {
            params.insert("margin".to_string(), json!(value));
        }
        if let Some(value) = download {
            params.insert("download".to_string(), json!(value));
        }

        let path = "/avatars/qr".to_string();

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

    /// Use this endpoint to capture a screenshot of any website URL. This endpoint
    /// uses a headless browser to render the webpage and capture it as an image.
    /// 
    /// You can configure the browser viewport size, theme, user agent,
    /// geolocation, permissions, and more. Capture either just the viewport or the
    /// full page scroll.
    /// 
    /// When width and height are specified, the image is resized accordingly. If
    /// both dimensions are 0, the API provides an image at original size. If
    /// dimensions are not specified, the default viewport size is 1280x720px.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_screenshot(
        &self,
        url: impl Into<String>,
        headers: Option<serde_json::Value>,
        viewport_width: Option<i64>,
        viewport_height: Option<i64>,
        scale: Option<f64>,
        theme: Option<crate::enums::Theme>,
        user_agent: Option<&str>,
        fullpage: Option<bool>,
        locale: Option<&str>,
        timezone: Option<crate::enums::Timezone>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        accuracy: Option<f64>,
        touch: Option<bool>,
        permissions: Option<Vec<crate::enums::BrowserPermission>>,
        sleep: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        quality: Option<i64>,
        output: Option<crate::enums::ImageFormat>,
    ) -> crate::error::Result<Vec<u8>> {
        let mut params = HashMap::new();
        params.insert("url".to_string(), json!(url.into()));
        if let Some(value) = headers {
            params.insert("headers".to_string(), json!(value));
        }
        if let Some(value) = viewport_width {
            params.insert("viewportWidth".to_string(), json!(value));
        }
        if let Some(value) = viewport_height {
            params.insert("viewportHeight".to_string(), json!(value));
        }
        if let Some(value) = scale {
            params.insert("scale".to_string(), json!(value));
        }
        if let Some(value) = theme {
            params.insert("theme".to_string(), json!(value));
        }
        if let Some(value) = user_agent {
            params.insert("userAgent".to_string(), json!(value));
        }
        if let Some(value) = fullpage {
            params.insert("fullpage".to_string(), json!(value));
        }
        if let Some(value) = locale {
            params.insert("locale".to_string(), json!(value));
        }
        if let Some(value) = timezone {
            params.insert("timezone".to_string(), json!(value));
        }
        if let Some(value) = latitude {
            params.insert("latitude".to_string(), json!(value));
        }
        if let Some(value) = longitude {
            params.insert("longitude".to_string(), json!(value));
        }
        if let Some(value) = accuracy {
            params.insert("accuracy".to_string(), json!(value));
        }
        if let Some(value) = touch {
            params.insert("touch".to_string(), json!(value));
        }
        if let Some(value) = permissions {
            params.insert("permissions".to_string(), json!(value));
        }
        if let Some(value) = sleep {
            params.insert("sleep".to_string(), json!(value));
        }
        if let Some(value) = width {
            params.insert("width".to_string(), json!(value));
        }
        if let Some(value) = height {
            params.insert("height".to_string(), json!(value));
        }
        if let Some(value) = quality {
            params.insert("quality".to_string(), json!(value));
        }
        if let Some(value) = output {
            params.insert("output".to_string(), json!(value));
        }

        let path = "/avatars/screenshots".to_string();

        self.client.call_bytes(Method::GET, &path, None, Some(params)).await
    }

}

impl crate::services::Service for Avatars {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatars_creation() {
        let client = Client::new();
        let service = Avatars::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
