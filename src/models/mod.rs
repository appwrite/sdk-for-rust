//! Data models for Appwrite SDK

pub mod row_list;
pub use row_list::RowList;
pub mod document_list;
pub use document_list::DocumentList;
pub mod presence_list;
pub use presence_list::PresenceList;
pub mod table_list;
pub use table_list::TableList;
pub mod collection_list;
pub use collection_list::CollectionList;
pub mod database_list;
pub use database_list::DatabaseList;
pub mod index_list;
pub use index_list::IndexList;
pub mod column_index_list;
pub use column_index_list::ColumnIndexList;
pub mod user_list;
pub use user_list::UserList;
pub mod session_list;
pub use session_list::SessionList;
pub mod identity_list;
pub use identity_list::IdentityList;
pub mod log_list;
pub use log_list::LogList;
pub mod file_list;
pub use file_list::FileList;
pub mod bucket_list;
pub use bucket_list::BucketList;
pub mod resource_token_list;
pub use resource_token_list::ResourceTokenList;
pub mod team_list;
pub use team_list::TeamList;
pub mod membership_list;
pub use membership_list::MembershipList;
pub mod site_list;
pub use site_list::SiteList;
pub mod function_list;
pub use function_list::FunctionList;
pub mod framework_list;
pub use framework_list::FrameworkList;
pub mod runtime_list;
pub use runtime_list::RuntimeList;
pub mod deployment_list;
pub use deployment_list::DeploymentList;
pub mod execution_list;
pub use execution_list::ExecutionList;
pub mod project_list;
pub use project_list::ProjectList;
pub mod webhook_list;
pub use webhook_list::WebhookList;
pub mod key_list;
pub use key_list::KeyList;
pub mod country_list;
pub use country_list::CountryList;
pub mod continent_list;
pub use continent_list::ContinentList;
pub mod language_list;
pub use language_list::LanguageList;
pub mod currency_list;
pub use currency_list::CurrencyList;
pub mod phone_list;
pub use phone_list::PhoneList;
pub mod variable_list;
pub use variable_list::VariableList;
pub mod mock_number_list;
pub use mock_number_list::MockNumberList;
pub mod policy_list;
pub use policy_list::PolicyList;
pub mod email_template_list;
pub use email_template_list::EmailTemplateList;
pub mod proxy_rule_list;
pub use proxy_rule_list::ProxyRuleList;
pub mod locale_code_list;
pub use locale_code_list::LocaleCodeList;
pub mod provider_list;
pub use provider_list::ProviderList;
pub mod message_list;
pub use message_list::MessageList;
pub mod topic_list;
pub use topic_list::TopicList;
pub mod subscriber_list;
pub use subscriber_list::SubscriberList;
pub mod target_list;
pub use target_list::TargetList;
pub mod transaction_list;
pub use transaction_list::TransactionList;
pub mod specification_list;
pub use specification_list::SpecificationList;
pub mod insight_list;
pub use insight_list::InsightList;
pub mod report_list;
pub use report_list::ReportList;
pub mod database;
pub use database::Database;
pub mod collection;
pub use collection::Collection;
pub mod attribute_list;
pub use attribute_list::AttributeList;
pub mod attribute_string;
pub use attribute_string::AttributeString;
pub mod attribute_integer;
pub use attribute_integer::AttributeInteger;
pub mod attribute_bigint;
pub use attribute_bigint::AttributeBigint;
pub mod attribute_float;
pub use attribute_float::AttributeFloat;
pub mod attribute_boolean;
pub use attribute_boolean::AttributeBoolean;
pub mod attribute_email;
pub use attribute_email::AttributeEmail;
pub mod attribute_enum;
pub use attribute_enum::AttributeEnum;
pub mod attribute_ip;
pub use attribute_ip::AttributeIp;
pub mod attribute_url;
pub use attribute_url::AttributeUrl;
pub mod attribute_datetime;
pub use attribute_datetime::AttributeDatetime;
pub mod attribute_relationship;
pub use attribute_relationship::AttributeRelationship;
pub mod attribute_point;
pub use attribute_point::AttributePoint;
pub mod attribute_line;
pub use attribute_line::AttributeLine;
pub mod attribute_polygon;
pub use attribute_polygon::AttributePolygon;
pub mod attribute_varchar;
pub use attribute_varchar::AttributeVarchar;
pub mod attribute_text;
pub use attribute_text::AttributeText;
pub mod attribute_mediumtext;
pub use attribute_mediumtext::AttributeMediumtext;
pub mod attribute_longtext;
pub use attribute_longtext::AttributeLongtext;
pub mod table;
pub use table::Table;
pub mod column_list;
pub use column_list::ColumnList;
pub mod column_string;
pub use column_string::ColumnString;
pub mod column_integer;
pub use column_integer::ColumnInteger;
pub mod column_bigint;
pub use column_bigint::ColumnBigint;
pub mod column_float;
pub use column_float::ColumnFloat;
pub mod column_boolean;
pub use column_boolean::ColumnBoolean;
pub mod column_email;
pub use column_email::ColumnEmail;
pub mod column_enum;
pub use column_enum::ColumnEnum;
pub mod column_ip;
pub use column_ip::ColumnIp;
pub mod column_url;
pub use column_url::ColumnUrl;
pub mod column_datetime;
pub use column_datetime::ColumnDatetime;
pub mod column_relationship;
pub use column_relationship::ColumnRelationship;
pub mod column_point;
pub use column_point::ColumnPoint;
pub mod column_line;
pub use column_line::ColumnLine;
pub mod column_polygon;
pub use column_polygon::ColumnPolygon;
pub mod column_varchar;
pub use column_varchar::ColumnVarchar;
pub mod column_text;
pub use column_text::ColumnText;
pub mod column_mediumtext;
pub use column_mediumtext::ColumnMediumtext;
pub mod column_longtext;
pub use column_longtext::ColumnLongtext;
pub mod index;
pub use index::Index;
pub mod column_index;
pub use column_index::ColumnIndex;
pub mod row;
pub use row::Row;
pub mod document;
pub use document::Document;
pub mod presence;
pub use presence::Presence;
pub mod log;
pub use log::Log;
pub mod user;
pub use user::User;
pub mod algo_md5;
pub use algo_md5::AlgoMd5;
pub mod algo_sha;
pub use algo_sha::AlgoSha;
pub mod algo_phpass;
pub use algo_phpass::AlgoPhpass;
pub mod algo_bcrypt;
pub use algo_bcrypt::AlgoBcrypt;
pub mod algo_scrypt;
pub use algo_scrypt::AlgoScrypt;
pub mod algo_scrypt_modified;
pub use algo_scrypt_modified::AlgoScryptModified;
pub mod algo_argon2;
pub use algo_argon2::AlgoArgon2;
pub mod preferences;
pub use preferences::Preferences;
pub mod session;
pub use session::Session;
pub mod identity;
pub use identity::Identity;
pub mod token;
pub use token::Token;
pub mod jwt;
pub use jwt::Jwt;
pub mod locale;
pub use locale::Locale;
pub mod locale_code;
pub use locale_code::LocaleCode;
pub mod file;
pub use file::File;
pub mod bucket;
pub use bucket::Bucket;
pub mod resource_token;
pub use resource_token::ResourceToken;
pub mod team;
pub use team::Team;
pub mod membership;
pub use membership::Membership;
pub mod site;
pub use site::Site;
pub mod function;
pub use function::Function;
pub mod runtime;
pub use runtime::Runtime;
pub mod framework;
pub use framework::Framework;
pub mod framework_adapter;
pub use framework_adapter::FrameworkAdapter;
pub mod deployment;
pub use deployment::Deployment;
pub mod execution;
pub use execution::Execution;
pub mod project;
pub use project::Project;
pub mod project_auth_method;
pub use project_auth_method::ProjectAuthMethod;
pub mod project_service;
pub use project_service::ProjectService;
pub mod project_protocol;
pub use project_protocol::ProjectProtocol;
pub mod webhook;
pub use webhook::Webhook;
pub mod key;
pub use key::Key;
pub mod ephemeral_key;
pub use ephemeral_key::EphemeralKey;
pub mod dev_key;
pub use dev_key::DevKey;
pub mod mock_number;
pub use mock_number::MockNumber;
pub mod o_auth2_github;
pub use o_auth2_github::OAuth2Github;
pub mod o_auth2_discord;
pub use o_auth2_discord::OAuth2Discord;
pub mod o_auth2_figma;
pub use o_auth2_figma::OAuth2Figma;
pub mod o_auth2_dropbox;
pub use o_auth2_dropbox::OAuth2Dropbox;
pub mod o_auth2_dailymotion;
pub use o_auth2_dailymotion::OAuth2Dailymotion;
pub mod o_auth2_bitbucket;
pub use o_auth2_bitbucket::OAuth2Bitbucket;
pub mod o_auth2_bitly;
pub use o_auth2_bitly::OAuth2Bitly;
pub mod o_auth2_box;
pub use o_auth2_box::OAuth2Box;
pub mod o_auth2_autodesk;
pub use o_auth2_autodesk::OAuth2Autodesk;
pub mod o_auth2_google;
pub use o_auth2_google::OAuth2Google;
pub mod o_auth2_zoom;
pub use o_auth2_zoom::OAuth2Zoom;
pub mod o_auth2_zoho;
pub use o_auth2_zoho::OAuth2Zoho;
pub mod o_auth2_yandex;
pub use o_auth2_yandex::OAuth2Yandex;
pub mod o_auth2_x;
pub use o_auth2_x::OAuth2X;
pub mod o_auth2_word_press;
pub use o_auth2_word_press::OAuth2WordPress;
pub mod o_auth2_twitch;
pub use o_auth2_twitch::OAuth2Twitch;
pub mod o_auth2_stripe;
pub use o_auth2_stripe::OAuth2Stripe;
pub mod o_auth2_spotify;
pub use o_auth2_spotify::OAuth2Spotify;
pub mod o_auth2_slack;
pub use o_auth2_slack::OAuth2Slack;
pub mod o_auth2_podio;
pub use o_auth2_podio::OAuth2Podio;
pub mod o_auth2_notion;
pub use o_auth2_notion::OAuth2Notion;
pub mod o_auth2_salesforce;
pub use o_auth2_salesforce::OAuth2Salesforce;
pub mod o_auth2_yahoo;
pub use o_auth2_yahoo::OAuth2Yahoo;
pub mod o_auth2_linkedin;
pub use o_auth2_linkedin::OAuth2Linkedin;
pub mod o_auth2_disqus;
pub use o_auth2_disqus::OAuth2Disqus;
pub mod o_auth2_amazon;
pub use o_auth2_amazon::OAuth2Amazon;
pub mod o_auth2_etsy;
pub use o_auth2_etsy::OAuth2Etsy;
pub mod o_auth2_facebook;
pub use o_auth2_facebook::OAuth2Facebook;
pub mod o_auth2_tradeshift;
pub use o_auth2_tradeshift::OAuth2Tradeshift;
pub mod o_auth2_paypal;
pub use o_auth2_paypal::OAuth2Paypal;
pub mod o_auth2_gitlab;
pub use o_auth2_gitlab::OAuth2Gitlab;
pub mod o_auth2_appwrite;
pub use o_auth2_appwrite::OAuth2Appwrite;
pub mod o_auth2_authentik;
pub use o_auth2_authentik::OAuth2Authentik;
pub mod o_auth2_auth0;
pub use o_auth2_auth0::OAuth2Auth0;
pub mod o_auth2_fusion_auth;
pub use o_auth2_fusion_auth::OAuth2FusionAuth;
pub mod o_auth2_keycloak;
pub use o_auth2_keycloak::OAuth2Keycloak;
pub mod o_auth2_oidc;
pub use o_auth2_oidc::OAuth2Oidc;
pub mod o_auth2_okta;
pub use o_auth2_okta::OAuth2Okta;
pub mod o_auth2_kick;
pub use o_auth2_kick::OAuth2Kick;
pub mod o_auth2_apple;
pub use o_auth2_apple::OAuth2Apple;
pub mod o_auth2_microsoft;
pub use o_auth2_microsoft::OAuth2Microsoft;
pub mod o_auth2_provider_list;
pub use o_auth2_provider_list::OAuth2ProviderList;
pub mod policy_password_dictionary;
pub use policy_password_dictionary::PolicyPasswordDictionary;
pub mod policy_password_history;
pub use policy_password_history::PolicyPasswordHistory;
pub mod policy_password_strength;
pub use policy_password_strength::PolicyPasswordStrength;
pub mod policy_password_personal_data;
pub use policy_password_personal_data::PolicyPasswordPersonalData;
pub mod policy_session_alert;
pub use policy_session_alert::PolicySessionAlert;
pub mod policy_session_duration;
pub use policy_session_duration::PolicySessionDuration;
pub mod policy_session_invalidation;
pub use policy_session_invalidation::PolicySessionInvalidation;
pub mod policy_session_limit;
pub use policy_session_limit::PolicySessionLimit;
pub mod policy_user_limit;
pub use policy_user_limit::PolicyUserLimit;
pub mod policy_membership_privacy;
pub use policy_membership_privacy::PolicyMembershipPrivacy;
pub mod policy_mfa_factors;
pub use policy_mfa_factors::PolicyMfaFactors;
pub mod platform_web;
pub use platform_web::PlatformWeb;
pub mod platform_apple;
pub use platform_apple::PlatformApple;
pub mod platform_android;
pub use platform_android::PlatformAndroid;
pub mod platform_windows;
pub use platform_windows::PlatformWindows;
pub mod platform_linux;
pub use platform_linux::PlatformLinux;
pub mod platform_list;
pub use platform_list::PlatformList;
pub mod variable;
pub use variable::Variable;
pub mod country;
pub use country::Country;
pub mod continent;
pub use continent::Continent;
pub mod language;
pub use language::Language;
pub mod currency;
pub use currency::Currency;
pub mod phone;
pub use phone::Phone;
pub mod headers;
pub use headers::Headers;
pub mod specification;
pub use specification::Specification;
pub mod proxy_rule;
pub use proxy_rule::ProxyRule;
pub mod email_template;
pub use email_template::EmailTemplate;
pub mod mfa_challenge;
pub use mfa_challenge::MfaChallenge;
pub mod mfa_challenge_secret;
pub use mfa_challenge_secret::MfaChallengeSecret;
pub mod mfa_recovery_codes;
pub use mfa_recovery_codes::MfaRecoveryCodes;
pub mod mfa_type;
pub use mfa_type::MfaType;
pub mod mfa_factors;
pub use mfa_factors::MfaFactors;
pub mod provider;
pub use provider::Provider;
pub mod message;
pub use message::Message;
pub mod topic;
pub use topic::Topic;
pub mod transaction;
pub use transaction::Transaction;
pub mod subscriber;
pub use subscriber::Subscriber;
pub mod target;
pub use target::Target;
pub mod insight;
pub use insight::Insight;
pub mod insight_cta;
pub use insight_cta::InsightCTA;
pub mod report;
pub use report::Report;
pub mod activity_event;
pub use activity_event::ActivityEvent;
pub mod additional_resource;
pub use additional_resource::AdditionalResource;
pub mod backup_archive;
pub use backup_archive::BackupArchive;
pub mod billing_limits;
pub use billing_limits::BillingLimits;
pub mod billing_plan;
pub use billing_plan::BillingPlan;
pub mod billing_plan_addon;
pub use billing_plan_addon::BillingPlanAddon;
pub mod billing_plan_addon_details;
pub use billing_plan_addon_details::BillingPlanAddonDetails;
pub mod billing_plan_limits;
pub use billing_plan_limits::BillingPlanLimits;
pub mod billing_plan_dedicated_database_limits;
pub use billing_plan_dedicated_database_limits::BillingPlanDedicatedDatabaseLimits;
pub mod billing_plan_supported_addons;
pub use billing_plan_supported_addons::BillingPlanSupportedAddons;
pub mod block;
pub use block::Block;
pub mod database_migration;
pub use database_migration::DatabaseMigration;
pub mod dedicated_database;
pub use dedicated_database::DedicatedDatabase;
pub mod database_status;
pub use database_status::DatabaseStatus;
pub mod dedicated_database_member;
pub use dedicated_database_member::DedicatedDatabaseMember;
pub mod dedicated_database_operation;
pub use dedicated_database_operation::DedicatedDatabaseOperation;
pub mod dedicated_database_operation_list;
pub use dedicated_database_operation_list::DedicatedDatabaseOperationList;
pub mod dedicated_database_replicas;
pub use dedicated_database_replicas::DedicatedDatabaseReplicas;
pub mod proxy_invalidation;
pub use proxy_invalidation::ProxyInvalidation;
pub mod organization;
pub use organization::Organization;
pub mod backup_policy;
pub use backup_policy::BackupPolicy;
pub mod policy_deny_aliased_email;
pub use policy_deny_aliased_email::PolicyDenyAliasedEmail;
pub mod policy_deny_disposable_email;
pub use policy_deny_disposable_email::PolicyDenyDisposableEmail;
pub mod policy_deny_free_email;
pub use policy_deny_free_email::PolicyDenyFreeEmail;
pub mod policy_deny_corporate_email;
pub use policy_deny_corporate_email::PolicyDenyCorporateEmail;
pub mod program;
pub use program::Program;
pub mod backup_restoration;
pub use backup_restoration::BackupRestoration;
pub mod dedicated_database_specification;
pub use dedicated_database_specification::DedicatedDatabaseSpecification;
pub mod dedicated_database_specification_list;
pub use dedicated_database_specification_list::DedicatedDatabaseSpecificationList;
pub mod dedicated_database_specification_pricing;
pub use dedicated_database_specification_pricing::DedicatedDatabaseSpecificationPricing;
pub mod database_status_connections;
pub use database_status_connections::DatabaseStatusConnections;
pub mod database_status_replica;
pub use database_status_replica::DatabaseStatusReplica;
pub mod database_status_volume;
pub use database_status_volume::DatabaseStatusVolume;
pub mod usage_billing_plan;
pub use usage_billing_plan::UsageBillingPlan;
pub mod app;
pub use app::App;
pub mod app_secret;
pub use app_secret::AppSecret;
pub mod app_secret_plaintext;
pub use app_secret_plaintext::AppSecretPlaintext;
pub mod app_scope;
pub use app_scope::AppScope;
pub mod app_installation;
pub use app_installation::AppInstallation;
pub mod app_key;
pub use app_key::AppKey;
pub mod oauth2_authorize;
pub use oauth2_authorize::Oauth2Authorize;
pub mod oauth2_approve;
pub use oauth2_approve::Oauth2Approve;
pub mod oauth2_reject;
pub use oauth2_reject::Oauth2Reject;
pub mod oauth2_grant;
pub use oauth2_grant::Oauth2Grant;
pub mod oauth2_device_authorization;
pub use oauth2_device_authorization::Oauth2DeviceAuthorization;
pub mod oauth2_par;
pub use oauth2_par::Oauth2PAR;
pub mod oauth2_token;
pub use oauth2_token::Oauth2Token;
pub mod oauth2_consent;
pub use oauth2_consent::Oauth2Consent;
pub mod oauth2_consent_token;
pub use oauth2_consent_token::Oauth2ConsentToken;
pub mod oauth2_project;
pub use oauth2_project::Oauth2Project;
pub mod oauth2_organization;
pub use oauth2_organization::Oauth2Organization;
pub mod oauth2_project_list;
pub use oauth2_project_list::Oauth2ProjectList;
pub mod oauth2_organization_list;
pub use oauth2_organization_list::Oauth2OrganizationList;
pub mod oauth2_consent_list;
pub use oauth2_consent_list::Oauth2ConsentList;
pub mod oauth2_consent_token_list;
pub use oauth2_consent_token_list::Oauth2ConsentTokenList;
pub mod activity_event_list;
pub use activity_event_list::ActivityEventList;
pub mod backup_archive_list;
pub use backup_archive_list::BackupArchiveList;
pub mod backup_policy_list;
pub use backup_policy_list::BackupPolicyList;
pub mod backup_restoration_list;
pub use backup_restoration_list::BackupRestorationList;
pub mod database_migration_list;
pub use database_migration_list::DatabaseMigrationList;
pub mod apps_list;
pub use apps_list::AppsList;
pub mod app_secret_list;
pub use app_secret_list::AppSecretList;
pub mod app_scope_list;
pub use app_scope_list::AppScopeList;
pub mod app_installation_list;
pub use app_installation_list::AppInstallationList;
pub mod app_key_list;
pub use app_key_list::AppKeyList;

