SELECT MU.MaterialId, MU.Date, MU.Rating, L.Id, L.Title, L.Text, L.Date 
    FROM MaterialUsage MU 
    INNER JOIN Licenses L 
    ON MU.LicenseId = L.Id
    WHERE MU.MaterialId = $1 AND MU.UserId = $2;