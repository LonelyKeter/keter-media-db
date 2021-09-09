SELECT MaterialId as Id, Format, Quality, LicenseName, DownloadLink 
  FROM Materials
  WHERE MediaId = $1;