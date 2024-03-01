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
    SqlxTypesJsonT,//todo what to do with generic?
    SerdeJsonValue,
}

impl std::convert::From<RustSqlxMapToPostgresTypeVariant> for SupportedSqlxPostgresType {
    fn from(value: RustSqlxMapToPostgresTypeVariant) -> Self {
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
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlName => Self::StdStringString,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlNameNotNull => Self::StdStringString,
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

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        
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

pub enum PostgresqlType {
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
    Name,
    NameNotNull,
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

impl std::convert::From<RustSqlxMapToPostgresTypeVariant> for PostgresqlType {
    fn from(value: RustSqlxMapToPostgresTypeVariant) -> Self {
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
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlName => Self::Name,
            RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlNameNotNull => Self::NameNotNull,
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

            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp => Self::Timestamp,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull => Self::TimestampNotNull,
        
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
            RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => Self::UuidNotNullPrimaryKey,
        
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
        
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => Self::Json,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull => Self::JsonNotNull,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => Self::JsonB,
            RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull => Self::JsonBNotNull,
        
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => Self::Json,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull => Self::JsonNotNull,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => Self::JsonB,
            RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => Self::JsonBNotNull,
        }
    }
}

impl PostgresqlType {
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
            Self::Name => "NAME",
            Self::NameNotNull => "NAME",
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
//todo maybe use it as type for struct field but with inner type like StdPrimitiveBoolAsPostgresqlBool(StdPrimitiveBool)
#[derive(
    Debug, 
    PartialEq,
    Eq,
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
    StdStringStringAsPostgresqlName,
    StdStringStringAsPostgresqlNameNotNull,
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

    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull,

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
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough

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

impl RustSqlxMapToPostgresTypeVariant {
    pub fn generate_path_stringified(&self) -> std::string::String {
        format!("postgresql_crud::{self}")
    }
}
//todo maybe move to generate_postgresql_crud macro 
// impl RustSqlxMapToPostgresTypeVariant {
//     pub fn generate_inner_type_stringified(&self, generic_type_str: &str) -> std::string::String {
//         match self {
//             Self::StdPrimitiveBoolAsPostgresqlBool => std::string::String::from("std::primitive::bool"),//todo maybe Option<T> for nullable ?
//             Self::StdPrimitiveBoolAsPostgresqlBoolNotNull => std::string::String::from("std::primitive::bool"),

//             Self::StdPrimitiveI16AsPostgresqlSmallInt => std::string::String::from("std::primitive::i16"),
//             Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull => std::string::String::from("std::primitive::i16"),
//             Self::StdPrimitiveI16AsPostgresqlSmallSerial => std::string::String::from("std::primitive::i16"),
//             Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => std::string::String::from("std::primitive::i16"),
//             Self::StdPrimitiveI16AsPostgresqlInt2 => std::string::String::from("std::primitive::i16"),
//             Self::StdPrimitiveI16AsPostgresqlInt2NotNull => std::string::String::from("std::primitive::i16"),

//             Self::StdPrimitiveI32AsPostgresqlInt => std::string::String::from("std::primitive::i32"),
//             Self::StdPrimitiveI32AsPostgresqlIntNotNull => std::string::String::from("std::primitive::i32"),
//             Self::StdPrimitiveI32AsPostgresqlSerial => std::string::String::from("std::primitive::i32"),
//             Self::StdPrimitiveI32AsPostgresqlSerialNotNull => std::string::String::from("std::primitive::i32"),
//             Self::StdPrimitiveI32AsPostgresqlInt4 => std::string::String::from("std::primitive::i32"),
//             Self::StdPrimitiveI32AsPostgresqlInt4NotNull => std::string::String::from("std::primitive::i32"),

//             Self::StdPrimitiveI64AsPostgresqlBigInt => std::string::String::from("std::primitive::i64"),
//             Self::StdPrimitiveI64AsPostgresqlBigIntNotNull => std::string::String::from("std::primitive::i64"),
//             Self::StdPrimitiveI64AsPostgresqlBigSerial => std::string::String::from("std::primitive::i64"),
//             Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull => std::string::String::from("std::primitive::i64"),
//             Self::StdPrimitiveI64AsPostgresqlInt8 => std::string::String::from("std::primitive::i64"),
//             Self::StdPrimitiveI64AsPostgresqlInt8NotNull => std::string::String::from("std::primitive::i64"),

//             Self::StdPrimitiveF32AsPostgresqlReal => std::string::String::from("std::primitive::f32"),
//             Self::StdPrimitiveF32AsPostgresqlRealNotNull => std::string::String::from("std::primitive::f32"),
//             Self::StdPrimitiveF32AsPostgresqlFloat4 => std::string::String::from("std::primitive::f32"),
//             Self::StdPrimitiveF32AsPostgresqlFloat4NotNull => std::string::String::from("std::primitive::f32"),

//             Self::StdPrimitiveF64AsPostgresqlDoublePrecision => std::string::String::from("std::primitive::f64"),
//             Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => std::string::String::from("std::primitive::f64"),
//             Self::StdPrimitiveF64AsPostgresqlFloat8 => std::string::String::from("std::primitive::f64"),
//             Self::StdPrimitiveF64AsPostgresqlFloat8NotNull => std::string::String::from("std::primitive::f64"),

//             Self::StdStringStringAsPostgresqlVarchar => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlVarcharNotNull => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlCharN => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlCharNNotNull => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlText => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlTextNotNull => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlName => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlNameNotNull => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlCiText => std::string::String::from("std::string::String"),
//             Self::StdStringStringAsPostgresqlCiTextNotNull => std::string::String::from("std::string::String"),

//             Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea => std::string::String::from("std::vec::Vec<std::primitive::u8>"),
//             Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => std::string::String::from("std::vec::Vec<std::primitive::u8>"),

//             Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => std::string::String::from("sqlx::postgres::types::PgInterval"),
//             Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => std::string::String::from("sqlx::postgres::types::PgInterval"),

//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => std::string::String::from("sqlx::postgres::types::PgRange<std::primitive::i64>"),
//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<std::primitive::i64>"),

//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => std::string::String::from("sqlx::postgres::types::PgRange<std::primitive::i32>"),
//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<std::primitive::i32>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::Date>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::time::Date>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::Decimal>"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => std::string::String::from("sqlx::postgres::types::PgRange<sqlx::types::Decimal>"),

//             Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => std::string::String::from("sqlx::postgres::types::PgMoney"),
//             Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => std::string::String::from("sqlx::postgres::types::PgMoney"),

//             Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => std::string::String::from("sqlx::postgres::types::PgCiText"),
//             Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => std::string::String::from("sqlx::postgres::types::PgCiText"),

//             Self::SqlxTypesBigDecimalAsPostgresqlNumeric => std::string::String::from("sqlx::types::BigDecimal"),
//             Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => std::string::String::from("sqlx::types::BigDecimal"),

//             Self::SqlxTypesDecimalAsPostgresqlNumeric => std::string::String::from("sqlx::types::Decimal"),
//             Self::SqlxTypesDecimalAsPostgresqlNumericNotNull => std::string::String::from("sqlx::types::Decimal"),

//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp => std::string::String::from("sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>"),
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull => std::string::String::from("sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>"),

//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => std::string::String::from("sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>"),
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => std::string::String::from("sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>"),

//             Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => std::string::String::from("sqlx::types::chrono::NaiveDateTime"),
//             Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => std::string::String::from("sqlx::types::chrono::NaiveDateTime"),

//             Self::SqlxTypesChronoNaiveDateAsPostgresqlDate => std::string::String::from("sqlx::types::chrono::NaiveDate"),
//             Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => std::string::String::from("sqlx::types::chrono::NaiveDate"),

//             Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime => std::string::String::from("sqlx::types::chrono::NaiveTime"),
//             Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => std::string::String::from("sqlx::types::chrono::NaiveTime"),

//             Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => std::string::String::from("sqlx::postgres::types::PgTimeTz"),
//             Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => std::string::String::from("sqlx::postgres::types::PgTimeTz"),

//             Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => std::string::String::from("sqlx::types::time::PrimitiveDateTime"),
//             Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => std::string::String::from("sqlx::types::time::PrimitiveDateTime"),

//             Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => std::string::String::from("sqlx::types::time::OffsetDateTime"),
//             Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => std::string::String::from("sqlx::types::time::OffsetDateTime"),

//             Self::SqlxTypesTimeDateAsPostgresqlDate => std::string::String::from("sqlx::types::time::Date"),
//             Self::SqlxTypesTimeDateAsPostgresqlDateNotNull => std::string::String::from("sqlx::types::time::Date"),

//             Self::SqlxTypesTimeTimeAsPostgresqlTime => std::string::String::from("sqlx::types::time::Time"),
//             Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => std::string::String::from("sqlx::types::time::Time"),

//             Self::SqlxTypesUuidUuidAsPostgresqlUuid => std::string::String::from("sqlx::types::uuid::Uuid"),
//             Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => std::string::String::from("sqlx::types::uuid::Uuid"),
//             Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => std::string::String::from("sqlx::types::uuid::Uuid"),

//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => std::string::String::from("sqlx::types::ipnetwork::IpNetwork"),
//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => std::string::String::from("sqlx::types::ipnetwork::IpNetwork"),
//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => std::string::String::from("sqlx::types::ipnetwork::IpNetwork"),
//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => std::string::String::from("sqlx::types::ipnetwork::IpNetwork"),

//             Self::StdNetIpAddrAsPostgresqlInet => std::string::String::from("std::net::IpAddr"),
//             Self::StdNetIpAddrAsPostgresqlInetNotNull => std::string::String::from("std::net::IpAddr"),
//             Self::StdNetIpAddrAsPostgresqlCidr => std::string::String::from("std::net::IpAddr"),
//             Self::StdNetIpAddrAsPostgresqlCidrNotNull => std::string::String::from("std::net::IpAddr"),

//             Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => std::string::String::from("sqlx::types::mac_address::MacAddress"),
//             Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => std::string::String::from("sqlx::types::mac_address::MacAddress"),

//             Self::SqlxTypesBitVecAsPostgresqlBit => std::string::String::from("sqlx::types::BitVec"),
//             Self::SqlxTypesBitVecAsPostgresqlBitNotNull => std::string::String::from("sqlx::types::BitVec"),
//             Self::SqlxTypesBitVecAsPostgresqlVarBit => std::string::String::from("sqlx::types::BitVec"),
//             Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull => std::string::String::from("sqlx::types::BitVec"),

//             Self::SqlxTypesJsonTAsPostgresqlJson => format!("sqlx::types::Json<{generic_type_str}>"),
//             Self::SqlxTypesJsonTAsPostgresqlJsonNotNull => format!("sqlx::types::Json<{generic_type_str}>"),
//             Self::SqlxTypesJsonTAsPostgresqlJsonB => format!("sqlx::types::Json<{generic_type_str}>"),
//             Self::SqlxTypesJsonTAsPostgresqlJsonBNotNull => format!("sqlx::types::Json<{generic_type_str}>"),

//             Self::SerdeJsonValueAsPostgresqlJson => std::string::String::from("serde_json::Value"),
//             Self::SerdeJsonValueAsPostgresqlJsonNotNull => std::string::String::from("serde_json::Value"),
//             Self::SerdeJsonValueAsPostgresqlJsonB => std::string::String::from("serde_json::Value"),
//             Self::SerdeJsonValueAsPostgresqlJsonBNotNull => std::string::String::from("serde_json::Value"),
//         }
//     }
// }
//
//todo rename conversion method
impl std::convert::TryFrom<&str> for RustSqlxMapToPostgresTypeVariant {
    type Error = std::string::String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "postgresql_crud::StdPrimitiveBoolAsPostgresqlBool" => Ok(Self::StdPrimitiveBoolAsPostgresqlBool),
            "postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull" => Ok(Self::StdPrimitiveBoolAsPostgresqlBoolNotNull),

            "postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallInt),
            "postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull),
            "postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerial),
            "postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull),
            "postgresql_crud::StdPrimitiveI16AsPostgresqlInt2" => Ok(Self::StdPrimitiveI16AsPostgresqlInt2),
            "postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull" => Ok(Self::StdPrimitiveI16AsPostgresqlInt2NotNull),

            "postgresql_crud::StdPrimitiveI32AsPostgresqlInt" => Ok(Self::StdPrimitiveI32AsPostgresqlInt),
            "postgresql_crud::StdPrimitiveI32AsPostgresqlIntNotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlIntNotNull),
            "postgresql_crud::StdPrimitiveI32AsPostgresqlSerial" => Ok(Self::StdPrimitiveI32AsPostgresqlSerial),
            "postgresql_crud::StdPrimitiveI32AsPostgresqlSerialNotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlSerialNotNull),
            "postgresql_crud::StdPrimitiveI32AsPostgresqlInt4" => Ok(Self::StdPrimitiveI32AsPostgresqlInt4),
            "postgresql_crud::StdPrimitiveI32AsPostgresqlInt4NotNull" => Ok(Self::StdPrimitiveI32AsPostgresqlInt4NotNull),

            "postgresql_crud::StdPrimitiveI64AsPostgresqlBigInt" => Ok(Self::StdPrimitiveI64AsPostgresqlBigInt),
            "postgresql_crud::StdPrimitiveI64AsPostgresqlBigIntNotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlBigIntNotNull),
            "postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerial" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerial),
            "postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull),
            "postgresql_crud::StdPrimitiveI64AsPostgresqlInt8" => Ok(Self::StdPrimitiveI64AsPostgresqlInt8),
            "postgresql_crud::StdPrimitiveI64AsPostgresqlInt8NotNull" => Ok(Self::StdPrimitiveI64AsPostgresqlInt8NotNull),

            "postgresql_crud::StdPrimitiveF32AsPostgresqlReal" => Ok(Self::StdPrimitiveF32AsPostgresqlReal),
            "postgresql_crud::StdPrimitiveF32AsPostgresqlRealNotNull" => Ok(Self::StdPrimitiveF32AsPostgresqlRealNotNull),
            "postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4" => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4),
            "postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4NotNull" => Ok(Self::StdPrimitiveF32AsPostgresqlFloat4NotNull),

            "postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecision" => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecision),
            "postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull" => Ok(Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull),
            "postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8" => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8),
            "postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8NotNull" => Ok(Self::StdPrimitiveF64AsPostgresqlFloat8NotNull),

            "postgresql_crud::StdStringStringAsPostgresqlVarchar" => Ok(Self::StdStringStringAsPostgresqlVarchar),
            "postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull" => Ok(Self::StdStringStringAsPostgresqlVarcharNotNull),
            "postgresql_crud::StdStringStringAsPostgresqlCharN" => Ok(Self::StdStringStringAsPostgresqlCharN),
            "postgresql_crud::StdStringStringAsPostgresqlCharNNotNull" => Ok(Self::StdStringStringAsPostgresqlCharNNotNull),
            "postgresql_crud::StdStringStringAsPostgresqlText" => Ok(Self::StdStringStringAsPostgresqlText),
            "postgresql_crud::StdStringStringAsPostgresqlTextNotNull" => Ok(Self::StdStringStringAsPostgresqlTextNotNull),
            "postgresql_crud::StdStringStringAsPostgresqlName" => Ok(Self::StdStringStringAsPostgresqlName),
            "postgresql_crud::StdStringStringAsPostgresqlNameNotNull" => Ok(Self::StdStringStringAsPostgresqlNameNotNull),
            "postgresql_crud::StdStringStringAsPostgresqlCiText" => Ok(Self::StdStringStringAsPostgresqlCiText),
            "postgresql_crud::StdStringStringAsPostgresqlCiTextNotNull" => Ok(Self::StdStringStringAsPostgresqlCiTextNotNull),

            "postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlBytea" => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea),
            "postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull" => Ok(Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull),

            "postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlInterval" => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval),
            "postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull" => Ok(Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range),
            "postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range),
            "postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange),
            "postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoney" => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney),
            "postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull" => Ok(Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull),

            "postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiText" => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText),
            "postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull" => Ok(Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull),

            "postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumeric" => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumeric),
            "postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumericNotNull" => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull),

            "postgresql_crud::SqlxTypesDecimalAsPostgresqlNumeric" => Ok(Self::SqlxTypesDecimalAsPostgresqlNumeric),
            "postgresql_crud::SqlxTypesDecimalAsPostgresqlNumericNotNull" => Ok(Self::SqlxTypesDecimalAsPostgresqlNumericNotNull),

            "postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp),
            "postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull),

            "postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz),
            "postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull),

            "postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp" => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),
            "postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull),

            "postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDate" => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDate),
            "postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull" => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull),

            "postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTime" => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime),
            "postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull" => Ok(Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull),

            "postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz" => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz),
            "postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull" => Ok(Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull),

            "postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp" => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),
            "postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull),

            "postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz" => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz),
            "postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull" => Ok(Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull),

            "postgresql_crud::SqlxTypesTimeDateAsPostgresqlDate" => Ok(Self::SqlxTypesTimeDateAsPostgresqlDate),
            "postgresql_crud::SqlxTypesTimeDateAsPostgresqlDateNotNull" => Ok(Self::SqlxTypesTimeDateAsPostgresqlDateNotNull),

            "postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime" => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTime),
            "postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTimeNotNull" => Ok(Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull),

            "postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuid" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuid),
            "postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNull" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull),
            "postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey" => Ok(Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey),

            "postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet),
            "postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull),
            "postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr),
            "postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull" => Ok(Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull),

            "postgresql_crud::StdNetIpAddrAsPostgresqlInet" => Ok(Self::StdNetIpAddrAsPostgresqlInet),
            "postgresql_crud::StdNetIpAddrAsPostgresqlInetNotNull" => Ok(Self::StdNetIpAddrAsPostgresqlInetNotNull),
            "postgresql_crud::StdNetIpAddrAsPostgresqlCidr" => Ok(Self::StdNetIpAddrAsPostgresqlCidr),
            "postgresql_crud::StdNetIpAddrAsPostgresqlCidrNotNull" => Ok(Self::StdNetIpAddrAsPostgresqlCidrNotNull),

            "postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr" => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr),
            "postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull" => Ok(Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull),

            "postgresql_crud::SqlxTypesBitVecAsPostgresqlBit" => Ok(Self::SqlxTypesBitVecAsPostgresqlBit),
            "postgresql_crud::SqlxTypesBitVecAsPostgresqlBitNotNull" => Ok(Self::SqlxTypesBitVecAsPostgresqlBitNotNull),
            "postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBit" => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBit),
            "postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBitNotNull" => Ok(Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull),

            //todo what to do with generic?
            "postgresql_crud::SqlxTypesJsonTAsPostgresqlJson" => Ok(Self::SqlxTypesJsonTAsPostgresqlJson),
            "postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonNotNull" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonNotNull),
            "postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonB" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonB),
            "postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonBNotNull" => Ok(Self::SqlxTypesJsonTAsPostgresqlJsonBNotNull),

            "postgresql_crud::SerdeJsonValueAsPostgresqlJson" => Ok(Self::SerdeJsonValueAsPostgresqlJson),
            "postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonNotNull),
            "postgresql_crud::SerdeJsonValueAsPostgresqlJsonB" => Ok(Self::SerdeJsonValueAsPostgresqlJsonB),
            "postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull" => Ok(Self::SerdeJsonValueAsPostgresqlJsonBNotNull),
            _ => Err(format!(
                "unsupported value: {value}, {:?}",
                Self::into_array().into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<std::string::String>>()
            ))
        }
    }
}
//todo maybe remove later. impl only for migration to different type
// impl std::fmt::Display for RustSqlxMapToPostgresTypeVariant {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             Self::StdPrimitiveBoolAsPostgresqlBool => write!(f, "StdPrimitiveBoolAsPostgresqlBool"),
//             Self::StdPrimitiveBoolAsPostgresqlBoolNotNull => write!(f, "StdPrimitiveBoolAsPostgresqlBoolNotNull"),

