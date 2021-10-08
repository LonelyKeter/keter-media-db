SELECT 
    (SELECT COUNT(MaterialId) 
        FROM unauthenticated.MaterialUsage 
        WHERE MaterialId = $1 AND UserId = $2
    ) > 0;