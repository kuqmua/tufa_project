pub const POSTGRESQL_CRUD_SNAKE_CASE: &str = "postgresql_crud";

fn add_path(value: &str) -> std::string::String {
    format!("{POSTGRESQL_CRUD_SNAKE_CASE}::{value}")
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
            PostgresqlTypeWithMetadata::Bool |
            PostgresqlTypeWithMetadata::BoolNotNull => Self::Bool,
            PostgresqlTypeWithMetadata::Char |
            PostgresqlTypeWithMetadata::CharNotNull => Self::Char,
            PostgresqlTypeWithMetadata::SmallInt |
            PostgresqlTypeWithMetadata::SmallIntNotNull => Self::SmallInt,
            PostgresqlTypeWithMetadata::SmallSerial |
            PostgresqlTypeWithMetadata::SmallSerialNotNull => Self::SmallSerial,
            PostgresqlTypeWithMetadata::Int2 |
            PostgresqlTypeWithMetadata::Int2NotNull => Self::Int2,
            PostgresqlTypeWithMetadata::Int |
            PostgresqlTypeWithMetadata::IntNotNull => Self::Int,
            PostgresqlTypeWithMetadata::Serial |
            PostgresqlTypeWithMetadata::SerialNotNull => Self::Serial,
            PostgresqlTypeWithMetadata::Int4 |
            PostgresqlTypeWithMetadata::Int4NotNull => Self::Int4,
            PostgresqlTypeWithMetadata::BigInt |
            PostgresqlTypeWithMetadata::BigIntNotNull => Self::BigInt,
            PostgresqlTypeWithMetadata::BigSerial |
            PostgresqlTypeWithMetadata::BigSerialNotNull |
            PostgresqlTypeWithMetadata::BigSerialNotNullPrimaryKey => Self::BigSerial,
            PostgresqlTypeWithMetadata::Int8 |
            PostgresqlTypeWithMetadata::Int8NotNull => Self::Int8,
            PostgresqlTypeWithMetadata::Real |
            PostgresqlTypeWithMetadata::RealNotNull => Self::Real,
            PostgresqlTypeWithMetadata::Float4 |
            PostgresqlTypeWithMetadata::Float4NotNull => Self::Float4,
            PostgresqlTypeWithMetadata::DoublePrecision |
            PostgresqlTypeWithMetadata::DoublePrecisionNotNull => Self::DoublePrecision,
            PostgresqlTypeWithMetadata::Float8 |
            PostgresqlTypeWithMetadata::Float8NotNull => Self::Float8,
            PostgresqlTypeWithMetadata::Varchar |
            PostgresqlTypeWithMetadata::VarcharNotNull => Self::Varchar,
            PostgresqlTypeWithMetadata::CharN |
            PostgresqlTypeWithMetadata::CharNNotNull => Self::CharN,
            PostgresqlTypeWithMetadata::Text |
            PostgresqlTypeWithMetadata::TextNotNull => Self::Text,
            PostgresqlTypeWithMetadata::CiText |
            PostgresqlTypeWithMetadata::CiTextNotNull => Self::CiText,
            PostgresqlTypeWithMetadata::Bytea |
            PostgresqlTypeWithMetadata::ByteaNotNull => Self::Bytea,
            PostgresqlTypeWithMetadata::Interval |
            PostgresqlTypeWithMetadata::IntervalNotNull => Self::Interval,
            PostgresqlTypeWithMetadata::Int8Range |
            PostgresqlTypeWithMetadata::Int8RangeNotNull => Self::Int8Range,
            PostgresqlTypeWithMetadata::Int4Range |
            PostgresqlTypeWithMetadata::Int4RangeNotNull => Self::Int4Range,
            PostgresqlTypeWithMetadata::TsRange |
            PostgresqlTypeWithMetadata::TsRangeNotNull => Self::TsRange,
            PostgresqlTypeWithMetadata::TsTzRange |
            PostgresqlTypeWithMetadata::TsTzRangeNotNull => Self::TsTzRange,
            PostgresqlTypeWithMetadata::DateRange |
            PostgresqlTypeWithMetadata::DateRangeNotNull => Self::DateRange,
            PostgresqlTypeWithMetadata::NumRange |
            PostgresqlTypeWithMetadata::NumRangeNotNull => Self::NumRange,
            PostgresqlTypeWithMetadata::Money |
            PostgresqlTypeWithMetadata::MoneyNotNull => Self::Money,
            PostgresqlTypeWithMetadata::Numeric |
            PostgresqlTypeWithMetadata::NumericNotNull => Self::Numeric,
            PostgresqlTypeWithMetadata::TimestampTz |
            PostgresqlTypeWithMetadata::TimestampTzNotNull => Self::TimestampTz,
            PostgresqlTypeWithMetadata::Date |
            PostgresqlTypeWithMetadata::DateNotNull => Self::Date,
            PostgresqlTypeWithMetadata::Time |
            PostgresqlTypeWithMetadata::TimeNotNull => Self::Time,
            PostgresqlTypeWithMetadata::TimeTz |
            PostgresqlTypeWithMetadata::TimeTzNotNull => Self::TimeTz,
            PostgresqlTypeWithMetadata::Timestamp |
            PostgresqlTypeWithMetadata::TimestampNotNull => Self::Timestamp,
            PostgresqlTypeWithMetadata::Uuid |
            PostgresqlTypeWithMetadata::UuidNotNull |
            PostgresqlTypeWithMetadata::UuidNotNullPrimaryKey => Self::Uuid,
            PostgresqlTypeWithMetadata::Inet |
            PostgresqlTypeWithMetadata::InetNotNull => Self::Inet,
            PostgresqlTypeWithMetadata::Cidr |
            PostgresqlTypeWithMetadata::CidrNotNull => Self::Cidr,
            PostgresqlTypeWithMetadata::MacAddr |
            PostgresqlTypeWithMetadata::MacAddrNotNull => Self::MacAddr,
            PostgresqlTypeWithMetadata::Bit |
            PostgresqlTypeWithMetadata::BitNotNull => Self::Bit,
            PostgresqlTypeWithMetadata::VarBit |
            PostgresqlTypeWithMetadata::VarBitNotNull => Self::VarBit,
            PostgresqlTypeWithMetadata::Json |
            PostgresqlTypeWithMetadata::JsonNotNull => Self::Json,
            PostgresqlTypeWithMetadata::JsonB |
            PostgresqlTypeWithMetadata::JsonBNotNull => Self::JsonB,
        }
    }
}

impl PostgresqlTypeWithMetadata {
    //todo add NOT NULL or not? or add different method and Primary Key
    pub const fn postgresql_naming(&self) -> &str {
        match self {
            Self::Bool |
            Self::BoolNotNull => "BOOL",
            Self::Char |
            Self::CharNotNull => "CHAR",
            Self::SmallInt |
            Self::SmallIntNotNull => "SMALLINT",
            Self::SmallSerial |
            Self::SmallSerialNotNull => "SMALLSERIAL",
            Self::Int2 |
            Self::Int2NotNull => "INT2",
            Self::Int |
            Self::IntNotNull => "INT",
            Self::Serial |
            Self::SerialNotNull => "SERIAL",
            Self::Int4 |
            Self::Int4NotNull => "INT4",
            Self::BigInt |
            Self::BigIntNotNull => "BIGINT",
            Self::BigSerial |
            Self::BigSerialNotNull |
            Self::BigSerialNotNullPrimaryKey => "BIGSERIAL",
            Self::Int8 |
            Self::Int8NotNull => "INT8",
            Self::Real |
            Self::RealNotNull => "REAL",
            Self::Float4 |
            Self::Float4NotNull => "FLOAT4",
            Self::DoublePrecision |
            Self::DoublePrecisionNotNull => "DOUBLE PRECISION",
            Self::Float8 |
            Self::Float8NotNull => "FLOAT8",
            Self::Varchar |
            Self::VarcharNotNull => "VARCHAR",
            Self::CharN |
            Self::CharNNotNull => "CHAR(N)",
            Self::Text |
            Self::TextNotNull => "TEXT",
            Self::CiText |
            Self::CiTextNotNull => "CITEXT",
            Self::Bytea |
            Self::ByteaNotNull => "BYTEA",
            Self::Interval |
            Self::IntervalNotNull => "INTERVAL",
            Self::Int8Range |
            Self::Int8RangeNotNull => "INT8RANGE",
            Self::Int4Range |
            Self::Int4RangeNotNull => "INT4RANGE",
            Self::TsRange |
            Self::TsRangeNotNull => "TSRANGE",
            Self::TsTzRange |
            Self::TsTzRangeNotNull => "TSTZRANGE",
            Self::DateRange |
            Self::DateRangeNotNull => "DATERANGE",
            Self::NumRange |
            Self::NumRangeNotNull => "NUMRANGE",
            Self::Money |
            Self::MoneyNotNull => "MONEY",
            Self::Numeric |
            Self::NumericNotNull => "NUMERIC",
            Self::TimestampTz |
            Self::TimestampTzNotNull => "TIMESTAMPTZ",
            Self::Date |
            Self::DateNotNull => "DATE",
            Self::Time |
            Self::TimeNotNull => "TIME",
            Self::TimeTz |
            Self::TimeTzNotNull => "TIMETZ",
            Self::Timestamp |
            Self::TimestampNotNull => "TIMESTAMP",
            Self::Uuid |
            Self::UuidNotNull |
            Self::UuidNotNullPrimaryKey => "UUID",
            Self::Inet |
            Self::InetNotNull => "INET",
            Self::Cidr |
            Self::CidrNotNull => "CIDR",
            Self::MacAddr |
            Self::MacAddrNotNull => "MACADDR",
            Self::Bit |
            Self::BitNotNull => "BIT",
            Self::VarBit |
            Self::VarBitNotNull => "VARBIT",
            Self::Json |
            Self::JsonNotNull => "JSON",
            Self::JsonB |
            Self::JsonBNotNull => "JSONB",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::Display,
    strum_macros::EnumIter,
    proc_macro_assistants::ToSnakeCaseStringified,
)]
pub enum SupportedSqlxPostgresType {
    StdPrimitiveBool,
    StdOptionOptionStdPrimitiveBool,
    StdPrimitiveI16,
    StdOptionOptionStdPrimitiveI16,
    StdPrimitiveI32,
    StdOptionOptionStdPrimitiveI32,
    StdPrimitiveI64,
    StdOptionOptionStdPrimitiveI64,
    StdPrimitiveF32,
    StdOptionOptionStdPrimitiveF32,
    StdPrimitiveF64,
    StdOptionOptionStdPrimitiveF64,
    StdStringString,
    StdOptionOptionStdStringString,
    StdVecVecStdPrimitiveU8,
    StdOptionOptionStdVecVecStdPrimitiveU8,
    SqlxPostgresTypesPgInterval,
    StdOptionOptionSqlxPostgresTypesPgInterval,
    SqlxPostgresTypesPgRangeStdPrimitiveI64,
    StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64,
    SqlxPostgresTypesPgRangeStdPrimitiveI32,
    StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal,
    SqlxPostgresTypesPgMoney,
    StdOptionOptionSqlxPostgresTypesPgMoney,
    SqlxPostgresTypesPgCiText,
    StdOptionOptionSqlxPostgresTypesPgCiText,
    SqlxTypesBigDecimal,
    StdOptionOptionSqlxTypesBigDecimal,
    SqlxTypesDecimal,
    StdOptionOptionSqlxTypesDecimal,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxTypesChronoNaiveDateTime,
    StdOptionOptionSqlxTypesChronoNaiveDateTime,
    SqlxTypesChronoNaiveDate,
    StdOptionOptionSqlxTypesChronoNaiveDate,
    SqlxTypesChronoNaiveTime,
    StdOptionOptionSqlxTypesChronoNaiveTime,
    SqlxPostgresTypesPgTimeTz,
    StdOptionOptionSqlxPostgresTypesPgTimeTz,
    SqlxTypesTimePrimitiveDateTime,
    StdOptionOptionSqlxTypesTimePrimitiveDateTime,
    SqlxTypesTimeOffsetDateTime,
    StdOptionOptionSqlxTypesTimeOffsetDateTime,
    SqlxTypesTimeDate,
    StdOptionOptionSqlxTypesTimeDate,
    SqlxTypesTimeTime,
    StdOptionOptionSqlxTypesTimeTime,
    SqlxTypesUuidUuid,
    StdOptionOptionSqlxTypesUuidUuid,
    SqlxTypesIpnetworkIpNetwork,
    StdOptionOptionSqlxTypesIpnetworkIpNetwork,
    StdNetIpAddr,
    StdOptionOptionStdNetIpAddr,
    SqlxTypesMacAddressMacAddress,
    StdOptionOptionSqlxTypesMacAddressMacAddress,
    SqlxTypesBitVec,
    StdOptionOptionSqlxTypesBitVec,
    SqlxTypesJsonT,
    StdOptionOptionSqlxTypesJsonT,
    SerdeJsonValue,
    StdOptionOptionSerdeJsonValue,
}

impl std::convert::From<&SqlxPostgresType> for SupportedSqlxPostgresType {
    fn from(value: &SqlxPostgresType) -> Self {
        match value {
            SqlxPostgresType::StdPrimitiveBool => Self::StdPrimitiveBool,
            SqlxPostgresType::StdPrimitiveI16 => Self::StdPrimitiveI16,
            SqlxPostgresType::StdPrimitiveI32 => Self::StdPrimitiveI32,
            SqlxPostgresType::StdPrimitiveI64 => Self::StdPrimitiveI64,
            SqlxPostgresType::StdPrimitiveF32 => Self::StdPrimitiveF32,
            SqlxPostgresType::StdPrimitiveF64 => Self::StdPrimitiveF64,
            SqlxPostgresType::StdStringString => Self::StdStringString,
            SqlxPostgresType::StdVecVecStdPrimitiveU8 => Self::StdVecVecStdPrimitiveU8,
            SqlxPostgresType::SqlxPostgresTypesPgInterval => Self::SqlxPostgresTypesPgInterval,
            SqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
            SqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
            SqlxPostgresType::SqlxPostgresTypesPgMoney => Self::SqlxPostgresTypesPgMoney,
            SqlxPostgresType::SqlxPostgresTypesPgCiText => Self::SqlxPostgresTypesPgCiText,
            SqlxPostgresType::SqlxTypesBigDecimal => Self::SqlxTypesBigDecimal,
            SqlxPostgresType::SqlxTypesDecimal => Self::SqlxTypesDecimal,
            SqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SqlxPostgresType::SqlxTypesChronoNaiveDateTime => Self::SqlxTypesChronoNaiveDateTime,
            SqlxPostgresType::SqlxTypesChronoNaiveDate => Self::SqlxTypesChronoNaiveDate,
            SqlxPostgresType::SqlxTypesChronoNaiveTime => Self::SqlxTypesChronoNaiveTime,
            SqlxPostgresType::SqlxPostgresTypesPgTimeTz => Self::SqlxPostgresTypesPgTimeTz,
            SqlxPostgresType::SqlxTypesTimePrimitiveDateTime => Self::SqlxTypesTimePrimitiveDateTime,
            SqlxPostgresType::SqlxTypesTimeOffsetDateTime => Self::SqlxTypesTimeOffsetDateTime,
            SqlxPostgresType::SqlxTypesTimeDate => Self::SqlxTypesTimeDate,
            SqlxPostgresType::SqlxTypesTimeTime => Self::SqlxTypesTimeTime,
            SqlxPostgresType::SqlxTypesUuidUuid => Self::SqlxTypesUuidUuid,
            SqlxPostgresType::SqlxTypesIpnetworkIpNetwork => Self::SqlxTypesIpnetworkIpNetwork,
            SqlxPostgresType::StdNetIpAddr => Self::StdNetIpAddr,
            SqlxPostgresType::SqlxTypesMacAddressMacAddress => Self::SqlxTypesMacAddressMacAddress,
            SqlxPostgresType::SqlxTypesBitVec => Self::SqlxTypesBitVec,
            SqlxPostgresType::SqlxTypesJsonT => Self::SqlxTypesJsonT,
            SqlxPostgresType::SerdeJsonValue => Self::SerdeJsonValue,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SqlxPostgresTypeOrOptionSupportedSqlxPostgresType {
    SqlxPostgresType(SqlxPostgresType),
    OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType),
}

impl std::convert::From<&SupportedSqlxPostgresType> for SqlxPostgresTypeOrOptionSupportedSqlxPostgresType {
    fn from(value: &SupportedSqlxPostgresType) -> Self {
        match value {
            SupportedSqlxPostgresType::StdPrimitiveBool => Self::SqlxPostgresType(SqlxPostgresType::StdPrimitiveBool),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveBool => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdPrimitiveBool),
            SupportedSqlxPostgresType::StdPrimitiveI16 => Self::SqlxPostgresType(SqlxPostgresType::StdPrimitiveI16),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI16 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdPrimitiveI16),
            SupportedSqlxPostgresType::StdPrimitiveI32 => Self::SqlxPostgresType(SqlxPostgresType::StdPrimitiveI32),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI32 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdPrimitiveI32),
            SupportedSqlxPostgresType::StdPrimitiveI64 => Self::SqlxPostgresType(SqlxPostgresType::StdPrimitiveI64),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI64 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdPrimitiveI64),
            SupportedSqlxPostgresType::StdPrimitiveF32 => Self::SqlxPostgresType(SqlxPostgresType::StdPrimitiveF32),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF32 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdPrimitiveF32),
            SupportedSqlxPostgresType::StdPrimitiveF64 => Self::SqlxPostgresType(SqlxPostgresType::StdPrimitiveF64),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF64 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdPrimitiveF64),
            SupportedSqlxPostgresType::StdStringString => Self::SqlxPostgresType(SqlxPostgresType::StdStringString),
            SupportedSqlxPostgresType::StdOptionOptionStdStringString => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdStringString),
            SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 => Self::SqlxPostgresType(SqlxPostgresType::StdVecVecStdPrimitiveU8),
            SupportedSqlxPostgresType::StdOptionOptionStdVecVecStdPrimitiveU8 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdVecVecStdPrimitiveU8),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgInterval),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgInterval => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgInterval),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgMoney),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgMoney => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgMoney),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgCiText),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgCiText => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgCiText),
            SupportedSqlxPostgresType::SqlxTypesBigDecimal => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesBigDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBigDecimal => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesBigDecimal),
            SupportedSqlxPostgresType::SqlxTypesDecimal => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesDecimal => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesDecimal),
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDateTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDate => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesChronoNaiveTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz => Self::SqlxPostgresType(SqlxPostgresType::SqlxPostgresTypesPgTimeTz),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgTimeTz => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz),
            SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimePrimitiveDateTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeOffsetDateTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::SqlxTypesTimeDate => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesTimeDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeDate => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesTimeDate),
            SupportedSqlxPostgresType::SqlxTypesTimeTime => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesTimeTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeTime => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesTimeTime),
            SupportedSqlxPostgresType::SqlxTypesUuidUuid => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesUuidUuid),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesUuidUuid => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesUuidUuid),
            SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesIpnetworkIpNetwork),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesIpnetworkIpNetwork => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork),
            SupportedSqlxPostgresType::StdNetIpAddr => Self::SqlxPostgresType(SqlxPostgresType::StdNetIpAddr),
            SupportedSqlxPostgresType::StdOptionOptionStdNetIpAddr => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::StdNetIpAddr),
            SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesMacAddressMacAddress),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesMacAddressMacAddress => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress),
            SupportedSqlxPostgresType::SqlxTypesBitVec => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesBitVec),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBitVec => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesBitVec),
            SupportedSqlxPostgresType::SqlxTypesJsonT => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesJsonT),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJsonT => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesJsonT),
            SupportedSqlxPostgresType::SerdeJsonValue => Self::SqlxPostgresType(SqlxPostgresType::SerdeJsonValue),
            SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SerdeJsonValue),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SqlxPostgresType {
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
    SqlxTypesJsonT,
    SerdeJsonValue,
}

