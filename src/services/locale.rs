//! Locale service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;

use std::collections::HashMap;

/// The Locale service allows you to customize your app based on your users'
/// location.
#[derive(Debug, Clone)]
pub struct Locale {
    client: Client,
}

impl Locale {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the current user location based on IP. Returns an object with user
    /// country code, country name, continent name, continent code, ip address and
    /// suggested currency. You can use the locale header to get the data in a
    /// supported language.
    /// 
    /// ([IP Geolocation by DB-IP](https://db-ip.com))
    pub async fn get(
        &self,
    ) -> crate::error::Result<crate::models::Locale> {
        let params = HashMap::new();

        let path = "/locale".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all locale codes in [ISO
    /// 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
    pub async fn list_codes(
        &self,
    ) -> crate::error::Result<crate::models::LocaleCodeList> {
        let params = HashMap::new();

        let path = "/locale/codes".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all continents. You can use the locale header to get the data in a
    /// supported language.
    pub async fn list_continents(
        &self,
    ) -> crate::error::Result<crate::models::ContinentList> {
        let params = HashMap::new();

        let path = "/locale/continents".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all countries. You can use the locale header to get the data in a
    /// supported language.
    pub async fn list_countries(
        &self,
    ) -> crate::error::Result<crate::models::CountryList> {
        let params = HashMap::new();

        let path = "/locale/countries".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all countries that are currently members of the EU. You can use the
    /// locale header to get the data in a supported language.
    pub async fn list_countries_eu(
        &self,
    ) -> crate::error::Result<crate::models::CountryList> {
        let params = HashMap::new();

        let path = "/locale/countries/eu".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all countries phone codes. You can use the locale header to get the
    /// data in a supported language.
    pub async fn list_countries_phones(
        &self,
    ) -> crate::error::Result<crate::models::PhoneList> {
        let params = HashMap::new();

        let path = "/locale/countries/phones".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all currencies, including currency symbol, name, plural, and
    /// decimal digits for all major and minor currencies. You can use the locale
    /// header to get the data in a supported language.
    pub async fn list_currencies(
        &self,
    ) -> crate::error::Result<crate::models::CurrencyList> {
        let params = HashMap::new();

        let path = "/locale/currencies".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// List of all languages classified by ISO 639-1 including 2-letter code, name
    /// in English, and name in the respective language.
    pub async fn list_languages(
        &self,
    ) -> crate::error::Result<crate::models::LanguageList> {
        let params = HashMap::new();

        let path = "/locale/languages".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

}

impl crate::services::Service for Locale {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_creation() {
        let client = Client::new();
        let service = Locale::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
