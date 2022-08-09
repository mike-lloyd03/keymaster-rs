use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

pub mod assignments;
pub mod keys;
pub mod users;

use crate::models::{Credentials, User};

#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    session: Session,
    creds: web::Json<Credentials>,
) -> impl Responder {
    let creds = creds.into_inner();

    match User::authenticate(&pool, creds).await {
        Ok(user) => {
            session.insert("username", user.username).unwrap();
            HttpResponse::Ok().json("Success")
        }
        Err(_) => HttpResponse::Unauthorized().json("Authentication failed"),
    }
}

pub fn validate_session(session: &Session) -> Result<String, HttpResponse> {
    let username: Option<String> = session.get("username").unwrap_or(None);

    match username {
        Some(u) => {
            session.renew();
            Ok(u)
        }
        None => Err(HttpResponse::Unauthorized().json("Unauthorized")),
    }
}

pub async fn validate_admin(
    session: &Session,
    pool: &web::Data<PgPool>,
) -> Result<(), HttpResponse> {
    let username = validate_session(session)?;
    let user = User::get(pool, &username)
        .await
        .map_err(actix_web::error::ErrorUnauthorized)?;

    if user.admin {
        Ok(())
    } else {
        Err(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

#[cfg(test)]
mod routes_tests {
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::test as actix_test;
    use actix_web::web::Data;
    use actix_web::{
        cookie::{Key, SameSite},
        App,
    };
    use sqlx::PgPool;

    use crate::routes;

    // use crate::routes;

    // use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    // use actix_web::{
    //     cookie::{Key, SameSite},
    //     test,
    //     // web::Data,
    //     App,
    // };

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
