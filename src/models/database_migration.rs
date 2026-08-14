//! DatabaseMigration model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Database Migration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct DatabaseMigration {
    /// Database migration ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Migration creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Migration update time in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Project ID that owns the migrating database.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Logical database ID being migrated.
    #[serde(rename = "databaseId")]
    pub database_id: String,
    /// Dedicated compute specification provisioned for the migration target.
    #[serde(rename = "specification")]
    pub specification: String,
    /// Migration phase. Possible values: pending, provisioned, capturing,
    /// backfilling, catching_up, verifying, ready_to_cutover, cutover, soaking,
    /// done, failed, rolled_back.
    #[serde(rename = "phase")]
    pub phase: String,
    /// Number of times a migration step has failed and been recorded.
    #[serde(rename = "attempt")]
    pub attempt: i64,
    /// Reason the most recent migration step failed, empty while none has.
    #[serde(rename = "lastError")]
    pub last_error: String,
    /// Number of documents still pending replication to the target.
    #[serde(rename = "lagDocuments")]
    pub lag_documents: i64,
    /// Time the migrated data was verified against the source in ISO 8601 format.
    #[serde(rename = "verifiedAt")]
    pub verified_at: String,
    /// Time routing was flipped to the target in ISO 8601 format.
    #[serde(rename = "cutoverAt")]
    pub cutover_at: String,
    /// Time the post-cutover soak window ends in ISO 8601 format.
    #[serde(rename = "soakUntil")]
    pub soak_until: String,
    /// Whether the migration cuts over automatically once ready. Set when the
    /// migration is created and never changed afterwards, so it always reports
    /// what was asked for.
    #[serde(rename = "autoCutover")]
    pub auto_cutover: bool,
    /// Whether a cutover has been requested and not yet attempted. Set by the
    /// cutover endpoint and cleared when the attempt is made, so a cutover that
    /// fails a check parks the migration again rather than retrying on its own.
    #[serde(rename = "cutoverRequested")]
    pub cutover_requested: bool,
    /// Whether the migration is paused.
    #[serde(rename = "paused")]
    pub paused: bool,
}

impl DatabaseMigration {
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

    /// Get project_id
    pub fn project_id(&self) -> &String {
        &self.project_id
    }

    /// Get database_id
    pub fn database_id(&self) -> &String {
        &self.database_id
    }

    /// Get specification
    pub fn specification(&self) -> &String {
        &self.specification
    }

    /// Get phase
    pub fn phase(&self) -> &String {
        &self.phase
    }

    /// Get attempt
    pub fn attempt(&self) -> &i64 {
        &self.attempt
    }

    /// Get last_error
    pub fn last_error(&self) -> &String {
        &self.last_error
    }

    /// Get lag_documents
    pub fn lag_documents(&self) -> &i64 {
        &self.lag_documents
    }

    /// Get verified_at
    pub fn verified_at(&self) -> &String {
        &self.verified_at
    }

    /// Get cutover_at
    pub fn cutover_at(&self) -> &String {
        &self.cutover_at
    }

    /// Get soak_until
    pub fn soak_until(&self) -> &String {
        &self.soak_until
    }

    /// Get auto_cutover
    pub fn auto_cutover(&self) -> &bool {
        &self.auto_cutover
    }

    /// Get cutover_requested
    pub fn cutover_requested(&self) -> &bool {
        &self.cutover_requested
    }

    /// Get paused
    pub fn paused(&self) -> &bool {
        &self.paused
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_migration_creation() {
        let _model = <DatabaseMigration as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.project_id();
        let _ = _model.database_id();
        let _ = _model.specification();
        let _ = _model.phase();
        let _ = _model.attempt();
        let _ = _model.last_error();
        let _ = _model.lag_documents();
        let _ = _model.verified_at();
        let _ = _model.cutover_at();
        let _ = _model.soak_until();
        let _ = _model.auto_cutover();
        let _ = _model.cutover_requested();
        let _ = _model.paused();
    }

    #[test]
    fn test_database_migration_serialization() {
        let model = <DatabaseMigration as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<DatabaseMigration, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
