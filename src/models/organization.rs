//! Organization model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Organization {
    /// Team ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Team creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Team update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Team name.
    #[serde(rename = "name")]
    pub name: String,
    /// Total number of team members.
    #[serde(rename = "total")]
    pub total: i64,
    /// Team preferences as a key-value object
    #[serde(rename = "prefs")]
    pub prefs: crate::models::Preferences,
    /// Project budget limit. Null when no budget is set.
    #[serde(rename = "billingBudget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_budget: Option<i64>,
    /// Project budget limit
    #[serde(rename = "budgetAlerts")]
    pub budget_alerts: Vec<i64>,
    /// Organization's billing plan ID.
    #[serde(rename = "billingPlan")]
    pub billing_plan: String,
    /// Organization's billing plan ID.
    #[serde(rename = "billingPlanId")]
    pub billing_plan_id: String,
    /// Organization's billing plan.
    #[serde(rename = "billingPlanDetails")]
    pub billing_plan_details: crate::models::BillingPlan,
    /// Billing email set for the organization.
    #[serde(rename = "billingEmail")]
    pub billing_email: String,
    /// Billing cycle start date.
    #[serde(rename = "billingStartDate")]
    pub billing_start_date: String,
    /// Current invoice cycle start date.
    #[serde(rename = "billingCurrentInvoiceDate")]
    pub billing_current_invoice_date: String,
    /// Next invoice cycle start date.
    #[serde(rename = "billingNextInvoiceDate")]
    pub billing_next_invoice_date: String,
    /// Start date of trial.
    #[serde(rename = "billingTrialStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_trial_start_date: Option<String>,
    /// Number of trial days.
    #[serde(rename = "billingTrialDays")]
    pub billing_trial_days: i64,
    /// Current active aggregation id.
    #[serde(rename = "billingAggregationId")]
    pub billing_aggregation_id: String,
    /// Current active aggregation id.
    #[serde(rename = "billingInvoiceId")]
    pub billing_invoice_id: String,
    /// Default payment method.
    #[serde(rename = "paymentMethodId")]
    pub payment_method_id: String,
    /// Default payment method.
    #[serde(rename = "billingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_id: Option<String>,
    /// Backup payment method.
    #[serde(rename = "backupPaymentMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_payment_method_id: Option<String>,
    /// Team status.
    #[serde(rename = "status")]
    pub status: String,
    /// Remarks on team status.
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// Organization agreements
    #[serde(rename = "agreementBAA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_baa: Option<String>,
    /// Program manager's name.
    #[serde(rename = "programManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_manager_name: Option<String>,
    /// Program manager's calendar link.
    #[serde(rename = "programManagerCalendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_manager_calendar: Option<String>,
    /// Program's discord channel name.
    #[serde(rename = "programDiscordChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_discord_channel_name: Option<String>,
    /// Program's discord channel URL.
    #[serde(rename = "programDiscordChannelUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_discord_channel_url: Option<String>,
    /// Billing limits reached
    #[serde(rename = "billingLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_limits: Option<crate::models::BillingLimits>,
    /// Billing plan selected for downgrade.
    #[serde(rename = "billingPlanDowngrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_plan_downgrade: Option<String>,
    /// Tax Id
    #[serde(rename = "billingTaxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tax_id: Option<String>,
    /// Marked for deletion
    #[serde(rename = "markedForDeletion")]
    pub marked_for_deletion: bool,
    /// Product with which the organization is associated (appwrite or imagine)
    #[serde(rename = "platform")]
    pub platform: String,
    /// Selected projects
    #[serde(rename = "projects")]
    pub projects: Vec<String>,
}

impl Organization {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get prefs
    pub fn prefs(&self) -> &crate::models::Preferences {
        &self.prefs
    }

    /// Set billing_budget
    pub fn set_billing_budget(mut self, billing_budget: i64) -> Self {
        self.billing_budget = Some(billing_budget);
        self
    }

    /// Get billing_budget
    pub fn billing_budget(&self) -> Option<&i64> {
        self.billing_budget.as_ref()
    }

    /// Get budget_alerts
    pub fn budget_alerts(&self) -> &Vec<i64> {
        &self.budget_alerts
    }

    /// Get billing_plan
    pub fn billing_plan(&self) -> &String {
        &self.billing_plan
    }

    /// Get billing_plan_id
    pub fn billing_plan_id(&self) -> &String {
        &self.billing_plan_id
    }

    /// Get billing_plan_details
    pub fn billing_plan_details(&self) -> &crate::models::BillingPlan {
        &self.billing_plan_details
    }

    /// Get billing_email
    pub fn billing_email(&self) -> &String {
        &self.billing_email
    }

    /// Get billing_start_date
    pub fn billing_start_date(&self) -> &String {
        &self.billing_start_date
    }

    /// Get billing_current_invoice_date
    pub fn billing_current_invoice_date(&self) -> &String {
        &self.billing_current_invoice_date
    }

    /// Get billing_next_invoice_date
    pub fn billing_next_invoice_date(&self) -> &String {
        &self.billing_next_invoice_date
    }

    /// Set billing_trial_start_date
    pub fn set_billing_trial_start_date(mut self, billing_trial_start_date: String) -> Self {
        self.billing_trial_start_date = Some(billing_trial_start_date);
        self
    }

    /// Get billing_trial_start_date
    pub fn billing_trial_start_date(&self) -> Option<&String> {
        self.billing_trial_start_date.as_ref()
    }

