use actix_web::{web, App, HttpServer, middleware::Logger, http::header};

mod auth;
mod entities;
mod rgpd;
mod database;
mod config;
mod middleware;
mod entities_orm;

use database::get_connection;
use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();
    let db = get_connection(&config.database_url).await
        .expect("Failed to create database connection");

    // Run migrations (still using sqlx for migrations)
    // Note: SeaORM has its own migration system, but we'll keep sqlx for now
    // You can migrate to SeaORM migrations later if needed

    let server_address = format!("{}:{}", config.host, config.port);
    log::info!("Starting server on {}", server_address);

    HttpServer::new(move || {
        // In development, allow all localhost origins
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::ACCEPT_LANGUAGE,
                header::CONTENT_LANGUAGE,
            ])
            .expose_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(auth::handlers::login))
                            .route("/register", web::post().to(auth::handlers::register))
                            .route("/oidc/callback", web::get().to(auth::handlers::oidc_callback))
                            .route("/oidc/authorize", web::get().to(auth::handlers::oidc_authorize))
                            .route("/refresh", web::post().to(auth::handlers::refresh_token))
                            .route("/me", web::get().to(auth::handlers::get_current_user))
                    )
                    .service(
                        web::scope("/entities")
                            .wrap(middleware::AuthMiddleware)
                            .route("", web::get().to(entities::handlers::list_entities))
                            .route("", web::post().to(entities::handlers::create_entity))
                            .route("/{id}", web::get().to(entities::handlers::get_entity))
                            .route("/{id}", web::put().to(entities::handlers::update_entity))
                            .route("/{id}/users", web::get().to(entities::handlers::get_entity_users))
                    )
                    .service(
                        web::scope("/rgpd")
                            .wrap(middleware::AuthMiddleware)
                            .route("/register", web::get().to(rgpd::handlers::get_register))
                            .route("/register", web::post().to(rgpd::handlers::add_to_register))
                            .route("/register/{id}", web::put().to(rgpd::handlers::update_register_entry))
                            .route("/access-requests", web::get().to(rgpd::handlers::list_access_requests))
                            .route("/access-requests", web::post().to(rgpd::handlers::create_access_request))
                            .route("/access-requests/{id}", web::get().to(rgpd::handlers::get_access_request))
                            .route("/access-requests/{id}/respond", web::post().to(rgpd::handlers::respond_to_request))
                            .route("/breaches", web::get().to(rgpd::handlers::list_breaches))
                            .route("/breaches", web::post().to(rgpd::handlers::create_breach))
                            .route("/breaches/{id}", web::get().to(rgpd::handlers::get_breach))
                            .route("/breaches/{id}", web::put().to(rgpd::handlers::update_breach))
                    )
            )
    })
    .bind(&server_address)?
    .run()
    .await
}

