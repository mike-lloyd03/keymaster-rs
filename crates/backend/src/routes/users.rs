use actix_session::Session;
use actix_web::{delete, error, get, post, web, HttpResponse, Responder};
use log::error;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::User,
    routes::{unpack, validate_admin, validate_session},
};

#[derive(Deserialize)]
struct SetPasswdPayload {
    new_password: String,
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

#[post("/users")]
async fn create(
    session: Session,
    user: web::Either<web::Json<User>, web::Form<User>>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let user = unpack(user);
    if let Err(e) = user.validate_fields() {
        return Err(error::ErrorBadRequest(e));
    }

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
    body: web::Either<web::Json<User>, web::Form<User>>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let body = unpack(body);
    let username = &username.into_inner();

    let mut user = match User::get(&pool, username).await {
        Ok(k) => k,
        Err(e) => {
            error!("User '{}' not found. {}", username, e);
            return Err(error::ErrorNotFound("User not found"));
        }
    };

    user.display_name = body.display_name;
    user.email = body.email;
    user.can_login = body.can_login;
    user.admin = body.admin;

    match user.update(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!("Updated user '{}'", user.username))),
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
        Ok(u) => {
            // Check if the user being deleted is an administrator. If so, check that we're
            // not about to delete the last admin in the database.
            if u.admin {
                match User::count_admins(&pool).await {
                    Ok(count) => {
                        if count <= 1 {
                            return Err(error::ErrorBadRequest(
                                "Unable to delete the last admin user",
                            ));
                        }
                    }
                    Err(e) => {
                        error!("Unable to count the number of existing admin users and thus cannot delete the user. {}", e);
                        return Err(error::ErrorInternalServerError("Failed to delete user"));
                    }
                }
            };
            match u.delete(&pool).await {
                Ok(_) => Ok(HttpResponse::Ok().json(format!("Deleted user '{}'", u.username))),
                Err(e) => {
                    error!("Failed to delete user. {}", e);
                    Err(error::ErrorInternalServerError("Failed to delete user"))
                }
            }
        }
        Err(e) => {
            error!("{}", e);
            Err(error::ErrorNotFound("User not found"))
        }
    }
}

#[post("/users/{username}/set_password")]
async fn set_password(
    session: Session,
    username: web::Path<String>,
    payload: web::Json<SetPasswdPayload>,
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
