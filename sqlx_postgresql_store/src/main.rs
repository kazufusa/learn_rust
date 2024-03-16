use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    let user_id = insert_user(&pool, "hoge", "hoge@example.com").await?;
    println!("Inserted user ID: {}", user_id);
    Ok(())
}

async fn insert_user(pool: &sqlx::PgPool, name: &str, email: &str) -> Result<i32, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let user_id: (i32,) =
        sqlx::query_as("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id")
            .bind(name)
            .bind(email)
            .fetch_one(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(user_id.0)
}
