use actix_web::{web, HttpResponse, HttpRequest};
use actix_multipart::Multipart;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, IntoActiveModel, QueryOrder, Order, ActiveModelTrait};
use crate::entities::catalogue::models::*;
use crate::middleware::get_current_user_id;
use crate::entities_orm::{
    endpoint::{Entity as EndpointEntity, ActiveModel as EndpointActiveModel},
    license_key::{Entity as LicenseKeyEntity, ActiveModel as LicenseKeyActiveModel},
    software_version::{Entity as SoftwareVersionEntity, ActiveModel as SoftwareVersionActiveModel},
    encryption_algorithm::{Entity as EncryptionAlgorithmEntity, ActiveModel as EncryptionAlgorithmActiveModel},
    catalogue_relation::{Entity as CatalogueRelationEntity, ActiveModel as CatalogueRelationActiveModel},
};
use uuid::Uuid;
use chrono::Utc;

// Helper function to check access (simplified - catalogue items are not entity-specific by default)
// But we can add entity_id later if needed
async fn check_entity_access(
    db: &DatabaseConnection,
    user_id: Uuid,
    entity_id: Option<Uuid>,
) -> Result<bool, sea_orm::DbErr> {
    if let Some(eid) = entity_id {
        use crate::entities_orm::user_entity::{Entity as UserEntityEntity, Column as UserEntityColumn};
        let has_access = UserEntityEntity::find()
            .filter(UserEntityColumn::UserId.eq(user_id))
            .filter(UserEntityColumn::EntityId.eq(eid))
            .one(db)
            .await?;
        Ok(has_access.is_some())
    } else {
        Ok(true) // Pas de restriction d'entité
    }
}

// ========== Endpoints ==========

pub async fn list_endpoints(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let endpoint_type: Option<String> = query.get("endpoint_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut query = EndpointEntity::find();
    if let Some(et) = endpoint_type {
        use crate::entities_orm::endpoint::Column as EndpointColumn;
        query = query.filter(EndpointColumn::EndpointType.eq(et));
    }

    let endpoints = query
        .order_by(crate::entities_orm::endpoint::Column::CreatedAt, Order::Desc)
        .all(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let endpoints: Vec<Endpoint> = endpoints.into_iter().map(|e| Endpoint {
        id: e.id,
        name: e.name,
        endpoint_type: e.endpoint_type,
        description: e.description,
        address: e.address,
        metadata: e.metadata,
        created_at: e.created_at,
        updated_at: e.updated_at,
    }).collect();

    Ok(HttpResponse::Ok().json(endpoints))
}

pub async fn create_endpoint(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<CreateEndpointRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let now = Utc::now();
    let endpoint = EndpointActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(body.name.clone()),
        endpoint_type: Set(body.endpoint_type.clone()),
        description: Set(body.description.clone()),
        address: Set(body.address.clone()),
        metadata: Set(body.metadata.clone()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let endpoint = EndpointEntity::insert(endpoint)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(Endpoint {
        id: endpoint.id,
        name: endpoint.name,
        endpoint_type: endpoint.endpoint_type,
        description: endpoint.description,
        address: endpoint.address,
        metadata: endpoint.metadata,
        created_at: endpoint.created_at,
        updated_at: endpoint.updated_at,
    }))
}

pub async fn get_endpoint(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let endpoint_id = path.into_inner();

    let endpoint = EndpointEntity::find_by_id(endpoint_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    match endpoint {
        Some(e) => Ok(HttpResponse::Ok().json(Endpoint {
            id: e.id,
            name: e.name,
            endpoint_type: e.endpoint_type,
            description: e.description,
            address: e.address,
            metadata: e.metadata,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Endpoint not found"
        }))),
    }
}

pub async fn update_endpoint(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEndpointRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let endpoint_id = path.into_inner();

    let endpoint = EndpointEntity::find_by_id(endpoint_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let mut endpoint: EndpointActiveModel = match endpoint {
        Some(e) => e.into_active_model(),
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Endpoint not found"
        }))),
    };

    if let Some(name) = &body.name {
        endpoint.name = Set(name.clone());
    }
    if let Some(endpoint_type) = &body.endpoint_type {
        endpoint.endpoint_type = Set(endpoint_type.clone());
    }
    if let Some(description) = &body.description {
        endpoint.description = Set(Some(description.clone()));
    }
    if let Some(address) = &body.address {
        endpoint.address = Set(Some(address.clone()));
    }
    if let Some(metadata) = &body.metadata {
        endpoint.metadata = Set(Some(metadata.clone()));
    }
    endpoint.updated_at = Set(Utc::now());

    let endpoint = endpoint.update(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(Endpoint {
        id: endpoint.id,
        name: endpoint.name,
        endpoint_type: endpoint.endpoint_type,
        description: endpoint.description,
        address: endpoint.address,
        metadata: endpoint.metadata,
        created_at: endpoint.created_at,
        updated_at: endpoint.updated_at,
    }))
}

// ========== License Keys ==========