// Re-export commonly used types
use serde::{Deserialize, Serialize};

/// Base trait for all Appwrite models
pub trait Model: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {}

// Implement the trait for all generated models
impl Model for RowList {}
impl Model for DocumentList {}
impl Model for PresenceList {}
impl Model for TableList {}
impl Model for CollectionList {}
impl Model for DatabaseList {}
impl Model for IndexList {}
impl Model for ColumnIndexList {}
impl Model for UserList {}
impl Model for SessionList {}
impl Model for IdentityList {}
impl Model for LogList {}
impl Model for FileList {}
impl Model for BucketList {}
impl Model for ResourceTokenList {}
impl Model for TeamList {}
impl Model for MembershipList {}
impl Model for SiteList {}
impl Model for FunctionList {}
impl Model for FrameworkList {}
impl Model for RuntimeList {}
impl Model for DeploymentList {}
impl Model for ExecutionList {}
impl Model for ProjectList {}
impl Model for WebhookList {}
impl Model for KeyList {}
impl Model for CountryList {}
impl Model for ContinentList {}
impl Model for LanguageList {}
impl Model for CurrencyList {}
impl Model for PhoneList {}
impl Model for VariableList {}
impl Model for MockNumberList {}
impl Model for PolicyList {}
impl Model for EmailTemplateList {}
impl Model for ProxyRuleList {}
impl Model for LocaleCodeList {}
impl Model for ProviderList {}
impl Model for MessageList {}
impl Model for TopicList {}
impl Model for SubscriberList {}
impl Model for TargetList {}
impl Model for TransactionList {}
impl Model for SpecificationList {}
impl Model for InsightList {}
impl Model for ReportList {}
impl Model for Database {}
impl Model for Collection {}
impl Model for AttributeList {}
impl Model for AttributeString {}
impl Model for AttributeInteger {}
impl Model for AttributeBigint {}
impl Model for AttributeFloat {}
impl Model for AttributeBoolean {}
impl Model for AttributeEmail {}
impl Model for AttributeEnum {}
impl Model for AttributeIp {}
impl Model for AttributeUrl {}
impl Model for AttributeDatetime {}
impl Model for AttributeRelationship {}
impl Model for AttributePoint {}
impl Model for AttributeLine {}
impl Model for AttributePolygon {}
impl Model for AttributeVarchar {}
impl Model for AttributeText {}
impl Model for AttributeMediumtext {}
impl Model for AttributeLongtext {}
impl Model for Table {}
impl Model for ColumnList {}
impl Model for ColumnString {}
impl Model for ColumnInteger {}
impl Model for ColumnBigint {}
impl Model for ColumnFloat {}
impl Model for ColumnBoolean {}
impl Model for ColumnEmail {}
impl Model for ColumnEnum {}
impl Model for ColumnIp {}
impl Model for ColumnUrl {}
impl Model for ColumnDatetime {}
impl Model for ColumnRelationship {}
impl Model for ColumnPoint {}
impl Model for ColumnLine {}
impl Model for ColumnPolygon {}
impl Model for ColumnVarchar {}
impl Model for ColumnText {}
impl Model for ColumnMediumtext {}
impl Model for ColumnLongtext {}
impl Model for Index {}
impl Model for ColumnIndex {}
impl Model for Row {}
impl Model for Document {}
impl Model for Presence {}
impl Model for Log {}
impl Model for User {}
impl Model for AlgoMd5 {}
impl Model for AlgoSha {}
impl Model for AlgoPhpass {}
impl Model for AlgoBcrypt {}
impl Model for AlgoScrypt {}
impl Model for AlgoScryptModified {}
impl Model for AlgoArgon2 {}
impl Model for Preferences {}
impl Model for Session {}
impl Model for Identity {}
impl Model for Token {}
impl Model for Jwt {}
impl Model for Locale {}
impl Model for LocaleCode {}
impl Model for File {}
impl Model for Bucket {}
impl Model for ResourceToken {}
impl Model for Team {}
impl Model for Membership {}
impl Model for Site {}
impl Model for Function {}
impl Model for Runtime {}
impl Model for Framework {}
impl Model for FrameworkAdapter {}
impl Model for Deployment {}
impl Model for Execution {}
impl Model for Project {}
impl Model for ProjectAuthMethod {}
impl Model for ProjectService {}
impl Model for ProjectProtocol {}
impl Model for Webhook {}
impl Model for Key {}
impl Model for EphemeralKey {}
impl Model for DevKey {}
impl Model for MockNumber {}
impl Model for OAuth2Github {}
impl Model for OAuth2Discord {}
impl Model for OAuth2Figma {}
impl Model for OAuth2Dropbox {}
impl Model for OAuth2Dailymotion {}
impl Model for OAuth2Bitbucket {}
impl Model for OAuth2Bitly {}
impl Model for OAuth2Box {}
impl Model for OAuth2Autodesk {}
impl Model for OAuth2Google {}
impl Model for OAuth2Zoom {}
impl Model for OAuth2Zoho {}
impl Model for OAuth2Yandex {}
impl Model for OAuth2X {}
impl Model for OAuth2WordPress {}
impl Model for OAuth2Twitch {}
impl Model for OAuth2Stripe {}
impl Model for OAuth2Spotify {}
impl Model for OAuth2Slack {}
impl Model for OAuth2Podio {}
impl Model for OAuth2Notion {}
impl Model for OAuth2Salesforce {}
impl Model for OAuth2Yahoo {}
impl Model for OAuth2Linkedin {}
impl Model for OAuth2Disqus {}
impl Model for OAuth2Amazon {}
impl Model for OAuth2Etsy {}
impl Model for OAuth2Facebook {}
impl Model for OAuth2Tradeshift {}
impl Model for OAuth2Paypal {}
impl Model for OAuth2Gitlab {}
impl Model for OAuth2Appwrite {}
impl Model for OAuth2Authentik {}
impl Model for OAuth2Auth0 {}
impl Model for OAuth2FusionAuth {}
impl Model for OAuth2Keycloak {}
impl Model for OAuth2Oidc {}
impl Model for OAuth2Okta {}
impl Model for OAuth2Kick {}
impl Model for OAuth2Apple {}
impl Model for OAuth2Microsoft {}
impl Model for OAuth2ProviderList {}
impl Model for PolicyPasswordDictionary {}
impl Model for PolicyPasswordHistory {}
impl Model for PolicyPasswordStrength {}
impl Model for PolicyPasswordPersonalData {}
impl Model for PolicySessionAlert {}
impl Model for PolicySessionDuration {}
impl Model for PolicySessionInvalidation {}
impl Model for PolicySessionLimit {}
impl Model for PolicyUserLimit {}
impl Model for PolicyMembershipPrivacy {}
impl Model for PolicyMfaFactors {}
impl Model for PlatformWeb {}
impl Model for PlatformApple {}
impl Model for PlatformAndroid {}
impl Model for PlatformWindows {}
impl Model for PlatformLinux {}
impl Model for PlatformList {}
impl Model for Variable {}
impl Model for Country {}
impl Model for Continent {}
impl Model for Language {}
impl Model for Currency {}
impl Model for Phone {}
impl Model for Headers {}
impl Model for Specification {}
impl Model for ProxyRule {}
impl Model for EmailTemplate {}
impl Model for MfaChallenge {}
impl Model for MfaChallengeSecret {}
impl Model for MfaRecoveryCodes {}
impl Model for MfaType {}
impl Model for MfaFactors {}
impl Model for Provider {}
impl Model for Message {}
impl Model for Topic {}
impl Model for Transaction {}
impl Model for Subscriber {}
impl Model for Target {}
impl Model for Insight {}
impl Model for InsightCTA {}
impl Model for Report {}
impl Model for ActivityEvent {}
impl Model for AdditionalResource {}
impl Model for BackupArchive {}
impl Model for BillingLimits {}
impl Model for BillingPlan {}
impl Model for BillingPlanAddon {}
impl Model for BillingPlanAddonDetails {}
impl Model for BillingPlanLimits {}
impl Model for BillingPlanDedicatedDatabaseLimits {}
impl Model for BillingPlanSupportedAddons {}
impl Model for Block {}
impl Model for DatabaseMigration {}
impl Model for DedicatedDatabase {}
impl Model for DatabaseStatus {}
impl Model for DedicatedDatabaseMember {}
impl Model for DedicatedDatabaseOperation {}
impl Model for DedicatedDatabaseOperationList {}
impl Model for DedicatedDatabaseReplicas {}
impl Model for ProxyInvalidation {}
impl Model for Organization {}
impl Model for BackupPolicy {}
impl Model for PolicyDenyAliasedEmail {}
impl Model for PolicyDenyDisposableEmail {}
impl Model for PolicyDenyFreeEmail {}
impl Model for PolicyDenyCorporateEmail {}
impl Model for Program {}
impl Model for BackupRestoration {}
impl Model for DedicatedDatabaseSpecification {}
impl Model for DedicatedDatabaseSpecificationList {}
impl Model for DedicatedDatabaseSpecificationPricing {}
impl Model for DatabaseStatusConnections {}
impl Model for DatabaseStatusReplica {}
impl Model for DatabaseStatusVolume {}
impl Model for UsageBillingPlan {}
impl Model for App {}
impl Model for AppSecret {}
impl Model for AppSecretPlaintext {}
impl Model for AppScope {}
impl Model for AppInstallation {}
impl Model for AppKey {}
impl Model for Oauth2Authorize {}
impl Model for Oauth2Approve {}
impl Model for Oauth2Reject {}
impl Model for Oauth2Grant {}
impl Model for Oauth2DeviceAuthorization {}
impl Model for Oauth2PAR {}
impl Model for Oauth2Token {}
impl Model for Oauth2Consent {}
impl Model for Oauth2ConsentToken {}
impl Model for Oauth2Project {}
impl Model for Oauth2Organization {}
impl Model for Oauth2ProjectList {}
impl Model for Oauth2OrganizationList {}
impl Model for Oauth2ConsentList {}
impl Model for Oauth2ConsentTokenList {}
impl Model for ActivityEventList {}
impl Model for BackupArchiveList {}
impl Model for BackupPolicyList {}
impl Model for BackupRestorationList {}
impl Model for DatabaseMigrationList {}
impl Model for AppsList {}
impl Model for AppSecretList {}
impl Model for AppScopeList {}
impl Model for AppInstallationList {}
impl Model for AppKeyList {}
