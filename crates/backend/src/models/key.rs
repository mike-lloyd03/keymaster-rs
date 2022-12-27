use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "_default_true")]
    pub active: bool,
}

fn _default_true() -> bool {
    true
}

impl Key {
    pub async fn get(pool: &PgPool, name: &str) -> Result<Self, sqlx::Error> {
        query_as!(
            Self,
            "SELECT name, description, active FROM keys WHERE name = $1",
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Self,
            "SELECT name, description, active FROM keys ORDER BY name"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_all_active(pool: &PgPool, active: bool) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Self,
            "SELECT name, description, active FROM keys where active = $1 ORDER BY name",
            active
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "INSERT INTO keys (name, description, active) VALUES ($1, $2, $3)",
            self.name,
            self.description,
            self.active
        )
        .execute(pool)
        .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "UPDATE keys SET description = $1, active = $2 WHERE name = $3",
            self.description,
            self.active,
            self.name
        )
        .execute(pool)
        .await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!("DELETE FROM keys WHERE name = $1", self.name)
            .execute(pool)
            .await
    }
}

#[cfg(test)]
mod key_tests {
    use crate::models::Key;
    use anyhow::Result;
    use sqlx::{query, PgPool};

    #[sqlx::test()]
    async fn create_key(pool: PgPool) -> Result<()> {
        let name = "k1";
        let description = "this is a key, the first of many";
        let k1 = Key {
            name: name.to_string(),
            description: Some(description.to_string()),
            active: true,
        };
        k1.create(&pool).await?;

        Ok(())
    }

    #[sqlx::test(fixtures("keys"))]
    async fn get_key(pool: PgPool) -> Result<()> {
        let key = Key::get(&pool, "key1").await?;

        assert_eq!("key1", key.name);
        assert_eq!(Some("this is a key".into()), key.description);
        assert!(key.active);

        Ok(())
    }

    #[sqlx::test(fixtures("keys"))]
    async fn get_all_active_keys(pool: PgPool) -> Result<()> {
        let active_keys = Key::get_all_active(&pool, true).await?;
        let inactive_keys = Key::get_all_active(&pool, false).await?;

        assert_eq!(2, active_keys.len());
        assert_eq!(1, inactive_keys.len());

        Ok(())
    }

    #[sqlx::test(fixtures("keys"))]
    async fn update_key(pool: PgPool) -> Result<()> {
        let new_desc = "it does stuff";
        let mut key = Key::get(&pool, "key1").await?;
        key.description = Some(new_desc.to_string());
        key.update(&pool).await?;

        let updated_key = Key::get(&pool, "key1").await?;

        assert_eq!(Some(new_desc.into()), updated_key.description);

        Ok(())
    }

    #[sqlx::test(fixtures("keys"))]
    async fn delete_key(pool: PgPool) -> Result<()> {
        let key = Key::get(&pool, "key1").await?;

        key.delete(&pool).await?;
        let res = query("SELECT * FROM keys WHERE name = $1")
            .bind(key.name)
            .execute(&pool)
            .await?;

        assert_eq!(res.rows_affected(), 0);

        Ok(())
    }
}
