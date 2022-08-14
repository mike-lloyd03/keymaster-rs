use actix_session::Session;
use actix_web::{delete, error, get, post, web, HttpResponse, Responder};
use log::error;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::User,
    routes::{validate_admin, validate_session},
};

#[derive(Deserialize)]
struct UpdateQuery {
    display_name: Option<String>,
    email: Option<String>,
    can_login: Option<bool>,
    admin: Option<bool>,
}

#[derive(Deserialize)]
struct ChangePasswdPayload {
    new_password: String,
}

#[get("/users")]
async fn get_all(
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    match User::get_all(&pool).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            error!("Failed to get users. {}", e);
            Err(error::ErrorInternalServerError("Failed to get users."))
        }
    }
}

#[get("/users/{username}")]
async fn get(
    session: Session,
    username: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    let username = &username.into_inner();
    match User::get(&pool, username).await {
        Ok(k) => Ok(HttpResponse::Ok().json(k)),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => {
                error!("User '{}' not found.", username);
                Err(error::ErrorNotFound("User not found"))
            }
            _ => {
                error!("Failed to get user '{}'. {}", username, e);
                Err(error::ErrorInternalServerError("Failed to get user."))
            }
        },
    }
}

#[post("/users")]
async fn create(
    session: Session,
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    match user.create(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!("Created user '{}'", user.username))),
        Err(e) => match e.to_string() {
            x if x.contains("duplicate key") => Err(error::ErrorBadRequest(format!(
                "User '{}' already exists.",
                user.username
            ))),
            _ => {
                error!("Failed to create user '{}'. {}", user.username, e);
                Err(error::ErrorInternalServerError("Failed to create user."))
            }
        },
    }
}

#[post("/users/{username}")]
async fn update(
    session: Session,
    username: web::Path<String>,
    query: web::Json<UpdateQuery>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let username = &username.into_inner();

    let mut user = match User::get(&pool, username).await {
        Ok(k) => k,
        Err(e) => {
            error!("User '{}' not found. {}", username, e);
            return Err(error::ErrorNotFound("User not found"));
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

    if let Some(c) = query.admin {
        user.admin = c
    };

    match user.update(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            error!("Failed to update user. {}", e);
            Err(error::ErrorInternalServerError("Failed to update user."))
        }
    }
}

#[delete("/users/{username}")]
async fn delete(
    session: Session,
    username: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    match User::get(&pool, &username.into_inner()).await {
        Ok(u) => match u.delete(&pool).await {
            Ok(_) => Ok(HttpResponse::Ok().json(format!("Deleted user '{}'", u.username))),
            Err(e) => {
                error!("Failed to delete user. {}", e);
                Err(error::ErrorInternalServerError("Failed to delete user."))
            }
        },
        Err(_) => Err(error::ErrorNotFound("User not found")),
    }
}

#[post("/users/{username}/set_password")]
async fn set_password(
    session: Session,
    username: web::Path<String>,
    payload: web::Json<ChangePasswdPayload>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    match User::get(&pool, &username.into_inner()).await {
        Ok(mut u) => match u.set_password(&pool, &payload.new_password).await {
            Ok(_) => {
                Ok(HttpResponse::Ok().json(format!("Password updated for user '{}'", u.username)))
            }
            Err(e) => {
                error!("Failed to update password. {}", e);
                Err(error::ErrorInternalServerError(
                    "Failed to update password.",
                ))
            }
        },
        Err(_) => Err(error::ErrorNotFound("User not found")),
    }
}

#[cfg(test)]
mod user_routes_tests {
    use crate::{models, routes};

    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::{
        cookie::{Key, SameSite},
        test,
        web::Data,
        App,
    };

    #[actix_web::test]
    async fn test_get_users() {
        let secret_key = Key::generate();

        let pool = match models::db().await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        let app = test::init_service(
            App::new()
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .cookie_secure(false)
                        .cookie_http_only(false)
                        .cookie_same_site(SameSite::Strict)
                        .build(),
                )
                .app_data(Data::new(pool.clone()))
                .service(routes::users::get_all),
        )
        .await;
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }
}
