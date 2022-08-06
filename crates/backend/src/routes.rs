use actix_web::{get, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::models::Key;

#[get("/keys")]
async fn get_keys(pool: web::Data<PgPool>) -> Result<impl Responder> {
    let keys = match Key::get_all(&pool).await {
        Ok(k) => k,
        Err(_) => todo!(),
    };

    Ok(HttpResponse::Ok().json(keys))
}
