pub const POSTGRESQL_CRUD_SNAKE_CASE: &str = "postgresql_crud";

pub enum PostgresqlType {
    Bool,
    Char,
    SmallInt,
    SmallSerial,
    Int2,
    Int,
    Serial,
    Int4,
    BigInt,
    BigSerial,
    Int8,
    Real,
    Float4,
    DoublePrecision,
    Float8,
    Varchar,
    CharN,
    Text,
    CiText,
    Bytea,
    Interval,
    Int8Range,
    Int4Range,
    TsRange,
    TsTzRange,
    DateRange,
    NumRange,
    Money,
    Numeric,
    TimestampTz,
    Date,
    Time,
    TimeTz,
    Timestamp,
    Uuid,
    Inet,
    Cidr,
    MacAddr,
    Bit,
    VarBit,
    Json,
    JsonB,
}

pub enum PostgresqlTypeWithMetadata {
    Bool,
    BoolNotNull,
    Char,
    CharNotNull,
    SmallInt,
    SmallIntNotNull,
    SmallSerial,
    SmallSerialNotNull,
    Int2,
    Int2NotNull,
    Int,
    IntNotNull,
    Serial,
    SerialNotNull,
    Int4,
    Int4NotNull,
    BigInt,
    BigIntNotNull,
    BigSerial,
    BigSerialNotNull,
    BigSerialNotNullPrimaryKey,
    Int8,
    Int8NotNull,
    Real,
    RealNotNull,
    Float4,
    Float4NotNull,
    DoublePrecision,
    DoublePrecisionNotNull,
    Float8,
    Float8NotNull,
    Varchar,
    VarcharNotNull,
    CharN,
    CharNNotNull,
    Text,
    TextNotNull,
    CiText,
    CiTextNotNull,
    Bytea,
    ByteaNotNull,
    Interval,
    IntervalNotNull,
    Int8Range,
    Int8RangeNotNull,
    Int4Range,
    Int4RangeNotNull,
    TsRange,
    TsRangeNotNull,
    TsTzRange,
    TsTzRangeNotNull,
    DateRange,
    DateRangeNotNull,
    NumRange,
    NumRangeNotNull,
    Money,
    MoneyNotNull,
    Numeric,
    NumericNotNull,
    TimestampTz,
    TimestampTzNotNull,
    Date,
    DateNotNull,
    Time,
    TimeNotNull,
    TimeTz,
    TimeTzNotNull,
    Timestamp,
    TimestampNotNull,
    Uuid,
    UuidNotNull,
    UuidNotNullPrimaryKey,
    Inet,
    InetNotNull,
    Cidr,
    CidrNotNull,
    MacAddr,
    MacAddrNotNull,
    Bit,
    BitNotNull,
    VarBit,
    VarBitNotNull,
    Json,
    JsonNotNull,
    JsonB,
    JsonBNotNull,
}

impl std::convert::From<&PostgresqlTypeWithMetadata> for PostgresqlType {
    fn from(value: &PostgresqlTypeWithMetadata) -> Self {
        match value {
            PostgresqlTypeWithMetadata::Bool => Self::Bool,
            PostgresqlTypeWithMetadata::BoolNotNull => Self::Bool,
            PostgresqlTypeWithMetadata::Char => Self::Char,
            PostgresqlTypeWithMetadata::CharNotNull => Self::Char,
            PostgresqlTypeWithMetadata::SmallInt => Self::SmallInt,
            PostgresqlTypeWithMetadata::SmallIntNotNull => Self::SmallInt,
            PostgresqlTypeWithMetadata::SmallSerial => Self::SmallSerial,
            PostgresqlTypeWithMetadata::SmallSerialNotNull => Self::SmallSerial,
            PostgresqlTypeWithMetadata::Int2 => Self::Int2,
            PostgresqlTypeWithMetadata::Int2NotNull => Self::Int2,
            PostgresqlTypeWithMetadata::Int => Self::Int,
            PostgresqlTypeWithMetadata::IntNotNull => Self::Int,
            PostgresqlTypeWithMetadata::Serial => Self::Serial,
            PostgresqlTypeWithMetadata::SerialNotNull => Self::Serial,
            PostgresqlTypeWithMetadata::Int4 => Self::Int4,
            PostgresqlTypeWithMetadata::Int4NotNull => Self::Int4,
            PostgresqlTypeWithMetadata::BigInt => Self::BigInt,
            PostgresqlTypeWithMetadata::BigIntNotNull => Self::BigInt,
            PostgresqlTypeWithMetadata::BigSerial => Self::BigSerial,
            PostgresqlTypeWithMetadata::BigSerialNotNull => Self::BigSerial,
            PostgresqlTypeWithMetadata::BigSerialNotNullPrimaryKey => Self::BigSerial,
            PostgresqlTypeWithMetadata::Int8 => Self::Int8,
            PostgresqlTypeWithMetadata::Int8NotNull => Self::Int8,
            PostgresqlTypeWithMetadata::Real => Self::Real,
            PostgresqlTypeWithMetadata::RealNotNull => Self::Real,
            PostgresqlTypeWithMetadata::Float4 => Self::Float4,
            PostgresqlTypeWithMetadata::Float4NotNull => Self::Float4,
            PostgresqlTypeWithMetadata::DoublePrecision => Self::DoublePrecision,
            PostgresqlTypeWithMetadata::DoublePrecisionNotNull => Self::DoublePrecision,
            PostgresqlTypeWithMetadata::Float8 => Self::Float8,
            PostgresqlTypeWithMetadata::Float8NotNull => Self::Float8,
            PostgresqlTypeWithMetadata::Varchar => Self::Varchar,
            PostgresqlTypeWithMetadata::VarcharNotNull => Self::Varchar,
            PostgresqlTypeWithMetadata::CharN => Self::CharN,
            PostgresqlTypeWithMetadata::CharNNotNull => Self::CharN,
            PostgresqlTypeWithMetadata::Text => Self::Text,
            PostgresqlTypeWithMetadata::TextNotNull => Self::Text,
            PostgresqlTypeWithMetadata::CiText => Self::CiText,
            PostgresqlTypeWithMetadata::CiTextNotNull => Self::CiText,
            PostgresqlTypeWithMetadata::Bytea => Self::Bytea,
            PostgresqlTypeWithMetadata::ByteaNotNull => Self::Bytea,
            PostgresqlTypeWithMetadata::Interval => Self::Interval,
            PostgresqlTypeWithMetadata::IntervalNotNull => Self::Interval,
            PostgresqlTypeWithMetadata::Int8Range => Self::Int8Range,
            PostgresqlTypeWithMetadata::Int8RangeNotNull => Self::Int8Range,
            PostgresqlTypeWithMetadata::Int4Range => Self::Int4Range,
            PostgresqlTypeWithMetadata::Int4RangeNotNull => Self::Int4Range,
            PostgresqlTypeWithMetadata::TsRange => Self::TsRange,
            PostgresqlTypeWithMetadata::TsRangeNotNull => Self::TsRange,
            PostgresqlTypeWithMetadata::TsTzRange => Self::TsTzRange,
            PostgresqlTypeWithMetadata::TsTzRangeNotNull => Self::TsTzRange,
            PostgresqlTypeWithMetadata::DateRange => Self::DateRange,
            PostgresqlTypeWithMetadata::DateRangeNotNull => Self::DateRange,
            PostgresqlTypeWithMetadata::NumRange => Self::NumRange,
            PostgresqlTypeWithMetadata::NumRangeNotNull => Self::NumRange,
            PostgresqlTypeWithMetadata::Money => Self::Money,
            PostgresqlTypeWithMetadata::MoneyNotNull => Self::Money,
            PostgresqlTypeWithMetadata::Numeric => Self::Numeric,
            PostgresqlTypeWithMetadata::NumericNotNull => Self::Numeric,
            PostgresqlTypeWithMetadata::TimestampTz => Self::TimestampTz,
            PostgresqlTypeWithMetadata::TimestampTzNotNull => Self::TimestampTz,
            PostgresqlTypeWithMetadata::Date => Self::Date,
            PostgresqlTypeWithMetadata::DateNotNull => Self::Date,
            PostgresqlTypeWithMetadata::Time => Self::Time,
            PostgresqlTypeWithMetadata::TimeNotNull => Self::Time,
            PostgresqlTypeWithMetadata::TimeTz => Self::TimeTz,
            PostgresqlTypeWithMetadata::TimeTzNotNull => Self::TimeTz,
            PostgresqlTypeWithMetadata::Timestamp => Self::Timestamp,
            PostgresqlTypeWithMetadata::TimestampNotNull => Self::Timestamp,
            PostgresqlTypeWithMetadata::Uuid => Self::Uuid,
            PostgresqlTypeWithMetadata::UuidNotNull => Self::Uuid,
            PostgresqlTypeWithMetadata::UuidNotNullPrimaryKey => Self::Uuid,
            PostgresqlTypeWithMetadata::Inet => Self::Inet,
            PostgresqlTypeWithMetadata::InetNotNull => Self::Inet,
            PostgresqlTypeWithMetadata::Cidr => Self::Cidr,
            PostgresqlTypeWithMetadata::CidrNotNull => Self::Cidr,
            PostgresqlTypeWithMetadata::MacAddr => Self::MacAddr,
            PostgresqlTypeWithMetadata::MacAddrNotNull => Self::MacAddr,
            PostgresqlTypeWithMetadata::Bit => Self::Bit,
            PostgresqlTypeWithMetadata::BitNotNull => Self::Bit,
            PostgresqlTypeWithMetadata::VarBit => Self::VarBit,
            PostgresqlTypeWithMetadata::VarBitNotNull => Self::VarBit,
            PostgresqlTypeWithMetadata::Json => Self::Json,
            PostgresqlTypeWithMetadata::JsonNotNull => Self::Json,
            PostgresqlTypeWithMetadata::JsonB => Self::JsonB,
            PostgresqlTypeWithMetadata::JsonBNotNull => Self::JsonB,
        }
    }
}

impl PostgresqlTypeWithMetadata {
    //todo add NOT NULL or not? or add different method and Primary Key
    pub fn postgresql_naming(&self) -> &str {
        match self {
            Self::Bool => "BOOL",
            Self::BoolNotNull => "BOOL",
            Self::Char => "CHAR",
            Self::CharNotNull => "CHAR",
            Self::SmallInt => "SMALLINT",
            Self::SmallIntNotNull => "SMALLINT",
            Self::SmallSerial => "SMALLSERIAL",
            Self::SmallSerialNotNull => "SMALLSERIAL",
            Self::Int2 => "INT2",
            Self::Int2NotNull => "INT2",
            Self::Int => "INT",
            Self::IntNotNull => "INT",
            Self::Serial => "SERIAL",
            Self::SerialNotNull => "SERIAL",
            Self::Int4 => "INT4",
            Self::Int4NotNull => "INT4",
            Self::BigInt => "BIGINT",
            Self::BigIntNotNull => "BIGINT",
            Self::BigSerial => "BIGSERIAL",
            Self::BigSerialNotNull => "BIGSERIAL",
            Self::BigSerialNotNullPrimaryKey => "BIGSERIAL",
            Self::Int8 => "INT8",
            Self::Int8NotNull => "INT8",
            Self::Real => "REAL",
            Self::RealNotNull => "REAL",
            Self::Float4 => "FLOAT4",
            Self::Float4NotNull => "FLOAT4",
            Self::DoublePrecision => "DOUBLE PRECISION",
            Self::DoublePrecisionNotNull => "DOUBLE PRECISION",
            Self::Float8 => "FLOAT8",
            Self::Float8NotNull => "FLOAT8",
            Self::Varchar => "VARCHAR",
            Self::VarcharNotNull => "VARCHAR",
            Self::CharN => "CHAR(N)",
            Self::CharNNotNull => "CHAR(N)",
            Self::Text => "TEXT",
            Self::TextNotNull => "TEXT",
            Self::CiText => "CITEXT",
            Self::CiTextNotNull => "CITEXT",
            Self::Bytea => "BYTEA",
            Self::ByteaNotNull => "BYTEA",
            Self::Interval => "INTERVAL",
            Self::IntervalNotNull => "INTERVAL",
            Self::Int8Range => "INT8RANGE",
            Self::Int8RangeNotNull => "INT8RANGE",
            Self::Int4Range => "INT4RANGE",
            Self::Int4RangeNotNull => "INT4RANGE",
            Self::TsRange => "TSRANGE",
            Self::TsRangeNotNull => "TSRANGE",
            Self::TsTzRange => "TSTZRANGE",
            Self::TsTzRangeNotNull => "TSTZRANGE",
            Self::DateRange => "DATERANGE",
            Self::DateRangeNotNull => "DATERANGE",
            Self::NumRange => "NUMRANGE",
            Self::NumRangeNotNull => "NUMRANGE",
            Self::Money => "MONEY",
            Self::MoneyNotNull => "MONEY",
            Self::Numeric => "NUMERIC",
            Self::NumericNotNull => "NUMERIC",
            Self::TimestampTz => "TIMESTAMPTZ",
            Self::TimestampTzNotNull => "TIMESTAMPTZ",
            Self::Date => "DATE",
            Self::DateNotNull => "DATE",
            Self::Time => "TIME",
            Self::TimeNotNull => "TIME",
            Self::TimeTz => "TIMETZ",
            Self::TimeTzNotNull => "TIMETZ",
            Self::Timestamp => "TIMESTAMP",
            Self::TimestampNotNull => "TIMESTAMP",
            Self::Uuid => "UUID",
            Self::UuidNotNull => "UUID",
            Self::UuidNotNullPrimaryKey => "UUID",
            Self::Inet => "INET",
            Self::InetNotNull => "INET",
            Self::Cidr => "CIDR",
            Self::CidrNotNull => "CIDR",
            Self::MacAddr => "MACADDR",
            Self::MacAddrNotNull => "MACADDR",
            Self::Bit => "BIT",
            Self::BitNotNull => "BIT",
            Self::VarBit => "VARBIT",
            Self::VarBitNotNull => "VARBIT",
            Self::Json => "JSON",
            Self::JsonNotNull => "JSON",
            Self::JsonB => "JSONB",
            Self::JsonBNotNull => "JSONB",
        }
    }
}

