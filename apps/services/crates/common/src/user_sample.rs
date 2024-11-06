use chrono::prelude::*;
use sqlx::{prelude::*, Pool, Postgres};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(FromRow, Debug)]
struct User {
    pub uuid: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub data_json: serde_json::Value,
}

pub async fn execute(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let uuid: Uuid = Uuid::now_v7();
    let row: User = sqlx::query_as("SELECT * FROM sample_get_list_users(p_uuid := $1)")
        .bind(&uuid)
        .fetch_one(pool)
        .await?;

    println!("{:?}", row);
    Ok(())
}
