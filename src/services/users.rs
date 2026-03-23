//! Users service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Users service allows you to manage your project users.
#[derive(Debug, Clone)]
pub struct Users {
    client: Client,
}

impl Users {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the project's users. You can use the query params to
    /// filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::UserList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/users".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new user.
    pub async fn create(
        &self,
        user_id: impl Into<String>,
        email: Option<&str>,
        phone: Option<&str>,
        password: Option<&str>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        if let Some(value) = email {
            params.insert("email".to_string(), json!(value));
        }
        if let Some(value) = phone {
            params.insert("phone".to_string(), json!(value));
        }
        if let Some(value) = password {
            params.insert("password".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the
    /// [Argon2](https://en.wikipedia.org/wiki/Argon2) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_argon2_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/argon2".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the
    /// [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_bcrypt_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/bcrypt".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get identities for all users.
    pub async fn list_identities(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::IdentityList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/users/identities".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete an identity by its unique ID.
    pub async fn delete_identity(
        &self,
        identity_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/identities/{identityId}".to_string().replace("{identityId}", &identity_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the
    /// [MD5](https://en.wikipedia.org/wiki/MD5) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_md5_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/md5".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the
    /// [PHPass](https://www.openwall.com/phpass/) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_ph_pass_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/phpass".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the
    /// [Scrypt](https://github.com/Tarsnap/scrypt) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_scrypt_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        password_salt: impl Into<String>,
        password_cpu: i64,
        password_memory: i64,
        password_parallel: i64,
        password_length: i64,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        params.insert("passwordSalt".to_string(), json!(password_salt.into()));
        params.insert("passwordCpu".to_string(), json!(password_cpu));
        params.insert("passwordMemory".to_string(), json!(password_memory));
        params.insert("passwordParallel".to_string(), json!(password_parallel));
        params.insert("passwordLength".to_string(), json!(password_length));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/scrypt".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the [Scrypt
    /// Modified](https://gist.github.com/Meldiron/eecf84a0225eccb5a378d45bb27462cc)
    /// algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_scrypt_modified_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        password_salt: impl Into<String>,
        password_salt_separator: impl Into<String>,
        password_signer_key: impl Into<String>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        params.insert("passwordSalt".to_string(), json!(password_salt.into()));
        params.insert("passwordSaltSeparator".to_string(), json!(password_salt_separator.into()));
        params.insert("passwordSignerKey".to_string(), json!(password_signer_key.into()));
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/scrypt-modified".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new user. Password provided must be hashed with the
    /// [SHA](https://en.wikipedia.org/wiki/Secure_Hash_Algorithm) algorithm. Use
    /// the [POST /users](https://appwrite.io/docs/server/users#usersCreate)
    /// endpoint to create users with a plain text password.
    pub async fn create_sha_user(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
        password_version: Option<crate::enums::PasswordHash>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = password_version {
            params.insert("passwordVersion".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/sha".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a user by its unique ID.
    pub async fn get(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let params = HashMap::new();

        let path = "/users/{userId}".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a user by its unique ID, thereby releasing it's ID. Since ID is
    /// released and can be reused, all user-related resources like documents or
    /// storage files should be deleted before user deletion. If you want to keep
    /// ID reserved, use the
    /// [updateStatus](https://appwrite.io/docs/server/users#usersUpdateStatus)
    /// endpoint instead.
    pub async fn delete(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user email by its unique ID.
    pub async fn update_email(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("email".to_string(), json!(email.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/email".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Enable or disable whether a user can impersonate other users. When
    /// impersonation headers are used, the request runs as the target user for API
    /// behavior, while internal audit logs still attribute the action to the
    /// original impersonator and store the impersonated target details only in
    /// internal audit payload data.
    pub async fn update_impersonator(
        &self,
        user_id: impl Into<String>,
        impersonator: bool,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("impersonator".to_string(), json!(impersonator));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/impersonator".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to create a JSON Web Token for user by its unique ID. You
    /// can use the resulting JWT to authenticate on behalf of the user. The JWT
    /// secret will become invalid if the session it uses gets deleted.
    pub async fn create_jwt(
        &self,
        user_id: impl Into<String>,
        session_id: Option<&str>,
        duration: Option<i64>,
    ) -> crate::error::Result<crate::models::Jwt> {
        let mut params = HashMap::new();
        if let Some(value) = session_id {
            params.insert("sessionId".to_string(), json!(value));
        }
        if let Some(value) = duration {
            params.insert("duration".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/jwts".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user labels by its unique ID.
    /// 
    /// Labels can be used to grant access to resources. While teams are a way for
    /// user's to share access to a resource, labels can be defined by the
    /// developer to grant access without an invitation. See the [Permissions
    /// docs](https://appwrite.io/docs/permissions) for more info.
    pub async fn update_labels(
        &self,
        user_id: impl Into<String>,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("labels".to_string(), json!(labels.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/labels".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Get the user activity logs list by its unique ID.
    pub async fn list_logs(
        &self,
        user_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::LogList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/users/{userId}/logs".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get the user membership list by its unique ID.
    pub async fn list_memberships(
        &self,
        user_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::MembershipList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/users/{userId}/memberships".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Enable or disable MFA on a user account.
    pub async fn update_mfa(
        &self,
        user_id: impl Into<String>,
        mfa: bool,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("mfa".to_string(), json!(mfa));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/mfa".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an authenticator app.
    pub async fn delete_mfa_authenticator(
        &self,
        user_id: impl Into<String>,
        r#type: crate::enums::AuthenticatorType,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/mfa/authenticators/{type}".to_string().replace("{userId}", &user_id.into().to_string()).replace("{type}", &r#type.to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// List the factors available on the account to be used as a MFA challange.
    pub async fn list_mfa_factors(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::MfaFactors> {
        let params = HashMap::new();

        let path = "/users/{userId}/mfa/factors".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get recovery codes that can be used as backup for MFA flow by User ID.
    /// Before getting codes, they must be generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method.
    pub async fn get_mfa_recovery_codes(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::MfaRecoveryCodes> {
        let params = HashMap::new();

        let path = "/users/{userId}/mfa/recovery-codes".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Regenerate recovery codes that can be used as backup for MFA flow by User
    /// ID. Before regenerating codes, they must be first generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method.
    pub async fn update_mfa_recovery_codes(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::MfaRecoveryCodes> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/mfa/recovery-codes".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Generate recovery codes used as backup for MFA flow for User ID. Recovery
    /// codes can be used as a MFA verification type in
    /// [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge)
    /// method by client SDK.
    pub async fn create_mfa_recovery_codes(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::MfaRecoveryCodes> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/mfa/recovery-codes".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user name by its unique ID.
    pub async fn update_name(
        &self,
        user_id: impl Into<String>,
        name: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/name".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user password by its unique ID.
    pub async fn update_password(
        &self,
        user_id: impl Into<String>,
        password: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("password".to_string(), json!(password.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/password".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user phone by its unique ID.
    pub async fn update_phone(
        &self,
        user_id: impl Into<String>,
        number: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("number".to_string(), json!(number.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/phone".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get the user preferences by its unique ID.
    pub async fn get_prefs(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Preferences> {
        let params = HashMap::new();

        let path = "/users/{userId}/prefs".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update the user preferences by its unique ID. The object you pass is stored
    /// as is, and replaces any previous value. The maximum allowed prefs size is
    /// 64kB and throws error if exceeded.
    pub async fn update_prefs(
        &self,
        user_id: impl Into<String>,
        prefs: serde_json::Value,
    ) -> crate::error::Result<crate::models::Preferences> {
        let mut params = HashMap::new();
        params.insert("prefs".to_string(), json!(prefs));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/prefs".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get the user sessions list by its unique ID.
    pub async fn list_sessions(
        &self,
        user_id: impl Into<String>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::SessionList> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/users/{userId}/sessions".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Creates a session for a user. Returns an immediately usable session object.
    /// 
    /// If you want to generate a token for a custom authentication flow, use the
    /// [POST
    /// /users/{userId}/tokens](https://appwrite.io/docs/server/users#createToken)
    /// endpoint.
    pub async fn create_session(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/sessions".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Delete all user's sessions by using the user's unique ID.
    pub async fn delete_sessions(
        &self,
        user_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/sessions".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a user sessions by its unique ID.
    pub async fn delete_session(
        &self,
        user_id: impl Into<String>,
        session_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/sessions/{sessionId}".to_string().replace("{userId}", &user_id.into().to_string()).replace("{sessionId}", &session_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user status by its unique ID. Use this endpoint as an
    /// alternative to deleting a user if you want to keep user's ID reserved.
    pub async fn update_status(
        &self,
        user_id: impl Into<String>,
        status: bool,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("status".to_string(), json!(status));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/status".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// List the messaging targets that are associated with a user.
    pub async fn list_targets(
        &self,
        user_id: impl Into<String>,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::TargetList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/users/{userId}/targets".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a messaging target.
    pub async fn create_target(
        &self,
        user_id: impl Into<String>,
        target_id: impl Into<String>,
        provider_type: crate::enums::MessagingProviderType,
        identifier: impl Into<String>,
        provider_id: Option<&str>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::Target> {
        let mut params = HashMap::new();
        params.insert("targetId".to_string(), json!(target_id.into()));
        params.insert("providerType".to_string(), json!(provider_type));
        params.insert("identifier".to_string(), json!(identifier.into()));
        if let Some(value) = provider_id {
            params.insert("providerId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/targets".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a user's push notification target by ID.
    pub async fn get_target(
        &self,
        user_id: impl Into<String>,
        target_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Target> {
        let params = HashMap::new();

        let path = "/users/{userId}/targets/{targetId}".to_string().replace("{userId}", &user_id.into().to_string()).replace("{targetId}", &target_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a messaging target.
    pub async fn update_target(
        &self,
        user_id: impl Into<String>,
        target_id: impl Into<String>,
        identifier: Option<&str>,
        provider_id: Option<&str>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::Target> {
        let mut params = HashMap::new();
        if let Some(value) = identifier {
            params.insert("identifier".to_string(), json!(value));
        }
        if let Some(value) = provider_id {
            params.insert("providerId".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/targets/{targetId}".to_string().replace("{userId}", &user_id.into().to_string()).replace("{targetId}", &target_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a messaging target.
    pub async fn delete_target(
        &self,
        user_id: impl Into<String>,
        target_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/targets/{targetId}".to_string().replace("{userId}", &user_id.into().to_string()).replace("{targetId}", &target_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Returns a token with a secret key for creating a session. Use the user ID
    /// and secret and submit a request to the [PUT
    /// /account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process.
    pub async fn create_token(
        &self,
        user_id: impl Into<String>,
        length: Option<i64>,
        expire: Option<i64>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        if let Some(value) = length {
            params.insert("length".to_string(), json!(value));
        }
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/tokens".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user email verification status by its unique ID.
    pub async fn update_email_verification(
        &self,
        user_id: impl Into<String>,
        email_verification: bool,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("emailVerification".to_string(), json!(email_verification));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/verification".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the user phone verification status by its unique ID.
    pub async fn update_phone_verification(
        &self,
        user_id: impl Into<String>,
        phone_verification: bool,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("phoneVerification".to_string(), json!(phone_verification));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/users/{userId}/verification/phone".to_string().replace("{userId}", &user_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Users {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_users_creation() {
        let client = Client::new();
        let service = Users::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
