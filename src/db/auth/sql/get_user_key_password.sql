SELECT id, password 
  FROM users 
  WHERE email = $1;