use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::User;

#[derive(Deserialize)]
struct UpdateQuery {
    display_name: Option<String>,
    email: Option<String>,
    can_login: Option<bool>,
}

#[get("/users")]
async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match User::get_all(&pool).await {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to retrieve users. {}", e))
        }
    }
}

#[get("/users/{username}")]
async fn get(username: web::Path<(String,)>, pool: web::Data<PgPool>) -> impl Responder {
    match User::get(&pool, &username.into_inner().0).await {
        Ok(k) => HttpResponse::Ok().json(k),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => HttpResponse::NotFound().json("User not found"),
            _ => HttpResponse::InternalServerError().json(format!("Failed to get user. {}", e)),
        },
    }
}

#[post("/users")]
async fn create(user: web::Json<User>, pool: web::Data<PgPool>) -> impl Responder {
    match user.create(&pool).await {
        Ok(_) => HttpResponse::Ok().json(format!("Created user '{}'", user.username)),
        Err(e) => match e.to_string() {
            x if x.contains("duplicate key") => {
                HttpResponse::BadRequest().json(format!("User '{}' already exists.", user.username))
            }
            _ => HttpResponse::InternalServerError().json(format!("Failed to create user. {}", e)),
        },
    }
}

#[put("/users/{username}")]
async fn update(
    username: web::Path<String>,
    query: web::Query<UpdateQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut user = match User::get(&pool, &username.into_inner()).await {
        Ok(k) => k,
        Err(e) => return HttpResponse::NotFound().json(format!("User not found, {}", e)),
    };

    if let Some(d) = &query.display_name {
        user.display_name = Some(d.to_string())
    };

    if let Some(e) = &query.email {
        user.email = Some(e.to_string())
    };

    if let Some(c) = query.can_login {
        user.can_login = c
    };

    match user.update(&pool).await {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to update user. {}", e)),
    }
}

#[delete("/users/{username}")]
async fn delete(username: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    match User::get(&pool, &username.into_inner()).await {
        Ok(u) => match u.delete(&pool).await {
            Ok(_) => HttpResponse::Ok().json(format!("Deleted user '{}'", u.username)),
            Err(e) => {
                HttpResponse::InternalServerError().json(format!("Failed to delete user. {}", e))
            }
        },
        Err(e) => HttpResponse::NotFound().json(format!("Failed to retrieve user. {}", e)),
    }
}