#[derive(
    strum_macros::Display,
    strum_macros::EnumIter,
    proc_macro_assistants::ToSnakeCaseStringified,
)]
pub enum SupportedSqlxPostgresType {
    StdPrimitiveBool,
    StdPrimitiveI16,
    StdPrimitiveI32,
    StdPrimitiveI64,
    StdPrimitiveF32,
    StdPrimitiveF64,
    StdStringString,
    StdVecVecStdPrimitiveU8,
    SqlxPostgresTypesPgInterval,
    SqlxPostgresTypesPgRangeStdPrimitiveI64,
    SqlxPostgresTypesPgRangeStdPrimitiveI32,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    SqlxPostgresTypesPgMoney,
    SqlxPostgresTypesPgCiText,
    SqlxTypesBigDecimal,
    SqlxTypesDecimal,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxTypesChronoNaiveDateTime,
    SqlxTypesChronoNaiveDate,
    SqlxTypesChronoNaiveTime,
    SqlxPostgresTypesPgTimeTz,
    SqlxTypesTimePrimitiveDateTime,
    SqlxTypesTimeOffsetDateTime,
    SqlxTypesTimeDate,
    SqlxTypesTimeTime,
    SqlxTypesUuidUuid,
    SqlxTypesIpnetworkIpNetwork,
    StdNetIpAddr,
    SqlxTypesMacAddressMacAddress,
    SqlxTypesBitVec,
    SqlxTypesJsonT, //todo what to do with generic?
    SerdeJsonValue,
}
fn add_path(value: &str) -> std::string::String {
    format!("{POSTGRESQL_CRUD_SNAKE_CASE}::{value}")
}
impl SupportedSqlxPostgresType {
    fn get_original_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        match self {
            Self::StdPrimitiveBool => std::string::String::from("std::primitive::bool"),//todo maybe Option<T> for nullable ?
            Self::StdPrimitiveI16 => std::string::String::from("std::primitive::i16"),
            Self::StdPrimitiveI32 => std::string::String::from("std::primitive::i32"),
            Self::StdPrimitiveI64 => std::string::String::from("std::primitive::i64"),
            Self::StdPrimitiveF32 => std::string::String::from("std::primitive::f32"),
            Self::StdPrimitiveF64 => std::string::String::from("std::primitive::f64"),
            Self::StdStringString => std::string::String::from("std::string::String"),
            Self::StdVecVecStdPrimitiveU8 => std::string::String::from("std::vec::Vec<std::primitive::u8>"),
            Self::SqlxPostgresTypesPgInterval => std::string::String::from("sqlx::postgres::types::PgInterval"),
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64 => std::string::String::from("sqlx::postgres::types::PgRange<std::primitive::i64>"),
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32 => std::string::String::from("sqlx::postgres::types::PgRange<std::primitive::i32>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::Date>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::Decimal>"),
            Self::SqlxPostgresTypesPgMoney => std::string::String::from("sqlx::postgres::types::PgMoney"),
            Self::SqlxPostgresTypesPgCiText => std::string::String::from("sqlx::postgres::types::PgCiText"),
            Self::SqlxTypesBigDecimal => std::string::String::from("sqlx::types::BigDecimal"),
            Self::SqlxTypesDecimal => std::string::String::from("sqlx::types::Decimal"),
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => std::string::String::from("sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>"),
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => std::string::String::from("sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>"),
            Self::SqlxTypesChronoNaiveDateTime => std::string::String::from("sqlx::types::chrono::NaiveDateTime"),
            Self::SqlxTypesChronoNaiveDate => std::string::String::from("sqlx::types::chrono::NaiveDate"),
            Self::SqlxTypesChronoNaiveTime => std::string::String::from("sqlx::types::chrono::NaiveTime"),
            Self::SqlxPostgresTypesPgTimeTz => std::string::String::from("sqlx::postgres::types::PgTimeTz"),
            Self::SqlxTypesTimePrimitiveDateTime => std::string::String::from("sqlx::types::time::PrimitiveDateTime"),
            Self::SqlxTypesTimeOffsetDateTime => std::string::String::from("sqlx::types::time::OffsetDateTime"),
            Self::SqlxTypesTimeDate => std::string::String::from("sqlx::types::time::Date"),
            Self::SqlxTypesTimeTime => std::string::String::from("sqlx::types::time::Time"),
            Self::SqlxTypesUuidUuid => std::string::String::from("sqlx::types::uuid::Uuid"),
            Self::SqlxTypesIpnetworkIpNetwork => std::string::String::from("sqlx::types::ipnetwork::IpNetwork"),
            Self::StdNetIpAddr => std::string::String::from("std::net::IpAddr"),
            Self::SqlxTypesMacAddressMacAddress => std::string::String::from("sqlx::types::mac_address::MacAddress"),
            Self::SqlxTypesBitVec => std::string::String::from("sqlx::types::BitVec"),
            Self::SqlxTypesJsonT => format!("sqlx::types::Json<{generic_type_str}>"),
            Self::SerdeJsonValue => std::string::String::from("serde_json::Value"),
        }
    }
    fn get_inner_type_handle_stringified(&self, generic_type_str: &str) -> std::string::String {
        match self {
            Self::SqlxTypesJsonT => format!("{self}<{generic_type_str}>"),
            _ => self.to_string()
        }
    }
    pub fn get_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_handle_stringified(
        &self,
        generic_type_str: &str,
    ) -> std::string::String {
        match self {
            Self::SqlxTypesJsonT => format!(
                "sqlx::types::Json{}<{generic_type_str}>",
                proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
            ),
            _ => format!(
                "{self}{}",
                proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
            ) 
        }
    }
    pub fn get_inner_type_with_serialize_deserialize_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_error_named_handle_stringified(&self, generic_type_str: &str) -> std::string::String {
        match self.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            FromOrTryFrom::From => std::string::String::from(""),
            FromOrTryFrom::TryFrom => format!(
                "{}{}{}", 
                self.get_inner_type_handle_stringified(generic_type_str),
                proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
                proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified()
            )
        }
    }
    fn get_where_with_serialize_deserialize_error_named_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&match self.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            FromOrTryFrom::From => std::string::String::from(""),
            FromOrTryFrom::TryFrom => format!(
                "{}{}{}{}",
                proc_macro_helpers::naming_conventions::where_upper_camel_case_stringified(),
                self.get_inner_type_handle_stringified(generic_type_str),
                proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
                proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified()
            )
        })
    }
    fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(&self) -> FromOrTryFrom {
        match self {
            Self::StdPrimitiveBool => FromOrTryFrom::From,
            Self::StdPrimitiveI16 => FromOrTryFrom::From,
            Self::StdPrimitiveI32 => FromOrTryFrom::From,
            Self::StdPrimitiveI64 => FromOrTryFrom::From,
            Self::StdPrimitiveF32 => FromOrTryFrom::From,
            Self::StdPrimitiveF64 => FromOrTryFrom::From,
            Self::StdStringString => FromOrTryFrom::From,
            Self::StdVecVecStdPrimitiveU8 => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgInterval => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64 => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32 => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgMoney => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgCiText => FromOrTryFrom::From,
            Self::SqlxTypesBigDecimal => FromOrTryFrom::From,
            Self::SqlxTypesDecimal => FromOrTryFrom::From,
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => FromOrTryFrom::From,
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => FromOrTryFrom::From,
            Self::SqlxTypesChronoNaiveDateTime => FromOrTryFrom::From,
            Self::SqlxTypesChronoNaiveDate => FromOrTryFrom::From,
            Self::SqlxTypesChronoNaiveTime => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgTimeTz => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimePrimitiveDateTime => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimeOffsetDateTime => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimeDate => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimeTime => FromOrTryFrom::TryFrom,
            Self::SqlxTypesUuidUuid => FromOrTryFrom::TryFrom,
            Self::SqlxTypesIpnetworkIpNetwork => FromOrTryFrom::From,
            Self::StdNetIpAddr => FromOrTryFrom::From,
            Self::SqlxTypesMacAddressMacAddress => FromOrTryFrom::From,
            Self::SqlxTypesBitVec => FromOrTryFrom::From,
            Self::SqlxTypesJsonT => FromOrTryFrom::From,//todo
            Self::SerdeJsonValue => FromOrTryFrom::From,
        }
    }
}

impl std::convert::From<&RustSqlxMapToPostgresTypeVariant> for SupportedSqlxPostgresType {
    fn from(value: &RustSqlxMapToPostgresTypeVariant) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Self::StdPrimitiveBool,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => Self::StdPrimitiveBool,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => Self::StdPrimitiveI16,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => Self::StdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => Self::StdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => Self::StdPrimitiveF32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull => Self::StdPrimitiveF32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Self::StdPrimitiveF32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => Self::StdPrimitiveF32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => Self::StdPrimitiveF64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => Self::StdPrimitiveF64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Self::StdPrimitiveF64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => Self::StdPrimitiveF64,

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => Self::StdStringString,

            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Self::StdVecVecStdPrimitiveU8,
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => Self::StdVecVecStdPrimitiveU8,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Self::SqlxPostgresTypesPgInterval,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => Self::SqlxPostgresTypesPgInterval,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Self::SqlxPostgresTypesPgMoney,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => Self::SqlxPostgresTypesPgMoney,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Self::SqlxPostgresTypesPgCiText,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => Self::SqlxPostgresTypesPgCiText,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::SqlxTypesBigDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => Self::SqlxTypesBigDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Self::SqlxTypesDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull => Self::SqlxTypesDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesChronoNaiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => Self::SqlxTypesChronoNaiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::SqlxTypesChronoNaiveDate,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => Self::SqlxTypesChronoNaiveDate,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Self::SqlxTypesChronoNaiveTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => Self::SqlxTypesChronoNaiveTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Self::SqlxPostgresTypesPgTimeTz,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => Self::SqlxPostgresTypesPgTimeTz,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesTimePrimitiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => Self::SqlxTypesTimePrimitiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Self::SqlxTypesTimeOffsetDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => Self::SqlxTypesTimeOffsetDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Self::SqlxTypesTimeDate,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull => Self::SqlxTypesTimeDate,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Self::SqlxTypesTimeTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => Self::SqlxTypesTimeTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Self::SqlxTypesUuidUuid,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => Self::SqlxTypesUuidUuid,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Self::SqlxTypesUuidUuid,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Self::SqlxTypesIpnetworkIpNetwork,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => Self::SqlxTypesIpnetworkIpNetwork,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::SqlxTypesIpnetworkIpNetwork,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => Self::SqlxTypesIpnetworkIpNetwork,

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => Self::StdNetIpAddr,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull => Self::StdNetIpAddr,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Self::StdNetIpAddr,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => Self::StdNetIpAddr,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::SqlxTypesMacAddressMacAddress,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => Self::SqlxTypesMacAddressMacAddress,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => Self::SqlxTypesBitVec,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull => Self::SqlxTypesBitVec,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Self::SqlxTypesBitVec,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => Self::SqlxTypesBitVec,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => Self::SqlxTypesJsonT,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull => Self::SqlxTypesJsonT,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Self::SqlxTypesJsonT,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull => Self::SqlxTypesJsonT,

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Self::SerdeJsonValue,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull => Self::SerdeJsonValue,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::SerdeJsonValue,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Self::SerdeJsonValue,
        }
    }
}

//todo maybe use it as type for struct field but with inner type like StdPrimitiveBoolAsPostgresqlBool(StdPrimitiveBool)
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    strum_macros::Display,
    strum_macros::EnumIter,
    enum_extension::EnumExtension,
)]
pub enum RustSqlxMapToPostgresTypeVariant {
    StdPrimitiveBoolAsPostgresqlBool,
    StdPrimitiveBoolAsPostgresqlBoolNotNull,

    StdPrimitiveI16AsPostgresqlSmallInt,
    StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    StdPrimitiveI16AsPostgresqlSmallSerial,
    StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    StdPrimitiveI16AsPostgresqlInt2,
    StdPrimitiveI16AsPostgresqlInt2NotNull,

    StdPrimitiveI32AsPostgresqlInt,
    StdPrimitiveI32AsPostgresqlIntNotNull,
    StdPrimitiveI32AsPostgresqlSerial,
    StdPrimitiveI32AsPostgresqlSerialNotNull,
    StdPrimitiveI32AsPostgresqlInt4,
    StdPrimitiveI32AsPostgresqlInt4NotNull,

    StdPrimitiveI64AsPostgresqlBigInt,
    StdPrimitiveI64AsPostgresqlBigIntNotNull,
    StdPrimitiveI64AsPostgresqlBigSerial,
    StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    StdPrimitiveI64AsPostgresqlInt8,
    StdPrimitiveI64AsPostgresqlInt8NotNull,

    StdPrimitiveF32AsPostgresqlReal,
    StdPrimitiveF32AsPostgresqlRealNotNull,
    StdPrimitiveF32AsPostgresqlFloat4,
    StdPrimitiveF32AsPostgresqlFloat4NotNull,

    StdPrimitiveF64AsPostgresqlDoublePrecision,
    StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    StdPrimitiveF64AsPostgresqlFloat8,
    StdPrimitiveF64AsPostgresqlFloat8NotNull,

    StdStringStringAsPostgresqlVarchar,
    StdStringStringAsPostgresqlVarcharNotNull,
    StdStringStringAsPostgresqlCharN,
    StdStringStringAsPostgresqlCharNNotNull,
    StdStringStringAsPostgresqlText,
    StdStringStringAsPostgresqlTextNotNull,
    StdStringStringAsPostgresqlCiText,
    StdStringStringAsPostgresqlCiTextNotNull,

    StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    SqlxTypesBigDecimalAsPostgresqlNumeric,
    SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    SqlxTypesDecimalAsPostgresqlNumeric,
    SqlxTypesDecimalAsPostgresqlNumericNotNull,

    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    SqlxTypesChronoNaiveDateAsPostgresqlDate,
    SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    SqlxTypesTimeDateAsPostgresqlDate,
    SqlxTypesTimeDateAsPostgresqlDateNotNull,

    SqlxTypesTimeTimeAsPostgresqlTime,
    SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    SqlxTypesUuidUuidAsPostgresqlUuid,
    SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey, //todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough

    SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    StdNetIpAddrAsPostgresqlInet,
    StdNetIpAddrAsPostgresqlInetNotNull,
    StdNetIpAddrAsPostgresqlCidr,
    StdNetIpAddrAsPostgresqlCidrNotNull,

    SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    SqlxTypesBitVecAsPostgresqlBit,
    SqlxTypesBitVecAsPostgresqlBitNotNull,
    SqlxTypesBitVecAsPostgresqlVarBit,
    SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    //todo what to do with generic?
    SqlxTypesJsonTAsPostgresqlJson,
    SqlxTypesJsonTAsPostgresqlJsonNotNull,
    SqlxTypesJsonTAsPostgresqlJsonB,
    SqlxTypesJsonTAsPostgresqlJsonBNotNull,

    SerdeJsonValueAsPostgresqlJson,
    SerdeJsonValueAsPostgresqlJsonNotNull,
    SerdeJsonValueAsPostgresqlJsonB,
    SerdeJsonValueAsPostgresqlJsonBNotNull,
}

#[derive(Debug)]
pub enum RustSqlxMapToPostgresTypeVariantPrimaryKey {
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,
}

impl std::convert::TryFrom<&RustSqlxMapToPostgresTypeVariant> for RustSqlxMapToPostgresTypeVariantPrimaryKey {
    type Error = ();
    fn try_from(value: &RustSqlxMapToPostgresTypeVariant) -> Result<Self, Self::Error> {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Err(()),
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Err(()),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Err(()),
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull => Err(()),
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Err(()),
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum FromOrTryFrom {
    From,
    TryFrom,
}

impl RustSqlxMapToPostgresTypeVariant {
    pub fn get_path_stringified(&self) -> std::string::String {
        add_path(&self.to_string())
    }
    //todo maybe move to generate_postgresql_crud macro
    pub fn get_original_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_original_type_stringified(generic_type_str)
    }
    fn get_inner_type_handle_stringified(&self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_inner_type_handle_stringified(generic_type_str)
    }
    pub fn get_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_handle_stringified(
        &self,
        generic_type_str: &str,
    ) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str)
    }
    pub fn get_inner_type_with_serialize_deserialize_stringified(
        &self,
        generic_type_str: &str,
    ) -> std::string::String {
        add_path(&self.get_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str))
    }
    pub fn get_inner_type_with_serialize_deserialize_error_named_stringified(
        &self,
        generic_type_str: &str,
    ) -> std::string::String {
        add_path(&SupportedSqlxPostgresType::from(self).get_inner_type_with_serialize_deserialize_error_named_handle_stringified(generic_type_str))
    }
    pub fn get_where_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&format!(
            "{}{}", 
            proc_macro_helpers::naming_conventions::where_upper_camel_case_stringified(), 
            self.get_inner_type_handle_stringified(generic_type_str)
        ))
    }
    pub fn get_where_inner_type_with_serialize_deserialize_handle_stringified(&self, generic_type_str: &str) -> std::string::String {
        format!(
            "{}{}",
            proc_macro_helpers::naming_conventions::where_upper_camel_case_stringified(),
            self.get_inner_type_with_serialize_deserialize_handle_stringified(
                generic_type_str,
            )
        )
    }
    pub fn get_where_inner_type_with_serialize_deserialize_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_where_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str))
    }
    pub fn get_where_with_serialize_deserialize_error_named_stringified(&self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_where_with_serialize_deserialize_error_named_stringified(&generic_type_str)
    }
    pub fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(&self) -> FromOrTryFrom {
        SupportedSqlxPostgresType::from(self).inner_type_from_or_try_from_inner_type_with_serialize_deserialize()
    }
}

