use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool};

use crate::models::User;

#[derive(Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub username: String, // Foreign key to User::username
    pub token: String,
    pub created: chrono::NaiveDateTime,
}

impl Session {
    pub async fn login(
        pool: &PgPool,
        username: &str,
        password_hash: &str,
    ) -> Result<Self, sqlx::Error> {
        let user = User::get(pool, username).await?;
        if user
            .validate_password(password_hash)
            .expect("Failed to validate password.")
        {
            let session = Self {
                username: username.to_string(),
                token: "thisisatoken".to_string(),
                created: Utc::now().naive_utc(),
            };
            query!(
                r#"INSERT INTO sessions (username, token, created) VALUES ($1, $2, $3)"#,
                session.username,
                session.token,
                session.created
            )
            .execute(pool)
            .await?;
            return Ok(session);
        }
        todo!();
    }
}
