pub use ::naming_constants::COMMIT;
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

pub mod app_state;
pub mod json_value_extractor;

pub trait IntoSerdeSerializeDeserialize {}

pub trait PostgresqlFilter {}

// impl PostgresqlFilter for sqlx::types:: {}

pub trait PostgresqlOrder {}
impl PostgresqlOrder for std::primitive::bool {} //BOOL
impl PostgresqlOrder for std::primitive::i16 {} //SMALLINT,SMALLSERIAL,INT2
impl PostgresqlOrder for std::primitive::i32 {} //INT,SERIAL,INT4
impl PostgresqlOrder for std::primitive::i64 {} //BIGINT,BIGSERIAL,INT8
impl PostgresqlOrder for sqlx::types::BigDecimal {} //NUMERIC
impl PostgresqlOrder for std::primitive::f32 {} //REAL,FLOAT4
impl PostgresqlOrder for std::primitive::f64 {} //DOUBLE PRECISION,FLOAT8
impl PostgresqlOrder for std::primitive::i8 {} //CHAR
impl PostgresqlOrder for std::primitive::str {} //VARCHAR,CHAR(N),TEXT,NAME,CITEXT
impl PostgresqlOrder for std::string::String {} //VARCHAR,CHAR(N),TEXT,NAME,CITEXT
impl PostgresqlOrder for chrono::NaiveDate {} //DATE
impl PostgresqlOrder for sqlx::types::time::Date {} //DATE
impl PostgresqlOrder for chrono::NaiveTime {} //TIME
impl PostgresqlOrder for sqlx::types::time::Time {} //TIME
impl PostgresqlOrder for chrono::NaiveDateTime {} //TIMESTAMP
impl PostgresqlOrder for sqlx::types::time::PrimitiveDateTime {} //TIMESTAMP
impl PostgresqlOrder for sqlx::postgres::types::PgInterval {} //INTERVAL
impl PostgresqlOrder for sqlx::types::BitVec {} //BIT,VARBIT
                                                //todo arrays, json and maybe something else...

pub trait PostgresqlLimit {}

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
{
}

pub trait PostgresqlSerdeSerialize<T: serde::Serialize> {
    fn serde_serialize() -> T;
}

struct Test<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    type_1: std::primitive::bool, //BOOL
    type_2: std::primitive::i8,   //“CHAR”
    type_3: std::primitive::i16,  //SMALLINT, SMALLSERIAL, INT2
    type_4: std::primitive::i32,  //INT, SERIAL, INT4
    type_5: std::primitive::i64,  //BIGINT, BIGSERIAL, INT8
    type_6: std::primitive::f32,  //REAL, FLOAT4
    type_7: std::primitive::f64,  //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    type_9: std::string::String, //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    type_11: std::vec::Vec<std::primitive::u8>, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    type_13: sqlx::postgres::types::PgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    type_14: sqlx::postgres::types::PgRange<std::primitive::i64>, //INT8RANGE
    type_15: sqlx::postgres::types::PgRange<std::primitive::i32>, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    type_161:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, //TSRANGE
    type_162: sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>, //maybe not correct//TSRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    type_171: sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    >, //TSTZRANGE
    type_172:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>, //TSTZRANGE
    type_173: sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>, //maybe not correct//TSTZRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    type_181: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>, //maybe not correct//DATERANGE
    type_182: sqlx::postgres::types::PgRange<sqlx::types::time::Date>, //maybe not correct//DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    type_191: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>, //NUMRANGE
    type_192: sqlx::postgres::types::PgRange<sqlx::types::Decimal>,    //NUMRANGE
    type_20: sqlx::postgres::types::PgMoney,                           //MONEY
    type_21: sqlx::postgres::types::PgLTree,                           //LTREE
    type_22: sqlx::postgres::types::PgLQuery,                          //LQUERY
    type_23: sqlx::postgres::types::PgCiText,                          //CITEXT
    type_24: sqlx::types::BigDecimal,                                  //NUMERIC
    type_25: sqlx::types::Decimal,                                     //NUMERIC
    type_26: sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>, //TIMESTAMPTZ
    type_27: sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>, //TIMESTAMPTZ
    type_28: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,  //TIMESTAMP
    type_29: sqlx::types::chrono::NaiveDate,                           //DATE
    type_30: sqlx::types::chrono::NaiveTime,                           //TIME
    type_31: sqlx::postgres::types::PgTimeTz, //just present chrono or time flag
    // type_31: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    type_32: sqlx::types::time::PrimitiveDateTime, //TIMESTAMP
    type_33: sqlx::types::time::OffsetDateTime,    //TIMESTAMPTZ
    type_34: sqlx::types::time::Date,              //DATE
    type_35: sqlx::types::time::Time,              //TIME
    // type_36: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    type_37: sqlx::types::uuid::Uuid,              //UUID
    type_38: sqlx::types::ipnetwork::IpNetwork,    //INET, CIDR
    type_39: std::net::IpAddr,                     //INET, CIDR
    type_40: sqlx::types::mac_address::MacAddress, //MACADDR
    type_41: sqlx::types::BitVec,                  //BIT, VARBIT
    type_42: sqlx::types::Json<T>,                 //JSON, JSONB
    type_43: serde_json::Value,                    //JSON, JSONB
                                                   // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
                                                   //maybe Composite types
                                                   //maybe Enumerations
}

