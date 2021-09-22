SELECT UserId as Id, UserName as Name, Id, Rating, Text, Date 
    FROM Reviews
    WHERE MediaId = $1; 