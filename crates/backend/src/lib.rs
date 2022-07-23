use sqlx::{postgres::PgPoolOptions, query_as, sqlx_macros};

mod models;
use models::Key;

#[sqlx_macros::test]
async fn add_key() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/keymaster")
        .await
        .unwrap();

    let key_name = "k1";
    let key_description = "this is a key, this first of many";
    let mut k1 = Key::new(key_name);
    k1.description = key_description.to_string();
    k1.create(&pool).await?;

    let key: Key = query_as("SELECT * FROM keys where name = $1")
        .bind(key_name)
        .fetch_one(&pool)
        .await?;

    assert_eq!(key_name.to_string(), key.name);
    assert_eq!(key_description, key.description);

    Ok(())
}

// #[sqlx_macros::test]
// async fn update_key() -> anyhow::Result<()> {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:postgres@localhost/keymaster")
//         .await
//         .unwrap();

//     let k1 = models::key::Key::new("k1");
//     k1.create(&pool).await;
//     Ok(())
// }
