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
    // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
    //maybe Composite types
    //maybe Enumerations
}


// pub trait Type<DB>
// where
//     DB: Database,
// {
//     // Required method
//     fn type_info() -> <DB as Database>::TypeInfo;

//     // Provided method
//     fn compatible(ty: &<DB as Database>::TypeInfo) -> bool { ... }
// }
//new type pattern
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveBool(pub std::primitive::bool);
impl StdPrimitiveBool {
    pub fn into_inner(self) -> std::primitive::bool {
        self.0
    }
}
impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
    fn from(value: StdPrimitiveBool) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBool {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBool {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(self.0 as u8);
        sqlx::encode::IsNull::No
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI8(std::primitive::i8);
impl StdPrimitiveI8 {
    pub fn into_inner(self) -> std::primitive::i8 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI8> for std::primitive::i8 {
    fn from(value: StdPrimitiveI8) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI16(std::primitive::i16);
impl StdPrimitiveI16 {
    pub fn into_inner(self) -> std::primitive::i16 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI16> for std::primitive::i16 {
    fn from(value: StdPrimitiveI16) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI32(std::primitive::i32);
impl StdPrimitiveI32 {
    pub fn into_inner(self) -> std::primitive::i32 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI32> for std::primitive::i32 {
    fn from(value: StdPrimitiveI32) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI64(std::primitive::i64);
impl StdPrimitiveI64 {
    pub fn into_inner(self) -> std::primitive::i64 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI64> for std::primitive::i64 {
    fn from(value: StdPrimitiveI64) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF32(std::primitive::f32);
impl StdPrimitiveF32 {
    pub fn into_inner(self) -> std::primitive::f32 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveF32> for std::primitive::f32 {
    fn from(value: StdPrimitiveF32) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF64(std::primitive::f64);
impl StdPrimitiveF64 {
    pub fn into_inner(self) -> std::primitive::f64 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveF64> for std::primitive::f64 {
    fn from(value: StdPrimitiveF64) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdStringString(std::string::String);
impl StdStringString {
    pub fn into_inner(self) -> std::string::String {
        self.0
    }
}
impl std::convert::From<StdStringString> for std::string::String {
    fn from(value: StdStringString) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdVecVecStdPrimitiveU8(std::vec::Vec<std::primitive::u8>);
impl StdVecVecStdPrimitiveU8 {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::u8> {
        self.0
    }
}
impl std::convert::From<StdVecVecStdPrimitiveU8> for std::vec::Vec<std::primitive::u8> {
    fn from(value: StdVecVecStdPrimitiveU8) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Unit(());
impl Unit {
    pub fn into_inner(self) -> () {
        self.0
    }
}
impl std::convert::From<Unit> for () {
    fn from(value: Unit) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval);
impl SqlxPostgresTypesPgInterval {
    pub fn into_inner(self) -> sqlx::postgres::types::PgInterval {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgInterval> for sqlx::postgres::types::PgInterval {
    fn from(value: SqlxPostgresTypesPgInterval) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange<std::primitive::i64>);
impl SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<std::primitive::i64> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64> for sqlx::postgres::types::PgRange<std::primitive::i64> {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange<std::primitive::i32>);
impl SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<std::primitive::i32> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32> for sqlx::postgres::types::PgRange<std::primitive::i32> {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc> for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>);
impl SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime> for sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>>);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset> for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal> for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime> for sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate> for sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange<sqlx::types::time::Date>);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::Date> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeDate> for sqlx::postgres::types::PgRange<sqlx::types::time::Date> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeDate) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal> for sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange<sqlx::types::Decimal>);
impl SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::Decimal> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimal> for sqlx::postgres::types::PgRange<sqlx::types::Decimal> {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesDecimal) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney);
impl SqlxPostgresTypesPgMoney {
    pub fn into_inner(self) -> sqlx::postgres::types::PgMoney {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgMoney> for sqlx::postgres::types::PgMoney {
    fn from(value: SqlxPostgresTypesPgMoney) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgLTree(sqlx::postgres::types::PgLTree);
impl SqlxPostgresTypesPgLTree {
    pub fn into_inner(self) -> sqlx::postgres::types::PgLTree {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgLTree> for sqlx::postgres::types::PgLTree {
    fn from(value: SqlxPostgresTypesPgLTree) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgLQuery(sqlx::postgres::types::PgLQuery);
impl SqlxPostgresTypesPgLQuery {
    pub fn into_inner(self) -> sqlx::postgres::types::PgLQuery {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgLQuery> for sqlx::postgres::types::PgLQuery {
    fn from(value: SqlxPostgresTypesPgLQuery) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgCiText(sqlx::postgres::types::PgCiText);
impl SqlxPostgresTypesPgCiText {
    pub fn into_inner(self) -> sqlx::postgres::types::PgCiText {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgCiText> for sqlx::postgres::types::PgCiText {
    fn from(value: SqlxPostgresTypesPgCiText) -> Self {
        value.0
    }
}
pub struct SqlxTypesBigDecimal(sqlx::types::BigDecimal);
impl SqlxTypesBigDecimal {
    pub fn into_inner(self) -> sqlx::types::BigDecimal {
        self.0
    }
}
impl std::convert::From<SqlxTypesBigDecimal> for sqlx::types::BigDecimal {
    fn from(value: SqlxTypesBigDecimal) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesDecimal(sqlx::types::Decimal);
impl SqlxTypesDecimal {
    pub fn into_inner(self) -> sqlx::types::Decimal {
        self.0
    }
}
impl std::convert::From<SqlxTypesDecimal> for sqlx::types::Decimal {
    fn from(value: SqlxTypesDecimal) -> Self {
        value.0
    }
}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>);
impl SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset> for sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> {
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset) -> Self {
        value.0
    }
}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>);
impl SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoLocal> for sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> {
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        value.0
    }
}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoUtc> for sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> {
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        value.0
    }
}
pub struct SqlxTypesChronoNaiveDate(sqlx::types::chrono::NaiveDate);
impl SqlxTypesChronoNaiveDate {
    pub fn into_inner(self) -> sqlx::types::chrono::NaiveDate {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoNaiveDate> for sqlx::types::chrono::NaiveDate {
    fn from(value: SqlxTypesChronoNaiveDate) -> Self {
        value.0
    }
}
pub struct SqlxTypesChronoNaiveTime(sqlx::types::chrono::NaiveTime);
impl SqlxTypesChronoNaiveTime {
    pub fn into_inner(self) -> sqlx::types::chrono::NaiveTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoNaiveTime> for sqlx::types::chrono::NaiveTime {
    fn from(value: SqlxTypesChronoNaiveTime) -> Self {
        value.0
    }
}
pub struct SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz);
impl SqlxPostgresTypesPgTimeTz {
    pub fn into_inner(self) -> sqlx::postgres::types::PgTimeTz {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgTimeTz> for sqlx::postgres::types::PgTimeTz {
    fn from(value: SqlxPostgresTypesPgTimeTz) -> Self {
        value.0
    }
}
pub struct SqlxTypesTimePrimitiveDateTime(sqlx::types::time::PrimitiveDateTime);
impl SqlxTypesTimePrimitiveDateTime {
    pub fn into_inner(self) -> sqlx::types::time::PrimitiveDateTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimePrimitiveDateTime> for sqlx::types::time::PrimitiveDateTime {
    fn from(value: SqlxTypesTimePrimitiveDateTime) -> Self {
        value.0
    }
}
pub struct SqlxTypesTimeOffsetDateTime(sqlx::types::time::OffsetDateTime);
impl SqlxTypesTimeOffsetDateTime {
    pub fn into_inner(self) -> sqlx::types::time::OffsetDateTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimeOffsetDateTime> for sqlx::types::time::OffsetDateTime {
    fn from(value: SqlxTypesTimeOffsetDateTime) -> Self {
        value.0
    }
}
pub struct SqlxTypesTimeDate(sqlx::types::time::Date);
impl SqlxTypesTimeDate {
    pub fn into_inner(self) -> sqlx::types::time::Date {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimeDate> for sqlx::types::time::Date {
    fn from(value: SqlxTypesTimeDate) -> Self {
        value.0
    }
}
pub struct SqlxTypesTimeTime(sqlx::types::time::Time);
impl SqlxTypesTimeTime {
    pub fn into_inner(self) -> sqlx::types::time::Time {
        self.0
    }
}
impl std::convert::From<SqlxTypesTimeTime> for sqlx::types::time::Time {
    fn from(value: SqlxTypesTimeTime) -> Self {
        value.0
    }
}
pub struct SqlxTypesUuidUuid(sqlx::types::uuid::Uuid);
impl SqlxTypesUuidUuid {
    pub fn into_inner(self) -> sqlx::types::uuid::Uuid {
        self.0
    }
}
impl std::convert::From<SqlxTypesUuidUuid> for sqlx::types::uuid::Uuid {
    fn from(value: SqlxTypesUuidUuid) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesIpnetworkIpNetwork(sqlx::types::ipnetwork::IpNetwork);
impl SqlxTypesIpnetworkIpNetwork {
    pub fn into_inner(self) -> sqlx::types::ipnetwork::IpNetwork {
        self.0
    }
}
impl std::convert::From<SqlxTypesIpnetworkIpNetwork> for sqlx::types::ipnetwork::IpNetwork {
    fn from(value: SqlxTypesIpnetworkIpNetwork) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdNetIpAddr(std::net::IpAddr);
impl StdNetIpAddr {
    pub fn into_inner(self) -> std::net::IpAddr {
        self.0
    }
}
impl std::convert::From<StdNetIpAddr> for std::net::IpAddr {
    fn from(value: StdNetIpAddr) -> Self {
        value.0
    }
}
pub struct SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress);
impl SqlxTypesMacAddressMacAddress {
    pub fn into_inner(self) -> sqlx::types::mac_address::MacAddress {
        self.0
    }
}
impl std::convert::From<SqlxTypesMacAddressMacAddress> for sqlx::types::mac_address::MacAddress {
    fn from(value: SqlxTypesMacAddressMacAddress) -> Self {
        value.0
    }
}
pub struct SqlxTypesBitVec(sqlx::types::BitVec);
impl SqlxTypesBitVec {
    pub fn into_inner(self) -> sqlx::types::BitVec {
        self.0
    }
}
impl std::convert::From<SqlxTypesBitVec> for sqlx::types::BitVec {
    fn from(value: SqlxTypesBitVec) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesJson<T>(sqlx::types::Json<T>);
impl<T> SqlxTypesJson<T> {
    pub fn into_inner(self) -> sqlx::types::Json<T> {
        self.0
    }
}
impl<T> std::convert::From<SqlxTypesJson<T>> for sqlx::types::Json<T> {
    fn from(value: SqlxTypesJson<T>) -> Self {
        value.0
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerdeJsonValue(serde_json::Value);
impl SerdeJsonValue {
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}
impl std::convert::From<SerdeJsonValue> for serde_json::Value {
    fn from(value: SerdeJsonValue) -> Self {
        value.0
    }
}

pub async fn something() {
    let mut query = sqlx::query::<sqlx::Postgres>("test");
    query = query.bind(Into::<bool>::into(StdPrimitiveBool(false)));
    query = query.bind(StdPrimitiveBool(false).into_inner());
    query = query.bind(StdPrimitiveBool(false));
}