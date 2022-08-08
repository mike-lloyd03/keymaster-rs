use actix_web::{middleware::Logger, middleware::NormalizePath, web::Data, App, HttpServer};

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = match models::db().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
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
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
