//! BillingPlan model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// billingPlan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct BillingPlan {
    /// Plan ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Plan name
    #[serde(rename = "name")]
    pub name: String,
    /// Plan description
    #[serde(rename = "desc")]
    pub desc: String,
    /// Plan order
    #[serde(rename = "order")]
    pub order: i64,
    /// Price
    #[serde(rename = "price")]
    pub price: f64,
    /// Trial days
    #[serde(rename = "trial")]
    pub trial: i64,
    /// Bandwidth
    #[serde(rename = "bandwidth")]
    pub bandwidth: i64,
    /// Storage
    #[serde(rename = "storage")]
    pub storage: i64,
    /// Image Transformations
    #[serde(rename = "imageTransformations")]
    pub image_transformations: i64,
    /// Screenshots generated
    #[serde(rename = "screenshotsGenerated")]
    pub screenshots_generated: i64,
    /// Members
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<i64>,
    /// Webhooks
    #[serde(rename = "webhooks")]
    pub webhooks: i64,
    /// Maximum WAF rules per project
    #[serde(rename = "wafRules")]
    pub waf_rules: i64,
    /// Projects
    #[serde(rename = "projects")]
    pub projects: i64,
    /// Platforms
    #[serde(rename = "platforms")]
    pub platforms: i64,
    /// Users
    #[serde(rename = "users")]
    pub users: i64,
    /// Teams
    #[serde(rename = "teams")]
    pub teams: i64,
    /// Databases
    #[serde(rename = "databases")]
    pub databases: i64,
    /// Database reads per month
    #[serde(rename = "databasesReads")]
    pub databases_reads: i64,
    /// Database writes per month
    #[serde(rename = "databasesWrites")]
    pub databases_writes: i64,
    /// Database batch size limit
    #[serde(rename = "databasesBatchSize")]
    pub databases_batch_size: i64,
    /// Buckets
    #[serde(rename = "buckets")]
    pub buckets: i64,
    /// File size
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    /// Functions
    #[serde(rename = "functions")]
    pub functions: i64,
    /// Sites
    #[serde(rename = "sites")]
    pub sites: i64,
    /// Function executions
    #[serde(rename = "executions")]
    pub executions: i64,
    /// Rolling max executions retained per function/site
    #[serde(rename = "executionsRetentionCount")]
    pub executions_retention_count: i64,
    /// GB hours for functions
    #[serde(rename = "GBHours")]
    pub gb_hours: i64,
    /// Realtime connections
    #[serde(rename = "realtime")]
    pub realtime: i64,
    /// Realtime messages
    #[serde(rename = "realtimeMessages")]
    pub realtime_messages: i64,
    /// Messages per month
    #[serde(rename = "messages")]
    pub messages: i64,
    /// Topics for messaging
    #[serde(rename = "topics")]
    pub topics: i64,
    /// SMS authentications per month
    #[serde(rename = "authPhone")]
    pub auth_phone: i64,
    /// Custom domains
    #[serde(rename = "domains")]
    pub domains: i64,
    /// Activity log days
    #[serde(rename = "activityLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_logs: Option<i64>,
    /// Usage history days
    #[serde(rename = "usageLogs")]
    pub usage_logs: i64,
    /// Usage log time intervals allowed for this plan (e.g. 15m, 1h, 1d).
    #[serde(rename = "usageLogsIntervals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_logs_intervals: Option<Vec<String>>,
    /// Number of days of console inactivity before a project is paused. 0 means
    /// pausing is disabled.
    #[serde(rename = "projectInactivityDays")]
    pub project_inactivity_days: i64,
    /// Alert threshold percentage
    #[serde(rename = "alertLimit")]
    pub alert_limit: i64,
    /// Additional resources
    #[serde(rename = "usage")]
    pub usage: crate::models::UsageBillingPlan,
    /// Addons
    #[serde(rename = "addons")]
    pub addons: crate::models::BillingPlanAddon,
    /// Budget cap enabled or disabled.
    #[serde(rename = "budgetCapEnabled")]
    pub budget_cap_enabled: bool,
    /// Custom SMTP
    #[serde(rename = "customSmtp")]
    pub custom_smtp: bool,
    /// Appwrite branding in email
    #[serde(rename = "emailBranding")]
    pub email_branding: bool,
    /// Does plan require payment method
    #[serde(rename = "requiresPaymentMethod")]
    pub requires_payment_method: bool,
    /// Does plan require billing address
    #[serde(rename = "requiresBillingAddress")]
    pub requires_billing_address: bool,
    /// Is the billing plan available
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    /// Can user change the plan themselves
    #[serde(rename = "selfService")]
    pub self_service: bool,
    /// Does plan enable premium support
    #[serde(rename = "premiumSupport")]
    pub premium_support: bool,
    /// Does plan support budget cap
    #[serde(rename = "budgeting")]
    pub budgeting: bool,
    /// Does plan support mock numbers
    #[serde(rename = "supportsMockNumbers")]
    pub supports_mock_numbers: bool,
    /// Does plan support organization roles
    #[serde(rename = "supportsOrganizationRoles")]
    pub supports_organization_roles: bool,
    /// Does plan support credit
    #[serde(rename = "supportsCredits")]
    pub supports_credits: bool,
    /// Does plan support blocking disposable email addresses.
    #[serde(rename = "supportsDisposableEmailValidation")]
    pub supports_disposable_email_validation: bool,
    /// Does plan support requiring canonical email addresses.
    #[serde(rename = "supportsCanonicalEmailValidation")]
    pub supports_canonical_email_validation: bool,
    /// Does plan support blocking free email addresses.
    #[serde(rename = "supportsFreeEmailValidation")]
    pub supports_free_email_validation: bool,
    /// Does plan support restricting sign-ups to corporate email addresses only.
    #[serde(rename = "supportsCorporateEmailValidation")]
    pub supports_corporate_email_validation: bool,
    /// Does plan support project-specific member roles.
    #[serde(rename = "supportsProjectSpecificRoles")]
    pub supports_project_specific_roles: bool,
    /// Does plan support backup policies.
    #[serde(rename = "backupsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups_enabled: Option<bool>,
    /// Whether usage addons are calculated per project.
    #[serde(rename = "usagePerProject")]
    pub usage_per_project: bool,
    /// Supported addons for this plan
    #[serde(rename = "supportedAddons")]
    pub supported_addons: crate::models::BillingPlanSupportedAddons,
    /// How many policies does plan support
    #[serde(rename = "backupPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_policies: Option<i64>,
    /// Maximum function and site deployment size in MB
    #[serde(rename = "deploymentSize")]
    pub deployment_size: i64,
    /// Maximum function and site deployment size in MB
    #[serde(rename = "buildSize")]
    pub build_size: i64,
    /// Does the plan support encrypted string attributes or not.
    #[serde(rename = "databasesAllowEncrypt")]
    pub databases_allow_encrypt: bool,
    /// Plan specific limits
    #[serde(rename = "limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<crate::models::BillingPlanLimits>,
    /// Group of this billing plan for variants
    #[serde(rename = "group")]
    pub group: crate::enums::BillingPlanGroup,
    /// Details of the program this plan is a part of.
    #[serde(rename = "program")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<crate::models::Program>,
    /// Dedicated database limits available to this plan.
    #[serde(rename = "dedicatedDatabases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_databases: Option<crate::models::BillingPlanDedicatedDatabaseLimits>,
}

