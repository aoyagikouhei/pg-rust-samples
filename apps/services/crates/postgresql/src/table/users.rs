use crate::Pool;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sql_query_builder as sqb;
use sqlx::prelude::*;
use uuid::Uuid;

pub struct UsersFields {
    pub uuid: &'static str,
    pub company_uuid: &'static str,
    pub user_name: &'static str,
    pub user_mail: &'static str,
    pub user_kbn: &'static str,
    pub created_at: &'static str,
    pub created_pg: &'static str,
    pub created_uuid: &'static str,
    pub deleted_at: &'static str,
    pub deleted_pg: &'static str,
    pub deleted_uuid: &'static str,
    pub updated_at: &'static str,
    pub updated_pg: &'static str,
    pub updated_uuid: &'static str,
    pub bk: &'static str,
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, FromRow)]
#[builder(setter(into), default, field(public))]
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
    pub const TABLE_NAME: &'static str = "public.users";

    pub const FIELDS: UsersFields = UsersFields {
        uuid: "uuid",
        company_uuid: "company_uuid",
        user_name: "user_name",
        user_mail: "user_mail",
        user_kbn: "user_kbn",
        created_at: "created_at",
        created_pg: "created_pg",
        created_uuid: "created_uuid",
        deleted_at: "deleted_at",
        deleted_pg: "deleted_pg",
        deleted_uuid: "deleted_uuid",
        updated_at: "updated_at",
        updated_pg: "updated_pg",
        updated_uuid: "updated_uuid",
        bk: "bk",
    };

    pub const SELECT_SQL: &str = r#"
    SELECT
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
    FROM
        public.users
    "#;

    pub const INSERT_SQL: &str = r#"
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

    pub const UPDATE_SQL: &str = r#"
    UPDATE public.users SET
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
        uuid = $1
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

    pub const DELETE_SQL: &str = r#"
    DELETE FROM public.users
    WHERE
        uuid = $1
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

    pub const DELETE_ALL_SQL: &str = r#"
    DELETE FROM public.users
    "#;

    pub async fn insert(&self, pool: &Pool) -> Result<Self, sqlx::Error> {
        sqlx::query_as(Self::INSERT_SQL)
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
            .fetch_one(pool)
            .await
    }

    pub async fn update(&self, pool: &Pool) -> Result<Self, sqlx::Error> {
        sqlx::query_as(Self::UPDATE_SQL)
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
            .fetch_one(pool)
            .await
    }

    pub async fn delete(&self, pool: &Pool) -> Result<Self, sqlx::Error> {
        Self::delete_one(pool, &self.uuid).await
    }

    pub async fn delete_one(pool: &Pool, uuid: &Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as(Self::DELETE_SQL)
            .bind(uuid)
            .fetch_one(pool)
            .await
    }

    pub async fn delete_all(pool: &Pool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(Self::DELETE_ALL_SQL).execute(pool).await?;
        Ok(())
    }

    pub async fn select_all(pool: &Pool) -> Result<Vec<Self>, sqlx::Error> {
        let rows: Vec<Self> = sqlx::query_as(Self::SELECT_SQL).fetch_all(pool).await?;
        Ok(rows)
    }

    pub async fn select_one(pool: &Pool, uuid: &Uuid) -> Result<Option<Self>, sqlx::Error> {
        let query = sqb::Select::new()
            .raw(Self::SELECT_SQL)
            .where_clause(&format!("{} = $1", Self::FIELDS.uuid));
        let one: Option<Self> = sqlx::query_as(&query.as_string())
            .bind(uuid)
            .fetch_optional(pool)
            .await?;
        Ok(one)
    }
}
