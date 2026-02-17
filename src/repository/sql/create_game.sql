INSERT INTO games (name, developer, encryption_key)
VALUES ($1, $2, $3)
RETURNING id, name, developer, encryption_key, created_at;