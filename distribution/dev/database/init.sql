CREATE TABLE IF NOT EXISTS users (
  id            SERIAL PRIMARY KEY,
  username      VARCHAR(64) NOT NULL,
  email         VARCHAR(64) NOT NULL UNIQUE,
  password_hash VARCHAR(64) NOT NULL,
  deleted_at    TIMESTAMP WITH TIME ZONE  DEFAULT NULL,
  token         TEXT DEFAULT NULL
);