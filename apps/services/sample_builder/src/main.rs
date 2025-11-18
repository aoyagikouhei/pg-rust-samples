use sql_query_builder as sql;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web")
        .await?;

    let mut select = sql::Select::new()
        .select("id, login")
        .from("users")
        .where_clause("login = $1");

    let is_admin = true;

    if is_admin {
        select = select.where_clause("is_admin = true");
    }

    let query = select.as_string();

    println!("{query}");

    //api::execute()?;
    Ok(())
}
