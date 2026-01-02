use async_trait::async_trait;
use uuid::Uuid;
use std::io::Result as IoResult;

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

// Implémentation pour S3 (à implémenter avec aws-sdk-s3)
// Pour l'instant, on laisse un placeholder
pub struct S3Storage {
    bucket: String,
    // client: s3::Client, // À implémenter avec aws-sdk-s3
}

impl S3Storage {
    pub fn new(bucket: String) -> Self {
        Self { bucket }
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn save_file(&self, _file_data: &[u8], _file_name: &str, _license_key_id: Uuid) -> IoResult<String> {
        // TODO: Implémenter avec aws-sdk-s3
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "S3 storage not yet implemented"
        ))
    }
    
    async fn get_file(&self, _file_path: &str) -> IoResult<Vec<u8>> {
        // TODO: Implémenter avec aws-sdk-s3
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "S3 storage not yet implemented"
        ))
    }
    
    async fn delete_file(&self, _file_path: &str) -> IoResult<()> {
        // TODO: Implémenter avec aws-sdk-s3
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "S3 storage not yet implemented"
        ))
    }
    
    async fn get_file_size(&self, _file_path: &str) -> IoResult<u64> {
        // TODO: Implémenter avec aws-sdk-s3
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "S3 storage not yet implemented"
        ))
    }
}

