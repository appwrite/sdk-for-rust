//! Data models for Appwrite SDK

pub mod row_list;
pub use row_list::RowList;
pub mod document_list;
pub use document_list::DocumentList;
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
pub mod health_status_list;
pub use health_status_list::HealthStatusList;
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
pub mod webhook;
pub use webhook::Webhook;
pub mod key;
pub use key::Key;
pub mod dev_key;
pub use dev_key::DevKey;
pub mod mock_number;
pub use mock_number::MockNumber;
pub mod auth_provider;
pub use auth_provider::AuthProvider;
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
pub mod health_antivirus;
pub use health_antivirus::HealthAntivirus;
pub mod health_queue;
pub use health_queue::HealthQueue;
pub mod health_status;
pub use health_status::HealthStatus;
pub mod health_certificate;
pub use health_certificate::HealthCertificate;
pub mod health_time;
pub use health_time::HealthTime;
pub mod headers;
pub use headers::Headers;
pub mod specification;
pub use specification::Specification;
pub mod mfa_challenge;
pub use mfa_challenge::MfaChallenge;
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
pub mod activity_event;
pub use activity_event::ActivityEvent;
pub mod backup_archive;
pub use backup_archive::BackupArchive;
pub mod billing_limits;
pub use billing_limits::BillingLimits;
pub mod block;
pub use block::Block;
pub mod backup_policy;
pub use backup_policy::BackupPolicy;
pub mod backup_restoration;
pub use backup_restoration::BackupRestoration;
pub mod activity_event_list;
pub use activity_event_list::ActivityEventList;
pub mod backup_archive_list;
pub use backup_archive_list::BackupArchiveList;
pub mod backup_policy_list;
pub use backup_policy_list::BackupPolicyList;
pub mod backup_restoration_list;
pub use backup_restoration_list::BackupRestorationList;

// Re-export commonly used types
use serde::{Deserialize, Serialize};

/// Base trait for all Appwrite models
pub trait Model: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {}

// Implement the trait for all generated models
impl Model for RowList {}
impl Model for DocumentList {}
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
impl Model for WebhookList {}
impl Model for KeyList {}
impl Model for CountryList {}
impl Model for ContinentList {}
impl Model for LanguageList {}
impl Model for CurrencyList {}
impl Model for PhoneList {}
impl Model for VariableList {}
impl Model for HealthStatusList {}
impl Model for LocaleCodeList {}
impl Model for ProviderList {}
impl Model for MessageList {}
impl Model for TopicList {}
impl Model for SubscriberList {}
impl Model for TargetList {}
impl Model for TransactionList {}
impl Model for SpecificationList {}
impl Model for Database {}
impl Model for Collection {}
impl Model for AttributeList {}
impl Model for AttributeString {}
impl Model for AttributeInteger {}
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
impl Model for Webhook {}
impl Model for Key {}
impl Model for DevKey {}
impl Model for MockNumber {}
impl Model for AuthProvider {}
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
impl Model for HealthAntivirus {}
impl Model for HealthQueue {}
impl Model for HealthStatus {}
impl Model for HealthCertificate {}
impl Model for HealthTime {}
impl Model for Headers {}
impl Model for Specification {}
impl Model for MfaChallenge {}
impl Model for MfaRecoveryCodes {}
impl Model for MfaType {}
impl Model for MfaFactors {}
impl Model for Provider {}
impl Model for Message {}
impl Model for Topic {}
impl Model for Transaction {}
impl Model for Subscriber {}
impl Model for Target {}
impl Model for ActivityEvent {}
impl Model for BackupArchive {}
impl Model for BillingLimits {}
impl Model for Block {}
impl Model for BackupPolicy {}
impl Model for BackupRestoration {}
impl Model for ActivityEventList {}
impl Model for BackupArchiveList {}
impl Model for BackupPolicyList {}
impl Model for BackupRestorationList {}
