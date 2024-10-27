DROP TYPE IF EXISTS type_enum_os CASCADE;
CREATE TYPE type_enum_os AS ENUM (
  'linux',
  'mac_os',
  'windows'
);

DROP DOMAIN IF EXISTS domain_integer CASCADE;
CREATE DOMAIN domain_integer AS BIGINT CHECK(VALUE >= 0);

DROP TYPE IF EXISTS type_sample_get_select_composite CASCADE;
CREATE TYPE type_sample_get_select_composite AS (
  os type_enum_os,
  cpu_count domain_integer,
  memory_size BIGINT
);

CREATE OR REPLACE FUNCTION sample_get_select_composite(
  p_value type_sample_get_select_composite DEFAULT NULL
) RETURNS SETOF type_sample_get_select_composite AS $FUNCTION$
DECLARE
BEGIN
  RETURN NEXT p_value;
  RETURN NEXT p_value;
END;
$FUNCTION$ LANGUAGE plpgsql;