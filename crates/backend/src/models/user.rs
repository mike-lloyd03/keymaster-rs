use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, sqlx_macros, FromRow, PgPool};

#[derive(Debug, Default, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    password_hash: String,
    pub can_login: bool,
}

impl User {
    pub fn new(username: &str, email: &str) -> Self {
        User {
            username: username.to_string(),
            email: email.to_string(),
            ..Default::default()
        }
    }

    pub async fn set_password(
        &mut self,
        pool: &PgPool,
        password_hash: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        self.password_hash = password_hash.to_string();
        query("Update users SET password_hash = $1 WHERE id = $1")
            .bind(&self.id)
            .execute(pool)
            .await
    }

    pub async fn get(pool: &PgPool, username: &str) -> Result<Self, sqlx::Error> {
        query_as("SELECT id, username, display_name, email, password_hash, can_login  FROM users WHERE id = $1")
            .bind(username)
            .fetch_one(pool)
            .await
    }

    pub async fn create(&self, pool: &PgPool) -> anyhow::Result<PgQueryResult, sqlx::Error> {
        let q = "INSERT INTO users (id, username, display_name, email, password_hash, can_login) VALUES ($1, $2, $3, $4, $5, $6)";

        query(q)
            .bind(&self.id)
            .bind(&self.username)
            .bind(&self.display_name)
            .bind(&self.email)
            .bind(&self.password_hash)
            .bind(&self.can_login)
            .execute(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool) -> anyhow::Result<PgQueryResult, sqlx::Error> {
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

#[sqlx_macros::test]
async fn test_key() -> Result<()> {
    use sqlx::migrate::MigrateDatabase;

    // Setup database
    let database_url = "postgres://postgres:postgres@localhost/keymaster_test";

    if sqlx::Postgres::database_exists(database_url).await? {
        sqlx::Postgres::drop_database(database_url).await?;
    }
    sqlx::Postgres::create_database(database_url).await?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let migrator = sqlx::migrate!();
    migrator.run(&pool).await?;

    // Test create
    let key_name = "k1";
    let key_description = "this is a key, the first of many";
    let mut k1 = Key::new(key_name);
    k1.description = key_description.to_string();
    k1.create(&pool).await?;

    // Test get
    let key = Key::get(&pool, key_name).await?;

    assert_eq!(key_name.to_string(), key.name);
    assert_eq!(key_description, key.description);

    // Test update
    let new_desc = "it does stuff";
    let mut key = Key::get(&pool, key_name).await?;
    key.description = new_desc.to_string();
    key.update(&pool).await?;

    let updated_key = Key::get(&pool, key_name).await?;

    assert_eq!(new_desc, updated_key.description);

    // Test delete
    key.delete(&pool).await?;
    let res = query("SELECT * FROM keys WHERE name = $1")
        .bind(key.name)
        .execute(&pool)
        .await?;

    assert_eq!(res.rows_affected(), 0);

    Ok(())
}