pub async fn list_license_keys(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let license_keys = LicenseKeyEntity::find()
        .order_by(crate::entities_orm::license_key::Column::CreatedAt, Order::Desc)
        .all(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let license_keys: Vec<LicenseKey> = license_keys.into_iter().map(|k| LicenseKey {
        id: k.id,
        name: k.name,
        license_type: k.license_type,
        key_value: k.key_value,
        file_path: k.file_path,
        file_name: k.file_name,
        file_size: k.file_size,
        storage_type: k.storage_type,
        description: k.description,
        expires_at: k.expires_at,
        created_at: k.created_at,
        updated_at: k.updated_at,
    }).collect();

    Ok(HttpResponse::Ok().json(license_keys))
}

pub async fn create_license_key(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<CreateLicenseKeyRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let now = Utc::now();
    let license_key = LicenseKeyActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(body.name.clone()),
        license_type: Set(body.license_type.clone()),
        key_value: Set(body.key_value.clone()),
        file_path: Set(None),
        file_name: Set(None),
        file_size: Set(None),
        storage_type: Set("local".to_string()), // Default to local
        description: Set(body.description.clone()),
        expires_at: Set(body.expires_at),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let license_key = LicenseKeyEntity::insert(license_key)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(LicenseKey {
        id: license_key.id,
        name: license_key.name,
        license_type: license_key.license_type,
        key_value: license_key.key_value,
        file_path: license_key.file_path,
        file_name: license_key.file_name,
        file_size: license_key.file_size,
        storage_type: license_key.storage_type,
        description: license_key.description,
        expires_at: license_key.expires_at,
        created_at: license_key.created_at,
        updated_at: license_key.updated_at,
    }))
}

pub async fn upload_license_key_file(
    _db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    _payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let _license_key_id = path.into_inner();

    // TODO: Implémenter l'upload de fichier avec le storage
    // Pour l'instant, on retourne une erreur
    Ok(HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "File upload not yet implemented"
    })))
}

// ========== Software Versions ==========

pub async fn list_software_versions(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let versions = SoftwareVersionEntity::find()
        .order_by(crate::entities_orm::software_version::Column::CreatedAt, Order::Desc)
        .all(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let versions: Vec<SoftwareVersion> = versions.into_iter().map(|v| SoftwareVersion {
        id: v.id,
        name: v.name,
        version: v.version,
        description: v.description,
        release_date: v.release_date,
        end_of_life: v.end_of_life,
        metadata: v.metadata,
        created_at: v.created_at,
        updated_at: v.updated_at,
    }).collect();

    Ok(HttpResponse::Ok().json(versions))
}

pub async fn create_software_version(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<CreateSoftwareVersionRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let now = Utc::now();
    let version = SoftwareVersionActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(body.name.clone()),
        version: Set(body.version.clone()),
        description: Set(body.description.clone()),
        release_date: Set(body.release_date),
        end_of_life: Set(body.end_of_life),
        metadata: Set(body.metadata.clone()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let version = SoftwareVersionEntity::insert(version)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(SoftwareVersion {
        id: version.id,
        name: version.name,
        version: version.version,
        description: version.description,
        release_date: version.release_date,
        end_of_life: version.end_of_life,
        metadata: version.metadata,
        created_at: version.created_at,
        updated_at: version.updated_at,
    }))
}

// ========== Encryption Algorithms ==========

pub async fn list_encryption_algorithms(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let algorithms = EncryptionAlgorithmEntity::find()
        .order_by(crate::entities_orm::encryption_algorithm::Column::CreatedAt, Order::Desc)
        .all(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let algorithms: Vec<EncryptionAlgorithm> = algorithms.into_iter().map(|a| EncryptionAlgorithm {
        id: a.id,
        name: a.name,
        algorithm_type: a.algorithm_type,
        key_size: a.key_size,
        description: a.description,
        standard: a.standard,
        metadata: a.metadata,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }).collect();

    Ok(HttpResponse::Ok().json(algorithms))
}

pub async fn create_encryption_algorithm(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<CreateEncryptionAlgorithmRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let now = Utc::now();
    let algorithm = EncryptionAlgorithmActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(body.name.clone()),
        algorithm_type: Set(body.algorithm_type.clone()),
        key_size: Set(body.key_size),
        description: Set(body.description.clone()),
        standard: Set(body.standard.clone()),
        metadata: Set(body.metadata.clone()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let algorithm = EncryptionAlgorithmEntity::insert(algorithm)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(EncryptionAlgorithm {
        id: algorithm.id,
        name: algorithm.name,
        algorithm_type: algorithm.algorithm_type,
        key_size: algorithm.key_size,
        description: algorithm.description,
        standard: algorithm.standard,
        metadata: algorithm.metadata,
        created_at: algorithm.created_at,
        updated_at: algorithm.updated_at,
    }))
}

// ========== Catalogue Relations ==========

pub async fn create_catalogue_relation(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<CreateCatalogueRelationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let now = Utc::now();
    let relation = CatalogueRelationActiveModel {
        id: Set(Uuid::new_v4()),
        source_type: Set(body.source_type.clone()),
        source_id: Set(body.source_id),
        target_type: Set(body.target_type.clone()),
        target_id: Set(body.target_id),
        relation_type: Set(body.relation_type.clone()),
        description: Set(body.description.clone()),
        created_at: Set(now),
    };

    let relation = CatalogueRelationEntity::insert(relation)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(CatalogueRelation {
        id: relation.id,
        source_type: relation.source_type,
        source_id: relation.source_id,
        target_type: relation.target_type,
        target_id: relation.target_id,
        relation_type: relation.relation_type,
        description: relation.description,
        created_at: relation.created_at,
    }))
}

