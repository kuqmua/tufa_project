pub use generate_postgresql_crud::additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::create_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::create_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::delete_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::delete_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::read_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::read_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::update_many_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::update_one_additional_http_status_codes_error_variants;
pub use generate_postgresql_crud::GeneratePostgresqlCrud;

pub trait IntoSerdeSerializeDeserialize{}

pub trait PostgresqlFilter{}

// impl PostgresqlFilter for sqlx::types:: {}

pub trait PostgresqlOrder{}
impl PostgresqlOrder for std::primitive::bool {}//BOOL
impl PostgresqlOrder for std::primitive::i16 {}//SMALLINT,SMALLSERIAL,INT2
impl PostgresqlOrder for std::primitive::i32 {}//INT,SERIAL,INT4
impl PostgresqlOrder for std::primitive::i64 {}//BIGINT,BIGSERIAL,INT8
impl PostgresqlOrder for sqlx::types::BigDecimal {}//NUMERIC
impl PostgresqlOrder for std::primitive::f32 {}//REAL,FLOAT4
impl PostgresqlOrder for std::primitive::f64 {}//DOUBLE PRECISION,FLOAT8
impl PostgresqlOrder for std::primitive::i8 {}//CHAR
impl PostgresqlOrder for std::primitive::str {}//VARCHAR,CHAR(N),TEXT,NAME,CITEXT
impl PostgresqlOrder for std::string::String {}//VARCHAR,CHAR(N),TEXT,NAME,CITEXT
impl PostgresqlOrder for chrono::NaiveDate {}//DATE
impl PostgresqlOrder for sqlx::types::time::Date {}//DATE
impl PostgresqlOrder for chrono::NaiveTime {}//TIME
impl PostgresqlOrder for sqlx::types::time::Time {}//TIME
impl PostgresqlOrder for chrono::NaiveDateTime {}//TIMESTAMP
impl PostgresqlOrder for sqlx::types::time::PrimitiveDateTime {}//TIMESTAMP
impl PostgresqlOrder for sqlx::postgres::types::PgInterval {}//INTERVAL
impl PostgresqlOrder for sqlx::types::BitVec {}//BIT,VARBIT
//todo arrays, json and maybe something else...

pub trait PostgresqlLimit{}

// integer, bigint
// real, double precision
// varchar
// text
// jsonb
// tsvector
// int4range
// daterange



// impl trait PostgresqlLimit for sqlx::types:: {}



//todo swagger type\schema

pub trait PostgersqlColumn<'a>:
    std::fmt::Debug
    + IntoSerdeSerializeDeserialize
    + utoipa::ToSchema<'a>
    + PostgresqlFilter
    + PostgresqlOrder
    + PostgresqlLimit
{}
