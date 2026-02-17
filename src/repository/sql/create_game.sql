INSERT INTO games (name, developer, encryption_key, nonce)
VALUES ($1, $2, $3, $4)
RETURNING id, name, developer, encryption_key, nonce, created_at;