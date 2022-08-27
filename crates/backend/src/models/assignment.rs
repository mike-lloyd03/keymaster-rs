use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(skip_deserializing)]
    id: i64,
    pub user: String, // Foreign key to User::username
    pub key: String,  // Foreign key to Key::name
    // #[serde(with = "ymd_format")]
    pub date_out: NaiveDate,
    // #[serde(with = "ymd_format_option")]
    pub date_in: Option<NaiveDate>,
}

#[derive(PartialEq, Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOption {
    ByUser,
    ByKey,
}

#[derive(Serialize, Deserialize)]
pub struct SortedResponse {
    index: String,
    values: String,
}

impl Assignment {
    pub fn id(&self) -> i64 {
        self.id
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

    pub async fn get(pool: &PgPool, id: i64) -> Result<Self, sqlx::Error> {
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
            FROM assignments
            ORDER BY date_out"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_all_sort(
        pool: &PgPool,
        sort_by: SortOption,
    ) -> Result<Vec<SortedResponse>, sqlx::Error> {
        match sort_by {
            SortOption::ByUser => {
                query_as!(
                    SortedResponse,
                    r#"
                SELECT
                    "user" as "index!",
                    string_agg(key, ', ') as "values!"
                FROM assignments
                GROUP BY "user"
                ORDER BY "user"
                "#
                )
                .fetch_all(pool)
                .await
            }
            SortOption::ByKey => {
                query_as!(
                    SortedResponse,
                    r#"
                SELECT
                    key as "index!",
                    string_agg("user", ', ') as "values!"
                FROM assignments
                GROUP BY key
                ORDER BY key
                "#
                )
                .fetch_all(pool)
                .await
            }
        }
    }

    pub async fn update(&mut self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        if self.id == 0 {
            return Err(sqlx::Error::RowNotFound);
        };

        query!(
            r#"UPDATE assignments
            SET
                "user" = $1,
                key = $2,
                date_out = $3,
                date_in = $4
            WHERE id = $5"#,
            self.user,
            self.key,
            self.date_out,
            self.date_in,
            self.id,
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

#[cfg(test)]
mod assignment_tests {
    use crate::models::{Assignment, Key, User};
    use anyhow::Result;
    use chrono::NaiveDate;
    use sqlx::{query, PgPool};

    #[sqlx::test(fixtures("users", "keys"))]
    async fn create_assignment(pool: PgPool) -> Result<()> {
        let user1 = User::get(&pool, "user1").await?;
        let key1 = Key::get(&pool, "key1").await?;

        let date_out = NaiveDate::from_ymd(1988, 10, 3);
        let a = Assignment {
            id: 0,
            user: user1.username,
            key: key1.name,
            date_out,
            date_in: None,
        };
        a.create(&pool).await?;

        Ok(())
    }

    #[sqlx::test(fixtures("users", "keys", "assignments"))]
    async fn get_assignment(pool: PgPool) -> Result<()> {
        let date_out = NaiveDate::from_ymd(1988, 10, 3);
        let assgn1 = Assignment::get(&pool, 1).await?;

        assert_eq!("user1", assgn1.user);
        assert_eq!("key1", assgn1.key);
        assert_eq!(date_out, assgn1.date_out);
        assert_eq!(None, assgn1.date_in);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "keys", "assignments"))]
    async fn delete_assignment(pool: PgPool) -> Result<()> {
        let assgn1 = Assignment::get(&pool, 1).await?;
        assgn1.delete(&pool).await?;

        let res = query("SELECT * FROM assignments WHERE id = $1")
            .bind(assgn1.id)
            .execute(&pool)
            .await?;

        assert_eq!(res.rows_affected(), 0);

        Ok(())
    }
}
