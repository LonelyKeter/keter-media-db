SELECT UserId, UserName, Id, Rating, Text, Date 
    FROM Reviews
    WHERE MediaId == $1; 