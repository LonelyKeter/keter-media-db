SELECT PostReview(
    user_id := $1, 
    media_id := $2, 
    rating := $3, 
    text := $4);