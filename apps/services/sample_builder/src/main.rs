use sql_query_builder as sql;
use sqlx::prelude::*;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(FromRow, Debug)]
struct User {
    pub uuid: Uuid,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web")
        .await?;

    let select = sql::Select::new()
        .select("uuid, user_name")
        .from("users")
        ;

    let query = select.as_string();

    println!("{query}");

    let row: User = sqlx::query_as(&query)
//        .bind(uuid)
        .fetch_one(&pool)
        .await?;

    println!("{:?}", row);

    //api::execute()?;
    Ok(())
}
