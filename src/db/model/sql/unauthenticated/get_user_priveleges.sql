SELECT Author, Moderator, Administrator
  FROM Users
  WHERE Id = $1;