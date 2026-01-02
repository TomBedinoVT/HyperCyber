use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Registre léger RGPD
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RegisterEntry {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub processing_name: String,
    pub purpose: String,
    pub legal_basis: String,
    pub data_categories: Vec<String>,
    pub data_subjects: Vec<String>,
    pub recipients: Vec<String>,
    pub retention_period: Option<String>,
    pub security_measures: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRegisterEntryRequest {
    pub processing_name: String,
    pub purpose: String,
    pub legal_basis: String,
    pub data_categories: Vec<String>,
    pub data_subjects: Vec<String>,
    pub recipients: Vec<String>,
    pub retention_period: Option<String>,
    pub security_measures: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRegisterEntryRequest {
    pub processing_name: Option<String>,
    pub purpose: Option<String>,
    pub legal_basis: Option<String>,
    pub data_categories: Option<Vec<String>>,
    pub data_subjects: Option<Vec<String>>,
    pub recipients: Option<Vec<String>>,
    pub retention_period: Option<String>,
    pub security_measures: Option<String>,
}

// Demandes d'accès RGPD
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccessRequest {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub requester_name: String,
    pub requester_email: String,
    pub request_type: String, // "access", "rectification", "erasure", "portability", "objection"
    pub description: Option<String>,
    pub status: String, // "pending", "in_progress", "completed", "rejected"
    pub response: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccessRequestRequest {
    pub requester_name: String,
    pub requester_email: String,
    pub request_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RespondToRequestRequest {
    pub status: String,
    pub response: Option<String>,
}

// Gestion des écarts (breaches)
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Breach {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub breach_date: DateTime<Utc>,
    pub discovery_date: DateTime<Utc>,
    pub description: String,
    pub data_categories_affected: Vec<String>,
    pub number_of_subjects: Option<i32>,
    pub severity: String, // "low", "medium", "high", "critical"
    pub status: String, // "detected", "contained", "investigating", "resolved", "reported"
    pub containment_measures: Option<String>,
    pub notification_date: Option<DateTime<Utc>>,
    pub authority_notified: bool,
    pub subjects_notified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBreachRequest {
    pub breach_date: DateTime<Utc>,
    pub discovery_date: DateTime<Utc>,
    pub description: String,
    pub data_categories_affected: Vec<String>,
    pub number_of_subjects: Option<i32>,
    pub severity: String,
    pub containment_measures: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBreachRequest {
    pub breach_date: Option<DateTime<Utc>>,
    pub discovery_date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub data_categories_affected: Option<Vec<String>>,
    pub number_of_subjects: Option<i32>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub containment_measures: Option<String>,
    pub notification_date: Option<DateTime<Utc>>,
    pub authority_notified: Option<bool>,
    pub subjects_notified: Option<bool>,
}

