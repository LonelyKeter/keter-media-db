SELECT u.id, u.name, mu.material_id, mu.date, l.id, l.title, l.text, l.date 
    FROM material_usage mu
    INNER JOIN users u
    ON mu.user_id = u.id
    INNER JOIN licenses l 
    ON mu.license_id = l.id
    WHERE mu.material_id = $1;