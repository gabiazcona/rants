-- Your SQL goes here
CREATE TABLE rants (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  username TEXT NOT NULL
 -- dateposted TIMESTAMP NOT NULL
)