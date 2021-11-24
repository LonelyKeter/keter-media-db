SELECT create_media(
    user_id := $1,
    media_title := $2,
    media_kind := $3::public.MEDIAKIND
);