use crate::Pool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use derive_builder::Builder;
use sqlx::prelude::*;

const SELECT_SQL: &str = r#"
SELECT
    t1.uuid
    ,t1.issue_name
    ,t1.created_uuid
    ,t1.updated_uuid
    ,t1.deleted_uuid
    ,t1.created_at
    ,t1.updated_at
    ,t1.deleted_at
    ,t1.created_pg
    ,t1.updated_pg
    ,t1.deleted_pg
    ,t1.bk
FROM
    public.issues AS t1
"#;

const INSERT_SQL: &str = r#"
INSERT INTO public.issues (
    uuid
    ,issue_name
    ,created_uuid
    ,updated_uuid
    ,deleted_uuid
    ,created_at
    ,updated_at
    ,deleted_at
    ,created_pg
    ,updated_pg
    ,deleted_pg
    ,bk
) VALUES (
    $1
    ,$2
    ,$3
    ,$4
    ,$5
    ,$6
    ,$7
    ,$8
    ,$9
    ,$10
    ,$11
    ,$12
) RETURNING
    uuid
    ,issue_name
    ,created_uuid
    ,updated_uuid
    ,deleted_uuid
    ,created_at
    ,updated_at
    ,deleted_at
    ,created_pg
    ,updated_pg
    ,deleted_pg
    ,bk
"#;

const UPDATE_SQL: &str = r#"
UPDATE public.issues AS t1 SET
    issue_name = $2
    ,created_uuid = $3
    ,updated_uuid = $4
    ,deletded_uuid = $5
    ,created_at = $6
    ,updated_at = $7
    ,deleted_at = $8
    ,created_pg = $9
    ,updated_pg = $10
    ,deleted_pg = $11
    ,bk = $12
WHERE
    t1.uuid = $1
RETURNING
    uuid
    ,issue_name
    ,created_uuid
    ,updated_uuid
    ,deleted_uuid
    ,created_at
    ,updated_at
    ,deleted_at
    ,created_pg
    ,updated_pg
    ,deleted_pg
    ,bk
"#;

const DELETE_SQL: &str = r#"
DELETE FROM public.issues AS t1
WHERE
    t1.uuid = $1
RETURNING
    uuid
    ,issue_name
    ,created_uuid
    ,updated_uuid
    ,deleted_uuid
    ,created_at
    ,updated_at
    ,deleted_at
    ,created_pg
    ,updated_pg
    ,deleted_pg
    ,bk
"#;

const DELETE_ALL_SQL: &str = r#"
DELETE FROM public.issues AS t1
"#;

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, FromRow)]
#[builder(setter(into), default, field(public))]
pub struct Issues {
    pub uuid: Uuid,
    pub issue_name: String,
    pub created_uuid: Uuid,
    pub updated_uuid: Uuid,
    pub deleted_uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_pg: String,
    pub updated_pg: String,
    pub deleted_pg: String,
    pub bk: Option<String>,
}

impl Issues {
    pub async fn insert(&self, pool: &Pool) -> Result<Self, sqlx::Error> {
        sqlx::query_as(INSERT_SQL)
            .bind(self.uuid)
            .bind(&self.issue_name)
            .bind(self.created_uuid)
            .bind(self.updated_uuid)
            .bind(self.deleted_uuid)
            .bind(self.created_at)
            .bind(self.updated_at)
            .bind(self.deleted_at)
            .bind(&self.created_pg)
            .bind(&self.updated_pg)
            .bind(&self.deleted_pg)
            .bind(&self.bk)
            .fetch_one(pool)
            .await
    }

    pub async fn update(&self, pool: &Pool) -> Result<Self, sqlx::Error> {
        sqlx::query_as(UPDATE_SQL)
            .bind(self.uuid)
            .bind(&self.issue_name)
            .bind(self.created_uuid)
            .bind(self.updated_uuid)
            .bind(self.deleted_uuid)
            .bind(self.created_at)
            .bind(self.updated_at)
            .bind(self.deleted_at)
            .bind(&self.created_pg)
            .bind(&self.updated_pg)
            .bind(&self.deleted_pg)
            .bind(&self.bk)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(&self, pool: &Pool) -> Result<Self, sqlx::Error> {
        Self::delete_one(pool, &self.uuid).await
    }

    pub async fn delete_one(pool: &Pool, uuid: &Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as(DELETE_SQL)
            .bind(uuid)
            .fetch_one(pool)
            .await
    }

    pub async fn delete_all(pool: &Pool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(DELETE_ALL_SQL)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn select_all(pool: &Pool) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<Self> = sqlx::query_as(SELECT_SQL)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn select_one(pool: &Pool, uuid: &Uuid) -> Result<Option<Self>, sqlx::Error> {
        let one: Option<Self> = sqlx::query_as(&format!("{} WHERE t1.uuid = $1", SELECT_SQL))
            .bind(uuid)
            .fetch_optional(pool)
            .await?;
        Ok(one)
    }
}
