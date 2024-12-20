-- UUIDv7を取得する
-- 引数
--   p_now : 現在時間
-- 戻り値
--   UUID
CREATE OR REPLACE FUNCTION uv_uuid_v7(
  p_now TIMESTAMPTZ DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
  w_epoch BIGINT := (extract(epoch from COALESCE(p_now, now())) * 1000)::BIGINT;
BEGIN
  RETURN
    -- timestamp
    lpad(to_hex((w_epoch >> 16)), 8, '0') || '-' ||
    lpad(to_hex((w_epoch & 0xffff)), 4, '0') || '-' ||
    -- version / rand_a
    lpad(to_hex((0x7000 + (random() * 0x0fff)::int)), 4, '0') || '-' ||
    -- variant / rand_b
    lpad(to_hex((0x8000 + (random() * 0x3fff)::int)), 4, '0') || '-' ||
    -- rand_b
    lpad(to_hex((floor(random() * (2^48))::bigint >> 16)), 12, '0');
END;
$$ LANGUAGE plpgsql;

-- UUIDv7から時間を取得する
-- 引数
--   p_uuid : UUID
-- 戻り値
--   UUIDの時間
CREATE OR REPLACE FUNCTION uv_uuid_to_timestamptz(
  p_uuid UUID DEFAULT NULL
) RETURNS TIMESTAMPTZ AS $$
DECLARE
  w_value BIGINT := ('x'||replace(p_uuid::TEXT, '-', ''))::bit(48)::BIGINT;
BEGIN
  RETURN
    to_timestamp(w_value / 1000) + ((w_value % 1000) || ' milliseconds')::INTERVAL
  ;
END;
$$ LANGUAGE plpgsql;