//todo rename conversion method
impl std::convert::TryFrom<&str> for RustSqlxMapToPostgresTypeVariant {
    type Error = std::string::String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "StdPrimitiveBoolAsPostgresqlBool" => Ok(Self::StdPrimitiveBoolAsPostgresqlBool),
            "StdPrimitiveBoolAsPostgresqlBoolNotNull" => Ok(Self::StdPrimitiveBoolAsPostgresqlBoolNotNull),

            "StdPrimitiveI16AsPostgresqlSmallInt" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallInt),
            "StdPrimitiveI16AsPostgresqlSmallIntNotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull),
            "StdPrimitiveI16AsPostgresqlSmallSerial" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerial),
            "StdPrimitiveI16AsPostgresqlSmallSerialNotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull),
            "StdPrimitiveI16AsPostgresqlInt2" => Ok(Self::StdPrimitiveI16AsPostgresqlInt2),
            "StdPrimitiveI16AsPostgresqlInt2NotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlInt2NotNull),

            "StdPrimitiveI32AsPostgresqlInt" => Ok(Self::StdPrimitiveI32AsPostgresqlInt),
            "StdPrimitiveI32AsPostgresqlIntNotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlIntNotNull),
            "StdPrimitiveI32AsPostgresqlSerial" => Ok(Self::StdPrimitiveI32AsPostgresqlSerial),
            "StdPrimitiveI32AsPostgresqlSerialNotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlSerialNotNull),
            "StdPrimitiveI32AsPostgresqlInt4" => Ok(Self::StdPrimitiveI32AsPostgresqlInt4),
            "StdPrimitiveI32AsPostgresqlInt4NotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlInt4NotNull),

            "StdPrimitiveI64AsPostgresqlBigInt" => Ok(Self::StdPrimitiveI64AsPostgresqlBigInt),
            "StdPrimitiveI64AsPostgresqlBigIntNotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlBigIntNotNull),
            "StdPrimitiveI64AsPostgresqlBigSerial" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerial),
            "StdPrimitiveI64AsPostgresqlBigSerialNotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull),
            "StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
            "StdPrimitiveI64AsPostgresqlInt8" => Ok(Self::StdPrimitiveI64AsPostgresqlInt8),
            "StdPrimitiveI64AsPostgresqlInt8NotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlInt8NotNull),

            "StdPrimitiveF32AsPostgresqlReal" => Ok(Self::StdPrimitiveF32AsPostgresqlReal),
            "StdPrimitiveF32AsPostgresqlRealNotNull" => Ok(Self::StdPrimitiveF32AsPostgresqlRealNotNull),
            "StdPrimitiveF32AsPostgresqlFloat4" => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4),
            "StdPrimitiveF32AsPostgresqlFloat4NotNull" => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4NotNull),

            "StdPrimitiveF64AsPostgresqlDoublePrecision" => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecision),
            "StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull" => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull),
            "StdPrimitiveF64AsPostgresqlFloat8" => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8),
            "StdPrimitiveF64AsPostgresqlFloat8NotNull" => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8NotNull),

            "StdStringStringAsPostgresqlVarchar" => Ok(Self::StdStringStringAsPostgresqlVarchar),
            "StdStringStringAsPostgresqlVarcharNotNull" => Ok(Self::StdStringStringAsPostgresqlVarcharNotNull),
            "StdStringStringAsPostgresqlCharN" => Ok(Self::StdStringStringAsPostgresqlCharN),
            "StdStringStringAsPostgresqlCharNNotNull" => Ok(Self::StdStringStringAsPostgresqlCharNNotNull),
            "StdStringStringAsPostgresqlText" => Ok(Self::StdStringStringAsPostgresqlText),
            "StdStringStringAsPostgresqlTextNotNull" => Ok(Self::StdStringStringAsPostgresqlTextNotNull),
            "StdStringStringAsPostgresqlCiText" => Ok(Self::StdStringStringAsPostgresqlCiText),
            "StdStringStringAsPostgresqlCiTextNotNull" => Ok(Self::StdStringStringAsPostgresqlCiTextNotNull),

            "StdVecVecStdPrimitiveU8AsPostgresqlBytea" => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea),
            "StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull" => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull),

            "SqlxPostgresTypesPgIntervalAsPostgresqlInterval" => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval),
            "SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull" => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull),

            "SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range),
            "SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull),

            "SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range),
            "SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange),
            "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange),
            "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange),
            "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange),
            "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange),
            "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange),
            "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange),
            "SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange),
            "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull),

            "SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange),
            "SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull),

            "SqlxPostgresTypesPgMoneyAsPostgresqlMoney" => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney),
            "SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull" => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull),

            "SqlxPostgresTypesPgCiTextAsPostgresqlCiText" => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText),
            "SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull" => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull),

            "SqlxTypesBigDecimalAsPostgresqlNumeric" => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumeric),
            "SqlxTypesBigDecimalAsPostgresqlNumericNotNull" => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull),

            "SqlxTypesDecimalAsPostgresqlNumeric" => Ok(Self::SqlxTypesDecimalAsPostgresqlNumeric),
            "SqlxTypesDecimalAsPostgresqlNumericNotNull" => Ok(Self::SqlxTypesDecimalAsPostgresqlNumericNotNull),

            "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz),
            "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull),

            "SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz),
            "SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull),

            "SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp" => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),
            "SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull),

            "SqlxTypesChronoNaiveDateAsPostgresqlDate" => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDate),
            "SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull" => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull),

            "SqlxTypesChronoNaiveTimeAsPostgresqlTime" => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime),
            "SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull" => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull),

            "SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz" => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz),
            "SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull" => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull),

            "SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp" => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),
            "SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull),

            "SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz),
            "SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull),

            "SqlxTypesTimeDateAsPostgresqlDate" => Ok(Self::SqlxTypesTimeDateAsPostgresqlDate),
            "SqlxTypesTimeDateAsPostgresqlDateNotNull" => Ok(Self::SqlxTypesTimeDateAsPostgresqlDateNotNull),

            "SqlxTypesTimeTimeAsPostgresqlTime" => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTime),
            "SqlxTypesTimeTimeAsPostgresqlTimeNotNull" => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull),

            "SqlxTypesUuidUuidAsPostgresqlUuid" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuid),
            "SqlxTypesUuidUuidAsPostgresqlUuidNotNull" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull),
            "SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey),

            "SqlxTypesIpnetworkIpNetworkAsPostgresqlInet" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet),
            "SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull),
            "SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr),
            "SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull),

            "StdNetIpAddrAsPostgresqlInet" => Ok(Self::StdNetIpAddrAsPostgresqlInet),
            "StdNetIpAddrAsPostgresqlInetNotNull" => Ok(Self::StdNetIpAddrAsPostgresqlInetNotNull),
            "StdNetIpAddrAsPostgresqlCidr" => Ok(Self::StdNetIpAddrAsPostgresqlCidr),
            "StdNetIpAddrAsPostgresqlCidrNotNull" => Ok(Self::StdNetIpAddrAsPostgresqlCidrNotNull),

            "SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr" => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr),
            "SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull" => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull),

            "SqlxTypesBitVecAsPostgresqlBit" => Ok(Self::SqlxTypesBitVecAsPostgresqlBit),
            "SqlxTypesBitVecAsPostgresqlBitNotNull" => Ok(Self::SqlxTypesBitVecAsPostgresqlBitNotNull),
            "SqlxTypesBitVecAsPostgresqlVarBit" => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBit),
            "SqlxTypesBitVecAsPostgresqlVarBitNotNull" => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull),

            //todo what to do with generic?
            "SqlxTypesJsonTAsPostgresqlJson" => Ok(Self::SqlxTypesJsonTAsPostgresqlJson),
            "SqlxTypesJsonTAsPostgresqlJsonNotNull" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonNotNull),
            "SqlxTypesJsonTAsPostgresqlJsonB" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonB),
            "SqlxTypesJsonTAsPostgresqlJsonBNotNull" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonBNotNull),

            "SerdeJsonValueAsPostgresqlJson" => Ok(Self::SerdeJsonValueAsPostgresqlJson),
            "SerdeJsonValueAsPostgresqlJsonNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonNotNull),
            "SerdeJsonValueAsPostgresqlJsonB" => Ok(Self::SerdeJsonValueAsPostgresqlJsonB),
            "SerdeJsonValueAsPostgresqlJsonBNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonBNotNull),
            _ => Err(format!(
                "unsupported value: {value}, {:?}",
                Self::into_array().into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<std::string::String>>()
            ))
        }
    }
}

pub trait CheckSupportedRustAndPostgresqlColumnType {
    fn check_supported_rust_and_postgresql_column_type();
}

