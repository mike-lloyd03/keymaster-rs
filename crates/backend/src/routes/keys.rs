use actix_session::Session;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::error;
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::{Assignment, Key};
use crate::routes::{unpack, validate_admin, validate_session};

#[derive(Deserialize, Clone)]
struct GetAllFilter {
    active: Option<bool>,
}

#[get("/keys/{key_name}")]
async fn get(
    key_name: web::Path<String>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    let key_name = key_name.into_inner();
    match Key::get(&pool, &key_name).await {
        Ok(k) => Ok(HttpResponse::Ok().json(k)),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => Err(ErrorNotFound("Key not found")),
            _ => {
                error!("Failed to get key '{}'. {}", key_name, e);
                Err(ErrorInternalServerError("Failed to get key."))
            }
        },
    }
}

#[get("/keys")]
async fn get_all(
    pool: web::Data<PgPool>,
    session: Session,
    filter: web::Query<GetAllFilter>,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    match filter.into_inner().active {
        None => match Key::get_all(&pool).await {
            Ok(k) => Ok(HttpResponse::Ok().json(k)),
            Err(e) => {
                error!("Failed to get keys. {}", e);
                Err(ErrorInternalServerError("Failed to get keys."))
            }
        },
        Some(a) => match Key::get_all_active(&pool, a).await {
            Ok(k) => Ok(HttpResponse::Ok().json(k)),
            Err(e) => {
                error!("Failed to get keys. {}", e);
                Err(ErrorInternalServerError("Failed to get keys."))
            }
        },
    }
}

#[post("/keys")]
async fn create(
    key: web::Either<web::Json<Key>, web::Form<Key>>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let key = unpack(key);

    match key.create(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!("Created key '{}'", key.name))),
        Err(e) => match e.to_string() {
            x if x.contains("duplicate key") => Err(ErrorBadRequest("Key already exists.")),
            _ => {
                error!("Failed to create key. {}", e);
                Err(ErrorInternalServerError("Failed to create key."))
            }
        },
    }
}

#[post("/keys/{key_name}")]
async fn update(
    key_name: web::Path<String>,
    body: web::Either<web::Json<Key>, web::Form<Key>>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let body = unpack(body);
    let key_name = &key_name.into_inner();

    let mut key = match Key::get(&pool, key_name).await {
        Ok(k) => k,
        Err(e) => {
            error!("Key '{}' not found. {}", key_name, e);
            return Err(ErrorNotFound("Key not found."));
        }
    };

    key.description = body.description;
    key.active = body.active;

    match key.update(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!("Updated key '{}'", key.name))),
        Err(e) => {
            error!("Failed to update key. {}", e);
            Err(ErrorInternalServerError("Failed to update key."))
        }
    }
}

#[delete("/keys/{key_name}")]
async fn delete(
    key_name: web::Path<String>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    match Key::get(&pool, &key_name.into_inner()).await {
        Ok(k) => match k.delete(&pool).await {
            Ok(_) => Ok(HttpResponse::Ok().json(format!("Deleted key '{}'", k.name))),
            Err(e) => {
                error!("Failed to delete key. {}", e);
                Err(ErrorInternalServerError("Failed to delete key."))
            }
        },
        Err(_) => Err(ErrorNotFound("Key not found.")),
    }
}

#[get("/keys/{key_name}/assignments")]
async fn get_assignments(
    key_name: web::Path<String>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    let key_name = key_name.into_inner();
    match Assignment::get_assignments_by_key(&pool, &key_name).await {
        Ok(k) => Ok(HttpResponse::Ok().json(k)),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => Err(ErrorNotFound("Key not found")),
            _ => {
                error!("Failed to get key '{}'. {}", key_name, e);
                Err(ErrorInternalServerError("Failed to get key."))
            }
        },
    }
}
