SELECT id, title, kind, author_id AS id, author_name AS name, author_country AS country, rating, use_count
  FROM mediaproducts
  WHERE id = $1;