use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub oidc_client_id: Option<String>,
    pub oidc_client_secret: Option<String>,
    pub oidc_issuer: Option<String>,
    pub oidc_redirect_uri: Option<String>,
    pub storage_type: String, // "local" or "s3"
    pub storage_local_path: String,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub s3_access_key_id: Option<String>,
    pub s3_secret_access_key: Option<String>,
    pub s3_endpoint: Option<String>, // For S3-compatible services
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/hypercyber".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
            oidc_client_id: env::var("OIDC_CLIENT_ID").ok(),
            oidc_client_secret: env::var("OIDC_CLIENT_SECRET").ok(),
            oidc_issuer: env::var("OIDC_ISSUER").ok(),
            oidc_redirect_uri: env::var("OIDC_REDIRECT_URI").ok(),
            storage_type: env::var("STORAGE_TYPE").unwrap_or_else(|_| "local".to_string()),
            storage_local_path: env::var("STORAGE_LOCAL_PATH").unwrap_or_else(|_| "./storage".to_string()),
            s3_bucket: env::var("S3_BUCKET").ok(),
            s3_region: env::var("S3_REGION").ok(),
            s3_access_key_id: env::var("S3_ACCESS_KEY_ID").ok(),
            s3_secret_access_key: env::var("S3_SECRET_ACCESS_KEY").ok(),
            s3_endpoint: env::var("S3_ENDPOINT").ok(),
        }
    }
}

