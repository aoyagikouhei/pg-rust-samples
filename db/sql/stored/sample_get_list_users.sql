DROP TYPE IF EXISTS type_sample_get_list_users CASCADE;
CREATE TYPE type_sample_get_list_users AS (
  uuid UUID
  ,content TEXT
  ,created_at TIMESTAMPTZ
);

-- 企業に登録されている項目と、その選択肢を返す
CREATE OR REPLACE FUNCTION sample_get_list_users(
  p_uuid UUID DEFAULT NULL
) RETURNS SETOF type_sample_get_list_users AS $FUNCTION$
BEGIN
  RAISE NOTICE 'sample_get_list_users';

  RETURN QUERY
  SELECT
    p_uuid
    ,'予定表～①💖ﾊﾝｶｸだ'
    ,now()
  ;
END;
$FUNCTION$ LANGUAGE plpgsql;
