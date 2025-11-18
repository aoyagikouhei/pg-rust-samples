DO $DO_STATEMENT$
DECLARE
  v_company_uuid UUID;
BEGIN
  INSERT INTO public.companies (company_name, created_at, updated_at)
  VALUES ('Sample Company', NOW(), NOW())
  RETURNING uuid INTO v_company_uuid;
  
  EXECUTE $$
    CREATE TABLE IF NOT EXISTS public.users_$$ || replace(v_company_uuid::TEXT, '-', '') || 
    $$ PARTITION OF public.users FOR VALUES IN ('$$ || v_company_uuid || $$') $$
  ;

  INSERT INTO public.users (company_uuid, user_name, created_at, updated_at)
  VALUES (v_company_uuid, 'Sample User', NOW(), NOW());
END $DO_STATEMENT$;