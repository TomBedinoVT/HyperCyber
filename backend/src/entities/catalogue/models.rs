use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Endpoint - peut être une machine, un programme, une URL, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub id: Uuid,
    pub name: String,
    pub endpoint_type: String, // "machine", "program", "url", "api", etc.
    pub description: Option<String>,
    pub address: Option<String>, // URL, IP, hostname, etc.
    pub metadata: Option<serde_json::Value>, // JSON pour stocker des infos spécifiques au type
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEndpointRequest {
    pub name: String,
    pub endpoint_type: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEndpointRequest {
    pub name: Option<String>,
    pub endpoint_type: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// Clé de licence - peut être un string ou un fichier
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseKey {
    pub id: Uuid,
    pub name: String,
    pub license_type: String, // "string", "file"
    pub key_value: Option<String>, // Pour les clés string
    pub file_path: Option<String>, // Pour les fichiers (chemin dans S3 ou storage local)
    pub file_name: Option<String>, // Nom du fichier original
    pub file_size: Option<i64>, // Taille du fichier en bytes
    pub storage_type: String, // "local", "s3"
    pub description: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLicenseKeyRequest {
    pub name: String,
    pub license_type: String,
    pub key_value: Option<String>,
    pub description: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLicenseKeyRequest {
    pub name: Option<String>,
    pub license_type: Option<String>,
    pub key_value: Option<String>,
    pub description: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

// Version de logiciel
#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareVersion {
    pub id: Uuid,
    pub name: String,
    pub version: String, // Version semver ou autre format
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub end_of_life: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSoftwareVersionRequest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub end_of_life: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSoftwareVersionRequest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub end_of_life: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

// Algorithme de cryptage
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionAlgorithm {
    pub id: Uuid,
    pub name: String,
    pub algorithm_type: String, // "symmetric", "asymmetric", "hashing", etc.
    pub key_size: Option<i32>, // Taille de la clé en bits
    pub description: Option<String>,
    pub standard: Option<String>, // Ex: "AES-256", "RSA-2048", "SHA-256"
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEncryptionAlgorithmRequest {
    pub name: String,
    pub algorithm_type: String,
    pub key_size: Option<i32>,
    pub description: Option<String>,
    pub standard: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEncryptionAlgorithmRequest {
    pub name: Option<String>,
    pub algorithm_type: Option<String>,
    pub key_size: Option<i32>,
    pub description: Option<String>,
    pub standard: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// Relations - pour lier les éléments du catalogue entre eux et avec d'autres entités
// Table de liaison générique pour permettre des relations flexibles
#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogueRelation {
    pub id: Uuid,
    pub source_type: String, // "endpoint", "license_key", "software_version", "encryption_algorithm", "entity", etc.
    pub source_id: Uuid,
    pub target_type: String,
    pub target_id: Uuid,
    pub relation_type: String, // "uses", "depends_on", "implements", "contains", etc.
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCatalogueRelationRequest {
    pub source_type: String,
    pub source_id: Uuid,
    pub target_type: String,
    pub target_id: Uuid,
    pub relation_type: String,
    pub description: Option<String>,
}

