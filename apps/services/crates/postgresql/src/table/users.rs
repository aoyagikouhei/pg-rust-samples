use crate::Pool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use derive_builder::Builder;
use sqlx::prelude::*;

const SELECT_SQL: &str = r#"
SELECT
    t1.uuid
    ,t1.user_name
    ,t1.user_mail
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
    public.users AS t1
"#;

const INSERT_SQL: &str = r#"
INSERT INTO public.users (
    uuid
    ,user_name
    ,user_mail
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
    ,$13
)
"#;

const UPDATE_SQL: &str = r#"
UPDATE public.users AS t1 SET
    user_name = $2
    ,user_mail = $3
    ,created_uuid = $4
    ,updated_uuid = $5
    ,deletded_uuid = $6
    ,created_at = $7
    ,updated_at = $8
    ,deleted_at = $9
    ,created_pg = $10
    ,updated_pg = $11
    ,deleted_pg = $12
    ,bk = $13
WHERE
    t1.uuid = $1
"#;

const DELETE_SQL: &str = r#"
DELETE FROM public.users AS t1
WHERE
    t1.uuid = $1
"#;

const DELETE_ALL_SQL: &str = r#"
DELETE FROM public.users AS t1
"#;

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, FromRow)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct Users {
    pub uuid: Uuid,
    pub user_name: String,
    pub user_mail: String,
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

impl Users {
    pub async fn insert(&self, pg_pool: &Pool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(INSERT_SQL)
            .bind(self.uuid)
            .bind(&self.user_name)
            .bind(&self.user_mail)
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
            .execute(pg_pool)
            .await?;
        Ok(())
    }

    pub async fn update(&self, pg_pool: &Pool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(UPDATE_SQL)
            .bind(self.uuid)
            .bind(&self.user_name)
            .bind(&self.user_mail)
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
            .execute(pg_pool)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, pg_pool: &Pool) -> Result<(), sqlx::Error> {
        Self::delete_one(pg_pool, &self.uuid).await
    }

    pub async fn delete_one(pg_pool: &Pool, uuid: &Uuid) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(DELETE_SQL)
            .bind(uuid)
            .execute(pg_pool)
            .await?;
        Ok(())
    }

    pub async fn delete_all(pg_pool: &Pool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(DELETE_ALL_SQL)
            .execute(pg_pool)
            .await?;
        Ok(())
    }

    pub async fn select_all(pg_pool: &Pool) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<Self> = sqlx::query_as(SELECT_SQL)
            .fetch_all(pg_pool)
            .await?;
        Ok(rows)
    }

    pub async fn select_one(pg_pool: &Pool, uuid: &Uuid) -> Result<Option<Self>, sqlx::Error> {
        let one: Option<Self> = sqlx::query_as(&format!("{} WHERE t1.uuid = $1", SELECT_SQL))
            .bind(uuid)
            .fetch_optional(pg_pool)
            .await?;
        Ok(one)
    }
}
