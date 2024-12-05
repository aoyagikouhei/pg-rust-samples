DROP TYPE IF EXISTS type_sample_get_list_companies CASCADE;
CREATE TYPE type_sample_get_list_companies AS (
  company_uuid UUID
  ,company_name TEXT
);

CREATE OR REPLACE FUNCTION sample_get_list_companies (
  p_company_name TEXT DEFAULT NULL
  ,p_now TIMESTAMPTZ DEFAULT NULL
  ,p_pg TEXT DEFAULT NULL
  ,p_operator_uuid UUID DEFAULT NULL
) RETURNS SETOF type_sample_get_list_companies AS $FUNCTION$
DECLARE
  w_now TIMESTAMPTZ := COALESCE(p_now, NOW());
  w_pg TEXT := COALESCE(p_pg, 'sample_get_list_companies');
  w_operator_uuid UUID := COALESCE(p_operator_uuid, '00000000-0000-0000-0000-000000000000');
BEGIN
  RAISE NOTICE 'sample_get_list_companies started p_company_name = %', p_company_name;
  
  RETURN QUERY SELECT
    t1.uuid
    ,t1.company_name
  FROM
    public.companies AS t1
  WHERE
    (
      p_company_name IS NULL 
      OR t1.company_name ILIKE '%' || replace(replace(p_company_name, '_', '\_'), '%', '\%') || '%'
    )
  ;
END;
$FUNCTION$ LANGUAGE plpgsql;