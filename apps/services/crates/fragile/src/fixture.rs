#[cfg(test)]
mod tests {
    use postgresql::{execute_sql, prelude::*, setup_test, Error, Pool};

    pub async fn fixtrue(pool: &Pool, key: &str) -> Result<(), Error> {
        let path = format!("../../fixtures/{}.sql", key);
        let content = tokio::fs::read_to_string(path).await?;
        execute_sql(pool, &content).await?;
        Ok(())
    }

    // RUST_LOG=info cargo test -p fragile test_fragile_fixture -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_fragile_fixture() -> anyhow::Result<()> {
        let (pool, _) = setup_test().await?;
        fixtrue(&pool, "company").await?;
        fixtrue(&pool, "user_taro").await?;
        fixtrue(&pool, "user_jiro").await?;
        let result = Users::select_all(&pool).await?;
        assert_eq!(result.len(), 2);
        Ok(())
    }
}
