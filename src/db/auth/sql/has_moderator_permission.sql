SELECT (SELECT administration_permissions
    FROM users
    WHERE id = $1) >= 'moderator'; 