mod assignment;
mod key;
mod user;

pub use assignment::Assignment;
pub use key::Key;
use sqlx::sqlx_macros;
pub use user::User;

#[sqlx_macros::test]
async fn test_connection() -> anyhow::Result<()> {
    use sqlx::{Connection, Row};

    dotenvy::dotenv()?;
    let db_url = match std::env::var(&&"DATABASE_URL") {
        Ok(u) => format!("{}_test", u),
        Err(e) => {
            eprintln!("Failed to get DATABASE_URL variable. {}", e);
            std::process::exit(1);
        }
    };

    let mut db = sqlx::PgConnection::connect(&db_url).await?;

    let value = sqlx::query("select 1 + 1")
        .try_map(|row: sqlx::postgres::PgRow| row.try_get::<i32, _>(0))
        .fetch_one(&mut db)
        .await?;

    assert_eq!(2i32, value);

    Ok(())
}
