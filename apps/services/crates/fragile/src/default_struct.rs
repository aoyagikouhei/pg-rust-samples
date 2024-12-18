#[cfg(test)]
mod tests {
    use postgresql::{prelude::*, setup_test};
    use uuid::Uuid;

    // RUST_LOG=info cargo test -p fragile test_fragile_default -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_fragile_default() -> anyhow::Result<()> {
        let (pool, _) = setup_test().await?;
        let company = make_companies(&pool, &mut CompaniesBuilder::default()).await?;
        let user = Users {
            uuid: Uuid::new_v4(),
            user_name: "taro".to_string(),
            user_mail: "taro@example.com".to_string(),
            company_uuid: company.uuid,
            ..Default::default()
        };
        user.insert(&pool).await?;
        let user = Users {
            uuid: Uuid::new_v4(),
            user_name: "taro".to_string(),
            user_mail: "taro@example.com".to_string(),
            company_uuid: company.uuid,
            ..Default::default()
        };
        user.insert(&pool).await?;
        let users = Users::select_all(&pool).await?;
        assert_eq!(users.len(), 2);
        Ok(())
    }
}
