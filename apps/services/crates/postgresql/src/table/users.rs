use crate::Pool;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

const SELECT_SQL: &str = r#"
SELECT
    t1.uuid
    ,t1.company_uuid
    ,t1.user_name
    ,t1.user_mail
    ,t1.user_kbn
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
    ,company_uuid
    ,user_name
    ,user_mail
    ,user_kbn
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
    ,$14
    ,$15
) RETURNING
    uuid
    ,company_uuid
    ,user_name
    ,user_mail
    ,user_kbn
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
UPDATE public.users AS t1 SET
    company_uuid = $2
    ,user_name = $3
    ,user_mail = $4
    ,user_kbn = $5
    ,created_uuid = $6
    ,updated_uuid = $7
    ,deletded_uuid = $8
    ,created_at = $9
    ,updated_at = $10
    ,deleted_at = $11
    ,created_pg = $12
    ,updated_pg = $13
    ,deleted_pg = $14
    ,bk = $15
WHERE
    t1.uuid = $1
RETURNING
    uuid
    ,company_uuid
    ,user_name
    ,user_mail
    ,user_kbn
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
DELETE FROM public.users AS t1
WHERE
    t1.uuid = $1
RETURNING
    uuid
    ,company_uuid
    ,user_name
    ,user_mail
    ,user_kbn
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
DELETE FROM public.users AS t1
"#;

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, FromRow)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct Users {
    pub uuid: Uuid,
    pub company_uuid: Uuid,
    pub user_name: String,
    pub user_mail: String,
    pub user_kbn: crate::kbn_constants::UserKbn,
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
    pub async fn insert(&self, pg_pool: &Pool) -> Result<Self, sqlx::Error> {
        sqlx::query_as(INSERT_SQL)
            .bind(self.uuid)
            .bind(self.company_uuid)
            .bind(&self.user_name)
            .bind(&self.user_mail)
            .bind(&self.user_kbn)
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
            .fetch_one(pg_pool)
            .await
    }

    pub async fn update(&self, pg_pool: &Pool) -> Result<Self, sqlx::Error> {
        sqlx::query_as(UPDATE_SQL)
            .bind(self.uuid)
            .bind(self.company_uuid)
            .bind(&self.user_name)
            .bind(&self.user_mail)
            .bind(&self.user_kbn)
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
            .fetch_one(pg_pool)
            .await
    }

    pub async fn delete(&self, pg_pool: &Pool) -> Result<Self, sqlx::Error> {
        Self::delete_one(pg_pool, &self.uuid).await
    }

    pub async fn delete_one(pg_pool: &Pool, uuid: &Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as(DELETE_SQL)
            .bind(uuid)
            .fetch_one(pg_pool)
            .await
    }

    pub async fn delete_all(pg_pool: &Pool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(DELETE_ALL_SQL).execute(pg_pool).await?;
        Ok(())
    }

    pub async fn select_all(pg_pool: &Pool) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<Self> = sqlx::query_as(SELECT_SQL).fetch_all(pg_pool).await?;
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
