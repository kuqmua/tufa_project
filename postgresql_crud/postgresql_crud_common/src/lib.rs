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
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
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
    SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
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
    LTree,
    LQuery,
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
    //maybe Composite types
    //maybe Enumerations
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
            Self::LTree => "LTREE",
            Self::LQuery => "LQUERY",
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
            //maybe Composite types
            //maybe Enumerations
        }
    }
}

pub enum Variants {
    StdPrimitiveBoolAsPostgresqlBool,

    StdPrimitiveI8AsPostgresqlChar,

    StdPrimitiveI16AsPostgresqlSmallInt,
    StdPrimitiveI16AsPostgresqlSmallSerial,
    StdPrimitiveI16AsPostgresqlInt2,

    StdPrimitiveI32AsPostgresqlReal,
    StdPrimitiveI32AsPostgresqlFloat4,

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

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffsetAsPostgresqlTsTzRange,

    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,

    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,//todo reorder from now

    SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,

    SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,

    SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,

    SqlxPostgresTypesPgMoneyAsPostgresqlMoney,

    SqlxPostgresTypesPgCiTextAsPostgresqlCiText,

    SqlxTypesBigDecimalAsPostgresqlNumeric,

    SqlxTypesDecimalAsPostgresqlNumeric,

    SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffsetAsPostgresqlTimeTz,

    SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,

    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp,

    SqlxTypesChronoNaiveDateTimeAsPostgresqlTsTzRange,

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