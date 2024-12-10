use chrono::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, PartialEq, Eq)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct DbInput {
    pub company_name: String,
    pub now: Option<DateTime<Utc>>,
    pub pg: Option<String>,
    pub operator_uuid: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, PartialEq, Eq, FromRow)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct DbOutput {
    pub company_uuid: Uuid,
    pub company_name: String,
}

const SQL: &str = r#"
SELECT
    t1.*
FROM
    sample_get_list_companies(
        p_company_name := $1
        ,p_now := $2
        ,p_pg := $3
        ,p_operator_uuid := $4
    ) AS t1
"#;

pub async fn execute(pg_pool: &crate::Pool, params: DbInput) -> Result<Vec<DbOutput>, sqlx::Error> {
    let res: Vec<DbOutput> = sqlx::query_as(SQL)
        .bind(&params.company_name)
        .bind(params.now)
        .bind(&params.pg)
        .bind(params.operator_uuid)
        .fetch_all(pg_pool)
        .await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, setup_test};

    // RUST_LOG=info REALM_CODE=test cargo test -p postgresql test_postgresql_sample_get_list_companies -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_postgresql_sample_get_list_companies() -> anyhow::Result<()> {
        let (pool, _) = setup_test().await?;

        make_companies(
            &pool,
            &mut CompaniesBuilder::default().company_name("あいうえお工業"),
        )
        .await?;
        make_companies(
            &pool,
            &mut CompaniesBuilder::default().company_name("かきくけこ工業"),
        )
        .await?;

        let params = DbInput {
            company_name: "あいうえお".to_string(),
            ..Default::default()
        };

        let result = execute(&pool, params).await?;
        assert_eq!(result.len(), 1);

        Ok(())
    }
}
