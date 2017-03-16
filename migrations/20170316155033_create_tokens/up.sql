-- Your SQL goes here
CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  value VARCHAR NOT NULL,
  user_id INTEGER REFERENCES users
)
