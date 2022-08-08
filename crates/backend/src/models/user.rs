use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};

#[derive(Debug, Default, PartialEq, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    password_hash: Option<String>,
    #[serde(default = "_default_false")]
    pub can_login: bool,
}

fn _default_false() -> bool {
    false
}

impl User {
    pub async fn get(pool: &PgPool, username: &str) -> Result<Self, sqlx::Error> {
        query_as!(Self, r#"SELECT id, username, display_name, email, password_hash, can_login FROM users WHERE username = $1"#, username)
            .fetch_one(pool)
            .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Self,
            r#"SELECT id, username, display_name, email, password_hash, can_login FROM users"#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn set_password(
        &mut self,
        pool: &PgPool,
        password_hash: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        self.password_hash = Some(password_hash.to_string());

        query!(
            "Update users SET password_hash = $1 WHERE username = $2",
            password_hash,
            self.username
        )
        .execute(pool)
        .await
    }

    pub fn validate_password(&self, password_hash: &str) -> Result<bool> {
        match &self.password_hash {
            Some(h) => Ok(h == password_hash),
            None => Ok(false),
        }
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

#[cfg(test)]
mod user_tests {
    use crate::models::User;
    use anyhow::Result;
    use sqlx::{query, PgPool};

    #[sqlx::test()]
    async fn create_user(pool: PgPool) -> Result<()> {
        let username = "user1";
        let display_name = "User 1";
        let email = "user1@email.com";
        let mut user = User {
            username: username.to_string(),
            display_name: Some(display_name.to_string()),
            email: Some(email.to_string()),
            ..Default::default()
        };
        user.email = Some(email.to_string());
        user.display_name = Some(display_name.to_string());
        user.create(&pool).await?;

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn get_user(pool: PgPool) -> Result<()> {
        let user = User::get(&pool, "user1").await?;

        assert_eq!("user1", user.username);
        assert_eq!("User Juan", user.display_name.unwrap());
        assert_eq!("user@email.com", user.email.unwrap());
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

    #[sqlx::test(fixtures("users"))]
    async fn set_password(pool: PgPool) -> Result<()> {
        let mut user = User::get(&pool, "userNoPass").await?;
        let pw_hash = "9305f590c0cb3fc7b81ecb2a948b759d036fa34dc60d63a2e0b1edcc7caca133";
        user.set_password(&pool, pw_hash).await?;

        user = User::get(&pool, "userNoPass").await?;

        assert_eq!(pw_hash, user.password_hash.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn validate_password(pool: PgPool) -> Result<()> {
        let user = User::get(&pool, "user1").await?;
        let good_hash = "46a9d5bde718bf366178313019f04a753bad00685d38e3ec81c8628f35dfcb1b";
        let bad_hash = "3d665bf9e919bbeba9101557048c61868e90ceabf8a94d30e9e02832acfc831e";

        assert!(user.validate_password(good_hash)?);
        assert!(!user.validate_password(bad_hash)?);
        assert!(!user.validate_password("")?);

        Ok(())
    }
}
