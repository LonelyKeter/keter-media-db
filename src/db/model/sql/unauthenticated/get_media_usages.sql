SELECT U.Id, U.Name, MU.MaterialId, MU.Date, MU.Rating, L.Id, L.Title, L.Text, L.Date 
    FROM MaterialUsage MU
    INNER JOIN Users U
    ON MU.UserId = U.Id
    INNER JOIN Materials M
    ON MU.MaterialId = M.Id 
    INNER JOIN Licenses L 
    ON MU.LicenseId = L.Id
    WHERE M.MediaId = $1;