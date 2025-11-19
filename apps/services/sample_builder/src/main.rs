use sql_query_builder as sql;
use sqlx::prelude::*;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use postgresql::table::{companies::Companies, users::Users};

#[allow(dead_code)]
#[derive(FromRow, Debug)]
struct Company {
    pub uuid: Uuid,
}

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
        .select(Companies::FIELDS.uuid)
        .from(Companies::TABLE_NAME)
    ;
    let company: Company = sqlx::query_as(&select.as_string())
        .fetch_one(&pool)
        .await?;


    let select = sql::Select::new()
        .select(&[Users::FIELDS.uuid, Users::FIELDS.user_name].join(", "))
        .from(Users::TABLE_NAME)
        .where_clause(&format!("{} = $1", Users::FIELDS.company_uuid))
    ;

    let query = select.as_string();

    let user: User = sqlx::query_as(&query)
        .bind(company.uuid)
        .fetch_one(&pool)
        .await?;

    println!("{:?}", user);

    //api::execute()?;
    Ok(())
}
