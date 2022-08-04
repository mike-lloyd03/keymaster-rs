use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, sqlx_macros, FromRow, PgPool};
#[derive(Debug, Default, PartialEq, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub email: String,
    password_hash: String,
    pub can_login: bool,
}

impl User {
    pub fn new(username: &str) -> Self {
        User {
            username: username.to_string(),
            ..Default::default()
        }
    }

    pub async fn get(pool: &PgPool, username: &str) -> Result<Self, sqlx::Error> {
        query_as("SELECT id, username, display_name, email, password_hash, can_login FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
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

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "INSERT INTO users (username, display_name, email, password_hash, can_login) VALUES ($1, $2, $3, $4, $5)";

        query(q)
            .bind(&self.username)
            .bind(&self.display_name)
            .bind(&self.email)
            .bind(&self.password_hash)
            .bind(&self.can_login)
            .execute(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q =
            "UPDATE users SET display_name = $1, email = $2, can_login = $3 WHERE username = $4";

        query(q)
            .bind(&self.display_name)
            .bind(&self.email)
            .bind(&self.can_login)
            .bind(&self.username)
            .execute(pool)
            .await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "DELETE FROM users WHERE username = $1";

        query(q).bind(&self.username).execute(pool).await
    }
}

#[sqlx_macros::test]
async fn test_user() -> Result<()> {
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
    let username = "user2";
    let user_display_name = "User 2";
    let user_email = "this2@that.com";
    let mut user2 = User::new(username);
    user2.email = user_email.to_string();
    user2.display_name = user_display_name.to_string();
    user2.create(&pool).await?;

    // Test get
    let user = User::get(&pool, username).await?;

    assert_eq!(username.to_string(), user.username);
    assert_eq!(user_email, user.email);

    // Test update
    let new_display_name = "User Too";
    let mut user = User::get(&pool, username).await?;
    user.display_name = new_display_name.to_string();
    user.update(&pool).await?;

    let updated_user = User::get(&pool, username).await?;

    assert_eq!(new_display_name, updated_user.display_name);

    // Test delete
    user.delete(&pool).await?;
    let res = query("SELECT * FROM users WHERE username = $1")
        .bind(user.username)
        .execute(&pool)
        .await?;

    assert_eq!(res.rows_affected(), 0);

    Ok(())
}
