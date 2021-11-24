SELECT user_id AS id, user_name AS name, id, text, date 
    FROM reviews
    WHERE media_id = $1; 