use sqlx::postgres::PgPoolOptions;
mod composit;
mod type_check;
mod user_sample;
mod stream;
mod listen;
mod listen2;

// RUST_LOG=info cargo run
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web")
        .await?;

    type_check::execute(&pool).await?;
    //user_sample::execute(&pool).await?;
    composit::execute(&pool).await?;
    //stream::execute(&pool).await?;
    listen2::execute(&pool).await?;
    listen::execute(&pool).await?;
    

    //api::execute()?;
    Ok(())
}
