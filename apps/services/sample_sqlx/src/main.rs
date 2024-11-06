use sqlx::postgres::PgPoolOptions;

// RUST_LOG=info cargo run
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgresql/web")
        .await?;

    common::type_check::execute(&pool).await?;
    common::user_sample::execute(&pool).await?;
    common::composit::execute(&pool).await?;

    api::execute()?;
    Ok(())
}
