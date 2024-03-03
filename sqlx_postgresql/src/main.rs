use chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::env;
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct User {
    id: Uuid,
    name: String,
    created_at: NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Successfully connected to the database.");
    let users = sqlx::query_as::<_, User>("SELECT id, name, created_at FROM users")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