//             Self::StdPrimitiveI16AsPostgresqlSmallInt => write!(f, "StdPrimitiveI16AsPostgresqlSmallInt"),
//             Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull => write!(f, "StdPrimitiveI16AsPostgresqlSmallIntNotNull"),
//             Self::StdPrimitiveI16AsPostgresqlSmallSerial => write!(f, "StdPrimitiveI16AsPostgresqlSmallSerial"),
//             Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => write!(f, "StdPrimitiveI16AsPostgresqlSmallSerialNotNull"),
//             Self::StdPrimitiveI16AsPostgresqlInt2 => write!(f, "StdPrimitiveI16AsPostgresqlInt2"),
//             Self::StdPrimitiveI16AsPostgresqlInt2NotNull => write!(f, "StdPrimitiveI16AsPostgresqlInt2NotNull"),

//             Self::StdPrimitiveI32AsPostgresqlInt => write!(f, "StdPrimitiveI32AsPostgresqlInt"),
//             Self::StdPrimitiveI32AsPostgresqlIntNotNull => write!(f, "StdPrimitiveI32AsPostgresqlIntNotNull"),
//             Self::StdPrimitiveI32AsPostgresqlSerial => write!(f, "StdPrimitiveI32AsPostgresqlSerial"),
//             Self::StdPrimitiveI32AsPostgresqlSerialNotNull => write!(f, "StdPrimitiveI32AsPostgresqlSerialNotNull"),
//             Self::StdPrimitiveI32AsPostgresqlInt4 => write!(f, "StdPrimitiveI32AsPostgresqlInt4"),
//             Self::StdPrimitiveI32AsPostgresqlInt4NotNull => write!(f, "StdPrimitiveI32AsPostgresqlInt4NotNull"),

//             Self::StdPrimitiveI64AsPostgresqlBigInt => write!(f, "StdPrimitiveI64AsPostgresqlBigInt"),
//             Self::StdPrimitiveI64AsPostgresqlBigIntNotNull => write!(f, "StdPrimitiveI64AsPostgresqlBigIntNotNull"),
//             Self::StdPrimitiveI64AsPostgresqlBigSerial => write!(f, "StdPrimitiveI64AsPostgresqlBigSerial"),
//             Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull => write!(f, "StdPrimitiveI64AsPostgresqlBigSerialNotNull"),
//             Self::StdPrimitiveI64AsPostgresqlInt8 => write!(f, "StdPrimitiveI64AsPostgresqlInt8"),
//             Self::StdPrimitiveI64AsPostgresqlInt8NotNull => write!(f, "StdPrimitiveI64AsPostgresqlInt8NotNull"),

//             Self::StdPrimitiveF32AsPostgresqlReal => write!(f, "StdPrimitiveF32AsPostgresqlReal"),
//             Self::StdPrimitiveF32AsPostgresqlRealNotNull => write!(f, "StdPrimitiveF32AsPostgresqlRealNotNull"),
//             Self::StdPrimitiveF32AsPostgresqlFloat4 => write!(f, "StdPrimitiveF32AsPostgresqlFloat4"),
//             Self::StdPrimitiveF32AsPostgresqlFloat4NotNull => write!(f, "StdPrimitiveF32AsPostgresqlFloat4NotNull"),

//             Self::StdPrimitiveF64AsPostgresqlDoublePrecision => write!(f, "StdPrimitiveF64AsPostgresqlDoublePrecision"),
//             Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => write!(f, "StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull"),
//             Self::StdPrimitiveF64AsPostgresqlFloat8 => write!(f, "StdPrimitiveF64AsPostgresqlFloat"),
//             Self::StdPrimitiveF64AsPostgresqlFloat8NotNull => write!(f, "StdPrimitiveF64AsPostgresqlFloat8NotNull"),

//             Self::StdStringStringAsPostgresqlVarchar => write!(f, "StdStringStringAsPostgresqlVarchar"),
//             Self::StdStringStringAsPostgresqlVarcharNotNull => write!(f, "StdStringStringAsPostgresqlVarcharNotNull"),
//             Self::StdStringStringAsPostgresqlCharN => write!(f, "StdStringStringAsPostgresqlCharN"),
//             Self::StdStringStringAsPostgresqlCharNNotNull => write!(f, "StdStringStringAsPostgresqlCharNNotNull"),
//             Self::StdStringStringAsPostgresqlText => write!(f, "StdStringStringAsPostgresqlText"),
//             Self::StdStringStringAsPostgresqlTextNotNull => write!(f, "StdStringStringAsPostgresqlTextNotNull"),
//             Self::StdStringStringAsPostgresqlName => write!(f, "StdStringStringAsPostgresqlName"),
//             Self::StdStringStringAsPostgresqlNameNotNull => write!(f, "StdStringStringAsPostgresqlNameNotNull"),
//             Self::StdStringStringAsPostgresqlCiText => write!(f, "StdStringStringAsPostgresqlCiText"),
//             Self::StdStringStringAsPostgresqlCiTextNotNull => write!(f, "StdStringStringAsPostgresqlCiTextNotNull"),

//             Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea => write!(f, "StdVecVecStdPrimitiveU8AsPostgresqlBytea"),
//             Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => write!(f, "StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull"),

//             Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => write!(f, "SqlxPostgresTypesPgIntervalAsPostgresqlInterval"),
//             Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => write!(f, "SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull"),

//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => write!(f, "SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range"),
//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => write!(f, "SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => write!(f, "SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range"),
//             Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => write!(f, "SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull"),

//             Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange"),
//             Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => write!(f, "SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull"),

//             Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => write!(f, "SqlxPostgresTypesPgMoneyAsPostgresqlMoney"),
//             Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => write!(f, "SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull"),

//             Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => write!(f, "SqlxPostgresTypesPgCiTextAsPostgresqlCiText"),
//             Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => write!(f, "SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull"),

//             Self::SqlxTypesBigDecimalAsPostgresqlNumeric => write!(f, "SqlxTypesBigDecimalAsPostgresqlNumeric"),
//             Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => write!(f, "SqlxTypesBigDecimalAsPostgresqlNumericNotNull"),

//             Self::SqlxTypesDecimalAsPostgresqlNumeric => write!(f, "SqlxTypesDecimalAsPostgresqlNumeric"),
//             Self::SqlxTypesDecimalAsPostgresqlNumericNotNull => write!(f, "SqlxTypesDecimalAsPostgresqlNumericNotNull"),

//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp => write!(f, "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp"),
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull => write!(f, "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull"),

//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => write!(f, "SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz"),
//             Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => write!(f, "SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull"),

//             Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => write!(f, "SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp"),
//             Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => write!(f, "SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull"),

//             Self::SqlxTypesChronoNaiveDateAsPostgresqlDate => write!(f, "SqlxTypesChronoNaiveDateAsPostgresqlDate"),
//             Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => write!(f, "SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull"),

//             Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime => write!(f, "SqlxTypesChronoNaiveTimeAsPostgresqlTime"),
//             Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => write!(f, "SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull"),

//             Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => write!(f, "SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz"),
//             Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => write!(f, "SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull"),

//             Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => write!(f, "SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp"),
//             Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => write!(f, "SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull"),

//             Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => write!(f, "SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz"),
//             Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => write!(f, "SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull"),

