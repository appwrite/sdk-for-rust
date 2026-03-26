//! Account service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Account service allows you to authenticate and manage a user account.
#[derive(Debug, Clone)]
pub struct Account {
    client: Client,
}

impl Account {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the currently logged in user.
    pub async fn get(
        &self,
    ) -> crate::error::Result<crate::models::User> {
        let params = HashMap::new();

        let path = "/account".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Use this endpoint to allow a new user to register a new account in your
    /// project. After the user registration completes successfully, you can use
    /// the
    /// [/account/verfication](https://appwrite.io/docs/references/cloud/client-web/account#createVerification)
    /// route to start verifying the user email address. To allow the new user to
    /// login to their new account, you need to create a new [account
    /// session](https://appwrite.io/docs/references/cloud/client-web/account#createEmailSession).
    pub async fn create(
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

        let path = "/account".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update currently logged in user account email address. After changing user
    /// address, the user confirmation status will get reset. A new confirmation
    /// email is not sent automatically however you can use the send confirmation
    /// email endpoint again to send the confirmation email. For security measures,
    /// user password is required to complete this request.
    /// This endpoint can also be used to convert an anonymous account to a normal
    /// one, by passing an email address and a new password.
    pub async fn update_email(
        &self,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/email".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get the list of identities for the currently logged in user.
    pub async fn list_identities(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::IdentityList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/account/identities".to_string();

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

        let path = "/account/identities/{identityId}".to_string().replace("{identityId}", &identity_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to create a JSON Web Token. You can use the resulting JWT
    /// to authenticate on behalf of the current user when working with the
    /// Appwrite server-side API and SDKs. The JWT secret is valid for 15 minutes
    /// from its creation and will be invalid if the user will logout in that time
    /// frame.
    pub async fn create_jwt(
        &self,
        duration: Option<i64>,
    ) -> crate::error::Result<crate::models::Jwt> {
        let mut params = HashMap::new();
        if let Some(value) = duration {
            params.insert("duration".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/jwts".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get the list of latest security activity logs for the currently logged in
    /// user. Each log returns user IP address, location and date and time of log.
    pub async fn list_logs(
        &self,
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

        let path = "/account/logs".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Enable or disable MFA on an account.
    pub async fn update_mfa(
        &self,
        mfa: bool,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("mfa".to_string(), json!(mfa));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Add an authenticator app to be used as an MFA factor. Verify the
    /// authenticator using the [verify
    /// authenticator](/docs/references/cloud/client-web/account#updateMfaAuthenticator)
    /// method.
    pub async fn create_mfa_authenticator(
        &self,
        r#type: crate::enums::AuthenticatorType,
    ) -> crate::error::Result<crate::models::MfaType> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/authenticators/{type}".to_string().replace("{type}", &r#type.to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Verify an authenticator app after adding it using the [add
    /// authenticator](/docs/references/cloud/client-web/account#createMfaAuthenticator)
    /// method.
    pub async fn update_mfa_authenticator(
        &self,
        r#type: crate::enums::AuthenticatorType,
        otp: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("otp".to_string(), json!(otp.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/authenticators/{type}".to_string().replace("{type}", &r#type.to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete an authenticator for a user by ID.
    pub async fn delete_mfa_authenticator(
        &self,
        r#type: crate::enums::AuthenticatorType,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/authenticators/{type}".to_string().replace("{type}", &r#type.to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Begin the process of MFA verification after sign-in. Finish the flow with
    /// [updateMfaChallenge](/docs/references/cloud/client-web/account#updateMfaChallenge)
    /// method.
    pub async fn create_mfa_challenge(
        &self,
        factor: crate::enums::AuthenticationFactor,
    ) -> crate::error::Result<crate::models::MfaChallenge> {
        let mut params = HashMap::new();
        params.insert("factor".to_string(), json!(factor));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/challenges".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Complete the MFA challenge by providing the one-time password. Finish the
    /// process of MFA verification by providing the one-time password. To begin
    /// the flow, use
    /// [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge)
    /// method.
    pub async fn update_mfa_challenge(
        &self,
        challenge_id: impl Into<String>,
        otp: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let mut params = HashMap::new();
        params.insert("challengeId".to_string(), json!(challenge_id.into()));
        params.insert("otp".to_string(), json!(otp.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/challenges".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// List the factors available on the account to be used as a MFA challange.
    pub async fn list_mfa_factors(
        &self,
    ) -> crate::error::Result<crate::models::MfaFactors> {
        let params = HashMap::new();

        let path = "/account/mfa/factors".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get recovery codes that can be used as backup for MFA flow. Before getting
    /// codes, they must be generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method. An OTP challenge is required to read recovery codes.
    pub async fn get_mfa_recovery_codes(
        &self,
    ) -> crate::error::Result<crate::models::MfaRecoveryCodes> {
        let params = HashMap::new();

        let path = "/account/mfa/recovery-codes".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Generate recovery codes as backup for MFA flow. It's recommended to
    /// generate and show then immediately after user successfully adds their
    /// authehticator. Recovery codes can be used as a MFA verification type in
    /// [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge)
    /// method.
    pub async fn create_mfa_recovery_codes(
        &self,
    ) -> crate::error::Result<crate::models::MfaRecoveryCodes> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/recovery-codes".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Regenerate recovery codes that can be used as backup for MFA flow. Before
    /// regenerating codes, they must be first generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method. An OTP challenge is required to regenreate recovery codes.
    pub async fn update_mfa_recovery_codes(
        &self,
    ) -> crate::error::Result<crate::models::MfaRecoveryCodes> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/mfa/recovery-codes".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update currently logged in user account name.
    pub async fn update_name(
        &self,
        name: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/name".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update currently logged in user password. For validation, user is required
    /// to pass in the new password, and the old password. For users created with
    /// OAuth, Team Invites and Magic URL, oldPassword is optional.
    pub async fn update_password(
        &self,
        password: impl Into<String>,
        old_password: Option<&str>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("password".to_string(), json!(password.into()));
        if let Some(value) = old_password {
            params.insert("oldPassword".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/password".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the currently logged in user's phone number. After updating the
    /// phone number, the phone verification status will be reset. A confirmation
    /// SMS is not sent automatically, however you can use the [POST
    /// /account/verification/phone](https://appwrite.io/docs/references/cloud/client-web/account#createPhoneVerification)
    /// endpoint to send a confirmation SMS.
    pub async fn update_phone(
        &self,
        phone: impl Into<String>,
        password: impl Into<String>,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("phone".to_string(), json!(phone.into()));
        params.insert("password".to_string(), json!(password.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/phone".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get the preferences as a key-value object for the currently logged in user.
    pub async fn get_prefs(
        &self,
    ) -> crate::error::Result<crate::models::Preferences> {
        let params = HashMap::new();

        let path = "/account/prefs".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update currently logged in user account preferences. The object you pass is
    /// stored as is, and replaces any previous value. The maximum allowed prefs
    /// size is 64kB and throws error if exceeded.
    pub async fn update_prefs(
        &self,
        prefs: serde_json::Value,
    ) -> crate::error::Result<crate::models::User> {
        let mut params = HashMap::new();
        params.insert("prefs".to_string(), json!(prefs));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/prefs".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Sends the user an email with a temporary secret key for password reset.
    /// When the user clicks the confirmation link he is redirected back to your
    /// app password reset URL with the secret key and email address values
    /// attached to the URL query string. Use the query string params to submit a
    /// request to the [PUT
    /// /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#updateRecovery)
    /// endpoint to complete the process. The verification link sent to the user's
    /// email address is valid for 1 hour.
    pub async fn create_recovery(
        &self,
        email: impl Into<String>,
        url: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("email".to_string(), json!(email.into()));
        params.insert("url".to_string(), json!(url.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/recovery".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to complete the user account password reset. Both the
    /// **userId** and **secret** arguments will be passed as query parameters to
    /// the redirect URL you have provided when sending your request to the [POST
    /// /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#createRecovery)
    /// endpoint.
    /// 
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md)
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    pub async fn update_recovery(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
        password: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        params.insert("password".to_string(), json!(password.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/recovery".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Get the list of active sessions across different devices for the currently
    /// logged in user.
    pub async fn list_sessions(
        &self,
    ) -> crate::error::Result<crate::models::SessionList> {
        let params = HashMap::new();

        let path = "/account/sessions".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete all sessions from the user account and remove any sessions cookies
    /// from the end client.
    pub async fn delete_sessions(
        &self,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions".to_string();

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to allow a new user to register an anonymous account in
    /// your project. This route will also create a new session for the user. To
    /// allow the new user to convert an anonymous account to a normal account, you
    /// need to update its [email and
    /// password](https://appwrite.io/docs/references/cloud/client-web/account#updateEmail)
    /// or create an [OAuth2
    /// session](https://appwrite.io/docs/references/cloud/client-web/account#CreateOAuth2Session).
    pub async fn create_anonymous_session(
        &self,
    ) -> crate::error::Result<crate::models::Session> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/anonymous".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Allow the user to login into their account by providing a valid email and
    /// password combination. This route will create a new session for the user.
    /// 
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_email_password_session(
        &self,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let mut params = HashMap::new();
        params.insert("email".to_string(), json!(email.into()));
        params.insert("password".to_string(), json!(password.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/email".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to create a session from token. Provide the **userId**
    /// and **secret** parameters from the successful response of authentication
    /// flows initiated by token creation. For example, magic URL and phone login.
    pub async fn update_magic_url_session(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/magic-url".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to create a session from token. Provide the **userId**
    /// and **secret** parameters from the successful response of authentication
    /// flows initiated by token creation. For example, magic URL and phone login.
    pub async fn update_phone_session(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/phone".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to create a session from token. Provide the **userId**
    /// and **secret** parameters from the successful response of authentication
    /// flows initiated by token creation. For example, magic URL and phone login.
    pub async fn create_session(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/token".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to get a logged in user's session using a Session ID.
    /// Inputting 'current' will return the current session being used.
    pub async fn get_session(
        &self,
        session_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let params = HashMap::new();

        let path = "/account/sessions/{sessionId}".to_string().replace("{sessionId}", &session_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Use this endpoint to extend a session's length. Extending a session is
    /// useful when session expiry is short. If the session was created using an
    /// OAuth provider, this endpoint refreshes the access token from the provider.
    pub async fn update_session(
        &self,
        session_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Session> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/{sessionId}".to_string().replace("{sessionId}", &session_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Logout the user. Use 'current' as the session ID to logout on this device,
    /// use a session ID to logout on another device. If you're looking to logout
    /// the user on all devices, use [Delete
    /// Sessions](https://appwrite.io/docs/references/cloud/client-web/account#deleteSessions)
    /// instead.
    pub async fn delete_session(
        &self,
        session_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/sessions/{sessionId}".to_string().replace("{sessionId}", &session_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Block the currently logged in user account. Behind the scene, the user
    /// record is not deleted but permanently blocked from any access. To
    /// completely delete a user, use the Users API instead.
    pub async fn update_status(
        &self,
    ) -> crate::error::Result<crate::models::User> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/status".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Sends the user an email with a secret key for creating a session. If the
    /// email address has never been used, a **new account is created** using the
    /// provided `userId`. Otherwise, if the email address is already attached to
    /// an account, the **user ID is ignored**. Then, the user will receive an
    /// email with the one-time password. Use the returned user ID and secret and
    /// submit a request to the [POST
    /// /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process. The secret sent to the user's email
    /// is valid for 15 minutes.
    /// 
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_email_token(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        phrase: Option<bool>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        if let Some(value) = phrase {
            params.insert("phrase".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/tokens/email".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Sends the user an email with a secret key for creating a session. If the
    /// provided user ID has not been registered, a new user will be created. When
    /// the user clicks the link in the email, the user is redirected back to the
    /// URL you provided with the secret key and userId values attached to the URL
    /// query string. Use the query string parameters to submit a request to the
    /// [POST
    /// /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process. The link sent to the user's email
    /// address is valid for 1 hour.
    /// 
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_magic_url_token(
        &self,
        user_id: impl Into<String>,
        email: impl Into<String>,
        url: Option<&str>,
        phrase: Option<bool>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("email".to_string(), json!(email.into()));
        if let Some(value) = url {
            params.insert("url".to_string(), json!(value));
        }
        if let Some(value) = phrase {
            params.insert("phrase".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/tokens/magic-url".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Allow the user to login to their account using the OAuth2 provider of their
    /// choice. Each OAuth2 provider should be enabled from the Appwrite console
    /// first. Use the success and failure arguments to provide a redirect URL's
    /// back to your app when login is completed.
    /// 
    /// If authentication succeeds, `userId` and `secret` of a token will be
    /// appended to the success URL as query parameters. These can be used to
    /// create a new session using the [Create
    /// session](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint.
    /// 
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_o_auth2_token(
        &self,
        provider: crate::enums::OAuthProvider,
        success: Option<&str>,
        failure: Option<&str>,
        scopes: Option<Vec<String>>,
    ) -> crate::error::Result<String> {
        let mut params = HashMap::new();
        if let Some(value) = success {
            params.insert("success".to_string(), json!(value));
        }
        if let Some(value) = failure {
            params.insert("failure".to_string(), json!(value));
        }
        if let Some(value) = scopes {
            params.insert("scopes".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }

        let path = "/account/tokens/oauth2/{provider}".to_string().replace("{provider}", &provider.to_string());

        self.client.call_location(Method::GET, &path, None, Some(params)).await
    }

    /// Sends the user an SMS with a secret key for creating a session. If the
    /// provided user ID has not be registered, a new user will be created. Use the
    /// returned user ID and secret and submit a request to the [POST
    /// /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process. The secret sent to the user's phone
    /// is valid for 15 minutes.
    /// 
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_phone_token(
        &self,
        user_id: impl Into<String>,
        phone: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("phone".to_string(), json!(phone.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/tokens/phone".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to send a verification message to your user email address
    /// to confirm they are the valid owners of that address. Both the **userId**
    /// and **secret** arguments will be passed as query parameters to the URL you
    /// have provided to be attached to the verification email. The provided URL
    /// should redirect the user back to your app and allow you to complete the
    /// verification process by verifying both the **userId** and **secret**
    /// parameters. Learn more about how to [complete the verification
    /// process](https://appwrite.io/docs/references/cloud/client-web/account#updateVerification).
    /// The verification link sent to the user's email address is valid for 7 days.
    /// 
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md),
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    pub async fn create_email_verification(
        &self,
        url: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("url".to_string(), json!(url.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/verifications/email".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to send a verification message to your user email address
    /// to confirm they are the valid owners of that address. Both the **userId**
    /// and **secret** arguments will be passed as query parameters to the URL you
    /// have provided to be attached to the verification email. The provided URL
    /// should redirect the user back to your app and allow you to complete the
    /// verification process by verifying both the **userId** and **secret**
    /// parameters. Learn more about how to [complete the verification
    /// process](https://appwrite.io/docs/references/cloud/client-web/account#updateVerification).
    /// The verification link sent to the user's email address is valid for 7 days.
    /// 
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md),
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    pub async fn create_verification(
        &self,
        url: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("url".to_string(), json!(url.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/verifications/email".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to complete the user email verification process. Use both
    /// the **userId** and **secret** parameters that were attached to your app URL
    /// to verify the user email ownership. If confirmed this route will return a
    /// 200 status code.
    pub async fn update_email_verification(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/verifications/email".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to complete the user email verification process. Use both
    /// the **userId** and **secret** parameters that were attached to your app URL
    /// to verify the user email ownership. If confirmed this route will return a
    /// 200 status code.
    pub async fn update_verification(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/verifications/email".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to send a verification SMS to the currently logged in
    /// user. This endpoint is meant for use after updating a user's phone number
    /// using the
    /// [accountUpdatePhone](https://appwrite.io/docs/references/cloud/client-web/account#updatePhone)
    /// endpoint. Learn more about how to [complete the verification
    /// process](https://appwrite.io/docs/references/cloud/client-web/account#updatePhoneVerification).
    /// The verification code sent to the user's phone number is valid for 15
    /// minutes.
    pub async fn create_phone_verification(
        &self,
    ) -> crate::error::Result<crate::models::Token> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/verifications/phone".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to complete the user phone verification process. Use the
    /// **userId** and **secret** that were sent to your user's phone number to
    /// verify the user email ownership. If confirmed this route will return a 200
    /// status code.
    pub async fn update_phone_verification(
        &self,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Token> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/account/verifications/phone".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Account {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_creation() {
        let client = Client::new();
        let service = Account::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
