SELECT auth.register_user(
    CAST($1 AS VARCHAR), 
    CAST($2 AS BYTEA), 
    CAST($3 AS VARCHAR)
);