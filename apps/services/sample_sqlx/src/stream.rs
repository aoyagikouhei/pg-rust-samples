use chrono::prelude::*;
use futures_util::TryStreamExt;
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
    let mut stream = sqlx::query_as::<_, User>("SELECT * FROM sample_get_list_users(p_uuid := $1)")
        .bind(uuid)
        .fetch(pool);

    while let Some(row) = stream.try_next().await? {
        println!("{:?}", row);
    }
    Ok(())
}
