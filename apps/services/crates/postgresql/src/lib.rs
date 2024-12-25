use crate::prelude::*;
use common::config::Config;
use sqlx::postgres::PgPoolOptions;
pub mod custom;
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

pub async fn setup_test() -> Result<(Pool, Config), sqlx::Error> {
    let _ = env_logger::try_init();
    let config = Config::setup().await.unwrap();
    println!("{:?}", std::thread::current().id());
    let pool = get_postgres_pool(&config.pg_url, config.max_connections).await?;
    clear_db(&pool).await?;
    Ok((pool, config))
}

pub async fn execute_sql(pool: &Pool, sql: &str) -> Result<(), sqlx::Error> {
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub fn get_code(e: sqlx::Error) -> Option<String> {
    match e {
        sqlx::Error::Database(db_err) => {
            let code = db_err.code();
            code.map(|c| c.to_string())
        }
        _ => None,
    }
}
