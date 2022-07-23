use anyhow::Result;
use std::fmt;

use sqlx::{query, Decode, FromRow, PgPool};

#[derive(Default, FromRow)]
pub struct Key {
    pub name: String,
    pub description: String,
    status: String,
}

#[derive(Decode, Default)]
pub enum KeyStatus {
    #[default]
    Active,
    Inactive,
}

impl fmt::Display for KeyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyStatus::Active => write!(f, "active"),
            KeyStatus::Inactive => write!(f, "inactive"),
        }
    }
}

impl Key {
    pub fn new(name: &str) -> Self {
        Key {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub async fn create(&self, pool: &PgPool) -> anyhow::Result<()> {
        let q = "INSERT INTO keys (name, description, status) VALUES ($1, $2, $3)";

        query(q)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status.to_string())
            .execute(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool) {
        let q = "UPDATE keys SET description = $1, status = $2 WHERE name = $3";

        query(q)
            .bind(&self.description)
            .bind(&self.status.to_string())
            .bind(&self.name)
            .execute(pool)
            .await
            .unwrap();
    }
}
