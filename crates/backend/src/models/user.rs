use anyhow::Result;

use sqlx::{
    database::{HasArguments, HasValueRef},
    encode::IsNull,
    error::BoxDynError,
    postgres::{
        types::{PgRecordDecoder, PgRecordEncoder},
        PgQueryResult, PgTypeInfo,
    },
    query, query_as, sqlx_macros, Encode, FromRow, PgPool, Postgres, Type,
};

#[derive(Debug, Default, PartialEq, Clone, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    password_hash: String,
    pub can_login: bool,
}

impl Type<Postgres> for User {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("user")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for User {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let mut decoder = PgRecordDecoder::new(value)?;

        let id = decoder.try_decode::<u32>()?;
        let username = decoder.try_decode::<String>()?;
        let display_name = decoder.try_decode::<String>()?;
        let email = decoder.try_decode::<String>()?;
        let password_hash = decoder.try_decode::<String>()?;
        let can_login = decoder.try_decode::<bool>()?;

        Ok(Self {
            id,
            username,
            display_name,
            email,
            password_hash,
            can_login,
        })
    }
}

impl<'q> Encode<'q, Postgres> for User {
    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        let mut encoder = PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.username);
        encoder.encode(&self.display_name);
        encoder.encode(&self.email);
        encoder.encode(&self.password_hash);
        encoder.encode(&self.can_login);
        encoder.finish();

        IsNull::No
    }
}

impl User {
    pub fn new(username: &str, email: &str) -> Self {
        User {
            username: username.to_string(),
            email: email.to_string(),
            ..Default::default()
        }
    }

    pub async fn get(pool: &PgPool, username: &str) -> Result<Self, sqlx::Error> {
        query_as("SELECT id, username, display_name, email, password_hash, can_login FROM users WHERE id = $1")
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
        let q = "UPDATE keys SET display_name = $1, email = $2, can_login = $3 WHERE username = $4";

        query(q)
            .bind(&self.display_name)
            .bind(&self.email)
            .bind(&self.can_login)
            .bind(&self.username)
            .execute(pool)
            .await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "DELETE FROM keys WHERE username = $1";

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
    let username = "user1";
    let user_display_name = "User 1";
    let user_email = "this@that.com";
    let mut user1 = User::new(username, user_email);
    user1.display_name = user_display_name.to_string();
    user1.create(&pool).await?;

    // Test get
    let user = User::get(&pool, username).await?;

    // assert_eq!(username.to_string(), user.username);
    // assert_eq!(user_email, user.email);

    // // Test update
    // let new_display_name = "User Juan";
    // let mut user = User::get(&pool, username).await?;
    // user.display_name = new_display_name.to_string();
    // user.update(&pool).await?;

    // let updated_user = User::get(&pool, username).await?;

    // assert_eq!(new_display_name, updated_user.display_name);

    // // Test delete
    // user.delete(&pool).await?;
    // let res = query("SELECT * FROM users WHERE username = $1")
    //     .bind(user.username)
    //     .execute(&pool)
    //     .await?;

    // assert_eq!(res.rows_affected(), 0);

    Ok(())
}
