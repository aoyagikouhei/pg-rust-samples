use crate::prelude::*;
use sqlx::postgres::PgPoolOptions;
pub mod table;
pub use sqlx;
pub use sqlx::Error;
pub mod kbn_constants;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn get_postgres_pool(url: &str, max_connections: u32) -> Result<Pool, sqlx::Error> {
    let res = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await?;
    Ok(res)
}

pub mod prelude {
    pub use crate::table::companies::*;
    pub use crate::table::users::*;
    pub use crate::table::*;
}

pub async fn clear_db(pool: &Pool) -> Result<(), sqlx::Error> {
    Users::delete_all(pool).await?;
    Companies::delete_all(pool).await?;
    Ok(())
}

pub async fn setup_test(url: &str, max_connections: u32) -> Result<Pool, sqlx::Error> {
    let pool = get_postgres_pool(url, max_connections).await?;
    clear_db(&pool).await?;
    Ok(pool)
}

pub async fn execute_sql(pool: &Pool, sql: &str) -> Result<(), sqlx::Error> {
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}