    /// Get billing_trial_days
    pub fn billing_trial_days(&self) -> &i64 {
        &self.billing_trial_days
    }

    /// Get billing_aggregation_id
    pub fn billing_aggregation_id(&self) -> &String {
        &self.billing_aggregation_id
    }

    /// Get billing_invoice_id
    pub fn billing_invoice_id(&self) -> &String {
        &self.billing_invoice_id
    }

    /// Get payment_method_id
    pub fn payment_method_id(&self) -> &String {
        &self.payment_method_id
    }

    /// Set billing_address_id
    pub fn set_billing_address_id(mut self, billing_address_id: String) -> Self {
        self.billing_address_id = Some(billing_address_id);
        self
    }

    /// Get billing_address_id
    pub fn billing_address_id(&self) -> Option<&String> {
        self.billing_address_id.as_ref()
    }

    /// Set backup_payment_method_id
    pub fn set_backup_payment_method_id(mut self, backup_payment_method_id: String) -> Self {
        self.backup_payment_method_id = Some(backup_payment_method_id);
        self
    }

    /// Get backup_payment_method_id
    pub fn backup_payment_method_id(&self) -> Option<&String> {
        self.backup_payment_method_id.as_ref()
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Set remarks
    pub fn set_remarks(mut self, remarks: String) -> Self {
        self.remarks = Some(remarks);
        self
    }

    /// Get remarks
    pub fn remarks(&self) -> Option<&String> {
        self.remarks.as_ref()
    }

    /// Set agreement_baa
    pub fn set_agreement_baa(mut self, agreement_baa: String) -> Self {
        self.agreement_baa = Some(agreement_baa);
        self
    }

    /// Get agreement_baa
    pub fn agreement_baa(&self) -> Option<&String> {
        self.agreement_baa.as_ref()
    }

    /// Set program_manager_name
    pub fn set_program_manager_name(mut self, program_manager_name: String) -> Self {
        self.program_manager_name = Some(program_manager_name);
        self
    }

    /// Get program_manager_name
    pub fn program_manager_name(&self) -> Option<&String> {
        self.program_manager_name.as_ref()
    }

    /// Set program_manager_calendar
    pub fn set_program_manager_calendar(mut self, program_manager_calendar: String) -> Self {
        self.program_manager_calendar = Some(program_manager_calendar);
        self
    }

    /// Get program_manager_calendar
    pub fn program_manager_calendar(&self) -> Option<&String> {
        self.program_manager_calendar.as_ref()
    }

    /// Set program_discord_channel_name
    pub fn set_program_discord_channel_name(
        mut self,
        program_discord_channel_name: String,
    ) -> Self {
        self.program_discord_channel_name = Some(program_discord_channel_name);
        self
    }

    /// Get program_discord_channel_name
    pub fn program_discord_channel_name(&self) -> Option<&String> {
        self.program_discord_channel_name.as_ref()
    }

    /// Set program_discord_channel_url
    pub fn set_program_discord_channel_url(mut self, program_discord_channel_url: String) -> Self {
        self.program_discord_channel_url = Some(program_discord_channel_url);
        self
    }

    /// Get program_discord_channel_url
    pub fn program_discord_channel_url(&self) -> Option<&String> {
        self.program_discord_channel_url.as_ref()
    }

    /// Set billing_limits
    pub fn set_billing_limits(mut self, billing_limits: crate::models::BillingLimits) -> Self {
        self.billing_limits = Some(billing_limits);
        self
    }

    /// Get billing_limits
    pub fn billing_limits(&self) -> Option<&crate::models::BillingLimits> {
        self.billing_limits.as_ref()
    }

    /// Set billing_plan_downgrade
    pub fn set_billing_plan_downgrade(mut self, billing_plan_downgrade: String) -> Self {
        self.billing_plan_downgrade = Some(billing_plan_downgrade);
        self
    }

    /// Get billing_plan_downgrade
    pub fn billing_plan_downgrade(&self) -> Option<&String> {
        self.billing_plan_downgrade.as_ref()
    }

    /// Set billing_tax_id
    pub fn set_billing_tax_id(mut self, billing_tax_id: String) -> Self {
        self.billing_tax_id = Some(billing_tax_id);
        self
    }

    /// Get billing_tax_id
    pub fn billing_tax_id(&self) -> Option<&String> {
        self.billing_tax_id.as_ref()
    }

    /// Get marked_for_deletion
    pub fn marked_for_deletion(&self) -> &bool {
        &self.marked_for_deletion
    }

    /// Get platform
    pub fn platform(&self) -> &String {
        &self.platform
    }

    /// Get projects
    pub fn projects(&self) -> &Vec<String> {
        &self.projects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_creation() {
        let _model = <Organization as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.total();
        let _ = _model.prefs();
        let _ = _model.budget_alerts();
        let _ = _model.billing_plan();
        let _ = _model.billing_plan_id();
        let _ = _model.billing_plan_details();
        let _ = _model.billing_email();
        let _ = _model.billing_start_date();
        let _ = _model.billing_current_invoice_date();
        let _ = _model.billing_next_invoice_date();
        let _ = _model.billing_trial_days();
        let _ = _model.billing_aggregation_id();
        let _ = _model.billing_invoice_id();
        let _ = _model.payment_method_id();
        let _ = _model.status();
        let _ = _model.marked_for_deletion();
        let _ = _model.platform();
        let _ = _model.projects();
    }

    #[test]
    fn test_organization_serialization() {
        let model = <Organization as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Organization, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
