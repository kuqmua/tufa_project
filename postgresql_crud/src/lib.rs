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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Test {
    type_1: bool,//BOOL
    type_2: std::primitive::i8,//“CHAR”
    // type_3: ,
    // type_4: ,
    // type_5: ,
    // type_6: ,
    // type_7: ,
    // type_8: ,
    // type_9: ,
    // type_10: ,
    // type_11: ,
    // type_12: ,
    // type_13: ,
    // type_14: ,
    // type_15: ,
    // type_16: ,
    // type_17: ,
    // type_18: ,
    // type_19: ,
    // type_20: ,
    // type_21: ,
    // type_22: ,
    // type_23: ,
    // type_24: ,
    // type_25: ,
    // type_26: ,
    // type_27: ,
    // type_28: ,
    // type_29: ,
    // type_30: ,
    // type_31: ,
    // type_32: ,
    // type_33: ,
    // type_34: ,
    // type_35: ,
    // type_36: ,
    // type_37: ,
    // type_38: ,
    // type_39: ,
    // type_40: ,
    // type_41: ,
    // type_42: ,
    // type_43: ,
    // type_44: ,
    // type_45: ,
    // type_46: ,
    // type_47: ,
    // type_48: ,
    // type_49: ,
    // type_50: ,
    // type_51: ,
    // type_52: ,
    // type_53: ,
    // type_54: ,
    // type_55: ,
    // type_56: ,




    // a: std::primitive::i16,
    // // b: &std::primitive::str,
    // c: std::primitive::i64,
    // d: std::primitive::i32,
    // e: std::primitive::f64,
    // f: std::primitive::f32,
    // g: std::string::String,
    // h: std::primitive::i8,
    // i: std::primitive::bool,
    // j: std::vec::Vec<std::primitive::u8>,
    // k: (),
    // // l: sqlx::types::Decimal,
    // // m: sqlx::types::BigDecimal,
    // // n: sqlx::types::time::Time,
    // o: sqlx::types::time::Date,
    // p: sqlx::types::chrono::NaiveDate,
    // q: sqlx::types::chrono::NaiveDateTime,
    // r: sqlx::types::chrono::NaiveTime,
    // s: sqlx::types::time::OffsetDateTime,
    // t: sqlx::types::time::PrimitiveDateTime,
    // u: core::time::Duration,
    // v: sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    // w: sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
    // x: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    // y: sqlx::types::Uuid,
    // z: sqlx::types::ipnetwork::IpNetwork,
    
//             Self::StdPrimitiveI16 => write!(f, "std::primitive::i16"),
//             Self::StdPrimitiveStr => write!(f, "std::primitive::str"),//todo borrow?
//             Self::StdPrimitiveI64 => write!(f, "std::primitive::i64"),
//             Self::StdPrimitiveI32 => write!(f, "std::primitive::i32"),
//             Self::StdPrimitiveF64 => write!(f, "std::primitive::f64"),
//             Self::StdPrimitiveF32 => write!(f, "std::primitive::f32"),
//             Self::StdStringString => write!(f, "std::string::String"),
//             Self::StdPrimitiveI8 => write!(f, "std::primitive::i8"),
//             Self::StdPrimitiveBool => write!(f, "std::primitive::bool"),
//             Self::StdVecVecStdPrimitiveU8 => write!(f, "std::vec::Vec<std::primitive::u8>"),
//             Self::StdPrimitiveArrayStdPrimitiveU8 => write!(f, "[std::primitive::u8]"),//ignoring constant size 
//             Self::StdPrimitiveUnit => write!(f, "()"),

//             Self::SqlxTypesDecimal => write!(f, "sqlx::types::Decimal"),
//             Self::SqlxTypesBigDecimal => write!(f, "sqlx::types::BigDecimal"),
//             Self::SqlxTypesTimeTime => write!(f, "sqlx::types::time::Time"),
//             Self::SqlxTypesTimeDate => write!(f, "sqlx::types::time::Date"),
//             Self::SqlxTypesChronoNaiveDate => write!(f, "sqlx::types::chrono::NaiveDate"),
//             Self::SqlxTypesChronoNaiveDateTime => write!(f, "sqlx::types::chrono::NaiveDateTime"),
//             Self::SqlxTypesChronoNaiveTime => write!(f, "sqlx::types::chrono::NaiveTime"),
//             Self::SqlxTypesTimeOffsetDateTime => write!(f, "sqlx::types::time::OffsetDateTime"),
//             Self::SqlxTypesTimePrimitiveDateTime => write!(f, "sqlx::types::time::PrimitiveDateTime"),
//             Self::CoreTimeDuration => write!(f, "core::time::Duration"),//todo maybe its std::time::Duration or core::time::Duration or both?
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset => write!(f, "sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>"),
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => write!(f, "sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>"),
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => write!(f, "sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>"),
//             Self::SqlxTypesUuid => write!(f, "sqlx::types::Uuid"),
//             Self::SqlxTypesIpnetworkIpNetwork => write!(f, "sqlx::types::ipnetwork::IpNetwork"),
//             Self::SqlxTypesMacAddressMacAddress => write!(f, "sqlx::types::mac_address::MacAddress"),
//             Self::SqlxPostgresTypesPgInterval => write!(f, "sqlx_postgres::types::PgInterval"),
//             Self::SqlxPostgresTypesPgMoney => write!(f, "sqlx_postgres::types::PgMoney"),
//             Self::SqlxPostgresTypesPgLQuery => write!(f, "sqlx_postgres::types::PgLQuery"),
//             Self::SqlxPostgresTypesPgLTree => write!(f, "sqlx_postgres::types::PgLTree"),
//             Self::SqlxPostgresTypesOid => write!(f, "sqlx_postgres::types::Oid"),
//             Self::SqlxTypesBitVecStdPrimitiveU32 => write!(f, "sqlx::types::BitVec<std::primitive::u32>"),
//             Self::SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesTimeTimeSqlxTypesTimeUtcOffset => write!(f, "sqlx_postgres::types::time_tz::PgTimeTz<sqlx::types::time::Time,sqlx::types::time::UtcOffset>"),
//             Self::SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesChronoNaiveTimeSqlxTypesChronoFixedOffset => write!(f, "sqlx_postgres::types::time_tz::PgTimeTz<sqlx::types::chrono::NaiveTime,sqlx::types::chrono::FixedOffset>"),
//             Self::StdBoxedBoxBorrowStdPrimitiveStr => write!(f, "std::boxed::Box<&std::primitive::str>"),
//             Self::StdBorrowCowAnonymousLifetimeStdPrimitiveStr => write!(f, "std::borrow::Cow<'_, std::primitive::str>"),

//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI32 => write!(f, "sqlx_postgres::types::PgRange<std::primitive::i32>"),
//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI64 => write!(f, "sqlx_postgres::types::PgRange<std::primitive::i64>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::Decimal>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::BigDecimal>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::TimeDate>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDate>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::chrono::FixedOffset>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::ChronoLocal>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::chrono::Utc>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::time::OffsetDateTime>"),
}
