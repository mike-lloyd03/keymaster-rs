use crate::{Key, User};
use anyhow::Result;

use sqlx::{
    database::HasValueRef,
    error::BoxDynError,
    postgres::{types::PgRecordDecoder, PgQueryResult},
    query, query_as, sqlx_macros,
    types::time,
    FromRow, PgPool, Postgres,
};

#[derive(Debug, Default, PartialEq, FromRow)]
pub struct Assignment {
    id: i64,
    pub user: User,
    pub key: Key,
    pub date_out: Option<time::Date>,
    pub date_in: Option<time::Date>,
}

impl<'r> sqlx::Decode<'r, Postgres> for Assignment {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let mut decoder = PgRecordDecoder::new(value)?;

        let id = decoder.try_decode::<i64>()?;
        let username = decoder.try_decode::<String>()?;
        let keyname = decoder.try_decode::<String>()?;
        let date_out = decoder.try_decode::<Option<time::Date>>()?;
        let date_in = decoder.try_decode::<Option<time::Date>>()?;

        let user = User::new(&username);
        let key = Key::new(&keyname);

        Ok(Self {
            id,
            user,
            key,
            date_out,
            date_in,
        })
    }
}

impl Assignment {
    pub async fn new(
        pool: &PgPool,
        user: &User,
        key: &Key,
        date_out: time::Date,
    ) -> Result<Self, sqlx::Error> {
        let q = "INSERT INTO assignments (\"user\", key, date_out) VALUES ($1, $2, $3)";
        query(q)
            .bind(user.clone().username)
            .bind(key.clone().name)
            .bind(date_out)
            .execute(pool)
            .await?;

        Self::get_by_user_key(pool, user, key).await
    }

    pub async fn get_by_user_key(
        pool: &PgPool,
        user: &User,
        key: &Key,
    ) -> Result<Self, sqlx::Error> {
        query_as(
            "SELECT id, \"user\", key, date_out, date_in FROM assignments WHERE \"user\" = $1 AND key = $2",
        )
        .bind(user.clone().username)
        .bind(key.clone().name)
        .fetch_one(pool)
        .await
    }

    pub async fn check_in(
        &mut self,
        pool: &PgPool,
        date: time::Date,
    ) -> Result<PgQueryResult, sqlx::Error> {
        self.date_in = Some(date);
        let q = "UPDATE assignments SET date_in = $1 WHERE id = $2";

        query(q).bind(Some(date)).bind(&self.id).execute(pool).await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "DELETE FROM assignments WHERE id = $1";

        query(q).bind(&self.id).execute(pool).await
    }
}

#[sqlx_macros::test]
async fn test_assignment() -> Result<()> {
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
    let user1 = User::new("user1");
    user1.create(&pool).await.unwrap();
    let key1 = Key::new("k1");
    key1.create(&pool).await.unwrap();

    let date_out = time::Date::try_from_ymd(1988, 10, 3).unwrap();
    let assgn1 = Assignment::new(&pool, &user1, &key1, date_out)
        .await
        .unwrap();

    // Test get
    // let assgn2 = Assignment::get_by_user_key(&pool, &user1, &key1)
    //     .await
    //     .unwrap();

    // assert_eq!(assgn1, assgn2);

    // // Test check_in
    // let new_display_name = "Assignment Juan";
    // let mut user = Assignment::get(&pool, username).await?;
    // user.display_name = new_display_name.to_string();
    // user.update(&pool).await?;

    // let updated_user = Assignment::get(&pool, username).await?;

    // assert_eq!(new_display_name, updated_user.display_name);

    // Test delete
    // assgn2.delete(&pool).await?;
    // let res = query("SELECT * FROM users WHERE username = $1")
    //     .bind(user.username)
    //     .execute(&pool)
    //     .await?;

    // assert_eq!(res.rows_affected(), 0);

    Ok(())
}
