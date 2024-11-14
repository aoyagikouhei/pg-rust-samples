DROP TYPE IF EXISTS type_sample_get_list_values CASCADE;
CREATE TYPE type_sample_get_list_values AS (
  uuid UUID
  ,content TEXT
  ,created_at TIMESTAMPTZ
  ,data_json JSONB
);

-- 企業に登録されている項目と、その選択肢を返す
CREATE OR REPLACE FUNCTION sample_get_list_values(
  p_uuid UUID DEFAULT NULL
) RETURNS SETOF type_sample_get_list_values AS $FUNCTION$
BEGIN
  RAISE NOTICE 'sample_get_list_values';

  RETURN QUERY
  SELECT
    p_uuid
    ,'予定表～①💖ﾊﾝｶｸだ'
    ,now()
    ,'{"key": "value"}'::JSONB
  ;
END;
$FUNCTION$ LANGUAGE plpgsql;
