use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

pub mod assignments;
pub mod keys;
pub mod users;

#[get("/login")]
async fn login(pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}
