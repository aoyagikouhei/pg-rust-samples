use postgresql::table::{companies::Companies, users::Users};
use sql_query_builder as sql;
use sqlx::postgres::PgPoolOptions;

fn make_where_clause(field: &str, operator: &str, index: usize) -> String {
    format!("{} {} ${}", field, operator, index)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web")
        .await?;

    let query = sql::Select::new()
        .raw(Companies::SELECT_SQL)
        .where_clause(&make_where_clause(
            Companies::FIELDS.company_name,
            "LIKE",
            1,
        ));
    let company: Companies = sqlx::query_as(&query.as_string())
        .bind("%Sample%")
        .fetch_one(&pool)
        .await?;

    let query = sql::Select::new()
        .select("*")
        .from(Users::TABLE_NAME)
        .where_clause(&make_where_clause(Users::FIELDS.company_uuid, "=", 1))
        .where_clause(&make_where_clause(Users::FIELDS.user_name, "LIKE", 2));

    println!("SQL: {}", query.as_string());

    let user: Users = sqlx::query_as(&query.as_string())
        .bind(company.uuid)
        .bind("%Sample%")
        .fetch_one(&pool)
        .await?;

    println!("{:?}", user);
    Ok(())
}
