//! Domains service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Domains {
    client: Client,
}

impl Domains {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List all domains registered for this project. This endpoint supports
    /// pagination.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::DomainsList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/domains".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new domain. Before creating a domain, you need to ensure that your
    /// DNS provider is properly configured. After creating the domain, you can use
    /// the verification endpoint to check if the domain is ready to be used.
    pub async fn create(
        &self,
        team_id: impl Into<String>,
        domain: impl Into<String>,
    ) -> crate::error::Result<crate::models::Domain> {
        let mut params = HashMap::new();
        params.insert("teamId".to_string(), json!(team_id.into()));
        params.insert("domain".to_string(), json!(domain.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/domains".to_string();

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get the registration price for a domain name.
    pub async fn get_price(
        &self,
        domain: impl Into<String>,
        period_years: Option<i64>,
        registration_type: Option<crate::enums::DomainRegistrationType>,
    ) -> crate::error::Result<crate::models::DomainPrice> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), json!(domain.into()));
        if let Some(value) = period_years {
            params.insert("periodYears".to_string(), json!(value));
        }
        if let Some(value) = registration_type {
            params.insert("registrationType".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/domains/price".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Check availability and get the requested registration price for one or more
    /// domain names. Availability is resolved for all domains in a single
    /// registrar lookup. Unavailable domains have a null price for new
    /// registrations, but can still be priced for renewal, transfer, or trade.
    /// Every priced domain also carries its renewal price for the same period, so
    /// a separate renewal lookup is not needed. A domain whose price could not be
    /// resolved, for example because its TLD is not supported, is returned with a
    /// null price.
    pub async fn list_prices(
        &self,
        domains: impl IntoIterator<Item = impl Into<String>>,
        period_years: Option<i64>,
        registration_type: Option<crate::enums::DomainRegistrationType>,
    ) -> crate::error::Result<crate::models::DomainPricesList> {
        let mut params = HashMap::new();
        params.insert(
            "domains".to_string(),
            json!(domains
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<String>>()),
        );
        if let Some(value) = period_years {
            params.insert("periodYears".to_string(), json!(value));
        }
        if let Some(value) = registration_type {
            params.insert("registrationType".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let path = "/domains/prices".to_string();

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a domain by its unique ID.
    pub async fn get(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Domain> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a domain by its unique ID. This endpoint can be used to delete a
    /// domain from your project.
    /// Once deleted, the domain will no longer be available for use and all
    /// associated resources will be removed.
    pub async fn delete(&self, domain_id: impl Into<String>) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update the registrar nameservers for the given domain. When nameservers are
    /// not provided,
    /// the domain will be updated to use Appwrite nameservers.
    pub async fn update_nameservers(
        &self,
        domain_id: impl Into<String>,
        nameservers: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Domain> {
        let mut params = HashMap::new();
        if let Some(value) = nameservers {
            params.insert(
                "nameservers".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/nameservers"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Verify which NS records are used and update the domain accordingly. This
    /// will check the domain's
    /// nameservers and update the domain's status based on whether the nameservers
    /// match the expected
    /// Appwrite nameservers.
    pub async fn verify_nameservers(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Domain> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/nameservers/verification"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// List Google Workspace DNS records.
    pub async fn get_preset_google_workspace(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/google-workspace"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add Google Workspace DNS records to the domain. This will create the
    /// required MX records
    /// for Google Workspace email hosting.
    pub async fn create_preset_google_workspace(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/google-workspace"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List iCloud DNS records.
    pub async fn get_preset_i_cloud(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/icloud"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add iCloud DNS records to the domain. This will create the required MX and
    /// SPF records
    /// for using iCloud email services with your domain.
    pub async fn create_preset_i_cloud(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/icloud"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List Mailgun DNS records.
    pub async fn get_preset_mailgun(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/mailgun"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add Mailgun DNS records to the domain. This endpoint will create the
    /// required DNS records
    /// for Mailgun in the specified domain.
    pub async fn create_preset_mailgun(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/mailgun"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List Outlook DNS records.
    pub async fn get_preset_outlook(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/outlook"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add Outlook DNS records to the domain. This will create the required MX
    /// records
    /// for setting up Outlook email hosting for your domain.
    pub async fn create_preset_outlook(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/outlook"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List ProtonMail DNS records.
    pub async fn get_preset_proton_mail(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/proton-mail"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add ProtonMail DNS records to the domain. This will create the required MX
    /// records
    /// for using ProtonMail with your custom domain.
    pub async fn create_preset_proton_mail(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/proton-mail"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List Zoho DNS records.
    pub async fn get_preset_zoho(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/zoho"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Add Zoho Mail DNS records to the domain. This will create the required MX
    /// records
    /// for setting up Zoho Mail on your domain.
    pub async fn create_preset_zoho(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/presets/zoho"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// List DNS records for a given domain. You can use this endpoint to list all
    /// the DNS records
    /// associated with your domain.
    pub async fn list_records(
        &self,
        domain_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::DnsRecordsList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert(
                "queries".to_string(),
                json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()),
            );
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new A record for the given domain. A records are used to point a
    /// domain name
    /// to an IPv4 address. The record value should be a valid IPv4 address.
    pub async fn create_record_a(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/a"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing A record for the given domain. This endpoint allows you
    /// to modify
    /// the properties of an A record including its name (subdomain), IPv4 address,
    /// TTL,
    /// and optional comment.
    pub async fn update_record_a(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/a/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new AAAA record for the given domain. This endpoint allows you to
    /// add a new IPv6 DNS record
    /// to your domain. The record will be used to point a hostname to an IPv6
    /// address.
    pub async fn create_record_aaaa(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/aaaa"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing AAAA record for the given domain. This endpoint allows
    /// you to modify
    /// the properties of an existing AAAA record, including its name (subdomain),
    /// IPv6 address,
    /// TTL, and optional comment.
    pub async fn update_record_aaaa(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/aaaa/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new ALIAS record for the given domain. This record type can be
    /// used to point your domain
    /// to another domain name that will serve as an alias. This is particularly
    /// useful when you want to
    /// map your domain to a target domain that may change its IP address.
    pub async fn create_record_alias(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/alias"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing ALIAS record for the specified domain. This endpoint
    /// allows you to modify
    /// the properties of an existing ALIAS record including its name, target
    /// domain, TTL, and comment.
    ///
    /// The ALIAS record type is similar to a CNAME record but can be used at the
    /// zone apex (root domain).
    /// It provides a way to map one domain name to another.
    pub async fn update_record_alias(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/alias/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new CAA record for the given domain. CAA records are used to
    /// specify which
    /// Certificate Authorities (CAs) are allowed to issue SSL/TLS certificates for
    /// your domain.
    pub async fn create_record_caa(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/caa"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing CAA record for the given domain. A CAA (Certification
    /// Authority Authorization)
    /// record is used to specify which certificate authorities (CAs) are
    /// authorized to issue certificates
    /// for a domain.
    pub async fn update_record_caa(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/caa/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new CNAME record for the given domain.
    ///
    /// A CNAME record maps a subdomain to another domain name, allowing you to
    /// create aliases
    /// for your domain. For example, you can create a CNAME record to point
    /// 'blog.example.com'
    /// to 'example.wordpress.com'.
    pub async fn create_record_cname(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/cname"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing CNAME record for the given domain.
    pub async fn update_record_cname(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/cname/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new HTTPS record for the given domain. This record is used to
    /// configure HTTPS
    /// settings for your domain, enabling secure communication over SSL/TLS.
    pub async fn create_record_https(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/https"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing HTTPS record for the given domain. This endpoint allows
    /// you to modify
    /// the properties of an HTTPS record associated with your domain, including
    /// the name (subdomain),
    /// target value, TTL, and optional comment.
    pub async fn update_record_https(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/https/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new MX record for the given domain. MX records are used to define
    /// the mail servers responsible
    /// for accepting email messages for the domain. Multiple MX records can be
    /// created with different priorities.
    /// The priority parameter determines the order in which mail servers are used,
    /// with lower values indicating
    /// higher priority.
    pub async fn create_record_mx(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        priority: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        params.insert("priority".to_string(), json!(priority));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/mx"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing MX record for the given domain.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_record_mx(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        priority: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        params.insert("priority".to_string(), json!(priority));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/mx/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new NS record for the given domain. NS records specify the
    /// nameservers that are used
    /// to resolve the domain name to IP addresses. Each domain can have multiple
    /// NS records.
    pub async fn create_record_ns(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/ns"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing NS record for the given domain. This endpoint allows you
    /// to modify
    /// the properties of an NS (nameserver) record associated with your domain.
    /// You can update
    /// the record name (subdomain), target nameserver value, TTL, and add or
    /// modify comments
    /// for better record management.
    pub async fn update_record_ns(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/ns/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new SRV record for the given domain. SRV records are used to
    /// define the location
    /// of servers for specific services. For example, they can be used to specify
    /// which server
    /// handles a specific service like SIP or XMPP for the domain.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_record_srv(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        priority: i64,
        weight: i64,
        port: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        params.insert("priority".to_string(), json!(priority));
        params.insert("weight".to_string(), json!(weight));
        params.insert("port".to_string(), json!(port));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/srv"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing SRV record for the given domain.
    ///
    /// Required parameters:
    /// - domainId: Domain unique ID
    /// - recordId: DNS record unique ID
    /// - name: Record name (service name)
    /// - value: Target hostname for this SRV record
    /// - ttl: Time to live, in seconds
    /// - priority: Record priority
    /// - weight: Record weight
    /// - port: Port number for the service
    ///
    /// Optional parameters:
    /// - comment: A comment for this record
    #[allow(clippy::too_many_arguments)]
    pub async fn update_record_srv(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        priority: i64,
        weight: i64,
        port: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        params.insert("priority".to_string(), json!(priority));
        params.insert("weight".to_string(), json!(weight));
        params.insert("port".to_string(), json!(port));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/srv/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Create a new TXT record for the given domain. TXT records can be used
    /// to provide additional information about your domain, such as domain
    /// verification records, SPF records, or DKIM records.
    pub async fn create_record_txt(
        &self,
        domain_id: impl Into<String>,
        name: impl Into<String>,
        ttl: i64,
        value: Option<&str>,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = value {
            params.insert("value".to_string(), json!(value));
        }
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/txt"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::POST, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update an existing TXT record for the given domain.
    ///
    /// Update the TXT record details for a specific domain by providing the domain
    /// ID,
    /// record ID, and the new record configuration including name, value, TTL, and
    /// an optional comment.
    pub async fn update_record_txt(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        ttl: i64,
        comment: Option<&str>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("value".to_string(), json!(value.into()));
        params.insert("ttl".to_string(), json!(ttl));
        if let Some(value) = comment {
            params.insert("comment".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/txt/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }

    /// Get a single DNS record for a given domain by record ID.
    ///
    /// This endpoint allows you to retrieve a specific DNS record associated with
    /// a domain
    /// using its unique identifier. The record contains information about the DNS
    /// configuration
    /// such as type, value, and TTL settings.
    pub async fn get_record(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DnsRecord> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Delete a DNS record for the given domain. This endpoint allows you to
    /// delete an existing DNS record
    /// from a specific domain.
    pub async fn delete_record(
        &self,
        domain_id: impl Into<String>,
        record_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"recordId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/records/{recordId}"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id))
            .replace("{recordId}", &Client::encode_path(&record_id));

        self.client
            .call(Method::DELETE, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update the team ID for a specific domain. The caller must administer the
    /// current
    /// team and be an owner of the destination team.
    ///
    /// Updating the team ID will transfer ownership and access control of the
    /// domain
    /// and all its DNS records to the new team.
    pub async fn update_team(
        &self,
        domain_id: impl Into<String>,
        team_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Domain> {
        let mut params = HashMap::new();
        params.insert("teamId".to_string(), json!(team_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/team"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::PATCH, &path, Some(api_headers), Some(params))
            .await
    }

    /// Retrieve the current transfer status for a domain. Returns the status, an
    /// optional reason, and a timestamp of the last status change.
    pub async fn get_transfer_status(
        &self,
        domain_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::DomainTransferStatus> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/transfers/status"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Retrieve the DNS zone file for the given domain. This endpoint will return
    /// the DNS
    /// zone file in a standardized format that can be used to configure DNS
    /// servers.
    pub async fn get_zone(&self, domain_id: impl Into<String>) -> crate::error::Result<String> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("accept".to_string(), "text/plain".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/zone"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call_text(Method::GET, &path, Some(api_headers), Some(params))
            .await
    }

    /// Update the DNS zone for the given domain using the provided zone file
    /// content.
    /// All parsed records the domain does not already have are imported and then
    /// the main domain document is returned.
    pub async fn update_zone(
        &self,
        domain_id: impl Into<String>,
        content: impl Into<String>,
    ) -> crate::error::Result<crate::models::Domain> {
        let mut params = HashMap::new();
        params.insert("content".to_string(), json!(content.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());
        api_headers.insert("accept".to_string(), "application/json".to_string());

        let domain_id = domain_id.into();
        if domain_id.is_empty() {
            return Err(crate::error::AppwriteError::new(
                0,
                "Missing required parameter: \"domainId\"",
                None,
                String::new(),
            ));
        }

        let path = "/domains/{domainId}/zone"
            .to_string()
            .replace("{domainId}", &Client::encode_path(&domain_id));

        self.client
            .call(Method::PUT, &path, Some(api_headers), Some(params))
            .await
    }
}

impl crate::services::Service for Domains {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domains_creation() {
        let client = Client::new();
        let service = Domains::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
