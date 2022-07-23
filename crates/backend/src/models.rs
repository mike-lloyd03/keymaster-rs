// use sqlx::postgres::PgPoolOptions;

mod key;
pub use key::Key;

// #[async_std::main]
// // or #[tokio::main]
// // or #[actix_web::main]
// async fn main() -> Result<(), sqlx::Error> {
//     // Create a connection pool
//     //  for MySQL, use MySqlPoolOptions::new()
//     //  for SQLite, use SqlitePoolOptions::new()
//     //  etc.
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:password@localhost/keymaster")
//         .await?;

//     let k1 = key::Key::new("k1");
//     k1.create(&pool).await;

//     Ok(())
// }
