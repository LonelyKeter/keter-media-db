SELECT post_review(
    user_id := $1, 
    media_id := $2, 
    text := $3
);