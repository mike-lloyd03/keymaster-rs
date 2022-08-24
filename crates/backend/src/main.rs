use std::env;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    middleware::NormalizePath,
    web::{scope, Data},
    App, HttpServer,
};
use log::info;

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let secret_string = env::var("KEYMASTER_SECRET_KEY");
    let secret_key = match secret_string {
        Ok(s) => {
            info!("Generating secret key from environment variable");
            Key::from(s.as_bytes())
        }
        Err(_) => {
            info!("Generating random secret key");
            Key::generate()
        }
    };

    let pool = match models::db().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    models::initialize_admin(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // TODO: set env specific
                    .cookie_http_only(false)
                    .cookie_same_site(SameSite::Strict)
                    .build(),
            )
            .app_data(Data::new(pool.clone()))
            .service(
                scope("/api")
                    .service(routes::keys::get)
                    .service(routes::keys::get_all)
                    .service(routes::keys::update)
                    .service(routes::keys::create)
                    .service(routes::keys::delete)
                    .service(routes::users::get)
                    .service(routes::users::get_all)
                    .service(routes::users::update)
                    .service(routes::users::create)
                    .service(routes::users::delete)
                    .service(routes::users::set_password)
                    .service(routes::assignments::get)
                    .service(routes::assignments::get_all)
                    .service(routes::assignments::update)
                    .service(routes::assignments::create)
                    .service(routes::assignments::delete)
                    .service(routes::login)
                    .service(routes::logout)
                    .service(routes::session_info),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
