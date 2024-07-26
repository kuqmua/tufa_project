-- Add migration script here
CREATE TABLE jsongeneric (
  std_primitive_i64_as_postgresql_big_serial_not_null_primary_key BIGSERIAL PRIMARY KEY,
  std_primitive_i32_as_postgresql_int INT,
  sqlx_types_json_t_as_postgresql_json_not_null JSON
);
