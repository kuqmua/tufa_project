-- Add migration script here
CREATE TABLE test_table (
  uuid BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name VARCHAR (255) NOT NULL,
  color VARCHAR (255) NOT NULL
);