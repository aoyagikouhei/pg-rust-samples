use postgresql::table::{companies::Companies, users::Users};
use sql_query_builder as sql;
use sqlx::postgres::PgPoolOptions;
pub mod sql_builder_helper;
use uuid::Uuid;

use sql_builder_helper::{Operator, ASTERISK, like_pattern, WhereMaker};

async fn get_company_by_name(pool: &sqlx::PgPool, name: &str) -> anyhow::Result<Companies> {
    let mut where_maker = WhereMaker::new();
    let query = sql::Select::new()
        .raw(Companies::SELECT_SQL)
        .where_clause(&where_maker.where_clause(&Operator::Like, Companies::FIELDS.company_name));
    let company: Companies = sqlx::query_as(&query.as_string())
        .bind(like_pattern(name))
        .fetch_one(pool)
        .await?;
    Ok(company)
}

async fn get_user_by_name(pool: &sqlx::PgPool, company_uuid: &Uuid, name: &str) -> anyhow::Result<Users> {
    let mut where_maker = WhereMaker::new();
    let query = sql::Select::new()
        .select(ASTERISK)
        .from(Users::TABLE_NAME)
        .where_clause(&where_maker.where_clause(&Operator::Equal, Users::FIELDS.company_uuid))
        .where_clause(&where_maker.where_clause(&Operator::Like, Users::FIELDS.user_name));

    let user: Users = sqlx::query_as(&query.as_string())
        .bind(company_uuid)
        .bind(like_pattern(name))
        .fetch_one(pool)
        .await?;
    Ok(user)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web")
        .await?;

    let company = get_company_by_name(&pool, "Sample").await?;
    let user = get_user_by_name(&pool, &company.uuid, "Sample").await?;

    println!("{:?}", user);
    Ok(())
}
