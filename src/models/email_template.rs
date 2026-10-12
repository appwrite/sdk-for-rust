//! EmailTemplate model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// EmailTemplate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct EmailTemplate {
    /// Template type
    #[serde(rename = "templateId")]
    #[serde(default = "default_template_id")]
    pub template_id: String,
    /// Template locale
    #[serde(rename = "locale")]
    #[serde(default = "default_locale")]
    pub locale: String,
    /// Template message
    #[serde(rename = "message")]
    #[serde(default = "default_message")]
    pub message: String,
    /// Name of the sender
    #[serde(rename = "senderName")]
    #[serde(default = "default_sender_name")]
    pub sender_name: String,
    /// Email of the sender
    #[serde(rename = "senderEmail")]
    #[serde(default = "default_sender_email")]
    pub sender_email: String,
    /// Reply to email address
    #[serde(rename = "replyToEmail")]
    #[serde(default = "default_reply_to_email")]
    pub reply_to_email: String,
    /// Reply to name
    #[serde(rename = "replyToName")]
    #[serde(default = "default_reply_to_name")]
    pub reply_to_name: String,
    /// Email subject
    #[serde(rename = "subject")]
    #[serde(default = "default_subject")]
    pub subject: String,
}

fn default_template_id() -> String {
    "".to_string()
}

fn default_locale() -> String {
    "".to_string()
}

fn default_message() -> String {
    "".to_string()
}

fn default_sender_name() -> String {
    "".to_string()
}

fn default_sender_email() -> String {
    "".to_string()
}

fn default_reply_to_email() -> String {
    "".to_string()
}

fn default_reply_to_name() -> String {
    "".to_string()
}

fn default_subject() -> String {
    "".to_string()
}

impl EmailTemplate {
    /// Get template_id
    pub fn template_id(&self) -> &String {
        &self.template_id
    }

    /// Get locale
    pub fn locale(&self) -> &String {
        &self.locale
    }

    /// Get message
    pub fn message(&self) -> &String {
        &self.message
    }

    /// Get sender_name
    pub fn sender_name(&self) -> &String {
        &self.sender_name
    }

    /// Get sender_email
    pub fn sender_email(&self) -> &String {
        &self.sender_email
    }

    /// Get reply_to_email
    pub fn reply_to_email(&self) -> &String {
        &self.reply_to_email
    }

    /// Get reply_to_name
    pub fn reply_to_name(&self) -> &String {
        &self.reply_to_name
    }

    /// Get subject
    pub fn subject(&self) -> &String {
        &self.subject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_template_creation() {
        let _model = <EmailTemplate as Default>::default();
        let _ = _model.template_id();
        let _ = _model.locale();
        let _ = _model.message();
        let _ = _model.sender_name();
        let _ = _model.sender_email();
        let _ = _model.reply_to_email();
        let _ = _model.reply_to_name();
        let _ = _model.subject();
    }

    #[test]
    fn test_email_template_serialization() {
        let model = <EmailTemplate as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<EmailTemplate, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
