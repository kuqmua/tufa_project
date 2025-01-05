pub mod generate_postgresql_json_type;
pub mod value;
pub mod postgresql_type;
pub mod postgresql_json_type;

fn add_path(value: &str) -> std::string::String {
    format!("{}::{value}", naming::PostgresqlCrudSnakeCase)
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
            PostgresqlTypeWithMetadata::Bool | PostgresqlTypeWithMetadata::BoolNotNull => Self::Bool,
            PostgresqlTypeWithMetadata::Char | PostgresqlTypeWithMetadata::CharNotNull => Self::Char,
            PostgresqlTypeWithMetadata::SmallInt | PostgresqlTypeWithMetadata::SmallIntNotNull => Self::SmallInt,
            PostgresqlTypeWithMetadata::SmallSerial | PostgresqlTypeWithMetadata::SmallSerialNotNull => Self::SmallSerial,
            PostgresqlTypeWithMetadata::Int2 | PostgresqlTypeWithMetadata::Int2NotNull => Self::Int2,
            PostgresqlTypeWithMetadata::Int | PostgresqlTypeWithMetadata::IntNotNull => Self::Int,
            PostgresqlTypeWithMetadata::Serial | PostgresqlTypeWithMetadata::SerialNotNull => Self::Serial,
            PostgresqlTypeWithMetadata::Int4 | PostgresqlTypeWithMetadata::Int4NotNull => Self::Int4,
            PostgresqlTypeWithMetadata::BigInt | PostgresqlTypeWithMetadata::BigIntNotNull => Self::BigInt,
            PostgresqlTypeWithMetadata::BigSerial | PostgresqlTypeWithMetadata::BigSerialNotNull | PostgresqlTypeWithMetadata::BigSerialNotNullPrimaryKey => Self::BigSerial,
            PostgresqlTypeWithMetadata::Int8 | PostgresqlTypeWithMetadata::Int8NotNull => Self::Int8,
            PostgresqlTypeWithMetadata::Real | PostgresqlTypeWithMetadata::RealNotNull => Self::Real,
            PostgresqlTypeWithMetadata::Float4 | PostgresqlTypeWithMetadata::Float4NotNull => Self::Float4,
            PostgresqlTypeWithMetadata::DoublePrecision | PostgresqlTypeWithMetadata::DoublePrecisionNotNull => Self::DoublePrecision,
            PostgresqlTypeWithMetadata::Float8 | PostgresqlTypeWithMetadata::Float8NotNull => Self::Float8,
            PostgresqlTypeWithMetadata::Varchar | PostgresqlTypeWithMetadata::VarcharNotNull => Self::Varchar,
            PostgresqlTypeWithMetadata::CharN | PostgresqlTypeWithMetadata::CharNNotNull => Self::CharN,
            PostgresqlTypeWithMetadata::Text | PostgresqlTypeWithMetadata::TextNotNull => Self::Text,
            PostgresqlTypeWithMetadata::CiText | PostgresqlTypeWithMetadata::CiTextNotNull => Self::CiText,
            PostgresqlTypeWithMetadata::Bytea | PostgresqlTypeWithMetadata::ByteaNotNull => Self::Bytea,
            PostgresqlTypeWithMetadata::Interval | PostgresqlTypeWithMetadata::IntervalNotNull => Self::Interval,
            PostgresqlTypeWithMetadata::Int8Range | PostgresqlTypeWithMetadata::Int8RangeNotNull => Self::Int8Range,
            PostgresqlTypeWithMetadata::Int4Range | PostgresqlTypeWithMetadata::Int4RangeNotNull => Self::Int4Range,
            PostgresqlTypeWithMetadata::TsRange | PostgresqlTypeWithMetadata::TsRangeNotNull => Self::TsRange,
            PostgresqlTypeWithMetadata::TsTzRange | PostgresqlTypeWithMetadata::TsTzRangeNotNull => Self::TsTzRange,
            PostgresqlTypeWithMetadata::DateRange | PostgresqlTypeWithMetadata::DateRangeNotNull => Self::DateRange,
            PostgresqlTypeWithMetadata::NumRange | PostgresqlTypeWithMetadata::NumRangeNotNull => Self::NumRange,
            PostgresqlTypeWithMetadata::Money | PostgresqlTypeWithMetadata::MoneyNotNull => Self::Money,
            PostgresqlTypeWithMetadata::Numeric | PostgresqlTypeWithMetadata::NumericNotNull => Self::Numeric,
            PostgresqlTypeWithMetadata::TimestampTz | PostgresqlTypeWithMetadata::TimestampTzNotNull => Self::TimestampTz,
            PostgresqlTypeWithMetadata::Date | PostgresqlTypeWithMetadata::DateNotNull => Self::Date,
            PostgresqlTypeWithMetadata::Time | PostgresqlTypeWithMetadata::TimeNotNull => Self::Time,
            PostgresqlTypeWithMetadata::TimeTz | PostgresqlTypeWithMetadata::TimeTzNotNull => Self::TimeTz,
            PostgresqlTypeWithMetadata::Timestamp | PostgresqlTypeWithMetadata::TimestampNotNull => Self::Timestamp,
            PostgresqlTypeWithMetadata::Uuid | PostgresqlTypeWithMetadata::UuidNotNull | PostgresqlTypeWithMetadata::UuidNotNullPrimaryKey => Self::Uuid,
            PostgresqlTypeWithMetadata::Inet | PostgresqlTypeWithMetadata::InetNotNull => Self::Inet,
            PostgresqlTypeWithMetadata::Cidr | PostgresqlTypeWithMetadata::CidrNotNull => Self::Cidr,
            PostgresqlTypeWithMetadata::MacAddr | PostgresqlTypeWithMetadata::MacAddrNotNull => Self::MacAddr,
            PostgresqlTypeWithMetadata::Bit | PostgresqlTypeWithMetadata::BitNotNull => Self::Bit,
            PostgresqlTypeWithMetadata::VarBit | PostgresqlTypeWithMetadata::VarBitNotNull => Self::VarBit,
            PostgresqlTypeWithMetadata::Json | PostgresqlTypeWithMetadata::JsonNotNull => Self::Json,
            PostgresqlTypeWithMetadata::JsonB | PostgresqlTypeWithMetadata::JsonBNotNull => Self::JsonB,
        }
    }
}

impl PostgresqlTypeWithMetadata {
    pub const fn postgresql_naming(&self) -> &str {
        match self {
            Self::Bool => "BOOL",
            Self::BoolNotNull => "BOOL NOT NULL",
            Self::Char => "CHAR",
            Self::CharNotNull => "CHAR NOT NULL",
            Self::SmallInt => "SMALLINT",
            Self::SmallIntNotNull => "SMALLINT NOT NULL",
            Self::SmallSerial => "SMALLSERIAL",
            Self::SmallSerialNotNull => "SMALLSERIAL NOT NULL",
            Self::Int2 => "INT2",
            Self::Int2NotNull => "INT2 NOT NULL",
            Self::Int => "INT",
            Self::IntNotNull => "INT NOT NULL",
            Self::Serial => "SERIAL",
            Self::SerialNotNull => "SERIAL NOT NULL",
            Self::Int4 => "INT4",
            Self::Int4NotNull => "INT4 NOT NULL",
            Self::BigInt => "BIGINT",
            Self::BigIntNotNull => "BIGINT NOT NULL",
            Self::BigSerial => "BIGSERIAL",
            Self::BigSerialNotNull => "BIGSERIAL NOT NULL",
            Self::BigSerialNotNullPrimaryKey => "BIGSERIAL PRIMARY KEY", //not null to add NOT NULL coz its primary key
            Self::Int8 => "INT8",
            Self::Int8NotNull => "INT8 NOT NULL",
            Self::Real => "REAL",
            Self::RealNotNull => "REAL NOT NULL",
            Self::Float4 => "FLOAT4",
            Self::Float4NotNull => "FLOAT4 NOT NULL",
            Self::DoublePrecision => "DOUBLE PRECISION",
            Self::DoublePrecisionNotNull => "DOUBLE PRECISION NOT NULL",
            Self::Float8 => "FLOAT8",
            Self::Float8NotNull => "FLOAT8 NOT NULL",
            Self::Varchar => "VARCHAR",
            Self::VarcharNotNull => "VARCHAR NOT NULL",
            Self::CharN => "CHAR(N)",
            Self::CharNNotNull => "CHAR(N) NOT NULL",
            Self::Text => "TEXT",
            Self::TextNotNull => "TEXT NOT NULL",
            Self::CiText => "CITEXT",
            Self::CiTextNotNull => "CITEXT NOT NULL",
            Self::Bytea => "BYTEA",
            Self::ByteaNotNull => "BYTEA NOT NULL",
            Self::Interval => "INTERVAL",
            Self::IntervalNotNull => "INTERVAL NOT NULL",
            Self::Int8Range => "INT8RANGE",
            Self::Int8RangeNotNull => "INT8RANGE NOT NULL",
            Self::Int4Range => "INT4RANGE",
            Self::Int4RangeNotNull => "INT4RANGE NOT NULL",
            Self::TsRange => "TSRANGE",
            Self::TsRangeNotNull => "TSRANGE NOT NULL",
            Self::TsTzRange => "TSTZRANGE",
            Self::TsTzRangeNotNull => "TSTZRANGE NOT NULL",
            Self::DateRange => "DATERANGE",
            Self::DateRangeNotNull => "DATERANGE NOT NULL",
            Self::NumRange => "NUMRANGE",
            Self::NumRangeNotNull => "NUMRANGE NOT NULL",
            Self::Money => "MONEY",
            Self::MoneyNotNull => "MONEY NOT NULL",
            Self::Numeric => "NUMERIC",
            Self::NumericNotNull => "NUMERIC NOT NULL",
            Self::TimestampTz => "TIMESTAMPTZ",
            Self::TimestampTzNotNull => "TIMESTAMPTZ NOT NULL",
            Self::Date => "DATE",
            Self::DateNotNull => "DATE NOT NULL",
            Self::Time => "TIME",
            Self::TimeNotNull => "TIME NOT NULL",
            Self::TimeTz => "TIMETZ",
            Self::TimeTzNotNull => "TIMETZ NOT NULL",
            Self::Timestamp => "TIMESTAMP",
            Self::TimestampNotNull => "TIMESTAMP NOT NULL",
            Self::Uuid => "UUID",
            Self::UuidNotNull => "UUID NOT NULL",
            Self::UuidNotNullPrimaryKey => "UUID PRIMARY KEY", //not null to add NOT NULL coz its primary key
            Self::Inet => "INET",
            Self::InetNotNull => "INET NOT NULL",
            Self::Cidr => "CIDR",
            Self::CidrNotNull => "CIDR NOT NULL",
            Self::MacAddr => "MACADDR",
            Self::MacAddrNotNull => "MACADDR NOT NULL",
            Self::Bit => "BIT",
            Self::BitNotNull => "BIT NOT NULL",
            Self::VarBit => "VARBIT",
            Self::VarBitNotNull => "VARBIT NOT NULL",
            Self::Json => "JSON",
            Self::JsonNotNull => "JSON NOT NULL",
            Self::JsonB => "JSONB",
            Self::JsonBNotNull => "JSONB NOT NULL",
        }
    }
}

#[derive(Debug, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
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
    SqlxTypesJson,
    StdOptionOptionSqlxTypesJson,
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
            SqlxPostgresType::SqlxTypesJson => Self::SqlxTypesJson,
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
            SupportedSqlxPostgresType::SqlxTypesJson => Self::SqlxPostgresType(SqlxPostgresType::SqlxTypesJson),
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJson => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SqlxTypesJson),
            SupportedSqlxPostgresType::SerdeJsonValue => Self::SqlxPostgresType(SqlxPostgresType::SerdeJsonValue),
            SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Self::OptionSupportedSqlxPostgresType(OptionSupportedSqlxPostgresType::SerdeJsonValue),
        }
    }
}

#[derive(Debug, Clone, Copy, strum_macros::Display)]
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
    SqlxTypesJson,
    SerdeJsonValue,
}

impl SqlxPostgresType {
    pub const fn from_supported_sqlx_postgres_type_removing_option(value: &SupportedSqlxPostgresType) -> Self {
        match value {
            SupportedSqlxPostgresType::StdPrimitiveBool | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveBool => Self::StdPrimitiveBool,
            SupportedSqlxPostgresType::StdPrimitiveI16 | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI16 => Self::StdPrimitiveI16,
            SupportedSqlxPostgresType::StdPrimitiveI32 | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI32 => Self::StdPrimitiveI32,
            SupportedSqlxPostgresType::StdPrimitiveI64 | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI64 => Self::StdPrimitiveI64,
            SupportedSqlxPostgresType::StdPrimitiveF32 | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF32 => Self::StdPrimitiveF32,
            SupportedSqlxPostgresType::StdPrimitiveF64 | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF64 => Self::StdPrimitiveF64,
            SupportedSqlxPostgresType::StdStringString | SupportedSqlxPostgresType::StdOptionOptionStdStringString => Self::StdStringString,
            SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8 | SupportedSqlxPostgresType::StdOptionOptionStdVecVecStdPrimitiveU8 => Self::StdVecVecStdPrimitiveU8,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgInterval => Self::SqlxPostgresTypesPgInterval,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64 | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32 | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32 => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgMoney => Self::SqlxPostgresTypesPgMoney,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgCiText => Self::SqlxPostgresTypesPgCiText,
            SupportedSqlxPostgresType::SqlxTypesBigDecimal | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBigDecimal => Self::SqlxTypesBigDecimal,
            SupportedSqlxPostgresType::SqlxTypesDecimal | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesDecimal => Self::SqlxTypesDecimal,
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDateTime => Self::SqlxTypesChronoNaiveDateTime,
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDate => Self::SqlxTypesChronoNaiveDate,
            SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveTime => Self::SqlxTypesChronoNaiveTime,
            SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgTimeTz => Self::SqlxPostgresTypesPgTimeTz,
            SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimePrimitiveDateTime => Self::SqlxTypesTimePrimitiveDateTime,
            SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeOffsetDateTime => Self::SqlxTypesTimeOffsetDateTime,
            SupportedSqlxPostgresType::SqlxTypesTimeDate | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeDate => Self::SqlxTypesTimeDate,
            SupportedSqlxPostgresType::SqlxTypesTimeTime | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeTime => Self::SqlxTypesTimeTime,
            SupportedSqlxPostgresType::SqlxTypesUuidUuid | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesUuidUuid => Self::SqlxTypesUuidUuid,
            SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesIpnetworkIpNetwork => Self::SqlxTypesIpnetworkIpNetwork,
            SupportedSqlxPostgresType::StdNetIpAddr | SupportedSqlxPostgresType::StdOptionOptionStdNetIpAddr => Self::StdNetIpAddr,
            SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesMacAddressMacAddress => Self::SqlxTypesMacAddressMacAddress,
            SupportedSqlxPostgresType::SqlxTypesBitVec | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBitVec => Self::SqlxTypesBitVec,
            SupportedSqlxPostgresType::SqlxTypesJson | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJson => Self::SqlxTypesJson,
            SupportedSqlxPostgresType::SerdeJsonValue | SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Self::SerdeJsonValue,
        }
    }
    fn get_type_stringified(self, generic_type_str: &str) -> std::string::String {
        match self {
            Self::StdPrimitiveBool => std::string::String::from("std::primitive::bool"), //todo maybe Option<T> for nullable ?
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
            Self::SqlxTypesJson => format!("sqlx::types::Json{generic_type_str}"),
            Self::SerdeJsonValue => std::string::String::from("serde_json::Value"),
        }
    }
    pub fn get_path_stringified(&self) -> std::string::String {
        add_path(&self.to_string())
    }
}

