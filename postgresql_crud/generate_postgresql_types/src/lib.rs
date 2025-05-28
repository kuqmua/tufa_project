#[proc_macro]
pub fn generate_postgresql_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    #[derive(Debug, strum_macros::Display)]
    enum RustTypeName {
        StdPrimitiveI16,
        StdPrimitiveI32,
        StdPrimitiveI64,
        StdPrimitiveF32,
        StdPrimitiveF64,
        SqlxPostgresTypesPgMoney,
        SqlxTypesBigDecimal,
        StdPrimitiveBool,
        StdStringString,
        StdVecVecStdPrimitiveU8,
        SqlxTypesChronoNaiveTime,
        SqlxTypesTimeTime,
        SqlxPostgresTypesPgInterval,
        SqlxTypesTimeDate,
        SqlxTypesChronoNaiveDate,
        SqlxTypesChronoNaiveDateTime,
        SqlxTypesTimePrimitiveDateTime,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
        SqlxTypesUuidUuid,
        SqlxTypesIpnetworkIpNetwork,
        SqlxTypesMacAddressMacAddress,
        SqlxPostgresTypesPgRangeStdPrimitiveI32,
        SqlxPostgresTypesPgRangeStdPrimitiveI64,
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
        SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    }
    impl std::convert::From<&PostgresqlType> for RustTypeName {
        fn from(value: &PostgresqlType) -> Self {
            match &value {
                PostgresqlType::StdPrimitiveI16AsInt2 => Self::StdPrimitiveI16,
                PostgresqlType::StdPrimitiveI32AsInt4 => Self::StdPrimitiveI32,
                PostgresqlType::StdPrimitiveI64AsInt8 => Self::StdPrimitiveI64,
                PostgresqlType::StdPrimitiveF32AsFloat4 => Self::StdPrimitiveF32,
                PostgresqlType::StdPrimitiveF64AsFloat8 => Self::StdPrimitiveF64,
                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => Self::StdPrimitiveI16,
                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => Self::StdPrimitiveI32,
                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => Self::StdPrimitiveI64,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => Self::SqlxPostgresTypesPgMoney,
                PostgresqlType::SqlxTypesBigDecimalAsNumeric => Self::SqlxTypesBigDecimal,
                PostgresqlType::StdPrimitiveBoolAsBool => Self::StdPrimitiveBool,
                PostgresqlType::StdStringStringAsText => Self::StdStringString,
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Self::StdVecVecStdPrimitiveU8,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::SqlxPostgresTypesPgInterval,
                PostgresqlType::SqlxTypesTimeDateAsDate => Self::SqlxTypesTimeDate,
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => Self::SqlxTypesTimePrimitiveDateTime,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Self::SqlxTypesUuidUuid,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::SqlxTypesUuidUuid,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
            }
        }
    }
    #[derive(Debug, strum_macros::Display)]
    enum PostgresqlTypeName {
        Int2,
        Int4,
        Int8,
        Float4,
        Float8,
        SmallSerialInitializedByPostgresql,
        SerialInitializedByPostgresql,
        BigSerialInitializedByPostgresql,
        Money,
        Numeric,
        Bool,
        Text,
        Bytea,
        Time,
        Interval,
        Date,
        Timestamp,
        TimestampTz,
        UuidV4InitializedByPostgresql,
        UuidInitializedByClient,
        Inet,
        MacAddr,
        Int4Range,
        Int8Range,
        NumRange,
        DateRange,
        TimestampRange,
        TimestampTzRange,
    }
    impl std::convert::From<&PostgresqlType> for PostgresqlTypeName {
        fn from(value: &PostgresqlType) -> Self {
            match &value {
                PostgresqlType::StdPrimitiveI16AsInt2 => Self::Int2,
                PostgresqlType::StdPrimitiveI32AsInt4 => Self::Int4,
                PostgresqlType::StdPrimitiveI64AsInt8 => Self::Int8,
                PostgresqlType::StdPrimitiveF32AsFloat4 => Self::Float4,
                PostgresqlType::StdPrimitiveF64AsFloat8 => Self::Float8,
                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => Self::SmallSerialInitializedByPostgresql,
                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => Self::SerialInitializedByPostgresql,
                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => Self::BigSerialInitializedByPostgresql,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => Self::Money,
                PostgresqlType::SqlxTypesBigDecimalAsNumeric => Self::Numeric,
                PostgresqlType::StdPrimitiveBoolAsBool => Self::Bool,
                PostgresqlType::StdStringStringAsText => Self::Text,
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Self::Bytea,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::Time,
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::Time,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::Interval,
                PostgresqlType::SqlxTypesTimeDateAsDate => Self::Date,
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => Self::Timestamp,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => Self::TimestampTz,
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Self::UuidV4InitializedByPostgresql,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::UuidInitializedByClient,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Inet,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::MacAddr,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::Int4Range,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::Int8Range,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => Self::NumRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => Self::DateRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::DateRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::TimestampRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => Self::TimestampRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::TimestampTzRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => Self::TimestampTzRange,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlType {
        StdPrimitiveI16AsInt2,
        StdPrimitiveI32AsInt4,
        StdPrimitiveI64AsInt8,
        StdPrimitiveF32AsFloat4,
        StdPrimitiveF64AsFloat8,
        StdPrimitiveI16AsSmallSerialInitializedByPostgresql,
        StdPrimitiveI32AsSerialInitializedByPostgresql,
        StdPrimitiveI64AsBigSerialInitializedByPostgresql,
        SqlxPostgresTypesPgMoneyAsMoney,
        SqlxTypesBigDecimalAsNumeric,
        StdPrimitiveBoolAsBool,
        StdStringStringAsText,
        StdVecVecStdPrimitiveU8AsBytea,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxPostgresTypesPgIntervalAsInterval,
        SqlxTypesTimeDateAsDate,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz,
        SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql,
        SqlxTypesUuidUuidAsUuidInitializedByClient,
        SqlxTypesIpnetworkIpNetworkAsInet,
        SqlxTypesMacAddressMacAddressAsMacAddr,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange,
    }
    enum CanBeNullable {
        True,
        False,
    }
    impl PostgresqlType {
        fn can_be_nullable(&self) -> CanBeNullable {
            match &self {
                Self::StdPrimitiveI16AsInt2 => CanBeNullable::True,
                Self::StdPrimitiveI32AsInt4 => CanBeNullable::True,
                Self::StdPrimitiveI64AsInt8 => CanBeNullable::True,
                Self::StdPrimitiveF32AsFloat4 => CanBeNullable::True,
                Self::StdPrimitiveF64AsFloat8 => CanBeNullable::True,
                Self::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => CanBeNullable::False,
                Self::StdPrimitiveI32AsSerialInitializedByPostgresql => CanBeNullable::False,
                Self::StdPrimitiveI64AsBigSerialInitializedByPostgresql => CanBeNullable::False,
                Self::SqlxPostgresTypesPgMoneyAsMoney => CanBeNullable::True,
                Self::SqlxTypesBigDecimalAsNumeric => CanBeNullable::True,
                Self::StdPrimitiveBoolAsBool => CanBeNullable::True,
                Self::StdStringStringAsText => CanBeNullable::True,
                Self::StdVecVecStdPrimitiveU8AsBytea => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveTimeAsTime => CanBeNullable::True,
                Self::SqlxTypesTimeTimeAsTime => CanBeNullable::True,
                Self::SqlxPostgresTypesPgIntervalAsInterval => CanBeNullable::True,
                Self::SqlxTypesTimeDateAsDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateAsDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => CanBeNullable::True,
                Self::SqlxTypesTimePrimitiveDateTimeAsTimestamp => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => CanBeNullable::True,
                Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBeNullable::False,
                Self::SqlxTypesUuidUuidAsUuidInitializedByClient => CanBeNullable::True,
                Self::SqlxTypesIpnetworkIpNetworkAsInet => CanBeNullable::True,
                Self::SqlxTypesMacAddressMacAddressAsMacAddr => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => CanBeNullable::True,
            }
        }
        fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
            let value = {
                let std_primitive_i16_stringified = "std::primitive::i16".to_string();
                let std_primitive_i32_stringified = "std::primitive::i32".to_string();
                let std_primitive_i64_stringified = "std::primitive::i64".to_string();
                let std_primitive_f32_stringified = "std::primitive::f32".to_string();
                let std_primitive_f64_stringified = "std::primitive::f64".to_string();
                let sqlx_postgres_types_pg_money_stringified = "sqlx::postgres::types::PgMoney".to_string();
                let sqlx_types_big_decimal_stringified = "sqlx::types::BigDecimal".to_string();
                let std_primitive_bool_stringified = "std::primitive::bool".to_string();
                let std_string_string_stringified = "std::string::String".to_string();
                let std_vec_vec_std_primitive_u8_stringified = "std::vec::Vec<std::primitive::u8>".to_string();
                let sqlx_types_time_date_stringified = "sqlx::types::time::Date".to_string();
                let sqlx_types_chrono_naive_date_stringified = "sqlx::types::chrono::NaiveDate".to_string();
                let sqlx_types_chrono_naive_time_stringified = "sqlx::types::chrono::NaiveTime".to_string();
                let sqlx_types_time_time_stringified = "sqlx::types::time::Time".to_string();
                let sqlx_postgres_types_pg_interval_stringified = "sqlx::postgres::types::PgInterval".to_string();
                let sqlx_types_chrono_naive_date_time_stringified = "sqlx::types::chrono::NaiveDateTime".to_string();
                let sqlx_types_time_primitive_date_time_stringified = "sqlx::types::time::PrimitiveDateTime".to_string();
                let (sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified, sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified) = {
                    let wrap_into_sqlx_types_chrono_date_time_stringified = |value: &dyn std::fmt::Display| format!("sqlx::types::chrono::DateTime<{value}>");
                    (wrap_into_sqlx_types_chrono_date_time_stringified(&"sqlx::types::chrono::Utc"), wrap_into_sqlx_types_chrono_date_time_stringified(&"sqlx::types::chrono::Local"))
                };
                let sqlx_types_uuid_uuid_stringified = "sqlx::types::uuid::Uuid".to_string();
                let sqlx_types_ipnetwork_ip_network_stringified = "sqlx::types::ipnetwork::IpNetwork".to_string();
                let sqlx_types_mac_address_mac_address_stringified = "sqlx::types::mac_address::MacAddress".to_string();
                let (
                    sqlx_postgres_types_pg_range_std_primitive_i32_stringified,
                    sqlx_postgres_types_pg_range_std_primitive_i64_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_time_date_stringified,
                    sqlx_postgres_types_pg_range_sqlx_types_big_decimal_stringified,
                ) = {
                    let wrap_into_sqlx_postgres_types_pg_range_stringified = |value: &dyn std::fmt::Display| format!("sqlx::postgres::types::PgRange<{value}>");
                    (
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i32_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i64_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_time_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_time_primitive_date_time_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_time_date_stringified),
                        wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_big_decimal_stringified),
                    )
                };
                match &self {
                    Self::StdPrimitiveI16AsInt2 => std_primitive_i16_stringified,
                    Self::StdPrimitiveI32AsInt4 => std_primitive_i32_stringified,
                    Self::StdPrimitiveI64AsInt8 => std_primitive_i64_stringified,
                    Self::StdPrimitiveF32AsFloat4 => std_primitive_f32_stringified,
                    Self::StdPrimitiveF64AsFloat8 => std_primitive_f64_stringified,
                    Self::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => std_primitive_i16_stringified,
                    Self::StdPrimitiveI32AsSerialInitializedByPostgresql => std_primitive_i32_stringified,
                    Self::StdPrimitiveI64AsBigSerialInitializedByPostgresql => std_primitive_i64_stringified,
                    Self::SqlxPostgresTypesPgMoneyAsMoney => sqlx_postgres_types_pg_money_stringified,
                    Self::SqlxTypesBigDecimalAsNumeric => sqlx_types_big_decimal_stringified,
                    Self::StdPrimitiveBoolAsBool => std_primitive_bool_stringified,
                    Self::StdStringStringAsText => std_string_string_stringified,
                    Self::StdVecVecStdPrimitiveU8AsBytea => std_vec_vec_std_primitive_u8_stringified,
                    Self::SqlxTypesChronoNaiveTimeAsTime => sqlx_types_chrono_naive_time_stringified,
                    Self::SqlxTypesTimeTimeAsTime => sqlx_types_time_time_stringified,
                    Self::SqlxPostgresTypesPgIntervalAsInterval => sqlx_postgres_types_pg_interval_stringified,
                    Self::SqlxTypesTimeDateAsDate => sqlx_types_time_date_stringified,
                    Self::SqlxTypesChronoNaiveDateAsDate => sqlx_types_chrono_naive_date_stringified,
                    Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => sqlx_types_chrono_naive_date_time_stringified,
                    Self::SqlxTypesTimePrimitiveDateTimeAsTimestamp => sqlx_types_time_primitive_date_time_stringified,
                    Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                    Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                    Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => sqlx_types_uuid_uuid_stringified,
                    Self::SqlxTypesUuidUuidAsUuidInitializedByClient => sqlx_types_uuid_uuid_stringified,
                    Self::SqlxTypesIpnetworkIpNetworkAsInet => sqlx_types_ipnetwork_ip_network_stringified,
                    Self::SqlxTypesMacAddressMacAddressAsMacAddr => sqlx_types_mac_address_mac_address_stringified,
                    Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => sqlx_postgres_types_pg_range_std_primitive_i32_stringified,
                    Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => sqlx_postgres_types_pg_range_std_primitive_i64_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => sqlx_postgres_types_pg_range_sqlx_types_big_decimal_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => sqlx_postgres_types_pg_range_sqlx_types_time_date_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                }
            };
            value.parse::<proc_macro2::TokenStream>().unwrap()
        }
    }
    impl quote::ToTokens for PostgresqlType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlType to proc_macro2::TokenStream")).to_tokens(tokens)
        }
    }
    impl std::convert::From<&PostgresqlTypeRange> for PostgresqlType {
        fn from(value: &PostgresqlTypeRange) -> PostgresqlType {
            match value {
                PostgresqlTypeRange::StdPrimitiveI32AsInt4 => Self::StdPrimitiveI32AsInt4,
                PostgresqlTypeRange::StdPrimitiveI64AsInt8 => Self::StdPrimitiveI64AsInt8,
                PostgresqlTypeRange::SqlxTypesBigDecimalAsNumeric => Self::SqlxTypesBigDecimalAsNumeric,
                PostgresqlTypeRange::SqlxTypesTimeDateAsDate => Self::SqlxTypesTimeDateAsDate,
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PostgresqlTypeRange::SqlxTypesTimePrimitiveDateTimeAsTimestamp => Self::SqlxTypesTimePrimitiveDateTimeAsTimestamp,
                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz,
            }
        }
    }
    enum PostgresqlTypeRange {
        StdPrimitiveI32AsInt4,
        StdPrimitiveI64AsInt8,
        SqlxTypesBigDecimalAsNumeric,
        SqlxTypesTimeDateAsDate,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz,
    }
    impl std::convert::TryFrom<PostgresqlType> for PostgresqlTypeRange {
        type Error = ();
        fn try_from(value: PostgresqlType) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlType::StdPrimitiveI16AsInt2 => Err(()),
                PostgresqlType::StdPrimitiveI32AsInt4 => Err(()),
                PostgresqlType::StdPrimitiveI64AsInt8 => Err(()),
                PostgresqlType::StdPrimitiveF32AsFloat4 => Err(()),
                PostgresqlType::StdPrimitiveF64AsFloat8 => Err(()),
                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => Err(()),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => Err(()),
                PostgresqlType::SqlxTypesBigDecimalAsNumeric => Err(()),
                PostgresqlType::StdPrimitiveBoolAsBool => Err(()),
                PostgresqlType::StdStringStringAsText => Err(()),
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Err(()),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Err(()),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Err(()),
                PostgresqlType::SqlxTypesTimeDateAsDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Err(()),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Err(()),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::StdPrimitiveI32AsInt4),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::StdPrimitiveI64AsInt8),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => Ok(Self::SqlxTypesBigDecimalAsNumeric),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => Ok(Self::SqlxTypesTimeDateAsDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz),
            }
        }
    }
    impl std::fmt::Display for PostgresqlTypeRange {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "{}", naming::parameter::SelfNotNullUpperCamelCase::from_display(&PostgresqlType::from(self)))
        }
    }
    impl quote::ToTokens for PostgresqlTypeRange {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlTypeRange to proc_macro2::TokenStream")).to_tokens(tokens)
        }
    }
    // todo reuse it(move to postgresql_macros_common) if sqlx devs will add nested array support
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlTypePattern {
        Standart,
        ArrayDimension1 { dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable },
        // sqlx does not support nested arrays yet. https://github.com/launchbadge/sqlx/issues/2280
        // ArrayDimension2 {
        //     dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        //     dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        // },
        // ArrayDimension3 {
        //     dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        //     dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        //     dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        // },
        // ArrayDimension4 {
        //     dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        //     dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        //     dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        //     dimension4_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        // },
    }
    impl PostgresqlTypePattern {
        fn array_dimensions_number(&self) -> std::primitive::usize {
            match &self {
                PostgresqlTypePattern::Standart => 0,
                PostgresqlTypePattern::ArrayDimension1 { .. } => 1,
                // PostgresqlTypePattern::ArrayDimension2 { .. } => 2,
                // PostgresqlTypePattern::ArrayDimension3 { .. } => 3,
                // PostgresqlTypePattern::ArrayDimension4 { .. } => 4,
            }
        }
        fn all() -> std::vec::Vec<Self> {
            Self::into_array().into_iter().fold(vec![], |mut acc, postgresql_type_pattern| {
                match &postgresql_type_pattern {
                    Self::Standart => {
                        acc.push(postgresql_type_pattern);
                    }
                    Self::ArrayDimension1 { .. } => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                            acc.push(Self::ArrayDimension1 { dimension1_not_null_or_nullable });
                        });
                    } // Self::ArrayDimension2 {..} => {
                      //     postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                      //         postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                      //             acc.push(Self::ArrayDimension2 {
                      //                 dimension1_not_null_or_nullable,
                      //                 dimension2_not_null_or_nullable,
                      //              });
                      //         });
                      //     });
                      // },
                      // Self::ArrayDimension3 {..} => {
                      //     postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                      //         postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                      //             postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension3_not_null_or_nullable| {
                      //                 acc.push(Self::ArrayDimension3 {
                      //                     dimension1_not_null_or_nullable,
                      //                     dimension2_not_null_or_nullable,
                      //                     dimension3_not_null_or_nullable,
                      //                 });
                      //             });
                      //         });
                      //     });
                      // },
                      // Self::ArrayDimension4 {..} => {
                      //     postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                      //         postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                      //             postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension3_not_null_or_nullable| {
                      //                 postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension4_not_null_or_nullable| {
                      //                     acc.push(Self::ArrayDimension4 {
                      //                         dimension1_not_null_or_nullable,
                      //                         dimension2_not_null_or_nullable,
                      //                         dimension3_not_null_or_nullable,
                      //                         dimension4_not_null_or_nullable,
                      //                     });
                      //                 });
                      //             });
                      //         });
                      //     });
                      // },
                }
                acc
            })
        }
    }
    #[derive(Debug, PartialEq, serde::Serialize)]
    struct PostgresqlTypeRecord {
        postgresql_type: PostgresqlType,
        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        postgresql_type_pattern: PostgresqlTypePattern,
    }
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PostgresqlTypeRecord {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
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
                #[automatically_derived]
                impl _serde::de::Visitor<'_> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(&self, __f: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__f, "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "postgresql_type" => _serde::__private::Ok(__Field::__field0),
                            "not_null_or_nullable" => _serde::__private::Ok(__Field::__field1),
                            "postgresql_type_pattern" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"postgresql_type" => _serde::__private::Ok(__Field::__field0),
                            b"not_null_or_nullable" => _serde::__private::Ok(__Field::__field1),
                            b"postgresql_type_pattern" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PostgresqlTypeRecord>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PostgresqlTypeRecord;
                    fn expecting(&self, __f: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__f, "struct PostgresqlTypeRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<PostgresqlType>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeRecord with 3 elements"));
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<postgresql_crud_macros_common::NotNullOrNullable>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeRecord with 3 elements"));
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<PostgresqlTypePattern>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlTypeRecord with 3 elements"));
                            }
                        };
                        Ok(PostgresqlTypeRecord {
                            postgresql_type: __field0,
                            not_null_or_nullable: __field1,
                            postgresql_type_pattern: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<PostgresqlType> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<postgresql_crud_macros_common::NotNullOrNullable> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<PostgresqlTypePattern> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("postgresql_type"));
                                    }
                                    __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<PostgresqlType>(&mut __map)?);
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("not_null_or_nullable"));
                                    }
                                    __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<postgresql_crud_macros_common::NotNullOrNullable>(&mut __map)?);
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("postgresql_type_pattern"));
                                    }
                                    __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<PostgresqlTypePattern>(&mut __map)?);
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => _serde::__private::de::missing_field("postgresql_type")?,
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => _serde::__private::de::missing_field("not_null_or_nullable")?,
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::de::missing_field("postgresql_type_pattern")?,
                        };
                        Ok(PostgresqlTypeRecord {
                            postgresql_type: __field0,
                            not_null_or_nullable: __field1,
                            postgresql_type_pattern: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &[&str] = &["postgresql_type", "not_null_or_nullable", "postgresql_type_pattern"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PostgresqlTypeRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PostgresqlTypeRecord>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl std::convert::From<(PostgresqlType, postgresql_crud_macros_common::NotNullOrNullable, PostgresqlTypePattern)> for PostgresqlTypeRecord {
        fn from(value: (PostgresqlType, postgresql_crud_macros_common::NotNullOrNullable, PostgresqlTypePattern)) -> Self {
            let error_message = "cant support nullable variants: ";
            match &value.0.can_be_nullable() {
                CanBeNullable::True => Self {
                    postgresql_type: value.0,
                    not_null_or_nullable: value.1,
                    postgresql_type_pattern: value.2,
                },
                CanBeNullable::False => {
                    if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &value.1 {
                        panic!("{error_message}{value:#?}");
                    }
                    match &value.2 {
                        PostgresqlTypePattern::Standart => Self {
                            postgresql_type: value.0,
                            not_null_or_nullable: value.1,
                            postgresql_type_pattern: value.2,
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => Self {
                                postgresql_type: value.0,
                                not_null_or_nullable: value.1,
                                postgresql_type_pattern: value.2,
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => panic!("{error_message}{value:#?}"),
                        },
                        // PostgresqlTypePattern::ArrayDimension2 {
                        //     dimension1_not_null_or_nullable,
                        //     dimension2_not_null_or_nullable,
                        // } => {
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension1_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension2_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     Self {
                        //         postgresql_type: value.0,
                        //         not_null_or_nullable: value.1,
                        //         postgresql_type_pattern: value.2,
                        //     }
                        // },
                        // PostgresqlTypePattern::ArrayDimension3 {
                        //     dimension1_not_null_or_nullable,
                        //     dimension2_not_null_or_nullable,
                        //     dimension3_not_null_or_nullable,
                        // } => {
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension1_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension2_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension3_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     Self {
                        //         postgresql_type: value.0,
                        //         not_null_or_nullable: value.1,
                        //         postgresql_type_pattern: value.2,
                        //     }
                        // },
                        // PostgresqlTypePattern::ArrayDimension4 {
                        //     dimension1_not_null_or_nullable,
                        //     dimension2_not_null_or_nullable,
                        //     dimension3_not_null_or_nullable,
                        //     dimension4_not_null_or_nullable,
                        // } => {
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension1_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension2_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension3_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &dimension4_not_null_or_nullable {
                        //         panic!("{error_message}{value:#?}");
                        //     }
                        //     Self {
                        //         postgresql_type: value.0,
                        //         not_null_or_nullable: value.1,
                        //         postgresql_type_pattern: value.2,
                        //     }
                        // },
                    }
                }
            }
        }
    }
    impl PostgresqlTypeRecord {
        fn all() -> std::vec::Vec<Self> {
            PostgresqlType::into_array().into_iter().fold(vec![], |mut acc, postgresql_type| {
                postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| match &postgresql_type.can_be_nullable() {
                    CanBeNullable::True => {
                        PostgresqlTypePattern::all().into_iter().for_each(|postgresql_type_pattern| {
                            acc.push(PostgresqlTypeRecord {
                                postgresql_type: postgresql_type.clone(),
                                not_null_or_nullable,
                                postgresql_type_pattern,
                            });
                        });
                    }
                    CanBeNullable::False => {
                        acc.push(PostgresqlTypeRecord {
                            postgresql_type: postgresql_type.clone(),
                            not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            postgresql_type_pattern: PostgresqlTypePattern::Standart,
                        });
                    }
                });
                acc
            })
        }
    }
    let postgresql_type_record_vec = {
        if false {
            PostgresqlTypeRecord::all()
        } else {
            let postgresql_type_record_vec = serde_json::from_str::<std::vec::Vec<PostgresqlTypeRecord>>(&input_token_stream.to_string()).expect("failed to get Config for generate_postgresql_type");
            {
                let mut acc = vec![];
                for element in &postgresql_type_record_vec {
                    if acc.contains(&element) {
                        panic!("not unique postgersql type provided: {element:#?}");
                    } else {
                        acc.push(element);
                    }
                }
            }
            let expanded_postgresql_type_record_vec = postgresql_type_record_vec.into_iter().fold(vec![], |mut acc, postgresql_type_record_element| {
                use postgresql_crud_macros_common::NotNullOrNullable;
                #[derive(Clone)]
                struct PostgresqlTypeRecordHandle {
                    not_null_or_nullable: NotNullOrNullable,
                    postgresql_type_pattern: PostgresqlTypePattern,
                }
                fn generate_postgresql_type_record_handle_vec(postgresql_type_record_handle: PostgresqlTypeRecordHandle) -> std::vec::Vec<PostgresqlTypeRecordHandle> {
                    let generate_vec = |current_postgresql_type_record_handle: PostgresqlTypeRecordHandle|{
                        let mut acc = vec![];
                        for element in generate_postgresql_type_record_handle_vec(current_postgresql_type_record_handle) {
                            acc.push(element);
                        }
                        acc.push(postgresql_type_record_handle.clone());
                        acc
                    };
                    //same pattern was in generate_postgresql_types 21.05.2025
                    match (&postgresql_type_record_handle.not_null_or_nullable, &postgresql_type_record_handle.postgresql_type_pattern) {
                        (NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) => vec![postgresql_type_record_handle],
                        (NotNullOrNullable::Nullable, PostgresqlTypePattern::Standart) => generate_vec(PostgresqlTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_type_pattern: PostgresqlTypePattern::Standart,
                        }),
                        (NotNullOrNullable::NotNull, PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlTypeRecordHandle {
                            not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                            postgresql_type_pattern: PostgresqlTypePattern::Standart,
                        }),
                        (NotNullOrNullable::Nullable, PostgresqlTypePattern::ArrayDimension1 { .. }) => generate_vec(PostgresqlTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_type_pattern: postgresql_type_record_handle.postgresql_type_pattern.clone(),
                        }),
                    }
                }
                generate_postgresql_type_record_handle_vec(PostgresqlTypeRecordHandle {
                    not_null_or_nullable: postgresql_type_record_element.not_null_or_nullable,
                    postgresql_type_pattern: postgresql_type_record_element.postgresql_type_pattern,
                }).into_iter().for_each(|postgresql_type_record_handle_element|{
                    let postgresql_type_record = PostgresqlTypeRecord {
                        postgresql_type: postgresql_type_record_element.postgresql_type.clone(),
                        not_null_or_nullable: postgresql_type_record_handle_element.not_null_or_nullable,
                        postgresql_type_pattern: postgresql_type_record_handle_element.postgresql_type_pattern,
                    };
                    if !acc.contains(&postgresql_type_record) {
                        acc.push(postgresql_type_record);
                    }
                });
                acc
            });
            // {
            //     let mut debug_vec = vec![];
            //     for element in &expanded_postgresql_type_record_vec {
            //         if let PostgresqlType::StdPrimitiveI16AsInt2 = &element.postgresql_type {
            //             debug_vec.push(element);
            //         }
            //     }
            //     println!("{debug_vec:#?}");
            //     println!("{}", debug_vec.len());
            // }
            expanded_postgresql_type_record_vec
        }
    }
    // .into_iter()
    // .filter(|element| {
    //     use postgresql_crud_macros_common::NotNullOrNullable;
    //     let postgresql_type_filter = match &element.postgresql_type {
    //         PostgresqlType::StdPrimitiveI16AsInt2 => false,
    //         PostgresqlType::StdPrimitiveI32AsInt4 => false,
    //         PostgresqlType::StdPrimitiveI64AsInt8 => false,
    //         PostgresqlType::StdPrimitiveF32AsFloat4 => false,
    //         PostgresqlType::StdPrimitiveF64AsFloat8 => false,
    //         PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => false,
    //         PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => false,
    //         PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => false,
    //         PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => false,
    //         PostgresqlType::SqlxTypesBigDecimalAsNumeric => false,
    //         PostgresqlType::StdPrimitiveBoolAsBool => false,
    //         PostgresqlType::StdStringStringAsText => false,
    //         PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => false,
    //         PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => false,
    //         PostgresqlType::SqlxTypesTimeTimeAsTime => false,
    //         PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => false,
    //         PostgresqlType::SqlxTypesTimeDateAsDate => false,
    //         PostgresqlType::SqlxTypesChronoNaiveDateAsDate => false,
    //         PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => false,
    //         PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => false,
    //         PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => false,
    //         PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => false,
    //         PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => false,
    //         PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => false,
    //         PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => false,
    //         PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => false,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => false,
    //     };
    //     let not_null_or_nullable_filter = match &element.not_null_or_nullable {
    //         NotNullOrNullable::NotNull => false,
    //         NotNullOrNullable::Nullable => false,
    //     };
    //     let postgresql_type_pattern_filter = match &element.postgresql_type_pattern {
    //         PostgresqlTypePattern::Standart => false,
    //         PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
    //             NotNullOrNullable::NotNull => false,
    //             NotNullOrNullable::Nullable => false,
    //         },
    //         // PostgresqlTypePattern::ArrayDimension2 {
    //         //     dimension1_not_null_or_nullable,
    //         //     dimension2_not_null_or_nullable,
    //         // } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         // },
    //         // PostgresqlTypePattern::ArrayDimension3 {
    //         //     dimension1_not_null_or_nullable,
    //         //     dimension2_not_null_or_nullable,
    //         //     dimension3_not_null_or_nullable,
    //         // } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         // },
    //         // PostgresqlTypePattern::ArrayDimension4 {
    //         //     dimension1_not_null_or_nullable,
    //         //     dimension2_not_null_or_nullable,
    //         //     dimension3_not_null_or_nullable,
    //         //     dimension4_not_null_or_nullable,
    //         // } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
    //         //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
    //         // }
    //     };
    //     postgresql_type_filter && not_null_or_nullable_filter && postgresql_type_pattern_filter
    // })
    // .collect::<std::vec::Vec<PostgresqlTypeRecord>>()
    ;
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlTypesJsonVariants",
    //     &serde_json::to_string(&postgresql_type_record_vec).unwrap(),
    // );
    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;
    let (postgresql_crud_table_rust_struct_fields_token_stream, postgresql_type_array) = postgresql_type_record_vec
        .par_iter()
        // .into_iter()//just for console prints ordering
        .map(|element| {
            // println!("{element:#?}");
            let postgresql_type = &element.postgresql_type;
            let not_null_or_nullable = &element.not_null_or_nullable;
            let postgresql_type_pattern = &element.postgresql_type_pattern;

            let array_dimensions_number = postgresql_type_pattern.array_dimensions_number();

            let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

            let column_snake_case = naming::ColumnSnakeCase;
            let query_snake_case = naming::QuerySnakeCase;
            let value_snake_case = naming::ValueSnakeCase;
            let self_snake_case = naming::SelfSnakeCase;
            let increment_snake_case = naming::IncrementSnakeCase;
            let start_snake_case = naming::StartSnakeCase;
            let end_snake_case = naming::EndSnakeCase;
            let digits_snake_case = naming::DigitsSnakeCase;
            let scale_snake_case = naming::ScaleSnakeCase;
            let year_snake_case = naming::YearSnakeCase;
            let month_snake_case = naming::MonthSnakeCase;
            let day_snake_case = naming::DaySnakeCase;
            let months_snake_case = naming::MonthsSnakeCase;
            let days_snake_case = naming::DaysSnakeCase;
            let microseconds_snake_case = naming::MicrosecondsSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;

            let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
            let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
            let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
            let std_string_string_token_stream = token_patterns::StdStringString;

            let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
            let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

            let postgresql_crud_macros_common_import_path_crate = postgresql_crud_macros_common::ImportPath::Crate;

            let generate_ident_token_stream = |postgresql_type: &PostgresqlType, not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, postgresql_type_pattern: &PostgresqlTypePattern| {
                let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
                let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
                let rust_type_name = RustTypeName::from(postgresql_type);
                let postgresql_type_name = PostgresqlTypeName::from(postgresql_type);
                let not_null_or_nullable_rust = not_null_or_nullable.rust();
                let (rust_part, postgresql_part) = match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => (format!("{rust_type_name}"), format!("{postgresql_type_name}")),
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                        let d1 = dimension1_not_null_or_nullable;
                        let d1_rust = dimension1_not_null_or_nullable.rust();
                        (format!("{vec_of_upper_camel_case}{d1_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{postgresql_type_name}"))
                    } // PostgresqlTypePattern::ArrayDimension2 {
                      //     dimension1_not_null_or_nullable,
                      //     dimension2_not_null_or_nullable,
                      // } => {
                      //     let d1 = dimension1_not_null_or_nullable;
                      //     let d1_rust = dimension1_not_null_or_nullable.rust();
                      //     let d2 = dimension2_not_null_or_nullable;
                      //     let d2_rust = dimension2_not_null_or_nullable.rust();
                      //     (
                      //         format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{rust_type_name}"),
                      //         format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{postgresql_type_name}")
                      //     )
                      // },
                      // PostgresqlTypePattern::ArrayDimension3 {
                      //     dimension1_not_null_or_nullable,
                      //     dimension2_not_null_or_nullable,
                      //     dimension3_not_null_or_nullable,
                      // } => {
                      //     let d1 = dimension1_not_null_or_nullable;
                      //     let d1_rust = dimension1_not_null_or_nullable.rust();
                      //     let d2 = dimension2_not_null_or_nullable;
                      //     let d2_rust = dimension2_not_null_or_nullable.rust();
                      //     let d3 = dimension3_not_null_or_nullable;
                      //     let d3_rust = dimension3_not_null_or_nullable.rust();
                      //     (
                      //         format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{rust_type_name}"),
                      //         format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{postgresql_type_name}")
                      //     )
                      // },
                      // PostgresqlTypePattern::ArrayDimension4 {
                      //     dimension1_not_null_or_nullable,
                      //     dimension2_not_null_or_nullable,
                      //     dimension3_not_null_or_nullable,
                      //     dimension4_not_null_or_nullable,
                      // } => {
                      //     let d1 = dimension1_not_null_or_nullable;
                      //     let d1_rust = dimension1_not_null_or_nullable.rust();
                      //     let d2 = dimension2_not_null_or_nullable;
                      //     let d2_rust = dimension2_not_null_or_nullable.rust();
                      //     let d3 = dimension3_not_null_or_nullable;
                      //     let d3_rust = dimension3_not_null_or_nullable.rust();
                      //     let d4 = dimension4_not_null_or_nullable;
                      //     let d4_rust = dimension4_not_null_or_nullable.rust();
                      //     (
                      //         format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{vec_of_upper_camel_case}{d4_rust}{rust_type_name}"),
                      //         format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{array_of_upper_camel_case}{d4}{postgresql_type_name}")
                      //     )
                      // }
                };
                format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().unwrap()
            };
            let ident = &generate_ident_token_stream(postgresql_type, not_null_or_nullable, postgresql_type_pattern);
            let generate_ident_standart_not_null_token_stream = |postgresql_type: &PostgresqlType| generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlTypePattern::Standart);
            let ident_standart_not_null_upper_camel_case = &generate_ident_standart_not_null_token_stream(postgresql_type);
            let ident_token_stream = {
                quote::quote! {
                    #[derive(Debug)]
                    pub struct #ident;
                }
            };
            let ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);

            let field_type_standart_not_null = postgresql_type.field_type_token_stream();
            let generate_current_ident_origin_non_wrapping = |current_postgresql_type_pattern: &PostgresqlTypePattern, current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| {
                naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(postgresql_type, current_not_null_or_nullable, current_postgresql_type_pattern))
            };
            let field_type_handle: &dyn quote::ToTokens = {
                let generate_current_ident_origin = |current_postgresql_type_pattern: &PostgresqlTypePattern, current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| {
                    let value = generate_current_ident_origin_non_wrapping(current_postgresql_type_pattern, current_not_null_or_nullable);
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&value),
                    }
                };
                match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &field_type_standart_not_null,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                    },
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                        let (current_postgresql_type_pattern, current_not_null_or_nullable): (&PostgresqlTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&PostgresqlTypePattern::Standart, dimension1_not_null_or_nullable),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin(current_postgresql_type_pattern, current_not_null_or_nullable)
                    },
                    // PostgresqlTypePattern::ArrayDimension2 {
                    //     dimension1_not_null_or_nullable,
                    //     dimension2_not_null_or_nullable,
                    // } => &{
                    //     let (
                    //         current_postgresql_type_pattern,
                    //         current_not_null_or_nullable,
                    //     ): (
                    //         &PostgresqlTypePattern,
                    //         &postgresql_crud_macros_common::NotNullOrNullable,
                    //     ) = match &not_null_or_nullable {
                    //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                    //             &PostgresqlTypePattern::ArrayDimension1 {
                    //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                    //             },
                    //             &dimension1_not_null_or_nullable,
                    //         ),
                    //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                    //             &postgresql_type_pattern,
                    //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    //         )
                    //     };
                    //     generate_current_ident_origin(
                    //         &current_postgresql_type_pattern,
                    //         &current_not_null_or_nullable
                    //     )
                    // },
                    // PostgresqlTypePattern::ArrayDimension3 {
                    //     dimension1_not_null_or_nullable,
                    //     dimension2_not_null_or_nullable,
                    //     dimension3_not_null_or_nullable,
                    // } => &{
                    //     let (
                    //         current_postgresql_type_pattern,
                    //         current_not_null_or_nullable,
                    //     ): (
                    //         &PostgresqlTypePattern,
                    //         &postgresql_crud_macros_common::NotNullOrNullable,
                    //     ) = match &not_null_or_nullable {
                    //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                    //             &PostgresqlTypePattern::ArrayDimension2 {
                    //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                    //                 dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                    //             },
                    //             &dimension1_not_null_or_nullable,
                    //         ),
                    //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                    //             &postgresql_type_pattern,
                    //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    //         )
                    //     };
                    //     generate_current_ident_origin(
                    //         &current_postgresql_type_pattern,
                    //         &current_not_null_or_nullable
                    //     )
                    // },
                    // PostgresqlTypePattern::ArrayDimension4 {
                    //     dimension1_not_null_or_nullable,
                    //     dimension2_not_null_or_nullable,
                    //     dimension3_not_null_or_nullable,
                    //     dimension4_not_null_or_nullable,
                    // } => &{
                    //     let (
                    //         current_postgresql_type_pattern,
                    //         current_not_null_or_nullable,
                    //     ): (
                    //         &PostgresqlTypePattern,
                    //         &postgresql_crud_macros_common::NotNullOrNullable,
                    //     ) = match &not_null_or_nullable {
                    //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                    //             &PostgresqlTypePattern::ArrayDimension3 {
                    //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                    //                 dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                    //                 dimension3_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                    //             },
                    //             &dimension1_not_null_or_nullable,
                    //         ),
                    //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                    //             &postgresql_type_pattern,
                    //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    //         )
                    //     };
                    //     generate_current_ident_origin(
                    //         &current_postgresql_type_pattern,
                    //         &current_not_null_or_nullable
                    //     )
                    // },
                }
            };
            // println!("{}", quote::quote!{#field_type_handle});

            let generate_typical_query_bind_token_stream = |content_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                    #query_snake_case = #query_snake_case.bind(#content_token_stream);
                    #query_snake_case
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    #query_snake_case = #query_snake_case.bind(match #content_token_stream .0 {
                        Some(#value_snake_case) => Some(#value_snake_case),
                        None => None
                    });
                    #query_snake_case
                },
            };
            let typical_query_bind_token_stream = generate_typical_query_bind_token_stream(&value_snake_case);

            let sqlx_types_time_primitive_date_time_as_timestamp = PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp;
            let sqlx_types_time_date_as_date = PostgresqlType::SqlxTypesTimeDateAsDate;
            let sqlx_types_big_decimal_as_numeric = PostgresqlType::SqlxTypesBigDecimalAsNumeric;

            let sqlx_types_chrono_naive_date_time_as_timestamp_field_type_token_stream = PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp.field_type_token_stream();
            let sqlx_types_time_primitive_date_time_as_timestamp_field_type_token_stream = sqlx_types_time_primitive_date_time_as_timestamp.field_type_token_stream();
            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_timestamp_tz_field_type_token_stream = PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz.field_type_token_stream();
            let sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_timestamp_tz_field_type_token_stream = PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz.field_type_token_stream();
            let sqlx_types_chrono_naive_date_as_date_field_type_token_stream = PostgresqlType::SqlxTypesChronoNaiveDateAsDate.field_type_token_stream();
            let sqlx_types_time_date_as_date_field_type_token_stream = sqlx_types_time_date_as_date.field_type_token_stream();
            let sqlx_types_big_decimal_as_numeric_field_type_token_stream = sqlx_types_big_decimal_as_numeric.field_type_token_stream();

            let sqlx_postgres_types_pg_money_field_type_token_stream = PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney.field_type_token_stream();
            let sqlx_types_uuid_uuid_field_type_token_stream = PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient.field_type_token_stream();
            let sqlx_types_mac_address_mac_address_field_type_token_stream = PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr.field_type_token_stream();
            let sqlx_postgres_types_pg_interval_field_type_token_stream = PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval.field_type_token_stream();

            let time_month_token_stream = quote::quote! {time::Month};
            let crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream = quote::quote! {crate::postgresql_type::NumBigintBigInt};
            let sqlx_postgres_types_pg_range_token_stream = quote::quote! {sqlx::postgres::types::PgRange};
            let sqlx_types_time_time_midnight_token_stream = token_patterns::SqlxTypesTimeTimeMidnight;

            let generate_qlx_postgres_types_pg_range_start_end_token_stream = |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#sqlx_postgres_types_pg_range_token_stream {
                    #start_snake_case: #start_token_stream,
                    #end_snake_case: #end_token_stream
                }}
            };
            let generate_double_dot_space_tokens_token_stream = |value: &dyn quote::ToTokens| {
                quote::quote! {: #value}
            };
            let generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream = |months_token_stream: &dyn quote::ToTokens, days_token_stream: &dyn quote::ToTokens, microseconds_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#sqlx_postgres_types_pg_interval_field_type_token_stream {
                    #months_snake_case #months_token_stream,
                    #days_snake_case #days_token_stream,
                    #microseconds_snake_case #microseconds_token_stream
                }}
            };

            let (serde_serialize, serde_deserialize) = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&not_null_or_nullable, &postgresql_type_pattern) {
                let sqlx_types_time_primitive_date_time_as_not_null_timestamp_origin_upper_camel_case_token_stream = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_standart_not_null_token_stream(&sqlx_types_time_primitive_date_time_as_timestamp));
                let sqlx_types_time_date_as_not_null_date_origin_upper_camel_case_token_stream = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_standart_not_null_token_stream(&sqlx_types_time_date_as_date));
                let sqlx_types_big_decimal_as_not_null_numeric_origin_upper_camel_case_token_stream = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_standart_not_null_token_stream(&sqlx_types_big_decimal_as_numeric));

                let self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};
                enum ParameterNumber {
                    One,
                    Two,
                    Three,
                }
                impl ParameterNumber {
                    fn get_index(&self) -> std::primitive::usize {
                        match &self {
                            Self::One => 0,
                            Self::Two => 1,
                            Self::Three => 2,
                        }
                    }
                    fn get_index_starting_with_one(&self) -> std::primitive::usize {
                        match &self {
                            Self::One => 1,
                            Self::Two => 2,
                            Self::Three => 3,
                        }
                    }
                    fn get_vec_from_index_starting_with_one(&self) -> std::vec::Vec<std::primitive::usize> {
                        (0..self.get_index_starting_with_one()).collect()
                    }
                }
                let ident_standart_not_null_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_standart_not_null_upper_camel_case);
                let generate_match_std_collections_bound_token_stream = |match_token_stream: &dyn quote::ToTokens, init_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {match #match_token_stream {
                        std::collections::Bound::Included(#value_snake_case) => std::collections::Bound::Included(#init_token_stream),
                        std::collections::Bound::Excluded(#value_snake_case) => std::collections::Bound::Excluded(#init_token_stream),
                        std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                    }}
                };
                let generate_std_collections_bound_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {std::collections::Bound<#type_token_stream>}
                };
                let serde_serialize = {
                    let generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl _serde::Serialize for #ident_standart_not_null_origin_upper_camel_case {
                                    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                                    where
                                        __S: _serde::Serializer,
                                    {
                                        #content_token_stream
                                    }
                                }
                            };
                        }
                    };
                    let generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4 = |value_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #ident_standart_not_null_double_quotes_token_stream, &#self_dot_zero_token_stream #value_token_stream)}
                    };
                    let generate_serde_state_initialization_token_stream = |parameter_number: ParameterNumber| {
                        let parameter_number_token_stream = {
                            let value = parameter_number.get_vec_from_index_starting_with_one().into_iter().map(|_| quote::quote! {+ 1});
                            quote::quote! {#(#value)*}
                        };
                        quote::quote! {
                            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_standart_not_null_double_quotes_token_stream, false as std::primitive::usize #parameter_number_token_stream)?;
                        }
                    };
                    let serde_state_initialization_two_fields_token_stream = generate_serde_state_initialization_token_stream(ParameterNumber::Two);
                    let serde_state_initialization_three_fields_token_stream = generate_serde_state_initialization_token_stream(ParameterNumber::Three);
                    let generate_serialize_field_token_stream = |field_name: &dyn std::fmt::Display, third_parameter_token_stream: &dyn quote::ToTokens| {
                        let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_name);
                        quote::quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_double_quotes_token_stream, #third_parameter_token_stream)?;}
                    };
                    let serde_ser_serialize_struct_end_token_stream = quote::quote! {_serde::ser::SerializeStruct::end(__serde_state)};
                    let serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = {
                        let generate_self_zero_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {&#self_dot_zero_token_stream.#value_token_stream}
                        };
                        let start_serialize_field_token_stream = generate_serialize_field_token_stream(&start_snake_case, &generate_self_zero_tokens_token_stream(&start_snake_case));
                        let end_serialize_field_token_stream = generate_serialize_field_token_stream(&end_snake_case, &generate_self_zero_tokens_token_stream(&end_snake_case));
                        quote::quote! {
                            #serde_state_initialization_two_fields_token_stream
                            #start_serialize_field_token_stream
                            #end_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    };
                    enum IsNeedToBeCloned {
                        True,
                        False,
                    }
                    let generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25 = |type_token_stream: &dyn quote::ToTokens, is_need_to_be_cloned: IsNeedToBeCloned| {
                        let maybe_clone_token_stream: &dyn quote::ToTokens = match &is_need_to_be_cloned {
                            IsNeedToBeCloned::True => &quote::quote! {.clone()},
                            IsNeedToBeCloned::False => &proc_macro2_token_stream_new,
                        };
                        let generate_self_zero_match_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
                            let token_stream = generate_match_std_collections_bound_token_stream(&quote::quote! {#self_dot_zero_token_stream.#value_token_stream #maybe_clone_token_stream}, &quote::quote! {#type_token_stream(#value_snake_case)});
                            quote::quote! {&#token_stream}
                        };
                        let start_serialize_field_token_stream = generate_serialize_field_token_stream(&start_snake_case, &generate_self_zero_match_tokens_token_stream(&start_snake_case));
                        let end_serialize_field_token_stream = generate_serialize_field_token_stream(&end_snake_case, &generate_self_zero_match_tokens_token_stream(&end_snake_case));
                        quote::quote! {
                            #serde_state_initialization_two_fields_token_stream
                            #start_serialize_field_token_stream
                            #end_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    };
                    let impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream =
                        generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
                    let impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.to_string()}));
                    match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.0}))),
                        PostgresqlType::SqlxTypesBigDecimalAsNumeric => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            let digits_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::DigitsSnakeCase, &quote::quote! {&#crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream(bigint)});
                            let scale_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::ScaleSnakeCase, &quote::quote! {&exponent});
                            quote::quote! {
                                let (bigint, exponent) = #self_dot_zero_token_stream.clone().into_bigint_and_exponent();
                                #serde_state_initialization_two_fields_token_stream
                                #digits_serialize_field_token_stream
                                #scale_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            let generate_serialize_field_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&#self_dot_zero_token_stream.#value});
                            let months_serialize_field_token_stream = generate_serialize_field_token_stream(&months_snake_case);
                            let days_serialize_field_token_stream = generate_serialize_field_token_stream(&days_snake_case);
                            let microseconds_serialize_field_token_stream = generate_serialize_field_token_stream(&microseconds_snake_case);
                            quote::quote! {
                                #serde_state_initialization_three_fields_token_stream
                                #months_serialize_field_token_stream
                                #days_serialize_field_token_stream
                                #microseconds_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesTimeDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            let generate_self_zero_tokens_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&#self_dot_zero_token_stream.#value()});
                            let year_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&year_snake_case);
                            let month_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&month_snake_case);
                            let day_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&day_snake_case);
                            quote::quote! {
                                #serde_state_initialization_three_fields_token_stream
                                #year_serialize_field_token_stream
                                #month_serialize_field_token_stream
                                #day_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.bytes()})))
                        }
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_big_decimal_as_not_null_numeric_origin_upper_camel_case_token_stream, IsNeedToBeCloned::True))
                        }),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_time_date_as_not_null_date_origin_upper_camel_case_token_stream, IsNeedToBeCloned::False))
                        }),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(
                                &sqlx_types_time_primitive_date_time_as_not_null_timestamp_origin_upper_camel_case_token_stream,
                                IsNeedToBeCloned::False,
                            ))
                        }),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream)
                        }
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream)
                        }
                    }
                };
                let serde_deserialize = {
                    let struct_ident_double_quotes_token_stream = postgresql_crud_macros_common::generate_struct_ident_double_quotes_token_stream(&postgresql_type);
                    let postgresql_type_visitor_upper_camel_case = naming::parameter::SelfVisitorUpperCamelCase::from_tokens(&postgresql_type);

                    let struct_visitor_token_stream = quote::quote! {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<#ident_standart_not_null_origin_upper_camel_case>,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                    };

                    let digits_scale_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&digits_snake_case, &scale_snake_case];
                    let year_month_day_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&year_snake_case, &month_snake_case, &day_snake_case];
                    let start_end_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&start_snake_case, &end_snake_case];
                    let months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&months_snake_case, &days_snake_case, &microseconds_snake_case];

                    let (serde_deserializer_deserialize_struct_visitor_token_stream, serde_deserializer_deserialize_struct_ident_visitor_token_stream) = {
                        let generate_serde_deserializer_deserialize_struct_visitor_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                _serde::Deserializer::deserialize_struct(
                                    __deserializer,
                                    #ident_standart_not_null_double_quotes_token_stream,
                                    FIELDS,
                                    #content_token_stream
                                )
                            }
                        };
                        (
                            generate_serde_deserializer_deserialize_struct_visitor_token_stream(&quote::quote! {
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<#ident_standart_not_null_origin_upper_camel_case>,
                                    lifetime: _serde::__private::PhantomData,
                                }
                            }),
                            generate_serde_deserializer_deserialize_struct_visitor_token_stream(&postgresql_type_visitor_upper_camel_case),
                        )
                    };

                    let serde_deserializer_deserialize_newtype_struct_token_stream = quote::quote! {
                        _serde::Deserializer::deserialize_newtype_struct(
                            __deserializer,
                            #ident_standart_not_null_double_quotes_token_stream,
                            __Visitor {
                                marker: serde::__private::PhantomData::<#ident_standart_not_null_origin_upper_camel_case>,
                                lifetime: serde::__private::PhantomData,
                            },
                        )
                    };

                    let generate_impl_serde_deserialize_for_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for #ident_standart_not_null_origin_upper_camel_case {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        #content_token_stream
                                    }
                                }
                            };
                        }
                    };

                    let parameter_number_two = ParameterNumber::Two;
                    let parameter_number_three = ParameterNumber::Three;

                    let generate_field_index_token_stream = |index: std::primitive::usize| format!("__{}{index}", naming::FieldSnakeCase).parse::<proc_macro2::TokenStream>().unwrap();

                    let (enum_field_two_token_stream, enum_field_three_token_stream) = {
                        let generate_enum_field_token_stream = |parameter_number: &ParameterNumber| {
                            let fields_token_stream = {
                                let fields_token_stream = parameter_number.get_vec_from_index_starting_with_one().into_iter().map(&generate_field_index_token_stream);
                                quote::quote! {#(#fields_token_stream),*}
                            };
                            quote::quote! {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    #fields_token_stream,
                                    __ignore,
                                }
                            }
                        };
                        (generate_enum_field_token_stream(&parameter_number_two), generate_enum_field_token_stream(&parameter_number_three))
                    };

                    let (fn_expecting_struct_ident_double_quotes_token_stream, fn_expecting_field_identifier_token_stream, fn_expecting_months_or_days_or_microseconds_token_stream, fn_expecting_start_or_end_token_stream) = {
                        let generate_fn_expecting_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(__f, #content_token_stream)
                                }
                            }
                        };
                        (
                            generate_fn_expecting_token_stream(&struct_ident_double_quotes_token_stream),
                            generate_fn_expecting_token_stream(&quote::quote! {"field identifier"}),
                            generate_fn_expecting_token_stream(&quote::quote! {"`months` or `days` or `microseconds`"}),
                            generate_fn_expecting_token_stream(&quote::quote! {"`start` or `end`"}),
                        )
                    };

                    let field_0_token_stream = generate_field_index_token_stream(ParameterNumber::One.get_index());
                    let field_1_token_stream = generate_field_index_token_stream(ParameterNumber::Two.get_index());
                    let field_2_token_stream = generate_field_index_token_stream(ParameterNumber::Three.get_index());

                    let generate_serde_private_ok_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {serde::__private::Ok(#content_token_stream)}
                    };
                    let generate_serde_private_ok_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_serde_private_ok_token_stream(&quote::quote! {#ident_standart_not_null_origin_upper_camel_case(#content_token_stream)});

                    let match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream = quote::quote! {match #sqlx_types_uuid_uuid_field_type_token_stream::try_parse(&#field_0_token_stream) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }};
                    let sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream = quote::quote! {#sqlx_types_mac_address_mac_address_field_type_token_stream::new(#field_0_token_stream)};
                    let array_std_primitive_u8_6_token_stream = quote::quote! {[std::primitive::u8; 6]};
                    let (fn_visit_newtype_struct_pg_money_token_stream, fn_visit_newtype_struct_uuid_token_stream, fn_visit_newtype_struct_mac_address_token_stream) = {
                        let generate_fn_visit_newtype_struct_token_stream = |type_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                            let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(content_token_stream);
                            quote::quote! {
                                #[inline]
                                fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                                where
                                    __E: serde::Deserializer<'de>,
                                {
                                    let #field_0_token_stream = <#type_token_stream as serde::Deserialize>::deserialize(__e)?;
                                    #serde_private_ok_postgresql_type_token_stream
                                }
                            }
                        };
                        (
                            generate_fn_visit_newtype_struct_token_stream(&std_primitive_i64_token_stream, &quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#field_0_token_stream)}),
                            generate_fn_visit_newtype_struct_token_stream(&std_string_string_token_stream, &match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream),
                            generate_fn_visit_newtype_struct_token_stream(&array_std_primitive_u8_6_token_stream, &sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream),
                        )
                    };

                    let generate_fn_visit_seq_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::SeqAccess<'de>,
                            {
                                #content_token_stream
                            }
                        }
                    };

                    let generate_fields_serde_de_seq_access_next_element_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                        let error_message_token_stream = postgresql_crud_macros_common::generate_struct_ident_with_number_elements_double_quotes_token_stream(&postgresql_type, vec_token_stream.len());
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                            let field_index_token_stream = generate_field_index_token_stream(index);
                            let index_usize_token_stream = format!("{index}usize").parse::<proc_macro2::TokenStream>().unwrap();
                            quote::quote! {
                                let #field_index_token_stream = match serde::de::SeqAccess::next_element::<#element>(&mut __seq)? {
                                    serde::__private::Some(__value) => __value,
                                    serde::__private::None => {
                                        return serde::__private::Err(serde::de::Error::invalid_length(#index_usize_token_stream, &#error_message_token_stream));
                                    }
                                };
                            }
                        });
                        quote::quote! {#(#fields_initialization_token_stream)*}
                    };

                    let serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#sqlx_types_big_decimal_as_numeric_field_type_token_stream::new(
                        #field_0_token_stream.0,
                        #field_1_token_stream
                    )});

                    let fn_visit_seq_pg_money_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i64_token_stream]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#field_0_token_stream)});
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream, &std_primitive_i64_token_stream]);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream
                        }
                    });

                    let sqlx_types_time_date_specific_initialization = quote::quote! {
                        match sqlx::types::time::Date::from_calendar_date(#field_0_token_stream, #field_1_token_stream, #field_2_token_stream) {
                            Ok(value) => {
                                let minimum = sqlx::types::time::Date::from_calendar_date(-4713, time::Month::December, 31).unwrap();
                                if minimum > value {
                                    Err(_serde::de::Error::custom(format!("SqlxTypesTimeDate less than minimum postgresql value {value:?}")))
                                } else {
                                    _serde::__private::Ok(#ident_standart_not_null_origin_upper_camel_case(value))
                                }
                            },
                            Err(value) => Err(_serde::de::Error::custom(format!("SqlxTypesTimeDate from calendar date {value:?}")))
                        }
                    };

                    let (seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream, seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream, seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream) = {
                        let generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream = |parameter_number: &ParameterNumber| {
                            let index_token_stream = match &parameter_number {
                                ParameterNumber::One => quote::quote! {0},
                                ParameterNumber::Two => quote::quote! {1},
                                ParameterNumber::Three => quote::quote! {2},
                            };
                            quote::quote! {__seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(#index_token_stream, &self))?;}
                        };
                        (
                            generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::One),
                            generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::Two),
                            generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::Three),
                        )
                    };
                    let fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream = generate_fn_visit_seq_token_stream(&{
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(&proc_macro2_token_stream_new, &proc_macro2_token_stream_new, &proc_macro2_token_stream_new));
                        quote::quote! {
                            let #months_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                            let #days_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                            let #microseconds_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let sqlx_postgres_types_pg_range_start_end_token_stream = generate_qlx_postgres_types_pg_range_start_end_token_stream(&field_0_token_stream, &field_1_token_stream);
                    let sqlx_postgres_types_pg_range_bound_start_end_token_stream = {
                        let value_zero_token_stream = quote::quote! {#value_snake_case.0};
                        generate_qlx_postgres_types_pg_range_start_end_token_stream(
                            &generate_match_std_collections_bound_token_stream(&field_0_token_stream, &value_zero_token_stream),
                            &generate_match_std_collections_bound_token_stream(&field_1_token_stream, &value_zero_token_stream),
                        )
                    };
                    let fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream = generate_fn_visit_seq_token_stream(&{
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            let #field_0_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                            let #field_1_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_timestamp_field_type_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_time_primitive_date_time_as_not_null_timestamp_origin_upper_camel_case_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream, &std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_timestamp_tz_field_type_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_timestamp_tz_field_type_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let std_collections_bound_sqlx_types_chrono_naive_date_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_as_date_field_type_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let std_collections_bound_sqlx_types_time_date_as_date_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_time_date_as_not_null_date_origin_upper_camel_case_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_date_as_date_token_stream, &std_collections_bound_sqlx_types_time_date_as_date_token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = {
                            let token_stream = generate_std_collections_bound_token_stream(&sqlx_types_big_decimal_as_not_null_numeric_origin_upper_camel_case_token_stream);
                            generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&token_stream, &token_stream])
                        };
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_types_uuid_uuid_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_string_string_token_stream]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&array_std_primitive_u8_6_token_stream]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });

                    let (fn_visit_u64_two_token_stream, fn_visit_u64_three_token_stream) = {
                        let generate_fn_visit_u64_token_stream = |parameter_number: &ParameterNumber| {
                            let fields_token_stream = {
                                parameter_number.get_vec_from_index_starting_with_one().into_iter().map(|element| {
                                    let index_variant_token_stream = format!("{element}u64").parse::<proc_macro2::TokenStream>().unwrap();
                                    let field_index_token_stream = generate_field_index_token_stream(element);
                                    quote::quote! {#index_variant_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)}
                                })
                            };
                            quote::quote! {
                                fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
                                where
                                    __E: serde::de::Error,
                                {
                                    match __value {
                                        #(#fields_token_stream),*,
                                        _ => serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                        };
                        (generate_fn_visit_u64_token_stream(&ParameterNumber::Two), generate_fn_visit_u64_token_stream(&ParameterNumber::Three))
                    };
                    let (fn_visit_str_value_digits_scale_token_stream, fn_visit_str_value_year_month_day_token_stream, fn_visit_str_value_start_end_token_stream) = {
                        let generate_fn_visit_str_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let fields_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                                let element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                                let field_index_name_token_stream = generate_field_index_token_stream(index);
                                quote::quote! {
                                    #element_double_quotes_token_stream => _serde::__private::Ok(__Field::#field_index_name_token_stream)
                                }
                            });
                            quote::quote! {
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        #(#fields_token_stream),*,
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                        };
                        (
                            generate_fn_visit_str_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };

                    let (fn_visit_str_field_months_days_microseconds_token_stream, fn_visit_str_field_start_end_token_stream) = {
                        let generate_fn_visit_str_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let fields_token_stream = vec_token_stream.iter().map(|element| {
                                let element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                                let element_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element);
                                quote::quote! {#element_double_quotes_token_stream => Ok(Field::#element_upper_camel_case_token_stream)}
                            });
                            quote::quote! {
                                fn visit_str<E>(self, value: &str) -> Result<Field, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        #(#fields_token_stream),*,
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                        };
                        (
                            generate_fn_visit_str_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };

                    let (fn_visit_bytes_digits_scale_token_stream, fn_visit_bytes_year_month_day_token_stream, fn_visit_bytes_start_end_token_stream) = {
                        let generate_fn_visit_bytes_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let fields_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                                let b_element_double_quotes_token_stream = format!("b{}", generate_quotes::double_quotes_stringified(&element)).parse::<proc_macro2::TokenStream>().unwrap();
                                let field_index_name_token_stream = generate_field_index_token_stream(index);
                                quote::quote! {
                                    #b_element_double_quotes_token_stream => serde::__private::Ok(__Field::#field_index_name_token_stream)
                                }
                            });
                            quote::quote! {
                                fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                                where
                                    __E: serde::de::Error,
                                {
                                    match __value {
                                        #(#fields_token_stream),*,
                                        _ => serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                        };
                        (
                            generate_fn_visit_bytes_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };

                    let serde_deserializer_deserialize_identifier_token_stream = quote::quote! {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    };

                    let impl_serde_deserialize_for_field_token_stream = quote::quote! {
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                #serde_deserializer_deserialize_identifier_token_stream
                            }
                        }
                    };

                    let (
                        std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                        std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream,
                        std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                        std_collections_bound_sqlx_types_time_date_as_date_token_stream,
                        std_collections_bound_sqlx_types_big_decimal_as_numeric_token_stream,
                    ) = {
                        (
                            generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_timestamp_field_type_token_stream),
                            generate_std_collections_bound_token_stream(&sqlx_types_time_primitive_date_time_as_not_null_timestamp_origin_upper_camel_case_token_stream),
                            generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_timestamp_tz_field_type_token_stream),
                            generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_timestamp_tz_field_type_token_stream),
                            generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_as_date_field_type_token_stream),
                            generate_std_collections_bound_token_stream(&sqlx_types_time_date_as_not_null_date_origin_upper_camel_case_token_stream),
                            generate_std_collections_bound_token_stream(&sqlx_types_big_decimal_as_not_null_numeric_origin_upper_camel_case_token_stream),
                        )
                    };

                    let (
                        fn_visit_map_sqlx_types_big_decimal_token_stream,
                        fn_visit_map_sqlx_types_time_date_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                    ) = {
                        let generate_fn_visit_map_token_stream =
                            |field_option_none_initialization_token_stream: &dyn quote::ToTokens, while_some_next_key_field_token_stream: &dyn quote::ToTokens, match_field_initialization_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
                                quote::quote! {
                                    #[inline]
                                    fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: serde::de::MapAccess<'de>,
                                    {
                                        #field_option_none_initialization_token_stream
                                        #while_some_next_key_field_token_stream
                                        #match_field_initialization_token_stream
                                        #serde_private_ok_token_stream
                                    }
                                }
                            };

                        let (
                            field_option_none_initialization_sqlx_types_big_decimal_token_stream,
                            field_option_none_initialization_sqlx_types_time_date_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        ) = {
                            let generate_field_option_none_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                                let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                                    let field_index_name_token_stream = generate_field_index_token_stream(index);
                                    quote::quote! {
                                        let mut #field_index_name_token_stream: serde::__private::Option<#element> = serde::__private::None;
                                    }
                                });
                                quote::quote! {#(#fields_initialization_token_stream)*}
                            };
                            (
                                generate_field_option_none_initialization_token_stream(&[&crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream, &std_primitive_i64_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_primitive_i32_token_stream, &time_month_token_stream, &std_primitive_u8_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream, &std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_date_as_date_token_stream, &std_collections_bound_sqlx_types_time_date_as_date_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_big_decimal_as_numeric_token_stream, &std_collections_bound_sqlx_types_big_decimal_as_numeric_token_stream]),
                            )
                        };

                        let (
                            while_some_next_key_field_sqlx_types_big_decimal_token_stream,
                            while_some_next_key_field_sqlx_types_time_date_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        ) = {
                            let generate_while_some_next_key_field_token_stream = |vec_token_stream: &[(&dyn std::fmt::Display, &dyn quote::ToTokens)]| {
                                let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                                    let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element.0);
                                    let field_type_token_stream = &element.1;
                                    let field_index_name_token_stream = generate_field_index_token_stream(index);
                                    quote::quote! {
                                        __Field::#field_index_name_token_stream => {
                                            if serde::__private::Option::is_some(&#field_index_name_token_stream) {
                                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field(#field_name_double_quotes_token_stream));
                                            }
                                            #field_index_name_token_stream = serde::__private::Some(serde::de::MapAccess::next_value::<#field_type_token_stream>(&mut __map)?);
                                        }
                                    }
                                });
                                quote::quote! {
                                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                        match __key {
                                            #(#fields_initialization_token_stream)*
                                            _ => {
                                                let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                            }
                                        }
                                    }
                                }
                            };
                            (
                                generate_while_some_next_key_field_token_stream(&[(&digits_snake_case, &crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream), (&scale_snake_case, &std_primitive_i64_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&year_snake_case, &std_primitive_i32_token_stream), (&month_snake_case, &time_month_token_stream), (&day_snake_case, &std_primitive_u8_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[
                                    (&start_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream),
                                    (&end_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_timestamp_token_stream),
                                ]),
                                generate_while_some_next_key_field_token_stream(&[
                                    (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                                    (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                                ]),
                                generate_while_some_next_key_field_token_stream(&[
                                    (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream),
                                    (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream),
                                ]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_time_date_as_date_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_time_date_as_date_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_numeric_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_numeric_token_stream)]),
                            )
                        };

                        let (match_field_initialization_sqlx_types_big_decimal_token_stream, match_field_initialization_sqlx_types_time_date_token_stream, match_field_initialization_start_end_token_stream) = {
                            let generate_match_field_initialization_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                                let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                                    let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element);
                                    let field_index_name_token_stream = generate_field_index_token_stream(index);
                                    quote::quote! {
                                        let #field_index_name_token_stream = match #field_index_name_token_stream {
                                            serde::__private::Some(#field_index_name_token_stream) => #field_index_name_token_stream,
                                            serde::__private::None => serde::__private::de::missing_field(#field_name_double_quotes_token_stream)?,
                                        };
                                    }
                                });
                                quote::quote! {#(#fields_initialization_token_stream)*}
                            };
                            (
                                generate_match_field_initialization_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                            )
                        };

                        let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                        (
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_types_big_decimal_token_stream,
                                &while_some_next_key_field_sqlx_types_big_decimal_token_stream,
                                &match_field_initialization_sqlx_types_big_decimal_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_types_time_date_token_stream,
                                &while_some_next_key_field_sqlx_types_time_date_token_stream,
                                &match_field_initialization_sqlx_types_time_date_token_stream,
                                &sqlx_types_time_date_specific_initialization,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                            ),
                        )
                    };

                    let (fn_visit_map_sqlx_postgres_types_pg_interval_token_stream, fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream) = {
                        let generate_fn_visit_map_token_stream =
                            |field_option_none_initialization_token_stream: &dyn quote::ToTokens, while_some_next_key_field_token_stream: &dyn quote::ToTokens, match_field_initialization_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
                                let serde_private_ok_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&serde_private_ok_token_stream);
                                quote::quote! {
                                    #[inline]
                                    fn visit_map<V>(self, mut map: V) -> Result<#ident_standart_not_null_origin_upper_camel_case, V::Error>
                                    where
                                        V: serde::de::MapAccess<'de>,
                                    {
                                        #field_option_none_initialization_token_stream
                                        #while_some_next_key_field_token_stream
                                        #match_field_initialization_token_stream
                                        #serde_private_ok_token_stream
                                    }
                                }
                            };

                        let (field_option_none_initialization_months_days_microseconds_token_stream, field_option_none_initialization_start_end_token_stream) = {
                            let generate_field_option_none_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                                let fields_initialization_token_stream = vec_token_stream.iter().map(|element| {
                                    quote::quote! {
                                        let mut #element = None;
                                    }
                                });
                                quote::quote! {#(#fields_initialization_token_stream)*}
                            };
                            (
                                generate_field_option_none_initialization_token_stream(&[&months_snake_case, &days_snake_case, &microseconds_snake_case]),
                                generate_field_option_none_initialization_token_stream(&[&start_snake_case, &end_snake_case]),
                            )
                        };

                        let (while_some_next_key_field_months_days_microseconds_token_stream, while_some_next_key_field_start_end_token_stream) = {
                            let generate_while_some_next_key_field_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                                let fields_initialization_token_stream = vec_token_stream.iter().map(|element| {
                                    let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                                    let element_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element);
                                    quote::quote! {
                                        Field::#element_upper_camel_case_token_stream => {
                                            if #element.is_some() {
                                                return Err(serde::de::Error::duplicate_field(#field_name_double_quotes_token_stream));
                                            }
                                            #element = Some(map.next_value()?);
                                        }
                                    }
                                });
                                quote::quote! {
                                    while let Some(key) = map.next_key()? {
                                        match key {
                                            #(#fields_initialization_token_stream)*
                                        }
                                    }
                                }
                            };
                            (
                                generate_while_some_next_key_field_token_stream(&[&months_snake_case, &days_snake_case, &microseconds_snake_case]),
                                generate_while_some_next_key_field_token_stream(&[&start_snake_case, &end_snake_case]),
                            )
                        };

                        let (match_field_initialization_months_days_microseconds_token_stream, match_field_initialization_start_end_token_stream) = {
                            let generate_match_field_initialization_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                                let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)| {
                                    let field_index_name_token_stream = generate_field_index_token_stream(index);
                                    let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element);
                                    quote::quote! {
                                        let #field_index_name_token_stream = #element.ok_or_else(|| serde::de::Error::missing_field(#field_name_double_quotes_token_stream))?;
                                    }
                                });
                                quote::quote! {#(#fields_initialization_token_stream)*}
                            };
                            (
                                generate_match_field_initialization_token_stream(&[&months_snake_case, &days_snake_case, &microseconds_snake_case]),
                                generate_match_field_initialization_token_stream(&[&start_snake_case, &end_snake_case]),
                            )
                        };
                        (
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_months_days_microseconds_token_stream,
                                &while_some_next_key_field_months_days_microseconds_token_stream,
                                &match_field_initialization_months_days_microseconds_token_stream,
                                &generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                                    &generate_double_dot_space_tokens_token_stream(&field_0_token_stream),
                                    &generate_double_dot_space_tokens_token_stream(&field_1_token_stream),
                                    &generate_double_dot_space_tokens_token_stream(&field_2_token_stream),
                                ),
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_start_end_token_stream,
                                &while_some_next_key_field_start_end_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                        )
                    };

                    let (field_months_days_microseconds_token_stream, field_start_end_token_stream) = {
                        let generate_field_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let variants_token_stream = vec_token_stream.iter().map(|element| naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element));
                            quote::quote! {
                                enum Field {
                                    #(#variants_token_stream),*
                                }
                            }
                        };
                        (
                            generate_field_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                            generate_field_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };

                    let (const_fields_sqlx_types_big_decimal_token_stream, const_fields_sqlx_types_time_date_token_stream, const_fields_sqlx_postgres_types_pg_interval_token_stream, const_fields_start_end_token_stream) = {
                        let generate_const_fields_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let field_names_token_stream = vec_token_stream.iter().map(|element| generate_quotes::double_quotes_token_stream(&element));
                            quote::quote! {
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[#(#field_names_token_stream),*];
                            }
                        };
                        (
                            generate_const_fields_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };

                    let (
                        impl_serde_de_visitor_for_visitor_pg_money_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_types_big_decimal_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_types_time_date_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream,
                        impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream,
                    ) = {
                        let generate_impl_serde_de_visitor_for_visitor_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = #ident_standart_not_null_origin_upper_camel_case;
                                    #fn_expecting_struct_ident_double_quotes_token_stream
                                    #first_token_stream
                                    #second_token_stream
                                }
                            }
                        };
                        (
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_pg_money_token_stream, &fn_visit_seq_pg_money_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_types_big_decimal_token_stream, &fn_visit_map_sqlx_types_big_decimal_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(
                                &generate_fn_visit_seq_token_stream(&{
                                    let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i32_token_stream, &time_month_token_stream, &std_primitive_u8_token_stream]);
                                    quote::quote! {
                                        #fields_initialization_token_stream
                                        #sqlx_types_time_date_specific_initialization
                                    }
                                }),
                                &fn_visit_map_sqlx_types_time_date_token_stream,
                            ),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(
                                &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                                &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                            ),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(
                                &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            ),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(
                                &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                                &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                            ),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_uuid_token_stream, &fn_visit_seq_sqlx_types_uuid_uuid_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_mac_address_token_stream, &fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream),
                        )
                    };

                    let generate_impl_serde_de_visitor_for_tokens_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            impl<'de> _serde::de::Visitor<'de> for #ident_token_stream {
                                #content_token_stream
                            }
                        }
                    };

                    let (
                        impl_serde_de_visitor_for_field_visitor_token_stream_8ae918a4_5464_4f56_8078_ab475f269079,
                        impl_serde_de_visitor_for_field_visitor_token_stream_77c8b6d8_4ac3_4551_8498_36b9d77317f2,
                        impl_serde_de_visitor_for_field_visitor_token_stream_31609291_37e6_427f_8d04_d19e2af929f8,
                        impl_serde_de_visitor_for_field_visitor_token_stream_ca843915_2330_4969_8bc8_8b33bff7a565,
                        impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6,
                    ) = {
                        let generate_impl_serde_de_visitor_for_field_visitor_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            let field_visitor_token_stream = quote::quote! {__FieldVisitor};
                            let impl_serde_de_visitor_for_tokens_token_stream = generate_impl_serde_de_visitor_for_tokens_token_stream(&field_visitor_token_stream, &content_token_stream);
                            quote::quote! {
                                #[doc(hidden)]
                                struct #field_visitor_token_stream;
                                #impl_serde_de_visitor_for_tokens_token_stream
                            }
                        };
                        (
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                type Value = __Field;
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_two_token_stream
                                #fn_visit_str_value_digits_scale_token_stream
                                #fn_visit_bytes_digits_scale_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                type Value = __Field;
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_three_token_stream
                                #fn_visit_str_value_year_month_day_token_stream
                                #fn_visit_bytes_year_month_day_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                type Value = Field;
                                #fn_expecting_months_or_days_or_microseconds_token_stream
                                #fn_visit_str_field_months_days_microseconds_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                type Value = Field;
                                #fn_expecting_start_or_end_token_stream
                                #fn_visit_str_field_start_end_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                type Value = __Field;
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_two_token_stream
                                #fn_visit_str_value_start_end_token_stream
                                #fn_visit_bytes_start_end_token_stream
                            }),
                        )
                    };

                    let (impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_interval_token_stream, impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream) = {
                        let generate_impl_serde_de_visitor_for_ident_visitor_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                            let impl_serde_de_visitor_for_tokens_token_stream = generate_impl_serde_de_visitor_for_tokens_token_stream(
                                &postgresql_type_visitor_upper_camel_case,
                                &quote::quote! {
                                    type Value = #ident_standart_not_null_origin_upper_camel_case;
                                    #fn_expecting_struct_ident_double_quotes_token_stream
                                    #first_token_stream
                                    #second_token_stream
                                },
                            );
                            quote::quote! {
                                struct #postgresql_type_visitor_upper_camel_case;
                                #impl_serde_de_visitor_for_tokens_token_stream
                            }
                        };
                        (
                            generate_impl_serde_de_visitor_for_ident_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream, &fn_visit_map_sqlx_postgres_types_pg_interval_token_stream),
                            generate_impl_serde_de_visitor_for_ident_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream),
                        )
                    };

                    let (impl_serde_deserialize_for_field_sqlx_postgres_types_pg_interval_token_stream, impl_serde_deserialize_for_field_token_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_stream) = {
                        let generate_impl_serde_deserialize_for_field_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                impl<'de> serde::Deserialize<'de> for Field {
                                    fn deserialize<D>(__deserializer: D) -> Result<Field, D::Error>
                                    where
                                        D: serde::Deserializer<'de>,
                                    {
                                        #content_token_stream
                                        #serde_deserializer_deserialize_identifier_token_stream
                                    }
                                }
                            }
                        };
                        (
                            generate_impl_serde_deserialize_for_field_token_stream(&impl_serde_de_visitor_for_field_visitor_token_stream_31609291_37e6_427f_8d04_d19e2af929f8),
                            generate_impl_serde_deserialize_for_field_token_stream(&impl_serde_de_visitor_for_field_visitor_token_stream_ca843915_2330_4969_8bc8_8b33bff7a565),
                        )
                    };
                    let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #field_start_end_token_stream
                            #impl_serde_deserialize_for_field_token_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_stream
                            #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                        }
                    });
                    let impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        }
                    });
                    let generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream = |impl_serde_de_visitor_for_visitor_tokens_token_stream: &dyn quote::ToTokens| {
                        generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_tokens_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })
                    };
                    match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&{
                            quote::quote! {
                                #struct_visitor_token_stream
                                #impl_serde_de_visitor_for_visitor_pg_money_token_stream
                                #serde_deserializer_deserialize_newtype_struct_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesBigDecimalAsNumeric => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&{
                            quote::quote! {
                                #enum_field_two_token_stream
                                #impl_serde_de_visitor_for_field_visitor_token_stream_8ae918a4_5464_4f56_8078_ab475f269079
                                #impl_serde_deserialize_for_field_token_stream
                                #struct_visitor_token_stream
                                #impl_serde_de_visitor_for_visitor_sqlx_types_big_decimal_token_stream
                                #const_fields_sqlx_types_big_decimal_token_stream
                                #serde_deserializer_deserialize_struct_visitor_token_stream
                            }
                        })),
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        //default deserialize impl can cause an postgresql error "date of out range". pub const fn from_ordinal_date( do it too. if u want to check it just use sqlx::types::time::Date::MIN
                        PostgresqlType::SqlxTypesTimeDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&{
                            quote::quote! {
                                #enum_field_three_token_stream
                                #impl_serde_de_visitor_for_field_visitor_token_stream_77c8b6d8_4ac3_4551_8498_36b9d77317f2
                                #impl_serde_deserialize_for_field_token_stream
                                #struct_visitor_token_stream
                                #impl_serde_de_visitor_for_visitor_sqlx_types_time_date_token_stream
                                #const_fields_sqlx_types_time_date_token_stream
                                #serde_deserializer_deserialize_struct_visitor_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&{
                            quote::quote! {
                                #struct_visitor_token_stream
                                #impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream
                                #serde_deserializer_deserialize_newtype_struct_token_stream
                            }
                        })),
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&{
                            quote::quote! {
                                #field_months_days_microseconds_token_stream
                                #impl_serde_deserialize_for_field_sqlx_postgres_types_pg_interval_token_stream
                                #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_interval_token_stream
                                #const_fields_sqlx_postgres_types_pg_interval_token_stream
                                #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                            }
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(
                                generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream)
                            )
                        }
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(
                            &impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        )),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream))
                        }
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&{
                            quote::quote! {
                                #enum_field_two_token_stream
                                #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                                #impl_serde_deserialize_for_field_token_stream
                                #struct_visitor_token_stream
                                #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream
                                #const_fields_start_end_token_stream
                                #serde_deserializer_deserialize_struct_visitor_token_stream
                            }
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream))
                        }
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream)
                        }),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream)
                        }),
                    }
                };
                (serde_serialize, serde_deserialize)
            } else {
                (postgresql_crud_macros_common::DeriveOrImpl::Derive, postgresql_crud_macros_common::DeriveOrImpl::Derive)
            };
            let impl_new_for_ident_origin_type_token_stream = match &element.postgresql_type_pattern {
                PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &field_type_standart_not_null,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {std::option::Option<#field_type_standart_not_null>},
                },
                PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {#field_type_standart_not_null});
                    not_null_or_nullable.maybe_option_wrap(quote::quote! {std::vec::Vec<#dimension1_type>})
                },
                // PostgresqlTypePattern::ArrayDimension2 {
                //     dimension1_not_null_or_nullable,
                //     dimension2_not_null_or_nullable,
                // } => &{
                //     let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote!{#field_type_standart_not_null});
                //     let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension2_type>});
                //     not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension1_type>})
                // },
                // PostgresqlTypePattern::ArrayDimension3 {
                //     dimension1_not_null_or_nullable,
                //     dimension2_not_null_or_nullable,
                //     dimension3_not_null_or_nullable,
                // } => &{
                //     let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote!{#field_type_standart_not_null});
                //     let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension3_type>});
                //     let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension2_type>});
                //     not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension1_type>})
                // },
                // PostgresqlTypePattern::ArrayDimension4 {
                //     dimension1_not_null_or_nullable,
                //     dimension2_not_null_or_nullable,
                //     dimension3_not_null_or_nullable,
                //     dimension4_not_null_or_nullable,
                // } => &{
                //     let dimension4_type = dimension4_not_null_or_nullable.maybe_option_wrap(quote::quote!{#field_type_standart_not_null});
                //     let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension4_type>});
                //     let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension3_type>});
                //     let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension2_type>});
                //     not_null_or_nullable.maybe_option_wrap(quote::quote!{std::vec::Vec<#dimension1_type>})
                // },
            };
            let ident_origin_token_stream = {
                let ident_origin_token_stream = {
                    let maybe_derive_partial_ord_token_stream = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&not_null_or_nullable, &postgresql_type_pattern) {
                        let partial_ord_comma_token_stream = quote::quote! {PartialOrd,};
                        match &postgresql_type {
                            PostgresqlType::StdPrimitiveI16AsInt2 => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveI32AsInt4 => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveI64AsInt8 => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveF32AsFloat4 => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveF64AsFloat8 => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesBigDecimalAsNumeric => partial_ord_comma_token_stream,
                            PostgresqlType::StdPrimitiveBoolAsBool => partial_ord_comma_token_stream,
                            PostgresqlType::StdStringStringAsText => partial_ord_comma_token_stream,
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesTimeTimeAsTime => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesTimeDateAsDate => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => proc_macro2::TokenStream::new(),
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let maybe_derive_serde_serialize_token_stream = match &serde_serialize {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Serialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_serde_deserialize_token_stream = match &serde_serialize {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Deserialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #maybe_derive_partial_ord_token_stream
                            #maybe_derive_serde_serialize_token_stream
                            #maybe_derive_serde_deserialize_token_stream
                        )]
                        pub struct #ident_origin_upper_camel_case(#field_type_handle);
                    }
                };
                let impl_new_for_ident_origin_token_stream = {
                    let content_token_stream = {
                        let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {match value {
                                Some(value) => Some(#type_token_stream::new(value)),
                                None => None
                            }}
                        };
                        let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value.into_iter().map(|element|#type_token_stream::new(element)).collect()},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&type_token_stream),
                        };
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_array_dimensions_initialization_token_stream(&{
                                let (current_postgresql_type_pattern, current_not_null_or_nullable): (&PostgresqlTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&PostgresqlTypePattern::Standart, dimension1_not_null_or_nullable),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                                };
                                generate_current_ident_origin_non_wrapping(current_postgresql_type_pattern, current_not_null_or_nullable)
                            }),
                            // PostgresqlTypePattern::ArrayDimension2{
                            //     dimension1_not_null_or_nullable,
                            //     dimension2_not_null_or_nullable,
                            // } => generate_array_dimensions_initialization_token_stream(&{
                            //     let (
                            //         current_postgresql_type_pattern,
                            //         current_not_null_or_nullable,
                            //     ): (
                            //         &PostgresqlTypePattern,
                            //         &postgresql_crud_macros_common::NotNullOrNullable,
                            //     ) = match &not_null_or_nullable {
                            //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            //             &PostgresqlTypePattern::ArrayDimension1 {
                            //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                            //             },
                            //             &dimension1_not_null_or_nullable,
                            //         ),
                            //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            //             &postgresql_type_pattern,
                            //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            //         )
                            //     };
                            //     generate_current_ident_origin_non_wrapping(
                            //         &current_postgresql_type_pattern,
                            //         &current_not_null_or_nullable
                            //     )
                            // }),
                            // PostgresqlTypePattern::ArrayDimension3{
                            //     dimension1_not_null_or_nullable,
                            //     dimension2_not_null_or_nullable,
                            //     dimension3_not_null_or_nullable,
                            // } => generate_array_dimensions_initialization_token_stream(&{
                            //     let (
                            //         current_postgresql_type_pattern,
                            //         current_not_null_or_nullable,
                            //     ): (
                            //         &PostgresqlTypePattern,
                            //         &postgresql_crud_macros_common::NotNullOrNullable,
                            //     ) = match &not_null_or_nullable {
                            //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            //             &PostgresqlTypePattern::ArrayDimension2 {
                            //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                            //                 dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                            //             },
                            //             &dimension1_not_null_or_nullable,
                            //         ),
                            //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            //             &postgresql_type_pattern,
                            //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            //         )
                            //     };
                            //     generate_current_ident_origin_non_wrapping(
                            //         &current_postgresql_type_pattern,
                            //         &current_not_null_or_nullable
                            //     )
                            // }),
                            // PostgresqlTypePattern::ArrayDimension4{
                            //     dimension1_not_null_or_nullable,
                            //     dimension2_not_null_or_nullable,
                            //     dimension3_not_null_or_nullable,
                            //     dimension4_not_null_or_nullable,
                            // } => generate_array_dimensions_initialization_token_stream(&{
                            //     let (
                            //         current_postgresql_type_pattern,
                            //         current_not_null_or_nullable,
                            //     ): (
                            //         &PostgresqlTypePattern,
                            //         &postgresql_crud_macros_common::NotNullOrNullable,
                            //     ) = match &not_null_or_nullable {
                            //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            //             &PostgresqlTypePattern::ArrayDimension3 {
                            //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                            //                 dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                            //                 dimension3_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                            //             },
                            //             &dimension1_not_null_or_nullable,
                            //         ),
                            //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            //             &postgresql_type_pattern,
                            //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            //         )
                            //     };
                            //     generate_current_ident_origin_non_wrapping(
                            //         &current_postgresql_type_pattern,
                            //         &current_not_null_or_nullable
                            //     )
                            // }),
                        }
                    };
                    quote::quote! {
                        impl #ident_origin_upper_camel_case {
                            pub fn new(value: #impl_new_for_ident_origin_type_token_stream) -> Self {
                                Self(#content_token_stream)
                            }
                        }
                    }
                };
                let maybe_impl_is_string_empty_for_ident_origin_token_stream = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&not_null_or_nullable, &postgresql_type_pattern) {
                    let impl_is_string_empty_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case);
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_type {
                            PostgresqlType::StdPrimitiveI16AsInt2 => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveI32AsInt4 => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveI64AsInt8 => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveF32AsFloat4 => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveF64AsFloat8 => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesBigDecimalAsNumeric => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdPrimitiveBoolAsBool => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdStringStringAsText => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesTimeTimeAsTime => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesTimeDateAsDate => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => proc_macro2::TokenStream::new(),
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_impl_serde_serialize_for_ident_standart_not_null_origin_token_stream = match serde_serialize {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(value) => value,
                };
                let maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_token_stream = match serde_deserialize {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(value) => value,
                };
                let impl_std_fmt_display_for_ident_origin_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(formatter, "{self:?}")});
                let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream =
                    macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {self.to_string()});

                let sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream = quote::quote! {
                    #sqlx_types_time_date_as_date_field_type_token_stream::from_ordinal_date(
                        #core_default_default_default_token_stream,
                        1,
                    ).unwrap()//todo
                };

                let sqlx_types_time_primitive_date_time_new_token_stream = quote::quote! {#sqlx_types_time_primitive_date_time_as_timestamp_field_type_token_stream::new(
                    #sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                    #sqlx_types_time_time_midnight_token_stream,
                )};
                // fn std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream() -> proc_macro2::TokenStream {
                //     quote::quote! {std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)}
                // }
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream =
                    postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_origin_upper_camel_case, &{
                        let content_token_stream = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let generate_sqlx_postgres_types_pg_range_token_stream = |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| generate_qlx_postgres_types_pg_range_start_end_token_stream(
                                    &quote::quote! {std::ops::Bound::Included(#start_token_stream)},
                                    &quote::quote! {std::ops::Bound::Excluded(#end_token_stream)}
                                );
                                let sqlx_postgres_types_pg_range_core_default_default_default_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&core_default_default_default_token_stream, &core_default_default_default_token_stream);
                                let initialization_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                                    PostgresqlType::StdPrimitiveI16AsInt2
                                    | PostgresqlType::StdPrimitiveI32AsInt4
                                    | PostgresqlType::StdPrimitiveI64AsInt8
                                    | PostgresqlType::StdPrimitiveF32AsFloat4
                                    | PostgresqlType::StdPrimitiveF64AsFloat8
                                    | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                                    | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                                    | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => &quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#core_default_default_default_token_stream)},
                                    PostgresqlType::SqlxTypesBigDecimalAsNumeric
                                    | PostgresqlType::StdPrimitiveBoolAsBool
                                    | PostgresqlType::StdStringStringAsText => &core_default_default_default_token_stream,
                                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => &quote::quote! {vec![#core_default_default_default_token_stream]},
                                    PostgresqlType::SqlxTypesTimeTimeAsTime => &quote::quote! {#sqlx_types_time_time_midnight_token_stream},
                                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => &{
                                        let double_dots_space_core_default_default_default_token_stream = generate_double_dot_space_tokens_token_stream(&core_default_default_default_token_stream);
                                        generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                                            &double_dots_space_core_default_default_default_token_stream,
                                            &double_dots_space_core_default_default_default_token_stream,
                                            &double_dots_space_core_default_default_default_token_stream,
                                        )
                                    },
                                    PostgresqlType::SqlxTypesTimeDateAsDate => &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => &sqlx_types_time_primitive_date_time_new_token_stream,
                                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                                    | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz
                                    | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql
                                    | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => &quote::quote! {
                                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_token_stream).unwrap())
                                    },
                                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => &generate_sqlx_postgres_types_pg_range_token_stream(
                                        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                                        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                                    ),
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => &generate_sqlx_postgres_types_pg_range_token_stream(&sqlx_types_time_primitive_date_time_new_token_stream, &sqlx_types_time_primitive_date_time_new_token_stream),
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                };
                                quote::quote! {#initialization_token_stream}
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                        },
                        PostgresqlTypePattern::ArrayDimension1 {..}
                        // |
                        // PostgresqlTypePattern::ArrayDimension2 {..} |
                        // PostgresqlTypePattern::ArrayDimension3 {..} |
                        // PostgresqlTypePattern::ArrayDimension4 {..} 
                        => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote!{vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                        }
                    };
                        quote::quote! {Self(#content_token_stream)}
                    });
                let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &field_type_handle);
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = {
                    let self_snake_case = naming::SelfSnakeCase;
                    quote::quote! {
                        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_origin_upper_camel_case {
                            fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
                            }
                        }
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &field_type_handle, &quote::quote! {Ok(Self(#value_snake_case))});
                let impl_sqlx_postgres_pg_has_array_type_for_ident_origin_token_stream = {
                    quote::quote! {
                        impl sqlx::postgres::PgHasArrayType for #ident_origin_upper_camel_case {
                            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                                <#field_type_standart_not_null as sqlx::postgres::PgHasArrayType>::array_type_info()
                            }
                        }
                    }
                };
                enum CanBePrimaryKey {
                    True,
                    False,
                }
                let can_be_primary_key = match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsInt2 => CanBePrimaryKey::False,
                    PostgresqlType::StdPrimitiveI32AsInt4 => CanBePrimaryKey::False,
                    PostgresqlType::StdPrimitiveI64AsInt8 => CanBePrimaryKey::False,
                    PostgresqlType::StdPrimitiveF32AsFloat4 => CanBePrimaryKey::False,
                    PostgresqlType::StdPrimitiveF64AsFloat8 => CanBePrimaryKey::False,
                    PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => CanBePrimaryKey::True,
                    PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => CanBePrimaryKey::True,
                    PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => CanBePrimaryKey::True,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesBigDecimalAsNumeric => CanBePrimaryKey::False,
                    PostgresqlType::StdPrimitiveBoolAsBool => CanBePrimaryKey::False,
                    PostgresqlType::StdStringStringAsText => CanBePrimaryKey::False,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesTimeTimeAsTime => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesTimeDateAsDate => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBePrimaryKey::True,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => CanBePrimaryKey::False,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePrimaryKey::False,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => CanBePrimaryKey::False,
                };
                let (maybe_impl_postgresql_type_where_filter_for_ident_origin_if_can_be_primary_key_token_stream, maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream) =
                    if let (CanBePrimaryKey::True, postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&can_be_primary_key, &not_null_or_nullable, &postgresql_type_pattern) {
                        (
                            postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                                &quote::quote! {<'a>},
                                &ident_standart_not_null_origin_upper_camel_case,
                                &proc_macro2::TokenStream::new(),
                                &{
                                    let crate_query_part_error_named_checked_add_initialization_token_stream = postgresql_crud_macros_common::crate_query_part_error_named_checked_add_initialization_token_stream();
                                    quote::quote! {
                                        match #increment_snake_case.checked_add(1) {
                                            Some(value) => {
                                                *#increment_snake_case = value;
                                                Ok(format!("({} = ${})", column, #increment_snake_case))
                                            }
                                            None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream),
                                        }
                                    }
                                },
                                &postgresql_crud_macros_common::IsQueryBindMutable::True,
                                &generate_typical_query_bind_token_stream(&naming::SelfSnakeCase),
                                &postgresql_crud_macros_common_import_path_crate,
                            ),
                            quote::quote! {
                                impl crate::PostgresqlTypePrimaryKey for #ident_standart_not_null_upper_camel_case {
                                    type PrimaryKey = #ident_standart_not_null_origin_upper_camel_case;
                                }
                            },
                        )
                    } else {
                        (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new())
                    };
                let impl_create_table_column_query_part_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_create_table_column_query_part_token_stream(
                    &ident_origin_upper_camel_case,
                    postgresql_crud_macros_common::IsPrimaryKeyUsed::True,
                    &match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveI32AsInt4 => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveI64AsInt8 => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesBigDecimalAsNumeric => &proc_macro2_token_stream_new,
                        PostgresqlType::StdPrimitiveBoolAsBool => &proc_macro2_token_stream_new,
                        PostgresqlType::StdStringStringAsText => &proc_macro2_token_stream_new,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimeDateAsDate => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => &proc_macro2_token_stream_new,
                    },
                    &{
                        let postgresql_query_type = match &postgresql_type {
                            PostgresqlType::StdPrimitiveI16AsInt2 => "int2",
                            PostgresqlType::StdPrimitiveI32AsInt4 => "int4",
                            PostgresqlType::StdPrimitiveI64AsInt8 => "int8",
                            PostgresqlType::StdPrimitiveF32AsFloat4 => "float4",
                            PostgresqlType::StdPrimitiveF64AsFloat8 => "float8",
                            PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => "smallserial",
                            PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => "serial",
                            PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => "bigserial",
                            PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => "money",
                            PostgresqlType::SqlxTypesBigDecimalAsNumeric => "numeric",
                            PostgresqlType::StdPrimitiveBoolAsBool => "bool",
                            PostgresqlType::StdStringStringAsText => "text",
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => "bytea",
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => "time",
                            PostgresqlType::SqlxTypesTimeTimeAsTime => "time",
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => "interval",
                            PostgresqlType::SqlxTypesTimeDateAsDate => "date",
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => "date",
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => "timestamp",
                            PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => "timestamp",
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => "timestamptz",
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => "timestamptz",
                            PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => "uuid",
                            PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => "uuid",
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => "inet",
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => "macaddr",
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => "int4range",
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => "int8range",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => "numrange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => "daterange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => "daterange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => "tsrange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => "tsrange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => "tstzrange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => "tstzrange",
                        };
                        let maybe_array_part = match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => "".to_string(),
                            PostgresqlTypePattern::ArrayDimension1 {..}
                            // |
                            // PostgresqlTypePattern::ArrayDimension2 {..} |
                            // PostgresqlTypePattern::ArrayDimension3 {..} |
                            // PostgresqlTypePattern::ArrayDimension4 {..} 
                            => std::iter::repeat_n("[]", array_dimensions_number).collect::<String>()
                        };
                        let maybe_constraint_part = match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => "".to_string(),
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => ",check (array_position({column},null) is null)".to_string(),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => "".to_string(),
                            },
                            // if nested array with be supported in sqlx in the future than add additional checks using created in db functions coz postgreql does not support not null elements of array with keyword "not null"
                            // PostgresqlTypePattern::ArrayDimension2 {..} => "".to_string(),
                            // PostgresqlTypePattern::ArrayDimension3 {..} => "".to_string(),
                            // PostgresqlTypePattern::ArrayDimension4 {..} => "".to_string(),
                        };
                        let crate_maybe_primary_key_is_primary_key_token_stream = quote::quote! {crate::maybe_primary_key(is_primary_key)};
                        let column_postgresql_query_type = format!("{{column}} {postgresql_query_type}{maybe_array_part}{maybe_constraint_part}");
                        let column_postgresql_query_type_not_null = format!("{{column}} {postgresql_query_type}{maybe_array_part} not null{maybe_constraint_part}");
                        let space_additional_parameter = " {}";
                        match (&not_null_or_nullable, &can_be_primary_key) {
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, CanBePrimaryKey::False) => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type_not_null.to_string());
                                quote::quote! {
                                    format!(#format_handle_token_stream)
                                }
                            }
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, CanBePrimaryKey::True) => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{column_postgresql_query_type_not_null}{space_additional_parameter}"));
                                quote::quote! {
                                    format!(#format_handle_token_stream, #crate_maybe_primary_key_is_primary_key_token_stream)
                                }
                            }
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, CanBePrimaryKey::False) => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type);
                                quote::quote! {
                                    format!(#format_handle_token_stream)
                                }
                            }
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, CanBePrimaryKey::True) => {
                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{column_postgresql_query_type}{space_additional_parameter}"));
                                quote::quote! {
                                    format!(#format_handle_token_stream, #crate_maybe_primary_key_is_primary_key_token_stream)
                                }
                            }
                        }
                    },
                );
                quote::quote! {
                    #ident_origin_token_stream
                    #impl_new_for_ident_origin_token_stream
                    #maybe_impl_is_string_empty_for_ident_origin_token_stream
                    #maybe_impl_serde_serialize_for_ident_standart_not_null_origin_token_stream
                    #maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_token_stream
                    #impl_std_fmt_display_for_ident_origin_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_origin_token_stream
                    #impl_sqlx_postgres_pg_has_array_type_for_ident_origin_token_stream
                    #maybe_impl_postgresql_type_where_filter_for_ident_origin_if_can_be_primary_key_token_stream
                    #maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream
                    #impl_create_table_column_query_part_for_ident_origin_token_stream
                }
            };
            enum ImplDefault {
                True,
                False,
            }
            let generate_pub_struct_tokens_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens, impl_default: ImplDefault| {
                let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
                let maybe_impl_default_token_stream: &dyn quote::ToTokens = match &impl_default {
                    ImplDefault::True => &quote::quote! {Default,},
                    ImplDefault::False => &proc_macro2_token_stream_new,
                };
                quote::quote! {
                    #[derive(
                        Debug,
                        #maybe_impl_default_token_stream
                        Clone,
                        PartialEq,
                        serde::Serialize,
                        serde::Deserialize,
                    )]
                    pub struct #ident_token_stream #content_token_stream
                }
            };
            let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
            let ident_table_type_declaration_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_table_type_declaration_upper_camel_case, &ident_origin_upper_camel_case);
            let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
            let ident_create_token_stream = {
                let ident_create_token_stream = {
                    let ident_create_token_stream = generate_pub_struct_tokens_token_stream(&ident_create_upper_camel_case, &quote::quote! {(());}, ImplDefault::False);
                    let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream =
                        postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_create_upper_camel_case, &quote::quote! {Self(#core_default_default_default_token_stream)});
                    quote::quote! {
                        #ident_create_token_stream
                        #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream
                    }
                };
                let alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_create_upper_camel_case, &ident_origin_upper_camel_case);
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsInt2 => alias_token_stream,
                    PostgresqlType::StdPrimitiveI32AsInt4 => alias_token_stream,
                    PostgresqlType::StdPrimitiveI64AsInt8 => alias_token_stream,
                    PostgresqlType::StdPrimitiveF32AsFloat4 => alias_token_stream,
                    PostgresqlType::StdPrimitiveF64AsFloat8 => alias_token_stream,
                    PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => ident_create_token_stream,
                    PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => ident_create_token_stream,
                    PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => ident_create_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => alias_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsNumeric => alias_token_stream,
                    PostgresqlType::StdPrimitiveBoolAsBool => alias_token_stream,
                    PostgresqlType::StdStringStringAsText => alias_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => alias_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsTime => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => alias_token_stream,
                    PostgresqlType::SqlxTypesTimeDateAsDate => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => alias_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => alias_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => ident_create_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => alias_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => alias_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => alias_token_stream,
                }
            };
            let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
            let ident_select_token_stream = {
                let pub_struct_ident_select_token_stream = generate_pub_struct_tokens_token_stream(
                    &ident_select_upper_camel_case,
                    &match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => quote::quote! {;},
                    PostgresqlTypePattern::ArrayDimension1 {..}
                    // |
                    // PostgresqlTypePattern::ArrayDimension2 {..} |
                    // PostgresqlTypePattern::ArrayDimension3 {..} |
                    // PostgresqlTypePattern::ArrayDimension4 {..} 
                    => {
                        let mut arguments_token_stream = vec![];
                        for element in 1..=array_dimensions_number {
                            let dimension_number_pagination_token_stream = format!("dimension{element}_pagination")
                            .parse::<proc_macro2::TokenStream>().unwrap();
                            arguments_token_stream.push(quote::quote! {
                                #dimension_number_pagination_token_stream: crate::Pagination
                            });
                        }
                        quote::quote! {{
                            #(#arguments_token_stream),*
                        }}
                    }
                },
                    ImplDefault::True,
                );
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_select_upper_camel_case,
                    &match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => quote::quote! {#core_default_default_default_token_stream},
                    PostgresqlTypePattern::ArrayDimension1 {..}
                    // |
                    // PostgresqlTypePattern::ArrayDimension2 {..} |
                    // PostgresqlTypePattern::ArrayDimension3 {..} |
                    // PostgresqlTypePattern::ArrayDimension4 {..}
                    => {
                        let mut arguments_token_stream = vec![];
                        for element in 1..=array_dimensions_number {
                            let dimension_number_pagination_token_stream = format!("dimension{element}_pagination")
                            .parse::<proc_macro2::TokenStream>().unwrap();
                            arguments_token_stream.push(quote::quote! {
                                #dimension_number_pagination_token_stream: #crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            });
                        }
                        quote::quote! {Self {
                            #(#arguments_token_stream),*
                        }}
                    }
                },
                );
                quote::quote! {
                    #pub_struct_ident_select_token_stream
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
                }
            };
            let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
            let ident_read_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_read_upper_camel_case, &ident_origin_upper_camel_case);
            let ident_read_token_stream_f = {
                let ident_read_token_stream = {
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            serde::Serialize,
                            serde::Deserialize
                        )]
                        pub struct #ident_read_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_read_token_stream = {
                    let pub_fn_new_token_stream = quote::quote!{
                        pub fn new(#value_snake_case: #impl_new_for_ident_origin_type_token_stream) -> Self {
                            Self(#ident_origin_upper_camel_case::new(#value_snake_case))
                        }
                    };
                    let pub_fn_into_inner_token_stream = {
                        let content_token_stream = {
                            quote::quote!{

                            }
                        };
                        quote::quote!{
                            // pub fn into_inner(self) ->  {
                            //     #content_token_stream
                            // }
                        }
                    };
                    quote::quote!{
                        impl #ident_read_upper_camel_case {
                            #pub_fn_new_token_stream
                            #pub_fn_into_inner_token_stream
                        }
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                    &ident_read_upper_camel_case,
                    &ident_origin_upper_camel_case,
                    &quote::quote! {Ok(Self(#value_snake_case))}
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &ident_read_upper_camel_case,
                    &ident_origin_upper_camel_case,
                );
                quote::quote!{
                    #ident_read_token_stream
                    #impl_ident_read_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream
                }
            };
            let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
            let ident_update_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_update_upper_camel_case, &ident_origin_upper_camel_case);
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
            let ident_where_element_token_stream = {
                let generate_ident_where_element_token_stream = |variants: &std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter>| {
                    postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                        variants,
                        |is_relevant_only_for_not_null: std::primitive::bool| {
                            if is_relevant_only_for_not_null { &ident_standart_not_null_origin_upper_camel_case } else { &ident_origin_upper_camel_case }
                        },
                        &ident,
                        &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::False,
                        &postgresql_crud_macros_common::IsQueryBindMutable::False,
                    )
                };

                let equal = postgresql_crud_macros_common::PostgresqlTypeFilter::Equal;
                let greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan;
                let between = postgresql_crud_macros_common::PostgresqlTypeFilter::Between;
                let in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::In;
                let case_sensitive_regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::CaseSensitiveRegularExpression;
                let case_insensitive_regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::CaseInsensitiveRegularExpression;
                let equal_to_encoded_string_representation = postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation;
                let current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate;
                let greater_than_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate;
                let current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime;
                let greater_than_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime;
                let value_is_contained_within_range = postgresql_crud_macros_common::PostgresqlTypeFilter::ValueIsContainedWithinRange;
                let contains_another_range = postgresql_crud_macros_common::PostgresqlTypeFilter::ContainsAnotherRange;
                let strictly_to_left_of_range = postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange;
                let strictly_to_right_of_range = postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange;
                let included_lower_bound = postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound;
                let excluded_upper_bound = postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound;
                let greater_than_lower_bound = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanLowerBound;
                let overlap_with_range = postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange;
                let adjacent_with_range = postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange;
                let range_length = postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength;
                let current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp;
                let greater_than_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp;
                let before = postgresql_crud_macros_common::PostgresqlTypeFilter::Before;
                // let bit_vec_position_equal = postgresql_crud_macros_common::PostgresqlTypeFilter::BitVecPositionEqual;
                let array_length_dimension_one = postgresql_crud_macros_common::PostgresqlTypeFilter::ArrayLengthDimensionOne;
                let array_length_more_than_dimension_one = postgresql_crud_macros_common::PostgresqlTypeFilter::ArrayLengthMoreThanDimensionOne;

                let where_element_number_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &in_handle]);
                let where_element_sqlx_postgres_types_pg_money_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &in_handle]);
                let where_element_sqlx_types_big_decimal_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between]);
                let where_element_bool_token_stream = generate_ident_where_element_token_stream(&vec![&equal]);
                let where_element_std_string_string_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
                let where_element_std_vec_vec_std_primitive_u8_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &equal_to_encoded_string_representation]);
                let where_element_sqlx_types_chrono_naive_time_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_time, &greater_than_current_time]);
                let where_element_sqlx_types_time_time_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_time, &greater_than_current_time]);
                let where_element_sqlx_postgres_types_pg_interval_token_stream = generate_ident_where_element_token_stream(&vec![&equal]);
                let where_element_sqlx_types_time_date_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_date, &greater_than_current_date]);
                let where_element_sqlx_types_chrono_naive_date_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_date, &greater_than_current_date]);
                let where_element_sqlx_types_chrono_naive_date_time_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_timestamp, &greater_than_current_timestamp]);
                let where_element_sqlx_types_time_primitive_date_time_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_timestamp, &greater_than_current_timestamp]);
                let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &before, &between]);
                let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &before, &between]);
                let where_element_sqlx_types_uuid_uuid_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
                let where_element_sqlx_types_ipnetwork_ip_network_token_stream = generate_ident_where_element_token_stream(&vec![&equal]);
                let where_element_sqlx_types_mac_address_mac_address_token_stream = generate_ident_where_element_token_stream(&vec![&equal, &greater_than, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
                let (
                    where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                    where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                ) = {
                    let generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream = |postgresql_type_range: PostgresqlTypeRange| {
                        enum ShouldImplPostgresqlTypeRangeLength {
                            True,
                            False,
                        }
                        let should_impl_postgresql_type_range_length = {
                            let should_impl_postgresql_type_range_length_true = ShouldImplPostgresqlTypeRangeLength::True;
                            let should_impl_postgresql_type_range_length_false = ShouldImplPostgresqlTypeRangeLength::False;
                            match &postgresql_type_range {
                                PostgresqlTypeRange::StdPrimitiveI32AsInt4 => should_impl_postgresql_type_range_length_true,
                                PostgresqlTypeRange::StdPrimitiveI64AsInt8 => should_impl_postgresql_type_range_length_true,
                                PostgresqlTypeRange::SqlxTypesBigDecimalAsNumeric => should_impl_postgresql_type_range_length_false,
                                PostgresqlTypeRange::SqlxTypesTimeDateAsDate => should_impl_postgresql_type_range_length_false,
                                PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate => should_impl_postgresql_type_range_length_false,
                                PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => should_impl_postgresql_type_range_length_false,
                                PostgresqlTypeRange::SqlxTypesTimePrimitiveDateTimeAsTimestamp => should_impl_postgresql_type_range_length_false,
                                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => should_impl_postgresql_type_range_length_false,
                                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => should_impl_postgresql_type_range_length_false,
                            }
                        };
                        generate_ident_where_element_token_stream(&{
                            let mut value: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = vec![
                                &equal,
                                &value_is_contained_within_range,
                                &contains_another_range,
                                &strictly_to_left_of_range,
                                &strictly_to_right_of_range,
                                &included_lower_bound,
                                &excluded_upper_bound,
                                &greater_than_lower_bound,
                                &overlap_with_range,
                                &adjacent_with_range,
                            ];
                            if let ShouldImplPostgresqlTypeRangeLength::True = &should_impl_postgresql_type_range_length {
                                value.push(&range_length);
                            }
                            value
                        })
                    };
                    (
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI32AsInt4),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI64AsInt8),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesBigDecimalAsNumeric),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesTimeDateAsDate),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesTimePrimitiveDateTimeAsTimestamp),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                        generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz),
                    )
                };
                match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveI32AsInt4 => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveI64AsInt8 => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => where_element_number_token_stream,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => where_element_number_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => where_element_sqlx_postgres_types_pg_money_token_stream,
                        PostgresqlType::SqlxTypesBigDecimalAsNumeric => where_element_sqlx_types_big_decimal_token_stream,
                        PostgresqlType::StdPrimitiveBoolAsBool => where_element_bool_token_stream,
                        PostgresqlType::StdStringStringAsText => where_element_std_string_string_token_stream,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => where_element_std_vec_vec_std_primitive_u8_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => where_element_sqlx_types_chrono_naive_time_token_stream,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => where_element_sqlx_types_time_time_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => where_element_sqlx_postgres_types_pg_interval_token_stream,
                        PostgresqlType::SqlxTypesTimeDateAsDate => where_element_sqlx_types_time_date_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => where_element_sqlx_types_chrono_naive_date_token_stream,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => where_element_sqlx_types_chrono_naive_date_time_token_stream,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => where_element_sqlx_types_time_primitive_date_time_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => where_element_sqlx_types_uuid_uuid_token_stream,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => where_element_sqlx_types_uuid_uuid_token_stream,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => where_element_sqlx_types_mac_address_mac_address_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    },
                    PostgresqlTypePattern::ArrayDimension1 { .. } => generate_ident_where_element_token_stream(&vec![&equal, &array_length_dimension_one, &array_length_more_than_dimension_one]),
                    // PostgresqlTypePattern::ArrayDimension2 {..} => generate_ident_where_element_token_stream(&vec![&equal]),
                    // PostgresqlTypePattern::ArrayDimension3 {..} => generate_ident_where_element_token_stream(&vec![&equal]),
                    // PostgresqlTypePattern::ArrayDimension4 {..} => generate_ident_where_element_token_stream(&vec![&equal]),
                }
            };
            let impl_postgresql_type_for_ident_token_stream = {
                let generate_ok_std_string_string_from_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {Ok(#std_string_string_token_stream::from(#content_token_stream))}
                };
                let ok_std_string_string_from_default_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"DEFAULT"});
                let ok_std_string_string_from_uuid_generate_v4_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"uuid_generate_v4()"});
                let typical_query_part_token_stream = {
                    let acc_snake_case = naming::AccSnakeCase;
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                    let crate_query_part_error_named_token_stream = postgresql_crud_macros_common::crate_query_part_error_named_token_stream();
                    quote::quote! {
                        let mut #acc_snake_case = std::string::String::default();
                        match #increment_snake_case.checked_add(1) {
                            Some(#value_snake_case) => {
                                *#increment_snake_case = #value_snake_case;
                                #acc_snake_case.push_str(&format!(#format_handle_token_stream));
                            }
                            None => {
                                return Err(#crate_query_part_error_named_token_stream::#checked_add_upper_camel_case {
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                        Ok(#acc_snake_case)
                    }
                };
                type Handle<'a> = (&'a dyn quote::ToTokens, &'a dyn quote::ToTokens);
                let (query_part_create_token_stream, bind_value_to_query_create_token_stream): Handle = {
                    let typical: Handle = { (&typical_query_part_token_stream, &typical_query_bind_token_stream) };
                    let default_initialized_by_postgresql: Handle = (&ok_std_string_string_from_default_token_stream, &query_snake_case);
                    let uuid_generate_v4_initialized_by_postgresql: Handle = (&ok_std_string_string_from_uuid_generate_v4_token_stream, &query_snake_case);
                    match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => typical,
                        PostgresqlType::StdPrimitiveI32AsInt4 => typical,
                        PostgresqlType::StdPrimitiveI64AsInt8 => typical,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => typical,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => typical,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => default_initialized_by_postgresql,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => default_initialized_by_postgresql,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => default_initialized_by_postgresql,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => typical,
                        PostgresqlType::SqlxTypesBigDecimalAsNumeric => typical,
                        PostgresqlType::StdPrimitiveBoolAsBool => typical,
                        PostgresqlType::StdStringStringAsText => typical,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => typical,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => typical,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => typical,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => typical,
                        PostgresqlType::SqlxTypesTimeDateAsDate => typical,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => typical,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => typical,
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp => typical,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => typical,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz => typical,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => uuid_generate_v4_initialized_by_postgresql,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => typical,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => typical,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange => typical,
                    }
                };
                postgresql_crud_macros_common::generate_impl_postgresql_type_for_ident_token_stream(
                    &postgresql_crud_macros_common_import_path_crate,
                    &ident,
                    &ident_table_type_declaration_upper_camel_case,
                    &ident_create_upper_camel_case,
                    &query_part_create_token_stream,
                    &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                    &bind_value_to_query_create_token_stream,
                    &ident_select_upper_camel_case,
                    &match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => quote::quote! {#column_snake_case.to_string()},
                    PostgresqlTypePattern::ArrayDimension1 {..}
                    // |
                    // PostgresqlTypePattern::ArrayDimension2 {..} |
                    // PostgresqlTypePattern::ArrayDimension3 {..} |
                    // PostgresqlTypePattern::ArrayDimension4 {..}
                    => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&{
                            let acc = std::iter::repeat_n("[{}:{}]", array_dimensions_number).collect::<String>();
                            format!("{{column}}{acc}")
                        });
                        let arguments_token_stream = {
                            let mut acc = vec![];
                            for element in 1..=array_dimensions_number {
                                let dimension_number_pagination_token_stream = format!("dimension{element}_pagination")
                                .parse::<proc_macro2::TokenStream>().unwrap();
                                acc.push(quote::quote! {
                                    #value_snake_case.#dimension_number_pagination_token_stream.start(),
                                    #value_snake_case.#dimension_number_pagination_token_stream.end(),
                                });
                            }
                            acc
                        };
                        quote::quote! {
                            format!(
                                #format_handle_token_stream,
                                #(#arguments_token_stream)*
                            )
                        }
                    }
                },
                    &ident_where_element_upper_camel_case,
                    &ident_read_upper_camel_case,
                    &ident_update_upper_camel_case,
                    &typical_query_part_token_stream,
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &typical_query_bind_token_stream,
                )
            };
            let generated = quote::quote! {
                #ident_token_stream
                #ident_origin_token_stream
                #ident_table_type_declaration_token_stream
                #ident_create_token_stream
                #ident_select_token_stream
                #ident_where_element_token_stream
                #ident_read_token_stream
                #ident_update_token_stream
                #impl_postgresql_type_for_ident_token_stream
            };
            // if let (
            //     PostgresqlType::StdPrimitiveI16AsInt2,
            //     // PostgresqlType::StdPrimitiveI32AsInt4,
            //     // PostgresqlType::StdPrimitiveI64AsInt8,
            //     // PostgresqlType::StdPrimitiveF32AsFloat4,
            //     // PostgresqlType::StdPrimitiveF64AsFloat8,
            //     // PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql,
            //     // PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql,
            //     // PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql,
            //     // PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney,
            //     // PostgresqlType::SqlxTypesBigDecimalAsNumeric,
            //     // PostgresqlType::StdPrimitiveBoolAsBool,
            //     // PostgresqlType::StdStringStringAsText,
            //     // PostgresqlType::StdVecVecStdPrimitiveU8AsBytea,
            //     // PostgresqlType::SqlxTypesChronoNaiveTimeAsTime,
            //     // PostgresqlType::SqlxTypesTimeTimeAsTime,
            //     // PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval,
            //     // PostgresqlType::SqlxTypesTimeDateAsDate,
            //     // PostgresqlType::SqlxTypesChronoNaiveDateAsDate,
            //     // PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp,
            //     // PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsTimestamp,
            //     // PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
            //     // PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTz,
            //     // PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql,
            //     // PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient,
            //     // PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet,
            //     // PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsDateRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsTimestampRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsTimestampTzRange,

            //     postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            //     // postgresql_crud_macros_common::NotNullOrNullable::Nullable,

            //     // PostgresqlTypePattern::Standart,
            //     PostgresqlTypePattern::ArrayDimension1 {
            //         dimension1_not_null_or_nullable,
            //     },
            //     //// PostgresqlTypePattern::ArrayDimension2 {
            //     ////     dimension1_not_null_or_nullable,
            //     ////     dimension2_not_null_or_nullable,
            //     //// },
            //     //// PostgresqlTypePattern::ArrayDimension3 {
            //     ////     dimension1_not_null_or_nullable,
            //     ////     dimension2_not_null_or_nullable,
            //     ////     dimension3_not_null_or_nullable,
            //     //// },
            //     //// PostgresqlTypePattern::ArrayDimension4 {
            //     ////     dimension1_not_null_or_nullable,
            //     ////     dimension2_not_null_or_nullable,
            //     ////     dimension3_not_null_or_nullable,
            //     ////     dimension4_not_null_or_nullable,
            //     //// }
            // ) = (
            //     &postgresql_type,
            //     &not_null_or_nullable,
            //     &postgresql_type_pattern
            // ) {
            //     use postgresql_crud_macros_common::NotNullOrNullable;
            //     let d1 = match &dimension1_not_null_or_nullable {
            //         NotNullOrNullable::NotNull => true,
            //         NotNullOrNullable::Nullable => false,
            //     };
            //     // let d2 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => true,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     // };
            //     // let d3 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     // };
            //     // let d4 = match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     // };
            //     if d1 {
            //         macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            //             "PostgresqlTypeTokens",
            //             &generated,
            //         );
            //     }
            // }
            (
                {
                    let field_ident = format!("column_{}", uuid::Uuid::new_v4()).replace("-", "_").parse::<proc_macro2::TokenStream>().unwrap();
                    quote::quote! {
                        pub #field_ident: postgresql_crud::postgresql_type:: #ident,
                    }
                    .to_string()
                },
                generated.to_string(),
            )
        })
        .collect::<(std::vec::Vec<String>, std::vec::Vec<String>)>();
    //this need only for better development experience
    if false {
        let postgresql_crud_table_rust_struct_fields_token_stream = postgresql_crud_table_rust_struct_fields_token_stream
            .into_iter()
            .map(|element| element.parse::<proc_macro2::TokenStream>().unwrap())
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            "PostgresqlTypeTokensExampleStruct",
            &quote::quote! {
                struct Example {
                    #(#postgresql_crud_table_rust_struct_fields_token_stream)*
                }
            },
        );
    }
    let generated = {
        let postgresql_type_array = postgresql_type_array.into_iter().map(|element| element.parse::<proc_macro2::TokenStream>().unwrap()).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#postgresql_type_array)*}
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlTypeTokens",
    //     &generated,
    // );
    generated.into()
}
