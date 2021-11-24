SELECT id, title, text, date 
    FROM licenses
    WHERE id = $1;