impl BillingPlan {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get desc
    pub fn desc(&self) -> &String {
        &self.desc
    }

    /// Get order
    pub fn order(&self) -> &i64 {
        &self.order
    }

    /// Get price
    pub fn price(&self) -> &f64 {
        &self.price
    }

    /// Get trial
    pub fn trial(&self) -> &i64 {
        &self.trial
    }

    /// Get bandwidth
    pub fn bandwidth(&self) -> &i64 {
        &self.bandwidth
    }

    /// Get storage
    pub fn storage(&self) -> &i64 {
        &self.storage
    }

    /// Get image_transformations
    pub fn image_transformations(&self) -> &i64 {
        &self.image_transformations
    }

    /// Get screenshots_generated
    pub fn screenshots_generated(&self) -> &i64 {
        &self.screenshots_generated
    }

    /// Set members
    pub fn set_members(mut self, members: i64) -> Self {
        self.members = Some(members);
        self
    }

    /// Get members
    pub fn members(&self) -> Option<&i64> {
        self.members.as_ref()
    }

    /// Get webhooks
    pub fn webhooks(&self) -> &i64 {
        &self.webhooks
    }

    /// Get waf_rules
    pub fn waf_rules(&self) -> &i64 {
        &self.waf_rules
    }

    /// Get projects
    pub fn projects(&self) -> &i64 {
        &self.projects
    }

    /// Get platforms
    pub fn platforms(&self) -> &i64 {
        &self.platforms
    }

    /// Get users
    pub fn users(&self) -> &i64 {
        &self.users
    }

    /// Get teams
    pub fn teams(&self) -> &i64 {
        &self.teams
    }

    /// Get databases
    pub fn databases(&self) -> &i64 {
        &self.databases
    }

    /// Get databases_reads
    pub fn databases_reads(&self) -> &i64 {
        &self.databases_reads
    }

    /// Get databases_writes
    pub fn databases_writes(&self) -> &i64 {
        &self.databases_writes
    }

    /// Get databases_batch_size
    pub fn databases_batch_size(&self) -> &i64 {
        &self.databases_batch_size
    }

    /// Get buckets
    pub fn buckets(&self) -> &i64 {
        &self.buckets
    }

