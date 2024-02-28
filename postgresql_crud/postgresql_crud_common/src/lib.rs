pub enum SupportedSqlxPostgresType {
    StdPrimitiveBool,
    StdPrimitiveI8,
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
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
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
    SqlxTypesJsonT,//todo what to do with generic?
    SerdeJsonValue,
}

impl std::convert::From<RustSqlxMapToPostgresTypeVariant> for SupportedSqlxPostgresType {
    fn from(value: RustSqlxMapToPostgresTypeVariant) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Self::StdPrimitiveBool,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI8AsPostgresqlChar => Self::StdPrimitiveI8,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => Self::StdPrimitiveI16,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Self::StdPrimitiveI16,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => Self::StdPrimitiveI32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Self::StdPrimitiveI32,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => Self::StdPrimitiveI64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Self::StdPrimitiveI64,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => Self::StdPrimitiveF32,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Self::StdPrimitiveF32,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => Self::StdPrimitiveF64,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Self::StdPrimitiveF64,
        
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlName => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Self::StdStringString,
        
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Self::StdVecVecStdPrimitiveU8,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Self::SqlxPostgresTypesPgInterval,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Self::SqlxPostgresTypesPgMoney,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Self::SqlxPostgresTypesPgCiText,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::SqlxTypesBigDecimal,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Self::SqlxTypesDecimal,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,            

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesChronoNaiveDateTime,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::SqlxTypesChronoNaiveDate,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Self::SqlxTypesChronoNaiveTime,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Self::SqlxPostgresTypesPgTimeTz,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesTimePrimitiveDateTime,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Self::SqlxTypesTimeOffsetDateTime,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Self::SqlxTypesTimeDate,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Self::SqlxTypesTimeTime,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Self::SqlxTypesUuidUuid,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Self::SqlxTypesIpnetworkIpNetwork,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::SqlxTypesIpnetworkIpNetwork,
        
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => Self::StdNetIpAddr,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Self::StdNetIpAddr,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::SqlxTypesMacAddressMacAddress,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => Self::SqlxTypesBitVec,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Self::SqlxTypesBitVec,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => Self::SqlxTypesJsonT,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Self::SqlxTypesJsonT,
        
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Self::SerdeJsonValue,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::SerdeJsonValue,
        }
    }
}

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
    Name,
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

impl std::convert::From<RustSqlxMapToPostgresTypeVariant> for PostgresqlType {
    fn from(value: RustSqlxMapToPostgresTypeVariant) -> Self {
        match value {
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => Self::Bool,

            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI8AsPostgresqlChar => Self::Char,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => Self::SmallInt,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => Self::SmallSerial,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => Self::Int2,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => Self::Int,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => Self::Serial,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => Self::Int4,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => Self::BigInt,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => Self::BigSerial,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => Self::Int8,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => Self::Real,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => Self::Float4,
        
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => Self::DoublePrecision,
            RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => Self::Float8,
        
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => Self::Varchar,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => Self::CharN,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => Self::Text,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlName => Self::Name,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => Self::CiText,
        
            RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Self::Bytea,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Self::Interval,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Self::Int8Range,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Self::Int4Range,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsRange => Self::TsRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => Self::TsRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => Self::TsTzRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => Self::TsTzRange,

            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Self::DateRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Self::DateRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Self::NumRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Self::NumRange,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Self::Money,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => Self::CiText,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::Numeric,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => Self::Numeric,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp => Self::Timestamp,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::TimestampTz,

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::Timestamp,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::Date,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Self::Time,
        
            RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => Self::TimeTz,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::Timestamp,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => Self::TimestampTz,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => Self::Date,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => Self::Time,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => Self::Uuid,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Self::Inet,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Self::Cidr,
        
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => Self::Inet,
            RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => Self::Cidr,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Self::MacAddr,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => Self::Bit,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => Self::VarBit,
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => Self::Json,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Self::JsonB,
        
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Self::Json,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::JsonB,
        }
    }
}

impl PostgresqlType {
    pub fn postgresql_naming(&self) -> &str {
        match self {
            Self::Bool => "BOOL",
            Self::Char => "CHAR",
            Self::SmallInt => "SMALLINT",
            Self::SmallSerial => "SMALLSERIAL",
            Self::Int2 => "INT2",
            Self::Int => "INT",
            Self::Serial => "SERIAL",
            Self::Int4 => "INT4",
            Self::BigInt => "BIGINT",
            Self::BigSerial => "BIGSERIAL",
            Self::Int8 => "INT8",
            Self::Real => "REAL",
            Self::Float4 => "FLOAT4",
            Self::DoublePrecision => "DOUBLE PRECISION",
            Self::Float8 => "FLOAT8",
            Self::Varchar => "VARCHAR",
            Self::CharN => "CHAR(N)",
            Self::Text => "TEXT",
            Self::Name => "NAME",
            Self::CiText => "CITEXT",
            Self::Bytea => "BYTEA",
            Self::Interval => "INTERVAL",
            Self::Int8Range => "INT8RANGE",
            Self::Int4Range => "INT4RANGE",
            Self::TsRange => "TSRANGE",
            Self::TsTzRange => "TSTZRANGE",
            Self::DateRange => "DATERANGE",
            Self::NumRange => "NUMRANGE",
            Self::Money => "MONEY",
            Self::Numeric => "NUMERIC",
            Self::TimestampTz => "TIMESTAMPTZ",
            Self::Date => "DATE",
            Self::Time => "TIME",
            Self::TimeTz => "TIMETZ",
            Self::Timestamp => "TIMESTAMP",
            Self::Uuid => "UUID",
            Self::Inet => "INET",
            Self::Cidr => "CIDR",
            Self::MacAddr => "MACADDR",
            Self::Bit => "BIT",
            Self::VarBit => "VARBIT",
            Self::Json => "JSON",
            Self::JsonB => "JSONB",
        }
    }
}

pub enum RustSqlxMapToPostgresTypeVariant {
    StdPrimitiveBoolAsPostgresqlBool,

    StdPrimitiveI8AsPostgresqlChar,

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
    StdStringStringAsPostgresqlName,
    StdStringStringAsPostgresqlCiText,

    StdVecVecStdPrimitiveU8AsPostgresqlBytea,

    SqlxPostgresTypesPgIntervalAsPostgresqlInterval,

    SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,

    SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,

    SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,

    SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,

    SqlxPostgresTypesPgMoneyAsPostgresqlMoney,

    SqlxPostgresTypesPgCiTextAsPostgresqlCiText,

    SqlxTypesBigDecimalAsPostgresqlNumeric,

    SqlxTypesDecimalAsPostgresqlNumeric,

    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp,

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