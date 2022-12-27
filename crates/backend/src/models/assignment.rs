use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool, Postgres, QueryBuilder};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
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

#[derive(Default, Deserialize, Clone)]
pub struct AssignmentQuery {
    pub id: Option<i64>,
    pub user: Option<String>,
    pub key: Option<String>,
    pub sort: Option<String>,
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

    pub async fn get_all(pool: &PgPool, filter: AssignmentQuery) -> Result<Vec<Self>, sqlx::Error> {
        let valid_columns = vec!["id", "user", "key", "date_out", "date_in"];

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"SELECT
                id,
                "user",
                key,
                date_out,
                date_in
            FROM assignments
            "#,
        );

        let mut where_clause = false;
        if let Some(u) = filter.user {
            if where_clause {
                query.push("AND ");
            } else {
                where_clause = true;
                query.push("WHERE ");
            }
            query.push(r#""user" ="#).push_bind(u);
        }

        if let Some(k) = filter.key {
            if where_clause {
                query.push("AND ");
            } else {
                where_clause = true;
                query.push("WHERE ");
            }
            query.push("key =").push_bind(k);
        }

        if let Some(id) = filter.id {
            if where_clause {
                query.push("AND ");
            } else {
                query.push("WHERE ");
            }
            query.push("id =").push_bind(id);
        }

        if let Some(s) = filter.sort {
            if valid_columns.iter().any(|c| *c == s.as_str()) {
                query.push(format!(r#"ORDER BY "{}""#, s));
            }
        };

        log::warn!("{}", query.sql());
        query.build_query_as::<Assignment>().fetch_all(pool).await
        // log::warn!("{}", query.into_sql());
        // Ok(vec![Assignment {
        //     id: 12,
        //     user: "".into(),
        //     key: "".into(),
        //     date_out: chrono::NaiveDate::default(),
        //     date_in: None,
        // }])
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

    pub async fn get_assignments_by_user(
        pool: &PgPool,
        username: &str,
    ) -> Result<Vec<Assignment>, sqlx::Error> {
        query_as!(
            Assignment,
            r#"SELECT
                *
                FROM assignments
                WHERE "user" = $1
                AND date_in is null
                ORDER BY key"#,
            username
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_assignments_by_key(
        pool: &PgPool,
        key_name: &str,
    ) -> Result<Vec<Assignment>, sqlx::Error> {
        query_as!(
            Assignment,
            r#"SELECT
                *
                FROM assignments
                WHERE key = $1
                AND date_in is null
                ORDER BY "user""#,
            key_name
        )
        .fetch_all(pool)
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
            .bind(1)
            .execute(&pool)
            .await?;

        assert_eq!(res.rows_affected(), 0);

        Ok(())
    }
}
