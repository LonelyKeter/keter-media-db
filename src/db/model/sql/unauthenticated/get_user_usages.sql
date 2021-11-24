SELECT mu.material_id, mu.date, mu.rating, l.id, l.title, l.text, l.date 
    FROM material_usage mu 
    INNER JOIN licenses l 
    ON mu.license_id = l.id
    WHERE user_id = $1;