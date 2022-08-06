use actix_web::{web::Data, App, HttpServer};

mod models;
mod routes;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = match models::db().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .service(routes::get_keys)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
