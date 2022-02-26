CREATE TABLE IF NOT EXISTS links (
  id BIGSERIAL NOT NULL PRIMARY KEY,
  shorten VARCHAR(100) NOT NULL,
  link_type int NOT NULL,
  url TEXT NOT NULL,
  created_by BIGINT,
  updated_by BIGINT,
  created_at Timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at Timestamptz
);

