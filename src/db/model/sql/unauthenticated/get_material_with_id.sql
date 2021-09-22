SELECT MaterialId as Id, MediaId, Format, Quality, Size, LicenseName 
  FROM Materials
  WHERE MaterialId = $1;