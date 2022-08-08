use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::User;

#[derive(Deserialize)]
struct UpdateQuery {
    display_name: Option<String>,
    email: Option<String>,
    can_login: Option<bool>,
}

#[derive(Deserialize)]
struct ChangePasswdPayload {
    new_password: String,
}

#[get("/users")]
async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match User::get_all(&pool).await {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => {
            error!("Failed to get users. {}", e);
            HttpResponse::InternalServerError().json("Failed to get users.")
        }
    }
}

#[get("/users/{username}")]
async fn get(username: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let username = &username.into_inner();
    match User::get(&pool, username).await {
        Ok(k) => HttpResponse::Ok().json(k),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => {
                error!("User '{}' not found.", username);
                HttpResponse::NotFound().json("User not found.")
            }
            _ => {
                error!("Failed to get user '{}'. {}", username, e);
                HttpResponse::InternalServerError().json("Failed to get user.")
            }
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
            _ => {
                error!("Failed to create user '{}'. {}", user.username, e);
                HttpResponse::InternalServerError().json("Failed to create user.")
            }
        },
    }
}

#[put("/users/{username}")]
async fn update(
    username: web::Path<String>,
    query: web::Json<UpdateQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let username = &username.into_inner();

    let mut user = match User::get(&pool, username).await {
        Ok(k) => k,
        Err(e) => {
            error!("User '{}' not found. {}", username, e);
            return HttpResponse::NotFound().json("User not found");
        }
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
        Err(e) => {
            error!("Failed to update user. {}", e);
            HttpResponse::InternalServerError().json("Failed to update user.")
        }
    }
}

#[delete("/users/{username}")]
async fn delete(username: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    match User::get(&pool, &username.into_inner()).await {
        Ok(u) => match u.delete(&pool).await {
            Ok(_) => HttpResponse::Ok().json(format!("Deleted user '{}'", u.username)),
            Err(e) => {
                error!("Failed to delete user. {}", e);
                HttpResponse::InternalServerError().json("Failed to delete user.")
            }
        },
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}

#[put("/users/{username}/set_password")]
async fn set_password(
    username: web::Path<String>,
    payload: web::Json<ChangePasswdPayload>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match User::get(&pool, &username.into_inner()).await {
        Ok(mut u) => match u.set_password(&pool, &payload.new_password).await {
            Ok(_) => HttpResponse::Ok().json(format!("Password updated for user '{}'", u.username)),
            Err(e) => {
                error!("Failed to update password. {}", e);
                HttpResponse::InternalServerError().json("Failed to update password.")
            }
        },
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}
