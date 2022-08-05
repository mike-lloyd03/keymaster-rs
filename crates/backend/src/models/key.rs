use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};

#[derive(Debug, Default, PartialEq, Clone, FromRow)]
pub struct Key {
    pub name: String,
    pub description: String,
    pub active: bool,
}

impl Key {
    pub fn new(name: &str) -> Self {
        Key {
            name: name.to_string(),
            active: true,
            ..Default::default()
        }
    }

    pub async fn get(pool: &PgPool, name: &str) -> Result<Self, sqlx::Error> {
        query_as("SELECT name, description, active FROM keys WHERE name = $1")
            .bind(name)
            .fetch_one(pool)
            .await
    }

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "INSERT INTO keys (name, description, active) VALUES ($1, $2, $3)";

        query(q)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.active)
            .execute(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "UPDATE keys SET description = $1 WHERE name = $2";

        query(q)
            .bind(&self.description)
            .bind(&self.name)
            .execute(pool)
            .await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "DELETE FROM keys WHERE name = $1";

        query(q).bind(&self.name).execute(pool).await
    }
}

#[sqlx::test()]
async fn create_key(pool: PgPool) -> Result<()> {
    let key_name = "k1";
    let key_description = "this is a key, the first of many";
    let mut k1 = Key::new(key_name);
    k1.description = key_description.to_string();
    k1.create(&pool).await?;

    Ok(())
}

#[sqlx::test(fixtures("keys"))]
async fn get_key(pool: PgPool) -> Result<()> {
    let key = Key::get(&pool, "key1").await?;

    assert_eq!("key1", key.name);
    assert_eq!("this is a key", key.description);
    assert!(key.active);

    Ok(())
}

#[sqlx::test(fixtures("keys"))]
async fn update_key(pool: PgPool) -> Result<()> {
    let new_desc = "it does stuff";
    let mut key = Key::get(&pool, "key1").await?;
    key.description = new_desc.to_string();
    key.update(&pool).await?;

    let updated_key = Key::get(&pool, "key1").await?;

    assert_eq!(new_desc, updated_key.description);

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
