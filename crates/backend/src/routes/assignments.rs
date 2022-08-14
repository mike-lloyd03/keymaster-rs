use actix_session::Session;
use actix_web::{
    delete,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    get, post, web, HttpResponse, Responder,
};
use chrono::NaiveDate;
use log::error;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::Assignment,
    routes::{validate_admin, validate_session},
};

#[derive(Deserialize, Debug)]
struct UpdateQuery {
    user: Option<String>,
    key: Option<String>,
    date_out: Option<NaiveDate>,
    date_in: Option<NaiveDate>,
}

#[get("/assignments")]
async fn get_all(
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    match Assignment::get_all(&pool).await {
        Ok(a) => Ok(HttpResponse::Ok().json(a)),
        Err(e) => {
            error!("Failed to get assignments. {}", e);
            Err(ErrorInternalServerError("Failed to get assignments."))
        }
    }
}

#[get("/assignments/{assignment_id}")]
async fn get(
    assignment_id: web::Path<i64>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    let assignment_id = assignment_id.into_inner();

    match Assignment::get(&pool, assignment_id).await {
        Ok(a) => Ok(HttpResponse::Ok().json(a)),
        Err(e) => match e.to_string() {
            x if x.contains("no rows returned") => Err(ErrorNotFound("Assignment not found.")),
            _ => {
                error!("Failed to get assignment '{}'. {}", assignment_id, e);
                Err(ErrorInternalServerError("Failed to get assignment."))
            }
        },
    }
}

#[post("/assignments")]
async fn create(
    assignment: web::Json<Assignment>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    match assignment.create(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!(
            "Key '{}' assigned to '{}'",
            assignment.key, assignment.user
        ))),
        Err(e) => match e.to_string() {
            x if x.contains("duplicate key") => Err(ErrorBadRequest(format!(
                "Key '{}' already assigned to {}",
                assignment.key, assignment.user
            ))),
            x if x.contains("violates foreign key") => match x {
                y if y.contains("assignments_key_fkey") => Err(ErrorBadRequest(format!(
                    "Key '{}' does not exist.",
                    assignment.key
                ))),
                y if y.contains("assignments_user_fkey") => Err(ErrorBadRequest(format!(
                    "User '{}' does not exist.",
                    assignment.user
                ))),
                _ => {
                    error!("Foreign key error. {}", e);
                    Err(ErrorInternalServerError("Failed to create assignment."))
                }
            },
            _ => {
                error!("Failed to create assignment. {}", e);
                Err(ErrorInternalServerError("Failed to create assignment."))
            }
        },
    }
}

#[post("/assignments/{assignment_id}")]
async fn update(
    assignment_id: web::Path<i64>,
    query: web::Json<UpdateQuery>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let assignment_id = assignment_id.into_inner();

    let mut assignment = match Assignment::get(&pool, assignment_id).await {
        Ok(k) => k,
        Err(e) => {
            error!("Assignment '{}' not found. {}", assignment_id, e);
            return Err(ErrorNotFound("Assignment not found."));
        }
    };

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
        Ok(_) => Ok(HttpResponse::Ok().json(assignment)),
        Err(e) => {
            error!("Failed to update assignment. {}", e);
            Err(ErrorInternalServerError("Failed to update assignment."))
        }
    }
}

#[delete("/assignments/{assignment_id}")]
async fn delete(
    assignment_id: web::Path<i64>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    match Assignment::get(&pool, assignment_id.into_inner()).await {
        Ok(a) => match a.delete(&pool).await {
            Ok(_) => Ok(HttpResponse::Ok().json(format!("Deleted assignment '{}'", a.id()))),
            Err(e) => {
                error!("Failed to delete assignment. {}", e);
                Err(ErrorInternalServerError("Failed to delete assignment."))
            }
        },
        Err(_) => Err(ErrorNotFound("Assignment not found.")),
    }
}