//             Self::SqlxTypesTimeDateAsPostgresqlDate => write!(f, "SqlxTypesTimeDateAsPostgresqlDate"),
//             Self::SqlxTypesTimeDateAsPostgresqlDateNotNull => write!(f, "SqlxTypesTimeDateAsPostgresqlDateNotNull"),

//             Self::SqlxTypesTimeTimeAsPostgresqlTime => write!(f, "SqlxTypesTimeTimeAsPostgresqlTime"),
//             Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => write!(f, "SqlxTypesTimeTimeAsPostgresqlTimeNotNull"),

//             Self::SqlxTypesUuidUuidAsPostgresqlUuid => write!(f, "SqlxTypesUuidUuidAsPostgresqlUuid"),
//             Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => write!(f, "SqlxTypesUuidUuidAsPostgresqlUuidNotNull"),
//             Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => write!(f, "SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey"),

//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => write!(f, "SqlxTypesIpnetworkIpNetworkAsPostgresqlInet"),
//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => write!(f, "SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull"),
//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => write!(f, "SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr"),
//             Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => write!(f, "SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull"),

//             Self::StdNetIpAddrAsPostgresqlInet => write!(f, "StdNetIpAddrAsPostgresqlInet"),
//             Self::StdNetIpAddrAsPostgresqlInetNotNull => write!(f, "StdNetIpAddrAsPostgresqlInetNotNull"),
//             Self::StdNetIpAddrAsPostgresqlCidr => write!(f, "StdNetIpAddrAsPostgresqlCidr"),
//             Self::StdNetIpAddrAsPostgresqlCidrNotNull => write!(f, "StdNetIpAddrAsPostgresqlCidrNotNull"),

//             Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => write!(f, "SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr"),
//             Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => write!(f, "SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull"),

//             Self::SqlxTypesBitVecAsPostgresqlBit => write!(f, "SqlxTypesBitVecAsPostgresqlBit"),
//             Self::SqlxTypesBitVecAsPostgresqlBitNotNull => write!(f, "SqlxTypesBitVecAsPostgresqlBitNotNull"),
//             Self::SqlxTypesBitVecAsPostgresqlVarBit => write!(f, "SqlxTypesBitVecAsPostgresqlVarBit"),
//             Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull => write!(f, "SqlxTypesBitVecAsPostgresqlVarBitNotNull"),

//             //todo what to do with generic?
//             Self::SqlxTypesJsonTAsPostgresqlJson => write!(f, "SqlxTypesJsonTAsPostgresqlJson"),
//             Self::SqlxTypesJsonTAsPostgresqlJsonNotNull => write!(f, "SqlxTypesJsonTAsPostgresqlJsonNotNull"),
//             Self::SqlxTypesJsonTAsPostgresqlJsonB => write!(f, "SqlxTypesJsonTAsPostgresqlJsonB"),
//             Self::SqlxTypesJsonTAsPostgresqlJsonBNotNull => write!(f, "SqlxTypesJsonTAsPostgresqlJsonBNotNull"),

