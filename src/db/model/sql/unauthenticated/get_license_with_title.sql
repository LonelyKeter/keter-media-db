SELECT 
    id, 
    title, 
    text, 
    date 
FROM licenses
WHERE title = $1;