SELECT Id, MediaId, Format, Quality, LicenseName 
  FROM Materials
  WHERE Id = $1;