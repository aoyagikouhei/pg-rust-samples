#[cfg(test)]
mod tests {
    use postgresql::{prelude::*, setup_test};

    // RUST_LOG=info cargo test -p fragile test_fragile_builder -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_fragile_builder() -> anyhow::Result<()> {
        let (pool, _) = setup_test().await?;
        let company =
            make_companies(&pool, CompaniesBuilder::default().company_name("company")).await?;
        let _ = make_users(
            &pool,
            UsersBuilder::default()
                .user_name("taro")
                .company_uuid(company.uuid),
        )
        .await?;
        let _ = make_users(
            &pool,
            UsersBuilder::default()
                .user_name("jiro")
                .company_uuid(company.uuid),
        )
        .await?;
        let users = Users::select_all(&pool).await?;
        assert_eq!(users.len(), 2);
        Ok(())
    }
}
