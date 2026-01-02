use async_trait::async_trait;
use uuid::Uuid;
use std::io::Result as IoResult;
use aws_sdk_s3::primitives::ByteStream;

#[async_trait]
pub trait Storage: Send + Sync {
    /// Sauvegarde un fichier et retourne le chemin/URL
    async fn save_file(&self, file_data: &[u8], file_name: &str, license_key_id: Uuid) -> IoResult<String>;
    
    /// Récupère un fichier
    async fn get_file(&self, file_path: &str) -> IoResult<Vec<u8>>;
    
    /// Supprime un fichier
    async fn delete_file(&self, file_path: &str) -> IoResult<()>;
    
    /// Obtient la taille d'un fichier
    async fn get_file_size(&self, file_path: &str) -> IoResult<u64>;
}

// Implémentation pour le stockage local
pub struct LocalStorage {
    base_path: String,
}

impl LocalStorage {
    pub fn new(base_path: String) -> Self {
        // Créer le répertoire s'il n'existe pas
        std::fs::create_dir_all(&base_path).ok();
        Self { base_path }
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn save_file(&self, file_data: &[u8], file_name: &str, license_key_id: Uuid) -> IoResult<String> {
        use tokio::fs;
        use tokio::io::AsyncWriteExt;
        
        // Créer un chemin unique basé sur l'ID de la clé de licence
        let file_path = format!("{}/{}/{}", self.base_path, license_key_id, file_name);
        
        // Créer le répertoire parent si nécessaire
        if let Some(parent) = std::path::Path::new(&file_path).parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Écrire le fichier
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(file_data).await?;
        file.sync_all().await?;
        
        Ok(file_path)
    }
    
    async fn get_file(&self, file_path: &str) -> IoResult<Vec<u8>> {
        use tokio::fs;
        fs::read(file_path).await
    }
    
    async fn delete_file(&self, file_path: &str) -> IoResult<()> {
        use tokio::fs;
        fs::remove_file(file_path).await
    }
    
    async fn get_file_size(&self, file_path: &str) -> IoResult<u64> {
        use tokio::fs;
        let metadata = fs::metadata(file_path).await?;
        Ok(metadata.len())
    }
}

// Implémentation pour S3
pub struct S3Storage {
    bucket: String,
    client: aws_sdk_s3::Client,
}

impl S3Storage {
    pub async fn new(bucket: String, region: Option<String>, endpoint: Option<String>, access_key_id: Option<String>, secret_access_key: Option<String>) -> Self {
        let mut config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        
        // Configuration personnalisée si fournie
        let mut config_builder = config.to_builder();
        
        if let Some(region_str) = region {
            config_builder = config_builder.region(aws_sdk_s3::config::Region::new(region_str));
        }
        
        // Pour les services S3-compatibles (MinIO, etc.)
        if let Some(endpoint_str) = endpoint {
            config_builder = config_builder.endpoint_url(endpoint_str);
        }
        
        // Credentials personnalisées si fournies
        if let (Some(ak), Some(sk)) = (access_key_id, secret_access_key) {
            let credentials = aws_sdk_s3::config::Credentials::new(ak, sk, None, None, "static");
            let credentials_provider = aws_sdk_s3::config::SharedCredentialsProvider::new(credentials);
            config_builder = config_builder.credentials_provider(credentials_provider);
        }
        
        config = config_builder.build();
        let client = aws_sdk_s3::Client::new(&config);
        Self { bucket, client }
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn save_file(&self, file_data: &[u8], file_name: &str, license_key_id: Uuid) -> IoResult<String> {
        let key = format!("license-keys/{}/{}", license_key_id, file_name);
        
        let body = ByteStream::from(file_data.to_vec());
        
        let result = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(body)
            .send()
            .await;
        
        match result {
            Ok(_) => Ok(key),
            Err(e) => {
                log::error!("S3 upload error: {}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("S3 upload failed: {}", e)
                ))
            }
        }
    }
    
    async fn get_file(&self, file_path: &str) -> IoResult<Vec<u8>> {
        let result = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(file_path)
            .send()
            .await;
        
        match result {
            Ok(output) => {
                let mut data = Vec::new();
                let mut body = output.body;
                while let Some(chunk) = body.next().await {
                    match chunk {
                        Ok(bytes) => data.extend_from_slice(&bytes),
                        Err(e) => {
                            log::error!("S3 read error: {}", e);
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("S3 read failed: {}", e)
                            ));
                        }
                    }
                }
                Ok(data)
            }
            Err(e) => {
                log::error!("S3 get error: {}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("S3 get failed: {}", e)
                ))
            }
        }
    }
    
    async fn delete_file(&self, file_path: &str) -> IoResult<()> {
        let result = self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(file_path)
            .send()
            .await;
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("S3 delete error: {}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("S3 delete failed: {}", e)
                ))
            }
        }
    }
    
    async fn get_file_size(&self, file_path: &str) -> IoResult<u64> {
        let result = self.client
            .head_object()
            .bucket(&self.bucket)
            .key(file_path)
            .send()
            .await;
        
        match result {
            Ok(output) => {
                Ok(output.content_length().unwrap_or(0) as u64)
            }
            Err(e) => {
                log::error!("S3 head error: {}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("S3 head failed: {}", e)
                ))
            }
        }
    }
}

