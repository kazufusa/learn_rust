use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use std::env;
use ulid::Ulid;

#[derive(Debug, FromRow)]
struct User {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    let ulid = Ulid::new();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        ulid.to_string(),
        "new user"
    )
    .execute(&pool)
    .await?;

    let users = sqlx::query_as!(User, "SELECT id, name, created_at FROM users;")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