impl std::convert::TryFrom<&SupportedSqlxPostgresType> for SqlxPostgresType {
    type Error = ();
    fn try_from(value: &SupportedSqlxPostgresType) -> Result<Self, Self::Error> {
        match value {
            SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveBool
            | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI16
            | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI32
            | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveI64
            | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF32
            | SupportedSqlxPostgresType::StdOptionOptionStdPrimitiveF64
            | SupportedSqlxPostgresType::StdOptionOptionStdStringString
            | SupportedSqlxPostgresType::StdOptionOptionStdVecVecStdPrimitiveU8
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgInterval
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgMoney
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgCiText
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBigDecimal
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesDecimal
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDateTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveDate
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesChronoNaiveTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxPostgresTypesPgTimeTz
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimePrimitiveDateTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeOffsetDateTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeDate
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesTimeTime
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesUuidUuid
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesIpnetworkIpNetwork
            | SupportedSqlxPostgresType::StdOptionOptionStdNetIpAddr
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesMacAddressMacAddress
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesBitVec
            | SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJson
            | SupportedSqlxPostgresType::StdOptionOptionSerdeJsonValue => Err(()),

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
            SupportedSqlxPostgresType::SqlxTypesJson => Ok(Self::SqlxTypesJson),
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
            OptionSupportedSqlxPostgresType::SqlxTypesJson => Self::SqlxTypesJson,
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
    SqlxTypesJson,
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
            OptionSupportedSqlxPostgresType::SqlxTypesJson => Self::StdOptionOptionSqlxTypesJson,
            OptionSupportedSqlxPostgresType::SerdeJsonValue => Self::StdOptionOptionSerdeJsonValue,
        }
    }
}

impl std::convert::TryFrom<&SupportedSqlxPostgresType> for OptionSupportedSqlxPostgresType {
    type Error = ();
    fn try_from(value: &SupportedSqlxPostgresType) -> Result<Self, Self::Error> {
        match value {
            SupportedSqlxPostgresType::StdPrimitiveBool
            | SupportedSqlxPostgresType::StdPrimitiveI16
            | SupportedSqlxPostgresType::StdPrimitiveI32
            | SupportedSqlxPostgresType::StdPrimitiveI64
            | SupportedSqlxPostgresType::StdPrimitiveF32
            | SupportedSqlxPostgresType::StdPrimitiveF64
            | SupportedSqlxPostgresType::StdStringString
            | SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText
            | SupportedSqlxPostgresType::SqlxTypesBigDecimal
            | SupportedSqlxPostgresType::SqlxTypesDecimal
            | SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime
            | SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate
            | SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime
            | SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz
            | SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime
            | SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime
            | SupportedSqlxPostgresType::SqlxTypesTimeDate
            | SupportedSqlxPostgresType::SqlxTypesTimeTime
            | SupportedSqlxPostgresType::SqlxTypesUuidUuid
            | SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork
            | SupportedSqlxPostgresType::StdNetIpAddr
            | SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress
            | SupportedSqlxPostgresType::SqlxTypesBitVec
            | SupportedSqlxPostgresType::SqlxTypesJson
            | SupportedSqlxPostgresType::SerdeJsonValue => Err(()),

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
            SupportedSqlxPostgresType::StdOptionOptionSqlxTypesJson => Ok(Self::SqlxTypesJson),
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
            SqlxPostgresType::SqlxTypesJson => Self::SqlxTypesJson,
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
            Self::SqlxTypesJson | Self::StdOptionOptionSqlxTypesJson | Self::SerdeJsonValue | Self::StdOptionOptionSerdeJsonValue => format!("{self}{generic_type_str}"),
            Self::StdPrimitiveBool
            | Self::StdOptionOptionStdPrimitiveBool
            | Self::StdPrimitiveI16
            | Self::StdOptionOptionStdPrimitiveI16
            | Self::StdPrimitiveI32
            | Self::StdOptionOptionStdPrimitiveI32
            | Self::StdPrimitiveI64
            | Self::StdOptionOptionStdPrimitiveI64
            | Self::StdPrimitiveF32
            | Self::StdOptionOptionStdPrimitiveF32
            | Self::StdPrimitiveF64
            | Self::StdOptionOptionStdPrimitiveF64
            | Self::StdStringString
            | Self::StdOptionOptionStdStringString
            | Self::StdVecVecStdPrimitiveU8
            | Self::StdOptionOptionStdVecVecStdPrimitiveU8
            | Self::SqlxPostgresTypesPgInterval
            | Self::StdOptionOptionSqlxPostgresTypesPgInterval
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI64
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI32
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate
            | Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal
            | Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal
            | Self::SqlxPostgresTypesPgMoney
            | Self::StdOptionOptionSqlxPostgresTypesPgMoney
            | Self::SqlxPostgresTypesPgCiText
            | Self::StdOptionOptionSqlxPostgresTypesPgCiText
            | Self::SqlxTypesBigDecimal
            | Self::StdOptionOptionSqlxTypesBigDecimal
            | Self::SqlxTypesDecimal
            | Self::StdOptionOptionSqlxTypesDecimal
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::SqlxTypesChronoNaiveDateTime
            | Self::StdOptionOptionSqlxTypesChronoNaiveDateTime
            | Self::SqlxTypesChronoNaiveDate
            | Self::StdOptionOptionSqlxTypesChronoNaiveDate
            | Self::SqlxTypesChronoNaiveTime
            | Self::StdOptionOptionSqlxTypesChronoNaiveTime
            | Self::SqlxPostgresTypesPgTimeTz
            | Self::StdOptionOptionSqlxPostgresTypesPgTimeTz
            | Self::SqlxTypesTimePrimitiveDateTime
            | Self::StdOptionOptionSqlxTypesTimePrimitiveDateTime
            | Self::SqlxTypesTimeOffsetDateTime
            | Self::StdOptionOptionSqlxTypesTimeOffsetDateTime
            | Self::SqlxTypesTimeDate
            | Self::StdOptionOptionSqlxTypesTimeDate
            | Self::SqlxTypesTimeTime
            | Self::StdOptionOptionSqlxTypesTimeTime
            | Self::SqlxTypesUuidUuid
            | Self::StdOptionOptionSqlxTypesUuidUuid
            | Self::SqlxTypesIpnetworkIpNetwork
            | Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork
            | Self::StdNetIpAddr
            | Self::StdOptionOptionStdNetIpAddr
            | Self::SqlxTypesMacAddressMacAddress
            | Self::StdOptionOptionSqlxTypesMacAddressMacAddress
            | Self::SqlxTypesBitVec
            | Self::StdOptionOptionSqlxTypesBitVec => self.to_string(),
        }
    }
    pub fn get_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_handle_stringified(generic_type_str))
    }
    fn get_inner_type_with_serialize_deserialize_error_named_handle_stringified(self, generic_type_str: &str) -> std::string::String {
        match self.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            FromOrTryFrom::From => std::string::String::from(""),
            FromOrTryFrom::TryFrom => format!("{}{}{}", self.get_inner_type_handle_stringified(generic_type_str), naming::WithSerializeDeserializeUpperCamelCase, naming::ErrorNamedUpperCamelCase),
        }
    }
    fn get_where_with_serialize_deserialize_error_named_stringified(self, generic_type_str: &str) -> std::string::String {
        add_path(&match self.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            FromOrTryFrom::From => std::string::String::from(""),
            FromOrTryFrom::TryFrom => format!(
                "{}{}{}{}",
                naming::WhereUpperCamelCase,
                self.get_inner_type_handle_stringified(generic_type_str),
                naming::WithSerializeDeserializeUpperCamelCase,
                naming::ErrorNamedUpperCamelCase
            ),
        })
    }
    const fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(self) -> FromOrTryFrom {
        match self {
            Self::StdPrimitiveBool
            | Self::StdOptionOptionStdPrimitiveBool
            | Self::StdPrimitiveI16
            | Self::StdOptionOptionStdPrimitiveI16
            | Self::StdPrimitiveI32
            | Self::StdOptionOptionStdPrimitiveI32
            | Self::StdPrimitiveI64
            | Self::StdOptionOptionStdPrimitiveI64
            | Self::StdPrimitiveF32
            | Self::StdOptionOptionStdPrimitiveF32
            | Self::StdPrimitiveF64
            | Self::StdOptionOptionStdPrimitiveF64
            | Self::StdStringString
            | Self::StdOptionOptionStdStringString
            | Self::StdVecVecStdPrimitiveU8
            | Self::StdOptionOptionStdVecVecStdPrimitiveU8
            | Self::SqlxPostgresTypesPgInterval
            | Self::StdOptionOptionSqlxPostgresTypesPgInterval
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI64
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI32
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
            | Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal
            | Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal
            | Self::SqlxPostgresTypesPgMoney
            | Self::StdOptionOptionSqlxPostgresTypesPgMoney
            | Self::SqlxPostgresTypesPgCiText
            | Self::StdOptionOptionSqlxPostgresTypesPgCiText
            | Self::SqlxTypesBigDecimal
            | Self::StdOptionOptionSqlxTypesBigDecimal
            | Self::SqlxTypesDecimal
            | Self::StdOptionOptionSqlxTypesDecimal
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal
            | Self::SqlxTypesChronoNaiveDateTime
            | Self::StdOptionOptionSqlxTypesChronoNaiveDateTime
            | Self::SqlxTypesChronoNaiveDate
            | Self::StdOptionOptionSqlxTypesChronoNaiveDate
            | Self::SqlxTypesChronoNaiveTime
            | Self::StdOptionOptionSqlxTypesChronoNaiveTime
            | Self::SqlxTypesIpnetworkIpNetwork
            | Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork
            | Self::StdNetIpAddr
            | Self::StdOptionOptionStdNetIpAddr
            | Self::SqlxTypesMacAddressMacAddress
            | Self::StdOptionOptionSqlxTypesMacAddressMacAddress
            | Self::SqlxTypesBitVec
            | Self::StdOptionOptionSqlxTypesBitVec
            | Self::SqlxTypesJson
            | Self::StdOptionOptionSqlxTypesJson
            | Self::SerdeJsonValue
            | Self::StdOptionOptionSerdeJsonValue => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate
            | Self::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate
            | Self::SqlxPostgresTypesPgTimeTz
            | Self::StdOptionOptionSqlxPostgresTypesPgTimeTz
            | Self::SqlxTypesTimePrimitiveDateTime
            | Self::StdOptionOptionSqlxTypesTimePrimitiveDateTime
            | Self::SqlxTypesTimeOffsetDateTime
            | Self::StdOptionOptionSqlxTypesTimeOffsetDateTime
            | Self::SqlxTypesTimeDate
            | Self::StdOptionOptionSqlxTypesTimeDate
            | Self::SqlxTypesTimeTime
            | Self::StdOptionOptionSqlxTypesTimeTime
            | Self::SqlxTypesUuidUuid
            | Self::StdOptionOptionSqlxTypesUuidUuid => FromOrTryFrom::TryFrom,
        }
    }
}

impl std::convert::From<&RustSqlxMapToPostgresTypeVariant> for SupportedSqlxPostgresType {
    fn from(value: &RustSqlxMapToPostgresTypeVariant) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Self::StdOptionOptionStdPrimitiveBool,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => Self::StdPrimitiveBool,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Self::StdOptionOptionStdPrimitiveI16,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => Self::StdPrimitiveI16,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Self::StdOptionOptionStdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => Self::StdPrimitiveI32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Self::StdOptionOptionStdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => Self::StdPrimitiveI64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Self::StdOptionOptionStdPrimitiveF32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => Self::StdPrimitiveF32,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Self::StdOptionOptionStdPrimitiveF64,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => Self::StdPrimitiveF64,

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => {
                Self::StdOptionOptionStdStringString
            }

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => Self::StdStringString,

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
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull | RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Self::SqlxTypesUuidUuid,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::StdOptionOptionSqlxTypesIpnetworkIpNetwork,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => Self::SqlxTypesIpnetworkIpNetwork,

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Self::StdOptionOptionStdNetIpAddr,

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => Self::StdNetIpAddr,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::StdOptionOptionSqlxTypesMacAddressMacAddress,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => Self::SqlxTypesMacAddressMacAddress,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Self::StdOptionOptionSqlxTypesBitVec,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => Self::SqlxTypesBitVec,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJson | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonB => Self::StdOptionOptionSqlxTypesJson,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonNotNull | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonBNotNull => Self::SqlxTypesJson,

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::StdOptionOptionSerdeJsonValue,

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Self::SerdeJsonValue,
        }
    }
}

