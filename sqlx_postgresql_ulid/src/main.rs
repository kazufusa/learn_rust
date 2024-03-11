use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use std::env;
use ulid::Ulid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_id")]
struct UserId(Ulid);
impl From<String> for UserId {
    fn from(s: String) -> Self {
        UserId(Ulid::from_string(&s).expect("Failed to parse ULID"))
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, FromRow)]
struct User {
    id: UserId,
    name: String,
    created_at: NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    let user_id = UserId(Ulid::new());

    // Since we cannot use sqlx::query! macro directly with cusome domain type
    // like "user_id", we opt for sqlx::query combined with manual parameter binding.
    sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
        .bind(user_id.to_string())
        .bind("new user")
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
