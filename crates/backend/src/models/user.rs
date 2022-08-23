use std::env;

use anyhow::Result;
use orion::pwhash;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    #[serde(skip)]
    password_hash: Option<String>,
    #[serde(default = "_default_false")]
    pub can_login: bool,
    #[serde(default = "_default_false")]
    pub admin: bool,
}

fn _default_false() -> bool {
    false
}

impl User {
    pub async fn get(pool: &PgPool, username: &str) -> Result<Self, sqlx::Error> {
        query_as!(Self, r#"SELECT id, username, display_name, email, password_hash, can_login, admin FROM users WHERE username = $1"#, username)
            .fetch_one(pool)
            .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Self,
            r#"SELECT id, username, display_name, email, password_hash, can_login, admin FROM users"#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn authenticate(pool: &PgPool, creds: Credentials) -> Result<Self, actix_web::Error> {
        let user = match Self::get(pool, &creds.username).await {
            Ok(u) => u,
            Err(_) => {
                // Attempt to validate the password on a fake account to prevent a timing attack
                User {
                    username: "_".into(),
                    password_hash: Some("$argon2i$v=19$m=65536,t=3,p=1$4MHN0rGSFfQxAfCHfD1Ncg$+psDULFfyWAaQ6H/tI/KH5LMcfZBjlpxOyFXJIa4ezM".into()),
                    ..Default::default()
                }
                .validate_password("hunter2");
                return Err(actix_web::error::ErrorUnauthorized("Authentication failed"));
            }
        };
        if user.can_login && user.validate_password(&creds.password) {
            Ok(user)
        } else {
            Err(actix_web::error::ErrorUnauthorized("Authentication failed"))
        }
    }

    pub async fn set_password(
        &mut self,
        pool: &PgPool,
        password: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let pw = pwhash::Password::from_slice(password.as_bytes()).unwrap();
        let hash = pwhash::hash_password(&pw, 3, 1 << 16).unwrap();
        self.password_hash = Some(hash.unprotected_as_encoded().to_string());

        query!(
            "Update users SET password_hash = $1 WHERE username = $2",
            self.password_hash,
            self.username
        )
        .execute(pool)
        .await
    }

    pub fn validate_password(&self, input_password: &str) -> bool {
        match &self.password_hash {
            Some(h) => {
                let hash = pwhash::PasswordHash::from_encoded(h).unwrap();
                let input_password =
                    pwhash::Password::from_slice(input_password.as_bytes()).unwrap();
                pwhash::hash_password_verify(&hash, &input_password).is_ok()
            }
            None => false,
        }
    }

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            r#"INSERT INTO users (username, display_name, email, password_hash, can_login, admin)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            self.username,
            self.display_name,
            self.email,
            self.password_hash,
            self.can_login,
            self.admin
        )
        .execute(pool)
        .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "UPDATE users SET display_name = $1, email = $2, can_login = $3, admin = $4 WHERE username = $5",
            self.display_name,
            self.email,
            self.can_login,
            self.admin,
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

    pub async fn count_admins(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count =
            sqlx::query_scalar!(r#"SELECT count(*) as "count!" FROM users WHERE admin = 't'"#)
                .fetch_one(pool)
                .await?;
        Ok(count)
    }
}

pub async fn initialize_admin(pool: &PgPool) -> Result<(), sqlx::Error> {
    if User::count_admins(pool).await? >= 1 {
        println!("Admin user exists.");
        return Ok(());
    }

    println!("Creating admin user");
    let mut pw_from_env = false;
    let admin_pass = match env::var("KEYMASTER_ADMIN_PASS") {
        Ok(v) => {
            pw_from_env = true;
            v
        }
        Err(_) => pwgen::generate("full", 14),
    };

    let mut admin = User {
        username: "admin".to_string(),
        display_name: Some("admin".to_string()),
        can_login: true,
        admin: true,
        ..Default::default()
    };
    admin.create(pool).await?;
    admin.set_password(pool, &admin_pass).await?;
    println!(
        "Admin user created. username: 'admin', password: '{}'",
        if pw_from_env {
            "<FROM ENVIRONMENT>"
        } else {
            &admin_pass
        }
    );

    Ok(())
}

#[cfg(test)]
mod user_tests {
    use crate::models::{Credentials, User};
    use anyhow::Result;
    use sqlx::{query, PgPool};

    #[sqlx::test()]
    async fn test_create_user(pool: PgPool) -> Result<()> {
        let username = "user1";
        let display_name = "User 1";
        let email = "user1@email.com";
        let mut user = User {
            username: username.to_string(),
            display_name: Some(display_name.to_string()),
            email: Some(email.to_string()),
            can_login: true,
            admin: true,
            password_hash: Some("123".to_string()),
            ..Default::default()
        };
        user.email = Some(email.to_string());
        user.display_name = Some(display_name.to_string());
        user.create(&pool).await?;
        let got_user = User::get(&pool, &user.username).await?;

        assert_eq!(user.username, got_user.username);
        assert_eq!(user.display_name, got_user.display_name);
        assert_eq!(user.email, got_user.email);
        assert_eq!(user.can_login, got_user.can_login);
        assert_eq!(user.admin, got_user.admin);
        assert_eq!(user.password_hash, got_user.password_hash);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_get_user(pool: PgPool) -> Result<()> {
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
    async fn test_update_user(pool: PgPool) -> Result<()> {
        let new_display_name = "User Too";
        let mut user = User::get(&pool, "user1").await?;
        user.display_name = Some(new_display_name.to_string());
        user.update(&pool).await?;

        let updated_user = User::get(&pool, "user1").await?;

        assert_eq!(new_display_name, updated_user.display_name.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_delete_user(pool: PgPool) -> Result<()> {
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
    async fn test_set_password(pool: PgPool) -> Result<()> {
        let mut user = User::get(&pool, "userNoPass").await?;
        let password = "itsagoodpass2";
        user.set_password(&pool, password).await?;

        user = User::get(&pool, "userNoPass").await?;

        assert!(user.password_hash.is_some());

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_validate_password(pool: PgPool) -> Result<()> {
        let mut user = User::get(&pool, "userNoPass").await?;
        let password = "itsagoodpass2";
        user.set_password(&pool, password).await?;

        user = User::get(&pool, "userNoPass").await?;

        assert!(user.validate_password(password));
        assert!(!user.validate_password("itsabadpass3"));

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_authenticate(pool: PgPool) -> Result<()> {
        let username = "userCanLogin".to_string();
        let password = "abc123".to_string();
        let creds = Credentials {
            username: username.clone(),
            password,
        };
        let auth_user = User::authenticate(&pool, creds).await.unwrap();

        assert_eq!(User::get(&pool, &username).await?, auth_user);

        Ok(())
    }
}
