#[cfg(test)]
mod tests {
    use postgresql::{prelude::*, setup_test, sqlx, Pool};
    use uuid::Uuid;
    const SQL: &str = r#"
        INSERT INTO users (user_name, user_mail, company_uuid, created_at, updated_at)
        VALUES ($1, $2, $3, now(), now())
    "#;
    async fn add_user(pool: &Pool, user_name: &str, user_mail: &str, company_uuid: &Uuid) -> anyhow::Result<()> {
        let _ = sqlx::query(SQL)
            .bind(user_name)
            .bind(user_mail)
            .bind(company_uuid)
            .execute(pool)
            .await?;
        Ok(())
    }

    // RUST_LOG=info cargo test -p fragile test_fragile_function -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_fragile_function() -> anyhow::Result<()> {
        let (pool, _) = setup_test().await?;
        let company = make_companies(&pool, &mut CompaniesBuilder::default()).await?;
        add_user(&pool, "taro", "taro@example.com", &company.uuid).await?;
        add_user(&pool, "jiro", "jiro@example.com", &company.uuid).await?;
        let users = Users::select_all(&pool).await?;
        assert_eq!(users.len(), 2);
        Ok(())
    }
}
