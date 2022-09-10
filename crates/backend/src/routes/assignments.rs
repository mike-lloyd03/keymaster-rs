use actix_session::Session;
use actix_web::{
    delete,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    get, post, web, HttpResponse, Responder,
};
use log::{error, info};
use sqlx::PgPool;
use std::fmt::Write;

use crate::{
    models::{Assignment, AssignmentQuery},
    routes::{unpack, validate_admin, validate_session},
};

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

#[get("/assignments")]
async fn get_all(
    pool: web::Data<PgPool>,
    session: Session,
    query: web::Query<AssignmentQuery>,
) -> Result<impl Responder, actix_web::Error> {
    validate_session(&session)?;

    match Assignment::get_all(&pool, query.into_inner()).await {
        Ok(a) => Ok(HttpResponse::Ok().json(a)),
        Err(e) => {
            error!("Failed to get assignments. {}", e);
            Err(ErrorInternalServerError("Failed to get assignments."))
        }
    }
}

/// Accepts an array of Assignment objects as either a form or json body
#[post("/assignments")]
async fn create(
    assignment: web::Either<web::Json<Vec<Assignment>>, web::Form<Vec<Assignment>>>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    info!("{:?}", assignment);
    validate_admin(&session, &pool).await?;

    let assignment = unpack(assignment);

    let mut error_msg = String::new();

    for a in &assignment {
        match a.create(&pool).await {
            Ok(_) => (),
            Err(e) => match e.to_string() {
                x if x.contains("duplicate key") => {
                    writeln!(error_msg, "Key '{}' already assigned to {}", a.key, a.user).unwrap();
                }
                x if x.contains("violates foreign key") => match x {
                    y if y.contains("assignments_key_fkey") => {
                        writeln!(error_msg, "Key '{}' does not exist", a.key).unwrap();
                    }
                    y if y.contains("assignments_user_fkey") => {
                        writeln!(error_msg, "User '{}' does not exist", a.user).unwrap();
                    }
                    _ => {
                        error!("Foreign key error. {}", e);
                        writeln!(
                            error_msg,
                            "Failed to assign key '{}' to user '{}'",
                            a.key, a.user
                        )
                        .unwrap();
                    }
                },
                _ => {
                    error!("Failed to create assignment. {}", e);
                    writeln!(
                        error_msg,
                        "Failed to assign key '{}' to user '{}'",
                        a.key, a.user
                    )
                    .unwrap();
                }
            },
        }
    }
    if error_msg.is_empty() {
        Ok(HttpResponse::Ok().json(format!("Created {} assignments.", &assignment.len())))
    } else {
        Err(ErrorBadRequest(format!(
            "Error creating assignments: {}",
            error_msg
        )))
    }
}

#[post("/assignments/{assignment_id}")]
async fn update(
    assignment_id: web::Path<i64>,
    body: web::Either<web::Json<Assignment>, web::Form<Assignment>>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    validate_admin(&session, &pool).await?;

    let body = unpack(body);

    let assignment_id = assignment_id.into_inner();

    let mut assignment = match Assignment::get(&pool, assignment_id).await {
        Ok(k) => k,
        Err(e) => {
            error!("Assignment '{}' not found. {}", assignment_id, e);
            return Err(ErrorNotFound("Assignment not found."));
        }
    };
    assignment.user = body.user;
    assignment.key = body.key;
    assignment.date_out = body.date_out;
    assignment.date_in = body.date_in;

    match assignment.update(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!("Updated assignment {}.", assignment.id()))),
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
