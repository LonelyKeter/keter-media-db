WITH casted(kinds, rating_filter, use_count_filter) AS (
    VALUES($2::public.MEDIAKIND[], $3::public.RANGE_FILTER, $4::public.RANGE_FILTER)
)
SELECT 
    id, 
    title, 
    kind, 
    author_id AS id, 
    author_name AS name, 
    rating, 
    use_count
FROM mediaproducts, casted
WHERE 
    ($1 IS NULL OR title ~* $1)
    AND
    (kinds IS NULL OR cardinality(kinds) = 0 OR kind = ANY(kinds))
    AND
    ((rating_filter).limits.min IS NULL OR rating >= (rating_filter).limits.min)
    AND
    ((rating_filter).limits.max IS NULL OR rating <= (rating_filter).limits.max)    
    AND
    ((use_count_filter).limits.min IS NULL OR use_count >= (use_count_filter).limits.min)
    AND
    ((use_count_filter).limits.max IS NULL OR use_count <= (use_count_filter).limits.max)
ORDER BY 
    CASE WHEN (rating_filter).ordering = 'desc' THEN rating END DESC NULLS LAST,
    CASE WHEN (rating_filter).ordering = 'asc' THEN rating END ASC NULLS LAST,
    CASE WHEN (use_count_filter).ordering = 'desc' THEN use_count END DESC NULLS LAST,
    CASE WHEN (use_count_filter).ordering = 'asc' THEN use_count END ASC NULLS LAST;