impl SqlxPostgresType {
    pub const fn from_supported_sqlx_postgres_type_removing_option(value: &SupportedSqlxPostgresType) -> Self {
        match value {
            SupportedSqlxPostgresType::StdPrimitiveBool |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveBool => Self::StdPrimitiveBool,
            SupportedSqlxPostgresType::StdPrimitiveI16 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI16 => Self::StdPrimitiveI16,
            SupportedSqlxPostgresType::StdPrimitiveI32 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI32 => Self::StdPrimitiveI32,
            SupportedSqlxPostgresType::StdPrimitiveI64 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI64 => Self::StdPrimitiveI64,
            SupportedSqlxPostgresType::StdPrimitiveF32 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF32 => Self::StdPrimitiveF32,
            SupportedSqlxPostgresType::StdPrimitiveF64 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF64 => Self::StdPrimitiveF64,
            SupportedSqlxPostgresType::StdStringString |
            SupportedSqlxPostgresType::StdOptionOptionStdStringString => Self::StdStringString,
            SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 |
            SupportedSqlxPostgresType::StdOptionOptionStdVecVecStdPrimitiveU8 => Self::StdVecVecStdPrimitiveU8,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgInterval => Self::SqlxPostgresTypesPgInterval,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgMoney => Self::SqlxPostgresTypesPgMoney,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgCiText => Self::SqlxPostgresTypesPgCiText,
            SupportedSqlxPostgresType::SqlxTypesBigDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBigDecimal => Self::SqlxTypesBigDecimal,
            SupportedSqlxPostgresType::SqlxTypesDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesDecimal => Self::SqlxTypesDecimal,
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDateTime => Self::SqlxTypesChronoNaiveDateTime,
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDate => Self::SqlxTypesChronoNaiveDate,
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveTime => Self::SqlxTypesChronoNaiveTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgTimeTz => Self::SqlxPostgresTypesPgTimeTz,
            SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimePrimitiveDateTime => Self::SqlxTypesTimePrimitiveDateTime,
            SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeOffsetDateTime => Self::SqlxTypesTimeOffsetDateTime,
            SupportedSqlxPostgresType::SqlxTypesTimeDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeDate => Self::SqlxTypesTimeDate,
            SupportedSqlxPostgresType::SqlxTypesTimeTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeTime => Self::SqlxTypesTimeTime,
            SupportedSqlxPostgresType::SqlxTypesUuidUuid |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesUuidUuid => Self::SqlxTypesUuidUuid,
            SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesIpnetworkIpNetwork => Self::SqlxTypesIpnetworkIpNetwork,
            SupportedSqlxPostgresType::StdNetIpAddr |
            SupportedSqlxPostgresType::StdOptionOptionStdNetIpAddr => Self::StdNetIpAddr,
            SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesMacAddressMacAddress => Self::SqlxTypesMacAddressMacAddress,
            SupportedSqlxPostgresType::SqlxTypesBitVec |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBitVec => Self::SqlxTypesBitVec,
            SupportedSqlxPostgresType::SqlxTypesJsonT |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJsonT => Self::SqlxTypesJsonT,
            SupportedSqlxPostgresType::SerdeJsonValue |
            SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Self::SerdeJsonValue,
        }
    }
    fn get_type_stringified(self, generic_type_str: &str) -> std::string::String {
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
}

impl std::convert::TryFrom<&SupportedSqlxPostgresType> for SqlxPostgresType {
    type Error = ();
    fn try_from(value: &SupportedSqlxPostgresType) -> Result<Self, Self::Error> {
        match value {
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveBool |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI16 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI32 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI64 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF32 |
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF64 |
            SupportedSqlxPostgresType::StdOptionOptionStdStringString |
            SupportedSqlxPostgresType::StdOptionOptionStdVecVecStdPrimitiveU8 |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgInterval |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgMoney |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgCiText |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBigDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesDecimal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgTimeTz |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimePrimitiveDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeOffsetDateTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeDate |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeTime |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesUuidUuid |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesIpnetworkIpNetwork |
            SupportedSqlxPostgresType::StdOptionOptionStdNetIpAddr |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesMacAddressMacAddress |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBitVec |
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJsonT |
            SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Err(()),

            SupportedSqlxPostgresType::StdPrimitiveBool => Ok(Self::StdPrimitiveBool),
            SupportedSqlxPostgresType::StdPrimitiveI16 => Ok(Self::StdPrimitiveI16),
            SupportedSqlxPostgresType::StdPrimitiveI32 => Ok(Self::StdPrimitiveI32),
            SupportedSqlxPostgresType::StdPrimitiveI64 => Ok(Self::StdPrimitiveI64),
            SupportedSqlxPostgresType::StdPrimitiveF32 => Ok(Self::StdPrimitiveF32),
            SupportedSqlxPostgresType::StdPrimitiveF64 => Ok(Self::StdPrimitiveF64),
            SupportedSqlxPostgresType::StdStringString => Ok(Self::StdStringString),
            SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 => Ok(Self::StdVecVecStdPrimitiveU8),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval => Ok(Self::SqlxPostgresTypesPgInterval),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney => Ok(Self::SqlxPostgresTypesPgMoney),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText => Ok(Self::SqlxPostgresTypesPgCiText),
            SupportedSqlxPostgresType::SqlxTypesBigDecimal => Ok(Self::SqlxTypesBigDecimal),
            SupportedSqlxPostgresType::SqlxTypesDecimal => Ok(Self::SqlxTypesDecimal),
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime => Ok(Self::SqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate => Ok(Self::SqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime => Ok(Self::SqlxTypesChronoNaiveTime),
            SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz => Ok(Self::SqlxPostgresTypesPgTimeTz),
            SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime => Ok(Self::SqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime => Ok(Self::SqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::SqlxTypesTimeDate => Ok(Self::SqlxTypesTimeDate),
            SupportedSqlxPostgresType::SqlxTypesTimeTime => Ok(Self::SqlxTypesTimeTime),
            SupportedSqlxPostgresType::SqlxTypesUuidUuid => Ok(Self::SqlxTypesUuidUuid),
            SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork => Ok(Self::SqlxTypesIpnetworkIpNetwork),
            SupportedSqlxPostgresType::StdNetIpAddr => Ok(Self::StdNetIpAddr),
            SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress => Ok(Self::SqlxTypesMacAddressMacAddress),
            SupportedSqlxPostgresType::SqlxTypesBitVec => Ok(Self::SqlxTypesBitVec),
            SupportedSqlxPostgresType::SqlxTypesJsonT => Ok(Self::SqlxTypesJsonT),
            SupportedSqlxPostgresType::SerdeJsonValue => Ok(Self::SerdeJsonValue),
            
        }
    }
}

impl std::convert::From<&OptionSupportedSqlxPostgresType> for SqlxPostgresType {
    fn from(value: &OptionSupportedSqlxPostgresType) -> Self {
        match value {
            OptionSupportedSqlxPostgresType::StdPrimitiveBool => Self::StdPrimitiveBool,
            OptionSupportedSqlxPostgresType::StdPrimitiveI16 => Self::StdPrimitiveI16,
            OptionSupportedSqlxPostgresType::StdPrimitiveI32 => Self::StdPrimitiveI32,
            OptionSupportedSqlxPostgresType::StdPrimitiveI64 => Self::StdPrimitiveI64,
            OptionSupportedSqlxPostgresType::StdPrimitiveF32 => Self::StdPrimitiveF32,
            OptionSupportedSqlxPostgresType::StdPrimitiveF64 => Self::StdPrimitiveF64,
            OptionSupportedSqlxPostgresType::StdStringString => Self::StdStringString,
            OptionSupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 => Self::StdVecVecStdPrimitiveU8,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgInterval => Self::SqlxPostgresTypesPgInterval,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgMoney => Self::SqlxPostgresTypesPgMoney,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgCiText => Self::SqlxPostgresTypesPgCiText,
            OptionSupportedSqlxPostgresType::SqlxTypesBigDecimal => Self::SqlxTypesBigDecimal,
            OptionSupportedSqlxPostgresType::SqlxTypesDecimal => Self::SqlxTypesDecimal,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime => Self::SqlxTypesChronoNaiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveDate => Self::SqlxTypesChronoNaiveDate,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveTime => Self::SqlxTypesChronoNaiveTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz => Self::SqlxPostgresTypesPgTimeTz,
            OptionSupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime => Self::SqlxTypesTimePrimitiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime => Self::SqlxTypesTimeOffsetDateTime,
            OptionSupportedSqlxPostgresType::SqlxTypesTimeDate => Self::SqlxTypesTimeDate,
            OptionSupportedSqlxPostgresType::SqlxTypesTimeTime => Self::SqlxTypesTimeTime,
            OptionSupportedSqlxPostgresType::SqlxTypesUuidUuid => Self::SqlxTypesUuidUuid,
            OptionSupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork => Self::SqlxTypesIpnetworkIpNetwork,
            OptionSupportedSqlxPostgresType::StdNetIpAddr => Self::StdNetIpAddr,
            OptionSupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress => Self::SqlxTypesMacAddressMacAddress,
            OptionSupportedSqlxPostgresType::SqlxTypesBitVec => Self::SqlxTypesBitVec,
            OptionSupportedSqlxPostgresType::SqlxTypesJsonT => Self::SqlxTypesJsonT,
            OptionSupportedSqlxPostgresType::SerdeJsonValue => Self::SerdeJsonValue,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OptionSupportedSqlxPostgresType {
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
    SqlxTypesJsonT,
    SerdeJsonValue,
}

impl OptionSupportedSqlxPostgresType {
    fn get_type_stringified(self, generic_type_str: &str) -> std::string::String {
        format!("std::option::Option<{}>", SqlxPostgresType::from(&self).get_type_stringified(generic_type_str))
    }
}

impl std::convert::From<&OptionSupportedSqlxPostgresType> for SupportedSqlxPostgresType {
    fn from(value: &OptionSupportedSqlxPostgresType) -> Self {
        match value {
            OptionSupportedSqlxPostgresType::StdPrimitiveBool => Self::StdOptionOptionStdPrimitiveBool,
            OptionSupportedSqlxPostgresType::StdPrimitiveI16 => Self::StdOptionOptionStdPrimitiveI16,
            OptionSupportedSqlxPostgresType::StdPrimitiveI32 => Self::StdOptionOptionStdPrimitiveI32,
            OptionSupportedSqlxPostgresType::StdPrimitiveI64 => Self::StdOptionOptionStdPrimitiveI64,
            OptionSupportedSqlxPostgresType::StdPrimitiveF32 => Self::StdOptionOptionStdPrimitiveF32,
            OptionSupportedSqlxPostgresType::StdPrimitiveF64 => Self::StdOptionOptionStdPrimitiveF64,
            OptionSupportedSqlxPostgresType::StdStringString => Self::StdOptionOptionStdStringString,
            OptionSupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 => Self::StdOptionOptionStdVecVecStdPrimitiveU8,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgInterval => Self::StdOptionOptionSqlxPostgresTypesPgInterval,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgMoney => Self::StdOptionOptionSqlxPostgresTypesPgMoney,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgCiText => Self::StdOptionOptionSqlxPostgresTypesPgCiText,
            OptionSupportedSqlxPostgresType::SqlxTypesBigDecimal => Self::StdOptionOptionSqlxTypesBigDecimal,
            OptionSupportedSqlxPostgresType::SqlxTypesDecimal => Self::StdOptionOptionSqlxTypesDecimal,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime => Self::StdOptionOptionSqlxTypesChronoNaiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveDate => Self::StdOptionOptionSqlxTypesChronoNaiveDate,
            OptionSupportedSqlxPostgresType::SqlxTypesChronoNaiveTime => Self::StdOptionOptionSqlxTypesChronoNaiveTime,
            OptionSupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz => Self::StdOptionOptionSqlxPostgresTypesPgTimeTz,
            OptionSupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime => Self::StdOptionOptionSqlxTypesTimePrimitiveDateTime,
            OptionSupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime => Self::StdOptionOptionSqlxTypesTimeOffsetDateTime,
            OptionSupportedSqlxPostgresType::SqlxTypesTimeDate => Self::StdOptionOptionSqlxTypesTimeDate,
            OptionSupportedSqlxPostgresType::SqlxTypesTimeTime => Self::StdOptionOptionSqlxTypesTimeTime,
            OptionSupportedSqlxPostgresType::SqlxTypesUuidUuid => Self::StdOptionOptionSqlxTypesUuidUuid,
            OptionSupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork => Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork,
            OptionSupportedSqlxPostgresType::StdNetIpAddr => Self::StdOptionOptionStdNetIpAddr,
            OptionSupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress => Self::StdOptionOptionSqlxTypesMacAddressMacAddress,
            OptionSupportedSqlxPostgresType::SqlxTypesBitVec => Self::StdOptionOptionSqlxTypesBitVec,
            OptionSupportedSqlxPostgresType::SqlxTypesJsonT => Self::StdOptionOptionSqlxTypesJsonT,
            OptionSupportedSqlxPostgresType::SerdeJsonValue => Self::StdOptionOptionSerdeJsonValue,
        }
    }
}

impl std::convert::TryFrom<&SupportedSqlxPostgresType> for OptionSupportedSqlxPostgresType {
    type Error = ();
    fn try_from(value: &SupportedSqlxPostgresType) -> Result<Self, Self::Error> {
        match value {
            SupportedSqlxPostgresType::StdPrimitiveBool |
            SupportedSqlxPostgresType::StdPrimitiveI16 |
            SupportedSqlxPostgresType::StdPrimitiveI32 |
            SupportedSqlxPostgresType::StdPrimitiveI64 |
            SupportedSqlxPostgresType::StdPrimitiveF32 |
            SupportedSqlxPostgresType::StdPrimitiveF64 |
            SupportedSqlxPostgresType::StdStringString |
            SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText |
            SupportedSqlxPostgresType::SqlxTypesBigDecimal |
            SupportedSqlxPostgresType::SqlxTypesDecimal |
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime |
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate |
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime |
            SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz |
            SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime |
            SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime |
            SupportedSqlxPostgresType::SqlxTypesTimeDate |
            SupportedSqlxPostgresType::SqlxTypesTimeTime |
            SupportedSqlxPostgresType::SqlxTypesUuidUuid |
            SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork |
            SupportedSqlxPostgresType::StdNetIpAddr |
            SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress |
            SupportedSqlxPostgresType::SqlxTypesBitVec |
            SupportedSqlxPostgresType::SqlxTypesJsonT |
            SupportedSqlxPostgresType::SerdeJsonValue => Err(()),

            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveBool => Ok(Self::StdPrimitiveBool),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI16 => Ok(Self::StdPrimitiveI16),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI32 => Ok(Self::StdPrimitiveI32),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI64 => Ok(Self::StdPrimitiveI64),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF32 => Ok(Self::StdPrimitiveF32),
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF64 => Ok(Self::StdPrimitiveF64),
            SupportedSqlxPostgresType::StdOptionOptionStdStringString => Ok(Self::StdStringString),
            SupportedSqlxPostgresType::StdOptionOptionStdVecVecStdPrimitiveU8 => Ok(Self::StdVecVecStdPrimitiveU8),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgInterval => Ok(Self::SqlxPostgresTypesPgInterval),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgMoney => Ok(Self::SqlxPostgresTypesPgMoney),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgCiText => Ok(Self::SqlxPostgresTypesPgCiText),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBigDecimal => Ok(Self::SqlxTypesBigDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesDecimal => Ok(Self::SqlxTypesDecimal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDateTime => Ok(Self::SqlxTypesChronoNaiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDate => Ok(Self::SqlxTypesChronoNaiveDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveTime => Ok(Self::SqlxTypesChronoNaiveTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgTimeTz => Ok(Self::SqlxPostgresTypesPgTimeTz),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimePrimitiveDateTime => Ok(Self::SqlxTypesTimePrimitiveDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeOffsetDateTime => Ok(Self::SqlxTypesTimeOffsetDateTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeDate => Ok(Self::SqlxTypesTimeDate),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeTime => Ok(Self::SqlxTypesTimeTime),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesUuidUuid => Ok(Self::SqlxTypesUuidUuid),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesIpnetworkIpNetwork => Ok(Self::SqlxTypesIpnetworkIpNetwork),
            SupportedSqlxPostgresType::StdOptionOptionStdNetIpAddr => Ok(Self::StdNetIpAddr),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesMacAddressMacAddress => Ok(Self::SqlxTypesMacAddressMacAddress),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBitVec => Ok(Self::SqlxTypesBitVec),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJsonT => Ok(Self::SqlxTypesJsonT),
            SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Ok(Self::SerdeJsonValue),
        }
    }
}

impl std::convert::From<&SqlxPostgresType> for OptionSupportedSqlxPostgresType {
    fn from(value: &SqlxPostgresType) -> Self {
        match value {
            SqlxPostgresType::StdPrimitiveBool => Self::StdPrimitiveBool,
            SqlxPostgresType::StdPrimitiveI16 => Self::StdPrimitiveI16,
            SqlxPostgresType::StdPrimitiveI32 => Self::StdPrimitiveI32,
            SqlxPostgresType::StdPrimitiveI64 => Self::StdPrimitiveI64,
            SqlxPostgresType::StdPrimitiveF32 => Self::StdPrimitiveF32,
            SqlxPostgresType::StdPrimitiveF64 => Self::StdPrimitiveF64,
            SqlxPostgresType::StdStringString => Self::StdStringString,
            SqlxPostgresType::StdVecVecStdPrimitiveU8 => Self::StdVecVecStdPrimitiveU8,
            SqlxPostgresType::SqlxPostgresTypesPgInterval => Self::SqlxPostgresTypesPgInterval,
            SqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
            SqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            SqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
            SqlxPostgresType::SqlxPostgresTypesPgMoney => Self::SqlxPostgresTypesPgMoney,
            SqlxPostgresType::SqlxPostgresTypesPgCiText => Self::SqlxPostgresTypesPgCiText,
            SqlxPostgresType::SqlxTypesBigDecimal => Self::SqlxTypesBigDecimal,
            SqlxPostgresType::SqlxTypesDecimal => Self::SqlxTypesDecimal,
            SqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SqlxPostgresType::SqlxTypesChronoNaiveDateTime => Self::SqlxTypesChronoNaiveDateTime,
            SqlxPostgresType::SqlxTypesChronoNaiveDate => Self::SqlxTypesChronoNaiveDate,
            SqlxPostgresType::SqlxTypesChronoNaiveTime => Self::SqlxTypesChronoNaiveTime,
            SqlxPostgresType::SqlxPostgresTypesPgTimeTz => Self::SqlxPostgresTypesPgTimeTz,
            SqlxPostgresType::SqlxTypesTimePrimitiveDateTime => Self::SqlxTypesTimePrimitiveDateTime,
            SqlxPostgresType::SqlxTypesTimeOffsetDateTime => Self::SqlxTypesTimeOffsetDateTime,
            SqlxPostgresType::SqlxTypesTimeDate => Self::SqlxTypesTimeDate,
            SqlxPostgresType::SqlxTypesTimeTime => Self::SqlxTypesTimeTime,
            SqlxPostgresType::SqlxTypesUuidUuid => Self::SqlxTypesUuidUuid,
            SqlxPostgresType::SqlxTypesIpnetworkIpNetwork => Self::SqlxTypesIpnetworkIpNetwork,
            SqlxPostgresType::StdNetIpAddr => Self::StdNetIpAddr,
            SqlxPostgresType::SqlxTypesMacAddressMacAddress => Self::SqlxTypesMacAddressMacAddress,
            SqlxPostgresType::SqlxTypesBitVec => Self::SqlxTypesBitVec,
            SqlxPostgresType::SqlxTypesJsonT => Self::SqlxTypesJsonT,
            SqlxPostgresType::SerdeJsonValue => Self::SerdeJsonValue,
        }
    }
}

impl SupportedSqlxPostgresType {
    fn get_original_type_stringified(self, generic_type_str: &str) -> std::string::String {
        match SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::from(&self) {
            SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::SqlxPostgresType(value) => value.get_type_stringified(generic_type_str),
            SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::OptionSupportedSqlxPostgresType(value) => value.get_type_stringified(generic_type_str),
        }
    }
    fn get_inner_type_handle_stringified(self, generic_type_str: &str) -> std::string::String {
        match self {
            Self::SqlxTypesJsonT => format!("{self}<{generic_type_str}>"),
            Self::StdPrimitiveBool | 
            Self::StdOptionOptionStdPrimitiveBool | 
            Self::StdPrimitiveI16 |
            Self::StdOptionOptionStdPrimitiveI16 |
            Self::StdPrimitiveI32 |
            Self::StdOptionOptionStdPrimitiveI32 |
            Self::StdPrimitiveI64 |
            Self::StdOptionOptionStdPrimitiveI64 |
            Self::StdPrimitiveF32 |
            Self::StdOptionOptionStdPrimitiveF32 |
            Self::StdPrimitiveF64 |
            Self::StdOptionOptionStdPrimitiveF64 |
            Self::StdStringString |
            Self::StdOptionOptionStdStringString |
            Self::StdVecVecStdPrimitiveU8 |
            Self::StdOptionOptionStdVecVecStdPrimitiveU8 |
            Self::SqlxPostgresTypesPgInterval |
            Self::StdOptionOptionSqlxPostgresTypesPgInterval |
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64 |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 |
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32 |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal |
            Self::SqlxPostgresTypesPgMoney |
            Self::StdOptionOptionSqlxPostgresTypesPgMoney |
            Self::SqlxPostgresTypesPgCiText |
            Self::StdOptionOptionSqlxPostgresTypesPgCiText |
            Self::SqlxTypesBigDecimal |
            Self::StdOptionOptionSqlxTypesBigDecimal |
            Self::SqlxTypesDecimal |
            Self::StdOptionOptionSqlxTypesDecimal |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxTypesChronoNaiveDateTime |
            Self::StdOptionOptionSqlxTypesChronoNaiveDateTime |
            Self::SqlxTypesChronoNaiveDate |
            Self::StdOptionOptionSqlxTypesChronoNaiveDate |
            Self::SqlxTypesChronoNaiveTime |
            Self::StdOptionOptionSqlxTypesChronoNaiveTime |
            Self::SqlxPostgresTypesPgTimeTz |
            Self::StdOptionOptionSqlxPostgresTypesPgTimeTz |
            Self::SqlxTypesTimePrimitiveDateTime |
            Self::StdOptionOptionSqlxTypesTimePrimitiveDateTime |
            Self::SqlxTypesTimeOffsetDateTime |
            Self::StdOptionOptionSqlxTypesTimeOffsetDateTime |
            Self::SqlxTypesTimeDate |
            Self::StdOptionOptionSqlxTypesTimeDate |
            Self::SqlxTypesTimeTime |
            Self::StdOptionOptionSqlxTypesTimeTime |
            Self::SqlxTypesUuidUuid |
            Self::StdOptionOptionSqlxTypesUuidUuid |
            Self::SqlxTypesIpnetworkIpNetwork |
            Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork |
            Self::StdNetIpAddr |
            Self::StdOptionOptionStdNetIpAddr |
            Self::SqlxTypesMacAddressMacAddress |
            Self::StdOptionOptionSqlxTypesMacAddressMacAddress |
            Self::SqlxTypesBitVec |
            Self::StdOptionOptionSqlxTypesBitVec |
            Self::StdOptionOptionSqlxTypesJsonT |
            Self::SerdeJsonValue |
            Self::StdOptionOptionSerdeJsonValue => self.to_string()
        }
    }
    pub fn get_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_handle_stringified(
        self,
        generic_type_str: &str,
    ) -> std::string::String {
        if matches!(self, Self::SqlxTypesJsonT) { format!(
            "sqlx::types::Json{}<{generic_type_str}>",
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        ) } else { 
            format!("{self}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()) 
        }
    }
    pub fn get_inner_type_with_serialize_deserialize_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_error_named_handle_stringified(self, generic_type_str: &str) -> std::string::String {
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
    fn get_where_with_serialize_deserialize_error_named_stringified(self, generic_type_str: &str) -> std::string::String {
        add_path(&match self.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            FromOrTryFrom::From => std::string::String::from(""),
            FromOrTryFrom::TryFrom => format!(
                "{}{}{}{}",
                <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
                self.get_inner_type_handle_stringified(generic_type_str),
                proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
                proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified()
            )
        })
    }
    const fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(self) -> FromOrTryFrom {
        match self {
            Self::StdPrimitiveBool |
            Self::StdOptionOptionStdPrimitiveBool |
            Self::StdPrimitiveI16 |
            Self::StdOptionOptionStdPrimitiveI16 |
            Self::StdPrimitiveI32 |
            Self::StdOptionOptionStdPrimitiveI32 |
            Self::StdPrimitiveI64 |
            Self::StdOptionOptionStdPrimitiveI64 |
            Self::StdPrimitiveF32 |
            Self::StdOptionOptionStdPrimitiveF32 |
            Self::StdPrimitiveF64 |
            Self::StdOptionOptionStdPrimitiveF64 |
            Self::StdStringString |
            Self::StdOptionOptionStdStringString |
            Self::StdVecVecStdPrimitiveU8 |
            Self::StdOptionOptionStdVecVecStdPrimitiveU8 |
            Self::SqlxPostgresTypesPgInterval |
            Self::StdOptionOptionSqlxPostgresTypesPgInterval |
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64 |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 |
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32 |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal |
            Self::SqlxPostgresTypesPgMoney |
            Self::StdOptionOptionSqlxPostgresTypesPgMoney |
            Self::SqlxPostgresTypesPgCiText |
            Self::StdOptionOptionSqlxPostgresTypesPgCiText |
            Self::SqlxTypesBigDecimal |
            Self::StdOptionOptionSqlxTypesBigDecimal |
            Self::SqlxTypesDecimal |
            Self::StdOptionOptionSqlxTypesDecimal |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxTypesChronoNaiveDateTime |
            Self::StdOptionOptionSqlxTypesChronoNaiveDateTime |
            Self::SqlxTypesChronoNaiveDate |
            Self::StdOptionOptionSqlxTypesChronoNaiveDate |
            Self::SqlxTypesChronoNaiveTime |
            Self::StdOptionOptionSqlxTypesChronoNaiveTime |
            Self::SqlxTypesIpnetworkIpNetwork |
            Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork |
            Self::StdNetIpAddr |
            Self::StdOptionOptionStdNetIpAddr |
            Self::SqlxTypesMacAddressMacAddress |
            Self::StdOptionOptionSqlxTypesMacAddressMacAddress |
            Self::SqlxTypesBitVec |
            Self::StdOptionOptionSqlxTypesBitVec |
            Self::SqlxTypesJsonT |
            Self::StdOptionOptionSqlxTypesJsonT |
            Self::SerdeJsonValue |
            Self::StdOptionOptionSerdeJsonValue => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::SqlxPostgresTypesPgTimeTz |
            Self::StdOptionOptionSqlxPostgresTypesPgTimeTz |
            Self::SqlxTypesTimePrimitiveDateTime |
            Self::StdOptionOptionSqlxTypesTimePrimitiveDateTime |
            Self::SqlxTypesTimeOffsetDateTime |
            Self::StdOptionOptionSqlxTypesTimeOffsetDateTime |
            Self::SqlxTypesTimeDate |
            Self::StdOptionOptionSqlxTypesTimeDate |
            Self::SqlxTypesTimeTime |
            Self::StdOptionOptionSqlxTypesTimeTime |
            Self::SqlxTypesUuidUuid |
            Self::StdOptionOptionSqlxTypesUuidUuid => FromOrTryFrom::TryFrom,
            
        }
    }
}

impl std::convert::From<&RustSqlxMapToPostgresTypeVariant> for SupportedSqlxPostgresType {
    fn from(value: &RustSqlxMapToPostgresTypeVariant) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Self::StdOptionOptionStdPrimitiveBool,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => Self::StdPrimitiveBool,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Self::StdOptionOptionStdPrimitiveI16,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => Self::StdPrimitiveI16,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Self::StdOptionOptionStdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => Self::StdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Self::StdOptionOptionStdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => Self::StdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Self::StdOptionOptionStdPrimitiveF32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => Self::StdPrimitiveF32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Self::StdOptionOptionStdPrimitiveF64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => Self::StdPrimitiveF64,

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Self::StdOptionOptionStdStringString,

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => Self::StdStringString,

            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Self::StdOptionOptionStdVecVecStdPrimitiveU8,
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => Self::StdVecVecStdPrimitiveU8,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Self::StdOptionOptionSqlxPostgresTypesPgInterval,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => Self::SqlxPostgresTypesPgInterval,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Self::StdOptionOptionSqlxPostgresTypesPgMoney,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => Self::SqlxPostgresTypesPgMoney,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Self::StdOptionOptionSqlxPostgresTypesPgCiText,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => Self::SqlxPostgresTypesPgCiText,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::StdOptionOptionSqlxTypesBigDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => Self::SqlxTypesBigDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Self::StdOptionOptionSqlxTypesDecimal,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull => Self::SqlxTypesDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::StdOptionOptionSqlxTypesChronoNaiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => Self::SqlxTypesChronoNaiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::StdOptionOptionSqlxTypesChronoNaiveDate,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => Self::SqlxTypesChronoNaiveDate,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Self::StdOptionOptionSqlxTypesChronoNaiveTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => Self::SqlxTypesChronoNaiveTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Self::StdOptionOptionSqlxPostgresTypesPgTimeTz,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => Self::SqlxPostgresTypesPgTimeTz,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::StdOptionOptionSqlxTypesTimePrimitiveDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => Self::SqlxTypesTimePrimitiveDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Self::StdOptionOptionSqlxTypesTimeOffsetDateTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => Self::SqlxTypesTimeOffsetDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Self::StdOptionOptionSqlxTypesTimeDate,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull => Self::SqlxTypesTimeDate,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Self::StdOptionOptionSqlxTypesTimeTime,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => Self::SqlxTypesTimeTime,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Self::StdOptionOptionSqlxTypesUuidUuid,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Self::SqlxTypesUuidUuid,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => Self::SqlxTypesIpnetworkIpNetwork,

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Self::StdOptionOptionStdNetIpAddr,

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => Self::StdNetIpAddr,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::StdOptionOptionSqlxTypesMacAddressMacAddress,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => Self::SqlxTypesMacAddressMacAddress,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Self::StdOptionOptionSqlxTypesBitVec,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => Self::SqlxTypesBitVec,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Self::StdOptionOptionSqlxTypesJsonT,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull => Self::SqlxTypesJsonT,

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson |
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::StdOptionOptionSerdeJsonValue,

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull |
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
    Copy,
    strum_macros::Display,
    strum_macros::EnumIter,
    enum_extension_lib::EnumExtension,
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

#[derive(Debug, Clone, Copy)]
pub enum RustSqlxMapToPostgresTypeVariantNullable {
    StdPrimitiveBoolAsPostgresqlBool,

    StdPrimitiveI16AsPostgresqlSmallInt,
    StdPrimitiveI16AsPostgresqlSmallSerial,
    StdPrimitiveI16AsPostgresqlInt2,

    StdPrimitiveI32AsPostgresqlInt,
    StdPrimitiveI32AsPostgresqlSerial,
    StdPrimitiveI32AsPostgresqlInt4,

    StdPrimitiveI64AsPostgresqlBigInt,
    StdPrimitiveI64AsPostgresqlBigSerial,
    StdPrimitiveI64AsPostgresqlInt8,

    StdPrimitiveF32AsPostgresqlReal,
    StdPrimitiveF32AsPostgresqlFloat4,

    StdPrimitiveF64AsPostgresqlDoublePrecision,
    StdPrimitiveF64AsPostgresqlFloat8,

    StdStringStringAsPostgresqlVarchar,
    StdStringStringAsPostgresqlCharN,
    StdStringStringAsPostgresqlText,
    StdStringStringAsPostgresqlCiText,

    StdVecVecStdPrimitiveU8AsPostgresqlBytea,

    SqlxPostgresTypesPgIntervalAsPostgresqlInterval,

    SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,

    SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,

    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,

    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,

    SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,

    SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,

    SqlxPostgresTypesPgMoneyAsPostgresqlMoney,

    SqlxPostgresTypesPgCiTextAsPostgresqlCiText,

    SqlxTypesBigDecimalAsPostgresqlNumeric,

    SqlxTypesDecimalAsPostgresqlNumeric,

    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,

    SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,

    SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,

    SqlxTypesChronoNaiveDateAsPostgresqlDate,

    SqlxTypesChronoNaiveTimeAsPostgresqlTime,

    SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,

    SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,

    SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,

    SqlxTypesTimeDateAsPostgresqlDate,

    SqlxTypesTimeTimeAsPostgresqlTime,

    SqlxTypesUuidUuidAsPostgresqlUuid,

    SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,

    StdNetIpAddrAsPostgresqlInet,
    StdNetIpAddrAsPostgresqlCidr,

    SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,

    SqlxTypesBitVecAsPostgresqlBit,
    SqlxTypesBitVecAsPostgresqlVarBit,

    //todo what to do with generic?
    SqlxTypesJsonTAsPostgresqlJson,
    SqlxTypesJsonTAsPostgresqlJsonB,

    SerdeJsonValueAsPostgresqlJson,
    SerdeJsonValueAsPostgresqlJsonB,
}

impl std::convert::From<&RustSqlxMapToPostgresTypeVariantNullable> for RustSqlxMapToPostgresTypeVariant {
    fn from(value: &RustSqlxMapToPostgresTypeVariantNullable) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveBoolAsPostgresqlBool => Self::StdPrimitiveBoolAsPostgresqlBool,

            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI16AsPostgresqlSmallInt => Self::StdPrimitiveI16AsPostgresqlSmallInt,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI16AsPostgresqlSmallSerial => Self::StdPrimitiveI16AsPostgresqlSmallSerial,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI16AsPostgresqlInt2 => Self::StdPrimitiveI16AsPostgresqlInt2,

            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI32AsPostgresqlInt => Self::StdPrimitiveI32AsPostgresqlInt,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI32AsPostgresqlSerial => Self::StdPrimitiveI32AsPostgresqlSerial,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI32AsPostgresqlInt4 => Self::StdPrimitiveI32AsPostgresqlInt4,

            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI64AsPostgresqlBigInt => Self::StdPrimitiveI64AsPostgresqlBigInt,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI64AsPostgresqlBigSerial => Self::StdPrimitiveI64AsPostgresqlBigSerial,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveI64AsPostgresqlInt8 => Self::StdPrimitiveI64AsPostgresqlInt8,

            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveF32AsPostgresqlReal => Self::StdPrimitiveF32AsPostgresqlReal,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveF32AsPostgresqlFloat4 => Self::StdPrimitiveF32AsPostgresqlFloat4,

            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveF64AsPostgresqlDoublePrecision => Self::StdPrimitiveF64AsPostgresqlDoublePrecision,
            RustSqlxMapToPostgresTypeVariantNullable::StdPrimitiveF64AsPostgresqlFloat8 => Self::StdPrimitiveF64AsPostgresqlFloat8,

            RustSqlxMapToPostgresTypeVariantNullable::StdStringStringAsPostgresqlVarchar => Self::StdStringStringAsPostgresqlVarchar,
            RustSqlxMapToPostgresTypeVariantNullable::StdStringStringAsPostgresqlCharN => Self::StdStringStringAsPostgresqlCharN,
            RustSqlxMapToPostgresTypeVariantNullable::StdStringStringAsPostgresqlText => Self::StdStringStringAsPostgresqlText,
            RustSqlxMapToPostgresTypeVariantNullable::StdStringStringAsPostgresqlCiText => Self::StdStringStringAsPostgresqlCiText,

            RustSqlxMapToPostgresTypeVariantNullable::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::SqlxTypesBigDecimalAsPostgresqlNumeric,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesDecimalAsPostgresqlNumeric => Self::SqlxTypesDecimalAsPostgresqlNumeric,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::SqlxTypesChronoNaiveDateAsPostgresqlDate,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesTimeDateAsPostgresqlDate => Self::SqlxTypesTimeDateAsPostgresqlDate,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesTimeTimeAsPostgresqlTime => Self::SqlxTypesTimeTimeAsPostgresqlTime,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesUuidUuidAsPostgresqlUuid => Self::SqlxTypesUuidUuidAsPostgresqlUuid,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,

            RustSqlxMapToPostgresTypeVariantNullable::StdNetIpAddrAsPostgresqlInet => Self::StdNetIpAddrAsPostgresqlInet,
            RustSqlxMapToPostgresTypeVariantNullable::StdNetIpAddrAsPostgresqlCidr => Self::StdNetIpAddrAsPostgresqlCidr,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesBitVecAsPostgresqlBit => Self::SqlxTypesBitVecAsPostgresqlBit,
            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesBitVecAsPostgresqlVarBit => Self::SqlxTypesBitVecAsPostgresqlVarBit,

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesJsonTAsPostgresqlJson => Self::SqlxTypesJsonTAsPostgresqlJson,
            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesJsonTAsPostgresqlJsonB => Self::SqlxTypesJsonTAsPostgresqlJsonB,

            RustSqlxMapToPostgresTypeVariantNullable::SerdeJsonValueAsPostgresqlJson => Self::SerdeJsonValueAsPostgresqlJson,
            RustSqlxMapToPostgresTypeVariantNullable::SerdeJsonValueAsPostgresqlJsonB => Self::SerdeJsonValueAsPostgresqlJsonB,
        }
    }
}

impl std::convert::TryFrom<&RustSqlxMapToPostgresTypeVariant> for RustSqlxMapToPostgresTypeVariantNullable {
    type Error = ();
    fn try_from(value: &RustSqlxMapToPostgresTypeVariant) -> Result<Self, Self::Error> {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull |
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull |
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull |
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Err(()),
            //
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Ok(Self::StdPrimitiveBoolAsPostgresqlBool),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => Ok(Self::StdPrimitiveI16AsPostgresqlSmallInt),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerial),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Ok(Self::StdPrimitiveI16AsPostgresqlInt2),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => Ok(Self::StdPrimitiveI32AsPostgresqlInt),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => Ok(Self::StdPrimitiveI32AsPostgresqlSerial),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Ok(Self::StdPrimitiveI32AsPostgresqlInt4),
            

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => Ok(Self::StdPrimitiveI64AsPostgresqlBigInt),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerial),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Ok(Self::StdPrimitiveI64AsPostgresqlInt8),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => Ok(Self::StdPrimitiveF32AsPostgresqlReal),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecision),
            
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8),

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => Ok(Self::StdStringStringAsPostgresqlVarchar),
            
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => Ok(Self::StdStringStringAsPostgresqlCharN),
            
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => Ok(Self::StdStringStringAsPostgresqlText),
            
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Ok(Self::StdStringStringAsPostgresqlCiText),

            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumeric),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Ok(Self::SqlxTypesDecimalAsPostgresqlNumeric),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDate),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime),

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Ok(Self::SqlxTypesTimeDateAsPostgresqlDate),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTime),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuid),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet),
            
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr),

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => Ok(Self::StdNetIpAddrAsPostgresqlInet),
            
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Ok(Self::StdNetIpAddrAsPostgresqlCidr),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => Ok(Self::SqlxTypesBitVecAsPostgresqlBit),
            
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBit),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => Ok(Self::SqlxTypesJsonTAsPostgresqlJson),
            
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonB),

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Ok(Self::SerdeJsonValueAsPostgresqlJson),
            
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Ok(Self::SerdeJsonValueAsPostgresqlJsonB),
            
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RustSqlxMapToPostgresTypeVariantPrimaryKey {
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,
}

impl std::convert::TryFrom<&RustSqlxMapToPostgresTypeVariant> for RustSqlxMapToPostgresTypeVariantPrimaryKey {
    type Error = ();
    fn try_from(value: &RustSqlxMapToPostgresTypeVariant) -> Result<Self, Self::Error> {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull |

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull |

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull |

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull |

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull |

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 |
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull |

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText |
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull |

            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea |
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz |
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull |

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr |
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull |

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB |
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull |

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson |
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull |
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB |
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Err(()),

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey),

            
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn get_inner_type_handle_stringified(self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(&self).get_inner_type_handle_stringified(generic_type_str)
    }
    pub fn get_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_handle_stringified(
        self,
        generic_type_str: &str,
    ) -> std::string::String {
        SupportedSqlxPostgresType::from(&self).get_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str)
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
    pub fn get_inner_type_with_serialize_deserialize_error_named_without_option_stringified(
        &self,
        generic_type_str: &str,
    ) -> std::string::String {
        add_path(&SupportedSqlxPostgresType::from(
            &SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                &SupportedSqlxPostgresType::from(self)
            )
        ).get_inner_type_with_serialize_deserialize_error_named_handle_stringified(generic_type_str))
    }
    pub fn get_where_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&format!(
            "{}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(), 
            self.get_inner_type_handle_stringified(generic_type_str)
        ))
    }
    pub fn get_where_inner_type_with_serialize_deserialize_handle_stringified(&self, generic_type_str: &str) -> std::string::String {
        format!(
            "{}{}",
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            self.get_inner_type_with_serialize_deserialize_handle_stringified(
                generic_type_str,
            )
        )
    }
    pub fn get_where_inner_type_with_serialize_deserialize_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_where_inner_type_with_serialize_deserialize_handle_stringified(generic_type_str))
    }
    pub fn get_where_with_serialize_deserialize_error_named_stringified(&self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_where_with_serialize_deserialize_error_named_stringified(generic_type_str)
    }
    pub fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(&self) -> FromOrTryFrom {
        SupportedSqlxPostgresType::from(self).inner_type_from_or_try_from_inner_type_with_serialize_deserialize()
    }
}

