use std::{env, process::exit};

use anyhow::Result;
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod assignment;
mod key;
mod user;

pub use assignment::Assignment;
pub use key::Key;
pub use user::User;

pub async fn db() -> Result<Pool<Postgres>> {
    dotenv()?;
    let db_url = match env::var("DATABASE_URL") {
        // Ok(u) => format!("{}_test", u),
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to get DATABASE_URL variable. {}", e);
            exit(1);
        }
    };

    // if sqlx::Postgres::database_exists(database_url).await? {
    //     sqlx::Postgres::drop_database(database_url).await?;
    // }
    // sqlx::Postgres::create_database(database_url).await?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let migrator = sqlx::migrate!();
    migrator.run(&pool).await?;

    Ok(pool)
}

mod ymd_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "[year]-[month]-[day]";

    pub fn serialize<S>(date: &time::Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = time::format_description::parse(FORMAT).expect("Improper date format");
        let s = date.format(&format).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<time::Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let format = time::format_description::parse(FORMAT).expect("Improper date format");
        time::Date::parse(&s, &format).map_err(serde::de::Error::custom)
    }
}

pub mod ymd_format_option {
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "[year]-[month]-[day]";

    pub fn serialize<S>(date: &Option<time::Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = time::format_description::parse(FORMAT).expect("Improper date format");
        let s = date
            .unwrap()
            .format(&format)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<time::Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let format = time::format_description::parse(FORMAT).expect("Improper date format");
        time::Date::parse(&s, &format)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

// #[sqlx_macros::test]
// async fn test_connection() -> anyhow::Result<()> {
//     use sqlx::{Connection, Row};

//     dotenvy::dotenv()?;
//     let db_url = match std::env::var(&&"DATABASE_URL") {
//         Ok(u) => format!("{}_test", u),
//         Err(e) => {
//             eprintln!("Failed to get DATABASE_URL variable. {}", e);
//             std::process::exit(1);
//         }
//     };

//     let mut db = sqlx::PgConnection::connect(&db_url).await?;

//     let value = sqlx::query("select 1 + 1")
//         .try_map(|row: sqlx::postgres::PgRow| row.try_get::<i32, _>(0))
//         .fetch_one(&mut db)
//         .await?;

//     assert_eq!(2i32, value);

//     Ok(())
// }
