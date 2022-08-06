use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::Key;

#[derive(Deserialize)]
struct UpdateQuery {
    description: Option<String>,
    active: Option<bool>,
}

#[get("/keys/{key_name}")]
async fn get(key_name: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    match Key::get(&pool, &key_name.into_inner()).await {
        Ok(k) => HttpResponse::Ok().json(k),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => HttpResponse::NotFound().json("Key not found"),
            _ => HttpResponse::InternalServerError().json(format!("Failed to get key. {}", e)),
        },
    }
}

#[get("/keys")]
async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match Key::get_all(&pool).await {
        Ok(k) => HttpResponse::Ok().json(k),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get keys. {}", e)),
    }
}

#[post("/keys")]
async fn create(key: web::Json<Key>, pool: web::Data<PgPool>) -> impl Responder {
    match key.create(&pool).await {
        Ok(_) => HttpResponse::Ok().json(key),
        Err(e) => match e.to_string() {
            x if x.contains("duplicate key") => {
                HttpResponse::BadRequest().json(format!("Key '{}' already exists.", key.name))
            }
            _ => HttpResponse::InternalServerError().json(format!("Failed to create key. {}", e)),
        },
    }
}

#[put("/keys/{key_name}")]
async fn update(
    key_name: web::Path<String>,
    query: web::Query<UpdateQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut key = match Key::get(&pool, &key_name.into_inner()).await {
        Ok(k) => k,
        Err(_) => return HttpResponse::NotFound().json("Key not found"),
    };

    if let Some(d) = &query.description {
        key.description = Some(d.to_string())
    };

    if let Some(a) = query.active {
        key.active = a
    };

    match key.update(&pool).await {
        Ok(_) => HttpResponse::Ok().json(key),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to update key. {}", e)),
    }
}

#[delete("/keys/{key_name}")]
async fn delete(key_name: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    match Key::get(&pool, &key_name.into_inner()).await {
        Ok(k) => match k.delete(&pool).await {
            Ok(_) => HttpResponse::Ok().json(format!("Deleted key '{}'", k.name)),
            Err(e) => {
                HttpResponse::InternalServerError().json(format!("Failed to delete key. {}", e))
            }
        },
        Err(_) => HttpResponse::NotFound().json("Failed to get key"),
    }
}
