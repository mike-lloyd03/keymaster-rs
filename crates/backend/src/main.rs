use std::env;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    middleware::NormalizePath,
    web::{scope, Data},
    App, HttpServer,
};
use actix_web_lab::web::spa;
use env_logger::Env;

mod models;
mod routes;

static PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenvy::dotenv().ok();

    let secret_key = get_secret_key("KEYMASTER_SECRET_KEY");

    let pool = match models::db().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    models::initialize_admin(&pool).await.unwrap();
    log::info!("Listening on port {}", PORT);

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
                    .service(routes::keys::get_assignments)
                    .service(routes::users::get)
                    .service(routes::users::get_all)
                    .service(routes::users::update)
                    .service(routes::users::create)
                    .service(routes::users::delete)
                    .service(routes::users::set_password)
                    .service(routes::users::get_assignments)
                    .service(routes::assignments::get)
                    .service(routes::assignments::get_all)
                    .service(routes::assignments::update)
                    .service(routes::assignments::create)
                    .service(routes::assignments::delete)
                    .service(routes::login)
                    .service(routes::logout)
                    .service(routes::session_info),
            )
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

/// Generates a secret key from a secret string. Secret string is either gathered from the
/// environment from the given environment variable or randomly generated.
fn get_secret_key(variable_name: &str) -> Key {
    match env::var(variable_name) {
        Ok(s) => {
            if s.len() < 64 {
                log::error!("Key length must be at least 64 bytes.");
                std::process::exit(1);
            }
            log::info!("Generating secret key from environment variable");
            Key::from(s.as_bytes())
        }
        Err(_) => {
            log::info!("Generating random secret key");
            Key::generate()
        }
    }
}
