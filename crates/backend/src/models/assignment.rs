use crate::{Key, User};
use anyhow::Result;

use sqlx::{postgres::PgQueryResult, query, query_as, sqlx_macros, types::time, FromRow, PgPool};

#[derive(Debug, Default, PartialEq, FromRow)]
pub struct Assignment {
    id: u32,
    pub user: User,
    pub key: Key,
    pub date_out: Option<time::Date>,
    pub date_in: Option<time::Date>,
}

impl Assignment {
    pub fn new(user: &User, key: &Key, date_out: time::Date) -> Self {
        Assignment {
            user: user.clone(),
            key: key.clone(),
            date_out: Some(date_out),
            ..Default::default()
        }
    }

    pub async fn get_by_user_key(
        pool: &PgPool,
        user: &User,
        key: &Key,
    ) -> Result<Self, sqlx::Error> {
        query_as(
            "SELECT id, user, key, dat&e_out, date_in FROM assignments WHERE user = $1 AND key = $2",
        )
        .bind(user)
        .bind(key)
        .fetch_one(pool)
        .await
    }

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let q = "INSERT INTO assignments (user, key, date_out) VALUES ($1, $2, $3)";

        query(q)
            .bind(&self.user)
            .bind(&self.key)
            .bind(&self.date_out)
            .execute(pool)
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
    let user1 = User::new("user1", "User1@email.com");
    let key1 = Key::new("k1");
    let date_out = time::Date::try_from_ymd(1988, 10, 3).unwrap();
    let assgn1 = Assignment::new(&user1, &key1, date_out);
    assgn1.create(&pool).await?;

    // Test get
    let assgn2 = Assignment::get_by_user_key(&pool, &user1, &key1)
        .await
        .unwrap();

    assert_eq!(assgn1, assgn2);

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
