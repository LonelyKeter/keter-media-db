SELECT 
    user_id AS id, 
    user_name AS name, 
    id, 
    text, 
    date 
FROM reviews
WHERE id = $1; 