#[cfg(test)]
mod tests {
    use postgresql::{prelude::*, setup_test, sqlx, Pool};
    const SQL: &str = r#"
        INSERT INTO users (user_name, user_mail, created_at, updated_at)
        VALUES ($1, $2, now(), now())
    "#;
    async fn add_user(pool: &Pool, user_name: &str, user_mail: &str) -> anyhow::Result<()> {
        let _ = sqlx::query(SQL)
            .bind(user_name)
            .bind(user_mail)
            .execute(pool)
            .await?;
        Ok(())
    }
    #[tokio::test]
    async fn test_fragile_function() -> anyhow::Result<()> {
        let pool = setup_test("postgres://user:pass@localhost/web", 5).await?;
        add_user(&pool, "taro", "taro@example.com").await?;
        add_user(&pool, "jiro", "jiro@example.com").await?;
        let users = Users::select_all(&pool).await?;
        assert_eq!(users.len(), 2);
        Ok(())
    }
}
