use chrono::prelude::*;
use sqlx::prelude::*;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(FromRow, Debug)]
struct User {
    pub uuid: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web").await?;

    let uuid: Uuid = Uuid::now_v7();
    let row: User = sqlx::query_as("SELECT * FROM sample_get_list_users(p_uuid := $1)")
        .bind(&uuid)
        .fetch_one(&pool).await?;

    println!("{:?}", row);
    Ok(())
}