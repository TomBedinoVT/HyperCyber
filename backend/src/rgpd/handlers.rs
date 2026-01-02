use actix_web::{web, HttpResponse, HttpRequest};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, IntoActiveModel, QueryOrder, Order, ActiveModelTrait};
use crate::rgpd::models::*;
use crate::middleware::get_current_user_id;
use crate::entities_orm::user_entity::{Entity as UserEntityEntity, Column as UserEntityColumn};
use crate::entities_orm::register_entry::{Entity as RegisterEntryEntity, Column as RegisterEntryColumn, ActiveModel as RegisterEntryActiveModel};
use crate::entities_orm::access_request::{Entity as AccessRequestEntity, Column as AccessRequestColumn, ActiveModel as AccessRequestActiveModel};
use crate::entities_orm::breach::{Entity as BreachEntity, Column as BreachColumn, ActiveModel as BreachActiveModel};
use uuid::Uuid;
use chrono::Utc;

// Helper function to check access
async fn check_entity_access(
    db: &DatabaseConnection,
    user_id: Uuid,
    entity_id: Uuid,
) -> Result<bool, sea_orm::DbErr> {
    let has_access = UserEntityEntity::find()
        .filter(UserEntityColumn::UserId.eq(user_id))
        .filter(UserEntityColumn::EntityId.eq(entity_id))
        .one(db)
        .await?;
    Ok(has_access.is_some())
}

// Registre léger
pub async fn get_register(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let entity_id: Option<Uuid> = query.get("entity_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    let entries: Vec<crate::entities_orm::register_entry::Model> = if let Some(eid) = entity_id {
        // Check access
        if !check_entity_access(db.get_ref(), user_id, eid).await.map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })? {
            return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied"
            })));
        }

        RegisterEntryEntity::find()
            .filter(RegisterEntryColumn::EntityId.eq(eid))
            .order_by(RegisterEntryColumn::CreatedAt, Order::Desc)
            .all(db.get_ref())
            .await
    } else {
        // Get all entities user has access to through user_entities
        let user_entities = UserEntityEntity::find()
            .filter(UserEntityColumn::UserId.eq(user_id))
            .all(db.get_ref())
            .await
            .map_err(|e| {
                log::error!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?;
        
        let entity_ids: Vec<Uuid> = user_entities.iter().map(|ue| ue.entity_id).collect();
        
        RegisterEntryEntity::find()
            .filter(RegisterEntryColumn::EntityId.is_in(entity_ids))
            .order_by(RegisterEntryColumn::CreatedAt, Order::Desc)
            .all(db.get_ref())
            .await
    }
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let entries: Vec<RegisterEntry> = entries.into_iter().map(|e| RegisterEntry {
        id: e.id,
        entity_id: e.entity_id,
        processing_name: e.processing_name,
        purpose: e.purpose,
        legal_basis: e.legal_basis,
        data_categories: e.data_categories,
        data_subjects: e.data_subjects,
        recipients: e.recipients,
        retention_period: e.retention_period,
        security_measures: e.security_measures,
        created_at: e.created_at,
        updated_at: e.updated_at,
    }).collect();

    Ok(HttpResponse::Ok().json(entries))
}

pub async fn add_to_register(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
    body: web::Json<CreateRegisterEntryRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let entity_id: Uuid = query.get("entity_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing entity_id"))?;

    // Check access
    if !check_entity_access(db.get_ref(), user_id, entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let now = Utc::now();
    let entry = RegisterEntryActiveModel {
        id: Set(Uuid::new_v4()),
        entity_id: Set(entity_id),
        processing_name: Set(body.processing_name.clone()),
        purpose: Set(body.purpose.clone()),
        legal_basis: Set(body.legal_basis.clone()),
        data_categories: Set(body.data_categories.clone()),
        data_subjects: Set(body.data_subjects.clone()),
        recipients: Set(body.recipients.clone()),
        retention_period: Set(body.retention_period.clone()),
        security_measures: Set(body.security_measures.clone()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let entry = RegisterEntryEntity::insert(entry)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(RegisterEntry {
        id: entry.id,
        entity_id: entry.entity_id,
        processing_name: entry.processing_name,
        purpose: entry.purpose,
        legal_basis: entry.legal_basis,
        data_categories: entry.data_categories,
        data_subjects: entry.data_subjects,
        recipients: entry.recipients,
        retention_period: entry.retention_period,
        security_measures: entry.security_measures,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
    }))
}

pub async fn update_register_entry(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRegisterEntryRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let entry_id = path.into_inner();

    // Check access through entry's entity
    let entry = RegisterEntryEntity::find_by_id(entry_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let entry = match entry {
        Some(e) => e,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Entry not found"
        }))),
    };

    if !check_entity_access(db.get_ref(), user_id, entry.entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let mut entry: RegisterEntryActiveModel = entry.into_active_model();
    if let Some(name) = &body.processing_name {
        entry.processing_name = Set(name.clone());
    }
    if let Some(purpose) = &body.purpose {
        entry.purpose = Set(purpose.clone());
    }
    if let Some(legal_basis) = &body.legal_basis {
        entry.legal_basis = Set(legal_basis.clone());
    }
    if let Some(categories) = &body.data_categories {
        entry.data_categories = Set(categories.clone());
    }
    if let Some(subjects) = &body.data_subjects {
        entry.data_subjects = Set(subjects.clone());
    }
    if let Some(recipients) = &body.recipients {
        entry.recipients = Set(recipients.clone());
    }
    if let Some(retention) = &body.retention_period {
        entry.retention_period = Set(Some(retention.clone()));
    }
    if let Some(security) = &body.security_measures {
        entry.security_measures = Set(Some(security.clone()));
    }
    entry.updated_at = Set(Utc::now());

    let entry = entry.update(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(RegisterEntry {
        id: entry.id,
        entity_id: entry.entity_id,
        processing_name: entry.processing_name,
        purpose: entry.purpose,
        legal_basis: entry.legal_basis,
        data_categories: entry.data_categories,
        data_subjects: entry.data_subjects,
        recipients: entry.recipients,
        retention_period: entry.retention_period,
        security_measures: entry.security_measures,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
    }))
}

