SELECT U.Id, U.Name, MU.MaterialId, MU.Date, MU.LicenseId
    FROM MaterialUsage MU
    INNER JOIN Users U
    ON MU.UserId = U.Id
    WHERE MU.MaterialId = $1;