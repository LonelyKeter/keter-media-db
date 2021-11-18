SELECT Id, Title, Kind, AuthorId as Id, AuthorName as Name, AuthorCountry as Country, Rating, UseCount
  FROM Mediaproducts
  WHERE AuthorId = $1;