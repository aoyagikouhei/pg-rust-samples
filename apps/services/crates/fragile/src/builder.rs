#[cfg(test)]
mod tests {
    use postgresql::{prelude::*, setup_test};
    #[tokio::test]
    async fn test_fragile_builder() -> anyhow::Result<()> {
        let pool = setup_test("postgres://user:pass@localhost/web", 5).await?;
        let _ = make_users(&pool, UsersBuilder::default().user_name("taro")).await?;
        let _ = make_users(&pool, UsersBuilder::default().user_name("jiro")).await?;
        let users = Users::select_all(&pool).await?;
        assert_eq!(users.len(), 2);
        Ok(())
    }
}
