DROP TABLE IF EXISTS public.users CASCADE;
DROP TABLE IF EXISTS garbage.users CASCADE;

CREATE TABLE public.users (
  uuid UUID NOT NULL DEFAULT gen_random_uuid() -- UUID
  ,company_uuid UUID NOT NULL -- 企業UUID
  ,user_name TEXT NOT NULL DEFAULT '' -- ユーザ名
  ,user_mail TEXT NOT NULL DEFAULT '' -- ユーザメール
  ,user_kbn type_enum_user NOT NULL DEFAULT 'normal' -- ユーザ区分
  ,created_uuid UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
  ,updated_uuid UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
  ,deleted_uuid UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
  ,created_at TIMESTAMPTZ NOT NULL
  ,updated_at TIMESTAMPTZ NOT NULL
  ,deleted_at TIMESTAMPTZ
  ,created_pg TEXT NOT NULL DEFAULT ''
  ,updated_pg TEXT NOT NULL DEFAULT ''
  ,deleted_pg TEXT NOT NULL DEFAULT ''
  ,bk TEXT
  ,PRIMARY KEY(uuid)  
  ,FOREIGN KEY (company_uuid) REFERENCES companies(uuid)
);

CREATE TABLE garbage.users (
  uuid UUID NOT NULL DEFAULT gen_random_uuid() -- UUID
  ,company_uuid UUID NOT NULL -- 企業UUID
  ,user_name TEXT NOT NULL DEFAULT '' -- ユーザ名
  ,user_mail TEXT NOT NULL DEFAULT '' -- ユーザメール
  ,user_kbn type_enum_user NOT NULL DEFAULT 'normal' -- ユーザ区分
  ,created_uuid UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
  ,updated_uuid UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
  ,deleted_uuid UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
  ,created_at TIMESTAMPTZ NOT NULL
  ,updated_at TIMESTAMPTZ NOT NULL
  ,deleted_at TIMESTAMPTZ
  ,created_pg TEXT NOT NULL DEFAULT ''
  ,updated_pg TEXT NOT NULL DEFAULT ''
  ,deleted_pg TEXT NOT NULL DEFAULT ''
  ,bk TEXT
  ,PRIMARY KEY(uuid) 
);