use actix_session::Session;
use actix_web::{
    error::{self, ErrorUnauthorized},
    post, web, Either, HttpResponse, Responder,
};
use sqlx::PgPool;

pub mod assignments;
pub mod keys;
pub mod users;

use crate::models::{Credentials, User};

#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    session: Session,
    creds: web::Either<web::Json<Credentials>, web::Form<Credentials>>,
) -> impl Responder {
    let creds = unpack(creds);

    match User::authenticate(&pool, creds).await {
        Ok(user) => {
            session.insert("username", user.username).unwrap();
            HttpResponse::Ok().body("Success")
        }
        Err(_) => HttpResponse::Unauthorized().json("Authentication failed"),
    }
}

#[post("/logout")]
async fn logout(session: Session) -> Result<impl Responder, actix_web::Error> {
    if validate_session(&session).is_err() {
        return Err(error::ErrorUnauthorized("Unauthorized"));
    };

    session.purge();

    Ok(HttpResponse::Ok().body("Logged out."))
}

pub fn validate_session(session: &Session) -> Result<String, actix_web::Error> {
    let username: Option<String> = session.get("username").unwrap_or(None);

    match username {
        Some(u) => {
            session.renew();
            Ok(u)
        }
        None => Err(ErrorUnauthorized("Unauthorized")),
    }
}

pub async fn validate_admin(
    session: &Session,
    pool: &web::Data<PgPool>,
) -> Result<(), actix_web::Error> {
    let username = validate_session(session)?;
    let user = User::get(pool, &username)
        .await
        .map_err(actix_web::error::ErrorUnauthorized)?;

    if user.admin {
        Ok(())
    } else {
        Err(ErrorUnauthorized("Unauthorized"))
    }
}

/// Unpacks a request with either json or form data into the specified type
pub fn unpack<T: Clone>(e: web::Either<web::Json<T>, web::Form<T>>) -> T {
    match e {
        Either::Left(json) => json.to_owned(),
        Either::Right(form) => form.to_owned(),
    }
}

#[cfg(test)]
mod routes_tests {
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::test as actix_test;
    use actix_web::{
        cookie::{Key, SameSite},
        web::Data,
        App,
    };
    use sqlx::PgPool;

    use crate::routes;

    #[sqlx::test(fixtures("users"))]
    async fn test_login(pool: PgPool) {
        let secret_key = Key::generate();

        let app = actix_test::init_service(
            App::new()
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .cookie_secure(false)
                        .cookie_http_only(false)
                        .cookie_same_site(SameSite::Strict)
                        .build(),
                )
                .app_data(Data::new(pool.clone()))
                .service(routes::login),
        )
        .await;
        // No request body
        let req = actix_test::TestRequest::post().uri("/login").to_request();
        let resp = actix_test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());

        // User not allowed to log in
        let creds = crate::models::Credentials {
            username: "user1".to_string(),
            password: "pass1".to_string(),
        };
        let req = actix_test::TestRequest::post()
            .uri("/login")
            .set_json(creds)
            .to_request();
        let resp = actix_test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());

        // User can log in
        let creds = crate::models::Credentials {
            username: "user2".to_string(),
            password: "pass2".to_string(),
        };
        let req = actix_test::TestRequest::post()
            .uri("/login")
            .set_json(creds)
            .to_request();
        let resp = actix_test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
