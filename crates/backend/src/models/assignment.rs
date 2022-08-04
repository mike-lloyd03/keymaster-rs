use crate::{Key, User};
use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, sqlx_macros, types::time, FromRow, PgPool};

#[derive(Debug, Default, PartialEq, FromRow)]
pub struct Assignment {
    id: i64,
    pub user: String,
    pub key: String,
    pub date_out: Option<time::Date>,
    pub date_in: Option<time::Date>,
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
    user1.create(&pool).await?;
    let key1 = Key::new("k1");
    key1.create(&pool).await?;

    let date_out = time::Date::try_from_ymd(1988, 10, 3)?;
    let assgn1 = Assignment::new(&pool, &user1, &key1, date_out).await?;

    // Test get
    let mut assgn2 = Assignment::get_by_user_key(&pool, &user1, &key1).await?;

    assert_eq!(assgn1, assgn2);

    // Test check_in
    let date_in = time::Date::try_from_ymd(1988, 11, 3)?;
    assgn2.check_in(&pool, date_in).await?;

    let assgn3 = Assignment::get_by_user_key(&pool, &user1, &key1).await?;

    match assgn3.date_in {
        Some(d) => assert_eq!(d, date_in),
        None => todo!(),
    }

    // Test delete
    assgn3.delete(&pool).await?;
    let res = query("SELECT * FROM assignments WHERE id = $1")
        .bind(assgn3.id)
        .execute(&pool)
        .await?;

    assert_eq!(res.rows_affected(), 0);

    Ok(())
}
