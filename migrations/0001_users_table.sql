-- Column definition order rule:
-- 1) Column type should come right after the column name.
-- 2) Constraint order like NOT NULL / UNIQUE can be swapped.
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);