// Demandes d'accès
pub async fn list_access_requests(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let entity_id: Option<Uuid> = query.get("entity_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    let requests: Vec<crate::entities_orm::access_request::Model> = if let Some(eid) = entity_id {
        if !check_entity_access(db.get_ref(), user_id, eid).await.map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })? {
            return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied"
            })));
        }

        AccessRequestEntity::find()
            .filter(AccessRequestColumn::EntityId.eq(eid))
            .order_by(AccessRequestColumn::CreatedAt, Order::Desc)
            .all(db.get_ref())
            .await
    } else {
        let user_entities = UserEntityEntity::find()
            .filter(UserEntityColumn::UserId.eq(user_id))
            .all(db.get_ref())
            .await
            .map_err(|e| {
                log::error!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?;
        
        let entity_ids: Vec<Uuid> = user_entities.iter().map(|ue| ue.entity_id).collect();
        
        AccessRequestEntity::find()
            .filter(AccessRequestColumn::EntityId.is_in(entity_ids))
            .order_by(AccessRequestColumn::CreatedAt, Order::Desc)
            .all(db.get_ref())
            .await
    }
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let requests: Vec<AccessRequest> = requests.into_iter().map(|r| AccessRequest {
        id: r.id,
        entity_id: r.entity_id,
        requester_name: r.requester_name,
        requester_email: r.requester_email,
        request_type: r.request_type,
        description: r.description,
        status: r.status,
        response: r.response,
        created_at: r.created_at,
        updated_at: r.updated_at,
        completed_at: r.completed_at,
    }).collect();

    Ok(HttpResponse::Ok().json(requests))
}

pub async fn create_access_request(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
    body: web::Json<CreateAccessRequestRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let entity_id: Uuid = query.get("entity_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing entity_id"))?;

    // Check access
    if !check_entity_access(db.get_ref(), user_id, entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let now = Utc::now();
    let access_request = AccessRequestActiveModel {
        id: Set(Uuid::new_v4()),
        entity_id: Set(entity_id),
        requester_name: Set(body.requester_name.clone()),
        requester_email: Set(body.requester_email.clone()),
        request_type: Set(body.request_type.clone()),
        description: Set(body.description.clone()),
        status: Set("pending".to_string()),
        response: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        completed_at: Set(None),
    };

    let access_request = AccessRequestEntity::insert(access_request)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(AccessRequest {
        id: access_request.id,
        entity_id: access_request.entity_id,
        requester_name: access_request.requester_name,
        requester_email: access_request.requester_email,
        request_type: access_request.request_type,
        description: access_request.description,
        status: access_request.status,
        response: access_request.response,
        created_at: access_request.created_at,
        updated_at: access_request.updated_at,
        completed_at: access_request.completed_at,
    }))
}

