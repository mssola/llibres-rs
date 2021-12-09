CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS books (
  id uuid DEFAULT uuid_generate_v4 (),
  title VARCHAR NOT NULL,
  supertitle VARCHAR,
  rate SMALLINT NOT NULL DEFAULT 0,
  status SMALLINT NOT NULL DEFAULT 0,
  location VARCHAR,
  author VARCHAR NOT NULL,
  publisher VARCHAR NOT NULL,
  language VARCHAR NOT NULL,
  notes VARCHAR,
  kind SMALLINT,
  created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
  updated_at TIMESTAMP,
  bought_at TIMESTAMP,

  PRIMARY KEY (id)
);

CREATE UNIQUE INDEX UNIQUE_TITLE_AUTHOR ON books (title, author);