//             Self::SerdeJsonValueAsPostgresqlJson => write!(f, "SerdeJsonValueAsPostgresqlJson"),
//             Self::SerdeJsonValueAsPostgresqlJsonNotNull => write!(f, "SerdeJsonValueAsPostgresqlJsonNotNull"),
//             Self::SerdeJsonValueAsPostgresqlJsonB => write!(f, "SerdeJsonValueAsPostgresqlJsonB"),
//             Self::SerdeJsonValueAsPostgresqlJsonBNotNull => write!(f, "SerdeJsonValueAsPostgresqlJsonBNotNull"),
//         }
//     }
// }
//todo maybe remove later. impl only for migration to different type
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
            "StdStringStringAsPostgresqlName" => Ok(Self::StdStringStringAsPostgresqlName),
            "StdStringStringAsPostgresqlNameNotNull" => Ok(Self::StdStringStringAsPostgresqlNameNotNull),
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

            "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp),
            "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull),

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
#[derive(Debug)]
pub struct StdPrimitiveBoolAsPostgresqlBool(pub StdPrimitiveBool);
//todo maybe make it a trait, but need to specify generics
impl StdPrimitiveBoolAsPostgresqlBool {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveBoolAsPostgresqlBool {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(pub StdPrimitiveBool);
impl StdPrimitiveBoolAsPostgresqlBoolNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI16AsPostgresqlSmallInt(pub StdPrimitiveI16);
impl StdPrimitiveI16AsPostgresqlSmallInt {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i16> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI16AsPostgresqlSmallInt {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI16AsPostgresqlSmallIntNotNull(pub StdPrimitiveI16);
impl StdPrimitiveI16AsPostgresqlSmallIntNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i16> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI16AsPostgresqlSmallIntNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerial(pub StdPrimitiveI16);
impl StdPrimitiveI16AsPostgresqlSmallSerial {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i16> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI16AsPostgresqlSmallSerial {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(pub StdPrimitiveI16);
impl StdPrimitiveI16AsPostgresqlSmallSerialNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i16> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI16AsPostgresqlSmallSerialNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI16AsPostgresqlInt2(pub StdPrimitiveI16);
impl StdPrimitiveI16AsPostgresqlInt2 {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i16> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI16AsPostgresqlInt2 {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(pub StdPrimitiveI16);
impl StdPrimitiveI16AsPostgresqlInt2NotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i16> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI16AsPostgresqlInt2NotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI32AsPostgresqlInt(pub StdPrimitiveI32);
impl StdPrimitiveI32AsPostgresqlInt {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI32AsPostgresqlInt {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI32AsPostgresqlIntNotNull(pub StdPrimitiveI32);
impl StdPrimitiveI32AsPostgresqlIntNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI32AsPostgresqlIntNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI32AsPostgresqlSerial(pub StdPrimitiveI32);
impl StdPrimitiveI32AsPostgresqlSerial {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI32AsPostgresqlSerial {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(pub StdPrimitiveI32);
impl StdPrimitiveI32AsPostgresqlSerialNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI32AsPostgresqlSerialNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI32AsPostgresqlInt4(pub StdPrimitiveI32);
impl StdPrimitiveI32AsPostgresqlInt4 {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI32AsPostgresqlInt4 {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(pub StdPrimitiveI32);
impl StdPrimitiveI32AsPostgresqlInt4NotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI32AsPostgresqlInt4NotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI64AsPostgresqlBigInt(pub StdPrimitiveI64);
impl StdPrimitiveI64AsPostgresqlBigInt {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlBigInt {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI64AsPostgresqlBigIntNotNull(pub StdPrimitiveI64);
impl StdPrimitiveI64AsPostgresqlBigIntNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlBigIntNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI64AsPostgresqlBigSerial(pub StdPrimitiveI64);
impl StdPrimitiveI64AsPostgresqlBigSerial {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlBigSerial {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(pub StdPrimitiveI64);
impl StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI64AsPostgresqlInt8(pub StdPrimitiveI64);
impl StdPrimitiveI64AsPostgresqlInt8 {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlInt8 {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(pub StdPrimitiveI64);
impl StdPrimitiveI64AsPostgresqlInt8NotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlInt8NotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF32AsPostgresqlReal(pub StdPrimitiveF32);
impl StdPrimitiveF32AsPostgresqlReal {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF32AsPostgresqlReal {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF32AsPostgresqlRealNotNull(pub StdPrimitiveF32);
impl StdPrimitiveF32AsPostgresqlRealNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF32AsPostgresqlRealNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF32AsPostgresqlFloat4(pub StdPrimitiveF32);
impl StdPrimitiveF32AsPostgresqlFloat4 {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF32AsPostgresqlFloat4 {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(pub StdPrimitiveF32);
impl StdPrimitiveF32AsPostgresqlFloat4NotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f32> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF32AsPostgresqlFloat4NotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF64AsPostgresqlDoublePrecision(pub StdPrimitiveF64);
impl StdPrimitiveF64AsPostgresqlDoublePrecision {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF64AsPostgresqlDoublePrecision {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull(pub StdPrimitiveF64);
impl StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF64AsPostgresqlFloat8(pub StdPrimitiveF64);
impl StdPrimitiveF64AsPostgresqlFloat8 {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF64AsPostgresqlFloat8 {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(pub StdPrimitiveF64);
impl StdPrimitiveF64AsPostgresqlFloat8NotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::f64> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveF64AsPostgresqlFloat8NotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlVarchar(pub StdStringString);
impl StdStringStringAsPostgresqlVarchar {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlVarchar {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlVarcharNotNull(pub StdStringString);
impl StdStringStringAsPostgresqlVarcharNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlVarcharNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlCharN(pub StdStringString);
impl StdStringStringAsPostgresqlCharN {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlCharN {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlCharNNotNull(pub StdStringString);
impl StdStringStringAsPostgresqlCharNNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlCharNNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlText(pub StdStringString);
impl StdStringStringAsPostgresqlText {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlText {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlTextNotNull(pub StdStringString);
impl StdStringStringAsPostgresqlTextNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlTextNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlName(pub StdStringString);
impl StdStringStringAsPostgresqlName {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlName {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlNameNotNull(pub StdStringString);
impl StdStringStringAsPostgresqlNameNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlNameNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlCiText(pub StdStringString);
impl StdStringStringAsPostgresqlCiText {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlCiText {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdStringStringAsPostgresqlCiTextNotNull(pub StdStringString);
impl StdStringStringAsPostgresqlCiTextNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::string::String> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdStringStringAsPostgresqlCiTextNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlBytea(pub StdVecVecStdPrimitiveU8);
impl StdVecVecStdPrimitiveU8AsPostgresqlBytea {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::vec::Vec<std::primitive::u8>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdVecVecStdPrimitiveU8AsPostgresqlBytea {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull(pub StdVecVecStdPrimitiveU8);
impl StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::vec::Vec<std::primitive::u8>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlInterval(pub SqlxPostgresTypesPgInterval);
impl SqlxPostgresTypesPgIntervalAsPostgresqlInterval {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgInterval> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgIntervalAsPostgresqlInterval {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull(pub SqlxPostgresTypesPgInterval);
impl SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgInterval> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range(pub SqlxPostgresTypesPgRangeStdPrimitiveI64);
impl SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i64>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull(pub SqlxPostgresTypesPgRangeStdPrimitiveI64);
impl SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i64>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range(pub SqlxPostgresTypesPgRangeStdPrimitiveI32);
impl SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i32>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull(pub SqlxPostgresTypesPgRangeStdPrimitiveI32);
impl SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i32>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange(pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange(pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange(pub SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange(pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange(pub SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
impl SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
impl SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange(pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange(pub SqlxPostgresTypesPgRangeSqlxTypesTimeDate);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::time::Date>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesTimeDate);
impl SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::time::Date>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange(pub SqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange(pub SqlxPostgresTypesPgRangeSqlxTypesDecimal);
impl SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::Decimal>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull(pub SqlxPostgresTypesPgRangeSqlxTypesDecimal);
impl SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::Decimal>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoney(pub SqlxPostgresTypesPgMoney);
impl SqlxPostgresTypesPgMoneyAsPostgresqlMoney {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgMoney> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgMoneyAsPostgresqlMoney {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull(pub SqlxPostgresTypesPgMoney);
impl SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgMoney> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgCiTextAsPostgresqlCiText(pub SqlxPostgresTypesPgCiText);
impl SqlxPostgresTypesPgCiTextAsPostgresqlCiText {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgCiText> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgCiTextAsPostgresqlCiText {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull(pub SqlxPostgresTypesPgCiText);
impl SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgCiText> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumeric(pub SqlxTypesBigDecimal);
impl SqlxTypesBigDecimalAsPostgresqlNumeric {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::BigDecimal> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesBigDecimalAsPostgresqlNumeric {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumericNotNull(pub SqlxTypesBigDecimal);
impl SqlxTypesBigDecimalAsPostgresqlNumericNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::BigDecimal> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesBigDecimalAsPostgresqlNumericNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesDecimalAsPostgresqlNumeric(pub SqlxTypesDecimal);
impl SqlxTypesDecimalAsPostgresqlNumeric {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Decimal> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesDecimalAsPostgresqlNumeric {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesDecimalAsPostgresqlNumericNotNull(pub SqlxTypesDecimal);
impl SqlxTypesDecimalAsPostgresqlNumericNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Decimal> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesDecimalAsPostgresqlNumericNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp(pub SqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestamp {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull(pub SqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz(pub SqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull(pub SqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp(pub SqlxTypesChronoNaiveDateTime);
impl SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::NaiveDateTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull(pub SqlxTypesChronoNaiveDateTime);
impl SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::NaiveDateTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDate(pub SqlxTypesChronoNaiveDate);
impl SqlxTypesChronoNaiveDateAsPostgresqlDate {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::NaiveDate> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoNaiveDateAsPostgresqlDate {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull(pub SqlxTypesChronoNaiveDate);
impl SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::NaiveDate> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTime(pub SqlxTypesChronoNaiveTime);
impl SqlxTypesChronoNaiveTimeAsPostgresqlTime {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::NaiveTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoNaiveTimeAsPostgresqlTime {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull(pub SqlxTypesChronoNaiveTime);
impl SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::chrono::NaiveTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz(pub SqlxPostgresTypesPgTimeTz);
impl SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgTimeTz> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull(pub SqlxPostgresTypesPgTimeTz);
impl SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::postgres::types::PgTimeTz> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp(pub SqlxTypesTimePrimitiveDateTime);
impl SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::PrimitiveDateTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull(pub SqlxTypesTimePrimitiveDateTime);
impl SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::PrimitiveDateTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz(pub SqlxTypesTimeOffsetDateTime);
impl SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::OffsetDateTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull(pub SqlxTypesTimeOffsetDateTime);
impl SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::OffsetDateTime> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimeDateAsPostgresqlDate(pub SqlxTypesTimeDate);
impl SqlxTypesTimeDateAsPostgresqlDate {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::Date> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimeDateAsPostgresqlDate {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimeDateAsPostgresqlDateNotNull(pub SqlxTypesTimeDate);
impl SqlxTypesTimeDateAsPostgresqlDateNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::Date> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimeDateAsPostgresqlDateNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimeTimeAsPostgresqlTime(pub SqlxTypesTimeTime);
impl SqlxTypesTimeTimeAsPostgresqlTime {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::Time> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimeTimeAsPostgresqlTime {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesTimeTimeAsPostgresqlTimeNotNull(pub SqlxTypesTimeTime);
impl SqlxTypesTimeTimeAsPostgresqlTimeNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::time::Time> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesTimeTimeAsPostgresqlTimeNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuid(pub SqlxTypesUuidUuid);
impl SqlxTypesUuidUuidAsPostgresqlUuid {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::uuid::Uuid> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesUuidUuidAsPostgresqlUuid {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuidNotNull(pub SqlxTypesUuidUuid);
impl SqlxTypesUuidUuidAsPostgresqlUuidNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::uuid::Uuid> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesUuidUuidAsPostgresqlUuidNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey(pub SqlxTypesUuidUuid);
impl SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::uuid::Uuid> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInet(pub SqlxTypesIpnetworkIpNetwork);
impl SqlxTypesIpnetworkIpNetworkAsPostgresqlInet {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::ipnetwork::IpNetwork> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesIpnetworkIpNetworkAsPostgresqlInet {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull(pub SqlxTypesIpnetworkIpNetwork);
impl SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::ipnetwork::IpNetwork> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr(pub SqlxTypesIpnetworkIpNetwork);
impl SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::ipnetwork::IpNetwork> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull(pub SqlxTypesIpnetworkIpNetwork);
impl SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::ipnetwork::IpNetwork> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdNetIpAddrAsPostgresqlInet(pub StdNetIpAddr);
impl StdNetIpAddrAsPostgresqlInet {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::net::IpAddr> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdNetIpAddrAsPostgresqlInet {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdNetIpAddrAsPostgresqlInetNotNull(pub StdNetIpAddr);
impl StdNetIpAddrAsPostgresqlInetNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::net::IpAddr> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdNetIpAddrAsPostgresqlInetNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdNetIpAddrAsPostgresqlCidr(pub StdNetIpAddr);
impl StdNetIpAddrAsPostgresqlCidr {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::net::IpAddr> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdNetIpAddrAsPostgresqlCidr {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct StdNetIpAddrAsPostgresqlCidrNotNull(pub StdNetIpAddr);
impl StdNetIpAddrAsPostgresqlCidrNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::net::IpAddr> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for StdNetIpAddrAsPostgresqlCidrNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr(pub SqlxTypesMacAddressMacAddress);
impl SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::mac_address::MacAddress> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull(pub SqlxTypesMacAddressMacAddress);
impl SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::mac_address::MacAddress> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesBitVecAsPostgresqlBit(pub SqlxTypesBitVec);
impl SqlxTypesBitVecAsPostgresqlBit {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::BitVec> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesBitVecAsPostgresqlBit {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesBitVecAsPostgresqlBitNotNull(pub SqlxTypesBitVec);
impl SqlxTypesBitVecAsPostgresqlBitNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::BitVec> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesBitVecAsPostgresqlBitNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesBitVecAsPostgresqlVarBit(pub SqlxTypesBitVec);
impl SqlxTypesBitVecAsPostgresqlVarBit {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::BitVec> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesBitVecAsPostgresqlVarBit {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesBitVecAsPostgresqlVarBitNotNull(pub SqlxTypesBitVec);
impl SqlxTypesBitVecAsPostgresqlVarBitNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::BitVec> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SqlxTypesBitVecAsPostgresqlVarBitNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
//todo what to do with generic?
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJson<T>(pub SqlxTypesJson<T>);
impl<T> SqlxTypesJsonTAsPostgresqlJson<T> {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJson<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonNotNull<T>(pub SqlxTypesJson<T>);
impl<T> SqlxTypesJsonTAsPostgresqlJsonNotNull<T> {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonB<T>(pub SqlxTypesJson<T>);
impl<T> SqlxTypesJsonTAsPostgresqlJsonB<T> {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonB<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SqlxTypesJsonTAsPostgresqlJsonBNotNull<T>(pub SqlxTypesJson<T>);
impl<T> SqlxTypesJsonTAsPostgresqlJsonBNotNull<T> {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<sqlx::types::Json<T>> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl<T> CheckSupportedRustAndPostgresqlColumnType for SqlxTypesJsonTAsPostgresqlJsonBNotNull<T> {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SerdeJsonValueAsPostgresqlJson(pub SerdeJsonValue);
impl SerdeJsonValueAsPostgresqlJson {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<serde_json::Value> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SerdeJsonValueAsPostgresqlJson {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SerdeJsonValueAsPostgresqlJsonNotNull(pub SerdeJsonValue);
impl SerdeJsonValueAsPostgresqlJsonNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<serde_json::Value> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SerdeJsonValueAsPostgresqlJsonNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SerdeJsonValueAsPostgresqlJsonB(pub SerdeJsonValue);
impl SerdeJsonValueAsPostgresqlJsonB {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<serde_json::Value> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SerdeJsonValueAsPostgresqlJsonB {
    fn check_supported_rust_and_postgresql_column_type() {}
}
#[derive(Debug)]
pub struct SerdeJsonValueAsPostgresqlJsonBNotNull(pub SerdeJsonValue);
impl SerdeJsonValueAsPostgresqlJsonBNotNull {
    pub fn into_inner_sqlx_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<serde_json::Value> {
        value.into_iter().map(|element|element.0.into_inner()).collect()
    }
}
impl CheckSupportedRustAndPostgresqlColumnType for SerdeJsonValueAsPostgresqlJsonBNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}

////////////////////////////////////////////////////////////////////////////////////////////////


// todo maybe wrap all errors into error occurence ?
// todo shared enum of postgres types for postgresql_crud and generate_postgresql_crud
// remove and make one
//todo support variations of init functions as enum

fn generate_sqlx_types_chrono_fixed_offset_east_opt_failed_message(fixed_offset: std::primitive::i32) -> std::string::String {
    format!("failed to create sqlx::types::chrono::FixedOffset with .east_opt {}", fixed_offset)
}

pub struct Test<T> {
    //https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
    std_primitive_bool: std::primitive::bool, //BOOL
    // std_primitive_i8: std::primitive::i8,   //“CHAR”//not clear how to make primary key from it
    std_primitive_i16: std::primitive::i16,  //SMALLINT, SMALLSERIAL, INT2
    std_primitive_i32: std::primitive::i32,  //INT, SERIAL, INT4
    std_primitive_i64: std::primitive::i64,  //BIGINT, BIGSERIAL, INT8
    std_primitive_f32: std::primitive::f32,  //REAL, FLOAT4
    std_primitive_f64: std::primitive::f64,  //DOUBLE PRECISION, FLOAT8
    // type_8: &std::primitive::str,//lifetimes are unexpectable i think //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    std_string_string: std::string::String, //VARCHAR, CHAR(N), TEXT, NAME, CITEXT
    // type_10: [std::primitive::u8;1],//ignoring coz deserialization problem//BYTEA
    std_vec_vec_std_primitive_u8: std::vec::Vec<std::primitive::u8>, //BYTEA
    // type_12: (),//didnt find Encode trait impl in sqlx//BYTEA
    sqlx_postgres_types_pg_interval: sqlx::postgres::types::PgInterval, //INTERVAL
    //INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
    sqlx_postgres_types_pg_range_std_primitive_i64: sqlx::postgres::types::PgRange<std::primitive::i64>, //INT8RANGE
    sqlx_postgres_types_pg_range_std_primitive_i32: sqlx::postgres::types::PgRange<std::primitive::i32>, //INT4RANGE
    // type_16: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>, //TSTZRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>, //TSTZRANGE
    // type_17: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>, //TSRANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>, //TSRANGE
    // type_18: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>, //DATERANGE
    sqlx_postgres_types_pg_range_sqlx_types_time_date: sqlx::postgres::types::PgRange<sqlx::types::time::Date>, //DATERANGE
    // type_19: sqlx::postgres::types::PgRange<Generic>,//maybe another impls//NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>, //NUMRANGE
    sqlx_postgres_types_pg_range_sqlx_types_decimal: sqlx::postgres::types::PgRange<sqlx::types::Decimal>,    //NUMRANGE
    sqlx_postgres_types_pg_money: sqlx::postgres::types::PgMoney,                           //MONEY
    // sqlx_postgres_types_pg_l_tree: sqlx::postgres::types::PgLTree,//LTREE//dont want to support that for postgresql_crud
    // sqlx_postgres_types_pg_l_query: sqlx::postgres::types::PgLQuery,//LQUERY//dont want to support that for postgresql_crud
    sqlx_postgres_types_pg_ci_text: sqlx::postgres::types::PgCiText,                          //CITEXT
    sqlx_types_big_decimal: sqlx::types::BigDecimal,                                  //NUMERIC
    sqlx_types_decimal: sqlx::types::Decimal,                                     //NUMERIC
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,  //TIMESTAMP
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>, //TIMESTAMPTZ
    sqlx_types_chrono_naive_date_time: sqlx::types::chrono::NaiveDateTime,//TIMESTAMP
    sqlx_types_chrono_naive_date: sqlx::types::chrono::NaiveDate,                           //DATE
    sqlx_types_chrono_naive_time: sqlx::types::chrono::NaiveTime,                           //TIME
    sqlx_postgres_types_pg_time_tz: sqlx::postgres::types::PgTimeTz, //just present chrono or time flag
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag chrono//TIMETZ
    sqlx_types_time_primitive_date_time: sqlx::types::time::PrimitiveDateTime, //TIMESTAMP
    sqlx_types_time_offset_date_time: sqlx::types::time::OffsetDateTime,    //TIMESTAMPTZ
    sqlx_types_time_date: sqlx::types::time::Date,              //DATE
    sqlx_types_time_time: sqlx::types::time::Time,              //TIME
    // type_: sqlx::postgres::types::PgTimeTz,//feature flag time//TIMETZ
    sqlx_types_uuid_uuid: sqlx::types::uuid::Uuid,              //UUID
    sqlx_types_ipnetwork_ip_network: sqlx::types::ipnetwork::IpNetwork,    //INET, CIDR
    std_net_ip_addr: std::net::IpAddr,                     //INET, CIDR
    sqlx_types_mac_address_mac_address: sqlx::types::mac_address::MacAddress, //MACADDR
    sqlx_types_bit_vec: sqlx::types::BitVec,                  //BIT, VARBIT
    sqlx_types_json: sqlx::types::Json<T>,                 //JSON, JSONB
    serde_json_value: serde_json::Value,                    //JSON, JSONB
    // type_44: serde_json::value::RawValue,//lifetime and borrow problem//JSON, JSONB
    //maybe Composite types
    //maybe Enumerations
}

pub struct TestNewType<T> {
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
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoney,
    sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiText,
    sqlx_types_big_decimal: SqlxTypesBigDecimal,
    sqlx_types_decimal: SqlxTypesDecimal,
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
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
    std_primitive_bool: StdPrimitiveBool,
    std_primitive_i16: StdPrimitiveI16,
    std_primitive_i32: StdPrimitiveI32,
    std_primitive_i64: StdPrimitiveI64,
    std_primitive_f32: StdPrimitiveF32,
    std_primitive_f64: StdPrimitiveF64,
    std_string_string: StdStringString,
    std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8,
    sqlx_postgres_types_pg_interval: SqlxPostgresTypesPgIntervalWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_std_primitive_i64: SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize,
    sqlx_postgres_types_pg_range_std_primitive_i32: SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize,//todo maybe naming
    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_time_date: SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_big_decimal: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize,
    sqlx_postgres_types_pg_range_sqlx_types_decimal: SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize,
    sqlx_postgres_types_pg_money: SqlxPostgresTypesPgMoneyWithSerializeDeserialize,
    sqlx_postgres_types_pg_ci_text: SqlxPostgresTypesPgCiTextWithSerializeDeserialize,
    sqlx_types_big_decimal: SqlxTypesBigDecimalNewWithSerializeDeserialize,
    sqlx_types_decimal: SqlxTypesDecimal,
    sqlx_types_chrono_date_time_sqlx_types_chrono_utc: SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize,
    sqlx_types_chrono_date_time_sqlx_types_chrono_local: SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize,
    sqlx_types_chrono_naive_date_time: SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize,
    sqlx_types_chrono_naive_date: SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize,
    sqlx_types_chrono_naive_time: SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize,
    sqlx_postgres_types_pg_time_tz: SqlxPostgresTypesPgTimeTzWithSerializeDeserialize,
    sqlx_types_time_primitive_date_time: SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize,
    sqlx_types_time_offset_date_time: SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize,
    sqlx_types_time_date: SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize,
    sqlx_types_time_time: SqlxTypesTimeTimeFromHmsWithSerializeDeserialize,
    sqlx_types_uuid_uuid: SqlxTypesUuidUuidTryParseWithSerializeDeserialize,
    sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork,
    std_net_ip_addr: StdNetIpAddr,
    sqlx_types_mac_address_mac_address: SqlxTypesMacAddressMacAddressNewWithSerializeDeserialize,
    sqlx_types_bit_vec: SqlxTypesBitVecFromBytesWithSerializeDeserialize,
    sqlx_types_json: SqlxTypesJson<T>,
    serde_json_value: SerdeJsonValue,
}

impl<T> std::convert::TryFrom<TestNewTypeWithSerializeDeserialize<T>> for TestNewType<T> {
    type Error = ();//todo
    fn try_from(value: TestNewTypeWithSerializeDeserialize<T>) -> Result<Self, Self::Error> {
        let std_primitive_bool: StdPrimitiveBool = value.std_primitive_bool;
        let std_primitive_i16: StdPrimitiveI16 = value.std_primitive_i16;
        let std_primitive_i32: StdPrimitiveI32 = value.std_primitive_i32;
        let std_primitive_i64: StdPrimitiveI64 = value.std_primitive_i64;
        let std_primitive_f32: StdPrimitiveF32 = value.std_primitive_f32;
        let std_primitive_f64: StdPrimitiveF64 = value.std_primitive_f64;
        let std_string_string: StdStringString = value.std_string_string;
        let std_vec_vec_std_primitive_u8: StdVecVecStdPrimitiveU8 = value.std_vec_vec_std_primitive_u8;
        let sqlx_postgres_types_pg_interval = SqlxPostgresTypesPgInterval::from(value.sqlx_postgres_types_pg_interval);
        let sqlx_postgres_types_pg_range_std_primitive_i64 = SqlxPostgresTypesPgRangeStdPrimitiveI64::from(value.sqlx_postgres_types_pg_range_std_primitive_i64);
        let sqlx_postgres_types_pg_range_std_primitive_i32 = SqlxPostgresTypesPgRangeStdPrimitiveI32::from(value.sqlx_postgres_types_pg_range_std_primitive_i32);
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc = match SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc) {
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
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time = match SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time = match SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time = match SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date = match SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_range_sqlx_types_time_date = match SqlxPostgresTypesPgRangeSqlxTypesTimeDate::try_from(value.sqlx_postgres_types_pg_range_sqlx_types_time_date) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_range_sqlx_types_big_decimal = SqlxPostgresTypesPgRangeSqlxTypesBigDecimal::from(value.sqlx_postgres_types_pg_range_sqlx_types_big_decimal);
        let sqlx_postgres_types_pg_range_sqlx_types_decimal = SqlxPostgresTypesPgRangeSqlxTypesDecimal::from(value.sqlx_postgres_types_pg_range_sqlx_types_decimal);
        let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney::from(value.sqlx_postgres_types_pg_money);
        let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText::from(value.sqlx_postgres_types_pg_ci_text);
        let sqlx_types_big_decimal = SqlxTypesBigDecimal::from(value.sqlx_types_big_decimal);
        let sqlx_types_decimal: SqlxTypesDecimal = value.sqlx_types_decimal;
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc = match SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(value.sqlx_types_chrono_date_time_sqlx_types_chrono_utc) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local = match SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(value.sqlx_types_chrono_date_time_sqlx_types_chrono_local) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_chrono_naive_date_time = match SqlxTypesChronoNaiveDateTime::try_from(value.sqlx_types_chrono_naive_date_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_chrono_naive_date = match SqlxTypesChronoNaiveDate::try_from(value.sqlx_types_chrono_naive_date) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_chrono_naive_time = match SqlxTypesChronoNaiveTime::try_from(value.sqlx_types_chrono_naive_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_postgres_types_pg_time_tz = match SqlxPostgresTypesPgTimeTz::try_from(value.sqlx_postgres_types_pg_time_tz) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_time_primitive_date_time = match SqlxTypesTimePrimitiveDateTime::try_from(value.sqlx_types_time_primitive_date_time) {
            Ok(value) => value,
            Err(_e) => {
                return Err(());
            }
        };
        let sqlx_types_time_offset_date_time = match SqlxTypesTimeOffsetDateTime::try_from(value.sqlx_types_time_offset_date_time) {
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
        let sqlx_types_ipnetwork_ip_network: SqlxTypesIpnetworkIpNetwork = value.sqlx_types_ipnetwork_ip_network;
        let std_net_ip_addr: StdNetIpAddr = value.std_net_ip_addr;
        let sqlx_types_mac_address_mac_address = SqlxTypesMacAddressMacAddress::from(value.sqlx_types_mac_address_mac_address);
        let sqlx_types_bit_vec = SqlxTypesBitVec::from(value.sqlx_types_bit_vec);
        let sqlx_types_json: SqlxTypesJson<T> = value.sqlx_types_json;
        let serde_json_value: SerdeJsonValue = value.serde_json_value;
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

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]//user type must implement utoipa::ToSchema trait
pub struct Something {
    something: std::string::String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum TimeMonthWithSerializeDeserialize {
    January,// = 1,
    February,// = 2,
    March,// = 3,
    April,// = 4,
    May,// = 5,
    June,// = 6,
    July,// = 7,
    August,// = 8,
    September,// = 9,
    October,// = 10,
    November,// = 11,
    December,// = 12,
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
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize {
    hours: std::primitive::i8,
    minutes: std::primitive::i8,
    seconds: std::primitive::i8
}
impl std::convert::TryFrom<SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize> for sqlx::types::time::UtcOffset {
    type Error = time::error::ComponentRange;//todo
    fn try_from(value: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match sqlx::types::time::UtcOffset::from_hms(
            value.hours,
            value.minutes,
            value.seconds
        ) {
            Ok(value) => Ok(value),
            Err(e) => Err(e)
        }
    }
}
impl std::convert::From<sqlx::types::time::UtcOffset> for SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize {
    fn from(value: sqlx::types::time::UtcOffset) -> Self {
        Self {
            hours: value.whole_hours(),
            minutes: value.minutes_past_hour(),
            seconds: value.seconds_past_minute()
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct NumBigintBigIntNewWithSerializeDeserialize {
    sign: NumBigintSignWithSerializeDeserialize,
    digits: std::vec::Vec<std::primitive::u32>
}
impl std::convert::From<NumBigintBigIntNewWithSerializeDeserialize> for num_bigint::BigInt {
    fn from(value: NumBigintBigIntNewWithSerializeDeserialize) -> Self {
        let sign = match value.sign {
            NumBigintSignWithSerializeDeserialize::Minus => num_bigint::Sign::Minus,
            NumBigintSignWithSerializeDeserialize::NoSign => num_bigint::Sign::NoSign,
            NumBigintSignWithSerializeDeserialize::Plus => num_bigint::Sign::Plus,
        };
        Self::new(sign, value.digits)
    }
}
impl std::convert::From<num_bigint::BigInt> for NumBigintBigIntNewWithSerializeDeserialize {
    fn from(value: num_bigint::BigInt) -> Self {
        let (sign, digits) = value.to_u32_digits();
        Self {
            sign: NumBigintSignWithSerializeDeserialize::from(sign),
            digits
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
        let sqlx_types_time_date_handle = sqlx::types::time::Date::from_calendar_date(
            2024,
            time::Month::February,
            3,
        )
        .unwrap();
        let sqlx_types_time_time_handle = sqlx::types::time::Time::from_hms(1,1,1).unwrap();
        let sqlx_types_chrono_naive_date_handle = sqlx::types::chrono::NaiveDate::from_ymd_opt(2016, 11, 3).unwrap();
        let sqlx_types_chrono_naive_time_handle = sqlx::types::chrono::NaiveTime::from_hms_opt(10, 10, 10).unwrap();
        let sqlx_types_chrono_naive_date_time_handle = sqlx::types::chrono::NaiveDateTime::new(
            sqlx_types_chrono_naive_date_handle.clone(),//todo
            sqlx_types_chrono_naive_time_handle.clone(),
        );
        let sqlx_types_time_primitive_date_time_handle = sqlx::types::time::PrimitiveDateTime::new(
            sqlx_types_time_date_handle.clone(), //todo
            sqlx_types_time_time_handle.clone(), //todo
        );
        let sqlx_types_chrono_fixed_offset_handle = sqlx::types::chrono::FixedOffset::west_opt(std_primitive_i32_handle.clone()).unwrap();
        let sqlx_types_time_offset_date_time_handle = sqlx::types::time::OffsetDateTime::from_unix_timestamp(std::primitive::i64::default()).unwrap();
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
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle = sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
            sqlx_types_chrono_naive_date_time_handle.clone(),
            sqlx_types_chrono_utc_handle.clone()
        );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle = sqlx::types::chrono::DateTime::<sqlx::types::chrono::Local>::from_naive_utc_and_offset(
            sqlx_types_chrono_naive_date_time_handle.clone(),
            sqlx_types_chrono_fixed_offset_handle.clone()
        );
        let std_ops_bound_std_primitive_i64_handle = std::ops::Bound::<std::primitive::i64>::Included(std_primitive_i64_handle.clone());
        let std_ops_bound_std_primitive_i32_handle = std::ops::Bound::<std::primitive::i32>::Included(std_primitive_i32_handle.clone());
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle = std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>::Included(sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle.clone());
        let std_ops_bound_sqlx_types_time_primitive_date_time_handle = std::ops::Bound::<sqlx::types::time::PrimitiveDateTime>::Included(
            sqlx_types_time_primitive_date_time_handle.clone(),
        );
        let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle = std::ops::Bound::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>::Included(sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle.clone());
        let std_ops_bound_sqlx_types_time_offset_date_time_handle = std::ops::Bound::<sqlx::types::time::OffsetDateTime>::Included(
            sqlx_types_time_offset_date_time_handle.clone(),
        );
        let std_ops_bound_sqlx_types_chrono_naive_date_handle = std::ops::Bound::<sqlx::types::chrono::NaiveDate>::Included(
            sqlx_types_chrono_naive_date_handle.clone(),
        );
        let std_ops_bound_sqlx_types_time_date_handle = std::ops::Bound::<sqlx::types::time::Date>::Included(
            sqlx_types_time_date_handle.clone(),
        );
        let std_ops_bound_sqlx_types_big_decimal_handle = std::ops::Bound::<sqlx::types::BigDecimal>::Included(sqlx_types_big_decimal_handle.clone());
        let std_ops_bound_sqlx_types_decimal_handle = std::ops::Bound::<sqlx::types::Decimal>::Included(
            sqlx_types_decimal_handle.clone(),
        );
        let std_ops_bound_sqlx_types_chrono_naive_date_time_handle = std::ops::Bound::<sqlx::types::chrono::NaiveDateTime>::Included(
            sqlx_types_chrono_naive_date_time_handle.clone()
        );
        let std_primitive_bool = StdPrimitiveBool(true);
        let std_primitive_i16 = StdPrimitiveI16(std::primitive::i16::default());
        let std_primitive_i32 = StdPrimitiveI32(std_primitive_i32_handle.clone());
        let std_primitive_i64 = StdPrimitiveI64(std_primitive_i64_handle.clone());
        let std_primitive_f32 = StdPrimitiveF32(std::primitive::f32::default());
        let std_primitive_f64 = StdPrimitiveF64(std::primitive::f64::default());
        let std_string_string = StdStringString(std_string_string_handle.clone());
        let std_vec_vec_std_primitive_u8 = StdVecVecStdPrimitiveU8(vec![std_primitive_u8_handle.clone()]);
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
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
            sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
            > {
                start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle.clone(),
                end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_handle.clone(),
            }
        );
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
            sqlx::postgres::types::PgRange::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> {
                start: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle.clone(),
                end: std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_handle.clone(),
            }
        );
        let sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::time::OffsetDateTime,
            > {
                start: std_ops_bound_sqlx_types_time_offset_date_time_handle.clone(),
                end: std_ops_bound_sqlx_types_time_offset_date_time_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(
            sqlx::postgres::types::PgRange::<
                sqlx::types::chrono::NaiveDateTime
            >{
                start: std_ops_bound_sqlx_types_chrono_naive_date_time_handle.clone(),
                end: std_ops_bound_sqlx_types_chrono_naive_date_time_handle.clone(),
            });
        let sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time =
            SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(sqlx::postgres::types::PgRange::<
                sqlx::types::time::PrimitiveDateTime,
            > {
                start: std_ops_bound_sqlx_types_time_primitive_date_time_handle.clone(),
                end: std_ops_bound_sqlx_types_time_primitive_date_time_handle.clone(),
            });
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
        let sqlx_postgres_types_pg_money = SqlxPostgresTypesPgMoney(sqlx::postgres::types::PgMoney(
            std_primitive_i64_handle.clone(),
        ));
        let sqlx_postgres_types_pg_ci_text = SqlxPostgresTypesPgCiText(
            sqlx::postgres::types::PgCiText(std_string_string_handle.clone()),
        );
        let sqlx_types_big_decimal = SqlxTypesBigDecimal(sqlx_types_big_decimal_handle.clone());
        let sqlx_types_decimal = SqlxTypesDecimal(
            sqlx_types_decimal_handle.clone(),
        );
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc =
            SqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Utc,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_utc_handle.clone()
            ));
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local =
            SqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::types::chrono::DateTime::<
                sqlx::types::chrono::Local,
            >::from_naive_utc_and_offset(
                sqlx_types_chrono_naive_date_time_handle.clone(),
                sqlx_types_chrono_fixed_offset_handle,
            ));
        let sqlx_types_chrono_naive_date_time = SqlxTypesChronoNaiveDateTime(
            sqlx_types_chrono_naive_date_time_handle.clone()
        );
        let sqlx_types_chrono_naive_date = SqlxTypesChronoNaiveDate(
            sqlx_types_chrono_naive_date_handle.clone(),
        );
        let sqlx_types_chrono_naive_time = SqlxTypesChronoNaiveTime(sqlx_types_chrono_naive_time_handle.clone());
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
        let sqlx_types_time_date = SqlxTypesTimeDate(
            sqlx_types_time_date_handle.clone(),
        );
        let sqlx_types_time_time = SqlxTypesTimeTime(
            sqlx_types_time_time_handle.clone(),
        );
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
        let serde_json_value = SerdeJsonValue(serde_json::Value::Bool(std::primitive::bool::default()));
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
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct StdPrimitiveBool(pub std::primitive::bool);//todo maybe make it private?
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBool {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBool {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBool for StdPrimitiveBool {}
impl PostgresqlOrder for StdPrimitiveBool {}
impl std::convert::From<StdPrimitiveBool> for SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveBool) -> Self {
        SupportedSqlxPostgresType::StdPrimitiveBool
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI16 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI16 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlSmallInt for StdPrimitiveI16 {}
impl AsPostgresqlSmallSerial for StdPrimitiveI16 {}
impl AsPostgresqlInt2 for StdPrimitiveI16 {}
impl PostgresqlOrder for StdPrimitiveI16 {}
impl std::convert::From<StdPrimitiveI16> for SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveI16) -> Self {
        SupportedSqlxPostgresType::StdPrimitiveI16
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI32 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI32 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI32 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInt for StdPrimitiveI32 {}
impl AsPostgresqlSerial for StdPrimitiveI32 {}
impl AsPostgresqlInt4 for StdPrimitiveI32 {}
impl PostgresqlOrder for StdPrimitiveI32 {}
impl std::convert::From<StdPrimitiveI32> for SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveI32) -> Self {
        SupportedSqlxPostgresType::StdPrimitiveI32
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBigInt for StdPrimitiveI64 {}
impl AsPostgresqlBigSerial for StdPrimitiveI64 {}
impl AsPostgresqlInt8 for StdPrimitiveI64 {}
impl PostgresqlOrder for StdPrimitiveI64 {}
impl std::convert::From<StdPrimitiveI64> for SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveI64) -> Self {
        SupportedSqlxPostgresType::StdPrimitiveI64
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::f32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveF32 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveF32 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdPrimitiveF32 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlReal for StdPrimitiveF32 {}
impl AsPostgresqlFloat4 for StdPrimitiveF32 {}
impl PostgresqlOrder for StdPrimitiveF32 {}
impl std::convert::From<StdPrimitiveF32> for SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveF32) -> Self {
        SupportedSqlxPostgresType::StdPrimitiveF32
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::f64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveF64 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveF64 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdPrimitiveF64 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDoublePrecision for StdPrimitiveF64 {}
impl AsPostgresqlFloat8 for StdPrimitiveF64 {}
impl PostgresqlOrder for StdPrimitiveF64 {}
impl std::convert::From<StdPrimitiveF64> for SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveF64) -> Self {
        SupportedSqlxPostgresType::StdPrimitiveF64
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdStringString {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdStringString {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdStringString {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlVarchar for StdStringString {}
impl AsPostgresqlCharN for StdStringString {}
impl AsPostgresqlText for StdStringString {}
impl AsPostgresqlName for StdStringString {}
impl AsPostgresqlCiText for StdStringString {}
impl PostgresqlOrder for StdStringString {}
impl std::convert::From<StdStringString> for SupportedSqlxPostgresType {
    fn from(_value: StdStringString) -> Self {
        SupportedSqlxPostgresType::StdStringString
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::vec::Vec<std::primitive::u8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdVecVecStdPrimitiveU8 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdVecVecStdPrimitiveU8 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBytea for StdVecVecStdPrimitiveU8 {}
impl std::convert::From<StdVecVecStdPrimitiveU8> for SupportedSqlxPostgresType {
    fn from(_value: StdVecVecStdPrimitiveU8) -> Self {
        SupportedSqlxPostgresType::StdVecVecStdPrimitiveU8
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgIntervalWithSerializeDeserialize {
    months: std::primitive::i32,
    days: std::primitive::i32,
    microseconds: std::primitive::i64,
}
impl std::convert::From<SqlxPostgresTypesPgIntervalWithSerializeDeserialize> for SqlxPostgresTypesPgInterval {
    fn from(value: SqlxPostgresTypesPgIntervalWithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgInterval{
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgInterval> for SqlxPostgresTypesPgIntervalWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgInterval) -> Self {
        Self {
            months: value.0.months,
            days: value.0.days,
            microseconds: value.0.microseconds,
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgInterval {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInterval for SqlxPostgresTypesPgInterval {}
impl PostgresqlOrder for SqlxPostgresTypesPgInterval {}
impl std::convert::From<SqlxPostgresTypesPgInterval> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgInterval) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgInterval
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(
    pub sqlx::postgres::types::PgRange<std::primitive::i64>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize {
    start: std::ops::Bound<std::primitive::i64>,
    end: std::ops::Bound<std::primitive::i64>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgRange{
            start: value.start,
            end: value.end,
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64> for SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI64) -> Self {
        Self {
            start: value.0.start,
            end: value.0.end,
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInt8Range for SqlxPostgresTypesPgRangeStdPrimitiveI64 {}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI64> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeStdPrimitiveI64) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(
    pub sqlx::postgres::types::PgRange<std::primitive::i32>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize {
    start: std::ops::Bound<std::primitive::i32>,
    end: std::ops::Bound<std::primitive::i32>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgRange{
            start: value.start,
            end: value.end,
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32> for SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeStdPrimitiveI32) -> Self {
        Self {
            start: value.0.start,
            end: value.0.end,
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}
impl std::convert::From<SqlxPostgresTypesPgRangeStdPrimitiveI32> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeStdPrimitiveI32) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcTryFromWithSerializeDeserializeError {
    Start {
        start: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError
    },
    End {
        end: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError
    },
    StartEnd {
        start: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError,
        end: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtc::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserializeTryFromOriginalError {
    Start {
        start: std::string::String
    },
    End {
        end: std::string::String
    },
    StartEnd {
        start: std::string::String,
        end: std::string::String
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserializeTryFromOriginalError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Result<Self, Self::Error> {
        let (start, end) = match (value.0.start, value.0.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)), 
                SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value),
                    std::ops::Bound::Included(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)), SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value),
                    std::ops::Bound::Excluded(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)) {
                Ok(value) => (std::ops::Bound::Included(value), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)), SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value),
                    std::ops::Bound::Included(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)), SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value),
                    std::ops::Bound::Excluded(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(start_value)) {
                Ok(value) => (std::ops::Bound::Excluded(value), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoUtc(end_value)) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self {
            start,
            end
        })
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres>
//     for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
// {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres>
//     for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
// {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
{
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError {
    Start {
        start: SqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError
    },
    End {
        end: SqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError
    },
    StartEnd {
        start: SqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError,
        end: SqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(start_value), SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocal::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserializeTryFromOriginalError {
    Start {
        start: std::string::String
    },
    End {
        end: std::string::String
    },
    StartEnd {
        start: std::string::String,
        end: std::string::String
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal> for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserializeTryFromOriginalError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Result<Self, Self::Error> {
        let (start, end) = match (value.0.start, value.0.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value)), 
                SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value),
                    std::ops::Bound::Included(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value)), SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value),
                    std::ops::Bound::Excluded(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value)) {
                Ok(value) => (std::ops::Bound::Included(value), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value)), SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value),
                    std::ops::Bound::Included(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (
                SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value)), SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value))
            ) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value),
                    std::ops::Bound::Excluded(end_value)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(start_value)) {
                Ok(value) => (std::ops::Bound::Excluded(value), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize::try_from(SqlxTypesChronoDateTimeSqlxTypesChronoLocal(end_value)) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self {
            start,
            end
        })
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres>
//     for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
// {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres>
//     for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
// {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
{
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeTryFromWithSerializeDeserializeError {
    Start {
        start: time::error::ComponentRange
    },
    End {
        end: time::error::ComponentRange
    },
    StartEnd {
        start: time::error::ComponentRange,
        end: time::error::ComponentRange
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesTimeOffsetDateTime::try_from(start_value), SqlxTypesTimeOffsetDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesTimeOffsetDateTime::try_from(start_value), SqlxTypesTimeOffsetDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesTimeOffsetDateTime::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesTimeOffsetDateTime::try_from(start_value), SqlxTypesTimeOffsetDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesTimeOffsetDateTime::try_from(start_value), SqlxTypesTimeOffsetDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesTimeOffsetDateTime::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesTimeOffsetDateTime::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesTimeOffsetDateTime::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize::from(SqlxTypesTimeOffsetDateTime(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize::from(SqlxTypesTimeOffsetDateTime(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize::from(SqlxTypesTimeOffsetDateTime(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize::from(SqlxTypesTimeOffsetDateTime(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError {
    Start {
        start: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError
    },
    End {
        end: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError
    },
    StartEnd {
        start: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError,
        end: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoNaiveDateTime::try_from(start_value), SqlxTypesChronoNaiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoNaiveDateTime::try_from(start_value), SqlxTypesChronoNaiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoNaiveDateTime::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoNaiveDateTime::try_from(start_value), SqlxTypesChronoNaiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoNaiveDateTime::try_from(start_value), SqlxTypesChronoNaiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoNaiveDateTime::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesChronoNaiveDateTime::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesChronoNaiveDateTime::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesChronoNaiveDateTime(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesChronoNaiveDateTime(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesChronoNaiveDateTime(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesChronoNaiveDateTime(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
impl SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    pub fn into_inner(self) -> sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> {
        self.0
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime>
    for sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>
{
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsTzRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeTryFromWithSerializeDeserializeError {
    Start {
        start: SqlxTypesTimePrimitiveDateTimeTryFromNewWithSerializeDeserializeError
    },
    End {
        end: SqlxTypesTimePrimitiveDateTimeTryFromNewWithSerializeDeserializeError
    },
    StartEnd {
        start: SqlxTypesTimePrimitiveDateTimeTryFromNewWithSerializeDeserializeError,
        end: SqlxTypesTimePrimitiveDateTimeTryFromNewWithSerializeDeserializeError
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesTimePrimitiveDateTime::try_from(start_value), SqlxTypesTimePrimitiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesTimePrimitiveDateTime::try_from(start_value), SqlxTypesTimePrimitiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesTimePrimitiveDateTime::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesTimePrimitiveDateTime::try_from(start_value), SqlxTypesTimePrimitiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesTimePrimitiveDateTime::try_from(start_value), SqlxTypesTimePrimitiveDateTime::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesTimePrimitiveDateTime::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesTimePrimitiveDateTime::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesTimePrimitiveDateTime::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesTimePrimitiveDateTime(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesTimePrimitiveDateTime(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesTimePrimitiveDateTime(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize::from(SqlxTypesTimePrimitiveDateTime(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTsRange for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTryFromWithSerializeDeserializeError {
    Start {
        start: std::string::String
    },
    End {
        end: std::string::String
    },
    StartEnd {
        start: std::string::String,
        end: std::string::String
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoNaiveDate::try_from(start_value), SqlxTypesChronoNaiveDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoNaiveDate::try_from(start_value), SqlxTypesChronoNaiveDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoNaiveDate::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesChronoNaiveDate::try_from(start_value), SqlxTypesChronoNaiveDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesChronoNaiveDate::try_from(start_value), SqlxTypesChronoNaiveDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesChronoNaiveDate::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesChronoNaiveDate::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesChronoNaiveDate::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate> as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(
    pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize>,
}
pub enum SqlxPostgresTypesPgRangeSqlxTypesTimeDateTryFromWithSerializeDeserializeError {
    Start {
        start: time::error::ComponentRange
    },
    End {
        end: time::error::ComponentRange
    },
    StartEnd {
        start: time::error::ComponentRange,
        end: time::error::ComponentRange
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    type Error = SqlxPostgresTypesPgRangeSqlxTypesTimeDateTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (start, end) = match (value.start, value.end) {
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesTimeDate::try_from(start_value), SqlxTypesTimeDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesTimeDate::try_from(start_value), SqlxTypesTimeDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Included(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Included(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesTimeDate::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Included(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Included(end_value),
            ) => match (SqlxTypesTimeDate::try_from(start_value), SqlxTypesTimeDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Included(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Excluded(end_value),
            ) => match (SqlxTypesTimeDate::try_from(start_value), SqlxTypesTimeDate::try_from(end_value)) {
                (Ok(start_value), Ok(end_value)) => (
                    std::ops::Bound::Excluded(start_value.0),
                    std::ops::Bound::Excluded(end_value.0)
                ),
                (Ok(_), Err(e)) => {
                    return Err(Self::Error::End {
                        end: e
                    })
                },
                (Err(e), Ok(_)) => {
                    return Err(Self::Error::Start {
                        start: e
                    })
                },
                (Err(start_error), Err(end_error)) => {
                    return Err(Self::Error::StartEnd {
                        start: start_error,
                        end: end_error
                    })
                },
            },
            (
                std::ops::Bound::Excluded(start_value),
                std::ops::Bound::Unbounded,
            ) => match SqlxTypesTimeDate::try_from(start_value) {
                Ok(value) => (std::ops::Bound::Excluded(value.0), std::ops::Bound::Unbounded),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(end_value),
            ) => match SqlxTypesTimeDate::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Included(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(end_value),
            ) => match SqlxTypesTimeDate::try_from(end_value) {
                Ok(value) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(value.0)),
                Err(e) => {
                    return Err(Self::Error::Start {
                        start: e
                    });
                }
            },
            (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded,
            ) => (
                std::ops::Bound::Unbounded,
                std::ops::Bound::Unbounded
            ),
        };
        Ok(Self(sqlx::postgres::types::PgRange{
            start,
            end
        }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeDate> for SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesTimeDate) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize::from(SqlxTypesTimeDate(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize::from(SqlxTypesTimeDate(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(
                SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize::from(SqlxTypesTimeDate(*value))
            ),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(
                SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize::from(SqlxTypesTimeDate(*value))
            ),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::time::Date> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDateRange for SqlxPostgresTypesPgRangeSqlxTypesTimeDate {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesTimeDate> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesTimeDate) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesBigDecimalNewWithSerializeDeserialize>,
    end: std::ops::Bound<SqlxTypesBigDecimalNewWithSerializeDeserialize>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize) -> Self {
        let start = match value.start {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimal::from(value).0),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimal::from(value).0),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.end {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimal::from(value).0),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimal::from(value).0),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self(sqlx::postgres::types::PgRange{
            start,
            end
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimalNewWithSerializeDeserialize::from(SqlxTypesBigDecimal(value.clone()))),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimalNewWithSerializeDeserialize::from(SqlxTypesBigDecimal(value.clone()))),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimalNewWithSerializeDeserialize::from(SqlxTypesBigDecimal(value.clone()))),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimalNewWithSerializeDeserialize::from(SqlxTypesBigDecimal(value.clone()))),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesBigDecimal {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesBigDecimal> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesBigDecimal) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(
    pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize{
    start: std::ops::Bound<SqlxTypesDecimal>,
    end: std::ops::Bound<SqlxTypesDecimal>,
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
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
        Self(sqlx::postgres::types::PgRange{
            start,
            end
        })
    }
}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimal> for SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgRangeSqlxTypesDecimal) -> Self {
        use std::ops::RangeBounds;
        let start = match value.0.start_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesDecimal(*value)),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesDecimal(*value)),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        let end = match value.0.end_bound() {
            std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesDecimal(*value)),
            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesDecimal(*value)),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        };
        Self {
            start,
            end
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<sqlx::types::Decimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumRange for SqlxPostgresTypesPgRangeSqlxTypesDecimal {}
impl std::convert::From<SqlxPostgresTypesPgRangeSqlxTypesDecimal> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgRangeSqlxTypesDecimal) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgRangeSqlxTypesDecimal
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgMoneyWithSerializeDeserialize(std::primitive::i64);
impl std::convert::From<SqlxPostgresTypesPgMoneyWithSerializeDeserialize> for SqlxPostgresTypesPgMoney {
    fn from(value: SqlxPostgresTypesPgMoneyWithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgMoney(value.0))
    }
}
impl std::convert::From<SqlxPostgresTypesPgMoney> for SqlxPostgresTypesPgMoneyWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgMoney) -> Self {
        Self(value.0.0)
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgMoney as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgMoney {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgMoney {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgMoney {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlMoney for SqlxPostgresTypesPgMoney {}
impl std::convert::From<SqlxPostgresTypesPgMoney> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgMoney) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgMoney
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgCiTextWithSerializeDeserialize(std::string::String);
impl std::convert::From<SqlxPostgresTypesPgCiTextWithSerializeDeserialize> for SqlxPostgresTypesPgCiText {
    fn from(value: SqlxPostgresTypesPgCiTextWithSerializeDeserialize) -> Self {
        Self(sqlx::postgres::types::PgCiText(value.0))
    }
}
impl std::convert::From<SqlxPostgresTypesPgCiText> for SqlxPostgresTypesPgCiTextWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgCiText) -> Self {
        Self(value.0.0)
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgCiText as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgCiText {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgCiText {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgCiText {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlCiText for SqlxPostgresTypesPgCiText {}
impl std::convert::From<SqlxPostgresTypesPgCiText> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgCiText) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgCiText
    }
}

#[derive(Debug)]
pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesBigDecimalNewWithSerializeDeserialize{
    digits: NumBigintBigIntNewWithSerializeDeserialize,
    scale: std::primitive::i64
}
impl std::convert::From<SqlxTypesBigDecimalNewWithSerializeDeserialize> for SqlxTypesBigDecimal {
    fn from(value: SqlxTypesBigDecimalNewWithSerializeDeserialize) -> Self {
        Self(sqlx::types::BigDecimal::new(num_bigint::BigInt::from(value.digits), value.scale))
    }
}
impl std::convert::From<SqlxTypesBigDecimal> for SqlxTypesBigDecimalNewWithSerializeDeserialize {
    fn from(value: SqlxTypesBigDecimal) -> Self {
        let (bigint, exponent) = value.0.into_bigint_and_exponent();
        Self {
            digits: NumBigintBigIntNewWithSerializeDeserialize::from(bigint),
            scale: exponent//todo is exponent equal to scale?
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBigDecimal {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBigDecimal {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesBigDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumeric for SqlxTypesBigDecimal {}
impl PostgresqlOrder for SqlxTypesBigDecimal {}
impl std::convert::From<SqlxTypesBigDecimal> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesBigDecimal) -> Self {
        SupportedSqlxPostgresType::SqlxTypesBigDecimal
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Decimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesDecimal {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesDecimal {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesDecimal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlNumeric for SqlxTypesDecimal {}
impl std::convert::From<SqlxTypesDecimal> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesDecimal) -> Self {
        SupportedSqlxPostgresType::SqlxTypesDecimal
    }
}

#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize(SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize);
impl std::convert::TryFrom<SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    type Error = SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let value = match SqlxTypesChronoNaiveDateTime::try_from(value.0) {
            Ok(value) => value.0,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(Self(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
            value,
            sqlx::types::chrono::Utc
        )))
    }
}
impl std::convert::TryFrom<SqlxTypesChronoDateTimeSqlxTypesChronoUtc> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcFromNaiveUtcAndOffsetWithSerializeDeserialize {
    type Error = std::string::String;
    fn try_from(value: SqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Result<Self, Self::Error> {
        //on commit time there is not non-panic version of .date_naive()
        let date: sqlx::types::chrono::NaiveDate = match std::panic::catch_unwind(||value.0.date_naive()) {
            Ok(value) => value,
            Err(_e) => {
                return Err(std::string::String::from("failed to create sqlx::types::chrono::NaiveDate with .date_naive()"));
            },
        };
        let time = value.0.time();
        Ok(Self(SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize {
            date: SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(date)),
            time: SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveTime(time)),
        }))
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestamp for SqlxTypesChronoDateTimeSqlxTypesChronoUtc {}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoUtc> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesChronoDateTimeSqlxTypesChronoUtc) -> Self {
        SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc
    }
}

#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(
    pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize{
    naive_date_time: SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize,
    fixed_offset: std::primitive::i32,
}
pub enum SqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError {
    NaiveDateTime {
        naive_date_time: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError,
    }, 
    FixedOffset {
        fixed_offset: std::string::String
    },
    NaiveDateTimeFixedOffset {
        naive_date_time: SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError,
        fixed_offset: std::string::String
    }
}
impl std::convert::TryFrom<SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    type Error = SqlxTypesChronoDateTimeSqlxTypesChronoLocalTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (naive_date_time, fixed_offset) = match (SqlxTypesChronoNaiveDateTime::try_from(value.naive_date_time), sqlx::types::chrono::FixedOffset::east_opt(value.fixed_offset)) {
            (Ok(naive_date_time), Some(fixed_offset)) => (naive_date_time.0, fixed_offset),
            (Err(e), Some(_)) => {
                return Err(Self::Error::NaiveDateTime {
                    naive_date_time: e,
                });
            },
            (Ok(_), None) => {
                return Err(Self::Error::FixedOffset {
                    fixed_offset: generate_sqlx_types_chrono_fixed_offset_east_opt_failed_message(value.fixed_offset)
                });
            },
            (Err(naive_date_time_error), None) => {
                return Err(Self::Error::NaiveDateTimeFixedOffset {
                    naive_date_time: naive_date_time_error,
                    fixed_offset: generate_sqlx_types_chrono_fixed_offset_east_opt_failed_message(value.fixed_offset)
                });
            },
        };
        Ok(Self(sqlx::types::chrono::DateTime::from_naive_utc_and_offset(
            naive_date_time,
            fixed_offset
        )))
    }
}
impl std::convert::TryFrom<SqlxTypesChronoDateTimeSqlxTypesChronoLocal> for SqlxTypesChronoDateTimeSqlxTypesChronoLocalFromNaiveUtcAndOffsetWithSerializeDeserialize {
    type Error = std::string::String;
    fn try_from(value: SqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Result<Self, Self::Error> {
        //on commit time there is not non-panic version of .date_naive()
        let date: sqlx::types::chrono::NaiveDate = match std::panic::catch_unwind(||value.0.date_naive()) {
            Ok(value) => value,
            Err(_e) => {
                return Err(std::string::String::from("failed to create sqlx::types::chrono::NaiveDate with .date_naive()"));
            },
        };
        let time = value.0.time();
        let offset = value.0.offset().local_minus_utc();//todo test - maybe need to use .utc_minus_local() instead
        Ok(Self{
            naive_date_time: SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize {
                date: SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(date)),
                time: SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveTime(time)),
            },
            fixed_offset: offset,
        })
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Local> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestampTz for SqlxTypesChronoDateTimeSqlxTypesChronoLocal {}
impl std::convert::From<SqlxTypesChronoDateTimeSqlxTypesChronoLocal> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesChronoDateTimeSqlxTypesChronoLocal) -> Self {
        SupportedSqlxPostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal
    }
}

#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize {
    date: SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize,
    time: SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize,
}
pub enum SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError {
    Date {
        date: std::string::String
    },
    Time {
        time: std::string::String
    },
    DateTime {
        date: std::string::String,
        time: std::string::String
    }
}
impl std::convert::TryFrom<SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize> for SqlxTypesChronoNaiveDateTime {
    type Error = SqlxTypesChronoNaiveDateTimeTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (date, time) = match (SqlxTypesChronoNaiveDate::try_from(value.date), SqlxTypesChronoNaiveTime::try_from(value.time)) {
            (Ok(date), Ok(time)) => (date.0, time.0),
            (Ok(_), Err(e)) => {
                return Err(Self::Error::Time {
                    time: e
                });
            },
            (Err(e), Ok(_)) => {
                return Err(Self::Error::Date {
                    date: e
                })
            },
            (Err(date_error), Err(time_error)) => {
                return Err(Self::Error::DateTime {
                    date: date_error,
                    time: time_error
                });
            },
        };
        Ok(Self(sqlx::types::chrono::NaiveDateTime::new(date,time)))
    }
}
impl std::convert::From<SqlxTypesChronoNaiveDateTime> for SqlxTypesChronoNaiveDateTimeNewWithSerializeDeserialize {
    fn from(value: SqlxTypesChronoNaiveDateTime) -> Self {
        Self {
            //todo maybe impl from directly
            date: SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveDate(value.0.date())),
            time: SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize::from(SqlxTypesChronoNaiveTime(value.0.time())),
        }
       
    }
}
impl SqlxTypesChronoNaiveDateTime {
    pub fn into_inner(self) -> sqlx::types::chrono::NaiveDateTime {
        self.0
    }
}
impl std::convert::From<SqlxTypesChronoNaiveDateTime>
    for sqlx::types::chrono::NaiveDateTime
{
    fn from(value: SqlxTypesChronoNaiveDateTime) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveDateTime {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<
            sqlx::Postgres,
        >>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveDateTime as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDateTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestamp for SqlxTypesChronoNaiveDateTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDateTime {}
impl std::convert::From<SqlxTypesChronoNaiveDateTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesChronoNaiveDateTime) -> Self {
        SupportedSqlxPostgresType::SqlxTypesChronoNaiveDateTime
    }
}

#[derive(Debug)]
pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize {
    year: std::primitive::i32,
    month: std::primitive::u32,
    day: std::primitive::u32
}
impl std::convert::TryFrom<SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize> for SqlxTypesChronoNaiveDate {
    type Error = std::string::String;
    fn try_from(value: SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let option_inner_value = sqlx::types::chrono::NaiveDate::from_ymd_opt(
            value.year,
            value.month,
            value.day
        );
        match option_inner_value {
            Some(value) => Ok(Self(value)),
            None => Err(std::string::String::from("failed to create sqlx::types::chrono::NaiveDate from year, month and day"))
        }
    }
}
impl std::convert::From<SqlxTypesChronoNaiveDate> for SqlxTypesChronoNaiveDateFromYmdOptWithSerializeDeserialize {
    fn from(value: SqlxTypesChronoNaiveDate) -> Self {
        use chrono::Datelike;
        Self {
            year: value.0.year(),
            month: value.0.month(),
            day: value.0.day()
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveDate as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDate {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveDate {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDate for SqlxTypesChronoNaiveDate {}
impl PostgresqlOrder for SqlxTypesChronoNaiveDate {}
impl std::convert::From<SqlxTypesChronoNaiveDate> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesChronoNaiveDate) -> Self {
        SupportedSqlxPostgresType::SqlxTypesChronoNaiveDate
    }
}

#[derive(Debug)]
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize{
    hour: std::primitive::u32,
    min: std::primitive::u32,
    sec: std::primitive::u32
}
impl std::convert::TryFrom<SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize> for SqlxTypesChronoNaiveTime {
    type Error = std::string::String;
    fn try_from(value: SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match sqlx::types::chrono::NaiveTime::from_hms_opt(
            value.hour,
            value.min, 
            value.sec
        ) {
            Some(value) => Ok(Self(value)),
            None => Err(std::string::String::from("failed to create sqlx::types::chrono::NaiveTime from hour, minute and second"))
        }
    }
}
impl std::convert::From<SqlxTypesChronoNaiveTime> for SqlxTypesChronoNaiveTimeFromHmsOptWithSerializeDeserialize {
    fn from(value: SqlxTypesChronoNaiveTime) -> Self {
        use chrono::Timelike;
        Self {
            hour: value.0.hour(),
            min: value.0.minute(),
            sec: value.0.second()
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesChronoNaiveTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTime for SqlxTypesChronoNaiveTime {}
impl PostgresqlOrder for SqlxTypesChronoNaiveTime {}
impl std::convert::From<SqlxTypesChronoNaiveTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesChronoNaiveTime) -> Self {
        SupportedSqlxPostgresType::SqlxTypesChronoNaiveTime
    }
}

#[derive(Debug)]
pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxPostgresTypesPgTimeTzWithSerializeDeserialize{
    time: SqlxTypesTimeTimeFromHmsWithSerializeDeserialize,
    offset: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize
}
pub enum SqlxPostgresTypesPgTimeTzTryFromWithSerializeDeserializeError {
    TimeOffset {
        time: time::error::ComponentRange,
        offset: time::error::ComponentRange,
    },
    Time {
        time: time::error::ComponentRange,
    },
    Offset {
        offset: time::error::ComponentRange
    }
}
impl std::convert::TryFrom<SqlxPostgresTypesPgTimeTzWithSerializeDeserialize> for SqlxPostgresTypesPgTimeTz {
    type Error = SqlxPostgresTypesPgTimeTzTryFromWithSerializeDeserializeError;
    fn try_from(value: SqlxPostgresTypesPgTimeTzWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (time, offset) = match (SqlxTypesTimeTime::try_from(value.time), sqlx::types::time::UtcOffset::try_from(value.offset)) {
            (Ok(time), Ok(offset)) => (time.0,offset),
            (Err(e), Ok(_)) => {
                return Err(Self::Error::Time {
                    time: e
                });
            },
            (Ok(_), Err(e)) => {
                return Err(Self::Error::Offset {
                    offset: e
                });
            },
            (Err(time_error), Err(offset_error)) => {
                return Err(Self::Error::TimeOffset {
                    time: time_error,
                    offset: offset_error
                });
            },
        };
        Ok(Self(sqlx::postgres::types::PgTimeTz {
            time,
            offset,
        }))
    }
}
impl std::convert::From<SqlxPostgresTypesPgTimeTz> for SqlxPostgresTypesPgTimeTzWithSerializeDeserialize {
    fn from(value: SqlxPostgresTypesPgTimeTz) -> Self {
        Self {
            //todo impl from directly from type?
            time: SqlxTypesTimeTimeFromHmsWithSerializeDeserialize::from(SqlxTypesTimeTime(value.0.time)),
            offset: SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize::from(value.0.offset)
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgTimeTz as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgTimeTz {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxPostgresTypesPgTimeTz {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimeTz for SqlxPostgresTypesPgTimeTz {}
impl std::convert::From<SqlxPostgresTypesPgTimeTz> for SupportedSqlxPostgresType {
    fn from(_value: SqlxPostgresTypesPgTimeTz) -> Self {
        SupportedSqlxPostgresType::SqlxPostgresTypesPgTimeTz
    }
}

#[derive(Debug)]
pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize{
    date: SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize,
    time: SqlxTypesTimeTimeFromHmsWithSerializeDeserialize
}
pub enum SqlxTypesTimePrimitiveDateTimeTryFromNewWithSerializeDeserializeError {
    DateTime {
        date: time::error::ComponentRange,
        time: time::error::ComponentRange,
    },
    Date {
        date: time::error::ComponentRange,
    },
    Time {
        time: time::error::ComponentRange
    }
}
impl std::convert::TryFrom<SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize> for SqlxTypesTimePrimitiveDateTime {
    type Error = SqlxTypesTimePrimitiveDateTimeTryFromNewWithSerializeDeserializeError;
    fn try_from(value: SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let (date, time) = match (SqlxTypesTimeDate::try_from(value.date), SqlxTypesTimeTime::try_from(value.time)) {
            (Ok(date), Ok(time)) => (date, time),
            (Err(e), Ok(_)) => {
                return Err(Self::Error::Date {
                    date: e
                });
            },
            (Ok(_), Err(e)) => {
                return Err(Self::Error::Time {
                    time: e
                });
            },
            (Err(date_error), Err(time_error)) => {
                return Err(Self::Error::DateTime {
                    date: date_error,
                    time: time_error
                });
            },
        };
        Ok(Self(sqlx::types::time::PrimitiveDateTime::new(
            date.0,
            time.0,
        )))
    }
}
impl std::convert::From<SqlxTypesTimePrimitiveDateTime> for SqlxTypesTimePrimitiveDateTimeNewWithSerializeDeserialize {
    fn from(value: SqlxTypesTimePrimitiveDateTime) -> Self {
        Self {
            //todo impl from directly from type?
            date: SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize::from(SqlxTypesTimeDate(value.0.date())),
            time: SqlxTypesTimeTimeFromHmsWithSerializeDeserialize::from(SqlxTypesTimeTime(value.0.time()))
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::PrimitiveDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimePrimitiveDateTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimePrimitiveDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestamp for SqlxTypesTimePrimitiveDateTime {}
impl PostgresqlOrder for SqlxTypesTimePrimitiveDateTime {}
impl std::convert::From<SqlxTypesTimePrimitiveDateTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesTimePrimitiveDateTime) -> Self {
        SupportedSqlxPostgresType::SqlxTypesTimePrimitiveDateTime
    }
}

#[derive(Debug)]
pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize(std::primitive::i64);
impl std::convert::TryFrom<SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize> for SqlxTypesTimeOffsetDateTime {
    type Error = time::error::ComponentRange;
    fn try_from(value: SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match sqlx::types::time::OffsetDateTime::from_unix_timestamp(value.0) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl std::convert::From<SqlxTypesTimeOffsetDateTime> for SqlxTypesTimeOffsetDateTimeFromUnixTimestampWithSerializeDeserialize {
    fn from(value: SqlxTypesTimeOffsetDateTime) -> Self {
        Self(value.0.unix_timestamp())
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::OffsetDateTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeOffsetDateTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeOffsetDateTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTimestampTz for SqlxTypesTimeOffsetDateTime {}
impl std::convert::From<SqlxTypesTimeOffsetDateTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesTimeOffsetDateTime) -> Self {
        SupportedSqlxPostgresType::SqlxTypesTimeOffsetDateTime
    }
}

#[derive(Debug)]
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize {
    year: std::primitive::i32,
    month: TimeMonthWithSerializeDeserialize,
    day: std::primitive::u8
}
impl std::convert::TryFrom<SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize> for SqlxTypesTimeDate {
    type Error = time::error::ComponentRange;
    fn try_from(value: SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match sqlx::types::time::Date::from_calendar_date(
            value.year,
            time::Month::from(value.month),
            value.day
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
impl std::convert::From<SqlxTypesTimeDate> for SqlxTypesTimeDateFromCalendarDateWithSerializeDeserialize {
    fn from(value: SqlxTypesTimeDate) -> Self {
        Self{
            year: value.0.year(),
            month: value.0.month().into(),
            day: value.0.day()
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::Date as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeDate {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeDate {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeDate {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlDate for SqlxTypesTimeDate {}
impl PostgresqlOrder for SqlxTypesTimeDate {}
impl std::convert::From<SqlxTypesTimeDate> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesTimeDate) -> Self {
        SupportedSqlxPostgresType::SqlxTypesTimeDate
    }
}

#[derive(Debug)]
pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesTimeTimeFromHmsWithSerializeDeserialize{
    hour: std::primitive::u8,
    minute: std::primitive::u8,
    second: std::primitive::u8
}
//todo different init methods support
impl std::convert::TryFrom<SqlxTypesTimeTimeFromHmsWithSerializeDeserialize> for SqlxTypesTimeTime {
    type Error = time::error::ComponentRange;
    fn try_from(value: SqlxTypesTimeTimeFromHmsWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match sqlx::types::time::Time::from_hms(
            value.hour,
            value.minute,
            value.second
        ) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e)
        }
    }
}
impl std::convert::From<SqlxTypesTimeTime> for SqlxTypesTimeTimeFromHmsWithSerializeDeserialize {
    fn from(value: SqlxTypesTimeTime) -> Self {
        Self{
            hour: value.0.hour(),
            minute: value.0.minute(),
            second: value.0.second()
        }
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::time::Time as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesTimeTime {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesTimeTime {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesTimeTime {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlTime for SqlxTypesTimeTime {}
impl PostgresqlOrder for SqlxTypesTimeTime {}
impl std::convert::From<SqlxTypesTimeTime> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesTimeTime) -> Self {
        SupportedSqlxPostgresType::SqlxTypesTimeTime
    }
}

#[derive(Debug)]
pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesUuidUuidTryParseWithSerializeDeserialize(std::string::String);
impl std::convert::TryFrom<SqlxTypesUuidUuidTryParseWithSerializeDeserialize> for SqlxTypesUuidUuid {
    type Error = sqlx::types::uuid::Error;
    fn try_from(value: SqlxTypesUuidUuidTryParseWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match sqlx::types::uuid::Uuid::try_parse(&value.0) {
            Ok(value) => Ok(Self(value)),
            Err(e) => Err(e)
        }
    }
}
impl std::convert::From<SqlxTypesUuidUuid> for  SqlxTypesUuidUuidTryParseWithSerializeDeserialize {
    fn from(value: SqlxTypesUuidUuid) -> Self {
        Self(value.0.to_string())
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuid {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuid {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesUuidUuid {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlUuid for SqlxTypesUuidUuid {}
impl std::convert::From<SqlxTypesUuidUuid> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesUuidUuid) -> Self {
        SupportedSqlxPostgresType::SqlxTypesUuidUuid
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::ipnetwork::IpNetwork as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::ipnetwork::IpNetwork as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesIpnetworkIpNetwork {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesIpnetworkIpNetwork {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInet for SqlxTypesIpnetworkIpNetwork {}
impl AsPostgresqlCidr for SqlxTypesIpnetworkIpNetwork {}
impl std::convert::From<SqlxTypesIpnetworkIpNetwork> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesIpnetworkIpNetwork) -> Self {
        SupportedSqlxPostgresType::SqlxTypesIpnetworkIpNetwork
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::net::IpAddr as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for StdNetIpAddr {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdNetIpAddr {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for StdNetIpAddr {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlInet for StdNetIpAddr {}
impl AsPostgresqlCidr for StdNetIpAddr {}
impl std::convert::From<StdNetIpAddr> for SupportedSqlxPostgresType {
    fn from(_value: StdNetIpAddr) -> Self {
        SupportedSqlxPostgresType::StdNetIpAddr
    }
}

#[derive(Debug)]
pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesMacAddressMacAddressNewWithSerializeDeserialize([std::primitive::u8; 6]);
impl std::convert::From<SqlxTypesMacAddressMacAddressNewWithSerializeDeserialize> for SqlxTypesMacAddressMacAddress {
    fn from(value: SqlxTypesMacAddressMacAddressNewWithSerializeDeserialize) -> Self {
        Self(sqlx::types::mac_address::MacAddress::new(value.0))
    }
}
impl std::convert::From<SqlxTypesMacAddressMacAddress> for SqlxTypesMacAddressMacAddressNewWithSerializeDeserialize {
    fn from(value: SqlxTypesMacAddressMacAddress) -> Self {
        Self(value.0.bytes())
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::mac_address::MacAddress as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesMacAddressMacAddress {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesMacAddressMacAddress {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlMacAddr for SqlxTypesMacAddressMacAddress {}
impl std::convert::From<SqlxTypesMacAddressMacAddress> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesMacAddressMacAddress) -> Self {
        SupportedSqlxPostgresType::SqlxTypesMacAddressMacAddress
    }
}

#[derive(Debug)]
pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SqlxTypesBitVecFromBytesWithSerializeDeserialize(std::vec::Vec<std::primitive::u8>);
impl std::convert::From<SqlxTypesBitVecFromBytesWithSerializeDeserialize> for SqlxTypesBitVec {
    fn from(value: SqlxTypesBitVecFromBytesWithSerializeDeserialize) -> Self {
        Self(sqlx::types::BitVec::from_bytes(&value.0))
    }
}
impl std::convert::From<SqlxTypesBitVec> for SqlxTypesBitVecFromBytesWithSerializeDeserialize {
    fn from(value: SqlxTypesBitVec) -> Self {
        Self(value.0.into_iter().map(|element|Into::into(element)).collect::<std::vec::Vec<std::primitive::u8>>())
    }
}
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBitVec {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBitVec {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SqlxTypesBitVec {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlBit for SqlxTypesBitVec {}
impl AsPostgresqlVarBit for SqlxTypesBitVec {}
impl PostgresqlOrder for SqlxTypesBitVec {}
impl std::convert::From<SqlxTypesBitVec> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesBitVec) -> Self {
        SupportedSqlxPostgresType::SqlxTypesBitVec
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
impl<T> AsPostgresqlJson for SqlxTypesJson<T> {}
impl<T> AsPostgresqlJsonB for SqlxTypesJson<T> {}
impl<T> std::convert::From<SqlxTypesJson<T>> for SupportedSqlxPostgresType {
    fn from(_value: SqlxTypesJson<T>) -> Self {
        SupportedSqlxPostgresType::SqlxTypesJsonT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
impl sqlx::Type<sqlx::Postgres> for SerdeJsonValue {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl sqlx::Encode<'_, sqlx::Postgres> for SerdeJsonValue {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
//     fn encode(
//         self,
//         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull
//     where
//         Self: Sized,
//     {
//         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
//     }
//     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
//         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
//     }
//     fn size_hint(&self) -> std::primitive::usize {
//         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for SerdeJsonValue {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(e) => Err(e),
//         }
//     }
// }
impl CheckSupportedPostgresqlColumnType for SerdeJsonValue {
    fn check_supported_postgresql_column_type() {}
}
impl AsPostgresqlJson for SerdeJsonValue {}
impl AsPostgresqlJsonB for SerdeJsonValue {}
impl std::convert::From<SerdeJsonValue> for SupportedSqlxPostgresType {
    fn from(_value: SerdeJsonValue) -> Self {
        SupportedSqlxPostgresType::SerdeJsonValue
    }
}

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
pub trait AsPostgresqlName {}
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