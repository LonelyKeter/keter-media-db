SELECT U.Id, U.Name, MU.MaterialId, MU.Date, MU.LicenseId
    FROM MaterialUsage MU
    INNER JOIN Users U
    ON MU.UserId = U.Id
    INNER JOIN Materials M
    ON MU.MaterialId = M.Id 
    WHERE M.MediaId = $1;