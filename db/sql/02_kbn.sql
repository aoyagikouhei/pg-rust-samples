
--  ユーザー区分
DROP TYPE IF EXISTS type_enum_user CASCADE;
CREATE TYPE type_enum_user AS ENUM (
  'normal'
  ,'admin'
);
