SELECT 
    is_author, 
    administration_permissions
FROM users
WHERE id = $1;