SELECT 
    (SELECT COUNT(MaterialId) 
        FROM registered.MaterialUsage 
        WHERE MaterialId = $1 AND UserId = $2
    ) > 0;