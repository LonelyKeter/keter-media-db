SELECT MaterialId as Id, MediaId, Format, Quality, Size, LicenseName 
  FROM Materials
  WHERE MediaId = $1;