pub async fn get_access_request(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let request_id = path.into_inner();

    let access_request = AccessRequestEntity::find_by_id(request_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let access_request = match access_request {
        Some(r) => r,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Request not found"
        }))),
    };

    if !check_entity_access(db.get_ref(), user_id, access_request.entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    Ok(HttpResponse::Ok().json(AccessRequest {
        id: access_request.id,
        entity_id: access_request.entity_id,
        requester_name: access_request.requester_name,
        requester_email: access_request.requester_email,
        request_type: access_request.request_type,
        description: access_request.description,
        status: access_request.status,
        response: access_request.response,
        created_at: access_request.created_at,
        updated_at: access_request.updated_at,
        completed_at: access_request.completed_at,
    }))
}

pub async fn respond_to_request(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<RespondToRequestRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let request_id = path.into_inner();

    let access_request = AccessRequestEntity::find_by_id(request_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let access_request = match access_request {
        Some(r) => r,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Request not found"
        }))),
    };

    if !check_entity_access(db.get_ref(), user_id, access_request.entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let mut access_request: AccessRequestActiveModel = access_request.into_active_model();
    access_request.status = Set(body.status.clone());
    access_request.response = Set(body.response.clone());
    if body.status == "completed" {
        access_request.completed_at = Set(Some(Utc::now()));
    }
    access_request.updated_at = Set(Utc::now());

    let access_request = access_request.update(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(AccessRequest {
        id: access_request.id,
        entity_id: access_request.entity_id,
        requester_name: access_request.requester_name,
        requester_email: access_request.requester_email,
        request_type: access_request.request_type,
        description: access_request.description,
        status: access_request.status,
        response: access_request.response,
        created_at: access_request.created_at,
        updated_at: access_request.updated_at,
        completed_at: access_request.completed_at,
    }))
}

