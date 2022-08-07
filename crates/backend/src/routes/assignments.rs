use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::{ymd_format_option, Assignment};

#[derive(Deserialize, Debug)]
struct UpdateQuery {
    user: Option<String>,
    key: Option<String>,
    #[serde(with = "ymd_format_option", default)]
    date_out: Option<time::Date>,
    #[serde(with = "ymd_format_option", default)]
    date_in: Option<time::Date>,
}

#[get("/assignments")]
async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match Assignment::get_all(&pool).await {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to get assignments. {}", e))
        }
    }
}

#[get("/assignments/{assignment_id}")]
async fn get(assignment_id: web::Path<i64>, pool: web::Data<PgPool>) -> impl Responder {
    match Assignment::get_by_id(&pool, assignment_id.into_inner()).await {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => {
                HttpResponse::NotFound().json("Assignment not found")
            }
            _ => {
                HttpResponse::InternalServerError().json(format!("Failed to get assignment. {}", e))
            }
        },
    }
}

#[post("/assignments")]
async fn create(assignment: web::Json<Assignment>, pool: web::Data<PgPool>) -> impl Responder {
    match assignment.create(&pool).await {
        Ok(_) => HttpResponse::Ok().json(format!(
            "Key '{}' assigned to '{}'",
            assignment.key, assignment.user
        )),
        Err(e) => match e.to_string() {
            x if x.contains("duplicate key") => HttpResponse::BadRequest().json(format!(
                "Key '{}' already assigned to {}",
                assignment.key, assignment.user
            )),
            x if x.contains("violates foreign key") => match x {
                y if y.contains("assignments_key_fkey") => HttpResponse::BadRequest()
                    .json(format!("Key '{}' does not exist.", assignment.key)),
                y if y.contains("assignments_user_fkey") => HttpResponse::BadRequest()
                    .json(format!("User '{}' does not exist.", assignment.user)),
                _ => HttpResponse::InternalServerError()
                    .json(format!("Failed to create assignment. {}", e)),
            },
            _ => HttpResponse::InternalServerError()
                .json(format!("Failed to create assignment. {}", e)),
        },
    }
}

#[put("/assignments/{assignment_id}")]
async fn update(
    assignment_id: web::Path<i64>,
    query: web::Json<UpdateQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut assignment = match Assignment::get_by_id(&pool, assignment_id.into_inner()).await {
        Ok(k) => k,
        Err(_) => return HttpResponse::NotFound().json("Assignment not found"),
    };
    println!("{:?}", query);

    if let Some(u) = &query.user {
        assignment.user = u.to_string()
    };

    if let Some(k) = &query.key {
        assignment.key = k.to_string()
    };

    if let Some(d) = query.date_out {
        assignment.date_out = d
    };

    if let Some(d) = query.date_in {
        assignment.date_in = Some(d)
    };

    match assignment.update(&pool).await {
        Ok(_) => HttpResponse::Ok().json(assignment),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to update assignment. {}", e))
        }
    }
}

#[delete("/assignments/{assignment_id}")]
async fn delete(assignment_id: web::Path<i64>, pool: web::Data<PgPool>) -> impl Responder {
    match Assignment::get_by_id(&pool, assignment_id.into_inner()).await {
        Ok(a) => match a.delete(&pool).await {
            Ok(_) => HttpResponse::Ok().json(format!("Deleted assignment '{}'", a.id())),
            Err(e) => HttpResponse::InternalServerError()
                .json(format!("Failed to delete assignment. {}", e)),
        },
        Err(_) => HttpResponse::NotFound().json("Assignment not found"),
    }
}
