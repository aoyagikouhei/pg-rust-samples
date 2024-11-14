INSERT INTO public.users (
    uuid
    ,user_name
    ,user_mail
    ,created_at
    ,updated_at
) VALUES (
    gen_random_uuid()
    ,'jiro'
    ,'jiro@example.com'
    ,NOW()
    ,NOW()
);