//todo maybe use it as type for struct field but with inner type like StdPrimitiveBoolAsPostgresqlBool(StdPrimitiveBool)
#[derive(Debug, PartialEq, Eq, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
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
    SqlxTypesJsonAsPostgresqlJson,
    SqlxTypesJsonAsPostgresqlJsonNotNull,
    SqlxTypesJsonAsPostgresqlJsonB,
    SqlxTypesJsonAsPostgresqlJsonBNotNull,

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
    SqlxTypesJsonAsPostgresqlJson,
    SqlxTypesJsonAsPostgresqlJsonB,

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

            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesJsonAsPostgresqlJson => Self::SqlxTypesJsonAsPostgresqlJson,
            RustSqlxMapToPostgresTypeVariantNullable::SqlxTypesJsonAsPostgresqlJsonB => Self::SqlxTypesJsonAsPostgresqlJsonB,

            RustSqlxMapToPostgresTypeVariantNullable::SerdeJsonValueAsPostgresqlJson => Self::SerdeJsonValueAsPostgresqlJson,
            RustSqlxMapToPostgresTypeVariantNullable::SerdeJsonValueAsPostgresqlJsonB => Self::SerdeJsonValueAsPostgresqlJsonB,
        }
    }
}

impl std::convert::TryFrom<&RustSqlxMapToPostgresTypeVariant> for RustSqlxMapToPostgresTypeVariantNullable {
    type Error = ();
    fn try_from(value: &RustSqlxMapToPostgresTypeVariant) -> Result<Self, Self::Error> {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull
            | RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull
            | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull
            | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonBNotNull
            | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull
            | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Err(()),
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

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJson => Ok(Self::SqlxTypesJsonAsPostgresqlJson),

            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonB => Ok(Self::SqlxTypesJsonAsPostgresqlJsonB),

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Ok(Self::SerdeJsonValueAsPostgresqlJson),

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Ok(Self::SerdeJsonValueAsPostgresqlJsonB),
        }
    }
}

impl std::convert::From<&RustSqlxMapToPostgresTypeVariant> for PostgresqlTypeWithMetadata {
    fn from(value: &RustSqlxMapToPostgresTypeVariant) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Self::Bool,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => Self::BoolNotNull,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => Self::SmallInt,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull => Self::SmallIntNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => Self::SmallSerial,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => Self::SmallSerialNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Self::Int2,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => Self::Int2NotNull,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => Self::Int,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull => Self::IntNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => Self::Serial,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull => Self::SerialNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Self::Int4,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => Self::Int4NotNull,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => Self::BigInt,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull => Self::BigIntNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => Self::BigSerial,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull => Self::BigSerialNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => Self::BigSerialNotNullPrimaryKey,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Self::Int8,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => Self::Int8NotNull,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => Self::Real,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull => Self::RealNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Self::Float4,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => Self::Float4NotNull,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => Self::DoublePrecision,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => Self::DoublePrecisionNotNull,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Self::Float8,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => Self::Float8NotNull,

            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => Self::Varchar,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull => Self::VarcharNotNull,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => Self::CharN,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull => Self::CharNNotNull,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => Self::Text,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull => Self::TextNotNull,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Self::CiText,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => Self::CiTextNotNull,

            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Self::Bytea,
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => Self::ByteaNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Self::Interval,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => Self::IntervalNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Self::Int8Range,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => Self::Int8RangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Self::Int4Range,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => Self::Int4RangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => Self::TsTzRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => Self::TsTzRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Self::TsTzRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => Self::TsTzRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Self::TsTzRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => Self::TsTzRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => Self::TsRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => Self::TsRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Self::TsRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => Self::TsRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Self::DateRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => Self::DateRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Self::DateRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => Self::DateRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Self::NumRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => Self::NumRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Self::NumRange,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => Self::NumRangeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Self::Money,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => Self::MoneyNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Self::CiText,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => Self::CiTextNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::Numeric,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => Self::NumericNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Self::Numeric,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull => Self::NumericNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Self::TimestampTz,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => Self::TimestampTzNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::TimestampTz,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => Self::TimestampTzNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::Timestamp,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => Self::TimestampNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::Date,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => Self::DateNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Self::Time,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => Self::TimeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Self::TimeTz,
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => Self::TimeTzNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::Timestamp,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => Self::TimestampNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Self::TimestampTz,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => Self::TimestampTzNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Self::Date,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull => Self::DateNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Self::Time,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => Self::TimeNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Self::Uuid,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => Self::UuidNotNull,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Self::UuidNotNullPrimaryKey, //todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough

            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Self::Inet,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => Self::InetNotNull,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::Cidr,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => Self::CidrNotNull,

            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => Self::Inet,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull => Self::InetNotNull,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Self::Cidr,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => Self::CidrNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::MacAddr,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => Self::MacAddrNotNull,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => Self::Bit,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull => Self::BitNotNull,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Self::VarBit,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => Self::VarBitNotNull,

