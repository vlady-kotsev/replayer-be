-- Add migration script here
CREATE TABLE games (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    developer VARCHAR(255) NOT NULL,
    encryption_key TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);