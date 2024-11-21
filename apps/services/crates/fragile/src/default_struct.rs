#[cfg(test)]
mod tests {
    use postgresql::{prelude::*, setup_test};
    #[tokio::test]
    async fn test_fragile_default() -> anyhow::Result<()> {
        let pool = setup_test("postgres://user:pass@localhost/web", 5).await?;
        let user = Users {
            user_name: "taro".to_string(),
            user_mail: "taro@example.com".to_string(),
            ..Default::default()
        };
        user.insert(&pool).await?;
        let user = Users {
            user_name: "taro".to_string(),
            user_mail: "taro@example.com".to_string(),
            ..Default::default()
        };
        user.insert(&pool).await?;
        let users = Users::select_all(&pool).await?;
        assert_eq!(users.len(), 1);
        Ok(())
    }
}
