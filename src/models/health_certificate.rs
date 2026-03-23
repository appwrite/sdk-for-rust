//! HealthCertificate model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Health Certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct HealthCertificate {
    /// Certificate name
    #[serde(rename = "name")]
    pub name: String,
    /// Subject SN
    #[serde(rename = "subjectSN")]
    pub subject_sn: String,
    /// Issuer organisation
    #[serde(rename = "issuerOrganisation")]
    pub issuer_organisation: String,
    /// Valid from
    #[serde(rename = "validFrom")]
    pub valid_from: String,
    /// Valid to
    #[serde(rename = "validTo")]
    pub valid_to: String,
    /// Signature type SN
    #[serde(rename = "signatureTypeSN")]
    pub signature_type_sn: String,
}

impl HealthCertificate {
    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get subject_sn
    pub fn subject_sn(&self) -> &String {
        &self.subject_sn
    }

    /// Get issuer_organisation
    pub fn issuer_organisation(&self) -> &String {
        &self.issuer_organisation
    }

    /// Get valid_from
    pub fn valid_from(&self) -> &String {
        &self.valid_from
    }

    /// Get valid_to
    pub fn valid_to(&self) -> &String {
        &self.valid_to
    }

    /// Get signature_type_sn
    pub fn signature_type_sn(&self) -> &String {
        &self.signature_type_sn
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_certificate_creation() {
        let _model = <HealthCertificate as Default>::default();
        let _ = _model.name();
        let _ = _model.subject_sn();
        let _ = _model.issuer_organisation();
        let _ = _model.valid_from();
        let _ = _model.valid_to();
        let _ = _model.signature_type_sn();
    }

    #[test]
    fn test_health_certificate_serialization() {
        let model = <HealthCertificate as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<HealthCertificate, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
