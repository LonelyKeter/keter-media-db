SELECT Id, Title, Kind, AuthorName, AuthorCountry, Rating 
  FROM FilterMedia($1, $2, $3, $4);