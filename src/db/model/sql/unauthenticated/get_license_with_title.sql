SELECT Id, Title, Text, Date 
    FROM Licenses
    WHERE Title = $1;