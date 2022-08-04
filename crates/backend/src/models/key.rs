use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, sqlx_macros, FromRow, PgPool};

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