//todo maybe inner value must be pub
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveBoolAsPostgresqlBool(pub StdPrimitiveBool);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(pub StdPrimitiveBool);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallInt(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallIntNotNull(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerial(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlInt2(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlInt(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlIntNotNull(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlSerial(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlInt4(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigInt(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigIntNotNull(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigSerial(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlInt8(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlReal(pub StdPrimitiveF32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlRealNotNull(pub StdPrimitiveF32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlFloat4(pub StdPrimitiveF32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(pub StdPrimitiveF32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlDoublePrecision(pub StdPrimitiveF64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull(pub StdPrimitiveF64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlFloat8(pub StdPrimitiveF64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(pub StdPrimitiveF64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlVarchar(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlVarcharNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCharN(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCharNNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlText(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlTextNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCiText(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCiTextNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlBytea(pub StdVecVecStdPrimitiveU8);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull(pub StdVecVecStdPrimitiveU8);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlInterval(pub SqlxPostgresTypesPgInterval);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull(pub SqlxPostgresTypesPgInterval);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range(
    pub SqlxPostgresTypesPgRangeStdPrimitiveI64,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull(
    pub SqlxPostgresTypesPgRangeStdPrimitiveI64,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range(
    pub SqlxPostgresTypesPgRangeStdPrimitiveI32,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull(
    pub SqlxPostgresTypesPgRangeStdPrimitiveI32,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange(
    pub SqlxPostgresTypesPgRangeSqlxTypesDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoney(pub SqlxPostgresTypesPgMoney);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull(pub SqlxPostgresTypesPgMoney);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgCiTextAsPostgresqlCiText(pub SqlxPostgresTypesPgCiText);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull(pub SqlxPostgresTypesPgCiText);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumeric(pub SqlxTypesBigDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumericNotNull(pub SqlxTypesBigDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesDecimalAsPostgresqlNumeric(pub SqlxTypesDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesDecimalAsPostgresqlNumericNotNull(pub SqlxTypesDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz(
    pub SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull(
    pub SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz(
    pub SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull(
    pub SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp(pub SqlxTypesChronoNaiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull(
    pub SqlxTypesChronoNaiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDate(pub SqlxTypesChronoNaiveDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull(pub SqlxTypesChronoNaiveDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTime(pub SqlxTypesChronoNaiveTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull(pub SqlxTypesChronoNaiveTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz(pub SqlxPostgresTypesPgTimeTz);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull(pub SqlxPostgresTypesPgTimeTz);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp(pub SqlxTypesTimePrimitiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull(
    pub SqlxTypesTimePrimitiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz(pub SqlxTypesTimeOffsetDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull(
    pub SqlxTypesTimeOffsetDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeDateAsPostgresqlDate(pub SqlxTypesTimeDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeDateAsPostgresqlDateNotNull(pub SqlxTypesTimeDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeTimeAsPostgresqlTime(pub SqlxTypesTimeTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeTimeAsPostgresqlTimeNotNull(pub SqlxTypesTimeTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuid(pub SqlxTypesUuidUuid);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuidNotNull(pub SqlxTypesUuidUuid);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey(pub SqlxTypesUuidUuid);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInet(pub SqlxTypesIpnetworkIpNetwork);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull(pub SqlxTypesIpnetworkIpNetwork);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr(pub SqlxTypesIpnetworkIpNetwork);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull(pub SqlxTypesIpnetworkIpNetwork);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlInet(pub StdNetIpAddr);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlInetNotNull(pub StdNetIpAddr);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlCidr(pub StdNetIpAddr);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlCidrNotNull(pub StdNetIpAddr);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr(pub SqlxTypesMacAddressMacAddress);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull(
    pub SqlxTypesMacAddressMacAddress,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlBit(pub SqlxTypesBitVec);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlBitNotNull(pub SqlxTypesBitVec);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlVarBit(pub SqlxTypesBitVec);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlVarBitNotNull(pub SqlxTypesBitVec);
//todo what to do with generic?
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJson<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJson<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonNotNull<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonB<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonB<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonBNotNull<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonBNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJson(pub SerdeJsonValue);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJsonNotNull(pub SerdeJsonValue);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJsonB(pub SerdeJsonValue);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJsonBNotNull(pub SerdeJsonValue);

////////////////////////////////////////////////////////////////////////////////////////////////

// todo shared enum of postgres types for postgresql_crud and generate_postgresql_crud
// remove and make one
//todo support variations of init functions as enum

pub struct Test<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    std_primitive_bool: std::primitive::bool, //BOOL
    // std_primitive_i8: std::primitive::i8,   //“CHAR”//not clear how to make primary key from it
    std_primitive_i16: std::primitive::i16, //SMALLINT, SMALLSERIAL, INT2
    std_primitive_i32: std::primitive::i32, //INT, SERIAL, INT4
    std_primitive_i64: std::primitive::i64, //BIGINT, BIGSERIAL, INT8
    std_primitive_f32: std::primitive::f32, //REAL, FLOAT4
    std_primitive_f64: std::primitive::f64, //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, CITEXT//NAME was ignored coz its recommended to not using it for typical user  
    std_string_string: std::string::String, //VARCHAR, CHAR(N), TEXT, CITEXT //NAME was ignored coz its recommended to not using it for typical user 
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    std_vec_vec_std_primitive_u8: std::vec::Vec<std::primitive::u8>, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    sqlx_postgres_types_pg_interval: sqlx::postgres::types::PgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    sqlx_postgres_types_pg_range_std_primitive_i64:
        sqlx::postgres::types::PgRange<std::primitive::i64>, //INT8RANGE
    sqlx_postgres_types_pg_range_std_primitive_i32:
        sqlx::postgres::types::PgRange<std::primitive::i32>, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time:
        sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>, //TSTZRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>, //TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time:
        sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>, //TSRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date:
        sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>, //DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_date:
        sqlx::postgres::types::PgRange<sqlx::types::time::Date>, //DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal:
        sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>, //NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_decimal:
        sqlx::postgres::types::PgRange<sqlx::types::Decimal>, //NUMRANGE
    sqlx_postgres_types_pg_money: sqlx::postgres::types::PgMoney, //MONEY
    // sqlx_postgres_types_pg_l_tree: sqlx::postgres::types::PgLTree,//LTREE//dont want to support that for postgresql_crud
    // sqlx_postgres_types_pg_l_query: sqlx::postgres::types::PgLQuery,//LQUERY//dont want to support that for postgresql_crud
    sqlx_postgres_types_pg_ci_text: sqlx::postgres::types::PgCiText, //CITEXT
    sqlx_types_big_decimal: sqlx::types::BigDecimal,                 //NUMERIC
    sqlx_types_decimal: sqlx::types::Decimal,                        //NUMERIC
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc:
        sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, //TIMESTAMP
    sqlx_types_chrono_date_time_sqlx_types_chrono_local:
        sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>, //TIMESTAMPTZ
    sqlx_types_chrono_naive_date_time: sqlx::types::chrono::NaiveDateTime, //TIMESTAMP
    sqlx_types_chrono_naive_date: sqlx::types::chrono::NaiveDate,    //DATE
    sqlx_types_chrono_naive_time: sqlx::types::chrono::NaiveTime,    //TIME
    sqlx_postgres_types_pg_time_tz: sqlx::postgres::types::PgTimeTz, //just present chrono or time flag
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    sqlx_types_time_primitive_date_time: sqlx::types::time::PrimitiveDateTime, //TIMESTAMP
    sqlx_types_time_offset_date_time: sqlx::types::time::OffsetDateTime,       //TIMESTAMPTZ
    sqlx_types_time_date: sqlx::types::time::Date,                             //DATE
    sqlx_types_time_time: sqlx::types::time::Time,                             //TIME
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    sqlx_types_uuid_uuid: sqlx::types::uuid::Uuid, //UUID
    sqlx_types_ipnetwork_ip_network: sqlx::types::ipnetwork::IpNetwork, //INET, CIDR
    std_net_ip_addr: std::net::IpAddr,             //INET, CIDR
    sqlx_types_mac_address_mac_address: sqlx::types::mac_address::MacAddress, //MACADDR
    sqlx_types_bit_vec: sqlx::types::BitVec,       //BIT, VARBIT
    sqlx_types_json: sqlx::types::Json<T>,         //JSON, JSONB
    serde_json_value: serde_json::Value,           //JSON, JSONB
                                                   // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
                                                   //maybe Composite types
                                                   //maybe Enumerations
}

pub struct TestNewTypeWithAdditionalInfo<T> {
    _std_primitive_bool_as_postgresql_bool: StdPrimitiveBoolAsPostgresqlBool,
    _std_primitive_bool_as_postgresql_bool_not_null: StdPrimitiveBoolAsPostgresqlBoolNotNull,

    _std_primitive_i16_as_postgresql_small_int: StdPrimitiveI16AsPostgresqlSmallInt,
    _std_primitive_i16_as_postgresql_small_int_not_null: StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    _std_primitive_i16_as_postgresql_small_serial: StdPrimitiveI16AsPostgresqlSmallSerial,
    _std_primitive_i16_as_postgresql_small_serial_not_null: StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    _std_primitive_i16_as_postgresql_small_int2: StdPrimitiveI16AsPostgresqlInt2,
    _std_primitive_i16_as_postgresql_small_int2_not_null: StdPrimitiveI16AsPostgresqlInt2NotNull,

    _std_primitive_i32_as_postgresql_int: StdPrimitiveI32AsPostgresqlInt,
    _std_primitive_i32_as_postgresql_int_not_null: StdPrimitiveI32AsPostgresqlIntNotNull,
    _std_primitive_i32_as_postgresql_serial: StdPrimitiveI32AsPostgresqlSerial,
    _std_primitive_i32_as_postgresql_serial_not_null: StdPrimitiveI32AsPostgresqlSerialNotNull,
    _std_primitive_i32_as_postgresql_int4: StdPrimitiveI32AsPostgresqlInt4,
    _std_primitive_i32_as_postgresql_int4_not_null: StdPrimitiveI32AsPostgresqlInt4NotNull,

    _std_primitive_i64_as_postgresql_big_int: StdPrimitiveI64AsPostgresqlBigInt,
    _std_primitive_i64_as_postgresql_big_int_not_null: StdPrimitiveI64AsPostgresqlBigIntNotNull,
    _std_primitive_i64_as_postgresql_big_serial: StdPrimitiveI64AsPostgresqlBigSerial,
    _std_primitive_i64_as_postgresql_big_serial_not_null: StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    _std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    _std_primitive_i64_as_postgresql_big_int8: StdPrimitiveI64AsPostgresqlInt8,
    _std_primitive_i64_as_postgresql_big_int8_not_null: StdPrimitiveI64AsPostgresqlInt8NotNull,

    _std_primitive_f32_as_postgresql_real: StdPrimitiveF32AsPostgresqlReal,
    _std_primitive_f32_as_postgresql_real_not_null: StdPrimitiveF32AsPostgresqlRealNotNull,
    _std_primitive_f32_as_postgresql_float4: StdPrimitiveF32AsPostgresqlFloat4,
    _std_primitive_f32_as_postgresql_float4_not_null: StdPrimitiveF32AsPostgresqlFloat4NotNull,

    _std_primitive_f64_as_postgresql_double_precision: StdPrimitiveF64AsPostgresqlDoublePrecision,
    _std_primitive_f64_as_postgresql_double_precision_not_null: StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    _std_primitive_f64_as_postgresql_float8: StdPrimitiveF64AsPostgresqlFloat8,
    _std_primitive_f64_as_postgresql_float8_not_null: StdPrimitiveF64AsPostgresqlFloat8NotNull,

    _std_string_string_as_postgresql_varchar: StdStringStringAsPostgresqlVarchar,
    _std_string_string_as_postgresql_varchar_not_null: StdStringStringAsPostgresqlVarcharNotNull,
    _std_string_string_as_postgresql_char_n: StdStringStringAsPostgresqlCharN,
    _std_string_string_as_postgresql_char_n_not_null: StdStringStringAsPostgresqlCharNNotNull,
    _std_string_string_as_postgresql_text: StdStringStringAsPostgresqlText,
    _std_string_string_as_postgresql_text_not_null: StdStringStringAsPostgresqlTextNotNull,
    _std_string_string_as_postgresql_ci_text: StdStringStringAsPostgresqlCiText,
    _std_string_string_as_postgresql_ci_text_not_null: StdStringStringAsPostgresqlCiTextNotNull,

    _std_vec_vec_std_primitive_u8_as_postgresql_bytea: StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    _std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    _sqlx_postgres_types_pg_interval_as_postgresql_interval: SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    _sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    _sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    _sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    _sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    _sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    _sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    _sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    _sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    _sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    _sqlx_postgres_types_pg_money_as_postgresql_money: SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    _sqlx_postgres_types_pg_money_as_postgresql_money_not_null: SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    _sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    _sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    _sqlx_types_big_decimal_as_postgresql_numeric: SqlxTypesBigDecimalAsPostgresqlNumeric,
    _sqlx_types_big_decimal_as_postgresql_numeric_not_null: SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    _sqlx_types_decimal_as_postgresql_numeric: SqlxTypesDecimalAsPostgresqlNumeric,
    _sqlx_types_decimal_as_postgresql_numeric_not_null: SqlxTypesDecimalAsPostgresqlNumericNotNull,

    _sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    _sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    _sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    _sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    _sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    _sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    _sqlx_types_chrono_naive_date_as_postgresql_date: SqlxTypesChronoNaiveDateAsPostgresqlDate,
    _sqlx_types_chrono_naive_date_as_postgresql_date_not_null: SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    _sqlx_types_chrono_naive_time_as_postgresql_time: SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    _sqlx_types_chrono_naive_time_as_postgresql_time_not_null: SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    _sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    _sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    _sqlx_types_time_primitive_date_time_as_postgresql_timestamp: SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    _sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    _sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    _sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    _sqlx_types_time_date_as_postgresql_date: SqlxTypesTimeDateAsPostgresqlDate,
    _sqlx_types_time_date_as_postgresql_date_not_null: SqlxTypesTimeDateAsPostgresqlDateNotNull,

    _sqlx_types_time_time_as_postgresql_time: SqlxTypesTimeTimeAsPostgresqlTime,
    _sqlx_types_time_time_as_postgresql_time_not_null: SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    _sqlx_types_uuid_uuida_as_postgresql_uuid: SqlxTypesUuidUuidAsPostgresqlUuid,
    _sqlx_types_uuid_uuida_as_postgresql_uuid_not_null: SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    _sqlx_types_uuid_uuida_as_postgresql_uuid_not_null_primary_key: SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough

    _sqlx_types_ipnetwork_ip_network_as_postgresql_inet: SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    _sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    _sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    _sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    _std_net_ip_addr_as_postgresql_inet: StdNetIpAddrAsPostgresqlInet,
    _std_net_ip_addr_as_postgresql_inet_not_null: StdNetIpAddrAsPostgresqlInetNotNull,
    _std_net_ip_addr_as_postgresql_cidr: StdNetIpAddrAsPostgresqlCidr,
    _std_net_ip_addr_as_postgresql_cidr_not_null: StdNetIpAddrAsPostgresqlCidrNotNull,

    _sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    _sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    _sqlx_types_bit_vec_as_postgresql_bit: SqlxTypesBitVecAsPostgresqlBit,
    _sqlx_types_bit_vec_as_postgresql_bit_not_null: SqlxTypesBitVecAsPostgresqlBitNotNull,
    _sqlx_types_bit_vec_as_postgresql_var_bit: SqlxTypesBitVecAsPostgresqlVarBit,
    _sqlx_types_bit_vec_as_postgresql_var_bit_not_null: SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    //todo what to do with generic?
    _sqlx_types_json_t_as_postgresql_json: SqlxTypesJsonTAsPostgresqlJson<T>,
    _sqlx_types_json_t_as_postgresql_json_not_null: SqlxTypesJsonTAsPostgresqlJsonNotNull<T>,
    _sqlx_types_json_t_as_postgresql_json_b: SqlxTypesJsonTAsPostgresqlJsonB<T>,
    _sqlx_types_json_t_as_postgresql_json_b_not_null: SqlxTypesJsonTAsPostgresqlJsonBNotNull<T>,

    _serde_json_value_as_postgresql_json: SerdeJsonValueAsPostgresqlJson,
    _serde_json_value_as_postgresql_json_not_null: SerdeJsonValueAsPostgresqlJsonNotNull,
    _serde_json_value_as_postgresql_json_b: SerdeJsonValueAsPostgresqlJsonB,
    _serde_json_value_as_postgresql_json_b_not_null: SerdeJsonValueAsPostgresqlJsonBNotNull,
}

struct TestNewType<T> {
    std_primitive_bool: StdPrimitiveBool,
    std_primitive_i16: StdPrimitiveI16,
    std_primitive_i32: StdPrimitiveI32,
    std_primitive_i64: StdPrimitiveI64,
    std_primitive_f32: StdPrimitiveF32,
    std_primitive_f64: StdPrimitiveF64,
    std_string_string: StdStringString,
    std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8,
    sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgInterval,
    sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64,
    sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc:
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local:
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time:
        SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time:
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time:
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date:
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal:
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney,
    sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText,
    sqlx_types_big_decimal: SqlxTypesBigDecimal,
    sqlx_types_decimal: SqlxTypesDecimal,
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    sqlx_types_chrono_date_time_sqlx_types_chrono_local:
        SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime,
    sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate,
    sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime,
    sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz,
    sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime,
    sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime,
    sqlx_types_time_date: SqlxTypesTimeDate,
    sqlx_types_time_time: SqlxTypesTimeTime,
    sqlx_types_uuid_uuid: SqlxTypesUuidUuid,
    sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork,
    std_net_ip_addr: StdNetIpAddr,
    sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress,
    sqlx_types_bit_vec: SqlxTypesBitVec,
    sqlx_types_json: SqlxTypesJson<T>,
    serde_json_value: SerdeJsonValue,
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct TestNewTypeWithSerializeDeserialize<T> {
    std_primitive_bool: StdPrimitiveBoolWithSerializeDeserialize,
    std_primitive_i16: StdPrimitiveI16WithSerializeDeserialize,
    std_primitive_i32: StdPrimitiveI32WithSerializeDeserialize,
    std_primitive_i64: StdPrimitiveI64WithSerializeDeserialize,
    std_primitive_f32: StdPrimitiveF32WithSerializeDeserialize,
    std_primitive_f64: StdPrimitiveF64WithSerializeDeserialize,
    std_string_string: StdStringStringWithSerializeDeserialize,
    std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8WithSerializeDeserialize,
    sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgIntervalWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_std_primitive_i64:
        SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize,
    sqlx_postgres_types_pg_range_std_primitive_i32:
        SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc:
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local:
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time:
        SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time:
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize, //todo maybe naming
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time:
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date:
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_time_date:
        SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal:
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_decimal:
        SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize,
    sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoneyWithSerializeDeserialize,
    sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiTextWithSerializeDeserialize,
    sqlx_types_big_decimal: SqlxTypesBigDecimalWithSerializeDeserialize,
    sqlx_types_decimal: SqlxTypesDecimalWithSerializeDeserialize,
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc:
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
    sqlx_types_chrono_date_time_sqlx_types_chrono_local:
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
    sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize,
    sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDateWithSerializeDeserialize,
    sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTimeWithSerializeDeserialize,
    sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTzWithSerializeDeserialize,
    sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
    sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
    sqlx_types_time_date: SqlxTypesTimeDateWithSerializeDeserialize,
    sqlx_types_time_time: SqlxTypesTimeTimeWithSerializeDeserialize,
    sqlx_types_uuid_uuid: SqlxTypesUuidUuidWithSerializeDeserialize,
    sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetworkWithSerializeDeserialize,
    std_net_ip_addr: StdNetIpAddrWithSerializeDeserialize,
    sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddressWithSerializeDeserialize,
    sqlx_types_bit_vec: SqlxTypesBitVecWithSerializeDeserialize,

    sqlx_types_json: SqlxTypesJsonWithSerializeDeserialize<T>,
    serde_json_value: SerdeJsonValueWithSerializeDeserialize,
}

impl<T> std::convert::TryFrom<TestNewTypeWithSerializeDeserialize<T>> for TestNewType<T> {
    type Error = (); //todo
    fn try_from(value: TestNewTypeWithSerializeDeserialize<T>) -> Result<Self, Self::Error> {
        let std_primitive_bool = StdPrimitiveBool::from(value.std_primitive_bool);
        let std_primitive_i16 = StdPrimitiveI16::from(value.std_primitive_i16);
        let std_primitive_i32 = StdPrimitiveI32::from(value.std_primitive_i32);
        let std_primitive_i64 = StdPrimitiveI64::from(value.std_primitive_i64);
        let std_primitive_f32 = StdPrimitiveF32::from(value.std_primitive_f32);
        let std_primitive_f64 = StdPrimitiveF64::from(value.std_primitive_f64);
        let std_string_string = StdStringString::from(value.std_string_string);
        let std_vec_vec_std_primitive_u8 =
            StdVecVecStdPrimitiveU8::from(value.std_vec_vec_std_primitive_u8);
        let sqlx_postgres_types_pg_interval =
            SqlxPostgresTypesPgInterval::from(value.sqlx_postgres_types_pg_interval);
        let sqlx_postgres_types_pg_range_std_primitive_i64 =
            SqlxPostgresTypesPgRangeStdPrimitiveI64::from(
                value.sqlx_postgres_types_pg_range_std_primitive_i64,
            );
        let sqlx_postgres_types_pg_range_std_primitive_i32 =
            SqlxPostgresTypesPgRangeStdPrimitiveI32::from(
                value.sqlx_postgres_types_pg_range_std_primitive_i32,
            );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            match SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(
                value
                    .sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local = match SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
            match SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time =
            match SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
            match SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date =
            match SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_time_date =
            match SqlxPostgresTypesPgRangeSqlxTypesTimeDate::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_date,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_big_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesBigDecimal::from(
                value.sqlx_postgres_types_pg_range_sqlx_types_big_decimal,
            );
        let sqlx_postgres_types_pg_range_sqlx_types_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesDecimal::from(
                value.sqlx_postgres_types_pg_range_sqlx_types_decimal,
            );
        let sqlx_postgres_types_pg_money =
            SqlxPostgresTypesPgMoney::from(value.sqlx_postgres_types_pg_money);
        let sqlx_postgres_types_pg_ci_text =
            SqlxPostgresTypesPgCiText::from(value.sqlx_postgres_types_pg_ci_text);
        let sqlx_types_big_decimal = SqlxTypesBigDecimal::from(value.sqlx_types_big_decimal);
        let sqlx_types_decimal = SqlxTypesDecimal::from(value.sqlx_types_decimal);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            match SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            match SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_types_chrono_naive_date_time =
            match SqlxTypesChronoNaiveDateTime::try_from(value.sqlx_types_chrono_naive_date_time) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_types_chrono_naive_date =
            match SqlxTypesChronoNaiveDate::try_from(value.sqlx_types_chrono_naive_date) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_types_chrono_naive_time =
            match SqlxTypesChronoNaiveTime::try_from(value.sqlx_types_chrono_naive_time) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_time_tz =
            match SqlxPostgresTypesPgTimeTz::try_from(value.sqlx_postgres_types_pg_time_tz) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_types_time_primitive_date_time = match SqlxTypesTimePrimitiveDateTime::try_from(
            value.sqlx_types_time_primitive_date_time,
        ) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_time_offset_date_time =
            match SqlxTypesTimeOffsetDateTime::try_from(value.sqlx_types_time_offset_date_time) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_types_time_date = match SqlxTypesTimeDate::try_from(value.sqlx_types_time_date) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_time_time = match SqlxTypesTimeTime::try_from(value.sqlx_types_time_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_uuid_uuid = match SqlxTypesUuidUuid::try_from(value.sqlx_types_uuid_uuid) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_ipnetwork_ip_network =
            SqlxTypesIpnetworkIpNetwork::from(value.sqlx_types_ipnetwork_ip_network);
        let std_net_ip_addr = StdNetIpAddr::from(value.std_net_ip_addr);
        let sqlx_types_mac_address_mac_address =
            SqlxTypesMacAddressMacAddress::from(value.sqlx_types_mac_address_mac_address);
        let sqlx_types_bit_vec = SqlxTypesBitVec::from(value.sqlx_types_bit_vec);
        let sqlx_types_json = SqlxTypesJson::<T>::from(value.sqlx_types_json);
        let serde_json_value = SerdeJsonValue::from(value.serde_json_value);
        Ok(Self {
            std_primitive_bool,
            std_primitive_i16,
            std_primitive_i32,
            std_primitive_i64,
            std_primitive_f32,
            std_primitive_f64,
            std_string_string,
            std_vec_vec_std_primitive_u8,
            sqlx_postgres_types_pg_interval,
            sqlx_postgres_types_pg_range_std_primitive_i64,
            sqlx_postgres_types_pg_range_std_primitive_i32,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date,
            sqlx_postgres_types_pg_range_sqlx_types_time_date,
            sqlx_postgres_types_pg_range_sqlx_types_big_decimal,
            sqlx_postgres_types_pg_range_sqlx_types_decimal,
            sqlx_postgres_types_pg_money,
            sqlx_postgres_types_pg_ci_text,
            sqlx_types_big_decimal,
            sqlx_types_decimal,
            sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            sqlx_types_chrono_naive_date_time,
            sqlx_types_chrono_naive_date,
            sqlx_types_chrono_naive_time,
            sqlx_postgres_types_pg_time_tz,
            sqlx_types_time_primitive_date_time,
            sqlx_types_time_offset_date_time,
            sqlx_types_time_date,
            sqlx_types_time_time,
            sqlx_types_uuid_uuid,
            sqlx_types_ipnetwork_ip_network,
            std_net_ip_addr,
            sqlx_types_mac_address_mac_address,
            sqlx_types_bit_vec,
            sqlx_types_json,
            serde_json_value,
        })
    }
}

impl<T> std::convert::From<Test<T>> for TestNewType<T> {
    fn from(value: Test<T>) -> Self {
        Self {
            std_primitive_bool: StdPrimitiveBool(value.std_primitive_bool),
            std_primitive_i16: StdPrimitiveI16(value.std_primitive_i16),
            std_primitive_i32: StdPrimitiveI32(value.std_primitive_i32),
            std_primitive_i64: StdPrimitiveI64(value.std_primitive_i64),
            std_primitive_f32: StdPrimitiveF32(value.std_primitive_f32),
            std_primitive_f64: StdPrimitiveF64(value.std_primitive_f64),
            std_string_string: StdStringString(value.std_string_string),
            std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8(value.std_vec_vec_std_primitive_u8),
            sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgInterval(value.sqlx_postgres_types_pg_interval),
            sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64(value.sqlx_postgres_types_pg_range_std_primitive_i64),
            sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32(value.sqlx_postgres_types_pg_range_std_primitive_i32),
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc
            ),
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local
            ),
            sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time
            ),
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time
            ),
            sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time
            ),
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
                value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date
            ),
            sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_date
            ),
            sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
                value.sqlx_postgres_types_pg_range_sqlx_types_big_decimal
            ),
            sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal(
                value.sqlx_postgres_types_pg_range_sqlx_types_decimal
            ),
            sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney(value.sqlx_postgres_types_pg_money),
            sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText(value.sqlx_postgres_types_pg_ci_text),
            sqlx_types_big_decimal: SqlxTypesBigDecimal(value.sqlx_types_big_decimal),
            sqlx_types_decimal: SqlxTypesDecimal(value.sqlx_types_decimal),
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_utc
            ),
            sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
                value.sqlx_types_chrono_date_time_sqlx_types_chrono_local
            ),
            sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime(value.sqlx_types_chrono_naive_date_time),
            sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate(value.sqlx_types_chrono_naive_date),
            sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime(value.sqlx_types_chrono_naive_time),
            sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz(value.sqlx_postgres_types_pg_time_tz),
            sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime(value.sqlx_types_time_primitive_date_time),
            sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime(value.sqlx_types_time_offset_date_time),
            sqlx_types_time_date: SqlxTypesTimeDate(value.sqlx_types_time_date),
            sqlx_types_time_time: SqlxTypesTimeTime(value.sqlx_types_time_time),
            sqlx_types_uuid_uuid: SqlxTypesUuidUuid(value.sqlx_types_uuid_uuid),
            sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork(value.sqlx_types_ipnetwork_ip_network),
            std_net_ip_addr: StdNetIpAddr(value.std_net_ip_addr),
            sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress(value.sqlx_types_mac_address_mac_address),
            sqlx_types_bit_vec: SqlxTypesBitVec(value.sqlx_types_bit_vec),
            sqlx_types_json: SqlxTypesJson::<T>(value.sqlx_types_json),
            serde_json_value: SerdeJsonValue(value.serde_json_value),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    something: std::string::String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum TimeMonthWithSerializeDeserialize {
    January,   // = 1,
    February,  // = 2,
    March,     // = 3,
    April,     // = 4,
    May,       // = 5,
    June,      // = 6,
    July,      // = 7,
    August,    // = 8,
    September, // = 9,
    October,   // = 10,
    November,  // = 11,
    December,  // = 12,
}
impl std::convert::From<TimeMonthWithSerializeDeserialize> for time::Month {
    fn from(value: TimeMonthWithSerializeDeserialize) -> Self {
        match value {
            TimeMonthWithSerializeDeserialize::January => time::Month::January,
            TimeMonthWithSerializeDeserialize::February => time::Month::February,
            TimeMonthWithSerializeDeserialize::March => time::Month::March,
            TimeMonthWithSerializeDeserialize::April => time::Month::April,
            TimeMonthWithSerializeDeserialize::May => time::Month::May,
            TimeMonthWithSerializeDeserialize::June => time::Month::June,
            TimeMonthWithSerializeDeserialize::July => time::Month::July,
            TimeMonthWithSerializeDeserialize::August => time::Month::August,
            TimeMonthWithSerializeDeserialize::September => time::Month::September,
            TimeMonthWithSerializeDeserialize::October => time::Month::October,
            TimeMonthWithSerializeDeserialize::November => time::Month::November,
            TimeMonthWithSerializeDeserialize::December => time::Month::December,
        }
    }
}
impl std::convert::From<time::Month> for TimeMonthWithSerializeDeserialize {
    fn from(value: time::Month) -> Self {
        match value {
            time::Month::January => Self::January,
            time::Month::February => Self::February,
            time::Month::March => Self::March,
            time::Month::April => Self::April,
            time::Month::May => Self::May,
            time::Month::June => Self::June,
            time::Month::July => Self::July,
            time::Month::August => Self::August,
            time::Month::September => Self::September,
            time::Month::October => Self::October,
            time::Month::November => Self::November,
            time::Month::December => Self::December,
        }
    }
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize {
    hours: std::primitive::i8,
    minutes: std::primitive::i8,
    seconds: std::primitive::i8,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_display]
        time_error_component_range: time::error::ComponentRange,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize> for sqlx::types::time::UtcOffset {
    type Error = SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        match sqlx::types::time::UtcOffset::from_hms(value.hours, value.minutes, value.seconds) {
            Ok(value) => Ok(value),
            Err(e) => Err(Self::Error::TimeErrorComponentRange {
                time_error_component_range: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
}
impl std::convert::From<sqlx::types::time::UtcOffset>
    for SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize
{
    fn from(value: sqlx::types::time::UtcOffset) -> Self {
        Self {
            hours: value.whole_hours(),
            minutes: value.minutes_past_hour(),
            seconds: value.seconds_past_minute(),
        }
    }
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum NumBigintSignWithSerializeDeserialize {
    Minus,
    NoSign,
    Plus,
}
impl std::convert::From<NumBigintSignWithSerializeDeserialize> for num_bigint::Sign {
    fn from(value: NumBigintSignWithSerializeDeserialize) -> Self {
        match value {
            NumBigintSignWithSerializeDeserialize::Minus => num_bigint::Sign::Minus,
            NumBigintSignWithSerializeDeserialize::NoSign => num_bigint::Sign::NoSign,
            NumBigintSignWithSerializeDeserialize::Plus => num_bigint::Sign::Plus,
        }
    }
}
impl std::convert::From<num_bigint::Sign> for NumBigintSignWithSerializeDeserialize {
    fn from(value: num_bigint::Sign) -> Self {
        match value {
            num_bigint::Sign::Minus => NumBigintSignWithSerializeDeserialize::Minus,
            num_bigint::Sign::NoSign => NumBigintSignWithSerializeDeserialize::NoSign,
            num_bigint::Sign::Plus => NumBigintSignWithSerializeDeserialize::Plus,
        }
    }
}
//todo pub or not for all - think
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct NumBigintBigIntWithSerializeDeserialize {
    sign: NumBigintSignWithSerializeDeserialize,
    digits: std::vec::Vec<std::primitive::u32>,
}
impl std::convert::From<NumBigintBigIntWithSerializeDeserialize> for num_bigint::BigInt {
    fn from(value: NumBigintBigIntWithSerializeDeserialize) -> Self {
        let sign = match value.sign {
            NumBigintSignWithSerializeDeserialize::Minus => num_bigint::Sign::Minus,
            NumBigintSignWithSerializeDeserialize::NoSign => num_bigint::Sign::NoSign,
            NumBigintSignWithSerializeDeserialize::Plus => num_bigint::Sign::Plus,
        };
        Self::new(sign, value.digits)
    }
}
impl std::convert::From<num_bigint::BigInt> for NumBigintBigIntWithSerializeDeserialize {
    fn from(value: num_bigint::BigInt) -> Self {
        let (sign, digits) = value.to_u32_digits();
        Self {
            sign: NumBigintSignWithSerializeDeserialize::from(sign),
            digits,
        }
    }
}
impl Default for TestNewType<Something> {
    fn default() -> Self {
        //todo unwraps
        let std_primitive_u8_handle = std::primitive::u8::default();
        let std_primitive_i8_handle = std::primitive::i8::default();
        let std_primitive_u16_handle = std::primitive::u16::default();
        let std_primitive_u32_handle = std::primitive::u32::default();
        let std_primitive_i32_handle = std::primitive::i32::default();
        let std_primitive_i64_handle = std::primitive::i64::default();
        let std_string_string_handle = std::string::String::default();
        let sqlx_types_time_date_handle =
            sqlx::types::time::Date::from_calendar_date(2024, time::Month::February, 3).unwrap();
        let sqlx_types_time_time_handle = sqlx::types::time::Time::from_hms(1, 1, 1).unwrap();
        let sqlx_types_chrono_naive_date_handle =
            sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap();
        let sqlx_types_chrono_naive_time_handle =
            sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap();
        let sqlx_types_chrono_naive_date_time_handle = sqlx::types::chrono::NaiveDateTime::new(
            sqlx_types_chrono_naive_date_handle.clone(), //todo
            sqlx_types_chrono_naive_time_handle.clone(),
        );
        let sqlx_types_time_primitive_date_time_handle = sqlx::types::time::PrimitiveDateTime::new(
            sqlx_types_time_date_handle.clone(), //todo
            sqlx_types_time_time_handle.clone(), //todo
        );
        let sqlx_types_chrono_fixed_offset_handle =
            sqlx::types::chrono::FixedOffset::west_opt(std_primitive_i32_handle.clone()).unwrap();
        let sqlx_types_time_offset_date_time_handle =
            sqlx::types::time::OffsetDateTime::from_unix_timestamp(std::primitive::i64::default())
                .unwrap();
        let sqlx_types_decimal_handle = sqlx::types::Decimal::try_new(
            std_primitive_i64_handle.clone(),
            std_primitive_u32_handle.clone(),
        )
        .unwrap();
        let sqlx_types_chrono_utc_handle = sqlx::types::chrono::Utc;
        let sqlx_types_big_decimal_handle = sqlx::types::BigDecimal::new(
            num_bigint::BigInt::new(
                num_bigint::Sign::Plus,
                vec![std_primitive_u32_handle.clone()],
            ),
            std_primitive_i64_handle.clone(),
        );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle =
            sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_utc_handle.clone(),
            );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle =
            sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_fixed_offset_handle.clone(),
            );
        let std_ops_bound_std_primitive_i64_handle =
            std::ops::Bound::<std::primitive::i64>::Included(std_primitive_i64_handle.clone());
        let std_ops_bound_std_primitive_i32_handle =
            std::ops::Bound::<std::primitive::i32>::Included(std_primitive_i32_handle.clone());
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle =
            std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>::Included(
                sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle.clone(),
            );
        let std_ops_bound_sqlx_types_time_primitive_date_time_handle =
            std::ops::Bound::<sqlx::types::time::PrimitiveDateTime>::Included(
                sqlx_types_time_primitive_date_time_handle.clone(),
            );
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle =
            std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>::Included(
                sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle.clone(),
            );
        let std_ops_bound_sqlx_types_time_offset_date_time_handle =
            std::ops::Bound::<sqlx::types::time::OffsetDateTime>::Included(
                sqlx_types_time_offset_date_time_handle.clone(),
            );
        let std_ops_bound_sqlx_types_chrono_naive_date_handle =
            std::ops::Bound::<sqlx::types::chrono::NaiveDate>::Included(
                sqlx_types_chrono_naive_date_handle.clone(),
            );
        let std_ops_bound_sqlx_types_time_date_handle =
            std::ops::Bound::<sqlx::types::time::Date>::Included(
                sqlx_types_time_date_handle.clone(),
            );
        let std_ops_bound_sqlx_types_big_decimal_handle =
            std::ops::Bound::<sqlx::types::BigDecimal>::Included(
                sqlx_types_big_decimal_handle.clone(),
            );
        let std_ops_bound_sqlx_types_decimal_handle =
            std::ops::Bound::<sqlx::types::Decimal>::Included(sqlx_types_decimal_handle.clone());
        let std_ops_bound_sqlx_types_chrono_naive_date_time_handle =
            std::ops::Bound::<sqlx::types::chrono::NaiveDateTime>::Included(
                sqlx_types_chrono_naive_date_time_handle.clone(),
            );
        let std_primitive_bool = StdPrimitiveBool(true);
        let std_primitive_i16 = StdPrimitiveI16(std::primitive::i16::default());
        let std_primitive_i32 = StdPrimitiveI32(std_primitive_i32_handle.clone());
        let std_primitive_i64 = StdPrimitiveI64(std_primitive_i64_handle.clone());
        let std_primitive_f32 = StdPrimitiveF32(std::primitive::f32::default());
        let std_primitive_f64 = StdPrimitiveF64(std::primitive::f64::default());
        let std_string_string = StdStringString(std_string_string_handle.clone());
        let std_vec_vec_std_primitive_u8 =
            StdVecVecStdPrimitiveU8(vec![std_primitive_u8_handle.clone()]);
        let sqlx_postgres_types_pg_interval =
            SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval {
                months: std_primitive_i32_handle.clone(),
                days: std_primitive_i32_handle.clone(),
                microseconds: std_primitive_i64_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_std_primitive_i64 =
            SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange::<
                std::primitive::i64,
            > {
                start: std_ops_bound_std_primitive_i64_handle.clone(),
                end: std_ops_bound_std_primitive_i64_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_std_primitive_i32 =
            SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange::<
                std::primitive::i32,
            > {
                start: std_ops_bound_std_primitive_i32_handle.clone(),
                end: std_ops_bound_std_primitive_i32_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
                sqlx::postgres::types::PgRange::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                > {
                    start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle
                        .clone(),
                    end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle
                        .clone(),
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
                sqlx::postgres::types::PgRange::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                > {
                    start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle
                        .clone(),
                    end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle
                        .clone(),
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::time::OffsetDateTime,
            > {
                start: std_ops_bound_sqlx_types_time_offset_date_time_handle.clone(),
                end: std_ops_bound_sqlx_types_time_offset_date_time_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::NaiveDateTime,
            > {
                start: std_ops_bound_sqlx_types_chrono_naive_date_time_handle.clone(),
                end: std_ops_bound_sqlx_types_chrono_naive_date_time_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
                sqlx::postgres::types::PgRange::<sqlx::types::time::PrimitiveDateTime> {
                    start: std_ops_bound_sqlx_types_time_primitive_date_time_handle.clone(),
                    end: std_ops_bound_sqlx_types_time_primitive_date_time_handle.clone(),
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date =
            SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::NaiveDate,
            > {
                start: std_ops_bound_sqlx_types_chrono_naive_date_handle.clone(),
                end: std_ops_bound_sqlx_types_chrono_naive_date_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_time_date =
            SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange::<
                sqlx::types::time::Date,
            > {
                start: std_ops_bound_sqlx_types_time_date_handle.clone(),
                end: std_ops_bound_sqlx_types_time_date_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_big_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange::<
                sqlx::types::BigDecimal,
            > {
                start: std_ops_bound_sqlx_types_big_decimal_handle.clone(),
                end: std_ops_bound_sqlx_types_big_decimal_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange::<
                sqlx::types::Decimal,
            > {
                start: std_ops_bound_sqlx_types_decimal_handle.clone(),
                end: std_ops_bound_sqlx_types_decimal_handle.clone(),
            });
        let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney(
            sqlx::postgres::types::PgMoney(std_primitive_i64_handle.clone()),
        );
        let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText(
            sqlx::postgres::types::PgCiText(std_string_string_handle.clone()),
        );
        let sqlx_types_big_decimal = SqlxTypesBigDecimal(sqlx_types_big_decimal_handle.clone());
        let sqlx_types_decimal = SqlxTypesDecimal(sqlx_types_decimal_handle.clone());
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Utc,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_utc_handle.clone(),
            ));
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Local,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_fixed_offset_handle,
            ));
        let sqlx_types_chrono_naive_date_time =
            SqlxTypesChronoNaiveDateTime(sqlx_types_chrono_naive_date_time_handle.clone());
        let sqlx_types_chrono_naive_date =
            SqlxTypesChronoNaiveDate(sqlx_types_chrono_naive_date_handle.clone());
        let sqlx_types_chrono_naive_time =
            SqlxTypesChronoNaiveTime(sqlx_types_chrono_naive_time_handle.clone());
        let sqlx_postgres_types_pg_time_tz =
            SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz {
                time: sqlx_types_time_time_handle.clone(),
                offset: sqlx::types::time::UtcOffset::from_hms(
                    std_primitive_i8_handle.clone(),
                    std_primitive_i8_handle.clone(),
                    std_primitive_i8_handle.clone(),
                )
                .unwrap(),
            });
        let sqlx_types_time_primitive_date_time =
            SqlxTypesTimePrimitiveDateTime(sqlx_types_time_primitive_date_time_handle.clone());
        let sqlx_types_time_offset_date_time =
            SqlxTypesTimeOffsetDateTime(sqlx_types_time_offset_date_time_handle.clone());
        let sqlx_types_time_date = SqlxTypesTimeDate(sqlx_types_time_date_handle.clone());
        let sqlx_types_time_time = SqlxTypesTimeTime(sqlx_types_time_time_handle.clone());
        let sqlx_types_uuid_uuid = SqlxTypesUuidUuid(sqlx::types::uuid::Uuid::from_u128(
            std::primitive::u128::default(),
        ));
        let sqlx_types_ipnetwork_ip_network =
            SqlxTypesIpnetworkIpNetwork(sqlx::types::ipnetwork::IpNetwork::V6(
                sqlx::types::ipnetwork::Ipv6Network::new(
                    std::net::Ipv6Addr::new(
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                        std_primitive_u16_handle.clone(),
                    ),
                    std_primitive_u8_handle.clone(),
                )
                .unwrap(),
            ));
        let std_net_ip_addr = StdNetIpAddr(std::net::IpAddr::V6(core::net::Ipv6Addr::new(
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
            std_primitive_u16_handle.clone(),
        )));
        let sqlx_types_mac_address_mac_address =
            SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress::new([
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
                std_primitive_u8_handle.clone(),
            ]));
        let sqlx_types_bit_vec = SqlxTypesBitVec(sqlx::types::BitVec::new());
        let sqlx_types_json = SqlxTypesJson(sqlx::types::Json(Something {
            something: std_string_string_handle.clone(),
        }));
        let serde_json_value =
            SerdeJsonValue(serde_json::Value::Bool(std::primitive::bool::default()));
        Self {
            std_primitive_bool,
            std_primitive_i16,
            std_primitive_i32,
            std_primitive_i64,
            std_primitive_f32,
            std_primitive_f64,
            std_string_string,
            std_vec_vec_std_primitive_u8,
            sqlx_postgres_types_pg_interval,
            sqlx_postgres_types_pg_range_std_primitive_i64,
            sqlx_postgres_types_pg_range_std_primitive_i32,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time,
            sqlx_types_chrono_naive_date_time,
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date,
            sqlx_postgres_types_pg_range_sqlx_types_time_date,
            sqlx_postgres_types_pg_range_sqlx_types_big_decimal,
            sqlx_postgres_types_pg_range_sqlx_types_decimal,
            sqlx_postgres_types_pg_money,
            sqlx_postgres_types_pg_ci_text,
            sqlx_types_big_decimal,
            sqlx_types_decimal,
            sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            sqlx_types_chrono_naive_date,
            sqlx_types_chrono_naive_time,
            sqlx_postgres_types_pg_time_tz,
            sqlx_types_time_primitive_date_time,
            sqlx_types_time_offset_date_time,
            sqlx_types_time_date,
            sqlx_types_time_time,
            sqlx_types_uuid_uuid,
            sqlx_types_ipnetwork_ip_network,
            std_net_ip_addr,
            sqlx_types_mac_address_mac_address,
            sqlx_types_bit_vec,
            sqlx_types_json,
            serde_json_value,
        }
    }
}

pub trait IntoSerdeSerializeDeserialize {}

pub trait PostgresqlFilter {}

// impl PostgresqlFilter for sqlx::types:: {}

pub trait PostgresqlOrder {}

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

pub trait CheckSupportedPostgresqlColumnType {
    fn check_supported_postgresql_column_type();
}
//new type pattern
// sqlx::Encode impl was copied from https://docs.rs/sqlx/0.7.3/sqlx/trait.Encode.html
#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdPrimitiveBool(pub std::primitive::bool); //todo maybe make it private?
impl AsPostgresqlBool for StdPrimitiveBool {}
impl PostgresqlOrder for StdPrimitiveBool {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
impl AsPostgresqlSmallInt for StdPrimitiveI16 {}
impl AsPostgresqlSmallSerial for StdPrimitiveI16 {}
impl AsPostgresqlInt2 for StdPrimitiveI16 {}
impl PostgresqlOrder for StdPrimitiveI16 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
impl AsPostgresqlInt for StdPrimitiveI32 {}
impl AsPostgresqlSerial for StdPrimitiveI32 {}
impl AsPostgresqlInt4 for StdPrimitiveI32 {}
impl PostgresqlOrder for StdPrimitiveI32 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
impl AsPostgresqlBigInt for StdPrimitiveI64 {}
impl AsPostgresqlBigSerial for StdPrimitiveI64 {}
impl AsPostgresqlInt8 for StdPrimitiveI64 {}
impl PostgresqlOrder for StdPrimitiveI64 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
impl AsPostgresqlReal for StdPrimitiveF32 {}
impl AsPostgresqlFloat4 for StdPrimitiveF32 {}
impl PostgresqlOrder for StdPrimitiveF32 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
impl AsPostgresqlDoublePrecision for StdPrimitiveF64 {}
impl AsPostgresqlFloat8 for StdPrimitiveF64 {}
impl PostgresqlOrder for StdPrimitiveF64 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdStringString(pub std::string::String);
impl AsPostgresqlVarchar for StdStringString {}
impl AsPostgresqlCharN for StdStringString {}
impl AsPostgresqlText for StdStringString {}
impl AsPostgresqlCiText for StdStringString {}
impl PostgresqlOrder for StdStringString {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
impl AsPostgresqlBytea for StdVecVecStdPrimitiveU8 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgIntervalWithSerializeDeserialize {
    months: std::primitive::i32,
    days: std::primitive::i32,
    microseconds: std::primitive::i64,
}
impl std::convert::From<SqlxPostgresTypesPgIntervalWithSerializeDeserialize>
    for SqlxPostgresTypesPgInterval
{
    fn from(value: SqlxPostgresTypesPgIntervalWithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgInterval {
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgInterval>
    for SqlxPostgresTypesPgIntervalWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgInterval) -> Self {
        Self {
            months: value.0.months,
            days: value.0.days,
            microseconds: value.0.microseconds,
        }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgIntervalWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "months: {}, days: {}, microseconds: {}", self.months, self.days, self.microseconds)
    }
}
impl AsPostgresqlInterval for SqlxPostgresTypesPgInterval {}
impl PostgresqlOrder for SqlxPostgresTypesPgInterval {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(
    pub sqlx::postgres::types::PgRange<std::primitive::i64>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize {
    start: std::ops::Bound<std::primitive::i64>,
    end: std::ops::Bound<std::primitive::i64>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize>
    for SqlxPostgresTypesPgRangeStdPrimitiveI64
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: value.start,
            end: value.end,
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64>
    for SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64) -> Self {
        Self {
            start: value.0.start,
            end: value.0.end,
        }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlInt8Range for SqlxPostgresTypesPgRangeStdPrimitiveI64 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(
    pub sqlx::postgres::types::PgRange<std::primitive::i32>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize {
    start: std::ops::Bound<std::primitive::i32>,
    end: std::ops::Bound<std::primitive::i32>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize>
    for SqlxPostgresTypesPgRangeStdPrimitiveI32
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: value.start,
            end: value.end,
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32>
    for SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32) -> Self {
        Self {
            start: value.0.start,
            end: value.0.end,
        }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize
{
    start: std::ops::Bound<
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
    >,
    end: std::ops::Bound<
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
    >,
}
impl std::convert::From<
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize) -> Self {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded)
            }
        };
        Self(sqlx::postgres::types::PgRange { start, end })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        let (start, end) = match (value.0.start, value.0.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)
                )),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)
                ))
            ),
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)
                )),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)
                ))
            ),
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)
                )), 
                std::ops::Bound::Unbounded
            ),
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)
                )),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)
                ))
            ),
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)
                )),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)
                ))
            ),
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)
                )), 
                std::ops::Bound::Unbounded
            ),
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)
                ))
            ),
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize::from(
                    SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)
                ))
            ),
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize
{
    start: std::ops::Bound<
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
    >,
    end: std::ops::Bound<
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
    >,
}
impl std::convert::From<
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize) -> Self {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Unbounded
            )
        };
        Self(sqlx::postgres::types::PgRange { start, end })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        let (start, end) = match (value.0.start, value.0.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value))),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)))
            ),
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value))),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)))
            ),
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value))), 
                std::ops::Bound::Unbounded
            ),
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value))),
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)))
            ),
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value))),
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)))
            ),
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value))), 
                std::ops::Bound::Unbounded
            ),
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)))
            ),
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize::from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)))
            ),
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize>,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed {
    Start {
        #[eo_error_occurence]
        start: SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence
    },
    End {
        #[eo_error_occurence]
        end: SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence
    },
    StartEnd {
        #[eo_error_occurence]
        start: SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed,
        #[eo_error_occurence]
        end: SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence
    },
}
impl std::convert::TryFrom<
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
{
    type Error =
        SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => {
                match (
                    SqlxTypesTimeOffsetDateTime::try_from(start_value),
                    SqlxTypesTimeOffsetDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Included(start_value.0),
                        std::ops::Bound::Included(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => {
                match (
                    SqlxTypesTimeOffsetDateTime::try_from(start_value),
                    SqlxTypesTimeOffsetDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Included(start_value.0),
                        std::ops::Bound::Excluded(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => {
                match SqlxTypesTimeOffsetDateTime::try_from(start_value) {
                    Ok(value) => (
                        std::ops::Bound::Included(value.0),
                        std::ops::Bound::Unbounded,
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => {
                match (
                    SqlxTypesTimeOffsetDateTime::try_from(start_value),
                    SqlxTypesTimeOffsetDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Excluded(start_value.0),
                        std::ops::Bound::Included(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => {
                match (
                    SqlxTypesTimeOffsetDateTime::try_from(start_value),
                    SqlxTypesTimeOffsetDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Excluded(start_value.0),
                        std::ops::Bound::Excluded(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => {
                match SqlxTypesTimeOffsetDateTime::try_from(start_value) {
                    Ok(value) => (
                        std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded,
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => {
                match SqlxTypesTimeOffsetDateTime::try_from(end_value) {
                    Ok(value) => (
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Included(value.0),
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => {
                match SqlxTypesTimeOffsetDateTime::try_from(end_value) {
                    Ok(value) => (
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Excluded(value.0),
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded)
            }
        };
        Ok(Self(sqlx::postgres::types::PgRange { start, end }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime>
    for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimeOffsetDateTime(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimeOffsetDateTime(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimeOffsetDateTime(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimeOffsetDateTime(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize>,
}
impl std::convert::From<
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize,
> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize) -> Self {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Unbounded
            )
        };
        Self(sqlx::postgres::types::PgRange { start, end })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize>,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed
{
    Start {
        #[eo_error_occurence]
        start: SqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    End {
        #[eo_error_occurence]
        end: SqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StartEnd {
        #[eo_error_occurence]
        start: SqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed,
        #[eo_error_occurence]
        end: SqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
{
    type Error = SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => {
                match (
                    SqlxTypesTimePrimitiveDateTime::try_from(start_value),
                    SqlxTypesTimePrimitiveDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Included(start_value.0),
                        std::ops::Bound::Included(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => {
                match (
                    SqlxTypesTimePrimitiveDateTime::try_from(start_value),
                    SqlxTypesTimePrimitiveDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Included(start_value.0),
                        std::ops::Bound::Excluded(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => {
                match SqlxTypesTimePrimitiveDateTime::try_from(start_value) {
                    Ok(value) => (
                        std::ops::Bound::Included(value.0),
                        std::ops::Bound::Unbounded,
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => {
                match (
                    SqlxTypesTimePrimitiveDateTime::try_from(start_value),
                    SqlxTypesTimePrimitiveDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Excluded(start_value.0),
                        std::ops::Bound::Included(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => {
                match (
                    SqlxTypesTimePrimitiveDateTime::try_from(start_value),
                    SqlxTypesTimePrimitiveDateTime::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Excluded(start_value.0),
                        std::ops::Bound::Excluded(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => {
                match SqlxTypesTimePrimitiveDateTime::try_from(start_value) {
                    Ok(value) => (
                        std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded,
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => {
                match SqlxTypesTimePrimitiveDateTime::try_from(end_value) {
                    Ok(value) => (
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Included(value.0),
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => {
                match SqlxTypesTimePrimitiveDateTime::try_from(end_value) {
                    Ok(value) => (
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Excluded(value.0),
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded)
            }
        };
        Ok(Self(sqlx::postgres::types::PgRange { start, end }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime>
    for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimePrimitiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimePrimitiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimePrimitiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize::from(
                    SqlxTypesTimePrimitiveDateTime(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsRange for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesChronoNaiveDateWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesChronoNaiveDateWithSerializeDeserialize>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize) -> Self {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoNaiveDate::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoNaiveDate::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Included(SqlxTypesChronoNaiveDate::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate::from(end_value).0),
            ),
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Included(SqlxTypesChronoNaiveDate::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate::from(start_value).0),
                std::ops::Bound::Included(SqlxTypesChronoNaiveDate::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate::from(start_value).0),
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate::from(end_value).0),
            ),
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => (
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate::from(start_value).0),
                std::ops::Bound::Unbounded,
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(SqlxTypesChronoNaiveDate::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate::from(end_value).0),
            ),
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (
                std::ops::Bound::Unbounded, 
                std::ops::Bound::Unbounded
            )
        };
        Self(sqlx::postgres::types::PgRange { start, end })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDate(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDate(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDate(*value),
                ),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateWithSerializeDeserialize::from(
                    SqlxTypesChronoNaiveDate(*value),
                ),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesTimeDateWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesTimeDateWithSerializeDeserialize>,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserializeErrorNamed {
    Start {
        #[eo_error_occurence]
        start: SqlxTypesTimeDateWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    End {
        #[eo_error_occurence]
        end: SqlxTypesTimeDateWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StartEnd {
        #[eo_error_occurence]
        start: SqlxTypesTimeDateWithSerializeDeserializeErrorNamed,
        #[eo_error_occurence]
        end: SqlxTypesTimeDateWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (std::ops::Bound::Included(start_value), std::ops::Bound::Included(end_value)) => {
                match (
                    SqlxTypesTimeDate::try_from(start_value),
                    SqlxTypesTimeDate::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Included(start_value.0),
                        std::ops::Bound::Included(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Included(start_value), std::ops::Bound::Excluded(end_value)) => {
                match (
                    SqlxTypesTimeDate::try_from(start_value),
                    SqlxTypesTimeDate::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Included(start_value.0),
                        std::ops::Bound::Excluded(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Included(start_value), std::ops::Bound::Unbounded) => {
                match SqlxTypesTimeDate::try_from(start_value) {
                    Ok(value) => (
                        std::ops::Bound::Included(value.0),
                        std::ops::Bound::Unbounded,
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Included(end_value)) => {
                match (
                    SqlxTypesTimeDate::try_from(start_value),
                    SqlxTypesTimeDate::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Excluded(start_value.0),
                        std::ops::Bound::Included(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Excluded(end_value)) => {
                match (
                    SqlxTypesTimeDate::try_from(start_value),
                    SqlxTypesTimeDate::try_from(end_value),
                ) {
                    (Ok(start_value), Ok(end_value)) => (
                        std::ops::Bound::Excluded(start_value.0),
                        std::ops::Bound::Excluded(end_value.0),
                    ),
                    (Ok(_), Err(e)) => return Err(Self::Error::End { 
                        end: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(e), Ok(_)) => return Err(Self::Error::Start { 
                        start: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(start_error), Err(end_error)) => {
                        return Err(Self::Error::StartEnd {
                            start: start_error,
                            end: end_error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            }
            (std::ops::Bound::Excluded(start_value), std::ops::Bound::Unbounded) => {
                match SqlxTypesTimeDate::try_from(start_value) {
                    Ok(value) => (
                        std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded,
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Included(end_value)) => {
                match SqlxTypesTimeDate::try_from(end_value) {
                    Ok(value) => (
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Included(value.0),
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(end_value)) => {
                match SqlxTypesTimeDate::try_from(end_value) {
                    Ok(value) => (
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Excluded(value.0),
                    ),
                    Err(e) => {
                        return Err(Self::Error::Start { 
                            start: e,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
            (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded)
            }
        };
        Ok(Self(sqlx::postgres::types::PgRange { start, end }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeDate>
    for SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeDate) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeDateWithSerializeDeserialize::from(SqlxTypesTimeDate(
                    *value,
                )),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeDateWithSerializeDeserialize::from(SqlxTypesTimeDate(
                    *value,
                )),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeDateWithSerializeDeserialize::from(SqlxTypesTimeDate(
                    *value,
                )),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeDateWithSerializeDeserialize::from(SqlxTypesTimeDate(
                    *value,
                )),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesBigDecimalWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesBigDecimalWithSerializeDeserialize>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize>
    for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize) -> Self {
        let start = match value.start {
            std::ops::Bound::Included(value) => {
                std::ops::Bound::Included(SqlxTypesBigDecimal::from(value).0)
            }
            std::ops::Bound::Excluded(value) => {
                std::ops::Bound::Excluded(SqlxTypesBigDecimal::from(value).0)
            }
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.end {
            std::ops::Bound::Included(value) => {
                std::ops::Bound::Included(SqlxTypesBigDecimal::from(value).0)
            }
            std::ops::Bound::Excluded(value) => {
                std::ops::Bound::Excluded(SqlxTypesBigDecimal::from(value).0)
            }
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self(sqlx::postgres::types::PgRange { start, end })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal>
    for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => {
                std::ops::Bound::Included(SqlxTypesBigDecimalWithSerializeDeserialize::from(
                    SqlxTypesBigDecimal(value.clone()),
                ))
            }
            std::ops::Bound::Excluded(value) => {
                std::ops::Bound::Excluded(SqlxTypesBigDecimalWithSerializeDeserialize::from(
                    SqlxTypesBigDecimal(value.clone()),
                ))
            }
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => {
                std::ops::Bound::Included(SqlxTypesBigDecimalWithSerializeDeserialize::from(
                    SqlxTypesBigDecimal(value.clone()),
                ))
            }
            std::ops::Bound::Excluded(value) => {
                std::ops::Bound::Excluded(SqlxTypesBigDecimalWithSerializeDeserialize::from(
                    SqlxTypesBigDecimal(value.clone()),
                ))
            }
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>,
);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize {
    start: std::ops::Bound<SqlxTypesDecimalWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesDecimalWithSerializeDeserialize>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize>
    for SqlxPostgresTypesPgRangeSqlxTypesDecimal
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize) -> Self {
        let start = match value.start {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.end {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self(sqlx::postgres::types::PgRange { start, end })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimal>
    for SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesDecimal) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesDecimalWithSerializeDeserialize::from(SqlxTypesDecimal(*value)),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesDecimalWithSerializeDeserialize::from(SqlxTypesDecimal(*value)),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesDecimalWithSerializeDeserialize::from(SqlxTypesDecimal(*value)),
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesDecimalWithSerializeDeserialize::from(SqlxTypesDecimal(*value)),
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self { start, end }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesDecimal {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgMoneyWithSerializeDeserialize(std::primitive::i64);
impl std::convert::From<SqlxPostgresTypesPgMoneyWithSerializeDeserialize>
    for SqlxPostgresTypesPgMoney
{
    fn from(value: SqlxPostgresTypesPgMoneyWithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgMoney(value.0))
    }
}
impl std::convert::From<SqlxPostgresTypesPgMoney>
    for SqlxPostgresTypesPgMoneyWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgMoney) -> Self {
        Self(value.0 .0)
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgMoneyWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsPostgresqlMoney for SqlxPostgresTypesPgMoney {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgCiTextWithSerializeDeserialize(std::string::String);
impl std::convert::From<SqlxPostgresTypesPgCiTextWithSerializeDeserialize>
    for SqlxPostgresTypesPgCiText
{
    fn from(value: SqlxPostgresTypesPgCiTextWithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgCiText(value.0))
    }
}
impl std::convert::From<SqlxPostgresTypesPgCiText>
    for SqlxPostgresTypesPgCiTextWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgCiText) -> Self {
        Self(value.0.0)
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgCiTextWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsPostgresqlCiText for SqlxPostgresTypesPgCiText {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesBigDecimalWithSerializeDeserialize {
    digits: NumBigintBigIntWithSerializeDeserialize,
    scale: std::primitive::i64,
}
impl std::convert::From<SqlxTypesBigDecimalWithSerializeDeserialize> for SqlxTypesBigDecimal {
    fn from(value: SqlxTypesBigDecimalWithSerializeDeserialize) -> Self {
        Self(sqlx::types::BigDecimal::new(
            num_bigint::BigInt::from(value.digits),
            value.scale,
        ))
    }
}
impl std::convert::From<SqlxTypesBigDecimal> for SqlxTypesBigDecimalWithSerializeDeserialize {
    fn from(value: SqlxTypesBigDecimal) -> Self {
        let (bigint, exponent) = value.0.into_bigint_and_exponent();
        Self {
            digits: NumBigintBigIntWithSerializeDeserialize::from(bigint),
            scale: exponent, //todo is exponent equal to scale?
        }
    }
}
impl std::fmt::Display for SqlxTypesBigDecimalWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "digits: {:?}, scale: {:?}", self.digits, self.scale)
    }
}
impl AsPostgresqlNumeric for SqlxTypesBigDecimal {}
impl PostgresqlOrder for SqlxTypesBigDecimal {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
);

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
);
impl AsPostgresqlTimestampTz for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
impl AsPostgresqlTimestamp for SqlxTypesChronoNaiveDateTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDateTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
impl AsPostgresqlDate for SqlxTypesChronoNaiveDate {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDate {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
impl AsPostgresqlTime for SqlxTypesChronoNaiveTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgTimeTzWithSerializeDeserialize {
    time: SqlxTypesTimeTimeWithSerializeDeserialize,
    offset: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxPostgresTypesPgTimeTzWithSerializeDeserializeErrorNamed {
    Time {
        #[eo_error_occurence]
        time: SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Offset {
        #[eo_error_occurence]
        offset: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TimeOffset {
        #[eo_error_occurence]
        time: SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed,
        #[eo_error_occurence]
        offset: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxPostgresTypesPgTimeTzWithSerializeDeserialize> for SqlxPostgresTypesPgTimeTz {
    type Error = SqlxPostgresTypesPgTimeTzWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxPostgresTypesPgTimeTzWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let (time, offset) = match (
            SqlxTypesTimeTime::try_from(value.time),
            sqlx::types::time::UtcOffset::try_from(value.offset),
        ) {
            (Ok(time), Ok(offset)) => (time.0, offset),
            (Err(e), Ok(_)) => {
                return Err(Self::Error::Time { 
                    time: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            (Ok(_), Err(e)) => {
                return Err(Self::Error::Offset { 
                    offset: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            (Err(time_error), Err(offset_error)) => {
                return Err(Self::Error::TimeOffset {
                    time: time_error,
                    offset: offset_error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(sqlx::postgres::types::PgTimeTz { time, offset }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgTimeTz>
    for SqlxPostgresTypesPgTimeTzWithSerializeDeserialize
{
    fn from(value: SqlxPostgresTypesPgTimeTz) -> Self {
        Self {
            //todo impl from directly from type?
            time: SqlxTypesTimeTimeWithSerializeDeserialize::from(SqlxTypesTimeTime(
                value.0.time,
            )),
            offset: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize::from(value.0.offset),
        }
    }
}
impl std::fmt::Display for SqlxPostgresTypesPgTimeTzWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "time: {}, offset: {:?}", self.time, self.offset)
    }
}
impl AsPostgresqlTimeTz for SqlxPostgresTypesPgTimeTz {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize {
    date: SqlxTypesTimeDateWithSerializeDeserialize,
    time: SqlxTypesTimeTimeWithSerializeDeserialize,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed {
    Date {
        #[eo_error_occurence]
        date: SqlxTypesTimeDateWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Time {
        #[eo_error_occurence]
        time: SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DateTime {
        #[eo_error_occurence]
        date: SqlxTypesTimeDateWithSerializeDeserializeErrorNamed,
        #[eo_error_occurence]
        time: SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize> for SqlxTypesTimePrimitiveDateTime {
    type Error = SqlxTypesTimePrimitiveDateTimeWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let (date, time) = match (
            SqlxTypesTimeDate::try_from(value.date),
            SqlxTypesTimeTime::try_from(value.time),
        ) {
            (Ok(date), Ok(time)) => (date, time),
            (Err(e), Ok(_)) => {
                return Err(Self::Error::Date { 
                    date: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            (Ok(_), Err(e)) => {
                return Err(Self::Error::Time { 
                    time: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            (Err(date_error), Err(time_error)) => {
                return Err(Self::Error::DateTime {
                    date: date_error,
                    time: time_error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(sqlx::types::time::PrimitiveDateTime::new(
            date.0, time.0,
        )))
    }
}
impl std::convert::From<SqlxTypesTimePrimitiveDateTime>
    for SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize
{
    fn from(value: SqlxTypesTimePrimitiveDateTime) -> Self {
        Self {
            //todo impl from directly from type?
            date: SqlxTypesTimeDateWithSerializeDeserialize::from(
                SqlxTypesTimeDate(value.0.date()),
            ),
            time: SqlxTypesTimeTimeWithSerializeDeserialize::from(SqlxTypesTimeTime(
                value.0.time(),
            )),
        }
    }
}
impl std::fmt::Display for SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "date: {}, time: {}", self.date, self.time)
    }
}
impl AsPostgresqlTimestamp for SqlxTypesTimePrimitiveDateTime {}
impl PostgresqlOrder for SqlxTypesTimePrimitiveDateTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize(
    std::primitive::i64,
);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_display]
        time_error_component_range: time::error::ComponentRange,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize> for SqlxTypesTimeOffsetDateTime {
    type Error = SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        match sqlx::types::time::OffsetDateTime::from_unix_timestamp(value.0) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(Self::Error::TimeErrorComponentRange {
                time_error_component_range: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
}
impl std::convert::From<SqlxTypesTimeOffsetDateTime>
    for SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize
{
    fn from(value: SqlxTypesTimeOffsetDateTime) -> Self {
        Self(value.0.unix_timestamp())
    }
}
impl std::fmt::Display for SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsPostgresqlTimestampTz for SqlxTypesTimeOffsetDateTime {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeDateWithSerializeDeserialize {
    year: std::primitive::i32,
    month: TimeMonthWithSerializeDeserialize,
    day: std::primitive::u8,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeDateWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_display]
        time_error_component_range: time::error::ComponentRange,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxTypesTimeDateWithSerializeDeserialize> for SqlxTypesTimeDate {
    type Error = SqlxTypesTimeDateWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesTimeDateWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {//todo maybe use better initialize function (not ony for what)
        match sqlx::types::time::Date::from_calendar_date(
            value.year,
            time::Month::from(value.month),
            value.day,
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(Self::Error::TimeErrorComponentRange {
                time_error_component_range: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
}
impl std::convert::From<SqlxTypesTimeDate>
    for SqlxTypesTimeDateWithSerializeDeserialize
{
    fn from(value: SqlxTypesTimeDate) -> Self {
        Self {
            year: value.0.year(),
            month: value.0.month().into(),
            day: value.0.day(),
        }
    }
}
impl std::fmt::Display for SqlxTypesTimeDateWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "year: {}, month: {:?}, day: {}", self.year, self.month, self.day)
    }
}
impl AsPostgresqlDate for SqlxTypesTimeDate {}
impl PostgresqlOrder for SqlxTypesTimeDate {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeTimeWithSerializeDeserialize {
    hour: std::primitive::u8,
    minute: std::primitive::u8,
    second: std::primitive::u8,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_display]
        time_error_component_range: time::error::ComponentRange,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
//todo different init methods support
impl std::convert::TryFrom<SqlxTypesTimeTimeWithSerializeDeserialize> for SqlxTypesTimeTime {
    type Error = SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesTimeTimeWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        match sqlx::types::time::Time::from_hms(value.hour, value.minute, value.second) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(Self::Error::TimeErrorComponentRange{
                time_error_component_range: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
}
impl std::convert::From<SqlxTypesTimeTime> for SqlxTypesTimeTimeWithSerializeDeserialize {
    fn from(value: SqlxTypesTimeTime) -> Self {
        Self {
            hour: value.0.hour(),
            minute: value.0.minute(),
            second: value.0.second(),
        }
    }
}
impl std::fmt::Display for SqlxTypesTimeTimeWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hour: {}, minute: {}, second: {}", self.hour, self.minute, self.second)
    }
}
impl AsPostgresqlTime for SqlxTypesTimeTime {}
impl PostgresqlOrder for SqlxTypesTimeTime {}
//todo maybe its possible to not use Clone (refactor where .clone() used)
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonTryFrom)]
pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesUuidUuidWithSerializeDeserialize(std::string::String);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidError {
        #[eo_display]
        sqlx_types_uuid_error: sqlx::types::uuid::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxTypesUuidUuidWithSerializeDeserialize> for SqlxTypesUuidUuid {
    type Error = SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesUuidUuidWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        match sqlx::types::uuid::Uuid::try_parse(&value.0) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(Self::Error::SqlxTypesUuidError{
                sqlx_types_uuid_error: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
}
impl std::convert::From<SqlxTypesUuidUuid> for SqlxTypesUuidUuidWithSerializeDeserialize {
    fn from(value: SqlxTypesUuidUuid) -> Self {
        Self(value.0.to_string())
    }
}
impl std::fmt::Display for SqlxTypesUuidUuidWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl AsPostgresqlUuid for SqlxTypesUuidUuid {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
impl AsPostgresqlInet for SqlxTypesIpnetworkIpNetwork {}
impl AsPostgresqlCidr for SqlxTypesIpnetworkIpNetwork {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct StdNetIpAddr(pub std::net::IpAddr);
impl AsPostgresqlInet for StdNetIpAddr {}
impl AsPostgresqlCidr for StdNetIpAddr {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesMacAddressMacAddressWithSerializeDeserialize([std::primitive::u8; 6]);
impl std::convert::From<SqlxTypesMacAddressMacAddressWithSerializeDeserialize>
    for SqlxTypesMacAddressMacAddress
{
    fn from(value: SqlxTypesMacAddressMacAddressWithSerializeDeserialize) -> Self {
        Self(sqlx::types::mac_address::MacAddress::new(value.0))
    }
}
impl std::convert::From<SqlxTypesMacAddressMacAddress>
    for SqlxTypesMacAddressMacAddressWithSerializeDeserialize
{
    fn from(value: SqlxTypesMacAddressMacAddress) -> Self {
        Self(value.0.bytes())
    }
}
impl std::fmt::Display for SqlxTypesMacAddressMacAddressWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl AsPostgresqlMacAddr for SqlxTypesMacAddressMacAddress {}

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::Common, postgresql_crud_types_macro_logic_reuse::CommonFrom)]
pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesBitVecWithSerializeDeserialize(std::vec::Vec<std::primitive::u8>);
impl std::convert::From<SqlxTypesBitVecWithSerializeDeserialize> for SqlxTypesBitVec {
    fn from(value: SqlxTypesBitVecWithSerializeDeserialize) -> Self {
        Self(sqlx::types::BitVec::from_bytes(&value.0))
    }
}
impl std::convert::From<SqlxTypesBitVec> for SqlxTypesBitVecWithSerializeDeserialize {
    fn from(value: SqlxTypesBitVec) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| Into::into(element))
                .collect::<std::vec::Vec<std::primitive::u8>>(),
        )
    }
}
impl std::fmt::Display for SqlxTypesBitVecWithSerializeDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl AsPostgresqlBit for SqlxTypesBitVec {}
impl AsPostgresqlVarBit for SqlxTypesBitVec {}
impl PostgresqlOrder for SqlxTypesBitVec {}

#[derive(Debug, PartialEq)]
pub struct SqlxTypesJson<T>(sqlx::types::Json<T>);
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesJsonWithSerializeDeserialize<T>(sqlx::types::Json<T>);
impl<T> std::convert::From<SqlxTypesJsonWithSerializeDeserialize<T>> for SqlxTypesJson<T> {
    fn from(value: SqlxTypesJsonWithSerializeDeserialize<T>) -> Self {
        Self(value.0)
    }
}
impl<T> std::convert::From<SqlxTypesJson<T>> for SqlxTypesJsonWithSerializeDeserialize<T> {
    fn from(value: SqlxTypesJson<T>) -> Self {
        Self(value.0)
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
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
        + serde::Deserialize<'a>
        + utoipa::ToSchema<'a>, //todo maybe add another traits impls
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
    fn size_hint(&self) -> std::primitive::usize {
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
        + serde::Deserialize<'a>
        + utoipa::ToSchema<'a>, //todo maybe add another traits impls
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
// impl std::fmt::Display for  SqlxTypesJson<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.0)
//     }
// }
impl<T> AsPostgresqlJson for SqlxTypesJson<T> {}
impl<T> AsPostgresqlJsonB for SqlxTypesJson<T> {}
impl<T> std::convert::From<SqlxTypesJson<T>> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesJson<T>) -> Self {
        SupportedSqlxPostgresType::SqlxTypesJsonT
    }
}
impl<T> SqlxTypesJson<T> {
    pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value
            .into_iter()
            .map(|element| element.into_inner())
            .collect()
    }
}
// impl std::convert::From<> for {
//     fn from(value: ) -> Self {
//         value.0
//     }
// }
//todo impl for bind query
// impl<T: serde::Serialize + std::marker::Send> BindQuery for SqlxTypesJson<T> {
//     fn try_increment(&self, increment: &mut u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(())
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                 checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             })
//         }
//     }
//     fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::default();
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("${increment}"));
//             }
//             None => {
//                 return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                     checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 });
//             }
//         }
//         Ok(increments)
//     }
//     fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> 
//     where T: 'a 
//     {
//         query = query.bind(self.0);
//         query
//     }
// }
//todo 
// #[derive(Debug)]
// pub struct WhereSqlxTypesJson {
//     pub value: SqlxTypesJson,
//     pub conjuctive_operator: ConjunctiveOperator,
// }
// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct WhereSqlxTypesJsonWithSerializeDeserialize {
//     pub value: SqlxTypesJsonWithSerializeDeserialize,
//     pub conjuctive_operator: ConjunctiveOperator,
// }
// impl std::convert::From<WhereSqlxTypesJsonWithSerializeDeserialize> for WhereSqlxTypesJson {
//     fn from(value: WhereSqlxTypesJsonWithSerializeDeserialize) -> Self {
//         Self {
//             value: SqlxTypesJson::from(value.value),
//             conjuctive_operator: value.conjuctive_operator
//         }
//     }
// }

#[derive(Debug, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserialize, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::Common)]
pub struct SerdeJsonValue(pub serde_json::Value);
impl AsPostgresqlJson for SerdeJsonValue {}
impl AsPostgresqlJsonB for SerdeJsonValue {}

pub async fn something() {
    // let mut query = sqlx::query::<sqlx::Postgres>("test");
    // query = query.bind(Into::<bool>::into(StdPrimitiveBool(false)));
    // query = query.bind(StdPrimitiveBool(false).into_inner());
    // let _query = query.bind(StdPrimitiveBool(false));
}

pub fn test_check_supported_postgresql_column_type() {
    //todo check if init functions are not panics. change to not panic init functions
    StdPrimitiveBool::check_supported_postgresql_column_type();
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
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::check_supported_postgresql_column_type(
    );
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesDecimal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgMoney::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgCiText::check_supported_postgresql_column_type();
    SqlxTypesBigDecimal::check_supported_postgresql_column_type();
    SqlxTypesDecimal::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc::check_supported_postgresql_column_type();
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
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
}

pub trait AsPostgresqlBool {}
pub trait AsPostgresqlChar {}
pub trait AsPostgresqlSmallInt {}
pub trait AsPostgresqlSmallSerial {}
pub trait AsPostgresqlInt2 {}
pub trait AsPostgresqlInt {}
pub trait AsPostgresqlSerial {}
pub trait AsPostgresqlInt4 {}
pub trait AsPostgresqlBigInt {}
pub trait AsPostgresqlBigSerial {}
pub trait AsPostgresqlInt8 {}
pub trait AsPostgresqlReal {}
pub trait AsPostgresqlFloat4 {}
pub trait AsPostgresqlDoublePrecision {}
pub trait AsPostgresqlFloat8 {}
pub trait AsPostgresqlVarchar {}
pub trait AsPostgresqlCharN {}
pub trait AsPostgresqlText {}
pub trait AsPostgresqlCiText {}
pub trait AsPostgresqlBytea {}
pub trait AsPostgresqlInterval {}
pub trait AsPostgresqlInt8Range {}
pub trait AsPostgresqlInt4Range {}
pub trait AsPostgresqlTsRange {}
pub trait AsPostgresqlTsTzRange {}
pub trait AsPostgresqlDateRange {}
pub trait AsPostgresqlNumRange {}
pub trait AsPostgresqlMoney {}
pub trait AsPostgresqlNumeric {}
pub trait AsPostgresqlTimestampTz {}
pub trait AsPostgresqlTimestamp {}
pub trait AsPostgresqlDate {}
pub trait AsPostgresqlTime {}
pub trait AsPostgresqlTimeTz {}
pub trait AsPostgresqlUuid {}
pub trait AsPostgresqlInet {}
pub trait AsPostgresqlCidr {}
pub trait AsPostgresqlMacAddr {}
pub trait AsPostgresqlBit {}
pub trait AsPostgresqlVarBit {}
pub trait AsPostgresqlJson {}
pub trait AsPostgresqlJsonB {}

const CHECKED_ADD_IS_NONE: &str = "checked_add is None";
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateBindIncrementsErrorNamed {
    CheckedAdd {
        #[eo_display_with_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub trait BindQuery {
    fn try_increment(&self, increment: &mut u64) -> Result<(), TryGenerateBindIncrementsErrorNamed>;
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed>;
    fn bind_value_to_query(self, query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub enum ConjunctiveOperator {
    Or,
    And,
}

impl std::fmt::Display for ConjunctiveOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConjunctiveOperator::Or => write!(f, "{}", naming_constants::OR),
            ConjunctiveOperator::And => {
                write!(f, "{}", naming_constants::AND)
            }
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct RegexFilter {
    pub regex: std::string::String,
    pub conjuctive_operator: ConjunctiveOperator,
}
impl std::fmt::Display for RegexFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "regex: {}, conjuctive_operator: {}",
            self.regex, self.conjuctive_operator
        )
    }
}
impl BindQuery for RegexFilter {
    fn try_increment(&self, increment: &mut u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(())
            },
            None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                checked_add: std::string::String::from("checked_add is None"),
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<
        std::string::String,
        TryGenerateBindIncrementsErrorNamed,
    > {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("${increment}"))
            },
            None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                checked_add: std::string::String::from("checked_add is None"),
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.regex);
        query
    }
}
impl BindQuery for std::vec::Vec<RegexFilter> {
    fn try_increment(&self, increment: &mut u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        for _ in self {
            match increment.checked_add(1) {
                Some(incr) => {
                    *increment = incr;
                }
                None => {
                    return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from("checked_add is None"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(())
    }
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<
        std::string::String,
        TryGenerateBindIncrementsErrorNamed,
    > {
        let mut value = std::string::String::default();
        for _ in self {
            match increment.checked_add(1) {
                Some(incr) => {
                    *increment = incr;
                    value.push_str(&format!("${increment},"));
                }
                None => {
                    return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from("checked_add is None"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        value.pop();
        Ok(value)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self {
            query = query.bind(element.regex);
        }
        query
    }
}