            // RustSqlxMapToPostgresTypeVariant:://todo what to do with generic?
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJson => Self::Json,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonNotNull => Self::JsonNotNull,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonB => Self::JsonB,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonBNotNull => Self::JsonBNotNull,

            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Self::Json,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull => Self::JsonNotNull,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::JsonB,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Self::JsonBNotNull,
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
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8
            | RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText
            | RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull
            | RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea
            | RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz
            | RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull
            | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet
            | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull
            | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr
            | RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJson
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonNotNull
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonB
            | RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonAsPostgresqlJsonBNotNull
            | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson
            | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull
            | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB
            | RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Err(()),

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
pub enum RustSqlxMapToPostgresPrimiryKeyTypeVariant {
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,
}
impl RustSqlxMapToPostgresPrimiryKeyTypeVariant {
    pub fn to_sqlx_postgres_type(&self) -> SqlxPostgresType {
        match self {
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => SqlxPostgresType::StdPrimitiveI64,
            Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => SqlxPostgresType::SqlxTypesUuidUuid,
        }
    }
}

impl RustSqlxMapToPostgresTypeVariant {
    pub fn get_path_stringified(&self) -> std::string::String {
        add_path(&self.to_string())
    }
    //todo maybe move to generate_postgresql_crud macro
    pub fn get_original_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_original_type_stringified(generic_type_str)
    }
    pub fn get_inner_type_handle_stringified(self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(&self).get_inner_type_handle_stringified(generic_type_str)
    }
    pub fn get_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&self.get_inner_type_handle_stringified(generic_type_str))
    }
    pub fn get_inner_type_with_serialize_deserialize_error_named_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&SupportedSqlxPostgresType::from(self).get_inner_type_with_serialize_deserialize_error_named_handle_stringified(generic_type_str))
    }
    pub fn get_inner_type_with_serialize_deserialize_error_named_without_option_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&SupportedSqlxPostgresType::from(&SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(&SupportedSqlxPostgresType::from(self))).get_inner_type_with_serialize_deserialize_error_named_handle_stringified(generic_type_str))
    }
    pub fn get_where_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
        add_path(&format!("{}{}", naming::WhereUpperCamelCase, self.get_inner_type_handle_stringified(generic_type_str)))
    }
    pub fn get_where_with_serialize_deserialize_error_named_stringified(&self, generic_type_str: &str) -> std::string::String {
        SupportedSqlxPostgresType::from(self).get_where_with_serialize_deserialize_error_named_stringified(generic_type_str)
    }
    pub fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(&self) -> FromOrTryFrom {
        SupportedSqlxPostgresType::from(self).inner_type_from_or_try_from_inner_type_with_serialize_deserialize()
    }
    pub fn get_supported_sqlx_postgres_type(&self) -> SupportedSqlxPostgresType {
        SupportedSqlxPostgresType::from(self)
    }
    pub fn try_convert_into_rust_sqlx_map_to_postgres_primiry_key_type_variant(&self) -> Result<RustSqlxMapToPostgresPrimiryKeyTypeVariant, ()> {
        match self {
            Self::StdPrimitiveBoolAsPostgresqlBool
            | Self::StdPrimitiveBoolAsPostgresqlBoolNotNull
            | Self::StdPrimitiveI16AsPostgresqlSmallInt
            | Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull
            | Self::StdPrimitiveI16AsPostgresqlSmallSerial
            | Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull
            | Self::StdPrimitiveI16AsPostgresqlInt2
            | Self::StdPrimitiveI16AsPostgresqlInt2NotNull
            | Self::StdPrimitiveI32AsPostgresqlInt
            | Self::StdPrimitiveI32AsPostgresqlIntNotNull
            | Self::StdPrimitiveI32AsPostgresqlSerial
            | Self::StdPrimitiveI32AsPostgresqlSerialNotNull
            | Self::StdPrimitiveI32AsPostgresqlInt4
            | Self::StdPrimitiveI32AsPostgresqlInt4NotNull
            | Self::StdPrimitiveI64AsPostgresqlBigInt
            | Self::StdPrimitiveI64AsPostgresqlBigIntNotNull
            | Self::StdPrimitiveI64AsPostgresqlBigSerial
            | Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull
            | Self::StdPrimitiveI64AsPostgresqlInt8
            | Self::StdPrimitiveI64AsPostgresqlInt8NotNull
            | Self::StdPrimitiveF32AsPostgresqlReal
            | Self::StdPrimitiveF32AsPostgresqlRealNotNull
            | Self::StdPrimitiveF32AsPostgresqlFloat4
            | Self::StdPrimitiveF32AsPostgresqlFloat4NotNull
            | Self::StdPrimitiveF64AsPostgresqlDoublePrecision
            | Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull
            | Self::StdPrimitiveF64AsPostgresqlFloat8
            | Self::StdPrimitiveF64AsPostgresqlFloat8NotNull
            | Self::StdStringStringAsPostgresqlVarchar
            | Self::StdStringStringAsPostgresqlVarcharNotNull
            | Self::StdStringStringAsPostgresqlCharN
            | Self::StdStringStringAsPostgresqlCharNNotNull
            | Self::StdStringStringAsPostgresqlText
            | Self::StdStringStringAsPostgresqlTextNotNull
            | Self::StdStringStringAsPostgresqlCiText
            | Self::StdStringStringAsPostgresqlCiTextNotNull
            | Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea
            | Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull
            | Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval
            | Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range
            | Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull
            | Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange
            | Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull
            | Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney
            | Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull
            | Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText
            | Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull
            | Self::SqlxTypesBigDecimalAsPostgresqlNumeric
            | Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull
            | Self::SqlxTypesDecimalAsPostgresqlNumeric
            | Self::SqlxTypesDecimalAsPostgresqlNumericNotNull
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz
            | Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull
            | Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp
            | Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull
            | Self::SqlxTypesChronoNaiveDateAsPostgresqlDate
            | Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull
            | Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime
            | Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull
            | Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz
            | Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull
            | Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp
            | Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull
            | Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz
            | Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull
            | Self::SqlxTypesTimeDateAsPostgresqlDate
            | Self::SqlxTypesTimeDateAsPostgresqlDateNotNull
            | Self::SqlxTypesTimeTimeAsPostgresqlTime
            | Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull
            | Self::SqlxTypesUuidUuidAsPostgresqlUuid
            | Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull
            | Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet
            | Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull
            | Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr
            | Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull
            | Self::StdNetIpAddrAsPostgresqlInet
            | Self::StdNetIpAddrAsPostgresqlInetNotNull
            | Self::StdNetIpAddrAsPostgresqlCidr
            | Self::StdNetIpAddrAsPostgresqlCidrNotNull
            | Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr
            | Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull
            | Self::SqlxTypesBitVecAsPostgresqlBit
            | Self::SqlxTypesBitVecAsPostgresqlBitNotNull
            | Self::SqlxTypesBitVecAsPostgresqlVarBit
            | Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull
            | Self::SqlxTypesJsonAsPostgresqlJson
            | Self::SqlxTypesJsonAsPostgresqlJsonNotNull
            | Self::SqlxTypesJsonAsPostgresqlJsonB
            | Self::SqlxTypesJsonAsPostgresqlJsonBNotNull
            | Self::SerdeJsonValueAsPostgresqlJson
            | Self::SerdeJsonValueAsPostgresqlJsonNotNull
            | Self::SerdeJsonValueAsPostgresqlJsonB
            | Self::SerdeJsonValueAsPostgresqlJsonBNotNull => Err(()),

            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => Ok(RustSqlxMapToPostgresPrimiryKeyTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
            Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Ok(RustSqlxMapToPostgresPrimiryKeyTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey),
        }
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
            "SqlxTypesJsonAsPostgresqlJson" => Ok(Self::SqlxTypesJsonAsPostgresqlJson),
            "SqlxTypesJsonAsPostgresqlJsonNotNull" => Ok(Self::SqlxTypesJsonAsPostgresqlJsonNotNull),
            "SqlxTypesJsonAsPostgresqlJsonB" => Ok(Self::SqlxTypesJsonAsPostgresqlJsonB),
            "SqlxTypesJsonAsPostgresqlJsonBNotNull" => Ok(Self::SqlxTypesJsonAsPostgresqlJsonBNotNull),

            "SerdeJsonValueAsPostgresqlJson" => Ok(Self::SerdeJsonValueAsPostgresqlJson),
            "SerdeJsonValueAsPostgresqlJsonNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonNotNull),
            "SerdeJsonValueAsPostgresqlJsonB" => Ok(Self::SerdeJsonValueAsPostgresqlJsonB),
            "SerdeJsonValueAsPostgresqlJsonBNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonBNotNull),
            _ => Err(format!("unsupported value: {value}, {:?}", Self::into_array().into_iter().map(|element| element.to_string()).collect::<std::vec::Vec<std::string::String>>())),
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
//             "SqlxTypesJsonAsPostgresqlJson" => Ok(Self::SqlxTypesJsonAsPostgresqlJson),
//             "SqlxTypesJsonAsPostgresqlJsonNotNull" => Ok(Self::SqlxTypesJsonAsPostgresqlJsonNotNull),
//             "SqlxTypesJsonAsPostgresqlJsonB" => Ok(Self::SqlxTypesJsonAsPostgresqlJsonB),
//             "SqlxTypesJsonAsPostgresqlJsonBNotNull" => Ok(Self::SqlxTypesJsonAsPostgresqlJsonBNotNull),

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
#[derive(Debug, Clone, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
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

// #[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
// pub struct StdVecVecStdPrimitiveU8AsPostgresqlBytea(pub StdOptionOptionStdVecVecStdPrimitiveU8);
// #[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
// pub struct StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull(pub StdVecVecStdPrimitiveU8);

#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlInterval(pub StdOptionOptionSqlxPostgresTypesPgInterval);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull(pub SqlxPostgresTypesPgInterval);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range(pub StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull(pub SqlxPostgresTypesPgRangeStdPrimitiveI64);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range(pub StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull(pub SqlxPostgresTypesPgRangeStdPrimitiveI32);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesTimeDate);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange(pub StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesDecimal);
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
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz(pub StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull(pub SqlxTypesChronoDateTimeSqlxTypesChronoUtc);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz(pub StdOptionOptionSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull(pub SqlxTypesChronoDateTimeSqlxTypesChronoLocal);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp(pub StdOptionOptionSqlxTypesChronoNaiveDateTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull(pub SqlxTypesChronoNaiveDateTime);
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
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull(pub SqlxTypesTimePrimitiveDateTime);
#[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz(pub StdOptionOptionSqlxTypesTimeOffsetDateTime);
#[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull(pub SqlxTypesTimeOffsetDateTime);
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
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull(pub SqlxTypesMacAddressMacAddress);
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
pub struct SqlxTypesJsonAsPostgresqlJson<T>(pub StdOptionOptionSqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonAsPostgresqlJson<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonAsPostgresqlJsonNotNull<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonAsPostgresqlJsonNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonAsPostgresqlJsonB<T>(pub StdOptionOptionSqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonAsPostgresqlJsonB<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonAsPostgresqlJsonBNotNull<T>(pub SqlxTypesJson<T>);
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonAsPostgresqlJsonBNotNull<T> {
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
pub struct Test
// <T>
{
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
    // std_vec_vec_std_primitive_u8: std::vec::Vec<std::primitive::u8>, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    sqlx_postgres_types_pg_interval: sqlx::postgres::types::PgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    sqlx_postgres_types_pg_range_std_primitive_i64: sqlx::postgres::types::PgRange<std::primitive::i64>, //INT8RANGE
    sqlx_postgres_types_pg_range_std_primitive_i32: sqlx::postgres::types::PgRange<std::primitive::i32>, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,                                        //TSTZRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>,     //TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>, //TSRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>, //DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_date: sqlx::postgres::types::PgRange<sqlx::types::time::Date>,                //DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>, //NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_decimal: sqlx::postgres::types::PgRange<sqlx::types::Decimal>,        //NUMRANGE
    sqlx_postgres_types_pg_money: sqlx::postgres::types::PgMoney,                                                 //MONEY
    // sqlx_postgres_types_pg_l_tree: sqlx::postgres::types::PgLTree,//LTREE//dont want to support that for postgresql_crud
    // sqlx_postgres_types_pg_l_query: sqlx::postgres::types::PgLQuery,//LQUERY//dont want to support that for postgresql_crud
    sqlx_postgres_types_pg_ci_text: sqlx::postgres::types::PgCiText,                                                //CITEXT
    sqlx_types_big_decimal: sqlx::types::BigDecimal,                                                                //NUMERIC
    sqlx_types_decimal: sqlx::types::Decimal,                                                                       //NUMERIC
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,     //TIMESTAMP
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>, //TIMESTAMPTZ
    sqlx_types_chrono_naive_date_time: sqlx::types::chrono::NaiveDateTime,                                          //TIMESTAMP
    sqlx_types_chrono_naive_date: sqlx::types::chrono::NaiveDate,                                                   //DATE
    sqlx_types_chrono_naive_time: sqlx::types::chrono::NaiveTime,                                                   //TIME
    sqlx_postgres_types_pg_time_tz: sqlx::postgres::types::PgTimeTz,                                                //just present chrono or time flag
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    sqlx_types_time_primitive_date_time: sqlx::types::time::PrimitiveDateTime, //TIMESTAMP
    sqlx_types_time_offset_date_time: sqlx::types::time::OffsetDateTime,       //TIMESTAMPTZ
    sqlx_types_time_date: sqlx::types::time::Date,                             //DATE
    sqlx_types_time_time: sqlx::types::time::Time,                             //TIME
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    sqlx_types_uuid_uuid: sqlx::types::uuid::Uuid,                            //UUID
    sqlx_types_ipnetwork_ip_network: sqlx::types::ipnetwork::IpNetwork,       //INET, CIDR
    std_net_ip_addr: std::net::IpAddr,                                        //INET, CIDR
    sqlx_types_mac_address_mac_address: sqlx::types::mac_address::MacAddress, //MACADDR
    sqlx_types_bit_vec: sqlx::types::BitVec,                                  //BIT, VARBIT
    // sqlx_types_json: sqlx::types::Json<T>,         //JSON, JSONB
    serde_json_value: serde_json::Value, //JSON, JSONB
                                         // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
                                         //maybe Composite types
                                         //maybe Enumerations
}
#[allow(dead_code)]
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

// #[derive(Debug)]
// pub struct TestNewTypeWithAdditionalInfo<T> {
//     _std_primitive_bool_as_postgresql_bool: StdPrimitiveBoolAsPostgresqlBool,
//     _std_primitive_bool_as_postgresql_bool_not_null: StdPrimitiveBoolAsPostgresqlBoolNotNull,

//     _std_primitive_i16_as_postgresql_small_int: StdPrimitiveI16AsPostgresqlSmallInt,
//     _std_primitive_i16_as_postgresql_small_int_not_null: StdPrimitiveI16AsPostgresqlSmallIntNotNull,
//     _std_primitive_i16_as_postgresql_small_serial: StdPrimitiveI16AsPostgresqlSmallSerial,
//     _std_primitive_i16_as_postgresql_small_serial_not_null: StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
//     _std_primitive_i16_as_postgresql_small_int2: StdPrimitiveI16AsPostgresqlInt2,
//     _std_primitive_i16_as_postgresql_small_int2_not_null: StdPrimitiveI16AsPostgresqlInt2NotNull,

//     _std_primitive_i32_as_postgresql_int: StdPrimitiveI32AsPostgresqlInt,
//     _std_primitive_i32_as_postgresql_int_not_null: StdPrimitiveI32AsPostgresqlIntNotNull,
//     _std_primitive_i32_as_postgresql_serial: StdPrimitiveI32AsPostgresqlSerial,
//     _std_primitive_i32_as_postgresql_serial_not_null: StdPrimitiveI32AsPostgresqlSerialNotNull,
//     _std_primitive_i32_as_postgresql_int4: StdPrimitiveI32AsPostgresqlInt4,
//     _std_primitive_i32_as_postgresql_int4_not_null: StdPrimitiveI32AsPostgresqlInt4NotNull,

//     _std_primitive_i64_as_postgresql_big_int: StdPrimitiveI64AsPostgresqlBigInt,
//     _std_primitive_i64_as_postgresql_big_int_not_null: StdPrimitiveI64AsPostgresqlBigIntNotNull,
//     _std_primitive_i64_as_postgresql_big_serial: StdPrimitiveI64AsPostgresqlBigSerial,
//     _std_primitive_i64_as_postgresql_big_serial_not_null: StdPrimitiveI64AsPostgresqlBigSerialNotNull,
//     _std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     _std_primitive_i64_as_postgresql_big_int8: StdPrimitiveI64AsPostgresqlInt8,
//     _std_primitive_i64_as_postgresql_big_int8_not_null: StdPrimitiveI64AsPostgresqlInt8NotNull,

//     _std_primitive_f32_as_postgresql_real: StdPrimitiveF32AsPostgresqlReal,
//     _std_primitive_f32_as_postgresql_real_not_null: StdPrimitiveF32AsPostgresqlRealNotNull,
//     _std_primitive_f32_as_postgresql_float4: StdPrimitiveF32AsPostgresqlFloat4,
//     _std_primitive_f32_as_postgresql_float4_not_null: StdPrimitiveF32AsPostgresqlFloat4NotNull,

//     _std_primitive_f64_as_postgresql_double_precision: StdPrimitiveF64AsPostgresqlDoublePrecision,
//     _std_primitive_f64_as_postgresql_double_precision_not_null: StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
//     _std_primitive_f64_as_postgresql_float8: StdPrimitiveF64AsPostgresqlFloat8,
//     _std_primitive_f64_as_postgresql_float8_not_null: StdPrimitiveF64AsPostgresqlFloat8NotNull,

//     _std_string_string_as_postgresql_varchar: StdStringStringAsPostgresqlVarchar,
//     _std_string_string_as_postgresql_varchar_not_null: StdStringStringAsPostgresqlVarcharNotNull,
//     _std_string_string_as_postgresql_char_n: StdStringStringAsPostgresqlCharN,
//     _std_string_string_as_postgresql_char_n_not_null: StdStringStringAsPostgresqlCharNNotNull,
//     _std_string_string_as_postgresql_text: StdStringStringAsPostgresqlText,
//     _std_string_string_as_postgresql_text_not_null: StdStringStringAsPostgresqlTextNotNull,
//     _std_string_string_as_postgresql_ci_text: StdStringStringAsPostgresqlCiText,
//     _std_string_string_as_postgresql_ci_text_not_null: StdStringStringAsPostgresqlCiTextNotNull,

//     _std_vec_vec_std_primitive_u8_as_postgresql_bytea: StdVecVecStdPrimitiveU8AsPostgresqlBytea,
//     _std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

//     _sqlx_postgres_types_pg_interval_as_postgresql_interval: SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
//     _sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

//     _sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
//     _sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

//     _sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
//     _sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

//     _sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
//     _sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

//     _sqlx_postgres_types_pg_money_as_postgresql_money: SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
//     _sqlx_postgres_types_pg_money_as_postgresql_money_not_null: SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

//     _sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
//     _sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

//     _sqlx_types_big_decimal_as_postgresql_numeric: SqlxTypesBigDecimalAsPostgresqlNumeric,
//     _sqlx_types_big_decimal_as_postgresql_numeric_not_null: SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

//     _sqlx_types_decimal_as_postgresql_numeric: SqlxTypesDecimalAsPostgresqlNumeric,
//     _sqlx_types_decimal_as_postgresql_numeric_not_null: SqlxTypesDecimalAsPostgresqlNumericNotNull,

//     _sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
//     _sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

//     _sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
//     _sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

//     _sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
//     _sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

//     _sqlx_types_chrono_naive_date_as_postgresql_date: SqlxTypesChronoNaiveDateAsPostgresqlDate,
//     _sqlx_types_chrono_naive_date_as_postgresql_date_not_null: SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

//     _sqlx_types_chrono_naive_time_as_postgresql_time: SqlxTypesChronoNaiveTimeAsPostgresqlTime,
//     _sqlx_types_chrono_naive_time_as_postgresql_time_not_null: SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

//     _sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
//     _sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

//     _sqlx_types_time_primitive_date_time_as_postgresql_timestamp: SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
//     _sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

//     _sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
//     _sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

//     _sqlx_types_time_date_as_postgresql_date: SqlxTypesTimeDateAsPostgresqlDate,
//     _sqlx_types_time_date_as_postgresql_date_not_null: SqlxTypesTimeDateAsPostgresqlDateNotNull,

//     _sqlx_types_time_time_as_postgresql_time: SqlxTypesTimeTimeAsPostgresqlTime,
//     _sqlx_types_time_time_as_postgresql_time_not_null: SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

//     _sqlx_types_uuid_uuida_as_postgresql_uuid: SqlxTypesUuidUuidAsPostgresqlUuid,
//     _sqlx_types_uuid_uuida_as_postgresql_uuid_not_null: SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
//     _sqlx_types_uuid_uuida_as_postgresql_uuid_not_null_primary_key: SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough

//     _sqlx_types_ipnetwork_ip_network_as_postgresql_inet: SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
//     _sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
//     _sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
//     _sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

//     _std_net_ip_addr_as_postgresql_inet: StdNetIpAddrAsPostgresqlInet,
//     _std_net_ip_addr_as_postgresql_inet_not_null: StdNetIpAddrAsPostgresqlInetNotNull,
//     _std_net_ip_addr_as_postgresql_cidr: StdNetIpAddrAsPostgresqlCidr,
//     _std_net_ip_addr_as_postgresql_cidr_not_null: StdNetIpAddrAsPostgresqlCidrNotNull,

//     _sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
//     _sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

//     _sqlx_types_bit_vec_as_postgresql_bit: SqlxTypesBitVecAsPostgresqlBit,
//     _sqlx_types_bit_vec_as_postgresql_bit_not_null: SqlxTypesBitVecAsPostgresqlBitNotNull,
//     _sqlx_types_bit_vec_as_postgresql_var_bit: SqlxTypesBitVecAsPostgresqlVarBit,
//     _sqlx_types_bit_vec_as_postgresql_var_bit_not_null: SqlxTypesBitVecAsPostgresqlVarBitNotNull,

//     //todo what to do with generic?
//     _sqlx_types_json_t_as_postgresql_json: SqlxTypesJsonAsPostgresqlJson<T>,
//     _sqlx_types_json_t_as_postgresql_json_not_null: SqlxTypesJsonAsPostgresqlJsonNotNull<T>,
//     _sqlx_types_json_t_as_postgresql_json_b: SqlxTypesJsonAsPostgresqlJsonB<T>,
//     _sqlx_types_json_t_as_postgresql_json_b_not_null: SqlxTypesJsonAsPostgresqlJsonBNotNull<T>,

//     _serde_json_value_as_postgresql_json: SerdeJsonValueAsPostgresqlJson,
//     _serde_json_value_as_postgresql_json_not_null: SerdeJsonValueAsPostgresqlJsonNotNull,
//     _serde_json_value_as_postgresql_json_b: SerdeJsonValueAsPostgresqlJsonB,
//     _serde_json_value_as_postgresql_json_b_not_null: SerdeJsonValueAsPostgresqlJsonBNotNull,
// }

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
struct TestNewType
// <T>
{
    _std_primitive_bool: StdPrimitiveBool,
    _std_primitive_i16: StdPrimitiveI16,
    _std_primitive_i32: StdPrimitiveI32,
    _std_primitive_i64: StdPrimitiveI64,
    _std_primitive_f32: StdPrimitiveF32,
    _std_primitive_f64: StdPrimitiveF64,
    _std_string_string: StdStringString,
    // _std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8,
    _sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgInterval,
    _sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64,
    _sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    _sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    _sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    _sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    _sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney,
    _sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText,
    _sqlx_types_big_decimal: SqlxTypesBigDecimal,
    _sqlx_types_decimal: SqlxTypesDecimal,
    _sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    _sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    _sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime,
    _sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate,
    _sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime,
    _sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz,
    _sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime,
    _sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime,
    _sqlx_types_time_date: SqlxTypesTimeDate,
    _sqlx_types_time_time: SqlxTypesTimeTime,
    _sqlx_types_uuid_uuid: SqlxTypesUuidUuid,
    _sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork,
    _std_net_ip_addr: StdNetIpAddr,
    _sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress,
    _sqlx_types_bit_vec: SqlxTypesBitVec,
    // _sqlx_types_json: SqlxTypesJson<T>,
    _serde_json_value: SerdeJsonValue,
}

impl
    std::convert::From<
        Test, // <T>
    > for TestNewType
// <T>
{
    fn from(value: Test, // <T>
    ) -> Self {
        Self {
            _std_primitive_bool: StdPrimitiveBool(value.std_primitive_bool),
            _std_primitive_i16: StdPrimitiveI16(value.std_primitive_i16),
            _std_primitive_i32: StdPrimitiveI32(value.std_primitive_i32),
            _std_primitive_i64: StdPrimitiveI64(value.std_primitive_i64),
            _std_primitive_f32: StdPrimitiveF32(value.std_primitive_f32),
            _std_primitive_f64: StdPrimitiveF64(value.std_primitive_f64),
            _std_string_string: StdStringString(value.std_string_string),
            // _std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8(value.std_vec_vec_std_primitive_u8),
            _sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgInterval(value.sqlx_postgres_types_pg_interval),
            _sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64(value.sqlx_postgres_types_pg_range_std_primitive_i64),
            _sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32(value.sqlx_postgres_types_pg_range_std_primitive_i32),
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc),
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local),
            _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(value.sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time),
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time),
            _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(value.sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time),
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date),
            _sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate(value.sqlx_postgres_types_pg_range_sqlx_types_time_date),
            _sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(value.sqlx_postgres_types_pg_range_sqlx_types_big_decimal),
            _sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal(value.sqlx_postgres_types_pg_range_sqlx_types_decimal),
            _sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney(value.sqlx_postgres_types_pg_money),
            _sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText(value.sqlx_postgres_types_pg_ci_text),
            _sqlx_types_big_decimal: SqlxTypesBigDecimal(value.sqlx_types_big_decimal),
            _sqlx_types_decimal: SqlxTypesDecimal(value.sqlx_types_decimal),
            _sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc(value.sqlx_types_chrono_date_time_sqlx_types_chrono_utc),
            _sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal(value.sqlx_types_chrono_date_time_sqlx_types_chrono_local),
            _sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTime(value.sqlx_types_chrono_naive_date_time),
            _sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDate(value.sqlx_types_chrono_naive_date),
            _sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTime(value.sqlx_types_chrono_naive_time),
            _sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTz(value.sqlx_postgres_types_pg_time_tz),
            _sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTime(value.sqlx_types_time_primitive_date_time),
            _sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTime(value.sqlx_types_time_offset_date_time),
            _sqlx_types_time_date: SqlxTypesTimeDate(value.sqlx_types_time_date),
            _sqlx_types_time_time: SqlxTypesTimeTime(value.sqlx_types_time_time),
            _sqlx_types_uuid_uuid: SqlxTypesUuidUuid(value.sqlx_types_uuid_uuid),
            _sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork(value.sqlx_types_ipnetwork_ip_network),
            _std_net_ip_addr: StdNetIpAddr(value.std_net_ip_addr),
            _sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddress(value.sqlx_types_mac_address_mac_address),
            _sqlx_types_bit_vec: SqlxTypesBitVec(value.sqlx_types_bit_vec),
            // _sqlx_types_json: SqlxTypesJson::<T>(value.sqlx_types_json),
            _serde_json_value: SerdeJsonValue(value.serde_json_value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, utoipa::ToSchema)]
pub struct TimeMonth(pub time::Month);
impl serde::Serialize for TimeMonth {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        match self.0 {
            time::Month::January => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 0u32, "January"),
            time::Month::February => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 1u32, "February"),
            time::Month::March => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 2u32, "March"),
            time::Month::April => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 3u32, "April"),
            time::Month::May => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 4u32, "May"),
            time::Month::June => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 5u32, "June"),
            time::Month::July => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 6u32, "July"),
            time::Month::August => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 7u32, "August"),
            time::Month::September => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 8u32, "September"),
            time::Month::October => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 9u32, "October"),
            time::Month::November => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 10u32, "November"),
            time::Month::December => serde::Serializer::serialize_unit_variant(__serializer, "TimeMonth", 11u32, "December"),
        }
    }
}
impl<'de> serde::Deserialize<'de> for TimeMonth {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __field7,
            __field8,
            __field9,
            __field10,
            __field11,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "variant identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    6u64 => serde::__private::Ok(__Field::__field6),
                    7u64 => serde::__private::Ok(__Field::__field7),
                    8u64 => serde::__private::Ok(__Field::__field8),
                    9u64 => serde::__private::Ok(__Field::__field9),
                    10u64 => serde::__private::Ok(__Field::__field10),
                    11u64 => serde::__private::Ok(__Field::__field11),
                    _ => serde::__private::Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(__value), &"variant index 0 <= i < 12")),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "January" => serde::__private::Ok(__Field::__field0),
                    "February" => serde::__private::Ok(__Field::__field1),
                    "March" => serde::__private::Ok(__Field::__field2),
                    "April" => serde::__private::Ok(__Field::__field3),
                    "May" => serde::__private::Ok(__Field::__field4),
                    "June" => serde::__private::Ok(__Field::__field5),
                    "July" => serde::__private::Ok(__Field::__field6),
                    "August" => serde::__private::Ok(__Field::__field7),
                    "September" => serde::__private::Ok(__Field::__field8),
                    "October" => serde::__private::Ok(__Field::__field9),
                    "November" => serde::__private::Ok(__Field::__field10),
                    "December" => serde::__private::Ok(__Field::__field11),
                    _ => serde::__private::Err(serde::de::Error::unknown_variant(__value, VARIANTS)),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"January" => serde::__private::Ok(__Field::__field0),
                    b"February" => serde::__private::Ok(__Field::__field1),
                    b"March" => serde::__private::Ok(__Field::__field2),
                    b"April" => serde::__private::Ok(__Field::__field3),
                    b"May" => serde::__private::Ok(__Field::__field4),
                    b"June" => serde::__private::Ok(__Field::__field5),
                    b"July" => serde::__private::Ok(__Field::__field6),
                    b"August" => serde::__private::Ok(__Field::__field7),
                    b"September" => serde::__private::Ok(__Field::__field8),
                    b"October" => serde::__private::Ok(__Field::__field9),
                    b"November" => serde::__private::Ok(__Field::__field10),
                    b"December" => serde::__private::Ok(__Field::__field11),
                    _ => {
                        let __value = &serde::__private::from_utf8_lossy(__value);
                        serde::__private::Err(serde::de::Error::unknown_variant(__value, VARIANTS))
                    }
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<TimeMonth>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = TimeMonth;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "enum TimeMonth")
            }
            fn visit_enum<__A>(self, __data: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::EnumAccess<'de>,
            {
                match serde::de::EnumAccess::variant(__data)? {
                    (__Field::__field0, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::January))
                    }
                    (__Field::__field1, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::February))
                    }
                    (__Field::__field2, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::March))
                    }
                    (__Field::__field3, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::April))
                    }
                    (__Field::__field4, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::May))
                    }
                    (__Field::__field5, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::June))
                    }
                    (__Field::__field6, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::July))
                    }
                    (__Field::__field7, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::August))
                    }
                    (__Field::__field8, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::September))
                    }
                    (__Field::__field9, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::October))
                    }
                    (__Field::__field10, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::November))
                    }
                    (__Field::__field11, __variant) => {
                        serde::de::VariantAccess::unit_variant(__variant)?;
                        serde::__private::Ok(TimeMonth(time::Month::December))
                    }
                }
            }
        }
        #[doc(hidden)]
        const VARIANTS: &'static [&'static str] = &["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        serde::Deserializer::deserialize_enum(
            __deserializer,
            "TimeMonth",
            VARIANTS,
            __Visitor {
                marker: serde::__private::PhantomData::<TimeMonth>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, utoipa::ToSchema)]