// Gestion des écarts
pub async fn list_breaches(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let entity_id: Option<Uuid> = query.get("entity_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    let breaches: Vec<crate::entities_orm::breach::Model> = if let Some(eid) = entity_id {
        if !check_entity_access(db.get_ref(), user_id, eid).await.map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })? {
            return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied"
            })));
        }

        BreachEntity::find()
            .filter(BreachColumn::EntityId.eq(eid))
            .order_by(BreachColumn::DiscoveryDate, Order::Desc)
            .all(db.get_ref())
            .await
    } else {
        let user_entities = UserEntityEntity::find()
            .filter(UserEntityColumn::UserId.eq(user_id))
            .all(db.get_ref())
            .await
            .map_err(|e| {
                log::error!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?;
        
        let entity_ids: Vec<Uuid> = user_entities.iter().map(|ue| ue.entity_id).collect();
        
        BreachEntity::find()
            .filter(BreachColumn::EntityId.is_in(entity_ids))
            .order_by(BreachColumn::DiscoveryDate, Order::Desc)
            .all(db.get_ref())
            .await
    }
    .map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let breaches: Vec<Breach> = breaches.into_iter().map(|b| Breach {
        id: b.id,
        entity_id: b.entity_id,
        breach_date: b.breach_date,
        discovery_date: b.discovery_date,
        description: b.description,
        data_categories_affected: b.data_categories_affected,
        number_of_subjects: b.number_of_subjects,
        severity: b.severity,
        status: b.status,
        containment_measures: b.containment_measures,
        notification_date: b.notification_date,
        authority_notified: b.authority_notified,
        subjects_notified: b.subjects_notified,
        created_at: b.created_at,
        updated_at: b.updated_at,
    }).collect();

    Ok(HttpResponse::Ok().json(breaches))
}

pub async fn create_breach(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
    body: web::Json<CreateBreachRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let entity_id: Uuid = query.get("entity_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing entity_id"))?;

    // Check access
    if !check_entity_access(db.get_ref(), user_id, entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let now = Utc::now();
    let breach = BreachActiveModel {
        id: Set(Uuid::new_v4()),
        entity_id: Set(entity_id),
        breach_date: Set(body.breach_date),
        discovery_date: Set(body.discovery_date),
        description: Set(body.description.clone()),
        data_categories_affected: Set(body.data_categories_affected.clone()),
        number_of_subjects: Set(body.number_of_subjects),
        severity: Set(body.severity.clone()),
        status: Set("detected".to_string()),
        containment_measures: Set(body.containment_measures.clone()),
        notification_date: Set(None),
        authority_notified: Set(false),
        subjects_notified: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let breach = BreachEntity::insert(breach)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(Breach {
        id: breach.id,
        entity_id: breach.entity_id,
        breach_date: breach.breach_date,
        discovery_date: breach.discovery_date,
        description: breach.description,
        data_categories_affected: breach.data_categories_affected,
        number_of_subjects: breach.number_of_subjects,
        severity: breach.severity,
        status: breach.status,
        containment_measures: breach.containment_measures,
        notification_date: breach.notification_date,
        authority_notified: breach.authority_notified,
        subjects_notified: breach.subjects_notified,
        created_at: breach.created_at,
        updated_at: breach.updated_at,
    }))
}

pub async fn get_breach(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let breach_id = path.into_inner();

    let breach = BreachEntity::find_by_id(breach_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let breach = match breach {
        Some(b) => b,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Breach not found"
        }))),
    };

    if !check_entity_access(db.get_ref(), user_id, breach.entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    Ok(HttpResponse::Ok().json(Breach {
        id: breach.id,
        entity_id: breach.entity_id,
        breach_date: breach.breach_date,
        discovery_date: breach.discovery_date,
        description: breach.description,
        data_categories_affected: breach.data_categories_affected,
        number_of_subjects: breach.number_of_subjects,
        severity: breach.severity,
        status: breach.status,
        containment_measures: breach.containment_measures,
        notification_date: breach.notification_date,
        authority_notified: breach.authority_notified,
        subjects_notified: breach.subjects_notified,
        created_at: breach.created_at,
        updated_at: breach.updated_at,
    }))
}

pub async fn update_breach(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateBreachRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let breach_id = path.into_inner();

    let breach = BreachEntity::find_by_id(breach_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let breach = match breach {
        Some(b) => b,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Breach not found"
        }))),
    };

    if !check_entity_access(db.get_ref(), user_id, breach.entity_id).await.map_err(|e| {
        log::error!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })? {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let mut breach: BreachActiveModel = breach.into_active_model();
    if let Some(date) = &body.breach_date {
        breach.breach_date = Set(*date);
    }
    if let Some(date) = &body.discovery_date {
        breach.discovery_date = Set(*date);
    }
    if let Some(desc) = &body.description {
        breach.description = Set(desc.clone());
    }
    if let Some(categories) = &body.data_categories_affected {
        breach.data_categories_affected = Set(categories.clone());
    }
    if let Some(number) = &body.number_of_subjects {
        breach.number_of_subjects = Set(Some(*number));
    }
    if let Some(severity) = &body.severity {
        breach.severity = Set(severity.clone());
    }
    if let Some(status) = &body.status {
        breach.status = Set(status.clone());
    }
    if let Some(measures) = &body.containment_measures {
        breach.containment_measures = Set(Some(measures.clone()));
    }
    if let Some(date) = &body.notification_date {
        breach.notification_date = Set(Some(*date));
    }
    if let Some(notified) = &body.authority_notified {
        breach.authority_notified = Set(*notified);
    }
    if let Some(notified) = &body.subjects_notified {
        breach.subjects_notified = Set(*notified);
    }
    breach.updated_at = Set(Utc::now());

    let breach = breach.update(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(Breach {
        id: breach.id,
        entity_id: breach.entity_id,
        breach_date: breach.breach_date,
        discovery_date: breach.discovery_date,
        description: breach.description,
        data_categories_affected: breach.data_categories_affected,
        number_of_subjects: breach.number_of_subjects,
        severity: breach.severity,
        status: breach.status,
        containment_measures: breach.containment_measures,
        notification_date: breach.notification_date,
        authority_notified: breach.authority_notified,
        subjects_notified: breach.subjects_notified,
        created_at: breach.created_at,
        updated_at: breach.updated_at,
    }))
}
