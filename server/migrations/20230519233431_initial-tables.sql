-- Add migration script here
-- why not just BIGSERIAL? https://youtu.be/VC9KbAA_5rE?t=2400
CREATE TABLE cats (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name VARCHAR (255) NOT NULL,
  color VARCHAR (255) NOT NULL
);