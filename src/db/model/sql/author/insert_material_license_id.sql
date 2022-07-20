SELECT add_material(
    user_id := $1, 
    media_id := $2, 
    license_id := $3, 
    format := $4, 
    quality := $5::public.Quality
);