struct TestWrapper<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    type_1: StdPrimitiveBool, //BOOL
    type_2: StdPrimitiveI8,   //“CHAR”
    type_3: StdPrimitiveI16,  //SMALLINT, SMALLSERIAL, INT2
    type_4: StdPrimitiveI32,  //INT, SERIAL, INT4
    type_5: StdPrimitiveI64,  //BIGINT, BIGSERIAL, INT8
    type_6: StdPrimitiveF32,  //REAL, FLOAT4
    type_7: StdPrimitiveF64,  //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    type_9: StdStringString, //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    type_11: StdVecVecStdPrimitiveU8, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    type_13: SqlxPostgresTypesPgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    type_14: SqlxPostgresTypesPgRangeStdPrimitiveI64, //INT8RANGE
    type_15: SqlxPostgresTypesPgRangeStdPrimitiveI32, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    type_161: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc, //TSRANGE
    type_162: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime, //maybe not correct//TSRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    type_171: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset, //TSTZRANGE
    type_172: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,       //TSTZRANGE
    type_173: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime, //maybe not correct//TSTZRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    type_181: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate, //maybe not correct//DATERANGE
    type_182: SqlxPostgresTypesPgRangeSqlxTypesTimeDate,        //maybe not correct//DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    type_191: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal, //NUMRANGE
    type_192: SqlxPostgresTypesPgRangeSqlxTypesDecimal,    //NUMRANGE
    type_20: SqlxPostgresTypesPgMoney,                     //MONEY
    type_21: SqlxPostgresTypesPgLTree,                     //LTREE
    type_22: SqlxPostgresTypesPgLQuery,                    //LQUERY
    type_23: SqlxPostgresTypesPgCiText,                    //CITEXT
    type_24: SqlxTypesBigDecimal,                          //NUMERIC
    type_25: SqlxTypesDecimal,                             //NUMERIC
    type_26: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset, //TIMESTAMPTZ
    type_27: SqlxTypesChronoDateTimeSqlxTypesChronoLocal,  //TIMESTAMPTZ
    type_28: SqlxTypesChronoDateTimeSqlxTypesChronoUtc,    //TIMESTAMP
    type_29: SqlxTypesChronoNaiveDate,                     //DATE
    type_30: SqlxTypesChronoNaiveTime,                     //TIME
    type_31: SqlxPostgresTypesPgTimeTz,                    //just present chrono or time flag
    // type_31: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    type_32: SqlxTypesTimePrimitiveDateTime, //TIMESTAMP
    type_33: SqlxTypesTimeOffsetDateTime,    //TIMESTAMPTZ
    type_34: SqlxTypesTimeDate,              //DATE
    type_35: SqlxTypesTimeTime,              //TIME
    // type_36: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    type_37: SqlxTypesUuidUuid,             //UUID
    type_38: SqlxTypesIpnetworkIpNetwork,   //INET, CIDR
    type_39: StdNetIpAddr,                  //INET, CIDR
    type_40: SqlxTypesMacAddressMacAddress, //MACADDR
    type_41: SqlxTypesBitVec,               //BIT, VARBIT
    type_42: SqlxTypesJson<T>,              //JSON, JSONB
    type_43: SerdeJsonValue,                //JSON, JSONB
                                            // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
                                            //maybe Composite types
                                            //maybe Enumerations
}