    /// Get file_size
    pub fn file_size(&self) -> &i64 {
        &self.file_size
    }

    /// Get functions
    pub fn functions(&self) -> &i64 {
        &self.functions
    }

    /// Get sites
    pub fn sites(&self) -> &i64 {
        &self.sites
    }

    /// Get executions
    pub fn executions(&self) -> &i64 {
        &self.executions
    }

    /// Get executions_retention_count
    pub fn executions_retention_count(&self) -> &i64 {
        &self.executions_retention_count
    }

    /// Get gb_hours
    pub fn gb_hours(&self) -> &i64 {
        &self.gb_hours
    }

    /// Get realtime
    pub fn realtime(&self) -> &i64 {
        &self.realtime
    }

    /// Get realtime_messages
    pub fn realtime_messages(&self) -> &i64 {
        &self.realtime_messages
    }

    /// Get messages
    pub fn messages(&self) -> &i64 {
        &self.messages
    }

    /// Get topics
    pub fn topics(&self) -> &i64 {
        &self.topics
    }

    /// Get auth_phone
    pub fn auth_phone(&self) -> &i64 {
        &self.auth_phone
    }

    /// Get domains
    pub fn domains(&self) -> &i64 {
        &self.domains
    }

    /// Set activity_logs
    pub fn set_activity_logs(mut self, activity_logs: i64) -> Self {
        self.activity_logs = Some(activity_logs);
        self
    }

    /// Get activity_logs
    pub fn activity_logs(&self) -> Option<&i64> {
        self.activity_logs.as_ref()
    }

    /// Get usage_logs
    pub fn usage_logs(&self) -> &i64 {
        &self.usage_logs
    }

    /// Set usage_logs_intervals
    pub fn set_usage_logs_intervals(mut self, usage_logs_intervals: Vec<String>) -> Self {
        self.usage_logs_intervals = Some(usage_logs_intervals);
        self
    }

    /// Get usage_logs_intervals
    pub fn usage_logs_intervals(&self) -> Option<&Vec<String>> {
        self.usage_logs_intervals.as_ref()
    }

    /// Get project_inactivity_days
    pub fn project_inactivity_days(&self) -> &i64 {
        &self.project_inactivity_days
    }

    /// Get alert_limit
    pub fn alert_limit(&self) -> &i64 {
        &self.alert_limit
    }

    /// Get usage
    pub fn usage(&self) -> &crate::models::UsageBillingPlan {
        &self.usage
    }

    /// Get addons
    pub fn addons(&self) -> &crate::models::BillingPlanAddon {
        &self.addons
    }

    /// Get budget_cap_enabled
    pub fn budget_cap_enabled(&self) -> &bool {
        &self.budget_cap_enabled
    }

    /// Get custom_smtp
    pub fn custom_smtp(&self) -> &bool {
        &self.custom_smtp
    }

    /// Get email_branding
    pub fn email_branding(&self) -> &bool {
        &self.email_branding
    }

    /// Get requires_payment_method
    pub fn requires_payment_method(&self) -> &bool {
        &self.requires_payment_method
    }

    /// Get requires_billing_address
    pub fn requires_billing_address(&self) -> &bool {
        &self.requires_billing_address
    }

    /// Get is_available
    pub fn is_available(&self) -> &bool {
        &self.is_available
    }

    /// Get self_service
    pub fn self_service(&self) -> &bool {
        &self.self_service
    }

    /// Get premium_support
    pub fn premium_support(&self) -> &bool {
        &self.premium_support
    }

    /// Get budgeting
    pub fn budgeting(&self) -> &bool {
        &self.budgeting
    }

    /// Get supports_mock_numbers
    pub fn supports_mock_numbers(&self) -> &bool {
        &self.supports_mock_numbers
    }

    /// Get supports_organization_roles
    pub fn supports_organization_roles(&self) -> &bool {
        &self.supports_organization_roles
    }

    /// Get supports_credits
    pub fn supports_credits(&self) -> &bool {
        &self.supports_credits
    }

    /// Get supports_disposable_email_validation
    pub fn supports_disposable_email_validation(&self) -> &bool {
        &self.supports_disposable_email_validation
    }

    /// Get supports_canonical_email_validation
    pub fn supports_canonical_email_validation(&self) -> &bool {
        &self.supports_canonical_email_validation
    }

    /// Get supports_free_email_validation
    pub fn supports_free_email_validation(&self) -> &bool {
        &self.supports_free_email_validation
    }

    /// Get supports_corporate_email_validation
    pub fn supports_corporate_email_validation(&self) -> &bool {
        &self.supports_corporate_email_validation
    }

    /// Get supports_project_specific_roles
    pub fn supports_project_specific_roles(&self) -> &bool {
        &self.supports_project_specific_roles
    }