pub struct SqlxTypesTimeUtcOffset(pub sqlx::types::time::UtcOffset);
impl serde::Serialize for SqlxTypesTimeUtcOffset {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesTimeUtcOffset", false as usize + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "hours", &self.0.whole_hours())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "minutes", &self.0.minutes_past_hour())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "seconds", &self.0.seconds_past_minute())?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesTimeUtcOffset {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "hours" => serde::__private::Ok(__Field::__field0),
                    "minutes" => serde::__private::Ok(__Field::__field1),
                    "seconds" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"hours" => serde::__private::Ok(__Field::__field0),
                    b"minutes" => serde::__private::Ok(__Field::__field1),
                    b"seconds" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesTimeUtcOffset>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesTimeUtcOffset;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesTimeUtcOffset")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesTimeUtcOffset with 3 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::i8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesTimeUtcOffset with 3 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::primitive::i8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(2usize, &"struct SqlxTypesTimeUtcOffset with 3 elements"));
                    }
                };
                serde::__private::Ok(SqlxTypesTimeUtcOffset(match sqlx::types::time::UtcOffset::from_hms(__field0, __field1, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::i8> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::i8> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::primitive::i8> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("hours"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i8>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("minutes"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i8>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("seconds"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i8>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("hours")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("minutes")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("seconds")?,
                };
                serde::__private::Ok(SqlxTypesTimeUtcOffset(match sqlx::types::time::UtcOffset::from_hms(__field0, __field1, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["hours", "minutes", "seconds"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesTimeUtcOffset",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesTimeUtcOffset>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, utoipa::ToSchema)]
pub struct NumBigintSign(pub num_bigint::Sign);
impl serde::Serialize for NumBigintSign {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(
            __serializer,
            "NumBigintSign",
            match self.0 {
                num_bigint::Sign::Minus => "Minus",
                num_bigint::Sign::NoSign => "NoSign",
                num_bigint::Sign::Plus => "Plus",
            },
        )
    }
}
impl<'de> serde::Deserialize<'de> for NumBigintSign {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<NumBigintSign>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = NumBigintSign;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct NumBigintSign")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::string::String = <std::string::String as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(NumBigintSign(match __field0.as_str() {
                    "Minus" => num_bigint::Sign::Minus,
                    "NoSign" => num_bigint::Sign::NoSign,
                    "Plus" => num_bigint::Sign::Plus,
                    _ => {
                        return Err(serde::de::Error::custom("unsupported value, supported: Minus, NoSign, Plus"));
                    }
                }))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct NumBigintSign with 1 element"));
                    }
                };
                serde::__private::Ok(NumBigintSign(match __field0.as_str() {
                    "Minus" => num_bigint::Sign::Minus,
                    "NoSign" => num_bigint::Sign::NoSign,
                    "Plus" => num_bigint::Sign::Plus,
                    _ => {
                        return Err(serde::de::Error::custom("unsupported value, supported: Minus, NoSign, Plus"));
                    }
                }))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "NumBigintSign",
            __Visitor {
                marker: serde::__private::PhantomData::<NumBigintSign>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
//todo pub or not for all - think
#[derive(Debug, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct NumBigintBigInt(pub num_bigint::BigInt);
impl serde::Serialize for NumBigintBigInt {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let (sign, digits) = self.0.to_u32_digits();
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "NumBigintBigInt", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "sign", &NumBigintSign(sign))?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "digits", &digits)?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for NumBigintBigInt {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "sign" => serde::__private::Ok(__Field::__field0),
                    "digits" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"sign" => serde::__private::Ok(__Field::__field0),
                    b"digits" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<NumBigintBigInt>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = NumBigintBigInt;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct NumBigintBigInt")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<NumBigintSign>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct NumBigintBigInt with 2 elements")),
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::u32>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct NumBigintBigInt with 2 elements")),
                };
                serde::__private::Ok(NumBigintBigInt(num_bigint::BigInt::new(__field0.0, __field1)))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<NumBigintSign> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::vec::Vec<std::primitive::u32>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("sign"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<NumBigintSign>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("digits"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<std::primitive::u32>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("sign")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("digits")?,
                };
                serde::__private::Ok(NumBigintBigInt(num_bigint::BigInt::new(__field0.0, __field1)))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["sign", "digits"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "NumBigintBigInt",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<NumBigintBigInt>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl Default for TestNewType
// crate::generate_postgresql_json_type::Something>
{
    fn default() -> Self {
        //todo unwraps
        let std_primitive_u8_handle = std::primitive::u8::default();
        let std_primitive_i8_handle = std::primitive::i8::default();
        let std_primitive_u16_handle = std::primitive::u16::default();
        let std_primitive_u32_handle = std::primitive::u32::default();
        let std_primitive_i32_handle = std::primitive::i32::default();
        let std_primitive_i64_handle = std::primitive::i64::default();
        let std_string_string_handle = std::string::String::default();
        let sqlx_types_time_date_handle = sqlx::types::time::Date::from_calendar_date(2024, time::Month::February, 3).unwrap();
        let sqlx_types_time_time_handle = sqlx::types::time::Time::from_hms(1, 1, 1).unwrap();
        let sqlx_types_chrono_naive_date_handle = sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap();
        let sqlx_types_chrono_naive_time_handle = sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap();
        let sqlx_types_chrono_naive_date_time_handle = sqlx::types::chrono::NaiveDateTime::new(sqlx_types_chrono_naive_date_handle, sqlx_types_chrono_naive_time_handle);
        let sqlx_types_time_primitive_date_time_handle = sqlx::types::time::PrimitiveDateTime::new(sqlx_types_time_date_handle, sqlx_types_time_time_handle);
        let sqlx_types_chrono_fixed_offset_handle = sqlx::types::chrono::FixedOffset::west_opt(std_primitive_i32_handle).unwrap();
        let sqlx_types_time_offset_date_time_handle = sqlx::types::time::OffsetDateTime::from_unix_timestamp(std::primitive::i64::default()).unwrap();
        let sqlx_types_decimal_handle = sqlx::types::Decimal::try_new(std_primitive_i64_handle, std_primitive_u32_handle).unwrap();
        let sqlx_types_chrono_utc_handle = sqlx::types::chrono::Utc;
        let sqlx_types_big_decimal_handle = sqlx::types::BigDecimal::new(num_bigint::BigInt::new(num_bigint::Sign::Plus, vec![std_primitive_u32_handle]), std_primitive_i64_handle);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle = sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(sqlx_types_chrono_naive_date_time_handle, sqlx_types_chrono_utc_handle);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle = sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(sqlx_types_chrono_naive_date_time_handle, sqlx_types_chrono_fixed_offset_handle);
        let std_ops_bound_std_primitive_i64_handle = std::ops::Bound::<std::primitive::i64>::Included(std_primitive_i64_handle);
        let std_ops_bound_std_primitive_i32_handle = std::ops::Bound::<std::primitive::i32>::Included(std_primitive_i32_handle);
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle = std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>::Included(sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle);
        let std_ops_bound_sqlx_types_time_primitive_date_time_handle = std::ops::Bound::<sqlx::types::time::PrimitiveDateTime>::Included(sqlx_types_time_primitive_date_time_handle);
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle = std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>::Included(sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle);
        let std_ops_bound_sqlx_types_time_offset_date_time_handle = std::ops::Bound::<sqlx::types::time::OffsetDateTime>::Included(sqlx_types_time_offset_date_time_handle);
        let std_ops_bound_sqlx_types_chrono_naive_date_handle = std::ops::Bound::<sqlx::types::chrono::NaiveDate>::Included(sqlx_types_chrono_naive_date_handle);
        let std_ops_bound_sqlx_types_time_date_handle = std::ops::Bound::<sqlx::types::time::Date>::Included(sqlx_types_time_date_handle);
        let std_ops_bound_sqlx_types_big_decimal_handle = std::ops::Bound::<sqlx::types::BigDecimal>::Included(sqlx_types_big_decimal_handle.clone());
        let std_ops_bound_sqlx_types_decimal_handle = std::ops::Bound::<sqlx::types::Decimal>::Included(sqlx_types_decimal_handle);
        let std_ops_bound_sqlx_types_chrono_naive_date_time_handle = std::ops::Bound::<sqlx::types::chrono::NaiveDateTime>::Included(sqlx_types_chrono_naive_date_time_handle);
        let std_primitive_bool = StdPrimitiveBool(true);
        let std_primitive_i16 = StdPrimitiveI16(std::primitive::i16::default());
        let std_primitive_i32 = StdPrimitiveI32(std_primitive_i32_handle);
        let std_primitive_i64 = StdPrimitiveI64(std_primitive_i64_handle);
        let std_primitive_f32 = StdPrimitiveF32(std::primitive::f32::default());
        let std_primitive_f64 = StdPrimitiveF64(std::primitive::f64::default());
        let std_string_string = StdStringString(std_string_string_handle.clone());
        // let std_vec_vec_std_primitive_u8 =
        //     StdVecVecStdPrimitiveU8(vec![std_primitive_u8_handle]);
        let sqlx_postgres_types_pg_interval = SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval {
            months: std_primitive_i32_handle,
            days: std_primitive_i32_handle,
            microseconds: std_primitive_i64_handle,
        });
        let sqlx_postgres_types_pg_range_std_primitive_i64 = SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange::<std::primitive::i64> {
            start: std_ops_bound_std_primitive_i64_handle,
            end: std_ops_bound_std_primitive_i64_handle,
        });
        let sqlx_postgres_types_pg_range_std_primitive_i32 = SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange::<std::primitive::i32> {
            start: std_ops_bound_std_primitive_i32_handle,
            end: std_ops_bound_std_primitive_i32_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
            start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle,
            end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
            start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle,
            end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time = SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange::<sqlx::types::time::OffsetDateTime> {
            start: std_ops_bound_sqlx_types_time_offset_date_time_handle,
            end: std_ops_bound_sqlx_types_time_offset_date_time_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange::<sqlx::types::chrono::NaiveDateTime> {
            start: std_ops_bound_sqlx_types_chrono_naive_date_time_handle,
            end: std_ops_bound_sqlx_types_chrono_naive_date_time_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time = SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange::<sqlx::types::time::PrimitiveDateTime> {
            start: std_ops_bound_sqlx_types_time_primitive_date_time_handle,
            end: std_ops_bound_sqlx_types_time_primitive_date_time_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange::<sqlx::types::chrono::NaiveDate> {
            start: std_ops_bound_sqlx_types_chrono_naive_date_handle,
            end: std_ops_bound_sqlx_types_chrono_naive_date_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_time_date = SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange::<sqlx::types::time::Date> {
            start: std_ops_bound_sqlx_types_time_date_handle,
            end: std_ops_bound_sqlx_types_time_date_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_big_decimal = SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange::<sqlx::types::BigDecimal> {
            start: std_ops_bound_sqlx_types_big_decimal_handle.clone(),
            end: std_ops_bound_sqlx_types_big_decimal_handle,
        });
        let sqlx_postgres_types_pg_range_sqlx_types_decimal = SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange::<sqlx::types::Decimal> {
            start: std_ops_bound_sqlx_types_decimal_handle,
            end: std_ops_bound_sqlx_types_decimal_handle,
        });
        let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney(std_primitive_i64_handle));
        let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText(sqlx::postgres::types::PgCiText(std_string_string_handle.clone()));
        let sqlx_types_big_decimal = SqlxTypesBigDecimal(sqlx_types_big_decimal_handle);
        let sqlx_types_decimal = SqlxTypesDecimal(sqlx_types_decimal_handle);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(sqlx_types_chrono_naive_date_time_handle, sqlx_types_chrono_utc_handle));
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(sqlx_types_chrono_naive_date_time_handle, sqlx_types_chrono_fixed_offset_handle));
        let sqlx_types_chrono_naive_date_time = SqlxTypesChronoNaiveDateTime(sqlx_types_chrono_naive_date_time_handle);
        let sqlx_types_chrono_naive_date = SqlxTypesChronoNaiveDate(sqlx_types_chrono_naive_date_handle);
        let sqlx_types_chrono_naive_time = SqlxTypesChronoNaiveTime(sqlx_types_chrono_naive_time_handle);
        let sqlx_postgres_types_pg_time_tz = SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz {
            time: sqlx_types_time_time_handle,
            offset: sqlx::types::time::UtcOffset::from_hms(std_primitive_i8_handle, std_primitive_i8_handle, std_primitive_i8_handle).unwrap(),
        });
        let sqlx_types_time_primitive_date_time = SqlxTypesTimePrimitiveDateTime(sqlx_types_time_primitive_date_time_handle);
        let sqlx_types_time_offset_date_time = SqlxTypesTimeOffsetDateTime(sqlx_types_time_offset_date_time_handle);
        let sqlx_types_time_date = SqlxTypesTimeDate(sqlx_types_time_date_handle);
        let sqlx_types_time_time = SqlxTypesTimeTime(sqlx_types_time_time_handle);
        let sqlx_types_uuid_uuid = SqlxTypesUuidUuid(sqlx::types::uuid::Uuid::from_u128(std::primitive::u128::default()));
        let sqlx_types_ipnetwork_ip_network = SqlxTypesIpnetworkIpNetwork(sqlx::types::ipnetwork::IpNetwork::V6(
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
        let sqlx_types_mac_address_mac_address = SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress::new([
            std_primitive_u8_handle,
            std_primitive_u8_handle,
            std_primitive_u8_handle,
            std_primitive_u8_handle,
            std_primitive_u8_handle,
            std_primitive_u8_handle,
        ]));
        let sqlx_types_bit_vec = SqlxTypesBitVec(sqlx::types::BitVec::new());
        // let sqlx_types_json = SqlxTypesJson(sqlx::types::Json(crate::generate_postgresql_json_type::Something {
        //     std_string_string: generate_postgresql_json_type::StdStringString(std_string_string_handle),
        //     std_vec_vec_std_primitive_bool: generate_postgresql_json_type::StdVecVecStdPrimitiveBool(vec![true, false]),
        //     generic: generate_postgresql_json_type::Generic(crate::generate_postgresql_json_type::Doggie {
        //         std_string_string: generate_postgresql_json_type::StdStringString(std::string::String::from("gav"))
        //     }),
        //     std_option_option_generic: generate_postgresql_json_type::StdOptionOptionGeneric(Some(crate::generate_postgresql_json_type::Doggie {
        //         std_string_string: generate_postgresql_json_type::StdStringString(std::string::String::from("gav"))
        //     })),
        //     std_vec_vec_generic: generate_postgresql_json_type::StdVecVecGeneric(vec![crate::generate_postgresql_json_type::Doggie {
        //         std_string_string: generate_postgresql_json_type::StdStringString(std::string::String::from("gav"))
        //     }]),
        //     std_option_option_std_vec_vec_generic: generate_postgresql_json_type::StdOptionOptionStdVecVecGeneric(Some(vec![crate::generate_postgresql_json_type::Doggie {
        //         std_string_string: generate_postgresql_json_type::StdStringString(std::string::String::from("gav"))
        //     }])),
        //     std_vec_vec_std_option_option_generic: generate_postgresql_json_type::StdVecVecStdOptionOptionGeneric(vec![Some(crate::generate_postgresql_json_type::Doggie {
        //         std_string_string: generate_postgresql_json_type::StdStringString(std::string::String::from("gav"))
        //     })]),
        //     std_option_option_std_vec_vec_std_option_option_generic: generate_postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionGeneric(Some(vec![Some(crate::generate_postgresql_json_type::Doggie {
        //         std_string_string: generate_postgresql_json_type::StdStringString(std::string::String::from("gav"))
        //     })])),
        // }));
        let serde_json_value = SerdeJsonValue(serde_json::Value::Bool(std::primitive::bool::default()));
        Self {
            _std_primitive_bool: std_primitive_bool,
            _std_primitive_i16: std_primitive_i16,
            _std_primitive_i32: std_primitive_i32,
            _std_primitive_i64: std_primitive_i64,
            _std_primitive_f32: std_primitive_f32,
            _std_primitive_f64: std_primitive_f64,
            _std_string_string: std_string_string,
            // _std_vec_vec_std_primitive_u8: std_vec_vec_std_primitive_u8,
            _sqlx_postgres_types_pg_interval: sqlx_postgres_types_pg_interval,
            _sqlx_postgres_types_pg_range_std_primitive_i64: sqlx_postgres_types_pg_range_std_primitive_i64,
            _sqlx_postgres_types_pg_range_std_primitive_i32: sqlx_postgres_types_pg_range_std_primitive_i32,
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            _sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time,
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time,
            _sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time,
            _sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date,
            _sqlx_postgres_types_pg_range_sqlx_types_time_date: sqlx_postgres_types_pg_range_sqlx_types_time_date,
            _sqlx_postgres_types_pg_range_sqlx_types_big_decimal: sqlx_postgres_types_pg_range_sqlx_types_big_decimal,
            _sqlx_postgres_types_pg_range_sqlx_types_decimal: sqlx_postgres_types_pg_range_sqlx_types_decimal,
            _sqlx_postgres_types_pg_money: sqlx_postgres_types_pg_money,
            _sqlx_postgres_types_pg_ci_text: sqlx_postgres_types_pg_ci_text,
            _sqlx_types_big_decimal: sqlx_types_big_decimal,
            _sqlx_types_decimal: sqlx_types_decimal,
            _sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx_types_chrono_date_time_sqlx_types_chrono_utc,
            _sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx_types_chrono_date_time_sqlx_types_chrono_local,
            _sqlx_types_chrono_naive_date_time: sqlx_types_chrono_naive_date_time,
            _sqlx_types_chrono_naive_date: sqlx_types_chrono_naive_date,
            _sqlx_types_chrono_naive_time: sqlx_types_chrono_naive_time,
            _sqlx_postgres_types_pg_time_tz: sqlx_postgres_types_pg_time_tz,
            _sqlx_types_time_primitive_date_time: sqlx_types_time_primitive_date_time,
            _sqlx_types_time_offset_date_time: sqlx_types_time_offset_date_time,
            _sqlx_types_time_date: sqlx_types_time_date,
            _sqlx_types_time_time: sqlx_types_time_time,
            _sqlx_types_uuid_uuid: sqlx_types_uuid_uuid,
            _sqlx_types_ipnetwork_ip_network: sqlx_types_ipnetwork_ip_network,
            _std_net_ip_addr: std_net_ip_addr,
            _sqlx_types_mac_address_mac_address: sqlx_types_mac_address_mac_address,
            _sqlx_types_bit_vec: sqlx_types_bit_vec,
            // _sqlx_types_json: sqlx_types_json,
            _serde_json_value: serde_json_value,
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

pub trait PostgersqlColumn<'a>: std::fmt::Debug + IntoSerdeSerializeDeserialize + utoipa::ToSchema<'a> + PostgresqlFilter + PostgresqlOrder + PostgresqlLimit {}

pub trait PostgresqlSerdeSerialize<T: serde::Serialize> {
    fn serde_serialize() -> T;
}

pub trait CheckSupportedPostgresqlColumnType {
    fn check_supported_postgresql_column_type();
}
//new type pattern
// sqlx::Encode impl was copied from https://docs.rs/sqlx/0.7.3/sqlx/trait.Encode.html
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
)]
pub struct StdPrimitiveBool(pub std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
impl AsPostgresqlBool for StdPrimitiveBool {}
impl PostgresqlOrder for StdPrimitiveBool {}
impl AsPostgresqlBool for StdOptionOptionStdPrimitiveBool {}
impl PostgresqlOrder for StdOptionOptionStdPrimitiveBool {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
impl AsPostgresqlSmallInt for StdPrimitiveI16 {}
impl AsPostgresqlSmallSerial for StdPrimitiveI16 {}
impl AsPostgresqlInt2 for StdPrimitiveI16 {}
impl PostgresqlOrder for StdPrimitiveI16 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
impl AsPostgresqlInt for StdPrimitiveI32 {}
impl AsPostgresqlSerial for StdPrimitiveI32 {}
impl AsPostgresqlInt4 for StdPrimitiveI32 {}
impl PostgresqlOrder for StdPrimitiveI32 {}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
impl AsPostgresqlBigInt for StdPrimitiveI64 {}
impl AsPostgresqlBigSerial for StdPrimitiveI64 {}
impl AsPostgresqlInt8 for StdPrimitiveI64 {}
impl PostgresqlOrder for StdPrimitiveI64 {}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithoutEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
impl AsPostgresqlReal for StdPrimitiveF32 {}
impl AsPostgresqlFloat4 for StdPrimitiveF32 {}
impl PostgresqlOrder for StdPrimitiveF32 {}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithoutEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
impl AsPostgresqlDoublePrecision for StdPrimitiveF64 {}
impl AsPostgresqlFloat8 for StdPrimitiveF64 {}
impl PostgresqlOrder for StdPrimitiveF64 {}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct StdStringString(pub std::string::String);
impl AsPostgresqlVarchar for StdStringString {}
impl AsPostgresqlCharN for StdStringString {}
impl AsPostgresqlText for StdStringString {}
impl AsPostgresqlCiText for StdStringString {}
impl PostgresqlOrder for StdStringString {}

// #[derive(Debug, Clone, PartialEq, Eq,
//     serde::Serialize,
//     serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
// pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
// impl AsPostgresqlBytea for StdVecVecStdPrimitiveU8 {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
impl serde::Serialize for SqlxPostgresTypesPgInterval {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgInterval", false as usize + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "months", &self.0.months)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "days", &self.0.days)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "microseconds", &self.0.microseconds)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Months,
            Days,
            Microseconds,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`months` or `days` or `microseconds`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "months" => Ok(Field::Months),
                            "days" => Ok(Field::Days),
                            "microseconds" => Ok(Field::Microseconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgIntervalVisitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgIntervalVisitor {
            type Value = SqlxPostgresTypesPgInterval;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgInterval")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgInterval, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let months = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let days = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let microseconds = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval { months, days, microseconds }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgInterval, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut months = None;
                let mut days = None;
                let mut microseconds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Months => {
                            if months.is_some() {
                                return Err(serde::de::Error::duplicate_field("months"));
                            }
                            months = Some(map.next_value()?);
                        }
                        Field::Days => {
                            if days.is_some() {
                                return Err(serde::de::Error::duplicate_field("days"));
                            }
                            days = Some(map.next_value()?);
                        }
                        Field::Microseconds => {
                            if microseconds.is_some() {
                                return Err(serde::de::Error::duplicate_field("microseconds"));
                            }
                            microseconds = Some(map.next_value()?);
                        }
                    }
                }
                let months = months.ok_or_else(|| serde::de::Error::missing_field("months"))?;
                let days = days.ok_or_else(|| serde::de::Error::missing_field("days"))?;
                let microseconds = microseconds.ok_or_else(|| serde::de::Error::missing_field("microseconds"))?;
                Ok(SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval { months, days, microseconds }))
            }
        }
        const FIELDS: &[&str] = &["months", "days", "microseconds"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgInterval", FIELDS, SqlxPostgresTypesPgIntervalVisitor)
    }
}
impl AsPostgresqlInterval for SqlxPostgresTypesPgInterval {}
impl PostgresqlOrder for SqlxPostgresTypesPgInterval {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgInterval {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgInterval {
            months: ::core::default::Default::default(),
            days: ::core::default::Default::default(),
            microseconds: ::core::default::Default::default(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(pub sqlx::postgres::types::PgRange<std::primitive::i64>);
impl serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI64", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "start", &self.0.start)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "end", &self.0.end)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`start` or `end`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            "end" => Ok(Field::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor {
            type Value = SqlxPostgresTypesPgRangeStdPrimitiveI64;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgRangeStdPrimitiveI64")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI64, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let start = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let end = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange { start, end }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI64, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Start => {
                            if start.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                        Field::End => {
                            if end.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end = Some(map.next_value()?);
                        }
                    }
                }
                let start = start.ok_or_else(|| serde::de::Error::missing_field("start"))?;
                let end = end.ok_or_else(|| serde::de::Error::missing_field("end"))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange { start, end }))
            }
        }
        const FIELDS: &[&str] = &["start", "end"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgRangeStdPrimitiveI64", FIELDS, SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor)
    }
}
impl AsPostgresqlInt8Range for SqlxPostgresTypesPgRangeStdPrimitiveI64 {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(pub sqlx::postgres::types::PgRange<std::primitive::i32>);
impl serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI32", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "start", &self.0.start)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "end", &self.0.end)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`start` or `end`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            "end" => Ok(Field::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgRangeStdPrimitiveI32Visitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32Visitor {
            type Value = SqlxPostgresTypesPgRangeStdPrimitiveI32;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgRangeStdPrimitiveI32")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI32, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let start = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let end = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange { start, end }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI32, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Start => {
                            if start.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                        Field::End => {
                            if end.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end = Some(map.next_value()?);
                        }
                    }
                }
                let start = start.ok_or_else(|| serde::de::Error::missing_field("start"))?;
                let end = end.ok_or_else(|| serde::de::Error::missing_field("end"))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange { start, end }))
            }
        }
        const FIELDS: &[&str] = &["start", "end"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgRangeStdPrimitiveI32", FIELDS, SqlxPostgresTypesPgRangeStdPrimitiveI32Visitor)
    }
}
impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtc>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocal>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesTimeOffsetDateTime(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesTimeOffsetDateTime(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesTimeOffsetDateTime(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesTimeOffsetDateTime(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesTimeOffsetDateTime>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesTimeOffsetDateTime>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesTimeOffsetDateTime>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesTimeOffsetDateTime>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesTimeOffsetDateTime>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesTimeOffsetDateTime>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(sqlx::types::time::OffsetDateTime::new_utc(sqlx::types::time::Date::MIN, sqlx::types::time::Time::MIDNIGHT)),
            end: std::ops::Bound::Excluded(sqlx::types::time::OffsetDateTime::new_utc(sqlx::types::time::Date::MIN, sqlx::types::time::Time::MIDNIGHT)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoNaiveDateTime(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoNaiveDateTime(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoNaiveDateTime>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoNaiveDateTime>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesChronoNaiveDateTime>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesChronoNaiveDateTime>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoNaiveDateTime>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoNaiveDateTime>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesTimePrimitiveDateTime(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesTimePrimitiveDateTime(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesTimePrimitiveDateTime(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesTimePrimitiveDateTime(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesTimePrimitiveDateTime>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesTimePrimitiveDateTime>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesTimePrimitiveDateTime>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesTimePrimitiveDateTime>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesTimePrimitiveDateTime>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesTimePrimitiveDateTime>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTsRange for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(sqlx::types::time::PrimitiveDateTime::MIN),
            end: std::ops::Bound::Excluded(sqlx::types::time::PrimitiveDateTime::MIN),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoNaiveDate(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesChronoNaiveDate(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesChronoNaiveDate(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoNaiveDate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesChronoNaiveDate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesChronoNaiveDate>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesChronoNaiveDate>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoNaiveDate>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesChronoNaiveDate>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesTimeDate", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesTimeDate(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesTimeDate(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesTimeDate(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesTimeDate(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesTimeDate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesTimeDate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesTimeDate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesTimeDate>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesTimeDate>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesTimeDate>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesTimeDate>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesTimeDate>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesTimeDate(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesTimeDate",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesTimeDate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(sqlx::types::time::Date::MIN),
            end: std::ops::Bound::Excluded(sqlx::types::time::Date::MIN),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesBigDecimal", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimal(value.clone())),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimal(value.clone())),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimal(value.clone())),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimal(value.clone())),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesBigDecimal;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesBigDecimal>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesBigDecimal>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesBigDecimal>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesBigDecimal>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesBigDecimal>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesBigDecimal>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesBigDecimal",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>);
impl serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesDecimal", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "start",
            &match std::ops::RangeBounds::start_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesDecimal(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesDecimal(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::serialize_field(
            &mut __serde_state,
            "end",
            &match std::ops::RangeBounds::end_bound(&self.0) {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesDecimal(*value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesDecimal(*value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        )?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "start" => serde::__private::Ok(__Field::__field0),
                    "end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"start" => serde::__private::Ok(__Field::__field0),
                    b"end" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesDecimal>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgRangeSqlxTypesDecimal;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgRangeSqlxTypesDecimal")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesDecimal>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesDecimal with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::ops::Bound<SqlxTypesDecimal>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesDecimal with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::ops::Bound<SqlxTypesDecimal>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::ops::Bound<SqlxTypesDecimal>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("start"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesDecimal>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("end"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::ops::Bound<SqlxTypesDecimal>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("start")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("end")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange {
                    start: match __field0 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match __field1 {
                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["start", "end"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgRangeSqlxTypesDecimal",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesDecimal>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesDecimal {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(::core::default::Default::default()),
            end: std::ops::Bound::Excluded(::core::default::Default::default()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
impl serde::Serialize for SqlxPostgresTypesPgMoney {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(__serializer, "SqlxPostgresTypesPgMoney", &self.0 .0)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgMoney {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgMoney>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgMoney;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxPostgresTypesPgMoney")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::primitive::i64 = <std::primitive::i64 as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney(__field0)))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxPostgresTypesPgMoney with 1 element"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney(__field0)))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SqlxPostgresTypesPgMoney",
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgMoney>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlMoney for SqlxPostgresTypesPgMoney {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgMoney {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgMoney(::core::default::Default::default()))
    }
}

#[derive(Debug, Clone, PartialEq, postgresql_crud_types_macro_logic_reuse::CommonWithoutEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
impl serde::Serialize for SqlxPostgresTypesPgCiText {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(__serializer, "SqlxPostgresTypesPgCiText", &self.0 .0)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgCiText {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgCiText>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgCiText;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxPostgresTypesPgCiText")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::string::String = <std::string::String as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(SqlxPostgresTypesPgCiText(sqlx::postgres::types::PgCiText(__field0)))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxPostgresTypesPgCiText with 1 element"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgCiText(sqlx::postgres::types::PgCiText(__field0)))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SqlxPostgresTypesPgCiText",
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgCiText>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlCiText for SqlxPostgresTypesPgCiText {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
impl serde::Serialize for SqlxTypesBigDecimal {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let (bigint, exponent) = self.0.clone().into_bigint_and_exponent();
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesBigDecimal", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "digits", &NumBigintBigInt(bigint))?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "scale", &exponent)?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesBigDecimal {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "digits" => serde::__private::Ok(__Field::__field0),
                    "scale" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"digits" => serde::__private::Ok(__Field::__field0),
                    b"scale" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesBigDecimal>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesBigDecimal;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesBigDecimal")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<NumBigintBigInt>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesBigDecimal with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesBigDecimal with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxTypesBigDecimal(sqlx::types::BigDecimal::new(__field0.0, __field1)))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<NumBigintBigInt> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("digits"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<NumBigintBigInt>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("scale"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("digits")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("scale")?,
                };
                serde::__private::Ok(SqlxTypesBigDecimal(sqlx::types::BigDecimal::new(__field0.0, __field1)))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["digits", "scale"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesBigDecimal",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesBigDecimal>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlNumeric for SqlxTypesBigDecimal {}
impl PostgresqlOrder for SqlxTypesBigDecimal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>);
impl AsPostgresqlTimestampTz for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
impl AsPostgresqlTimestamp for SqlxTypesChronoNaiveDateTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDateTime {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
impl AsPostgresqlDate for SqlxTypesChronoNaiveDate {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDate {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
impl AsPostgresqlTime for SqlxTypesChronoNaiveTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveTime {}

#[derive(Debug, Clone, Copy, PartialEq, postgresql_crud_types_macro_logic_reuse::CommonWithoutEqImpl)]
pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
impl serde::Serialize for SqlxPostgresTypesPgTimeTz {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgTimeTz", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "time", &SqlxTypesTimeTime(self.0.time))?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "offset", &SqlxTypesTimeUtcOffset(self.0.offset))?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgTimeTz {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "time" => serde::__private::Ok(__Field::__field0),
                    "offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"time" => serde::__private::Ok(__Field::__field0),
                    b"offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxPostgresTypesPgTimeTz>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxPostgresTypesPgTimeTz;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxPostgresTypesPgTimeTz")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<SqlxTypesTimeTime>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgTimeTz with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<SqlxTypesTimeUtcOffset>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgTimeTz with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz { time: __field0.0, offset: __field1.0 }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<SqlxTypesTimeTime> = serde::__private::None;
                let mut __field1: serde::__private::Option<SqlxTypesTimeUtcOffset> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("time"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<SqlxTypesTimeTime>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("offset"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<SqlxTypesTimeUtcOffset>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("time")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("offset")?,
                };
                serde::__private::Ok(SqlxPostgresTypesPgTimeTz(sqlx::postgres::types::PgTimeTz { time: __field0.0, offset: __field1.0 }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["time", "offset"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxPostgresTypesPgTimeTz",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxPostgresTypesPgTimeTz>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTimeTz for SqlxPostgresTypesPgTimeTz {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgTimeTz {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgTimeTz {
            time: sqlx::types::time::Time::MIDNIGHT,
            offset: sqlx::types::time::UtcOffset::UTC,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
impl serde::Serialize for SqlxTypesTimePrimitiveDateTime {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesTimePrimitiveDateTime", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "date", &SqlxTypesTimeDate(self.0.date()))?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "time", &SqlxTypesTimeTime(self.0.time()))?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesTimePrimitiveDateTime {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "date" => serde::__private::Ok(__Field::__field0),
                    "time" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"date" => serde::__private::Ok(__Field::__field0),
                    b"time" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesTimePrimitiveDateTime>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesTimePrimitiveDateTime;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesTimePrimitiveDateTime")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<SqlxTypesTimeDate>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesTimePrimitiveDateTime with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<SqlxTypesTimeTime>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesTimePrimitiveDateTime with 2 elements"));
                    }
                };
                serde::__private::Ok(SqlxTypesTimePrimitiveDateTime(sqlx::types::time::PrimitiveDateTime::new(__field0.0, __field1.0)))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<SqlxTypesTimeDate> = serde::__private::None;
                let mut __field1: serde::__private::Option<SqlxTypesTimeTime> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("date"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<SqlxTypesTimeDate>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("time"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<SqlxTypesTimeTime>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("date")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("time")?,
                };
                serde::__private::Ok(SqlxTypesTimePrimitiveDateTime(sqlx::types::time::PrimitiveDateTime::new(__field0.0, __field1.0)))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["date", "time"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesTimePrimitiveDateTime",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesTimePrimitiveDateTime>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTimestamp for SqlxTypesTimePrimitiveDateTime {}
impl PostgresqlOrder for SqlxTypesTimePrimitiveDateTime {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesTimePrimitiveDateTime {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::time::PrimitiveDateTime::MIN)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
impl serde::Serialize for SqlxTypesTimeOffsetDateTime {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesTimeOffsetDateTime", &self.0.unix_timestamp())
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesTimeOffsetDateTime {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesTimeOffsetDateTime>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesTimeOffsetDateTime;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesTimeOffsetDateTime")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::primitive::i64 = <std::primitive::i64 as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(SqlxTypesTimeOffsetDateTime(match sqlx::types::time::OffsetDateTime::from_unix_timestamp(__field0) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesTimeOffsetDateTime with 1 element"));
                    }
                };
                serde::__private::Ok(SqlxTypesTimeOffsetDateTime(match sqlx::types::time::OffsetDateTime::from_unix_timestamp(__field0) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SqlxTypesTimeOffsetDateTime",
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesTimeOffsetDateTime>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTimestampTz for SqlxTypesTimeOffsetDateTime {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesTimeOffsetDateTime {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::time::OffsetDateTime::UNIX_EPOCH)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
impl serde::Serialize for SqlxTypesTimeDate {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesTimeDate", false as usize + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "year", &self.0.year())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "month", &TimeMonth(self.0.month().into()))?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "day", &self.0.day())?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesTimeDate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "year" => serde::__private::Ok(__Field::__field0),
                    "month" => serde::__private::Ok(__Field::__field1),
                    "day" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"year" => serde::__private::Ok(__Field::__field0),
                    b"month" => serde::__private::Ok(__Field::__field1),
                    b"day" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesTimeDate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesTimeDate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesTimeDate")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesTimeDate with 3 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<TimeMonth>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesTimeDate with 3 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::primitive::u8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(2usize, &"struct SqlxTypesTimeDate with 3 elements"));
                    }
                };
                serde::__private::Ok(SqlxTypesTimeDate(match sqlx::types::time::Date::from_calendar_date(__field0, __field1.0, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::i32> = serde::__private::None;
                let mut __field1: serde::__private::Option<TimeMonth> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::primitive::u8> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("year"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("month"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<TimeMonth>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("day"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::u8>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("year")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("month")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("day")?,
                };
                serde::__private::Ok(SqlxTypesTimeDate(match sqlx::types::time::Date::from_calendar_date(__field0, __field1.0, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["year", "month", "day"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesTimeDate",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesTimeDate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlDate for SqlxTypesTimeDate {}
impl PostgresqlOrder for SqlxTypesTimeDate {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesTimeDate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::time::Date::MIN)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
impl serde::Serialize for SqlxTypesTimeTime {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesTimeTime", false as usize + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "hour", &self.0.hour())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "minute", &self.0.minute())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "second", &self.0.second())?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesTimeTime {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "hour" => serde::__private::Ok(__Field::__field0),
                    "minute" => serde::__private::Ok(__Field::__field1),
                    "second" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"hour" => serde::__private::Ok(__Field::__field0),
                    b"minute" => serde::__private::Ok(__Field::__field1),
                    b"second" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesTimeTime>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesTimeTime;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesTimeTime")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::u8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesTimeTime with 3 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::u8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesTimeTime with 3 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::primitive::u8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(2usize, &"struct SqlxTypesTimeTime with 3 elements"));
                    }
                };
                serde::__private::Ok(SqlxTypesTimeTime(match sqlx::types::time::Time::from_hms(__field0, __field1, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::u8> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::u8> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::primitive::u8> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("hour"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::u8>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("minute"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::u8>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("second"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::u8>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("hour")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("minute")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("second")?,
                };
                serde::__private::Ok(SqlxTypesTimeTime(match sqlx::types::time::Time::from_hms(__field0, __field1, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["hour", "minute", "second"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesTimeTime",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesTimeTime>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlTime for SqlxTypesTimeTime {}
impl PostgresqlOrder for SqlxTypesTimeTime {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesTimeTime {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::time::Time::MIDNIGHT)
    }
}

//todo maybe its possible to not use Clone (refactor where .clone() used)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
impl serde::Serialize for SqlxTypesUuidUuid {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesUuidUuid", &self.0.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesUuidUuid {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesUuidUuid>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesUuidUuid;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesUuidUuid")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::string::String = <std::string::String as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(SqlxTypesUuidUuid(match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesUuidUuid with 1 element"));
                    }
                };
                serde::__private::Ok(SqlxTypesUuidUuid(match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SqlxTypesUuidUuid",
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesUuidUuid>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlUuid for SqlxTypesUuidUuid {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
impl AsPostgresqlInet for SqlxTypesIpnetworkIpNetwork {}
impl AsPostgresqlCidr for SqlxTypesIpnetworkIpNetwork {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesIpnetworkIpNetwork {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, ::core::default::Default::default()).unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl)]
pub struct StdNetIpAddr(pub std::net::IpAddr);
impl AsPostgresqlInet for StdNetIpAddr {}
impl AsPostgresqlCidr for StdNetIpAddr {}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdNetIpAddr {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
impl serde::Serialize for SqlxTypesMacAddressMacAddress {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesMacAddressMacAddress", &self.0.bytes())
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesMacAddressMacAddress {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesMacAddressMacAddress>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesMacAddressMacAddress;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesMacAddressMacAddress")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: [std::primitive::u8; 6] = <[std::primitive::u8; 6] as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress::new(__field0)))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<[std::primitive::u8; 6]>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesMacAddressMacAddress with 1 element"));
                    }
                };
                serde::__private::Ok(SqlxTypesMacAddressMacAddress(sqlx::types::mac_address::MacAddress::new(__field0)))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SqlxTypesMacAddressMacAddress",
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesMacAddressMacAddress>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlMacAddr for SqlxTypesMacAddressMacAddress {}

#[derive(Debug, Clone, PartialEq, Eq, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
impl serde::Serialize for SqlxTypesBitVec {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesBitVec", &self.0.iter().map(|element| Into::into(element)).collect::<std::vec::Vec<std::primitive::u8>>())
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesBitVec {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesBitVec>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesBitVec;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesBitVec")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<std::primitive::u8> = <std::vec::Vec<std::primitive::u8> as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(SqlxTypesBitVec(sqlx::types::BitVec::from_bytes(&__field0)))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::u8>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesBitVec with 1 element"));
                    }
                };
                serde::__private::Ok(SqlxTypesBitVec(sqlx::types::BitVec::from_bytes(&__field0)))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SqlxTypesBitVec",
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesBitVec>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl AsPostgresqlBit for SqlxTypesBitVec {}
impl AsPostgresqlVarBit for SqlxTypesBitVec {}
impl PostgresqlOrder for SqlxTypesBitVec {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<T> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'a, T> sqlx::Encode<'a, sqlx::Postgres> for SqlxTypesJson<T>
where
    T: sqlx::Encode<'a, sqlx::Postgres> + Copy + Clone + std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a>, //todo maybe add another traits impls
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
    fn encode(self, buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer) -> sqlx::encode::IsNull
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
    T: sqlx::Decode<'a, sqlx::Postgres> + Copy + Clone + std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a>, //todo maybe add another traits impls
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
impl<T> std::fmt::Display for SqlxTypesJson<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl<T> error_occurence_lib::ToStdStringString for SqlxTypesJson<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl<T> AsPostgresqlJson for SqlxTypesJson<T> {}
impl<T> AsPostgresqlJsonB for SqlxTypesJson<T> {}
impl<T> std::convert::From<SqlxTypesJson<T>> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesJson<T>) -> Self {
        Self::SqlxTypesJson
    }
}
impl<T> SqlxTypesJson<T> {
    pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value.into_iter().map(Self::into_inner).collect()
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + BindQuery<'a> + 'a> BindQuery<'a> for SqlxTypesJson<T> {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        match self.0.try_increment(increment) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
        match self.0.try_generate_bind_increments(increment) {
            Ok(value) => Ok(value),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0 .0.bind_value_to_query(query);
        query
    }
}
impl<T> crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesJson<T>
where
    T: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement,
{
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::types::Json(
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WhereSqlxTypesJson<T> {
    pub value: SqlxTypesJson<T>,
    pub logical_operator: LogicalOperator,
}
impl<T> std::fmt::Display for WhereSqlxTypesJson<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "value: {:?}, logical_operator: {}", self.value, self.logical_operator)
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> BindQuery<'a> for WhereSqlxTypesJson<T> {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        //todo rewrite to support inner type BindQuery for t
        increment.checked_add(1).map_or_else(
            || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(())
            },
        )
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
        //todo rewrite to support inner type BindQuery for t
        increment.checked_add(1).map_or_else(
            || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(format!("${increment}"))
            },
        )
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        //todo rewrite to support inner type BindQuery for t
        query = query.bind(self.value.0);
        query
    }
}
impl<T> crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WhereSqlxTypesJson<T>
where
    T: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement,
{
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            logical_operator: LogicalOperator::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
impl<T> std::convert::From<SqlxTypesJson<T>> for StdOptionOptionSqlxTypesJson<T> {
    fn from(value: SqlxTypesJson<T>) -> Self {
        Self(Some(value.0))
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
        Self::SqlxTypesJson
    }
}
impl<T> StdOptionOptionSqlxTypesJson<T> {
    pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<sqlx::types::Json<T>>> {
        value.into_iter().map(Self::into_inner).collect()
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + sqlx::Type<sqlx::Postgres> + sqlx::Encode<'a, sqlx::Postgres> + 'a> BindQuery<'a> for StdOptionOptionSqlxTypesJson<T> {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        //todo rewrite to support inner type BindQuery for t
        increment.checked_add(1).map_or_else(
            || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(())
            },
        )
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
        //todo rewrite to support inner type BindQuery for t
        increment.checked_add(1).map_or_else(
            || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(format!("${increment}"))
            },
        )
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        //todo rewrite to support inner type BindQuery for t
        query = query.bind(match self.0 {
            Some(value) => Some(value.0),
            None => None,
        });
        query
    }
}
impl<T> crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionSqlxTypesJson<T>
where
    T: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement,
{
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(sqlx::types::Json(
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        )))
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WhereStdOptionOptionSqlxTypesJson<T> {
    pub value: StdOptionOptionSqlxTypesJson<T>,
    pub logical_operator: LogicalOperator,
}
impl<T: std::fmt::Debug> std::fmt::Display for WhereStdOptionOptionSqlxTypesJson<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "value: {}, logical_operator: {}", self.value, self.logical_operator)
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + sqlx::Type<sqlx::Postgres> + sqlx::Encode<'a, sqlx::Postgres> + 'a> BindQuery<'a> for WhereStdOptionOptionSqlxTypesJson<T> {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
        //todo rewrite to support inner type BindQuery for t
        increment.checked_add(1).map_or_else(
            || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(())
            },
        )
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
        //todo rewrite to support inner type BindQuery for t
        increment.checked_add(1).map_or_else(
            || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(format!("${increment}"))
            },
        )
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        //todo rewrite to support inner type BindQuery for t
        query = query.bind(match self.value.0 {
            Some(value) => Some(value.0),
            None => None,
        });
        query
    }
}
impl<T> crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WhereStdOptionOptionSqlxTypesJson<T>
where
    T: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement,
{
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            logical_operator: LogicalOperator::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl, postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub struct SerdeJsonValue(pub serde_json::Value);
impl AsPostgresqlJson for SerdeJsonValue {}
impl AsPostgresqlJsonB for SerdeJsonValue {}

pub fn test_check_supported_postgresql_column_type() {
    //todo check if init functions are not panics. change to not panic init functions
    StdPrimitiveBool::check_supported_postgresql_column_type();
    StdPrimitiveI16::check_supported_postgresql_column_type();
    StdPrimitiveI32::check_supported_postgresql_column_type();
    StdPrimitiveI64::check_supported_postgresql_column_type();
    StdPrimitiveF32::check_supported_postgresql_column_type();
    StdPrimitiveF64::check_supported_postgresql_column_type();
    StdStringString::check_supported_postgresql_column_type();
    // StdVecVecStdPrimitiveU8::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgInterval::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeStdPrimitiveI64::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeStdPrimitiveI32::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime::check_supported_postgresql_column_type();
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::check_supported_postgresql_column_type();
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

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateBindIncrementsErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
//todo add another error variant instead for PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed
impl std::convert::From<crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> for TryGenerateBindIncrementsErrorNamed {
    fn from(value: crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed) -> Self {
        match value {
            crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence }
        }
    }
}
pub trait BindQuery<'a> {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed>;
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed>;
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq, schemars::JsonSchema)]
pub enum LogicalOperator {
    And,
    Or,
    AndNot,
    OrNot,
}
impl LogicalOperator {
    pub fn to_query_part(&self, is_need_to_add_logical_operator: std::primitive::bool) -> std::string::String {
        let not_space = format!("{} ", naming::NotSnakeCase);
        if is_need_to_add_logical_operator {
            let and_space = format!("{} ", naming::AndSnakeCase);
            let or_space = format!("{} ", naming::OrSnakeCase);
            match &self {
                Self::And => and_space,
                Self::Or => or_space,
                Self::AndNot => format!("{and_space}{not_space}"),
                Self::OrNot => format!("{or_space}{not_space}"),
            }
        }
        else {
            match &self {
                Self::And |
                Self::Or => std::string::String::default(),
                Self::AndNot |
                Self::OrNot => not_space,
            }
        }
    }
}
impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl Default for LogicalOperator {
    fn default() -> Self {
        Self::Or
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for LogicalOperator {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
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
            Self::Asc => write!(formatter, "{}", naming::AscUpperCamelCase),
            Self::Desc => write!(formatter, "{}", naming::DescUpperCamelCase),
        }
    }
}
impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Order {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
impl Order {
    pub fn to_upper_camel_case_stringified(&self) -> std::string::String {
        naming::DisplayToUpperCamelCaseStringified::new(&self)
    }
    pub fn to_snake_case_stringified(&self) -> std::string::String {
        naming::DisplayToSnakeCaseStringified::new(&self)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<Order>,
}

pub trait GeneratePostgresqlJsonTypeToRead {
    fn generate_postgresql_json_type_to_read_from_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String
    where
        Self: Sized;
}

pub trait CreateTableColumnQueryPart {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display;
}
pub fn maybe_primary_key(is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
    if is_primary_key {
        " PRIMARY KEY"
    } else {
        ""
    }
}

pub trait BindQuerySecond<'a> {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed>;
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}