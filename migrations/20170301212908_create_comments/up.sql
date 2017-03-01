CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  user_id INTEGER REFERENCES users,
  post_id INTEGER REFERENCES posts
)
