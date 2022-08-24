use actix_session::Session;
use actix_web::{
    delete,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    get, post, web, HttpResponse, Responder,
};
use chrono::NaiveDate;
use log::{error, info};
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::Write;

use crate::{
    models::Assignment,
    routes::{unpack, validate_admin, validate_session},
};

#[derive(Deserialize, Clone)]
struct UpdateBody {
    user: Option<String>,
    key: Option<String>,
    date_out: Option<NaiveDate>,
    date_in: Option<NaiveDate>,
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

// #[post("/assignments")]
// async fn create(
//     assignment: web::Either<web::Json<Assignment>, web::Form<Assignment>>,
//     pool: web::Data<PgPool>,
//     session: Session,
// ) -> Result<impl Responder, actix_web::Error> {
//     validate_admin(&session, &pool).await?;

//     let assignment = unpack(assignment);

//     match assignment.create(&pool).await {
//         Ok(_) => Ok(HttpResponse::Ok().json(format!(
//             "Key '{}' assigned to '{}'",
//             assignment.key, assignment.user
//         ))),
//         Err(e) => match e.to_string() {
//             x if x.contains("duplicate key") => Err(ErrorBadRequest(format!(
//                 "Key '{}' already assigned to {}",
//                 assignment.key, assignment.user
//             ))),
//             x if x.contains("violates foreign key") => match x {
//                 y if y.contains("assignments_key_fkey") => Err(ErrorBadRequest(format!(
//                     "Key '{}' does not exist.",
//                     assignment.key
//                 ))),
//                 y if y.contains("assignments_user_fkey") => Err(ErrorBadRequest(format!(
//                     "User '{}' does not exist.",
//                     assignment.user
//                 ))),
//                 _ => {
//                     error!("Foreign key error. {}", e);
//                     Err(ErrorInternalServerError("Failed to create assignment."))
//                 }
//             },
//             _ => {
//                 error!("Failed to create assignment. {}", e);
//                 Err(ErrorInternalServerError("Failed to create assignment."))
//             }
//         },
//     }
// }

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
    body: web::Either<web::Json<UpdateBody>, web::Form<UpdateBody>>,
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

    if let Some(u) = &body.user {
        assignment.user = u.to_string()
    };

    if let Some(k) = &body.key {
        assignment.key = k.to_string()
    };

    if let Some(d) = body.date_out {
        assignment.date_out = d
    };

    if let Some(d) = body.date_in {
        assignment.date_in = Some(d)
    };

    match assignment.update(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(format!("Updated assignment '{}.", assignment.id()))),
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
