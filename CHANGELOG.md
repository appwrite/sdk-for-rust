# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - TBD

### Added
- Initial release of Appwrite Rust SDK
- Full support for Appwrite API 1.9.0
- Async/await support with tokio runtime
- Built-in error handling with custom error types
- File upload support with automatic chunking
- Query builder for database operations
- Permission and role management utilities
- ID generation utilities
- Comprehensive documentation and examples
- Support for self-signed certificates
- Custom header support
- Automatic JSON serialization/deserialization

### Features
- Account service with 54 methods
- Activities service with 2 methods
- Avatars service with 8 methods
- Backups service with 12 methods
- Databases service with 69 methods
- Functions service with 26 methods
- Graphql service with 2 methods
- Health service with 24 methods
- Locale service with 8 methods
- Messaging service with 56 methods
- Project service with 5 methods
- Sites service with 25 methods
- Storage service with 13 methods
- TablesDB service with 69 methods
- Teams service with 13 methods
- Tokens service with 5 methods
- Users service with 49 methods
- Webhooks service with 6 methods

### Services
#### Account
The Account service allows you to authenticate and manage a user account.
- `get()` - Get the currently logged in user.
- `create()` - Use this endpoint to allow a new user to register a new account in your project. After the user registration completes successfully, you can use the [/account/verfication](https://appwrite.io/docs/references/cloud/client-web/account#createVerification) route to start verifying the user email address. To allow the new user to login to their new account, you need to create a new [account session](https://appwrite.io/docs/references/cloud/client-web/account#createEmailSession).
- `update_email()` - Update currently logged in user account email address. After changing user address, the user confirmation status will get reset. A new confirmation email is not sent automatically however you can use the send confirmation email endpoint again to send the confirmation email. For security measures, user password is required to complete this request.
This endpoint can also be used to convert an anonymous account to a normal one, by passing an email address and a new password.

- `list_identities()` - Get the list of identities for the currently logged in user.
- `delete_identity()` - Delete an identity by its unique ID.
- `create_jwt()` - Use this endpoint to create a JSON Web Token. You can use the resulting JWT to authenticate on behalf of the current user when working with the Appwrite server-side API and SDKs. The JWT secret is valid for 15 minutes from its creation and will be invalid if the user will logout in that time frame.
- `list_logs()` - Get the list of latest security activity logs for the currently logged in user. Each log returns user IP address, location and date and time of log.
- `update_mfa()` - Enable or disable MFA on an account.
- `create_mfa_authenticator()` - Add an authenticator app to be used as an MFA factor. Verify the authenticator using the [verify authenticator](/docs/references/cloud/client-web/account#updateMfaAuthenticator) method.
- `create_mfa_authenticator()` - Add an authenticator app to be used as an MFA factor. Verify the authenticator using the [verify authenticator](/docs/references/cloud/client-web/account#updateMfaAuthenticator) method.
- `update_mfa_authenticator()` - Verify an authenticator app after adding it using the [add authenticator](/docs/references/cloud/client-web/account#createMfaAuthenticator) method.
- `update_mfa_authenticator()` - Verify an authenticator app after adding it using the [add authenticator](/docs/references/cloud/client-web/account#createMfaAuthenticator) method.
- `delete_mfa_authenticator()` - Delete an authenticator for a user by ID.
- `delete_mfa_authenticator()` - Delete an authenticator for a user by ID.
- `create_mfa_challenge()` - Begin the process of MFA verification after sign-in. Finish the flow with [updateMfaChallenge](/docs/references/cloud/client-web/account#updateMfaChallenge) method.
- `create_mfa_challenge()` - Begin the process of MFA verification after sign-in. Finish the flow with [updateMfaChallenge](/docs/references/cloud/client-web/account#updateMfaChallenge) method.
- `update_mfa_challenge()` - Complete the MFA challenge by providing the one-time password. Finish the process of MFA verification by providing the one-time password. To begin the flow, use [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge) method.
- `update_mfa_challenge()` - Complete the MFA challenge by providing the one-time password. Finish the process of MFA verification by providing the one-time password. To begin the flow, use [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge) method.
- `list_mfa_factors()` - List the factors available on the account to be used as a MFA challange.
- `list_mfa_factors()` - List the factors available on the account to be used as a MFA challange.
- `get_mfa_recovery_codes()` - Get recovery codes that can be used as backup for MFA flow. Before getting codes, they must be generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method. An OTP challenge is required to read recovery codes.
- `get_mfa_recovery_codes()` - Get recovery codes that can be used as backup for MFA flow. Before getting codes, they must be generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method. An OTP challenge is required to read recovery codes.
- `create_mfa_recovery_codes()` - Generate recovery codes as backup for MFA flow. It&#039;s recommended to generate and show then immediately after user successfully adds their authehticator. Recovery codes can be used as a MFA verification type in [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge) method.
- `create_mfa_recovery_codes()` - Generate recovery codes as backup for MFA flow. It&#039;s recommended to generate and show then immediately after user successfully adds their authehticator. Recovery codes can be used as a MFA verification type in [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge) method.
- `update_mfa_recovery_codes()` - Regenerate recovery codes that can be used as backup for MFA flow. Before regenerating codes, they must be first generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method. An OTP challenge is required to regenreate recovery codes.
- `update_mfa_recovery_codes()` - Regenerate recovery codes that can be used as backup for MFA flow. Before regenerating codes, they must be first generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method. An OTP challenge is required to regenreate recovery codes.
- `update_name()` - Update currently logged in user account name.
- `update_password()` - Update currently logged in user password. For validation, user is required to pass in the new password, and the old password. For users created with OAuth, Team Invites and Magic URL, oldPassword is optional.
- `update_phone()` - Update the currently logged in user&#039;s phone number. After updating the phone number, the phone verification status will be reset. A confirmation SMS is not sent automatically, however you can use the [POST /account/verification/phone](https://appwrite.io/docs/references/cloud/client-web/account#createPhoneVerification) endpoint to send a confirmation SMS.
- `get_prefs()` - Get the preferences as a key-value object for the currently logged in user.
- `update_prefs()` - Update currently logged in user account preferences. The object you pass is stored as is, and replaces any previous value. The maximum allowed prefs size is 64kB and throws error if exceeded.
- `create_recovery()` - Sends the user an email with a temporary secret key for password reset. When the user clicks the confirmation link he is redirected back to your app password reset URL with the secret key and email address values attached to the URL query string. Use the query string params to submit a request to the [PUT /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#updateRecovery) endpoint to complete the process. The verification link sent to the user&#039;s email address is valid for 1 hour.
- `update_recovery()` - Use this endpoint to complete the user account password reset. Both the **userId** and **secret** arguments will be passed as query parameters to the redirect URL you have provided when sending your request to the [POST /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#createRecovery) endpoint.

Please note that in order to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md) the only valid redirect URLs are the ones from domains you have set when adding your platforms in the console interface.
- `list_sessions()` - Get the list of active sessions across different devices for the currently logged in user.
- `delete_sessions()` - Delete all sessions from the user account and remove any sessions cookies from the end client.
- `create_anonymous_session()` - Use this endpoint to allow a new user to register an anonymous account in your project. This route will also create a new session for the user. To allow the new user to convert an anonymous account to a normal account, you need to update its [email and password](https://appwrite.io/docs/references/cloud/client-web/account#updateEmail) or create an [OAuth2 session](https://appwrite.io/docs/references/cloud/client-web/account#CreateOAuth2Session).
- `create_email_password_session()` - Allow the user to login into their account by providing a valid email and password combination. This route will create a new session for the user.

A user is limited to 10 active sessions at a time by default. [Learn more about session limits](https://appwrite.io/docs/authentication-security#limits).
- `update_magic_url_session()` - Use this endpoint to create a session from token. Provide the **userId** and **secret** parameters from the successful response of authentication flows initiated by token creation. For example, magic URL and phone login.
- `update_phone_session()` - Use this endpoint to create a session from token. Provide the **userId** and **secret** parameters from the successful response of authentication flows initiated by token creation. For example, magic URL and phone login.
- `create_session()` - Use this endpoint to create a session from token. Provide the **userId** and **secret** parameters from the successful response of authentication flows initiated by token creation. For example, magic URL and phone login.
- `get_session()` - Use this endpoint to get a logged in user&#039;s session using a Session ID. Inputting &#039;current&#039; will return the current session being used.
- `update_session()` - Use this endpoint to extend a session&#039;s length. Extending a session is useful when session expiry is short. If the session was created using an OAuth provider, this endpoint refreshes the access token from the provider.
- `delete_session()` - Logout the user. Use &#039;current&#039; as the session ID to logout on this device, use a session ID to logout on another device. If you&#039;re looking to logout the user on all devices, use [Delete Sessions](https://appwrite.io/docs/references/cloud/client-web/account#deleteSessions) instead.
- `update_status()` - Block the currently logged in user account. Behind the scene, the user record is not deleted but permanently blocked from any access. To completely delete a user, use the Users API instead.
- `create_email_token()` - Sends the user an email with a secret key for creating a session. If the email address has never been used, a **new account is created** using the provided `userId`. Otherwise, if the email address is already attached to an account, the **user ID is ignored**. Then, the user will receive an email with the one-time password. Use the returned user ID and secret and submit a request to the [POST /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession) endpoint to complete the login process. The secret sent to the user&#039;s email is valid for 15 minutes.

A user is limited to 10 active sessions at a time by default. [Learn more about session limits](https://appwrite.io/docs/authentication-security#limits).

- `create_magic_url_token()` - Sends the user an email with a secret key for creating a session. If the provided user ID has not been registered, a new user will be created. When the user clicks the link in the email, the user is redirected back to the URL you provided with the secret key and userId values attached to the URL query string. Use the query string parameters to submit a request to the [POST /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession) endpoint to complete the login process. The link sent to the user&#039;s email address is valid for 1 hour.

A user is limited to 10 active sessions at a time by default. [Learn more about session limits](https://appwrite.io/docs/authentication-security#limits).

- `create_o_auth2_token()` - Allow the user to login to their account using the OAuth2 provider of their choice. Each OAuth2 provider should be enabled from the Appwrite console first. Use the success and failure arguments to provide a redirect URL&#039;s back to your app when login is completed. 

If authentication succeeds, `userId` and `secret` of a token will be appended to the success URL as query parameters. These can be used to create a new session using the [Create session](https://appwrite.io/docs/references/cloud/client-web/account#createSession) endpoint.

A user is limited to 10 active sessions at a time by default. [Learn more about session limits](https://appwrite.io/docs/authentication-security#limits).
- `create_phone_token()` - Sends the user an SMS with a secret key for creating a session. If the provided user ID has not be registered, a new user will be created. Use the returned user ID and secret and submit a request to the [POST /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession) endpoint to complete the login process. The secret sent to the user&#039;s phone is valid for 15 minutes.

A user is limited to 10 active sessions at a time by default. [Learn more about session limits](https://appwrite.io/docs/authentication-security#limits).
- `create_email_verification()` - Use this endpoint to send a verification message to your user email address to confirm they are the valid owners of that address. Both the **userId** and **secret** arguments will be passed as query parameters to the URL you have provided to be attached to the verification email. The provided URL should redirect the user back to your app and allow you to complete the verification process by verifying both the **userId** and **secret** parameters. Learn more about how to [complete the verification process](https://appwrite.io/docs/references/cloud/client-web/account#updateVerification). The verification link sent to the user&#039;s email address is valid for 7 days.

Please note that in order to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md), the only valid redirect URLs are the ones from domains you have set when adding your platforms in the console interface.

- `create_verification()` - Use this endpoint to send a verification message to your user email address to confirm they are the valid owners of that address. Both the **userId** and **secret** arguments will be passed as query parameters to the URL you have provided to be attached to the verification email. The provided URL should redirect the user back to your app and allow you to complete the verification process by verifying both the **userId** and **secret** parameters. Learn more about how to [complete the verification process](https://appwrite.io/docs/references/cloud/client-web/account#updateVerification). The verification link sent to the user&#039;s email address is valid for 7 days.

Please note that in order to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md), the only valid redirect URLs are the ones from domains you have set when adding your platforms in the console interface.

- `update_email_verification()` - Use this endpoint to complete the user email verification process. Use both the **userId** and **secret** parameters that were attached to your app URL to verify the user email ownership. If confirmed this route will return a 200 status code.
- `update_verification()` - Use this endpoint to complete the user email verification process. Use both the **userId** and **secret** parameters that were attached to your app URL to verify the user email ownership. If confirmed this route will return a 200 status code.
- `create_phone_verification()` - Use this endpoint to send a verification SMS to the currently logged in user. This endpoint is meant for use after updating a user&#039;s phone number using the [accountUpdatePhone](https://appwrite.io/docs/references/cloud/client-web/account#updatePhone) endpoint. Learn more about how to [complete the verification process](https://appwrite.io/docs/references/cloud/client-web/account#updatePhoneVerification). The verification code sent to the user&#039;s phone number is valid for 15 minutes.
- `update_phone_verification()` - Use this endpoint to complete the user phone verification process. Use the **userId** and **secret** that were sent to your user&#039;s phone number to verify the user email ownership. If confirmed this route will return a 200 status code.

#### Activities

- `list_events()` - List all events for selected filters.
- `get_event()` - Get event by ID.


#### Avatars
The Avatars service aims to help you complete everyday tasks related to your app image, icons, and avatars.
- `get_browser()` - You can use this endpoint to show different browser icons to your users. The code argument receives the browser code as it appears in your user [GET /account/sessions](https://appwrite.io/docs/references/cloud/client-web/account#getSessions) endpoint. Use width, height and quality arguments to change the output settings.

When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px.
- `get_credit_card()` - The credit card endpoint will return you the icon of the credit card provider you need. Use width, height and quality arguments to change the output settings.

When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px.

- `get_favicon()` - Use this endpoint to fetch the favorite icon (AKA favicon) of any remote website URL.

This endpoint does not follow HTTP redirects.
- `get_flag()` - You can use this endpoint to show different country flags icons to your users. The code argument receives the 2 letter country code. Use width, height and quality arguments to change the output settings. Country codes follow the [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) standard.

When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px.

- `get_image()` - Use this endpoint to fetch a remote image URL and crop it to any image size you want. This endpoint is very useful if you need to crop and display remote images in your app or in case you want to make sure a 3rd party image is properly served using a TLS protocol.

When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 400x400px.

This endpoint does not follow HTTP redirects.
- `get_initials()` - Use this endpoint to show your user initials avatar icon on your website or app. By default, this route will try to print your logged-in user name or email initials. You can also overwrite the user name if you pass the &#039;name&#039; parameter. If no name is given and no user is logged, an empty avatar will be returned.

You can use the color and background params to change the avatar colors. By default, a random theme will be selected. The random theme will persist for the user&#039;s initials when reloading the same theme will always return for the same initials.

When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px.

- `get_qr()` - Converts a given plain text to a QR code image. You can use the query parameters to change the size and style of the resulting image.

- `get_screenshot()` - Use this endpoint to capture a screenshot of any website URL. This endpoint uses a headless browser to render the webpage and capture it as an image.

You can configure the browser viewport size, theme, user agent, geolocation, permissions, and more. Capture either just the viewport or the full page scroll.

When width and height are specified, the image is resized accordingly. If both dimensions are 0, the API provides an image at original size. If dimensions are not specified, the default viewport size is 1280x720px.

#### Backups

- `list_archives()` - List all archives for a project.
- `create_archive()` - Create a new archive asynchronously for a project.
- `get_archive()` - Get a backup archive using it&#039;s ID.
- `delete_archive()` - Delete an existing archive for a project.
- `list_policies()` - List all policies for a project.
- `create_policy()` - Create a new backup policy.
- `get_policy()` - Get a backup policy using it&#039;s ID.
- `update_policy()` - Update an existing policy using it&#039;s ID.
- `delete_policy()` - Delete a policy using it&#039;s ID.
- `create_restoration()` - Create and trigger a new restoration for a backup on a project.
- `list_restorations()` - List all backup restorations for a project.
- `get_restoration()` - Get the current status of a backup restoration.

#### Databases
The Databases service allows you to create structured collections of documents, query and filter lists of documents
- `list()` - Get a list of all databases from the current Appwrite project. You can use the search parameter to filter your results.
- `create()` - Create a new Database.

- `list_transactions()` - List transactions across all databases.
- `create_transaction()` - Create a new transaction.
- `get_transaction()` - Get a transaction by its unique ID.
- `update_transaction()` - Update a transaction, to either commit or roll back its operations.
- `delete_transaction()` - Delete a transaction by its unique ID.
- `create_operations()` - Create multiple operations in a single transaction.
- `get()` - Get a database by its unique ID. This endpoint response returns a JSON object with the database metadata.
- `update()` - Update a database by its unique ID.
- `delete()` - Delete a database by its unique ID. Only API keys with with databases.write scope can delete a database.
- `list_collections()` - Get a list of all collections that belong to the provided databaseId. You can use the search parameter to filter your results.
- `create_collection()` - Create a new Collection. Before using this route, you should create a new database resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.
- `get_collection()` - Get a collection by its unique ID. This endpoint response returns a JSON object with the collection metadata.
- `update_collection()` - Update a collection by its unique ID.
- `delete_collection()` - Delete a collection by its unique ID. Only users with write permissions have access to delete this resource.
- `list_attributes()` - List attributes in the collection.
- `create_boolean_attribute()` - Create a boolean attribute.

- `update_boolean_attribute()` - Update a boolean attribute. Changing the `default` value will not update already existing documents.
- `create_datetime_attribute()` - Create a date time attribute according to the ISO 8601 standard.
- `update_datetime_attribute()` - Update a date time attribute. Changing the `default` value will not update already existing documents.
- `create_email_attribute()` - Create an email attribute.

- `update_email_attribute()` - Update an email attribute. Changing the `default` value will not update already existing documents.

- `create_enum_attribute()` - Create an enum attribute. The `elements` param acts as a white-list of accepted values for this attribute. 

- `update_enum_attribute()` - Update an enum attribute. Changing the `default` value will not update already existing documents.

- `create_float_attribute()` - Create a float attribute. Optionally, minimum and maximum values can be provided.

- `update_float_attribute()` - Update a float attribute. Changing the `default` value will not update already existing documents.

- `create_integer_attribute()` - Create an integer attribute. Optionally, minimum and maximum values can be provided.

- `update_integer_attribute()` - Update an integer attribute. Changing the `default` value will not update already existing documents.

- `create_ip_attribute()` - Create IP address attribute.

- `update_ip_attribute()` - Update an ip attribute. Changing the `default` value will not update already existing documents.

- `create_line_attribute()` - Create a geometric line attribute.
- `update_line_attribute()` - Update a line attribute. Changing the `default` value will not update already existing documents.
- `create_longtext_attribute()` - Create a longtext attribute.

- `update_longtext_attribute()` - Update a longtext attribute. Changing the `default` value will not update already existing documents.

- `create_mediumtext_attribute()` - Create a mediumtext attribute.

- `update_mediumtext_attribute()` - Update a mediumtext attribute. Changing the `default` value will not update already existing documents.

- `create_point_attribute()` - Create a geometric point attribute.
- `update_point_attribute()` - Update a point attribute. Changing the `default` value will not update already existing documents.
- `create_polygon_attribute()` - Create a geometric polygon attribute.
- `update_polygon_attribute()` - Update a polygon attribute. Changing the `default` value will not update already existing documents.
- `create_relationship_attribute()` - Create relationship attribute. [Learn more about relationship attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).

- `update_relationship_attribute()` - Update relationship attribute. [Learn more about relationship attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).

- `create_string_attribute()` - Create a string attribute.

- `update_string_attribute()` - Update a string attribute. Changing the `default` value will not update already existing documents.

- `create_text_attribute()` - Create a text attribute.

- `update_text_attribute()` - Update a text attribute. Changing the `default` value will not update already existing documents.

- `create_url_attribute()` - Create a URL attribute.

- `update_url_attribute()` - Update an url attribute. Changing the `default` value will not update already existing documents.

- `create_varchar_attribute()` - Create a varchar attribute.

- `update_varchar_attribute()` - Update a varchar attribute. Changing the `default` value will not update already existing documents.

- `get_attribute()` - Get attribute by ID.
- `delete_attribute()` - Deletes an attribute.
- `list_documents()` - Get a list of all the user&#039;s documents in a given collection. You can use the query params to filter your results.
- `create_document()` - Create a new Document. Before using this route, you should create a new collection resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.
- `create_documents()` - Create new Documents. Before using this route, you should create a new collection resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.
- `upsert_documents()` - Create or update Documents. Before using this route, you should create a new collection resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.

- `update_documents()` - Update all documents that match your queries, if no queries are submitted then all documents are updated. You can pass only specific fields to be updated.
- `delete_documents()` - Bulk delete documents using queries, if no queries are passed then all documents are deleted.
- `get_document()` - Get a document by its unique ID. This endpoint response returns a JSON object with the document data.
- `upsert_document()` - Create or update a Document. Before using this route, you should create a new collection resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.
- `update_document()` - Update a document by its unique ID. Using the patch method you can pass only specific fields that will get updated.
- `delete_document()` - Delete a document by its unique ID.
- `decrement_document_attribute()` - Decrement a specific attribute of a document by a given value.
- `increment_document_attribute()` - Increment a specific attribute of a document by a given value.
- `list_indexes()` - List indexes in the collection.
- `create_index()` - Creates an index on the attributes listed. Your index should include all the attributes you will query in a single request.
Attributes can be `key`, `fulltext`, and `unique`.
- `get_index()` - Get an index by its unique ID.
- `delete_index()` - Delete an index.

#### Functions
The Functions Service allows you view, create and manage your Cloud Functions.
- `list()` - Get a list of all the project&#039;s functions. You can use the query params to filter your results.
- `create()` - Create a new function. You can pass a list of [permissions](https://appwrite.io/docs/permissions) to allow different project users or team with access to execute the function using the client API.
- `list_runtimes()` - Get a list of all runtimes that are currently active on your instance.
- `list_specifications()` - List allowed function specifications for this instance.
- `get()` - Get a function by its unique ID.
- `update()` - Update function by its unique ID.
- `delete()` - Delete a function by its unique ID.
- `update_function_deployment()` - Update the function active deployment. Use this endpoint to switch the code deployment that should be used when visitor opens your function.
- `list_deployments()` - Get a list of all the function&#039;s code deployments. You can use the query params to filter your results.
- `create_deployment()` - Create a new function code deployment. Use this endpoint to upload a new version of your code function. To execute your newly uploaded code, you&#039;ll need to update the function&#039;s deployment to use your new deployment UID.

This endpoint accepts a tar.gz file compressed with your code. Make sure to include any dependencies your code has within the compressed file. You can learn more about code packaging in the [Appwrite Cloud Functions tutorial](https://appwrite.io/docs/functions).

Use the &quot;command&quot; param to set the entrypoint used to execute your code.
- `create_duplicate_deployment()` - Create a new build for an existing function deployment. This endpoint allows you to rebuild a deployment with the updated function configuration, including its entrypoint and build commands if they have been modified. The build process will be queued and executed asynchronously. The original deployment&#039;s code will be preserved and used for the new build.
- `create_template_deployment()` - Create a deployment based on a template.

Use this endpoint with combination of [listTemplates](https://appwrite.io/docs/products/functions/templates) to find the template details.
- `create_vcs_deployment()` - Create a deployment when a function is connected to VCS.

This endpoint lets you create deployment from a branch, commit, or a tag.
- `get_deployment()` - Get a function deployment by its unique ID.
- `delete_deployment()` - Delete a code deployment by its unique ID.
- `get_deployment_download()` - Get a function deployment content by its unique ID. The endpoint response return with a &#039;Content-Disposition: attachment&#039; header that tells the browser to start downloading the file to user downloads directory.
- `update_deployment_status()` - Cancel an ongoing function deployment build. If the build is already in progress, it will be stopped and marked as canceled. If the build hasn&#039;t started yet, it will be marked as canceled without executing. You cannot cancel builds that have already completed (status &#039;ready&#039;) or failed. The response includes the final build status and details.
- `list_executions()` - Get a list of all the current user function execution logs. You can use the query params to filter your results.
- `create_execution()` - Trigger a function execution. The returned object will return you the current execution status. You can ping the `Get Execution` endpoint to get updates on the current execution status. Once this endpoint is called, your function execution process will start asynchronously.
- `get_execution()` - Get a function execution log by its unique ID.
- `delete_execution()` - Delete a function execution by its unique ID.
- `list_variables()` - Get a list of all variables of a specific function.
- `create_variable()` - Create a new function environment variable. These variables can be accessed in the function at runtime as environment variables.
- `get_variable()` - Get a variable by its unique ID.
- `update_variable()` - Update variable by its unique ID.
- `delete_variable()` - Delete a variable by its unique ID.

#### Graphql
The GraphQL API allows you to query and mutate your Appwrite server using GraphQL.
- `query()` - Execute a GraphQL mutation.
- `mutation()` - Execute a GraphQL mutation.

#### Health
The Health service allows you to both validate and monitor your Appwrite server&#039;s health.
- `get()` - Check the Appwrite HTTP server is up and responsive.
- `get_antivirus()` - Check the Appwrite Antivirus server is up and connection is successful.
- `get_cache()` - Check the Appwrite in-memory cache servers are up and connection is successful.
- `get_certificate()` - Get the SSL certificate for a domain
- `get_console_pausing()` - Get console pausing health status. Monitors projects approaching the pause threshold to detect potential issues with console access tracking.

- `get_db()` - Check the Appwrite database servers are up and connection is successful.
- `get_pub_sub()` - Check the Appwrite pub-sub servers are up and connection is successful.
- `get_queue_audits()` - Get the number of audit logs that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_builds()` - Get the number of builds that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_certificates()` - Get the number of certificates that are waiting to be issued against [Letsencrypt](https://letsencrypt.org/) in the Appwrite internal queue server.
- `get_queue_databases()` - Get the number of database changes that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_deletes()` - Get the number of background destructive changes that are waiting to be processed in the Appwrite internal queue server.
- `get_failed_jobs()` - Returns the amount of failed jobs in a given queue.

- `get_queue_functions()` - Get the number of function executions that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_logs()` - Get the number of logs that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_mails()` - Get the number of mails that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_messaging()` - Get the number of messages that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_migrations()` - Get the number of migrations that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_stats_resources()` - Get the number of metrics that are waiting to be processed in the Appwrite stats resources queue.
- `get_queue_usage()` - Get the number of metrics that are waiting to be processed in the Appwrite internal queue server.
- `get_queue_webhooks()` - Get the number of webhooks that are waiting to be processed in the Appwrite internal queue server.
- `get_storage()` - Check the Appwrite storage device is up and connection is successful.
- `get_storage_local()` - Check the Appwrite local storage device is up and connection is successful.
- `get_time()` - Check the Appwrite server time is synced with Google remote NTP server. We use this technology to smoothly handle leap seconds with no disruptive events. The [Network Time Protocol](https://en.wikipedia.org/wiki/Network_Time_Protocol) (NTP) is used by hundreds of millions of computers and devices to synchronize their clocks over the Internet. If your computer sets its own clock, it likely uses NTP.

#### Locale
The Locale service allows you to customize your app based on your users&#039; location.
- `get()` - Get the current user location based on IP. Returns an object with user country code, country name, continent name, continent code, ip address and suggested currency. You can use the locale header to get the data in a supported language.

([IP Geolocation by DB-IP](https://db-ip.com))
- `list_codes()` - List of all locale codes in [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
- `list_continents()` - List of all continents. You can use the locale header to get the data in a supported language.
- `list_countries()` - List of all countries. You can use the locale header to get the data in a supported language.
- `list_countries_eu()` - List of all countries that are currently members of the EU. You can use the locale header to get the data in a supported language.
- `list_countries_phones()` - List of all countries phone codes. You can use the locale header to get the data in a supported language.
- `list_currencies()` - List of all currencies, including currency symbol, name, plural, and decimal digits for all major and minor currencies. You can use the locale header to get the data in a supported language.
- `list_languages()` - List of all languages classified by ISO 639-1 including 2-letter code, name in English, and name in the respective language.

#### Messaging
The Messaging service allows you to send messages to any provider type (SMTP, push notification, SMS, etc.).
- `list_messages()` - Get a list of all messages from the current Appwrite project.
- `create_email()` - Create a new email message.
- `update_email()` - Update an email message by its unique ID. This endpoint only works on messages that are in draft status. Messages that are already processing, sent, or failed cannot be updated.

- `create_push()` - Create a new push notification.
- `update_push()` - Update a push notification by its unique ID. This endpoint only works on messages that are in draft status. Messages that are already processing, sent, or failed cannot be updated.

- `create_sms()` - Create a new SMS message.
- `create_sms()` - Create a new SMS message.
- `update_sms()` - Update an SMS message by its unique ID. This endpoint only works on messages that are in draft status. Messages that are already processing, sent, or failed cannot be updated.

- `update_sms()` - Update an SMS message by its unique ID. This endpoint only works on messages that are in draft status. Messages that are already processing, sent, or failed cannot be updated.

- `get_message()` - Get a message by its unique ID.

- `delete()` - Delete a message. If the message is not a draft or scheduled, but has been sent, this will not recall the message.
- `list_message_logs()` - Get the message activity logs listed by its unique ID.
- `list_targets()` - Get a list of the targets associated with a message.
- `list_providers()` - Get a list of all providers from the current Appwrite project.
- `create_apns_provider()` - Create a new Apple Push Notification service provider.
- `create_apns_provider()` - Create a new Apple Push Notification service provider.
- `update_apns_provider()` - Update a Apple Push Notification service provider by its unique ID.
- `update_apns_provider()` - Update a Apple Push Notification service provider by its unique ID.
- `create_fcm_provider()` - Create a new Firebase Cloud Messaging provider.
- `create_fcm_provider()` - Create a new Firebase Cloud Messaging provider.
- `update_fcm_provider()` - Update a Firebase Cloud Messaging provider by its unique ID.
- `update_fcm_provider()` - Update a Firebase Cloud Messaging provider by its unique ID.
- `create_mailgun_provider()` - Create a new Mailgun provider.
- `update_mailgun_provider()` - Update a Mailgun provider by its unique ID.
- `create_msg91_provider()` - Create a new MSG91 provider.
- `update_msg91_provider()` - Update a MSG91 provider by its unique ID.
- `create_resend_provider()` - Create a new Resend provider.
- `update_resend_provider()` - Update a Resend provider by its unique ID.
- `create_sendgrid_provider()` - Create a new Sendgrid provider.
- `update_sendgrid_provider()` - Update a Sendgrid provider by its unique ID.
- `create_smtp_provider()` - Create a new SMTP provider.
- `create_smtp_provider()` - Create a new SMTP provider.
- `update_smtp_provider()` - Update a SMTP provider by its unique ID.
- `update_smtp_provider()` - Update a SMTP provider by its unique ID.
- `create_telesign_provider()` - Create a new Telesign provider.
- `update_telesign_provider()` - Update a Telesign provider by its unique ID.
- `create_textmagic_provider()` - Create a new Textmagic provider.
- `update_textmagic_provider()` - Update a Textmagic provider by its unique ID.
- `create_twilio_provider()` - Create a new Twilio provider.
- `update_twilio_provider()` - Update a Twilio provider by its unique ID.
- `create_vonage_provider()` - Create a new Vonage provider.
- `update_vonage_provider()` - Update a Vonage provider by its unique ID.
- `get_provider()` - Get a provider by its unique ID.

- `delete_provider()` - Delete a provider by its unique ID.
- `list_provider_logs()` - Get the provider activity logs listed by its unique ID.
- `list_subscriber_logs()` - Get the subscriber activity logs listed by its unique ID.
- `list_topics()` - Get a list of all topics from the current Appwrite project.
- `create_topic()` - Create a new topic.
- `get_topic()` - Get a topic by its unique ID.

- `update_topic()` - Update a topic by its unique ID.

- `delete_topic()` - Delete a topic by its unique ID.
- `list_topic_logs()` - Get the topic activity logs listed by its unique ID.
- `list_subscribers()` - Get a list of all subscribers from the current Appwrite project.
- `create_subscriber()` - Create a new subscriber.
- `get_subscriber()` - Get a subscriber by its unique ID.

- `delete_subscriber()` - Delete a subscriber by its unique ID.

#### Project
The Project service allows you to manage all the projects in your Appwrite server.
- `list_variables()` - Get a list of all project environment variables.
- `create_variable()` - Create a new project environment variable. These variables can be accessed by all functions and sites in the project.
- `get_variable()` - Get a variable by its unique ID. 
- `update_variable()` - Update variable by its unique ID.
- `delete_variable()` - Delete a variable by its unique ID. 

#### Sites
The Sites Service allows you view, create and manage your web applications.
- `list()` - Get a list of all the project&#039;s sites. You can use the query params to filter your results.
- `create()` - Create a new site.
- `list_frameworks()` - Get a list of all frameworks that are currently available on the server instance.
- `list_specifications()` - List allowed site specifications for this instance.
- `get()` - Get a site by its unique ID.
- `update()` - Update site by its unique ID.
- `delete()` - Delete a site by its unique ID.
- `update_site_deployment()` - Update the site active deployment. Use this endpoint to switch the code deployment that should be used when visitor opens your site.
- `list_deployments()` - Get a list of all the site&#039;s code deployments. You can use the query params to filter your results.
- `create_deployment()` - Create a new site code deployment. Use this endpoint to upload a new version of your site code. To activate your newly uploaded code, you&#039;ll need to update the site&#039;s deployment to use your new deployment ID.
- `create_duplicate_deployment()` - Create a new build for an existing site deployment. This endpoint allows you to rebuild a deployment with the updated site configuration, including its commands and output directory if they have been modified. The build process will be queued and executed asynchronously. The original deployment&#039;s code will be preserved and used for the new build.
- `create_template_deployment()` - Create a deployment based on a template.

Use this endpoint with combination of [listTemplates](https://appwrite.io/docs/products/sites/templates) to find the template details.
- `create_vcs_deployment()` - Create a deployment when a site is connected to VCS.

This endpoint lets you create deployment from a branch, commit, or a tag.
- `get_deployment()` - Get a site deployment by its unique ID.
- `delete_deployment()` - Delete a site deployment by its unique ID.
- `get_deployment_download()` - Get a site deployment content by its unique ID. The endpoint response return with a &#039;Content-Disposition: attachment&#039; header that tells the browser to start downloading the file to user downloads directory.
- `update_deployment_status()` - Cancel an ongoing site deployment build. If the build is already in progress, it will be stopped and marked as canceled. If the build hasn&#039;t started yet, it will be marked as canceled without executing. You cannot cancel builds that have already completed (status &#039;ready&#039;) or failed. The response includes the final build status and details.
- `list_logs()` - Get a list of all site logs. You can use the query params to filter your results.
- `get_log()` - Get a site request log by its unique ID.
- `delete_log()` - Delete a site log by its unique ID.
- `list_variables()` - Get a list of all variables of a specific site.
- `create_variable()` - Create a new site variable. These variables can be accessed during build and runtime (server-side rendering) as environment variables.
- `get_variable()` - Get a variable by its unique ID.
- `update_variable()` - Update variable by its unique ID.
- `delete_variable()` - Delete a variable by its unique ID.

#### Storage
The Storage service allows you to manage your project files.
- `list_buckets()` - Get a list of all the storage buckets. You can use the query params to filter your results.
- `create_bucket()` - Create a new storage bucket.
- `get_bucket()` - Get a storage bucket by its unique ID. This endpoint response returns a JSON object with the storage bucket metadata.
- `update_bucket()` - Update a storage bucket by its unique ID.
- `delete_bucket()` - Delete a storage bucket by its unique ID.
- `list_files()` - Get a list of all the user files. You can use the query params to filter your results.
- `create_file()` - Create a new file. Before using this route, you should create a new bucket resource using either a [server integration](https://appwrite.io/docs/server/storage#storageCreateBucket) API or directly from your Appwrite console.

Larger files should be uploaded using multiple requests with the [content-range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range) header to send a partial request with a maximum supported chunk of `5MB`. The `content-range` header values should always be in bytes.

When the first request is sent, the server will return the **File** object, and the subsequent part request must include the file&#039;s **id** in `x-appwrite-id` header to allow the server to know that the partial upload is for the existing file and not for a new one.

If you&#039;re creating a new file using one of the Appwrite SDKs, all the chunking logic will be managed by the SDK internally.

- `get_file()` - Get a file by its unique ID. This endpoint response returns a JSON object with the file metadata.
- `update_file()` - Update a file by its unique ID. Only users with write permissions have access to update this resource.
- `delete_file()` - Delete a file by its unique ID. Only users with write permissions have access to delete this resource.
- `get_file_download()` - Get a file content by its unique ID. The endpoint response return with a &#039;Content-Disposition: attachment&#039; header that tells the browser to start downloading the file to user downloads directory.
- `get_file_preview()` - Get a file preview image. Currently, this method supports preview for image files (jpg, png, and gif), other supported formats, like pdf, docs, slides, and spreadsheets, will return the file icon image. You can also pass query string arguments for cutting and resizing your preview image. Preview is supported only for image files smaller than 10MB.
- `get_file_view()` - Get a file content by its unique ID. This endpoint is similar to the download method but returns with no  &#039;Content-Disposition: attachment&#039; header.

#### TablesDB

- `list()` - Get a list of all databases from the current Appwrite project. You can use the search parameter to filter your results.
- `create()` - Create a new Database.

- `list_transactions()` - List transactions across all databases.
- `create_transaction()` - Create a new transaction.
- `get_transaction()` - Get a transaction by its unique ID.
- `update_transaction()` - Update a transaction, to either commit or roll back its operations.
- `delete_transaction()` - Delete a transaction by its unique ID.
- `create_operations()` - Create multiple operations in a single transaction.
- `get()` - Get a database by its unique ID. This endpoint response returns a JSON object with the database metadata.
- `update()` - Update a database by its unique ID.
- `delete()` - Delete a database by its unique ID. Only API keys with with databases.write scope can delete a database.
- `list_tables()` - Get a list of all tables that belong to the provided databaseId. You can use the search parameter to filter your results.
- `create_table()` - Create a new Table. Before using this route, you should create a new database resource using either a [server integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable) API or directly from your database console.
- `get_table()` - Get a table by its unique ID. This endpoint response returns a JSON object with the table metadata.
- `update_table()` - Update a table by its unique ID.
- `delete_table()` - Delete a table by its unique ID. Only users with write permissions have access to delete this resource.
- `list_columns()` - List columns in the table.
- `create_boolean_column()` - Create a boolean column.

- `update_boolean_column()` - Update a boolean column. Changing the `default` value will not update already existing rows.
- `create_datetime_column()` - Create a date time column according to the ISO 8601 standard.
- `update_datetime_column()` - Update a date time column. Changing the `default` value will not update already existing rows.
- `create_email_column()` - Create an email column.

- `update_email_column()` - Update an email column. Changing the `default` value will not update already existing rows.

- `create_enum_column()` - Create an enumeration column. The `elements` param acts as a white-list of accepted values for this column.
- `update_enum_column()` - Update an enum column. Changing the `default` value will not update already existing rows.

- `create_float_column()` - Create a float column. Optionally, minimum and maximum values can be provided.

- `update_float_column()` - Update a float column. Changing the `default` value will not update already existing rows.

- `create_integer_column()` - Create an integer column. Optionally, minimum and maximum values can be provided.

- `update_integer_column()` - Update an integer column. Changing the `default` value will not update already existing rows.

- `create_ip_column()` - Create IP address column.

- `update_ip_column()` - Update an ip column. Changing the `default` value will not update already existing rows.

- `create_line_column()` - Create a geometric line column.
- `update_line_column()` - Update a line column. Changing the `default` value will not update already existing rows.
- `create_longtext_column()` - Create a longtext column.

- `update_longtext_column()` - Update a longtext column. Changing the `default` value will not update already existing rows.

- `create_mediumtext_column()` - Create a mediumtext column.

- `update_mediumtext_column()` - Update a mediumtext column. Changing the `default` value will not update already existing rows.

- `create_point_column()` - Create a geometric point column.
- `update_point_column()` - Update a point column. Changing the `default` value will not update already existing rows.
- `create_polygon_column()` - Create a geometric polygon column.
- `update_polygon_column()` - Update a polygon column. Changing the `default` value will not update already existing rows.
- `create_relationship_column()` - Create relationship column. [Learn more about relationship columns](https://appwrite.io/docs/databases-relationships#relationship-columns).

- `create_string_column()` - Create a string column.

- `update_string_column()` - Update a string column. Changing the `default` value will not update already existing rows.

- `create_text_column()` - Create a text column.

- `update_text_column()` - Update a text column. Changing the `default` value will not update already existing rows.

- `create_url_column()` - Create a URL column.

- `update_url_column()` - Update an url column. Changing the `default` value will not update already existing rows.

- `create_varchar_column()` - Create a varchar column.

- `update_varchar_column()` - Update a varchar column. Changing the `default` value will not update already existing rows.

- `get_column()` - Get column by ID.
- `delete_column()` - Deletes a column.
- `update_relationship_column()` - Update relationship column. [Learn more about relationship columns](https://appwrite.io/docs/databases-relationships#relationship-columns).

- `list_indexes()` - List indexes on the table.
- `create_index()` - Creates an index on the columns listed. Your index should include all the columns you will query in a single request.
Type can be `key`, `fulltext`, or `unique`.
- `get_index()` - Get index by ID.
- `delete_index()` - Delete an index.
- `list_rows()` - Get a list of all the user&#039;s rows in a given table. You can use the query params to filter your results.
- `create_row()` - Create a new Row. Before using this route, you should create a new table resource using either a [server integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable) API or directly from your database console.
- `create_rows()` - Create new Rows. Before using this route, you should create a new table resource using either a [server integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable) API or directly from your database console.
- `upsert_rows()` - Create or update Rows. Before using this route, you should create a new table resource using either a [server integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable) API or directly from your database console.

- `update_rows()` - Update all rows that match your queries, if no queries are submitted then all rows are updated. You can pass only specific fields to be updated.
- `delete_rows()` - Bulk delete rows using queries, if no queries are passed then all rows are deleted.
- `get_row()` - Get a row by its unique ID. This endpoint response returns a JSON object with the row data.
- `upsert_row()` - Create or update a Row. Before using this route, you should create a new table resource using either a [server integration](https://appwrite.io/docs/references/cloud/server-dart/tablesDB#createTable) API or directly from your database console.
- `update_row()` - Update a row by its unique ID. Using the patch method you can pass only specific fields that will get updated.
- `delete_row()` - Delete a row by its unique ID.
- `decrement_row_column()` - Decrement a specific column of a row by a given value.
- `increment_row_column()` - Increment a specific column of a row by a given value.

#### Teams
The Teams service allows you to group users of your project and to enable them to share read and write access to your project resources
- `list()` - Get a list of all the teams in which the current user is a member. You can use the parameters to filter your results.
- `create()` - Create a new team. The user who creates the team will automatically be assigned as the owner of the team. Only the users with the owner role can invite new members, add new owners and delete or update the team.
- `get()` - Get a team by its ID. All team members have read access for this resource.
- `update_name()` - Update the team&#039;s name by its unique ID.
- `delete()` - Delete a team using its ID. Only team members with the owner role can delete the team.
- `list_memberships()` - Use this endpoint to list a team&#039;s members using the team&#039;s ID. All team members have read access to this endpoint. Hide sensitive attributes from the response by toggling membership privacy in the Console.
- `create_membership()` - Invite a new member to join your team. Provide an ID for existing users, or invite unregistered users using an email or phone number. If initiated from a Client SDK, Appwrite will send an email or sms with a link to join the team to the invited user, and an account will be created for them if one doesn&#039;t exist. If initiated from a Server SDK, the new member will be added automatically to the team.

You only need to provide one of a user ID, email, or phone number. Appwrite will prioritize accepting the user ID &gt; email &gt; phone number if you provide more than one of these parameters.

Use the `url` parameter to redirect the user from the invitation email to your app. After the user is redirected, use the [Update Team Membership Status](https://appwrite.io/docs/references/cloud/client-web/teams#updateMembershipStatus) endpoint to allow the user to accept the invitation to the team. 

Please note that to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md) Appwrite will accept the only redirect URLs under the domains you have added as a platform on the Appwrite Console.

- `get_membership()` - Get a team member by the membership unique id. All team members have read access for this resource. Hide sensitive attributes from the response by toggling membership privacy in the Console.
- `update_membership()` - Modify the roles of a team member. Only team members with the owner role have access to this endpoint. Learn more about [roles and permissions](https://appwrite.io/docs/permissions).

- `delete_membership()` - This endpoint allows a user to leave a team or for a team owner to delete the membership of any other team member. You can also use this endpoint to delete a user membership even if it is not accepted.
- `update_membership_status()` - Use this endpoint to allow a user to accept an invitation to join a team after being redirected back to your app from the invitation email received by the user.

If the request is successful, a session for the user is automatically created.

- `get_prefs()` - Get the team&#039;s shared preferences by its unique ID. If a preference doesn&#039;t need to be shared by all team members, prefer storing them in [user preferences](https://appwrite.io/docs/references/cloud/client-web/account#getPrefs).
- `update_prefs()` - Update the team&#039;s preferences by its unique ID. The object you pass is stored as is and replaces any previous value. The maximum allowed prefs size is 64kB and throws an error if exceeded.

#### Tokens

- `list()` - List all the tokens created for a specific file or bucket. You can use the query params to filter your results.
- `create_file_token()` - Create a new token. A token is linked to a file. Token can be passed as a request URL search parameter.
- `get()` - Get a token by its unique ID.
- `update()` - Update a token by its unique ID. Use this endpoint to update a token&#039;s expiry date.
- `delete()` - Delete a token by its unique ID.

#### Users
The Users service allows you to manage your project users.
- `list()` - Get a list of all the project&#039;s users. You can use the query params to filter your results.
- `create()` - Create a new user.
- `create_argon2_user()` - Create a new user. Password provided must be hashed with the [Argon2](https://en.wikipedia.org/wiki/Argon2) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `create_bcrypt_user()` - Create a new user. Password provided must be hashed with the [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `list_identities()` - Get identities for all users.
- `delete_identity()` - Delete an identity by its unique ID.
- `create_md5_user()` - Create a new user. Password provided must be hashed with the [MD5](https://en.wikipedia.org/wiki/MD5) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `create_ph_pass_user()` - Create a new user. Password provided must be hashed with the [PHPass](https://www.openwall.com/phpass/) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `create_scrypt_user()` - Create a new user. Password provided must be hashed with the [Scrypt](https://github.com/Tarsnap/scrypt) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `create_scrypt_modified_user()` - Create a new user. Password provided must be hashed with the [Scrypt Modified](https://gist.github.com/Meldiron/eecf84a0225eccb5a378d45bb27462cc) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `create_sha_user()` - Create a new user. Password provided must be hashed with the [SHA](https://en.wikipedia.org/wiki/Secure_Hash_Algorithm) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.
- `get()` - Get a user by its unique ID.
- `delete()` - Delete a user by its unique ID, thereby releasing it&#039;s ID. Since ID is released and can be reused, all user-related resources like documents or storage files should be deleted before user deletion. If you want to keep ID reserved, use the [updateStatus](https://appwrite.io/docs/server/users#usersUpdateStatus) endpoint instead.
- `update_email()` - Update the user email by its unique ID.
- `update_impersonator()` - Enable or disable whether a user can impersonate other users. When impersonation headers are used, the request runs as the target user for API behavior, while internal audit logs still attribute the action to the original impersonator and store the impersonated target details only in internal audit payload data.

- `create_jwt()` - Use this endpoint to create a JSON Web Token for user by its unique ID. You can use the resulting JWT to authenticate on behalf of the user. The JWT secret will become invalid if the session it uses gets deleted.
- `update_labels()` - Update the user labels by its unique ID. 

Labels can be used to grant access to resources. While teams are a way for user&#039;s to share access to a resource, labels can be defined by the developer to grant access without an invitation. See the [Permissions docs](https://appwrite.io/docs/permissions) for more info.
- `list_logs()` - Get the user activity logs list by its unique ID.
- `list_memberships()` - Get the user membership list by its unique ID.
- `update_mfa()` - Enable or disable MFA on a user account.
- `update_mfa()` - Enable or disable MFA on a user account.
- `delete_mfa_authenticator()` - Delete an authenticator app.
- `delete_mfa_authenticator()` - Delete an authenticator app.
- `list_mfa_factors()` - List the factors available on the account to be used as a MFA challange.
- `list_mfa_factors()` - List the factors available on the account to be used as a MFA challange.
- `get_mfa_recovery_codes()` - Get recovery codes that can be used as backup for MFA flow by User ID. Before getting codes, they must be generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method.
- `get_mfa_recovery_codes()` - Get recovery codes that can be used as backup for MFA flow by User ID. Before getting codes, they must be generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method.
- `update_mfa_recovery_codes()` - Regenerate recovery codes that can be used as backup for MFA flow by User ID. Before regenerating codes, they must be first generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method.
- `update_mfa_recovery_codes()` - Regenerate recovery codes that can be used as backup for MFA flow by User ID. Before regenerating codes, they must be first generated using [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes) method.
- `create_mfa_recovery_codes()` - Generate recovery codes used as backup for MFA flow for User ID. Recovery codes can be used as a MFA verification type in [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge) method by client SDK.
- `create_mfa_recovery_codes()` - Generate recovery codes used as backup for MFA flow for User ID. Recovery codes can be used as a MFA verification type in [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge) method by client SDK.
- `update_name()` - Update the user name by its unique ID.
- `update_password()` - Update the user password by its unique ID.
- `update_phone()` - Update the user phone by its unique ID.
- `get_prefs()` - Get the user preferences by its unique ID.
- `update_prefs()` - Update the user preferences by its unique ID. The object you pass is stored as is, and replaces any previous value. The maximum allowed prefs size is 64kB and throws error if exceeded.
- `list_sessions()` - Get the user sessions list by its unique ID.
- `create_session()` - Creates a session for a user. Returns an immediately usable session object.

If you want to generate a token for a custom authentication flow, use the [POST /users/{userId}/tokens](https://appwrite.io/docs/server/users#createToken) endpoint.
- `delete_sessions()` - Delete all user&#039;s sessions by using the user&#039;s unique ID.
- `delete_session()` - Delete a user sessions by its unique ID.
- `update_status()` - Update the user status by its unique ID. Use this endpoint as an alternative to deleting a user if you want to keep user&#039;s ID reserved.
- `list_targets()` - List the messaging targets that are associated with a user.
- `create_target()` - Create a messaging target.
- `get_target()` - Get a user&#039;s push notification target by ID.
- `update_target()` - Update a messaging target.
- `delete_target()` - Delete a messaging target.
- `create_token()` - Returns a token with a secret key for creating a session. Use the user ID and secret and submit a request to the [PUT /account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession) endpoint to complete the login process.

- `update_email_verification()` - Update the user email verification status by its unique ID.
- `update_phone_verification()` - Update the user phone verification status by its unique ID.

#### Webhooks

- `list()` - Get a list of all webhooks belonging to the project. You can use the query params to filter your results.
- `create()` - Create a new webhook. Use this endpoint to configure a URL that will receive events from Appwrite when specific events occur.
- `get()` - Get a webhook by its unique ID. This endpoint returns details about a specific webhook configured for a project. 
- `update()` - Update a webhook by its unique ID. Use this endpoint to update the URL, events, or status of an existing webhook.
- `delete()` - Delete a webhook by its unique ID. Once deleted, the webhook will no longer receive project events. 
- `update_signature()` - Update the webhook signature key. This endpoint can be used to regenerate the signature key used to sign and validate payload deliveries for a specific webhook.


### Models
- `RowList` - Rows List
- `DocumentList` - Documents List
- `TableList` - Tables List
- `CollectionList` - Collections List
- `DatabaseList` - Databases List
- `IndexList` - Indexes List
- `ColumnIndexList` - Column Indexes List
- `UserList` - Users List
- `SessionList` - Sessions List
- `IdentityList` - Identities List
- `LogList` - Logs List
- `FileList` - Files List
- `BucketList` - Buckets List
- `ResourceTokenList` - Resource Tokens List
- `TeamList` - Teams List
- `MembershipList` - Memberships List
- `SiteList` - Sites List
- `FunctionList` - Functions List
- `FrameworkList` - Frameworks List
- `RuntimeList` - Runtimes List
- `DeploymentList` - Deployments List
- `ExecutionList` - Executions List
- `WebhookList` - Webhooks List
- `CountryList` - Countries List
- `ContinentList` - Continents List
- `LanguageList` - Languages List
- `CurrencyList` - Currencies List
- `PhoneList` - Phones List
- `VariableList` - Variables List
- `HealthStatusList` - Status List
- `LocaleCodeList` - Locale codes list
- `ProviderList` - Provider list
- `MessageList` - Message list
- `TopicList` - Topic list
- `SubscriberList` - Subscriber list
- `TargetList` - Target list
- `TransactionList` - Transaction List
- `SpecificationList` - Specifications List
- `Database` - Database
- `Collection` - Collection
- `AttributeList` - Attributes List
- `AttributeString` - AttributeString
- `AttributeInteger` - AttributeInteger
- `AttributeFloat` - AttributeFloat
- `AttributeBoolean` - AttributeBoolean
- `AttributeEmail` - AttributeEmail
- `AttributeEnum` - AttributeEnum
- `AttributeIp` - AttributeIP
- `AttributeUrl` - AttributeURL
- `AttributeDatetime` - AttributeDatetime
- `AttributeRelationship` - AttributeRelationship
- `AttributePoint` - AttributePoint
- `AttributeLine` - AttributeLine
- `AttributePolygon` - AttributePolygon
- `AttributeVarchar` - AttributeVarchar
- `AttributeText` - AttributeText
- `AttributeMediumtext` - AttributeMediumtext
- `AttributeLongtext` - AttributeLongtext
- `Table` - Table
- `ColumnList` - Columns List
- `ColumnString` - ColumnString
- `ColumnInteger` - ColumnInteger
- `ColumnFloat` - ColumnFloat
- `ColumnBoolean` - ColumnBoolean
- `ColumnEmail` - ColumnEmail
- `ColumnEnum` - ColumnEnum
- `ColumnIp` - ColumnIP
- `ColumnUrl` - ColumnURL
- `ColumnDatetime` - ColumnDatetime
- `ColumnRelationship` - ColumnRelationship
- `ColumnPoint` - ColumnPoint
- `ColumnLine` - ColumnLine
- `ColumnPolygon` - ColumnPolygon
- `ColumnVarchar` - ColumnVarchar
- `ColumnText` - ColumnText
- `ColumnMediumtext` - ColumnMediumtext
- `ColumnLongtext` - ColumnLongtext
- `Index` - Index
- `ColumnIndex` - Index
- `Row` - Row
- `Document` - Document
- `Log` - Log
- `User` - User
- `AlgoMd5` - AlgoMD5
- `AlgoSha` - AlgoSHA
- `AlgoPhpass` - AlgoPHPass
- `AlgoBcrypt` - AlgoBcrypt
- `AlgoScrypt` - AlgoScrypt
- `AlgoScryptModified` - AlgoScryptModified
- `AlgoArgon2` - AlgoArgon2
- `Preferences` - Preferences
- `Session` - Session
- `Identity` - Identity
- `Token` - Token
- `Jwt` - JWT
- `Locale` - Locale
- `LocaleCode` - LocaleCode
- `File` - File
- `Bucket` - Bucket
- `ResourceToken` - ResourceToken
- `Team` - Team
- `Membership` - Membership
- `Site` - Site
- `Function` - Function
- `Runtime` - Runtime
- `Framework` - Framework
- `FrameworkAdapter` - Framework Adapter
- `Deployment` - Deployment
- `Execution` - Execution
- `Webhook` - Webhook
- `Variable` - Variable
- `Country` - Country
- `Continent` - Continent
- `Language` - Language
- `Currency` - Currency
- `Phone` - Phone
- `HealthAntivirus` - Health Antivirus
- `HealthQueue` - Health Queue
- `HealthStatus` - Health Status
- `HealthCertificate` - Health Certificate
- `HealthTime` - Health Time
- `Headers` - Headers
- `Specification` - Specification
- `MfaChallenge` - MFA Challenge
- `MfaRecoveryCodes` - MFA Recovery Codes
- `MfaType` - MFAType
- `MfaFactors` - MFAFactors
- `Provider` - Provider
- `Message` - Message
- `Topic` - Topic
- `Transaction` - Transaction
- `Subscriber` - Subscriber
- `Target` - Target
- `ActivityEvent` - ActivityEvent
- `BackupArchive` - Archive
- `BackupPolicy` - backup
- `BackupRestoration` - Restoration
- `ActivityEventList` - Activity event list
- `BackupArchiveList` - Backup archive list
- `BackupPolicyList` - Backup policy list
- `BackupRestorationList` - Backup restoration list

### Dependencies
- reqwest 0.12+ for HTTP client
- serde 1.0+ for JSON serialization
- tokio 1.0+ for async runtime
- fastrand 2.0+ for ID generation
- thiserror 1.0+ for error handling

### Documentation
- Complete API documentation
- Usage examples for all methods
- Error handling guide
- File upload examples
- Query builder documentation

[0.2.0]: https://github.com/appwrite/sdk-for-rust/releases/tag/0.2.0