    /// Set backups_enabled
    pub fn set_backups_enabled(mut self, backups_enabled: bool) -> Self {
        self.backups_enabled = Some(backups_enabled);
        self
    }

    /// Get backups_enabled
    pub fn backups_enabled(&self) -> Option<&bool> {
        self.backups_enabled.as_ref()
    }

    /// Get usage_per_project
    pub fn usage_per_project(&self) -> &bool {
        &self.usage_per_project
    }

    /// Get supported_addons
    pub fn supported_addons(&self) -> &crate::models::BillingPlanSupportedAddons {
        &self.supported_addons
    }

    /// Set backup_policies
    pub fn set_backup_policies(mut self, backup_policies: i64) -> Self {
        self.backup_policies = Some(backup_policies);
        self
    }

    /// Get backup_policies
    pub fn backup_policies(&self) -> Option<&i64> {
        self.backup_policies.as_ref()
    }

    /// Get deployment_size
    pub fn deployment_size(&self) -> &i64 {
        &self.deployment_size
    }

    /// Get build_size
    pub fn build_size(&self) -> &i64 {
        &self.build_size
    }

    /// Get databases_allow_encrypt
    pub fn databases_allow_encrypt(&self) -> &bool {
        &self.databases_allow_encrypt
    }

    /// Set limits
    pub fn set_limits(mut self, limits: crate::models::BillingPlanLimits) -> Self {
        self.limits = Some(limits);
        self
    }

    /// Get limits
    pub fn limits(&self) -> Option<&crate::models::BillingPlanLimits> {
        self.limits.as_ref()
    }

    /// Get group
    pub fn group(&self) -> &crate::enums::BillingPlanGroup {
        &self.group
    }

    /// Set program
    pub fn set_program(mut self, program: crate::models::Program) -> Self {
        self.program = Some(program);
        self
    }

    /// Get program
    pub fn program(&self) -> Option<&crate::models::Program> {
        self.program.as_ref()
    }

    /// Set dedicated_databases
    pub fn set_dedicated_databases(mut self, dedicated_databases: crate::models::BillingPlanDedicatedDatabaseLimits) -> Self {
        self.dedicated_databases = Some(dedicated_databases);
        self
    }

    /// Get dedicated_databases
    pub fn dedicated_databases(&self) -> Option<&crate::models::BillingPlanDedicatedDatabaseLimits> {
        self.dedicated_databases.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_plan_creation() {
        let _model = <BillingPlan as Default>::default();
        let _ = _model.id();
        let _ = _model.name();
        let _ = _model.desc();
        let _ = _model.order();
        let _ = _model.price();
        let _ = _model.trial();
        let _ = _model.bandwidth();
        let _ = _model.storage();
        let _ = _model.image_transformations();
        let _ = _model.screenshots_generated();
        let _ = _model.webhooks();
        let _ = _model.waf_rules();
        let _ = _model.projects();
        let _ = _model.platforms();
        let _ = _model.users();
        let _ = _model.teams();
        let _ = _model.databases();
        let _ = _model.databases_reads();
        let _ = _model.databases_writes();
        let _ = _model.databases_batch_size();
        let _ = _model.buckets();
        let _ = _model.file_size();
        let _ = _model.functions();
        let _ = _model.sites();
        let _ = _model.executions();
        let _ = _model.executions_retention_count();
        let _ = _model.gb_hours();
        let _ = _model.realtime();
        let _ = _model.realtime_messages();
        let _ = _model.messages();
        let _ = _model.topics();
        let _ = _model.auth_phone();
        let _ = _model.domains();
        let _ = _model.usage_logs();
        let _ = _model.project_inactivity_days();
        let _ = _model.alert_limit();
        let _ = _model.usage();
        let _ = _model.addons();
        let _ = _model.budget_cap_enabled();
        let _ = _model.custom_smtp();
        let _ = _model.email_branding();
        let _ = _model.requires_payment_method();
        let _ = _model.requires_billing_address();
        let _ = _model.is_available();
        let _ = _model.self_service();
        let _ = _model.premium_support();
        let _ = _model.budgeting();
        let _ = _model.supports_mock_numbers();
        let _ = _model.supports_organization_roles();
        let _ = _model.supports_credits();
        let _ = _model.supports_disposable_email_validation();
        let _ = _model.supports_canonical_email_validation();
        let _ = _model.supports_free_email_validation();
        let _ = _model.supports_corporate_email_validation();
        let _ = _model.supports_project_specific_roles();
        let _ = _model.usage_per_project();
        let _ = _model.supported_addons();
        let _ = _model.deployment_size();
        let _ = _model.build_size();
        let _ = _model.databases_allow_encrypt();
        let _ = _model.group();
    }

    #[test]
    fn test_billing_plan_serialization() {
        let model = <BillingPlan as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<BillingPlan, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