pub trait CheckSupportedPostgresqlColumnType {
    fn check_supported_postgresql_column_type();
}
//new type pattern
// sqlx::Encode impl was copied from https://docs.rs/sqlx/0.7.3/sqlx/trait.Encode.html
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
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBool {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI8(pub std::primitive::i8);
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
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI8 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::i8 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI8 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI8 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI8 {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
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
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI16 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI16 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI16 {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
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
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI32 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI32 {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
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
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
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
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveF32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::f32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::f32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveF32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveF32 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveF32 {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
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
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveF64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::f64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::f64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveF64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveF64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdPrimitiveF64 {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdStringString(pub std::string::String);
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
impl sqlx::Type<sqlx::Postgres> for StdStringString {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdStringString {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdStringString {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdStringString {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
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
impl sqlx::Type<sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<std::primitive::u8> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<std::primitive::u8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdVecVecStdPrimitiveU8 {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
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
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgInterval {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(
    pub sqlx::postgres::types::PgRange<std::primitive::i64>,
);
impl SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<std::primitive::i64> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64>
    for sqlx::postgres::types::PgRange<std::primitive::i64>
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(
    pub sqlx::postgres::types::PgRange<std::primitive::i32>,
);
impl SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<std::primitive::i32> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32>
    for sqlx::postgres::types::PgRange<std::primitive::i32>
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>
    {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime>
    for sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
    pub  sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    >,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    > {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset>
    for sqlx::postgres::types::PgRange<
        sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    >
{
    fn from(
        value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
    ) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<
            sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
        > as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<
            sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
        > as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset
{
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    pub fn into_inner(
        self,
    ) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>
    {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime>
    for sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::time::Date> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeDate>
    for sqlx::postgres::types::PgRange<sqlx::types::time::Date>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeDate) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::time::Date> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::Date> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal>
    for sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>,
);
impl SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::Decimal> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimal>
    for sqlx::postgres::types::PgRange<sqlx::types::Decimal>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesDecimal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::Decimal> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::Decimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
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
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgMoney {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgMoney as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgMoney as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgMoney {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgMoney {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgMoney {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgLTree(pub sqlx::postgres::types::PgLTree);
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
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgLTree {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgLTree as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgLTree as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLTree {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLTree {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgLTree {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgLQuery(pub sqlx::postgres::types::PgLQuery);
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
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgLQuery {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgLQuery as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgLQuery as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLQuery {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgLQuery {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgLQuery {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
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
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgCiText {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgCiText as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgCiText as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgCiText {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgCiText {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgCiText {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBigDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBigDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBigDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesBigDecimal {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesDecimal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Decimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Decimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesDecimal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesDecimal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesDecimal {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
);
impl SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset>
    for sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>
{
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
);
impl SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>
    for sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>
{
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
);
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    pub fn into_inner(self) -> sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>
    for sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>
{
    fn from(value: SqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveDate {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveTime {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
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
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgTimeTz as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgTimeTz as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgTimeTz {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::PrimitiveDateTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::time::PrimitiveDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimePrimitiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::OffsetDateTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::time::OffsetDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeOffsetDateTime {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimeDate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::Date as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::time::Date as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeDate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeDate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeDate {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesTimeTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::time::Time as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::time::Time as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeTime {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeTime {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeTime {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesUuidUuid {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuid {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuid {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesUuidUuid {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::ipnetwork::IpNetwork as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::ipnetwork::IpNetwork as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesIpnetworkIpNetwork {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdNetIpAddr(pub std::net::IpAddr);
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
impl sqlx::Type<sqlx::Postgres> for StdNetIpAddr {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::net::IpAddr as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::net::IpAddr as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdNetIpAddr {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdNetIpAddr {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for StdNetIpAddr {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::mac_address::MacAddress as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::mac_address::MacAddress as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesMacAddressMacAddress {
    fn check_supported_postgresql_column_type() {}
}
pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBitVec {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBitVec {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBitVec {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SqlxTypesBitVec {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SqlxTypesJson<T>(pub sqlx::types::Json<T>);
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
impl<T> sqlx::Type<sqlx::Postgres> for SqlxTypesJson<T> {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<T> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<T> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'a, T> sqlx::Encode<'a, sqlx::Postgres> for SqlxTypesJson<T>
where
    T: sqlx::Encode<'a, sqlx::Postgres>
        + Copy
        + Clone
        + std::fmt::Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + Default
        + serde::Serialize
        + serde::Deserialize<'a>, //todo maybe add another traits impls
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl<'a, T: 'a> sqlx::Decode<'a, sqlx::Postgres> for SqlxTypesJson<T>
where
    T: sqlx::Decode<'a, sqlx::Postgres>
        + Copy
        + Clone
        + std::fmt::Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + Default
        + serde::Serialize
        + serde::Deserialize<'a>, //todo maybe add another traits impls
{
    fn decode(value: sqlx::postgres::PgValueRef<'a>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl<T> CheckSupportedPostgresqlColumnType for SqlxTypesJson<T> {
    fn check_supported_postgresql_column_type() {}
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerdeJsonValue(pub serde_json::Value);
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
impl sqlx::Type<sqlx::Postgres> for SerdeJsonValue {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SerdeJsonValue {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
    }
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SerdeJsonValue {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match sqlx::Decode::<sqlx::Postgres>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl CheckSupportedPostgresqlColumnType for SerdeJsonValue {
    fn check_supported_postgresql_column_type() {}
}

pub async fn something() {
    let mut query = sqlx::query::<sqlx::Postgres>("test");
    query = query.bind(Into::<bool>::into(StdPrimitiveBool(false)));
    query = query.bind(StdPrimitiveBool(false).into_inner());
    query = query.bind(StdPrimitiveBool(false));
}

pub fn test_check_supported_postgresql_column_type() {
    //
    let std_primitive_bool = StdPrimitiveBool(true);
    let std_primitive_i8 = StdPrimitiveI8(std::primitive::i8::default());
    let std_primitive_i16 = StdPrimitiveI16(std::primitive::i16::default());
    let std_primitive_i32 = StdPrimitiveI32(std::primitive::i32::default());
    let std_primitive_i64 = StdPrimitiveI64(std::primitive::i64::default());
    let std_primitive_f32 = StdPrimitiveF32(std::primitive::f32::default());
    let std_primitive_f64 = StdPrimitiveF64(std::primitive::f64::default());
    let std_string_string = StdStringString(std::string::String::default());
    let std_vec_vec_std_primitive_u8 = StdVecVecStdPrimitiveU8(vec![std::primitive::u8::default()]);
    let sqlx_postgres_types_pg_interval =
        SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval {
            months: std::primitive::i32::default(),
            days: std::primitive::i32::default(),
            microseconds: std::primitive::i64::default(),
        });
    let sqlx_postgres_types_pg_range_std_primitive_i64 =
        SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange::<
            std::primitive::i64,
        > {
            start: std::ops::Bound::<std::primitive::i64>::Included(std::primitive::i64::default()),
            end: std::ops::Bound::<std::primitive::i64>::Included(std::primitive::i64::default()),
        });
    let sqlx_postgres_types_pg_range_std_primitive_i32 =
        SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange::<
            std::primitive::i32,
        > {
            start: std::ops::Bound::<std::primitive::i32>::Included(std::primitive::i32::default()),
            end: std::ops::Bound::<std::primitive::i32>::Included(std::primitive::i32::default()),
        });
    let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
        sqlx::postgres::types::PgRange::<
            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
        > {
            start: std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>::Included(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                sqlx::types::chrono::NaiveDateTime::new(
                    sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap(),//todo
                    sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap(),
                ),
                sqlx::types::chrono::Utc
            )),
            end: std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>::Included(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                sqlx::types::chrono::NaiveDateTime::new(
                    sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap(),//todo
                    sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap(),
                ),
                sqlx::types::chrono::Utc
            )),
        }
        // pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
    );
    let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange::<
            sqlx::types::time::PrimitiveDateTime,
        > {
            start: std::ops::Bound::<sqlx::types::time::PrimitiveDateTime>::Included(
                sqlx::types::time::PrimitiveDateTime::new(
                    sqlx::types::time::Date::from_calendar_date(
                        std::primitive::i32::default(),
                        time::Month::February,
                        std::primitive::u8::default(),
                    )
                    .unwrap(), //todo
                    sqlx::types::time::Time::from_hms(
                        std::primitive::u8::default(),
                        std::primitive::u8::default(),
                        std::primitive::u8::default(),
                    )
                    .unwrap(), //todo
                ),
            ),
            end: std::ops::Bound::<sqlx::types::time::PrimitiveDateTime>::Included(
                sqlx::types::time::PrimitiveDateTime::new(
                    sqlx::types::time::Date::from_calendar_date(
                        std::primitive::i32::default(),
                        time::Month::February,
                        std::primitive::u8::default(),
                    )
                    .unwrap(), //todo
                    sqlx::types::time::Time::from_hms(
                        std::primitive::u8::default(),
                        std::primitive::u8::default(),
                        std::primitive::u8::default(),
                    )
                    .unwrap(), //todo
                ),
            ),
        });
    let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset =
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
            sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
            > {
                start: std::ops::Bound::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
                >::Included(sqlx::types::chrono::DateTime::<
                    sqlx::types::chrono::FixedOffset,
                >::from_naive_utc_and_offset(
                    sqlx::types::chrono::NaiveDateTime::new(
                        sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap(), //todo
                        sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap(),
                    ),
                    sqlx::types::chrono::FixedOffset::west_opt(std::primitive::i32::default())
                        .unwrap(),
                )),
                end: std::ops::Bound::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
                >::Included(sqlx::types::chrono::DateTime::<
                    sqlx::types::chrono::FixedOffset,
                >::from_naive_utc_and_offset(
                    sqlx::types::chrono::NaiveDateTime::new(
                        sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap(), //todo
                        sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap(),
                    ),
                    sqlx::types::chrono::FixedOffset::west_opt(std::primitive::i32::default())
                        .unwrap(),
                )),
            },
        );
    let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
        sqlx::postgres::types::PgRange::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
            start: std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>::Included(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(
                sqlx::types::chrono::NaiveDateTime::new(
                    sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap(),//todo
                    sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap(),
                ),
                sqlx::types::chrono::FixedOffset::west_opt(std::primitive::i32::default()).unwrap()
            )),
            end: std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>::Included(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(
                sqlx::types::chrono::NaiveDateTime::new(
                    sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap(),//todo
                    sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap(),
                ),
                sqlx::types::chrono::FixedOffset::west_opt(std::primitive::i32::default()).unwrap()
            )),
        }
    );
    let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
        SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange::<
            sqlx::types::time::OffsetDateTime,
        > {
            start: std::ops::Bound::<sqlx::types::time::OffsetDateTime>::Included(
                sqlx::types::time::OffsetDateTime::now_utc(),
            ),
            end: std::ops::Bound::<sqlx::types::time::OffsetDateTime>::Included(
                sqlx::types::time::OffsetDateTime::now_utc(),
            ),
        });
    // let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
    //     pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,
    // );
    // let sqlx_postgres_types_pg_range_sqlx_types_time_date = SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
    //     pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>,
    // );
    // let sqlx_postgres_types_pg_range_sqlx_types_big_decimal = SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
    //     pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,
    // );
    // let sqlx_postgres_types_pg_range_sqlx_types_decimal = SqlxPostgresTypesPgRangeSqlxTypesDecimal(
    //     pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>,
    // );
    // let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
    // let sqlx_postgres_types_pg_l_tree = SqlxPostgresTypesPgLTree(pub sqlx::postgres::types::PgLTree);
    // let sqlx_postgres_types_pg_l_query = SqlxPostgresTypesPgLQuery(pub sqlx::postgres::types::PgLQuery);
    // let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
    // let sqlx_types_big_decimal = SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
    // let sqlx_types_decimal = SqlxTypesDecimal(pub sqlx::types::Decimal);
    // let sqlx_types_chrono_date_time_sqlx_types_chrono_fixed_offset = SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset(
    //     pub sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>,
    // );
    // let sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    //     pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
    // );
    // let sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    //     pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    // );
    // let sqlx_types_chrono_naive_date = SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
    // let sqlx_types_chrono_naive_time = SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
    // let sqlx_postgres_types_pg_time_tz = SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
    // let sqlx_types_time_primitive_date_time = SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
    // let sqlx_types_time_offset_date_time = SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
    // let sqlx_types_time_date = SqlxTypesTimeDate(pub sqlx::types::time::Date);
    // let sqlx_types_time_time = SqlxTypesTimeTime(pub sqlx::types::time::Time);
    // let sqlx_types_uuid_uuid = SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
    // let sqlx_types_ipnetwork_ip_network = SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
    // let std_net_ip_addr = StdNetIpAddr(pub std::net::IpAddr);
    // let sqlx_types_mac_address_mac_address = SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
    // let sqlx_types_bit_vec = SqlxTypesBitVec(pub sqlx::types::BitVec);
    // let sqlx_types_json = SqlxTypesJson<T>(pub sqlx::types::Json<T>);
    // let serde_json_value = SerdeJsonValue(pub serde_json::Value);
    //
    //
    StdPrimitiveBool::check_supported_postgresql_column_type();
    StdPrimitiveI8::check_supported_postgresql_column_type();
    StdPrimitiveI16::check_supported_postgresql_column_type();
    StdPrimitiveI32::check_supported_postgresql_column_type();
    StdPrimitiveI64::check_supported_postgresql_column_type();
    StdPrimitiveF32::check_supported_postgresql_column_type();
    StdPrimitiveF64::check_supported_postgresql_column_type();
    StdStringString::check_supported_postgresql_column_type();
    StdVecVecStdPrimitiveU8::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgInterval::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeStdPrimitiveI64::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeStdPrimitiveI32::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::check_supported_postgresql_column_type(
    );
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesDecimal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgMoney::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgLTree::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgLQuery::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgCiText::check_supported_postgresql_column_type();
    SqlxTypesBigDecimal::check_supported_postgresql_column_type();
    SqlxTypesDecimal::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc::check_supported_postgresql_column_type();
    SqlxTypesChronoNaiveDate::check_supported_postgresql_column_type();
    SqlxTypesChronoNaiveTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgTimeTz::check_supported_postgresql_column_type();
    SqlxTypesTimePrimitiveDateTime::check_supported_postgresql_column_type();
    SqlxTypesTimeOffsetDateTime::check_supported_postgresql_column_type();
    SqlxTypesTimeDate::check_supported_postgresql_column_type();
    SqlxTypesTimeTime::check_supported_postgresql_column_type();
    SqlxTypesUuidUuid::check_supported_postgresql_column_type();
    SqlxTypesIpnetworkIpNetwork::check_supported_postgresql_column_type();
    StdNetIpAddr::check_supported_postgresql_column_type();
    SqlxTypesMacAddressMacAddress::check_supported_postgresql_column_type();
    SqlxTypesBitVec::check_supported_postgresql_column_type();
    SqlxTypesJson::<bool>::check_supported_postgresql_column_type();
    SerdeJsonValue::check_supported_postgresql_column_type();
    //
}
