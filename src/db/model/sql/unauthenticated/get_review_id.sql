SELECT UserId as Id, UserName as Name, Id, Text, Date 
    FROM Reviews
    WHERE Id = $1; 