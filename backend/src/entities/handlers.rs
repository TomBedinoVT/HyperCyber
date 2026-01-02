use actix_web::{web, HttpResponse, HttpRequest};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, IntoActiveModel, ActiveModelTrait};
use crate::entities::models::*;
use crate::middleware::get_current_user_id;
use crate::entities_orm::entity::{Entity as EntityEntity, ActiveModel as EntityActiveModel};
use crate::entities_orm::user_entity::{Entity as UserEntityEntity, Column as UserEntityColumn, ActiveModel as UserEntityActiveModel};
use crate::entities_orm::user::Entity as UserEntity;
use uuid::Uuid;
use chrono::Utc;

pub async fn list_entities(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Find entities through user_entities relationship
    let user_entities = UserEntityEntity::find()
        .filter(UserEntityColumn::UserId.eq(user_id))
        .find_also_related(EntityEntity)
        .all(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let entities: Vec<Entity> = user_entities
        .into_iter()
        .filter_map(|(_, entity)| entity)
        .map(|e| Entity {
            id: e.id,
            name: e.name,
            description: e.description,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(entities))
}

pub async fn create_entity(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<CreateEntityRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let now = Utc::now();
    let entity = EntityActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(body.name.clone()),
        description: Set(body.description.clone()),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let entity = EntityEntity::insert(entity)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    // Add creator as admin
    let user_entity = UserEntityActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        entity_id: Set(entity.id),
        role: Set("admin".to_string()),
        created_at: Set(now),
    };

    UserEntityEntity::insert(user_entity)
        .exec(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Created().json(Entity {
        id: entity.id,
        name: entity.name,
        description: entity.description,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    }))
}

pub async fn get_entity(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let entity_id = path.into_inner();

    // Check if user has access to this entity
    let has_access = UserEntityEntity::find()
        .filter(UserEntityColumn::UserId.eq(user_id))
        .filter(UserEntityColumn::EntityId.eq(entity_id))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if has_access.is_none() {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    let entity = EntityEntity::find_by_id(entity_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    match entity {
        Some(e) => Ok(HttpResponse::Ok().json(Entity {
            id: e.id,
            name: e.name,
            description: e.description,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Entity not found"
        }))),
    }
}

pub async fn update_entity(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEntityRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let entity_id = path.into_inner();

    // Check if user is admin of this entity
    let is_admin = UserEntityEntity::find()
        .filter(UserEntityColumn::UserId.eq(user_id))
        .filter(UserEntityColumn::EntityId.eq(entity_id))
        .filter(UserEntityColumn::Role.eq("admin"))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if is_admin.is_none() {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required"
        })));
    }

    let entity = EntityEntity::find_by_id(entity_id)
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let mut entity: EntityActiveModel = match entity {
        Some(e) => e.into_active_model(),
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Entity not found"
        }))),
    };

    if let Some(name) = &body.name {
        entity.name = Set(name.clone());
    }
    if let Some(description) = &body.description {
        entity.description = Set(Some(description.clone()));
    }
    entity.updated_at = Set(Utc::now());

    let entity = entity.update(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(Entity {
        id: entity.id,
        name: entity.name,
        description: entity.description,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    }))
}

pub async fn get_entity_users(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;
    let entity_id = path.into_inner();

    // Check if user has access to this entity
    let has_access = UserEntityEntity::find()
        .filter(UserEntityColumn::UserId.eq(user_id))
        .filter(UserEntityColumn::EntityId.eq(entity_id))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if has_access.is_none() {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        })));
    }

    // Get all user_entities for this entity with user details
    let user_entities: Vec<(crate::entities_orm::user_entity::Model, Option<crate::entities_orm::user::Model>)> = UserEntityEntity::find()
        .filter(UserEntityColumn::EntityId.eq(entity_id))
        .find_also_related(UserEntity)
        .all(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let users: Vec<EntityUser> = user_entities
        .into_iter()
        .filter_map(|(ue, user)| {
            user.map(|u| EntityUser {
                id: ue.id,
                user_id: ue.user_id,
                entity_id: ue.entity_id,
                role: ue.role,
                email: u.email,
                first_name: u.first_name,
                last_name: u.last_name,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(users))
}
