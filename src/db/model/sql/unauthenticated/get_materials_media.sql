SELECT 
	m.id, 
	m.media_id, 
	m.format, 
	m.quality, 
	m.license_name, 
	m.rating, 
	m.use_count, 
	(CASE 
	 	WHEN $2 IS NULL THEN NULL 
	 	ELSE mu.user_id IS NOT NULL
	 END) AS is_used
  FROM unauthenticated.materials m 
  LEFT JOIN unauthenticated.material_usage mu 
  ON m.id = mu.material_id AND mu.user_id = $2
  WHERE m.media_id = $1;