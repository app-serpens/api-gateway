-- Your SQL goes here
CREATE TABLE users (
  id uuid PRIMARY KEY,
  email TEXT NOT NULL,
  password TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
)