SELECT MaterialId, Date, LicenseId 
    FROM MaterialUsage
    WHERE UserId = $1;