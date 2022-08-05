use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};
#[derive(Debug, Default, PartialEq, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    password_hash: Option<String>,
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
        query_as!(Self, "SELECT id, username, display_name, email, password_hash, can_login FROM users WHERE username = $1", username)
            .fetch_one(pool)
            .await
    }

    pub async fn set_password(
        &mut self,
        pool: &PgPool,
        password_hash: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        self.password_hash = Some(password_hash.to_string());

        query!("Update users SET password_hash = $1 WHERE id = $1", self.id)
            .execute(pool)
            .await
    }

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            r#"INSERT INTO users (username, display_name, email, password_hash, can_login)
            VALUES ($1, $2, $3, $4, $5)"#,
            self.username,
            self.display_name,
            self.email,
            self.password_hash,
            self.can_login
        )
        .execute(pool)
        .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "UPDATE users SET display_name = $1, email = $2, can_login = $3 WHERE username = $4",
            self.display_name,
            self.email,
            self.can_login,
            self.username
        )
        .execute(pool)
        .await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!("DELETE FROM users WHERE username = $1", self.username)
            .execute(pool)
            .await
    }
}

#[sqlx::test()]
async fn create_user(pool: PgPool) -> Result<()> {
    let username = "user1";
    let user_display_name = "User 1";
    let user_email = "user1@email.com";
    let mut user = User::new(username);
    user.email = user_email.to_string();
    user.display_name = Some(user_display_name.to_string());
    user.create(&pool).await?;

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn get_user(pool: PgPool) -> Result<()> {
    let user = User::get(&pool, "user1").await?;

    assert_eq!("user1", user.username);
    assert_eq!("User Juan", user.display_name.unwrap());
    assert_eq!("user@email.com", user.email);
    assert_eq!(
        "46a9d5bde718bf366178313019f04a753bad00685d38e3ec81c8628f35dfcb1b",
        user.password_hash.unwrap()
    );
    assert!(!user.can_login);

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn update_user(pool: PgPool) -> Result<()> {
    let new_display_name = "User Too";
    let mut user = User::get(&pool, "user1").await?;
    user.display_name = Some(new_display_name.to_string());
    user.update(&pool).await?;

    let updated_user = User::get(&pool, "user1").await?;

    assert_eq!(new_display_name, updated_user.display_name.unwrap());

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn delete_user(pool: PgPool) -> Result<()> {
    let user = User::get(&pool, "user1").await?;
    user.delete(&pool).await?;

    let res = query("SELECT * FROM users WHERE username = $1")
        .bind(user.username)
        .execute(&pool)
        .await?;

    assert_eq!(res.rows_affected(), 0);

    Ok(())
}
