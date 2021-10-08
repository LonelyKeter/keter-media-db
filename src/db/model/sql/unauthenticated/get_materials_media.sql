SELECT Id, MediaId, Format, Quality, LicenseName 
  FROM Materials
  WHERE MediaId = $1;