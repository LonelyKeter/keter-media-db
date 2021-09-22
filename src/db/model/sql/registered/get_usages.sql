SELECT MaterialId, UserId, Date, LicenseId 
    FROM MaterialUsage
    WHERE UserId = $1;