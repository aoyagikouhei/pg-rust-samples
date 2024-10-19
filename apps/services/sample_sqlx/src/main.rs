use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);
    Ok(())
}