SELECT Author, Moderator, Admin
  FROM Users
  WHERE Id = $1;