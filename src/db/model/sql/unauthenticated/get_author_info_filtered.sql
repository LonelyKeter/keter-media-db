WITH casted(kinds, rating_filter) AS (
    VALUES($2::public.MEDIAKIND[], $3::public.RANGE_FILTER)
)
SELECT 
    id, 
    name,
    email,
    rating
FROM authors a, casted
WHERE 
    ($1 IS NULL OR name ~* $1)
    AND
    (
        kinds IS NULL OR cardinality(kinds) = 0 OR 
        EXISTS(SELECT 1 FROM mediaproducts WHERE kind = ANY(kinds) AND author_id = a.id)
    )
    AND
    ((rating_filter).limits.min IS NULL OR rating >= (rating_filter).limits.min)
    AND
    ((rating_filter).limits.max IS NULL OR rating <= (rating_filter).limits.max)    
ORDER BY 
    CASE WHEN (rating_filter).ordering = 'desc' THEN rating END DESC NULLS LAST,
    CASE WHEN (rating_filter).ordering = 'asc' THEN rating END ASC NULLS LAST;