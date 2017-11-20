-- Your SQL goes here

CREATE TABLE image_set (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 't'
)