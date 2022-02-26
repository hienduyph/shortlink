CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL NOT NULL PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(122) NOT NULL,
  password_salt VARCHAR(122) NOT NULL,
  updated_by BIGINT,
  created_at Timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at Timestamptz,
  status INT
);

