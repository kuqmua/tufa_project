-- Add migration script here
CREATE TABLE dogs (
  std_primitive_i64_as_postgresql_big_serial_not_null_primary_key BIGSERIAL PRIMARY KEY,
  std_primitive_bool_as_postgresql_bool BOOL,
  std_primitive_i16_as_postgresql_small_int SMALLINT,
  std_primitive_i32_as_postgresql_int INT
);
-- CREATE TABLE dogs (
--   id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
--   name VARCHAR (255) NOT NULL,
--   color VARCHAR (255) NOT NULL
-- );