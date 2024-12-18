INSERT INTO public.users (
    uuid
    ,user_name
    ,user_mail
    ,created_at
    ,updated_at
    ,company_uuid
) VALUES (
    gen_random_uuid()
    ,'taro'
    ,'taro@example.com'
    ,NOW()
    ,NOW()
    ,'7b5f4ee9-f35e-47fb-98ba-b27e040350cd'
)