impl std::str::FromStr for RustSqlxMapToPostgresTypeVariant {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
//todo rename conversion method
// impl std::convert::TryFrom<&str> for RustSqlxMapToPostgresTypeVariant {
//     type Error = std::string::String;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "StdPrimitiveBoolAsPostgresqlBool" => Ok(Self::StdPrimitiveBoolAsPostgresqlBool),
//             "StdPrimitiveBoolAsPostgresqlBoolNotNull" => Ok(Self::StdPrimitiveBoolAsPostgresqlBoolNotNull),

//             "StdPrimitiveI16AsPostgresqlSmallInt" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallInt),
//             "StdPrimitiveI16AsPostgresqlSmallIntNotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull),
//             "StdPrimitiveI16AsPostgresqlSmallSerial" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerial),
//             "StdPrimitiveI16AsPostgresqlSmallSerialNotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull),
//             "StdPrimitiveI16AsPostgresqlInt2" => Ok(Self::StdPrimitiveI16AsPostgresqlInt2),
//             "StdPrimitiveI16AsPostgresqlInt2NotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlInt2NotNull),

//             "StdPrimitiveI32AsPostgresqlInt" => Ok(Self::StdPrimitiveI32AsPostgresqlInt),
//             "StdPrimitiveI32AsPostgresqlIntNotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlIntNotNull),
//             "StdPrimitiveI32AsPostgresqlSerial" => Ok(Self::StdPrimitiveI32AsPostgresqlSerial),
//             "StdPrimitiveI32AsPostgresqlSerialNotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlSerialNotNull),
//             "StdPrimitiveI32AsPostgresqlInt4" => Ok(Self::StdPrimitiveI32AsPostgresqlInt4),
//             "StdPrimitiveI32AsPostgresqlInt4NotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlInt4NotNull),

//             "StdPrimitiveI64AsPostgresqlBigInt" => Ok(Self::StdPrimitiveI64AsPostgresqlBigInt),
//             "StdPrimitiveI64AsPostgresqlBigIntNotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlBigIntNotNull),
//             "StdPrimitiveI64AsPostgresqlBigSerial" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerial),
//             "StdPrimitiveI64AsPostgresqlBigSerialNotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull),
//             "StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveI64AsPostgresqlInt8" => Ok(Self::StdPrimitiveI64AsPostgresqlInt8),
//             "StdPrimitiveI64AsPostgresqlInt8NotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlInt8NotNull),

//             "StdPrimitiveF32AsPostgresqlReal" => Ok(Self::StdPrimitiveF32AsPostgresqlReal),
//             "StdPrimitiveF32AsPostgresqlRealNotNull" => Ok(Self::StdPrimitiveF32AsPostgresqlRealNotNull),
//             "StdPrimitiveF32AsPostgresqlFloat4" => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4),
//             "StdPrimitiveF32AsPostgresqlFloat4NotNull" => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4NotNull),

//             "StdPrimitiveF64AsPostgresqlDoublePrecision" => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecision),
//             "StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull" => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull),
//             "StdPrimitiveF64AsPostgresqlFloat8" => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8),
//             "StdPrimitiveF64AsPostgresqlFloat8NotNull" => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8NotNull),

//             "StdStringStringAsPostgresqlVarchar" => Ok(Self::StdStringStringAsPostgresqlVarchar),
//             "StdStringStringAsPostgresqlVarcharNotNull" => Ok(Self::StdStringStringAsPostgresqlVarcharNotNull),
//             "StdStringStringAsPostgresqlCharN" => Ok(Self::StdStringStringAsPostgresqlCharN),
//             "StdStringStringAsPostgresqlCharNNotNull" => Ok(Self::StdStringStringAsPostgresqlCharNNotNull),
//             "StdStringStringAsPostgresqlText" => Ok(Self::StdStringStringAsPostgresqlText),
//             "StdStringStringAsPostgresqlTextNotNull" => Ok(Self::StdStringStringAsPostgresqlTextNotNull),
//             "StdStringStringAsPostgresqlCiText" => Ok(Self::StdStringStringAsPostgresqlCiText),
//             "StdStringStringAsPostgresqlCiTextNotNull" => Ok(Self::StdStringStringAsPostgresqlCiTextNotNull),

//             "StdVecVecStdPrimitiveU8AsPostgresqlBytea" => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea),
//             "StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull" => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull),

//             "SqlxPostgresTypesPgIntervalAsPostgresqlInterval" => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval),
//             "SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull" => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull),

//             "SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range),
//             "SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull),

//             "SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range),
//             "SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull),

//             "SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange),
//             "SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull),

//             "SqlxPostgresTypesPgMoneyAsPostgresqlMoney" => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney),
//             "SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull" => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull),

//             "SqlxPostgresTypesPgCiTextAsPostgresqlCiText" => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText),
//             "SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull" => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull),

//             "SqlxTypesBigDecimalAsPostgresqlNumeric" => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumeric),
//             "SqlxTypesBigDecimalAsPostgresqlNumericNotNull" => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull),

//             "SqlxTypesDecimalAsPostgresqlNumeric" => Ok(Self::SqlxTypesDecimalAsPostgresqlNumeric),
//             "SqlxTypesDecimalAsPostgresqlNumericNotNull" => Ok(Self::SqlxTypesDecimalAsPostgresqlNumericNotNull),

//             "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz),
//             "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull),

//             "SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz),
//             "SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull),

//             "SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp" => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),
//             "SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull),

//             "SqlxTypesChronoNaiveDateAsPostgresqlDate" => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDate),
//             "SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull" => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull),

//             "SqlxTypesChronoNaiveTimeAsPostgresqlTime" => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime),
//             "SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull" => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull),

//             "SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz" => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz),
//             "SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull" => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull),

//             "SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp" => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),
//             "SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull),

//             "SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz),
//             "SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull),

//             "SqlxTypesTimeDateAsPostgresqlDate" => Ok(Self::SqlxTypesTimeDateAsPostgresqlDate),
//             "SqlxTypesTimeDateAsPostgresqlDateNotNull" => Ok(Self::SqlxTypesTimeDateAsPostgresqlDateNotNull),

//             "SqlxTypesTimeTimeAsPostgresqlTime" => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTime),
//             "SqlxTypesTimeTimeAsPostgresqlTimeNotNull" => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull),

//             "SqlxTypesUuidUuidAsPostgresqlUuid" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuid),
//             "SqlxTypesUuidUuidAsPostgresqlUuidNotNull" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull),
//             "SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey),

//             "SqlxTypesIpnetworkIpNetworkAsPostgresqlInet" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet),
//             "SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull),
//             "SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr),
//             "SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull),

//             "StdNetIpAddrAsPostgresqlInet" => Ok(Self::StdNetIpAddrAsPostgresqlInet),
//             "StdNetIpAddrAsPostgresqlInetNotNull" => Ok(Self::StdNetIpAddrAsPostgresqlInetNotNull),
//             "StdNetIpAddrAsPostgresqlCidr" => Ok(Self::StdNetIpAddrAsPostgresqlCidr),
//             "StdNetIpAddrAsPostgresqlCidrNotNull" => Ok(Self::StdNetIpAddrAsPostgresqlCidrNotNull),

//             "SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr" => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr),
//             "SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull" => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull),

//             "SqlxTypesBitVecAsPostgresqlBit" => Ok(Self::SqlxTypesBitVecAsPostgresqlBit),
//             "SqlxTypesBitVecAsPostgresqlBitNotNull" => Ok(Self::SqlxTypesBitVecAsPostgresqlBitNotNull),
//             "SqlxTypesBitVecAsPostgresqlVarBit" => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBit),
//             "SqlxTypesBitVecAsPostgresqlVarBitNotNull" => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull),

//             //todo what to do with generic?
//             "SqlxTypesJsonTAsPostgresqlJson" => Ok(Self::SqlxTypesJsonTAsPostgresqlJson),
//             "SqlxTypesJsonTAsPostgresqlJsonNotNull" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonNotNull),
//             "SqlxTypesJsonTAsPostgresqlJsonB" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonB),
//             "SqlxTypesJsonTAsPostgresqlJsonBNotNull" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonBNotNull),

//             "SerdeJsonValueAsPostgresqlJson" => Ok(Self::SerdeJsonValueAsPostgresqlJson),
//             "SerdeJsonValueAsPostgresqlJsonNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonNotNull),
//             "SerdeJsonValueAsPostgresqlJsonB" => Ok(Self::SerdeJsonValueAsPostgresqlJsonB),
//             "SerdeJsonValueAsPostgresqlJsonBNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonBNotNull),
//             _ => Err(format!(
//                 "unsupported value: {value}, {:?}",
//                 Self::into_array().into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<std::string::String>>()
//             ))
//         }
//     }
// }

pub trait CheckSupportedRustAndPostgresqlColumnType {
    fn check_supported_rust_and_postgresql_column_type();
}

//todo maybe inner value must be pub
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveBoolAsPostgresqlBool(pub StdOptionOptionStdPrimitiveBool);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(pub StdPrimitiveBool);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallInt(pub StdOptionOptionStdPrimitiveI16);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallIntNotNull(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerial(pub StdOptionOptionStdPrimitiveI16);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlInt2(pub StdOptionOptionStdPrimitiveI16);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(pub StdPrimitiveI16);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlInt(pub StdOptionOptionStdPrimitiveI32);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlIntNotNull(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlSerial(pub StdOptionOptionStdPrimitiveI32);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlInt4(pub StdOptionOptionStdPrimitiveI32);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(pub StdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigInt(pub StdOptionOptionStdPrimitiveI64);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigIntNotNull(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigSerial(pub StdOptionOptionStdPrimitiveI64);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(pub StdPrimitiveI64);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlInt8(pub StdOptionOptionStdPrimitiveI64);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(pub StdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlReal(pub StdOptionOptionStdPrimitiveF32);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlRealNotNull(pub StdPrimitiveF32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlFloat4(pub StdOptionOptionStdPrimitiveF32);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(pub StdPrimitiveF32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlDoublePrecision(pub StdOptionOptionStdPrimitiveF64);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull(pub StdPrimitiveF64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlFloat8(pub StdOptionOptionStdPrimitiveF64);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(pub StdPrimitiveF64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlVarchar(pub StdOptionOptionStdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlVarcharNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCharN(pub StdOptionOptionStdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCharNNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlText(pub StdOptionOptionStdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlTextNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCiText(pub StdOptionOptionStdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdStringStringAsPostgresqlCiTextNotNull(pub StdStringString);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlBytea(pub StdOptionOptionStdVecVecStdPrimitiveU8);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull(pub StdVecVecStdPrimitiveU8);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlInterval(pub StdOptionOptionSqlxPostgresTypesPgInterval);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull(pub SqlxPostgresTypesPgInterval);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range(
    pub StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull(
    pub SqlxPostgresTypesPgRangeStdPrimitiveI64,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range(
    pub StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull(
    pub SqlxPostgresTypesPgRangeStdPrimitiveI32,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange(
    pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull(
    pub SqlxPostgresTypesPgRangeSqlxTypesDecimal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoney(pub StdOptionOptionSqlxPostgresTypesPgMoney);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull(pub SqlxPostgresTypesPgMoney);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgCiTextAsPostgresqlCiText(pub StdOptionOptionSqlxPostgresTypesPgCiText);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull(pub SqlxPostgresTypesPgCiText);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumeric(pub StdOptionOptionSqlxTypesBigDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumericNotNull(pub SqlxTypesBigDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesDecimalAsPostgresqlNumeric(pub StdOptionOptionSqlxTypesDecimal);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesDecimalAsPostgresqlNumericNotNull(pub SqlxTypesDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz(
    pub StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull(
    pub SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz(
    pub StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull(
    pub SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp(pub StdOptionOptionSqlxTypesChronoNaiveDateTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull(
    pub SqlxTypesChronoNaiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDate(pub StdOptionOptionSqlxTypesChronoNaiveDate);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull(pub SqlxTypesChronoNaiveDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTime(pub StdOptionOptionSqlxTypesChronoNaiveTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull(pub SqlxTypesChronoNaiveTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz(pub StdOptionOptionSqlxPostgresTypesPgTimeTz);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull(pub SqlxPostgresTypesPgTimeTz);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp(pub StdOptionOptionSqlxTypesTimePrimitiveDateTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull(
    pub SqlxTypesTimePrimitiveDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz(pub StdOptionOptionSqlxTypesTimeOffsetDateTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull(
    pub SqlxTypesTimeOffsetDateTime,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeDateAsPostgresqlDate(pub StdOptionOptionSqlxTypesTimeDate);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeDateAsPostgresqlDateNotNull(pub SqlxTypesTimeDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeTimeAsPostgresqlTime(pub StdOptionOptionSqlxTypesTimeTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeTimeAsPostgresqlTimeNotNull(pub SqlxTypesTimeTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuid(pub StdOptionOptionSqlxTypesUuidUuid);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuidNotNull(pub SqlxTypesUuidUuid);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey(pub SqlxTypesUuidUuid);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInet(pub StdOptionOptionSqlxTypesIpnetworkIpNetwork);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull(pub SqlxTypesIpnetworkIpNetwork);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr(pub StdOptionOptionSqlxTypesIpnetworkIpNetwork);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull(pub SqlxTypesIpnetworkIpNetwork);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlInet(pub StdOptionOptionStdNetIpAddr);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlInetNotNull(pub StdNetIpAddr);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlCidr(pub StdOptionOptionStdNetIpAddr);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct StdNetIpAddrAsPostgresqlCidrNotNull(pub StdNetIpAddr);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr(pub StdOptionOptionSqlxTypesMacAddressMacAddress);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull(
    pub SqlxTypesMacAddressMacAddress,
);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlBit(pub StdOptionOptionSqlxTypesBitVec);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlBitNotNull(pub SqlxTypesBitVec);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlVarBit(pub StdOptionOptionSqlxTypesBitVec);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesBitVecAsPostgresqlVarBitNotNull(pub SqlxTypesBitVec);
//todo what to do with generic?
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJson<T>(pub StdOptionOptionSqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJson<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonNotNull<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonB<T>(pub StdOptionOptionSqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonB<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonBNotNull<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonBNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJson(pub StdOptionOptionSerdeJsonValue);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJsonNotNull(pub SerdeJsonValue);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJsonB(pub StdOptionOptionSerdeJsonValue);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SerdeJsonValueAsPostgresqlJsonBNotNull(pub SerdeJsonValue);
// todo shared enum of postgres types for postgresql_crud and generate_postgresql_crud
// remove and make one
//todo support variations of init functions as enum

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TestOption<T> {
    std_primitive_bool: std::option::Option<std::primitive::bool>,
    std_primitive_i16: std::option::Option<std::primitive::i16>,
    std_primitive_i32: std::option::Option<std::primitive::i32>,
    std_primitive_i64: std::option::Option<std::primitive::i64>,
    std_primitive_f32: std::option::Option<std::primitive::f32>,
    std_primitive_f64: std::option::Option<std::primitive::f64>,
    std_string_string: std::option::Option<std::string::String>,
    std_vec_vec_std_primitive_u8: std::option::Option<std::vec::Vec<std::primitive::u8>>,
    sqlx_postgres_types_pg_interval: std::option::Option<sqlx::postgres::types::PgInterval>,
    sqlx_postgres_types_pg_range_std_primitive_i64: std::option::Option<sqlx::postgres::types::PgRange<std::primitive::i64>>,
    sqlx_postgres_types_pg_range_std_primitive_i32: std::option::Option<sqlx::postgres::types::PgRange<std::primitive::i32>>,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>>,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>>,
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>>,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>>,
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>>,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>>,
    sqlx_postgres_types_pg_range_sqlx_types_time_date: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::time::Date>>,
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>,
    sqlx_postgres_types_pg_range_sqlx_types_decimal: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::Decimal>>,
    sqlx_postgres_types_pg_money: std::option::Option<sqlx::postgres::types::PgMoney>,
    sqlx_postgres_types_pg_ci_text: std::option::Option<sqlx::postgres::types::PgCiText>,
    sqlx_types_big_decimal: std::option::Option<sqlx::types::BigDecimal>,
    sqlx_types_decimal: std::option::Option<sqlx::types::Decimal>,
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: std::option::Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: std::option::Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,
    sqlx_types_chrono_naive_date_time: std::option::Option<sqlx::types::chrono::NaiveDateTime>,
    sqlx_types_chrono_naive_date: std::option::Option<sqlx::types::chrono::NaiveDate>,
    sqlx_types_chrono_naive_time: std::option::Option<sqlx::types::chrono::NaiveTime>,
    sqlx_postgres_types_pg_time_tz: std::option::Option<sqlx::postgres::types::PgTimeTz>,
    sqlx_types_time_primitive_date_time: std::option::Option<sqlx::types::time::PrimitiveDateTime>,
    sqlx_types_time_offset_date_time: std::option::Option<sqlx::types::time::OffsetDateTime>,
    sqlx_types_time_date: std::option::Option<sqlx::types::time::Date>,
    sqlx_types_time_time: std::option::Option<sqlx::types::time::Time>,
    sqlx_types_uuid_uuid: std::option::Option<sqlx::types::uuid::Uuid>,
    sqlx_types_ipnetwork_ip_network: std::option::Option<sqlx::types::ipnetwork::IpNetwork>,
    std_net_ip_addr: std::option::Option<std::net::IpAddr>,
    sqlx_types_mac_address_mac_address: std::option::Option<sqlx::types::mac_address::MacAddress>,
    sqlx_types_bit_vec: std::option::Option<sqlx::types::BitVec>,
    sqlx_types_json: std::option::Option<sqlx::types::Json<T>>,
    serde_json_value: std::option::Option<serde_json::Value>,
}

#[derive(Debug)]
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

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(
            value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc
        );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local);
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
            match SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime::from(
            value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time,
        );
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
            match SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::try_from(
                value.sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time,
            ) {
                Ok(value) => value,
                Err(_e) => {
                    return Err(());
                }
            };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate::from(
            value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date
        );
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
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxTypesChronoDateTimeSqlxTypesChronoUtc::from(
            value.sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
        );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxTypesChronoDateTimeSqlxTypesChronoLocal::from(
            value.sqlx_types_chrono_date_time_sqlx_types_chrono_local
        );
        let sqlx_types_chrono_naive_date_time = SqlxTypesChronoNaiveDateTime::from(value.sqlx_types_chrono_naive_date_time);
        let sqlx_types_chrono_naive_date = SqlxTypesChronoNaiveDate::from(value.sqlx_types_chrono_naive_date);
        let sqlx_types_chrono_naive_time = SqlxTypesChronoNaiveTime::from(value.sqlx_types_chrono_naive_time);
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
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc, 
            sqlx_types_chrono_date_time_sqlx_types_chrono_local, 
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
            serde_json_value
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

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    something: std::string::String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
            TimeMonthWithSerializeDeserialize::January => Self::January,
            TimeMonthWithSerializeDeserialize::February => Self::February,
            TimeMonthWithSerializeDeserialize::March => Self::March,
            TimeMonthWithSerializeDeserialize::April => Self::April,
            TimeMonthWithSerializeDeserialize::May => Self::May,
            TimeMonthWithSerializeDeserialize::June => Self::June,
            TimeMonthWithSerializeDeserialize::July => Self::July,
            TimeMonthWithSerializeDeserialize::August => Self::August,
            TimeMonthWithSerializeDeserialize::September => Self::September,
            TimeMonthWithSerializeDeserialize::October => Self::October,
            TimeMonthWithSerializeDeserialize::November => Self::November,
            TimeMonthWithSerializeDeserialize::December => Self::December,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize {
    hours: std::primitive::i8,
    minutes: std::primitive::i8,
    seconds: std::primitive::i8,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_to_std_string_string]
        time_error_component_range: time::error::ComponentRange,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize> for sqlx::types::time::UtcOffset {
    type Error = SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        match Self::from_hms(value.hours, value.minutes, value.seconds) {
            Ok(value) => Ok(value),
            Err(error) => Err(Self::Error::TimeErrorComponentRange {
                time_error_component_range: error,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum NumBigintSignWithSerializeDeserialize {
    Minus,
    NoSign,
    Plus,
}
impl std::convert::From<NumBigintSignWithSerializeDeserialize> for num_bigint::Sign {
    fn from(value: NumBigintSignWithSerializeDeserialize) -> Self {
        match value {
            NumBigintSignWithSerializeDeserialize::Minus => Self::Minus,
            NumBigintSignWithSerializeDeserialize::NoSign => Self::NoSign,
            NumBigintSignWithSerializeDeserialize::Plus => Self::Plus,
        }
    }
}
impl std::convert::From<num_bigint::Sign> for NumBigintSignWithSerializeDeserialize {
    fn from(value: num_bigint::Sign) -> Self {
        match value {
            num_bigint::Sign::Minus => Self::Minus,
            num_bigint::Sign::NoSign => Self::NoSign,
            num_bigint::Sign::Plus => Self::Plus,
        }
    }
}
//todo pub or not for all - think
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
            sqlx_types_chrono_naive_date_handle,
            sqlx_types_chrono_naive_time_handle,
        );
        let sqlx_types_time_primitive_date_time_handle = sqlx::types::time::PrimitiveDateTime::new(
            sqlx_types_time_date_handle,
            sqlx_types_time_time_handle,
        );
        let sqlx_types_chrono_fixed_offset_handle =
            sqlx::types::chrono::FixedOffset::west_opt(std_primitive_i32_handle).unwrap();
        let sqlx_types_time_offset_date_time_handle =
            sqlx::types::time::OffsetDateTime::from_unix_timestamp(std::primitive::i64::default())
                .unwrap();
        let sqlx_types_decimal_handle = sqlx::types::Decimal::try_new(
            std_primitive_i64_handle,
            std_primitive_u32_handle,
        )
        .unwrap();
        let sqlx_types_chrono_utc_handle = sqlx::types::chrono::Utc;
        let sqlx_types_big_decimal_handle = sqlx::types::BigDecimal::new(
            num_bigint::BigInt::new(
                num_bigint::Sign::Plus,
                vec![std_primitive_u32_handle],
            ),
            std_primitive_i64_handle,
        );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle =
            sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle,
                sqlx_types_chrono_utc_handle,
            );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle =
            sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle,
                sqlx_types_chrono_fixed_offset_handle,
            );
        let std_ops_bound_std_primitive_i64_handle =
            std::ops::Bound::<std::primitive::i64>::Included(std_primitive_i64_handle);
        let std_ops_bound_std_primitive_i32_handle =
            std::ops::Bound::<std::primitive::i32>::Included(std_primitive_i32_handle);
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle =
            std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>::Included(
                sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle,
            );
        let std_ops_bound_sqlx_types_time_primitive_date_time_handle =
            std::ops::Bound::<sqlx::types::time::PrimitiveDateTime>::Included(
                sqlx_types_time_primitive_date_time_handle,
            );
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle =
            std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>::Included(
                sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle,
            );
        let std_ops_bound_sqlx_types_time_offset_date_time_handle =
            std::ops::Bound::<sqlx::types::time::OffsetDateTime>::Included(
                sqlx_types_time_offset_date_time_handle,
            );
        let std_ops_bound_sqlx_types_chrono_naive_date_handle =
            std::ops::Bound::<sqlx::types::chrono::NaiveDate>::Included(
                sqlx_types_chrono_naive_date_handle,
            );
        let std_ops_bound_sqlx_types_time_date_handle =
            std::ops::Bound::<sqlx::types::time::Date>::Included(
                sqlx_types_time_date_handle,
            );
        let std_ops_bound_sqlx_types_big_decimal_handle =
            std::ops::Bound::<sqlx::types::BigDecimal>::Included(
                sqlx_types_big_decimal_handle.clone(),
            );
        let std_ops_bound_sqlx_types_decimal_handle =
            std::ops::Bound::<sqlx::types::Decimal>::Included(sqlx_types_decimal_handle);
        let std_ops_bound_sqlx_types_chrono_naive_date_time_handle =
            std::ops::Bound::<sqlx::types::chrono::NaiveDateTime>::Included(
                sqlx_types_chrono_naive_date_time_handle,
            );
        let std_primitive_bool = StdPrimitiveBool(true);
        let std_primitive_i16 = StdPrimitiveI16(std::primitive::i16::default());
        let std_primitive_i32 = StdPrimitiveI32(std_primitive_i32_handle);
        let std_primitive_i64 = StdPrimitiveI64(std_primitive_i64_handle);
        let std_primitive_f32 = StdPrimitiveF32(std::primitive::f32::default());
        let std_primitive_f64 = StdPrimitiveF64(std::primitive::f64::default());
        let std_string_string = StdStringString(std_string_string_handle.clone());
        let std_vec_vec_std_primitive_u8 =
            StdVecVecStdPrimitiveU8(vec![std_primitive_u8_handle]);
        let sqlx_postgres_types_pg_interval =
            SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval {
                months: std_primitive_i32_handle,
                days: std_primitive_i32_handle,
                microseconds: std_primitive_i64_handle,
            });
        let sqlx_postgres_types_pg_range_std_primitive_i64 =
            SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange::<
                std::primitive::i64,
            > {
                start: std_ops_bound_std_primitive_i64_handle,
                end: std_ops_bound_std_primitive_i64_handle,
            });
        let sqlx_postgres_types_pg_range_std_primitive_i32 =
            SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange::<
                std::primitive::i32,
            > {
                start: std_ops_bound_std_primitive_i32_handle,
                end: std_ops_bound_std_primitive_i32_handle,
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
                sqlx::postgres::types::PgRange::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                > {
                    start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle,
                    end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle,
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
                sqlx::postgres::types::PgRange::<
                    sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                > {
                    start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle,
                    end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle,
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::time::OffsetDateTime,
            > {
                start: std_ops_bound_sqlx_types_time_offset_date_time_handle,
                end: std_ops_bound_sqlx_types_time_offset_date_time_handle,
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::NaiveDateTime,
            > {
                start: std_ops_bound_sqlx_types_chrono_naive_date_time_handle,
                end: std_ops_bound_sqlx_types_chrono_naive_date_time_handle,
            });
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
                sqlx::postgres::types::PgRange::<sqlx::types::time::PrimitiveDateTime> {
                    start: std_ops_bound_sqlx_types_time_primitive_date_time_handle,
                    end: std_ops_bound_sqlx_types_time_primitive_date_time_handle,
                },
            );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date =
            SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::NaiveDate,
            > {
                start: std_ops_bound_sqlx_types_chrono_naive_date_handle,
                end: std_ops_bound_sqlx_types_chrono_naive_date_handle,
            });
        let sqlx_postgres_types_pg_range_sqlx_types_time_date =
            SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange::<
                sqlx::types::time::Date,
            > {
                start: std_ops_bound_sqlx_types_time_date_handle,
                end: std_ops_bound_sqlx_types_time_date_handle,
            });
        let sqlx_postgres_types_pg_range_sqlx_types_big_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange::<
                sqlx::types::BigDecimal,
            > {
                start: std_ops_bound_sqlx_types_big_decimal_handle.clone(),
                end: std_ops_bound_sqlx_types_big_decimal_handle
            });
        let sqlx_postgres_types_pg_range_sqlx_types_decimal =
            SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange::<
                sqlx::types::Decimal,
            > {
                start: std_ops_bound_sqlx_types_decimal_handle,
                end: std_ops_bound_sqlx_types_decimal_handle,
            });
        let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney(
            sqlx::postgres::types::PgMoney(std_primitive_i64_handle),
        );
        let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText(
            sqlx::postgres::types::PgCiText(std_string_string_handle.clone()),
        );
        let sqlx_types_big_decimal = SqlxTypesBigDecimal(sqlx_types_big_decimal_handle);
        let sqlx_types_decimal = SqlxTypesDecimal(sqlx_types_decimal_handle);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Utc,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle,
                sqlx_types_chrono_utc_handle,
            ));
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Local,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle,
                sqlx_types_chrono_fixed_offset_handle,
            ));
        let sqlx_types_chrono_naive_date_time =
            SqlxTypesChronoNaiveDateTime(sqlx_types_chrono_naive_date_time_handle);
        let sqlx_types_chrono_naive_date =
            SqlxTypesChronoNaiveDate(sqlx_types_chrono_naive_date_handle);
        let sqlx_types_chrono_naive_time =
            SqlxTypesChronoNaiveTime(sqlx_types_chrono_naive_time_handle);
        let sqlx_postgres_types_pg_time_tz =
            SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz {
                time: sqlx_types_time_time_handle,
                offset: sqlx::types::time::UtcOffset::from_hms(
                    std_primitive_i8_handle,
                    std_primitive_i8_handle,
                    std_primitive_i8_handle,
                )
                .unwrap(),
            });
        let sqlx_types_time_primitive_date_time =
            SqlxTypesTimePrimitiveDateTime(sqlx_types_time_primitive_date_time_handle);
        let sqlx_types_time_offset_date_time =
            SqlxTypesTimeOffsetDateTime(sqlx_types_time_offset_date_time_handle);
        let sqlx_types_time_date = SqlxTypesTimeDate(sqlx_types_time_date_handle);
        let sqlx_types_time_time = SqlxTypesTimeTime(sqlx_types_time_time_handle);
        let sqlx_types_uuid_uuid = SqlxTypesUuidUuid(sqlx::types::uuid::Uuid::from_u128(
            std::primitive::u128::default(),
        ));
        let sqlx_types_ipnetwork_ip_network =
            SqlxTypesIpnetworkIpNetwork(sqlx::types::ipnetwork::IpNetwork::V6(
                sqlx::types::ipnetwork::Ipv6Network::new(
                    std::net::Ipv6Addr::new(
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                        std_primitive_u16_handle,
                    ),
                    std_primitive_u8_handle,
                )
                .unwrap(),
            ));
        let std_net_ip_addr = StdNetIpAddr(std::net::IpAddr::V6(core::net::Ipv6Addr::new(
            std_primitive_u16_handle,
            std_primitive_u16_handle,
            std_primitive_u16_handle,
            std_primitive_u16_handle,
            std_primitive_u16_handle,
            std_primitive_u16_handle,
            std_primitive_u16_handle,
            std_primitive_u16_handle,
        )));
        let sqlx_types_mac_address_mac_address =
            SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress::new([
                std_primitive_u8_handle,
                std_primitive_u8_handle,
                std_primitive_u8_handle,
                std_primitive_u8_handle,
                std_primitive_u8_handle,
                std_primitive_u8_handle,
            ]));
        let sqlx_types_bit_vec = SqlxTypesBitVec(sqlx::types::BitVec::new());
        let sqlx_types_json = SqlxTypesJson(sqlx::types::Json(Something {
            something: std_string_string_handle,
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
            sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date, 
            sqlx_postgres_types_pg_range_sqlx_types_time_date, 
            sqlx_postgres_types_pg_range_sqlx_types_big_decimal, 
            sqlx_postgres_types_pg_range_sqlx_types_decimal, 
            sqlx_postgres_types_pg_money, 
            sqlx_postgres_types_pg_ci_text, 
            sqlx_types_big_decimal, 
            sqlx_types_decimal, 
            sqlx_types_chrono_date_time_sqlx_types_chrono_utc, 
            sqlx_types_chrono_date_time_sqlx_types_chrono_local, 
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
            serde_json_value
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, 
    postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, 
    postgresql_crud_types_macro_logic_reuse::CommonFrom, 
    postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl
)]
pub struct StdPrimitiveBool(pub std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
impl AsPostgresqlBool for StdPrimitiveBool {}
impl PostgresqlOrder for StdPrimitiveBool {}
impl AsPostgresqlBool for StdOptionOptionStdPrimitiveBool {}
impl PostgresqlOrder for StdOptionOptionStdPrimitiveBool {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
impl AsPostgresqlSmallInt for StdPrimitiveI16 {}
impl AsPostgresqlSmallSerial for StdPrimitiveI16 {}
impl AsPostgresqlInt2 for StdPrimitiveI16 {}
impl PostgresqlOrder for StdPrimitiveI16 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
impl AsPostgresqlInt for StdPrimitiveI32 {}
impl AsPostgresqlSerial for StdPrimitiveI32 {}
impl AsPostgresqlInt4 for StdPrimitiveI32 {}
impl PostgresqlOrder for StdPrimitiveI32 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
impl AsPostgresqlBigInt for StdPrimitiveI64 {}
impl AsPostgresqlBigSerial for StdPrimitiveI64 {}
impl AsPostgresqlInt8 for StdPrimitiveI64 {}
impl PostgresqlOrder for StdPrimitiveI64 {}

#[derive(Debug, Clone, Copy, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithoutEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithoutEqImpl)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
impl AsPostgresqlReal for StdPrimitiveF32 {}
impl AsPostgresqlFloat4 for StdPrimitiveF32 {}
impl PostgresqlOrder for StdPrimitiveF32 {}

#[derive(Debug, Clone, Copy, PartialEq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithoutEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithoutEqImpl)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
impl AsPostgresqlDoublePrecision for StdPrimitiveF64 {}
impl AsPostgresqlFloat8 for StdPrimitiveF64 {}
impl PostgresqlOrder for StdPrimitiveF64 {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdStringString(pub std::string::String);
impl AsPostgresqlVarchar for StdStringString {}
impl AsPostgresqlCharN for StdStringString {}
impl AsPostgresqlText for StdStringString {}
impl AsPostgresqlCiText for StdStringString {}
impl PostgresqlOrder for StdStringString {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
impl AsPostgresqlBytea for StdVecVecStdPrimitiveU8 {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "months: {}, days: {}, microseconds: {}", self.months, self.days, self.microseconds)
    }
}
impl AsPostgresqlInterval for SqlxPostgresTypesPgInterval {}
impl PostgresqlOrder for SqlxPostgresTypesPgInterval {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(
    pub sqlx::postgres::types::PgRange<std::primitive::i64>,
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlInt8Range for SqlxPostgresTypesPgRangeStdPrimitiveI64 {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(
    pub sqlx::postgres::types::PgRange<std::primitive::i32>,
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
);
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,
);
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {}
//*

//*

#[derive(Debug, Clone, PartialEq, Eq, 
    postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, 
    postgresql_crud_types_macro_logic_reuse::CommonTryFrom, 
    postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {}
//

//

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>,
);
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,
);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlTsRange for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,
);
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>,
);
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    (Ok(_), Err(error)) => return Err(Self::Error::End { 
                        end: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                    (Err(error), Ok(_)) => return Err(Self::Error::Start { 
                        start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
                    Err(error) => {
                        return Err(Self::Error::Start { 
                            start: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,
);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>,
);
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "start: {:?}, end: {:?}", self.start, self.end)
    }
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesDecimal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
impl AsPostgresqlMoney for SqlxPostgresTypesPgMoney {}

#[derive(Debug, Clone, PartialEq,
    postgresql_crud_types_macro_logic_reuse::CommonWithSerializeDeserializeEqImpl,
    postgresql_crud_types_macro_logic_reuse::CommonFrom, 
    postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl
)]
pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
impl AsPostgresqlCiText for SqlxPostgresTypesPgCiText {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "digits: {:?}, scale: {:?}", self.digits, self.scale)
    }
}
impl AsPostgresqlNumeric for SqlxTypesBigDecimal {}
impl PostgresqlOrder for SqlxTypesBigDecimal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
);
impl AsPostgresqlTimestampTz for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
impl AsPostgresqlTimestamp for SqlxTypesChronoNaiveDateTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDateTime {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
impl AsPostgresqlDate for SqlxTypesChronoNaiveDate {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDate {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
impl AsPostgresqlTime for SqlxTypesChronoNaiveTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveTime {}

#[derive(Debug, Clone, Copy, PartialEq, postgresql_crud_types_macro_logic_reuse::CommonWithSerializeDeserializeEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
            (Err(error), Ok(_)) => {
                return Err(Self::Error::Time { 
                    time: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            (Ok(_), Err(error)) => {
                return Err(Self::Error::Offset { 
                    offset: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "time: {}, offset: {:?}", self.time, self.offset)
    }
}
impl AsPostgresqlTimeTz for SqlxPostgresTypesPgTimeTz {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
            (Err(error), Ok(_)) => {
                return Err(Self::Error::Date { 
                    date: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            (Ok(_), Err(error)) => {
                return Err(Self::Error::Time { 
                    time: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "date: {}, time: {}", self.date, self.time)
    }
}
impl AsPostgresqlTimestamp for SqlxTypesTimePrimitiveDateTime {}
impl PostgresqlOrder for SqlxTypesTimePrimitiveDateTime {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize(
    std::primitive::i64,
);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeOffsetDateTimeWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_to_std_string_string]
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
            Err(error) => Err(Self::Error::TimeErrorComponentRange {
                time_error_component_range: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
impl AsPostgresqlTimestampTz for SqlxTypesTimeOffsetDateTime {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeDateWithSerializeDeserialize {
    year: std::primitive::i32,
    month: TimeMonthWithSerializeDeserialize,
    day: std::primitive::u8,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeDateWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_to_std_string_string]
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
            Err(error) => Err(Self::Error::TimeErrorComponentRange {
                time_error_component_range: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "year: {}, month: {:?}, day: {}", self.year, self.month, self.day)
    }
}
impl AsPostgresqlDate for SqlxTypesTimeDate {}
impl PostgresqlOrder for SqlxTypesTimeDate {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeTimeWithSerializeDeserialize {
    hour: std::primitive::u8,
    minute: std::primitive::u8,
    second: std::primitive::u8,
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed {
    TimeErrorComponentRange {
        #[eo_to_std_string_string]
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
            Err(error) => Err(Self::Error::TimeErrorComponentRange{
                time_error_component_range: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "hour: {}, minute: {}, second: {}", self.hour, self.minute, self.second)
    }
}
impl AsPostgresqlTime for SqlxTypesTimeTime {}
impl PostgresqlOrder for SqlxTypesTimeTime {}
//todo maybe its possible to not use Clone (refactor where .clone() used)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonTryFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificTryFromWithEqImpl)]
pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesUuidUuidWithSerializeDeserialize(std::string::String);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidError {
        #[eo_to_std_string_string]
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
            Err(error) => Err(Self::Error::SqlxTypesUuidError{
                sqlx_types_uuid_error: error,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl AsPostgresqlUuid for SqlxTypesUuidUuid {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
impl AsPostgresqlInet for SqlxTypesIpnetworkIpNetwork {}
impl AsPostgresqlCidr for SqlxTypesIpnetworkIpNetwork {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdNetIpAddr(pub std::net::IpAddr);
impl AsPostgresqlInet for StdNetIpAddr {}
impl AsPostgresqlCidr for StdNetIpAddr {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl AsPostgresqlMacAddr for SqlxTypesMacAddressMacAddress {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonSpecificFromWithEqImpl)]
pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl AsPostgresqlBit for SqlxTypesBitVec {}
impl AsPostgresqlVarBit for SqlxTypesBitVec {}
impl PostgresqlOrder for SqlxTypesBitVec {}

#[derive(Debug, PartialEq, Eq)]
pub struct SqlxTypesJson<T>(sqlx::types::Json<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
            Err(error) => Err(error),
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
        Self::SqlxTypesJsonT
    }
}
impl<T> SqlxTypesJson<T> {
    pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value
            .into_iter()
            .map(Self::into_inner)
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

//
#[derive(Debug, PartialEq, Eq)]
pub struct StdOptionOptionSqlxTypesJson<T>(pub std::option::Option<sqlx::types::Json<T>>);
impl<T: std::fmt::Debug> std::fmt::Display for StdOptionOptionSqlxTypesJson<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl<T> StdOptionOptionSqlxTypesJson<T> {
    pub fn into_inner(self) -> std::option::Option<sqlx::types::Json<T>> {
        self.0
    }
}
impl<T> std::convert::From<StdOptionOptionSqlxTypesJson<T>> for std::option::Option<sqlx::types::Json<T>> {
    fn from(value: StdOptionOptionSqlxTypesJson<T>) -> Self {
        value.0
    }
}
impl<T> sqlx::Type<sqlx::Postgres> for StdOptionOptionSqlxTypesJson<T> {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<sqlx::types::Json<T>> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::option::Option<sqlx::types::Json<T>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<T> CheckSupportedPostgresqlColumnType for StdOptionOptionSqlxTypesJson<T> {
    fn check_supported_postgresql_column_type() {}
}
impl<T> std::convert::From<StdOptionOptionSqlxTypesJson<T>> for SupportedSqlxPostgresType {
    fn from(_value: StdOptionOptionSqlxTypesJson<T>) -> Self {
        Self::SqlxTypesJsonT
    }
}
impl<T> StdOptionOptionSqlxTypesJson<T> {
    pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<sqlx::types::Json<T>>> {
        value
            .into_iter()
            .map(Self::into_inner)
            .collect()
    }
}
// impl<T: std::marker::Send + serde::Serialize> BindQuery for StdOptionOptionSqlxTypesJson<T> {
//     fn try_increment(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(())
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                 checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }),
//         }
//     }
//     fn try_generate_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
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
//     fn bind_value_to_query(
//         self,
//         mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.0);
//         query
//     }
// }
#[derive(Debug, PartialEq, Eq)]
pub struct WhereStdOptionOptionSqlxTypesJson<T> {
    pub value: StdOptionOptionSqlxTypesJson<T>,
    pub conjuctive_operator: ConjunctiveOperator,
}
impl<T: std::fmt::Debug> std::fmt::Display for WhereStdOptionOptionSqlxTypesJson<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "value: {}, conjuctive_operator: {}",
            self.value, self.conjuctive_operator
        )
    }
}
// impl<T: std::marker::Send + serde::Serialize> BindQuery for WhereStdOptionOptionSqlxTypesJson<T> {
//     fn try_increment(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(())
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                 checked_add: std::string::String::from("checked_add is None"),
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }),
//         }
//     }
//     fn try_generate_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                 checked_add: std::string::String::from("checked_add is None"),
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }),
//         }
//     }
//     fn bind_value_to_query(
//         self,
//         mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.value.0);
//         query
//     }
// }
#[derive(Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
pub struct WhereStdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T> {
    pub value: StdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T>,
    pub conjuctive_operator: ConjunctiveOperator,
}
impl<T: std::fmt::Debug> std::fmt::Display for WhereStdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "value: {}, conjuctive_operator: {}",
            self.value, self.conjuctive_operator
        )
    }
}
impl<T> std::convert::From<WhereStdOptionOptionSqlxTypesJson<T>> for WhereStdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T> {
    fn from(value: WhereStdOptionOptionSqlxTypesJson<T>) -> Self {
        Self {
            value: StdOptionOptionSqlxTypesJsonWithSerializeDeserialize::from(value.value),
            conjuctive_operator: value.conjuctive_operator,
        }
    }
}
//
impl<T> std::convert::From<WhereStdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T>> for WhereStdOptionOptionSqlxTypesJson<T> {
    fn from(value: WhereStdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T>) -> Self {
        Self {
            value: StdOptionOptionSqlxTypesJson::from(value.value),
            conjuctive_operator: value.conjuctive_operator,
        }
    }
}
//
#[derive(
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct StdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T>(std::option::Option<SqlxTypesJsonWithSerializeDeserialize<T>>);
impl<T: std::fmt::Debug> std::fmt::Display for StdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl<T> std::convert::From<StdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T>> for StdOptionOptionSqlxTypesJson<T> {
    fn from(value: StdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T>) -> Self {
        value.0.map_or_else(|| Self(None), |value| Self(Some(SqlxTypesJson::from(value).0)))
    }
}
impl<T> std::convert::From<StdOptionOptionSqlxTypesJson<T>> for StdOptionOptionSqlxTypesJsonWithSerializeDeserialize<T> {
    fn from(value: StdOptionOptionSqlxTypesJson<T>) -> Self {
        value.0.map_or_else(|| Self(None), |value| Self(Some(SqlxTypesJsonWithSerializeDeserialize::from(SqlxTypesJson(value)))))
    }
}
//


#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::FieldTypeImplementsSerializeDeserializeWithEqImpl, postgresql_crud_types_macro_logic_reuse::CommonFrom, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SerdeJsonValue(pub serde_json::Value);
impl AsPostgresqlJson for SerdeJsonValue {}
impl AsPostgresqlJsonB for SerdeJsonValue {}

// pub async fn something() {
    // let mut query = sqlx::query::<sqlx::Postgres>("test");
    // query = query.bind(Into::<bool>::into(StdPrimitiveBool(false)));
    // query = query.bind(StdPrimitiveBool(false).into_inner());
    // let _query = query.bind(StdPrimitiveBool(false));
// }

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
        #[eo_to_std_string_string_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub trait BindQuery {
    fn try_increment(&self, increment: &mut u64) -> Result<(), TryGenerateBindIncrementsErrorNamed>;
    fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed>;
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub enum ConjunctiveOperator {
    Or,
    And,
}

impl std::fmt::Display for ConjunctiveOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Or => write!(formatter, "{}", <naming_constants::Or as naming_constants::Naming>::upper_camel_case_stringified()),
            Self::And => {
                write!(formatter, "{}", <naming_constants::And as naming_constants::Naming>::upper_camel_case_stringified())
            }
        }
    }
}

//this needed coz serde std::option::Option<std::option::Option<T>> #[serde(skip_serializing_if = "Option::is_none")] - if both options: inner and parent is null then it skip - its not correct
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Value<T> {
    pub value: T
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, from_str::FromStr)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(formatter, "{}", <naming_constants::Asc as naming_constants::Naming>::upper_camel_case_stringified()),
            Self::Desc => write!(formatter, "{}", <naming_constants::Desc as naming_constants::Naming>::upper_camel_case_stringified()),
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<Order>,
}



// impl axum::response::IntoResponse for axum::extract::rejection::JsonRejection {
//     fn into_response(self) -> axum::response::Response {
//         // match &self {

//         // }
//         todo!()
//     }
// }

//
// #[derive(
//     Debug,
//     thiserror::Error,
//     error_occurence_lib::ErrorOccurence,
// )]
// pub enum JsonExtractorErrorNamed {
//     JsonDataError {
//         #[eo_to_std_string_string_serialize_deserialize]
//         json_data_error: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     JsonSyntaxError {
//         #[eo_to_std_string_string_serialize_deserialize]
//         json_syntax_error: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     MissingJsonContentType {
//         #[eo_to_std_string_string_serialize_deserialize]
//         missing_json_content_type: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     BytesRejection {
//         #[eo_to_std_string_string_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NonExhaustiveCase {
//         #[eo_to_std_string_string_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// impl std::convert::From<axum::extract::rejection::JsonRejection> for JsonExtractorErrorNamed {
//     fn from(value: axum::extract::rejection::JsonRejection) -> Self {
//                 let f = match e {
//                     axum::extract::rejection::JsonRejectionJsonDataError(value),
//                     axum::extract::rejection::JsonRejectionJsonSyntaxError(JsonSyntaxError),
//                     axum::extract::rejection::JsonRejectionMissingJsonContentType(MissingJsonContentType),
//                     axum::extract::rejection::JsonRejectionBytesRejection(BytesRejection),
//                 };
//     }
// }


//