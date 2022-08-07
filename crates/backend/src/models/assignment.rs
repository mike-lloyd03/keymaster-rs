use crate::models::{Key, User};
use anyhow::Result;

use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};

#[derive(Debug, PartialEq, FromRow, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(skip_deserializing)]
    id: i64,
    pub user: String,
    pub key: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date_out: time::Date,
    #[serde(deserialize_with = "deserialize_date_option", default)]
    pub date_in: Option<time::Date>,
}

pub fn deserialize_date_option<'de, D>(deserializer: D) -> Result<Option<time::Date>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format =
        time::format_description::parse("[year]-[month]-[day]").expect("Improper date format");
    time::Date::parse(&s, &format)
        .map(Some)
        .map_err(serde::de::Error::custom)
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<time::Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format =
        time::format_description::parse("[year]-[month]-[day]").expect("Improper date format");
    time::Date::parse(&s, &format).map_err(serde::de::Error::custom)
}

impl Assignment {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub async fn assign_key(
        pool: &PgPool,
        user: &User,
        key: &Key,
        date_out: time::Date,
    ) -> Result<Self, sqlx::Error> {
        query!(
            r#"INSERT INTO assignments ("user", key, date_out) VALUES ($1, $2, $3)"#,
            user.username,
            key.name,
            date_out,
        )
        .execute(pool)
        .await?;

        Self::get_by_user_key(pool, user, key).await
    }

    pub async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            r#"INSERT INTO assignments ("user", key, date_out) VALUES ($1, $2, $3)"#,
            &self.user,
            &self.key,
            &self.date_out,
        )
        .execute(pool)
        .await
    }

    pub async fn get_by_user_key(
        pool: &PgPool,
        user: &User,
        key: &Key,
    ) -> Result<Self, sqlx::Error> {
        query_as!(
            Self,
            r#"SELECT
                id,
                "user",
                key,
                date_out,
                date_in as "date_in?"
            FROM assignments
            WHERE "user" = $1 AND key = $2"#,
            user.username,
            key.name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, sqlx::Error> {
        query_as!(
            Self,
            r#"SELECT
                id,
                "user",
                key,
                date_out,
                date_in as "date_in?"
            FROM assignments
            WHERE id = $1"#,
            id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Self,
            r#"SELECT
                id,
                "user",
                key,
                date_out,
                date_in as "date_in?"
            FROM assignments"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(&mut self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        if self.id == 0 {
            return Err(sqlx::Error::RowNotFound);
        };

        query!(
            r#"UPDATE assignments SET "user" = $1, key = $2, date_out = $3, date_in = $4 WHERE id = $5"#,
            self.user,
            self.key,
            self.date_out,
            self.date_in,
            self.id,
        )
        .execute(pool)
        .await
    }

    pub async fn check_in(
        &mut self,
        pool: &PgPool,
        date: time::Date,
    ) -> Result<PgQueryResult, sqlx::Error> {
        self.date_in = Some(date);

        query!(
            r#"UPDATE assignments SET date_in = $1 WHERE "user" = $2 AND key = $3"#,
            date,
            self.user,
            self.key,
        )
        .execute(pool)
        .await
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            r#"DELETE FROM assignments WHERE "user" = $1 AND key = $2"#,
            self.user,
            self.key
        )
        .execute(pool)
        .await
    }
}

#[sqlx::test(fixtures("users", "keys"))]
async fn create_assignment(pool: PgPool) -> Result<()> {
    let user1 = User::get(&pool, "user1").await?;
    let key1 = Key::get(&pool, "key1").await?;

    let date_out = time::Date::from_calendar_date(1988, time::Month::October, 3)?;
    Assignment::assign_key(&pool, &user1, &key1, date_out).await?;

    Ok(())
}

#[sqlx::test(fixtures("users", "keys", "assignments"))]
async fn get_assignment(pool: PgPool) -> Result<()> {
    let user1 = User::get(&pool, "user1").await?;
    let key1 = Key::get(&pool, "key1").await?;
    let date_out = time::Date::from_calendar_date(1988, time::Month::October, 3)?;

    let assgn1 = Assignment::get_by_user_key(&pool, &user1, &key1).await?;

    assert_eq!("user1", assgn1.user);
    assert_eq!("key1", assgn1.key);
    assert_eq!(date_out, assgn1.date_out);
    assert_eq!(None, assgn1.date_in);

    Ok(())
}

#[sqlx::test(fixtures("users", "keys", "assignments"))]
async fn check_in_assignment(pool: PgPool) -> Result<()> {
    let user1 = User::get(&pool, "user1").await?;
    let key1 = Key::get(&pool, "key1").await?;
    let date_in = time::Date::from_calendar_date(1988, time::Month::November, 3)?;

    let mut assgn1 = Assignment::get_by_user_key(&pool, &user1, &key1).await?;
    assgn1.check_in(&pool, date_in).await?;
    let assgn2 = Assignment::get_by_user_key(&pool, &user1, &key1).await?;

    assert_eq!(date_in, assgn2.date_in.unwrap());

    Ok(())
}

#[sqlx::test(fixtures("users", "keys", "assignments"))]
async fn delete_assignment(pool: PgPool) -> Result<()> {
    let user1 = User::get(&pool, "user1").await?;
    let key1 = Key::get(&pool, "key1").await?;
    let assgn1 = Assignment::get_by_user_key(&pool, &user1, &key1).await?;
    assgn1.delete(&pool).await?;

    let res = query("SELECT * FROM assignments WHERE id = $1")
        .bind(assgn1.id)
        .execute(&pool)
        .await?;

    assert_eq!(res.rows_affected(), 0);

    Ok(())
}
