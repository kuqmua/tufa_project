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
pub use ::naming_constants::COMMIT;

pub mod json_value_extractor;
pub mod app_state;

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

pub trait PostgresqlSerdeSerialize<T: serde::Serialize> {
    fn serde_serialize() -> T;
}

struct Test<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    type_1: std::primitive::bool,//BOOL
    type_2: std::primitive::i8,//“CHAR”
    type_3: std::primitive::i16,//SMALLINT, SMALLSERIAL, INT2
    type_4: std::primitive::i32,//INT, SERIAL, INT4
    type_5: std::primitive::i64,//BIGINT, BIGSERIAL, INT8
    type_6: std::primitive::f32,//REAL, FLOAT4
    type_7: std::primitive::f64,//DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    type_9: std::string::String,//VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    type_11: std::vec::Vec<std::primitive::u8>,//BYTEA
    type_12: (),//BYTEA
    type_13: sqlx::postgres::types::PgInterval,//INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    type_14: sqlx::postgres::types::PgRange<std::primitive::i64>,//INT8RANGE
    type_15: sqlx::postgres::types::PgRange<std::primitive::i32>,//INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    type_161: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,//maybe not correct//TSRANGE
    type_162: sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,//maybe not correct//TSRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    type_171: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>>,//maybe not correct//TSTZRANGE
    type_172: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,//maybe not correct//TSTZRANGE
    type_173: sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,//maybe not correct//TSTZRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    type_181: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,//maybe not correct//DATERANGE
    type_182: sqlx::postgres::types::PgRange<sqlx::types::time::Date>,//maybe not correct//DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    type_191: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,//maybe not correct//NUMRANGE
    type_192: sqlx::postgres::types::PgRange<sqlx::types::Decimal>,//maybe not correct//NUMRANGE
    type_20: sqlx::postgres::types::PgMoney,//MONEY
    type_21: sqlx::postgres::types::PgLTree,//LTREE
    type_22: sqlx::postgres::types::PgLQuery,//LQUERY
    type_23: sqlx::postgres::types::PgCiText,//CITEXT
    type_24: sqlx::types::BigDecimal,//NUMERIC
    type_25: sqlx::types::Decimal,//NUMERIC
    type_26: sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,//TIMESTAMPTZ
    type_27: sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,//TIMESTAMPTZ
    type_28: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,//TIMESTAMP
    type_29: sqlx::types::chrono::NaiveDate,//DATE
    type_30: sqlx::types::chrono::NaiveTime,//TIME
    type_31: sqlx::postgres::types::PgTimeTz,//just present chrono or time flag
    // type_31: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    type_32: sqlx::types::time::PrimitiveDateTime,//TIMESTAMP
    type_33: sqlx::types::time::OffsetDateTime,//TIMESTAMPTZ
    type_34: sqlx::types::time::Date,//DATE
    type_35: sqlx::types::time::Time,//TIME
    // type_36: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    type_37: sqlx::types::uuid::Uuid,//UUID
    type_38: sqlx::types::ipnetwork::IpNetwork,//INET, CIDR
    type_39: std::net::IpAddr,//INET, CIDR
    type_40: sqlx::types::mac_address::MacAddress,//MACADDR
    type_41: sqlx::types::BitVec,//BIT, VARBIT
    type_42: sqlx::types::Json<T>,//JSON, JSONB
    type_43: serde_json::Value,//JSON, JSONB
    type_44: serde_json::value::RawValue,//JSON, JSONB
    //maybe Composite types
    //maybe Enumerations
}
//new type pattern
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveBool(std::primitive::bool);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI8(std::primitive::i8);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI16(std::primitive::i16);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI32(std::primitive::i32);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI64(std::primitive::i64);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF32(std::primitive::f32);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF64(std::primitive::f64);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdStringString(std::string::String);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdVecVecStdPrimitiveU8(std::vec::Vec<std::primitive::u8>);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Unit(());
pub struct SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval);
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange<std::primitive::i64>);
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange<std::primitive::i32>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange<sqlx::types::time::Date>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>);
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange<sqlx::types::Decimal>);
pub struct SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney);
pub struct SqlxPostgresTypesPgLTree(sqlx::postgres::types::PgLTree);
pub struct SqlxPostgresTypesPgLQuery(sqlx::postgres::types::PgLQuery);
pub struct SqlxPostgresTypesPgCiText(sqlx::postgres::types::PgCiText);
pub struct SqlxTypesBigDecimal(sqlx::types::BigDecimal);
pub struct SqlxTypesDecimal(sqlx::types::Decimal);
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>);
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>);
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);
pub struct SqlxTypesChronoNaiveDate(sqlx::types::chrono::NaiveDate);
pub struct SqlxTypesChronoNaiveTime(sqlx::types::chrono::NaiveTime);
pub struct SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz);
pub struct SqlxTypesTimePrimitiveDateTime(sqlx::types::time::PrimitiveDateTime);
pub struct SqlxTypesTimeOffsetDateTime(sqlx::types::time::OffsetDateTime);
pub struct SqlxTypesTimeDate(sqlx::types::time::Date);
pub struct SqlxTypesTimeTime(sqlx::types::time::Time);
pub struct SqlxTypesUuidUuid(sqlx::types::uuid::Uuid);
pub struct SqlxTypesIpnetworkIpNetwork(sqlx::types::ipnetwork::IpNetwork);
pub struct StdNetIpAddr(std::net::IpAddr);
pub struct SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress);
pub struct SqlxTypesBitVec(sqlx::types::BitVec);
pub struct SqlxTypesJson<T>(sqlx::types::Json<T>);
pub struct SerdeJsonValue(serde_json::Value);
pub struct SerdeJsonValueRawValue();