SELECT MaterialId as Id, Format, Quality, Size, LicenseName, DownloadLink 
  FROM Materials
  WHERE MediaId = $1;