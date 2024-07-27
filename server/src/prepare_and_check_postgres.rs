pub (crate) async fn prepare_and_check_postgres(pool: &sqlx::Pool<sqlx::Postgres>) {
    //todo how to check if table schema to potentially create equals to actual postgresql table schema if it exists?
	let _ = sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS jsongeneric (
          std_primitive_i64_as_postgresql_big_serial_not_null_primary_key BIGSERIAL PRIMARY KEY,
          std_primitive_i32_as_postgresql_int INT,
          sqlx_types_json_t_as_postgresql_json_not_null JSON
        )
    "#)
	.execute(pool)
	.await.unwrap();
}