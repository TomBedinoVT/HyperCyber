use actix_web::{web, HttpResponse, HttpRequest};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::auth::models::*;
use crate::auth::jwt::{create_token, create_refresh_token, verify_token};
use crate::config::Config;
use crate::middleware::get_current_user_id;
use crate::entities_orm::user::{Entity as UserEntity, Column as UserColumn, ActiveModel as UserActiveModel};
use uuid::Uuid;
use chrono::Utc;

pub async fn login(
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(&req.email))
        .filter(UserColumn::IsActive.eq(true))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let user = match user {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        }))),
    };

    if !verify(&req.password, &user.password_hash).unwrap_or(false) {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })));
    }

    let token = create_token(user.id, user.email.clone(), &config)
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    let refresh_token = create_refresh_token(user.id, user.email.clone())
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        refresh_token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        },
    }))
}

pub async fn register(
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Check if user already exists
    let existing = UserEntity::find()
        .filter(UserColumn::Email.eq(&req.email))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if existing.is_some() {
        return Ok(HttpResponse::Conflict().json(serde_json::json!({
            "error": "User already exists"
        })));
    }

    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| {
            log::error!("Bcrypt error: {}", e);
            actix_web::error::ErrorInternalServerError("Password hashing failed")
        })?;

    let now = Utc::now();
    let user = UserActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(req.email.clone()),
        password_hash: Set(password_hash),
        first_name: Set(req.first_name.clone()),
        last_name: Set(req.last_name.clone()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = UserEntity::insert(user)
        .exec_with_returning(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let token = create_token(user.id, user.email.clone(), &config)
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    let refresh_token = create_refresh_token(user.id, user.email.clone())
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        refresh_token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        },
    }))
}

pub async fn refresh_token(
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    req: web::Json<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let refresh_token = req.get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing refresh_token"))?;

    let claims = verify_token(refresh_token, &config)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid refresh token"))?;

    // Verify user still exists and is active
    let user = UserEntity::find()
        .filter(UserColumn::Id.eq(claims.user_id))
        .filter(UserColumn::IsActive.eq(true))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let user = user.ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))?;

    let token = create_token(user.id, user.email.clone(), &config)
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token
    })))
}

pub async fn get_current_user(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_current_user_id(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let user = UserEntity::find()
        .filter(UserColumn::Id.eq(user_id))
        .filter(UserColumn::IsActive.eq(true))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let user = user.ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    Ok(HttpResponse::Ok().json(UserInfo {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    }))
}

pub async fn oidc_authorize(
    config: web::Data<Config>,
) -> Result<HttpResponse, actix_web::Error> {
    if config.oidc_issuer.is_none() || config.oidc_client_id.is_none() || config.oidc_redirect_uri.is_none() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "OIDC not configured"
        })));
    }

    let issuer = config.oidc_issuer.as_ref().unwrap();
    let discovery = crate::auth::oidc::discover_oidc_config(issuer)
        .await
        .map_err(|e| {
            log::error!("OIDC discovery error: {}", e);
            actix_web::error::ErrorInternalServerError("OIDC discovery failed")
        })?;

    let client_id = config.oidc_client_id.as_ref().unwrap();
    let redirect_uri = config.oidc_redirect_uri.as_ref().unwrap();
    
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid email profile",
        discovery.authorization_endpoint,
        client_id,
        urlencoding::encode(redirect_uri)
    );

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish())
}

pub async fn oidc_callback(
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let code = req
        .uri()
        .query()
        .and_then(|q| {
            q.split('&')
                .find_map(|pair| {
                    let mut parts = pair.split('=');
                    if parts.next() == Some("code") {
                        parts.next().and_then(|c| {
                            urlencoding::decode(c)
                                .ok()
                                .map(|decoded| decoded.to_string())
                        })
                    } else {
                        None
                    }
                })
        })
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing authorization code"))?;

    let issuer = config.oidc_issuer.as_ref()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("OIDC not configured"))?;

    let discovery = crate::auth::oidc::discover_oidc_config(issuer)
        .await
        .map_err(|e| {
            log::error!("OIDC discovery error: {}", e);
            actix_web::error::ErrorInternalServerError("OIDC discovery failed")
        })?;

    let token_response = crate::auth::oidc::exchange_code_for_token(&code, &config, &discovery)
        .await
        .map_err(|e| {
            log::error!("Token exchange error: {}", e);
            actix_web::error::ErrorInternalServerError("Token exchange failed")
        })?;

    let user_info = crate::auth::oidc::get_user_info(&token_response.access_token, &discovery)
        .await
        .map_err(|e| {
            log::error!("User info error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to get user info")
        })?;

    // Get or create user
    let email = user_info.email
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Email not provided by OIDC provider"))?;

    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(&email))
        .one(db.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    let user = if let Some(u) = user {
        u
    } else {
        // Create new user from OIDC
        let password_hash = bcrypt::hash("oidc-user", bcrypt::DEFAULT_COST)
            .map_err(|e| {
                log::error!("Bcrypt error: {}", e);
                actix_web::error::ErrorInternalServerError("Password hashing failed")
            })?;

        let now = Utc::now();
        let new_user = UserActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(email.clone()),
            password_hash: Set(password_hash),
            first_name: Set(user_info.given_name),
            last_name: Set(user_info.family_name),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        };

        UserEntity::insert(new_user)
            .exec_with_returning(db.get_ref())
            .await
            .map_err(|e| {
                log::error!("Database error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?
    };

    let token = create_token(user.id, user.email.clone(), &config)
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    let refresh_token = create_refresh_token(user.id, user.email.clone())
        .map_err(|e| {
            log::error!("JWT error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation failed")
        })?;

    // Redirect to frontend with tokens
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let redirect_url = format!(
        "{}/auth/callback?token={}&refresh_token={}",
        frontend_url,
        urlencoding::encode(&token),
        urlencoding::encode(&refresh_token)
    );

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}

