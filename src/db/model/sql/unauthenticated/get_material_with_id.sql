SELECT 
	M.Id, 
	M.MediaId, 
	M.Format, 
	M.Quality, 
	M.LicenseName, 
	M.Rating, 
	M.UseCount, 
	(CASE 
	 	WHEN $2 IS NULL THEN NULL 
	 	ELSE MU.UserId IS NOT NULL
	 END) AS IsUsed
  FROM unauthenticated.Materials M 
  LEFT JOIN unauthenticated.MaterialUsage MU 
  ON M.Id = MU.MaterialId AND MU.UserId = $2
  WHERE M.Id = $1;