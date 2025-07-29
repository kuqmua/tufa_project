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
        StdPrimitiveBool,
        StdStringString,
        StdVecVecStdPrimitiveU8,
        SqlxTypesChronoNaiveTime,
        SqlxTypesTimeTime,
        SqlxPostgresTypesPgInterval,
        SqlxTypesChronoNaiveDate,
        SqlxTypesChronoNaiveDateTime,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        SqlxTypesUuidUuid,
        SqlxTypesIpnetworkIpNetwork,
        SqlxTypesMacAddressMacAddress,
        SqlxPostgresTypesPgRangeStdPrimitiveI32,
        SqlxPostgresTypesPgRangeStdPrimitiveI64,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
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
                PostgresqlType::StdPrimitiveBoolAsBool => Self::StdPrimitiveBool,
                PostgresqlType::StdStringStringAsText => Self::StdStringString,
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Self::StdVecVecStdPrimitiveU8,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::SqlxPostgresTypesPgInterval,
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Self::SqlxTypesUuidUuid,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::SqlxTypesUuidUuid,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
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
                PostgresqlType::StdPrimitiveBoolAsBool => Self::Bool,
                PostgresqlType::StdStringStringAsText => Self::Text,
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Self::Bytea,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::Time,
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::Time,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::Interval,
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Self::UuidV4InitializedByPostgresql,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::UuidInitializedByClient,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Inet,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::MacAddr,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::Int4Range,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::Int8Range,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::DateRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::TimestampRange,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::TimestampTzRange,
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
        // SqlxTypesBigDecimalAsNumeric, remove coz dont know how to deserialize with scale std::primitive::i64
        StdPrimitiveBoolAsBool,
        StdStringStringAsText,
        StdVecVecStdPrimitiveU8AsBytea,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxPostgresTypesPgIntervalAsInterval,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql,
        SqlxTypesUuidUuidAsUuidInitializedByClient,
        SqlxTypesIpnetworkIpNetworkAsInet,
        SqlxTypesMacAddressMacAddressAsMacAddr,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    fn wrap_into_sqlx_postgres_types_pg_range_stringified(value: &dyn std::fmt::Display) -> std::string::String {
        format!("sqlx::postgres::types::PgRange<{value}>")
    }
    enum CanBeNullable {
        True,
        False,
    }
    enum CanBeAnArrayElement {
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
                Self::StdPrimitiveBoolAsBool => CanBeNullable::True,
                Self::StdStringStringAsText => CanBeNullable::True,
                Self::StdVecVecStdPrimitiveU8AsBytea => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveTimeAsTime => CanBeNullable::True,
                Self::SqlxTypesTimeTimeAsTime => CanBeNullable::True,
                Self::SqlxPostgresTypesPgIntervalAsInterval => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateAsDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => CanBeNullable::True,
                Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBeNullable::False,
                Self::SqlxTypesUuidUuidAsUuidInitializedByClient => CanBeNullable::True,
                Self::SqlxTypesIpnetworkIpNetworkAsInet => CanBeNullable::True,
                Self::SqlxTypesMacAddressMacAddressAsMacAddr => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeNullable::True,
            }
        }
        fn can_be_an_array_element(&self) -> CanBeAnArrayElement {
            match &self {
                Self::StdPrimitiveI16AsInt2 => CanBeAnArrayElement::True,
                Self::StdPrimitiveI32AsInt4 => CanBeAnArrayElement::True,
                Self::StdPrimitiveI64AsInt8 => CanBeAnArrayElement::True,
                Self::StdPrimitiveF32AsFloat4 => CanBeAnArrayElement::True,
                Self::StdPrimitiveF64AsFloat8 => CanBeAnArrayElement::True,
                Self::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => CanBeAnArrayElement::False,
                Self::StdPrimitiveI32AsSerialInitializedByPostgresql => CanBeAnArrayElement::False,
                Self::StdPrimitiveI64AsBigSerialInitializedByPostgresql => CanBeAnArrayElement::False,
                Self::SqlxPostgresTypesPgMoneyAsMoney => CanBeAnArrayElement::True,
                Self::StdPrimitiveBoolAsBool => CanBeAnArrayElement::True,
                Self::StdStringStringAsText => CanBeAnArrayElement::True,
                Self::StdVecVecStdPrimitiveU8AsBytea => CanBeAnArrayElement::True,
                Self::SqlxTypesChronoNaiveTimeAsTime => CanBeAnArrayElement::True,
                Self::SqlxTypesTimeTimeAsTime => CanBeAnArrayElement::True,
                Self::SqlxPostgresTypesPgIntervalAsInterval => CanBeAnArrayElement::True,
                Self::SqlxTypesChronoNaiveDateAsDate => CanBeAnArrayElement::True,
                Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => CanBeAnArrayElement::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => CanBeAnArrayElement::True,
                Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBeAnArrayElement::False,
                Self::SqlxTypesUuidUuidAsUuidInitializedByClient => CanBeAnArrayElement::True,
                Self::SqlxTypesIpnetworkIpNetworkAsInet => CanBeAnArrayElement::True,
                Self::SqlxTypesMacAddressMacAddressAsMacAddr => CanBeAnArrayElement::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => CanBeAnArrayElement::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => CanBeAnArrayElement::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => CanBeAnArrayElement::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => CanBeAnArrayElement::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeAnArrayElement::True,
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
                let std_primitive_bool_stringified = "std::primitive::bool".to_string();
                let std_string_string_stringified = "std::string::String".to_string();
                let std_vec_vec_std_primitive_u8_stringified = "std::vec::Vec<std::primitive::u8>".to_string();
                let sqlx_types_chrono_naive_date_stringified = "sqlx::types::chrono::NaiveDate".to_string();
                let sqlx_types_chrono_naive_time_stringified = "sqlx::types::chrono::NaiveTime".to_string();
                let sqlx_types_time_time_stringified = "sqlx::types::time::Time".to_string();
                let sqlx_postgres_types_pg_interval_stringified = "sqlx::postgres::types::PgInterval".to_string();
                let sqlx_types_chrono_naive_date_time_stringified = "sqlx::types::chrono::NaiveDateTime".to_string();
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified = "crate::SqlxTypesChronoDateTimeSqlxTypesChronoUtc".to_string();
                let sqlx_types_uuid_uuid_stringified = "sqlx::types::uuid::Uuid".to_string();
                let sqlx_types_ipnetwork_ip_network_stringified = "sqlx::types::ipnetwork::IpNetwork".to_string();
                let sqlx_types_mac_address_mac_address_stringified = "sqlx::types::mac_address::MacAddress".to_string();
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
                    Self::StdPrimitiveBoolAsBool => std_primitive_bool_stringified,
                    Self::StdStringStringAsText => std_string_string_stringified,
                    Self::StdVecVecStdPrimitiveU8AsBytea => std_vec_vec_std_primitive_u8_stringified,
                    Self::SqlxTypesChronoNaiveTimeAsTime => sqlx_types_chrono_naive_time_stringified,
                    Self::SqlxTypesTimeTimeAsTime => sqlx_types_time_time_stringified,
                    Self::SqlxPostgresTypesPgIntervalAsInterval => sqlx_postgres_types_pg_interval_stringified,
                    Self::SqlxTypesChronoNaiveDateAsDate => sqlx_types_chrono_naive_date_stringified,
                    Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => sqlx_types_chrono_naive_date_time_stringified,
                    Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                    Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => sqlx_types_uuid_uuid_stringified,
                    Self::SqlxTypesUuidUuidAsUuidInitializedByClient => sqlx_types_uuid_uuid_stringified,
                    Self::SqlxTypesIpnetworkIpNetworkAsInet => sqlx_types_ipnetwork_ip_network_stringified,
                    Self::SqlxTypesMacAddressMacAddressAsMacAddr => sqlx_types_mac_address_mac_address_stringified,
                    Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i32_stringified),
                    Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i64_stringified),
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_stringified),
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_time_stringified),
                    Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => wrap_into_sqlx_postgres_types_pg_range_stringified(
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified
                    ),
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
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
            }
        }
    }
    enum PostgresqlTypeRange {
        StdPrimitiveI32AsInt4,
        StdPrimitiveI64AsInt8,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
    }
    impl std::convert::TryFrom<&PostgresqlType> for PostgresqlTypeRange {
        type Error = ();
        fn try_from(value: &PostgresqlType) -> Result<Self, Self::Error> {
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
                PostgresqlType::StdPrimitiveBoolAsBool => Err(()),
                PostgresqlType::StdStringStringAsText => Err(()),
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Err(()),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Err(()),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Err(()),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::StdPrimitiveI32AsInt4),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::StdPrimitiveI64AsInt8),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
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
                        Ok(PostgresqlTypeRecord::from((__field0, __field1, __field2)))
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
                        Ok(PostgresqlTypeRecord::from((__field0, __field1, __field2)))
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
            let cant_support_nullable_variants_message = "cant support nullable variants: ";
            let cant_support_array_version_message = "cant support array_version: ";
            match &value.0.can_be_nullable() {
                CanBeNullable::True => match &value.2 {
                    PostgresqlTypePattern::Standart => Self {
                        postgresql_type: value.0,
                        not_null_or_nullable: value.1,
                        postgresql_type_pattern: value.2,
                    },
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: _ } => match &value.0.can_be_an_array_element() {
                        CanBeAnArrayElement::True => Self {
                            postgresql_type: value.0,
                            not_null_or_nullable: value.1,
                            postgresql_type_pattern: value.2,
                        },
                        CanBeAnArrayElement::False => panic!("{cant_support_array_version_message}{value:#?}"),
                    },
                },
                CanBeNullable::False => {
                    if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &value.1 {
                        panic!("{cant_support_nullable_variants_message}{value:#?}");
                    }
                    match &value.2 {
                        PostgresqlTypePattern::Standart => Self {
                            postgresql_type: value.0,
                            not_null_or_nullable: value.1,
                            postgresql_type_pattern: value.2,
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &value.0.can_be_an_array_element() {
                            CanBeAnArrayElement::True => match &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => Self {
                                    postgresql_type: value.0,
                                    not_null_or_nullable: value.1,
                                    postgresql_type_pattern: value.2,
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => panic!("{cant_support_nullable_variants_message}{value:#?}"),
                            },
                            CanBeAnArrayElement::False => panic!("{cant_support_array_version_message}{value:#?}"),
                        },
                    }
                }
            }
        }
    }
    impl PostgresqlTypeRecord {
        fn all() -> std::vec::Vec<Self> {
            PostgresqlType::into_array().into_iter().fold(vec![], |mut acc, postgresql_type| {
                PostgresqlTypePattern::all().into_iter().for_each(|postgresql_type_pattern| match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => match &postgresql_type.can_be_nullable() {
                        CanBeNullable::True => postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable|{
                            acc.push(PostgresqlTypeRecord {
                                postgresql_type: postgresql_type.clone(),
                                not_null_or_nullable,
                                postgresql_type_pattern: postgresql_type_pattern.clone(),
                            });
                        }),
                        CanBeNullable::False => {
                            acc.push(PostgresqlTypeRecord {
                                postgresql_type: postgresql_type.clone(),
                                not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                postgresql_type_pattern,
                            });
                        }
                    },
                    PostgresqlTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable
                    } => match &postgresql_type.can_be_an_array_element() {
                        CanBeAnArrayElement::True => match &postgresql_type.can_be_nullable() {
                            CanBeNullable::True => postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable|{
                                acc.push(PostgresqlTypeRecord {
                                    postgresql_type: postgresql_type.clone(),
                                    not_null_or_nullable: not_null_or_nullable.clone(),
                                    postgresql_type_pattern: postgresql_type_pattern.clone(),
                                });
                            }),
                            CanBeNullable::False => if let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable|{
                                    acc.push(PostgresqlTypeRecord {
                                        postgresql_type: postgresql_type.clone(),
                                        not_null_or_nullable: not_null_or_nullable,
                                        postgresql_type_pattern: PostgresqlTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: dimension1_not_null_or_nullable.clone(),
                                        },
                                    });
                                });
                            }
                        },
                        CanBeAnArrayElement::False => ()
                    }
                });
                acc
            })
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlTypesConfig {
        All,
        Concrete(std::vec::Vec<PostgresqlTypeRecord>)
    }
    #[derive(Debug)]
    enum PostgresqlTypeInitializationTryNew {
        StdStringStringAsText,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
    }
    impl std::convert::TryFrom<&PostgresqlType> for PostgresqlTypeInitializationTryNew {
        type Error = ();
        fn try_from(value: &PostgresqlType) -> Result<Self, Self::Error> {
            match value {
                PostgresqlType::StdPrimitiveI16AsInt2 => Err(()),
                PostgresqlType::StdPrimitiveI32AsInt4 => Err(()),
                PostgresqlType::StdPrimitiveI64AsInt8 => Err(()),
                PostgresqlType::StdPrimitiveF32AsFloat4 => Err(()),
                PostgresqlType::StdPrimitiveF64AsFloat8 => Err(()),
                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => Err(()),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => Err(()),
                PostgresqlType::StdPrimitiveBoolAsBool => Err(()),
                PostgresqlType::StdStringStringAsText => Ok(Self::StdStringStringAsText),
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Err(()),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Err(()),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Err(()),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Err(()),
            }
        }
    }
    impl std::convert::From<&PostgresqlTypeInitializationTryNew> for PostgresqlType {
        fn from(value: &PostgresqlTypeInitializationTryNew) -> PostgresqlType {
            match value {
                PostgresqlTypeInitializationTryNew::StdStringStringAsText => Self::StdStringStringAsText,
                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
            }
        }
    }
    #[derive(Debug)]
    enum PostgresqlTypeImplNewForDeserialize {
        SqlxTypesChronoNaiveDateTimeAsTimestamp
    }
    #[derive(Debug)]
    enum PostgresqlTypeImplTryNewForDeserialize {
        StdStringStringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
    }
    #[derive(Debug)]
    enum PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize {
        NewForDeserialize(PostgresqlTypeImplNewForDeserialize),
        TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize)
    }
    #[derive(Debug)]
    enum PostgresqlTypeDeserialize {
        Derive,
        ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize)
    }
    impl std::convert::From<&PostgresqlType> for PostgresqlTypeDeserialize {
        fn from(value: &PostgresqlType) -> PostgresqlTypeDeserialize {
            match value {
                PostgresqlType::StdPrimitiveI16AsInt2 => Self::Derive,
                PostgresqlType::StdPrimitiveI32AsInt4 => Self::Derive,
                PostgresqlType::StdPrimitiveI64AsInt8 => Self::Derive,
                PostgresqlType::StdPrimitiveF32AsFloat4 => Self::Derive,
                PostgresqlType::StdPrimitiveF64AsFloat8 => Self::Derive,
                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => Self::Derive,
                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => Self::Derive,
                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => Self::Derive,
                PostgresqlType::StdPrimitiveBoolAsBool => Self::Derive,
                PostgresqlType::StdStringStringAsText => Self::Derive,//todo try_new_for_deserialize
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Self::Derive,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(
                    PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(
                        PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime
                    )
                ),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(
                    PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(
                        PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime
                    )
                ),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::Derive,
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::Derive,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::ImplNewForDeserializeOrTryNewForDeserialize(
                    PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(
                        PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp
                    )
                ),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::Derive,
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Self::Derive,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::Derive,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Derive,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::Derive,
            }
        }
    }
    let postgresql_type_record_vec = {
        let generate_postgresql_types_config = serde_json::from_str::<GeneratePostgresqlTypesConfig>(&input_token_stream.to_string()).expect("failed to get Config for generate_postgresql_type");
        let postgresql_type_record_vec = match generate_postgresql_types_config {
            GeneratePostgresqlTypesConfig::All => PostgresqlTypeRecord::all(),
            GeneratePostgresqlTypesConfig::Concrete(value) => value,
        };
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
    // .into_iter()
    // .filter(|element| {
    //     use postgresql_crud_macros_common::NotNullOrNullable;
    //     let postgresql_type_filter = match &element.postgresql_type {
    //         PostgresqlType::StdPrimitiveI16AsInt2 => true,
    //         PostgresqlType::StdPrimitiveI32AsInt4 => true,
    //         PostgresqlType::StdPrimitiveI64AsInt8 => true,
    //         PostgresqlType::StdPrimitiveF32AsFloat4 => true,
    //         PostgresqlType::StdPrimitiveF64AsFloat8 => true,
    //         PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => true,
    //         PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => true,
    //         PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => true,
    //         PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => true,
    //         PostgresqlType::StdPrimitiveBoolAsBool => true,
    //         PostgresqlType::StdStringStringAsText => true,
    //         PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => true,
    //         PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => true,
    //         PostgresqlType::SqlxTypesTimeTimeAsTime => true,
    //         PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => true,
    //         PostgresqlType::SqlxTypesChronoNaiveDateAsDate => true,
    //         PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => true,
    //         PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => true,
    //         PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => true,
    //         PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => true,
    //         PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => true,
    //         PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => true,
    //         PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => true,
    //         PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => true,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => true,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => true,
    //         PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => true,
    //     };
    //     let not_null_or_nullable_filter = match &element.not_null_or_nullable {
    //         NotNullOrNullable::NotNull => true,
    //         NotNullOrNullable::Nullable => true,
    //     };
    //     let postgresql_type_pattern_filter = match &element.postgresql_type_pattern {
    //         PostgresqlTypePattern::Standart => true,
    //         PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
    //             NotNullOrNullable::NotNull => true,
    //             NotNullOrNullable::Nullable => true,
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
    let (columns_token_stream, postgresql_type_array) = postgresql_type_record_vec
        .into_iter()
        .enumerate()
        .collect::<std::vec::Vec<(std::primitive::usize, PostgresqlTypeRecord)>>()
        .par_iter()
        // .into_iter() //just for console prints ordering
        .map(|(index, element)| {
            // println!("{element:#?}");
            let postgresql_type = &element.postgresql_type;
            let not_null_or_nullable = &element.not_null_or_nullable;
            let postgresql_type_pattern = &element.postgresql_type_pattern;
            let postgresql_type_initialization_try_new_try_from_postgresql_type = PostgresqlTypeInitializationTryNew::try_from(postgresql_type);
            let postgresql_type_deserialize = PostgresqlTypeDeserialize::from(postgresql_type);

            let array_dimensions_number = postgresql_type_pattern.array_dimensions_number();

            let postgresql_type_range_try_from_postgresql_type = PostgresqlTypeRange::try_from(postgresql_type);
            let postgresql_type_range_try_from_postgresql_type_is_ok = postgresql_type_range_try_from_postgresql_type.is_ok();

            let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

            let column_snake_case = naming::ColumnSnakeCase;
            let query_snake_case = naming::QuerySnakeCase;
            let value_snake_case = naming::ValueSnakeCase;
            let element_snake_case = naming::ElementSnakeCase;
            let self_snake_case = naming::SelfSnakeCase;
            let increment_snake_case = naming::IncrementSnakeCase;
            let start_snake_case = naming::StartSnakeCase;
            let end_snake_case = naming::EndSnakeCase;
            let year_snake_case = naming::YearSnakeCase;
            let month_snake_case = naming::MonthSnakeCase;
            let day_snake_case = naming::DaySnakeCase;
            let months_snake_case = naming::MonthsSnakeCase;
            let days_snake_case = naming::DaysSnakeCase;
            let microseconds_snake_case = naming::MicrosecondsSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
            let test_cases_snake_case = naming::TestCasesSnakeCase;
            let error_snake_case = naming::ErrorSnakeCase;
            let acc_snake_case = naming::AccSnakeCase;
            let hour_snake_case = naming::HourSnakeCase;
            let min_snake_case = naming::MinSnakeCase;
            let sec_snake_case = naming::SecSnakeCase;
            let micro_snake_case = naming::MicroSnakeCase;
            let minute_snake_case = naming::MinuteSnakeCase;
            let second_snake_case = naming::SecondSnakeCase;
            let microsecond_snake_case = naming::MicrosecondSnakeCase;
            let error_snake_case = naming::ErrorSnakeCase;
            
            let new_or_try_new_unwraped_for_test_snake_case = naming::NewOrTryNewUnwrapedForTestSnakeCase;

            let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
            let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
            let std_string_string_token_stream = token_patterns::StdStringString;
            let std_primitive_u32_token_stream = token_patterns::StdPrimitiveU32;

            let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
            let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

            let postgresql_crud_macros_common_import_path_crate = postgresql_crud_macros_common::ImportPath::Crate;

            let generate_ident_stringified = |postgresql_type: &PostgresqlType, not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, postgresql_type_pattern: &PostgresqlTypePattern| {
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
                format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{not_null_or_nullable}{postgresql_part}")
            };
            let generate_ident_token_stream = |postgresql_type: &PostgresqlType, not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, postgresql_type_pattern: &PostgresqlTypePattern| {
                generate_ident_stringified(&postgresql_type, &not_null_or_nullable, &postgresql_type_pattern).parse::<proc_macro2::TokenStream>().unwrap()
            };
            let ident = &generate_ident_token_stream(postgresql_type, not_null_or_nullable, postgresql_type_pattern);
            let generate_ident_standart_not_null_token_stream = |postgresql_type: &PostgresqlType| generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlTypePattern::Standart);
            let ident_standart_not_null_upper_camel_case = generate_ident_standart_not_null_token_stream(postgresql_type);
            let ident_token_stream = {
                quote::quote! {
                    #[derive(Debug)]
                    pub struct #ident;
                }
            };
            let ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
            let ident_standart_not_null_or_nullable_upper_camel_case = generate_ident_token_stream(postgresql_type, not_null_or_nullable, &PostgresqlTypePattern::Standart);

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

            let sqlx_postgres_types_pg_range_token_stream = quote::quote! {sqlx::postgres::types::PgRange};

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
                quote::quote! {#field_type_standart_not_null {
                    #months_snake_case #months_token_stream,
                    #days_snake_case #days_token_stream,
                    #microseconds_snake_case #microseconds_token_stream
                }}
            };
            let ident_inner_type_token_stream = match &element.postgresql_type_pattern {
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
                PostgresqlType::StdPrimitiveBoolAsBool => CanBePrimaryKey::False,
                PostgresqlType::StdStringStringAsText => CanBePrimaryKey::False,
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesTimeTimeAsTime => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBePrimaryKey::True,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => CanBePrimaryKey::False,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => CanBePrimaryKey::False,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePrimaryKey::False,
            };
            fn generate_pg_range_conversion_token_stream(match_content_token_stream: &dyn quote::ToTokens, input_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
                quote::quote!{
                    sqlx::postgres::types::PgRange {
                        start: match #match_content_token_stream.start {
                            std::ops::Bound::Included(value) => std::ops::Bound::Included(#input_token_stream),
                            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(#input_token_stream),
                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                        },
                        end: match #match_content_token_stream.end {
                            std::ops::Bound::Included(value) => std::ops::Bound::Included(#input_token_stream),
                            std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(#input_token_stream),
                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                        },
                    }
                }
            }
            enum IsNotNullStandartCanBePrimaryKey {
                True,
                False
            }
            let is_not_null_standart_can_be_primary_key = if let (
                postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                PostgresqlTypePattern::Standart,
                CanBePrimaryKey::True
            ) = (
                &not_null_or_nullable,
                &postgresql_type_pattern,
                &can_be_primary_key
            ) {
                IsNotNullStandartCanBePrimaryKey::True
            } else {
                IsNotNullStandartCanBePrimaryKey::False
            };
            let (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl) = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&not_null_or_nullable, &postgresql_type_pattern) {
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
                let ident_standart_not_null_origin_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_standart_not_null_origin_upper_camel_case);
                enum ShouldAddBorrow {
                    True,
                    False
                }
                let generate_match_std_collections_bound_token_stream = |
                    match_token_stream: &dyn quote::ToTokens,
                    init_token_stream: &dyn quote::ToTokens,
                    should_add_borrow: &ShouldAddBorrow,
                | {
                    let maybe_borrow_token_stream = match &should_add_borrow {
                        ShouldAddBorrow::True => quote::quote!{&},
                        ShouldAddBorrow::False => proc_macro2::TokenStream::new()
                    };
                    quote::quote! {match #maybe_borrow_token_stream #match_token_stream {
                        std::collections::Bound::Included(#value_snake_case) => std::collections::Bound::Included(#init_token_stream),
                        std::collections::Bound::Excluded(#value_snake_case) => std::collections::Bound::Excluded(#init_token_stream),
                        std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                    }}
                };
                let generate_std_collections_bound_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {std::collections::Bound<#type_token_stream>}
                };
                let serde_serialize_derive_or_impl = {
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
                        quote::quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #ident_standart_not_null_origin_double_quotes_token_stream, &#self_dot_zero_token_stream #value_token_stream)}
                    };
                    let generate_serde_state_initialization_token_stream = |parameter_number: ParameterNumber| {
                        let parameter_number_token_stream = {
                            let value = parameter_number.get_vec_from_index_starting_with_one().into_iter().map(|_| quote::quote! {+ 1});
                            quote::quote! {#(#value)*}
                        };
                        quote::quote! {
                            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_standart_not_null_origin_double_quotes_token_stream, false as std::primitive::usize #parameter_number_token_stream)?;
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
                    let serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25 = {
                        let generate_self_zero_match_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens| {
                            let token_stream = generate_match_std_collections_bound_token_stream(
                                &quote::quote! {#self_dot_zero_token_stream.#value_token_stream},
                                &value_snake_case,
                                &ShouldAddBorrow::True
                            );
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
                    let impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(
                        &serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream
                    );
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
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            //todo
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl _serde::Serialize for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
                                        fn serialize<__S>(
                                            &self,
                                            __serializer: __S,
                                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                                __serializer,
                                                "SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin",
                                                false as usize + 1 + 1 + 1 + 1,
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "hour",
                                                &<sqlx::types::chrono::NaiveTime as chrono::Timelike>::hour(&self.0),
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "min",
                                                &<sqlx::types::chrono::NaiveTime as chrono::Timelike>::minute(&self.0),
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "sec",
                                                &<sqlx::types::chrono::NaiveTime as chrono::Timelike>::second(&self.0),
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "micro",
                                                &(<sqlx::types::chrono::NaiveTime as chrono::Timelike>::nanosecond(&self.0) / 1000),
                                            )?;
                                            _serde::ser::SerializeStruct::end(__serde_state)
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            //todo
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl _serde::Serialize for SqlxTypesTimeTimeAsNotNullTimeOrigin {
                                        fn serialize<__S>(
                                            &self,
                                            __serializer: __S,
                                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                                __serializer,
                                                "SqlxTypesTimeTimeAsNotNullTimeOrigin",
                                                false as usize + 1 + 1 + 1 + 1,
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "hour",
                                                &self.0.hour(),
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "minute",
                                                &self.0.minute(),
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "second",
                                                &self.0.second(),
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "microsecond",
                                                &self.0.microsecond(),
                                            )?;
                                            _serde::ser::SerializeStruct::end(__serde_state)
                                        }
                                    }
                                };
                            }
                        }),
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
                        // Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                        //     let generate_self_zero_tokens_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&#self_dot_zero_token_stream.#value()});
                        //     let year_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&year_snake_case);
                        //     let month_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&month_snake_case);
                        //     let day_serialize_field_token_stream = generate_self_zero_tokens_token_stream(&day_snake_case);
                        //     quote::quote! {
                        //         #serde_state_initialization_three_fields_token_stream
                        //         #year_serialize_field_token_stream
                        //         #month_serialize_field_token_stream
                        //         #day_serialize_field_token_stream
                        //         #serde_ser_serialize_struct_end_token_stream
                        //     }
                        // })),
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            //todo
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl _serde::Serialize for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
                                        fn serialize<__S>(
                                            &self,
                                            __serializer: __S,
                                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                                __serializer,
                                                "SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin",
                                                false as usize + 1 + 1,
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "naive_date",
                                                &crate::SqlxTypesChronoNaiveDate::try_new(self.0.date()).unwrap()
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "naive_time",
                                                &crate::SqlxTypesChronoNaiveTime::try_new(self.0.time()).unwrap()
                                            )?;
                                            _serde::ser::SerializeStruct::end(__serde_state)
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.bytes()})))
                        }
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                            postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream)
                        }
                    }
                };
                let serde_deserialize_derive_or_impl = {
                    let struct_ident_double_quotes_token_stream = postgresql_crud_macros_common::generate_struct_ident_double_quotes_token_stream(&ident_origin_upper_camel_case);
                    let postgresql_type_visitor_upper_camel_case = naming::parameter::SelfVisitorUpperCamelCase::from_tokens(&postgresql_type);
                    let struct_visitor_token_stream = quote::quote! {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<#ident_standart_not_null_origin_upper_camel_case>,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                    };
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
                            #ident_standart_not_null_origin_double_quotes_token_stream,
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
                    let match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream = quote::quote! {match #field_type_standart_not_null::try_parse(&#field_0_token_stream) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }};
                    let sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream = quote::quote! {#field_type_standart_not_null::new(#field_0_token_stream)};
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
                            generate_fn_visit_newtype_struct_token_stream(&std_primitive_i64_token_stream, &quote::quote! {#field_type_standart_not_null(#field_0_token_stream)}),
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
                        let error_message_token_stream = postgresql_crud_macros_common::generate_struct_ident_with_number_elements_double_quotes_token_stream(
                            &ident_standart_not_null_origin_upper_camel_case,
                            vec_token_stream.len()
                        );
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
                    let fn_visit_seq_pg_money_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i64_token_stream]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#field_type_standart_not_null(#field_0_token_stream)});
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
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
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                            &proc_macro2_token_stream_new,
                            &proc_macro2_token_stream_new,
                            &proc_macro2_token_stream_new
                        ));
                        quote::quote! {
                            let #months_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                            let #days_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                            let #microseconds_snake_case = #seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let sqlx_postgres_types_pg_range_start_end_token_stream = generate_qlx_postgres_types_pg_range_start_end_token_stream(&field_0_token_stream, &field_1_token_stream);
                    let sqlx_postgres_types_pg_range_bound_start_end_token_stream = generate_qlx_postgres_types_pg_range_start_end_token_stream(
                        &generate_match_std_collections_bound_token_stream(&field_0_token_stream, &value_snake_case, &ShouldAddBorrow::False),
                        &generate_match_std_collections_bound_token_stream(&field_1_token_stream, &value_snake_case, &ShouldAddBorrow::False),
                    );
                    let match_ident_standart_not_null_try_new_sqlx_postgres_types_pg_range_token_stream = quote::quote!{
                        match #ident_standart_not_null_origin_upper_camel_case::try_new(sqlx::postgres::types::PgRange {
                            start: __field0,
                            end: __field1
                        }) {
                            Ok(value) => _serde::__private::Ok(value),
                            Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                        }
                    };
                    let fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_fn_visit_seq_token_stream(&{
                        quote::quote! {
                            let #field_0_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                            let #field_1_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                            #match_ident_standart_not_null_try_new_sqlx_postgres_types_pg_range_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_fn_visit_seq_token_stream(&{
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            let #field_0_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                            let #field_1_token_stream = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let (
                        std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                        std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                    ) = {
                        let generate_std_collections_bound_ident_standart_not_null_origin = |postgresql_type: &PostgresqlType|{
                            generate_std_collections_bound_token_stream(
                                &naming::parameter::SelfOriginUpperCamelCase::from_tokens(
                                    &generate_ident_token_stream(
                                        &postgresql_type,
                                        &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        &PostgresqlTypePattern::Standart
                                    )
                                )
                            )
                        };
                        (
                            generate_std_collections_bound_ident_standart_not_null_origin(&PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                            generate_std_collections_bound_ident_standart_not_null_origin(&PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                            generate_std_collections_bound_ident_standart_not_null_origin(&PostgresqlType::SqlxTypesChronoNaiveDateAsDate),
                        )
                    };
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                            &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream
                        ]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream
                        ]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        quote::quote! {
                            #fields_initialization_token_stream
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    });
                    let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_fn_visit_seq_token_stream(&{
                        let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                            &std_collections_bound_sqlx_types_chrono_naive_date_token_stream
                        ]);
                        let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
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
                    let (fn_visit_str_value_year_month_day_token_stream, fn_visit_str_value_start_end_token_stream) = {
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
                    let (fn_visit_bytes_year_month_day_token_stream, fn_visit_bytes_start_end_token_stream) = {
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
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    ) = {
                        let generate_fn_visit_map_token_stream = |
                            field_option_none_initialization_token_stream: &dyn quote::ToTokens,
                            while_some_next_key_field_token_stream: &dyn quote::ToTokens,
                            match_field_initialization_token_stream: &dyn quote::ToTokens,
                            serde_private_ok_token_stream: &dyn quote::ToTokens
                        | {
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
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
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
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream]),
                            )
                        };
                        let (
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
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
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[
                                    (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                                    (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                                ]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream)]),
                            )
                        };
                        let match_field_initialization_start_end_token_stream = {
                            let fields_initialization_token_stream = start_end_std_fmt_display_plus_quote_to_tokens_array.iter().enumerate().map(|(index, element)| {
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
                        let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                        let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                        (
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                            ),
                        )
                    };
                    let (
                        fn_visit_map_sqlx_postgres_types_pg_interval_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream
                    ) = {
                        let generate_fn_visit_map_token_stream = |
                            field_option_none_initialization_token_stream: &dyn quote::ToTokens,
                            while_some_next_key_field_token_stream: &dyn quote::ToTokens,
                            match_field_initialization_token_stream: &dyn quote::ToTokens,
                            initialization_token_stream: &dyn quote::ToTokens
                        | {
                            quote::quote! {
                                #[inline]
                                fn visit_map<V>(self, mut map: V) -> Result<#ident_standart_not_null_origin_upper_camel_case, V::Error>
                                where
                                    V: serde::de::MapAccess<'de>,
                                {
                                    #field_option_none_initialization_token_stream
                                    #while_some_next_key_field_token_stream
                                    #match_field_initialization_token_stream
                                    #initialization_token_stream
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
                                &generate_serde_private_ok_postgresql_type_token_stream(&generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                                    &generate_double_dot_space_tokens_token_stream(&field_0_token_stream),
                                    &generate_double_dot_space_tokens_token_stream(&field_1_token_stream),
                                    &generate_double_dot_space_tokens_token_stream(&field_2_token_stream),
                                )),
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_start_end_token_stream,
                                &while_some_next_key_field_start_end_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &match_ident_standart_not_null_try_new_sqlx_postgres_types_pg_range_token_stream
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_start_end_token_stream,
                                &while_some_next_key_field_start_end_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream)
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
                    let (const_fields_sqlx_postgres_types_pg_interval_token_stream, const_fields_start_end_token_stream) = {
                        let generate_const_fields_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let field_names_token_stream = vec_token_stream.iter().map(|element| generate_quotes::double_quotes_token_stream(&element));
                            quote::quote! {
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[#(#field_names_token_stream),*];
                            }
                        };
                        (
                            generate_const_fields_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };
                    let (
                        impl_serde_de_visitor_for_visitor_pg_money_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
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
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(
                                &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            ),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream),
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
                    let (
                        impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_interval_token_stream,
                        impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                        impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                    ) = {
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
                            generate_impl_serde_de_visitor_for_ident_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream),
                            generate_impl_serde_de_visitor_for_ident_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream),
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
                    let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #field_start_end_token_stream
                            #impl_serde_deserialize_for_field_token_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_stream
                            #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                        }
                    });
                    let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #field_start_end_token_stream
                            #impl_serde_deserialize_for_field_token_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_stream
                            #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream
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
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            //todo
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for StdStringStringAsNotNullTextOrigin {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private::PhantomData<
                                                    StdStringStringAsNotNullTextOrigin,
                                                >,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = StdStringStringAsNotNullTextOrigin;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "tuple struct StdStringStringAsNotNullTextOrigin",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_newtype_struct<__E>(
                                                    self,
                                                    __e: __E,
                                                ) -> _serde::__private::Result<Self::Value, __E::Error>
                                                where
                                                    __E: _serde::Deserializer<'de>,
                                                {
                                                    let __field0: std::string::String = <std::string::String as _serde::Deserialize>::deserialize(
                                                        __e,
                                                    )?;
                                                    match StdStringStringAsNotNullTextOrigin::try_new(__field0) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                                        std::string::String,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &"tuple struct StdStringStringAsNotNullTextOrigin with 1 element",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    match StdStringStringAsNotNullTextOrigin::try_new(__field0) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                            }
                                            _serde::Deserializer::deserialize_newtype_struct(
                                                __deserializer,
                                                "StdStringStringAsNotNullTextOrigin",
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<
                                                        StdStringStringAsNotNullTextOrigin,
                                                    >,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __field1,
                                                __field2,
                                                __field3,
                                                __ignore,
                                            }
                                            #[doc(hidden)]
                                            struct __FieldVisitor;
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                                type Value = __Field;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "field identifier",
                                                    )
                                                }
                                                fn visit_u64<__E>(
                                                    self,
                                                    __value: u64,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                                        2u64 => _serde::__private::Ok(__Field::__field2),
                                                        3u64 => _serde::__private::Ok(__Field::__field3),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_str<__E>(
                                                    self,
                                                    __value: &str,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        "hour" => _serde::__private::Ok(__Field::__field0),
                                                        "min" => _serde::__private::Ok(__Field::__field1),
                                                        "sec" => _serde::__private::Ok(__Field::__field2),
                                                        "micro" => _serde::__private::Ok(__Field::__field3),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_bytes<__E>(
                                                    self,
                                                    __value: &[u8],
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        b"hour" => _serde::__private::Ok(__Field::__field0),
                                                        b"min" => _serde::__private::Ok(__Field::__field1),
                                                        b"sec" => _serde::__private::Ok(__Field::__field2),
                                                        b"micro" => _serde::__private::Ok(__Field::__field3),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de> for __Field {
                                                #[inline]
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::Deserializer::deserialize_identifier(
                                                        __deserializer,
                                                        __FieldVisitor,
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private::PhantomData<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u32,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u32,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u32,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    2usize,
                                                                    &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field3 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u32,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    3usize,
                                                                    &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new_for_deserialize(__field0, __field1, __field2, __field3) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                                #[inline]
                                                fn visit_map<__A>(
                                                    self,
                                                    mut __map: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::MapAccess<'de>,
                                                {
                                                    let mut __field0: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                                                    let mut __field1: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                                                    let mut __field2: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                                                    let mut __field3: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if _serde::__private::Option::is_some(&__field0) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("hour"),
                                                                    );
                                                                }
                                                                __field0 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u32,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field1 => {
                                                                if _serde::__private::Option::is_some(&__field1) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("min"),
                                                                    );
                                                                }
                                                                __field1 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u32,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field2 => {
                                                                if _serde::__private::Option::is_some(&__field2) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("sec"),
                                                                    );
                                                                }
                                                                __field2 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u32,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field3 => {
                                                                if _serde::__private::Option::is_some(&__field3) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("micro"),
                                                                    );
                                                                }
                                                                __field3 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u32,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = _serde::de::MapAccess::next_value::<
                                                                    _serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        _serde::__private::Some(__field0) => __field0,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("hour")?
                                                        }
                                                    };
                                                    let __field1 = match __field1 {
                                                        _serde::__private::Some(__field1) => __field1,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("min")?
                                                        }
                                                    };
                                                    let __field2 = match __field2 {
                                                        _serde::__private::Some(__field2) => __field2,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("sec")?
                                                        }
                                                    };
                                                    let __field3 = match __field3 {
                                                        _serde::__private::Some(__field3) => __field3,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("micro")?
                                                        }
                                                    };
                                                    match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new_for_deserialize(__field0, __field1, __field2, __field3) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["hour", "min", "sec", "micro"];
                                            _serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                "SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin",
                                                FIELDS,
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            //todo
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for SqlxTypesTimeTimeAsNotNullTimeOrigin {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __field1,
                                                __field2,
                                                __field3,
                                                __ignore,
                                            }
                                            #[doc(hidden)]
                                            struct __FieldVisitor;
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                                type Value = __Field;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "field identifier",
                                                    )
                                                }
                                                fn visit_u64<__E>(
                                                    self,
                                                    __value: u64,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                                        2u64 => _serde::__private::Ok(__Field::__field2),
                                                        3u64 => _serde::__private::Ok(__Field::__field3),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_str<__E>(
                                                    self,
                                                    __value: &str,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        "hour" => _serde::__private::Ok(__Field::__field0),
                                                        "minute" => _serde::__private::Ok(__Field::__field1),
                                                        "second" => _serde::__private::Ok(__Field::__field2),
                                                        "microsecond" => _serde::__private::Ok(__Field::__field3),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_bytes<__E>(
                                                    self,
                                                    __value: &[u8],
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        b"hour" => _serde::__private::Ok(__Field::__field0),
                                                        b"minute" => _serde::__private::Ok(__Field::__field1),
                                                        b"second" => _serde::__private::Ok(__Field::__field2),
                                                        b"microsecond" => _serde::__private::Ok(__Field::__field3),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de> for __Field {
                                                #[inline]
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::Deserializer::deserialize_identifier(
                                                        __deserializer,
                                                        __FieldVisitor,
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private::PhantomData<SqlxTypesTimeTimeAsNotNullTimeOrigin>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = SqlxTypesTimeTimeAsNotNullTimeOrigin;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "struct SqlxTypesTimeTimeAsNotNullTimeOrigin",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u8,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &"struct SqlxTypesTimeTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u8,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &"struct SqlxTypesTimeTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u8,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    2usize,
                                                                    &"struct SqlxTypesTimeTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field3 = match _serde::de::SeqAccess::next_element::<
                                                        std::primitive::u32,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    3usize,
                                                                    &"struct SqlxTypesTimeTimeAsNotNullTimeOrigin with 4 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    match SqlxTypesTimeTimeAsNotNullTimeOrigin::try_new_for_deserialize(__field0, __field1, __field2, __field3) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                                #[inline]
                                                fn visit_map<__A>(
                                                    self,
                                                    mut __map: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::MapAccess<'de>,
                                                {
                                                    let mut __field0: _serde::__private::Option<std::primitive::u8> = _serde::__private::None;
                                                    let mut __field1: _serde::__private::Option<std::primitive::u8> = _serde::__private::None;
                                                    let mut __field2: _serde::__private::Option<std::primitive::u8> = _serde::__private::None;
                                                    let mut __field3: _serde::__private::Option<std::primitive::u32> = _serde::__private::None;
                                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if _serde::__private::Option::is_some(&__field0) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("hour"),
                                                                    );
                                                                }
                                                                __field0 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u8,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field1 => {
                                                                if _serde::__private::Option::is_some(&__field1) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("minute"),
                                                                    );
                                                                }
                                                                __field1 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u8,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field2 => {
                                                                if _serde::__private::Option::is_some(&__field2) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("second"),
                                                                    );
                                                                }
                                                                __field2 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u8,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field3 => {
                                                                if _serde::__private::Option::is_some(&__field3) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                                            "microsecond",
                                                                        ),
                                                                    );
                                                                }
                                                                __field3 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        std::primitive::u32,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = _serde::de::MapAccess::next_value::<
                                                                    _serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        _serde::__private::Some(__field0) => __field0,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("hour")?
                                                        }
                                                    };
                                                    let __field1 = match __field1 {
                                                        _serde::__private::Some(__field1) => __field1,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("minute")?
                                                        }
                                                    };
                                                    let __field2 = match __field2 {
                                                        _serde::__private::Some(__field2) => __field2,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("second")?
                                                        }
                                                    };
                                                    let __field3 = match __field3 {
                                                        _serde::__private::Some(__field3) => __field3,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("microsecond")?
                                                        }
                                                    };
                                                    match SqlxTypesTimeTimeAsNotNullTimeOrigin::try_new_for_deserialize(__field0, __field1, __field2, __field3) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &[
                                                "hour",
                                                "minute",
                                                "second",
                                                "microsecond",
                                            ];
                                            _serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                "SqlxTypesTimeTimeAsNotNullTimeOrigin",
                                                FIELDS,
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<SqlxTypesTimeTimeAsNotNullTimeOrigin>,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            // todo
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for SqlxTypesChronoNaiveDateAsNotNullDateOrigin {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private::PhantomData<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = SqlxTypesChronoNaiveDateAsNotNullDateOrigin;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "tuple struct SqlxTypesChronoNaiveDateAsNotNullDateOrigin",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_newtype_struct<__E>(
                                                    self,
                                                    __e: __E,
                                                ) -> _serde::__private::Result<Self::Value, __E::Error>
                                                where
                                                    __E: _serde::Deserializer<'de>,
                                                {
                                                    let __field0: sqlx::types::chrono::NaiveDate = <sqlx::types::chrono::NaiveDate as _serde::Deserialize>::deserialize(
                                                        __e,
                                                    )?;
                                                    match SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(__field0) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                                        sqlx::types::chrono::NaiveDate,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &"tuple struct SqlxTypesChronoNaiveDateAsNotNullDateOrigin with 1 element",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    match SqlxTypesChronoNaiveDateAsNotNullDateOrigin::try_new(__field0) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                                    }
                                                }
                                            }
                                            _serde::Deserializer::deserialize_newtype_struct(
                                                __deserializer,
                                                "SqlxTypesChronoNaiveDateAsNotNullDateOrigin",
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<SqlxTypesChronoNaiveDateAsNotNullDateOrigin>,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            quote::quote!{
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de>
                                    for SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
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
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                                type Value = __Field;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "field identifier",
                                                    )
                                                }
                                                fn visit_u64<__E>(
                                                    self,
                                                    __value: u64,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_str<__E>(
                                                    self,
                                                    __value: &str,
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        "naive_date" => _serde::__private::Ok(__Field::__field0),
                                                        "naive_time" => _serde::__private::Ok(__Field::__field1),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_bytes<__E>(
                                                    self,
                                                    __value: &[u8],
                                                ) -> _serde::__private::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        b"naive_date" => _serde::__private::Ok(__Field::__field0),
                                                        b"naive_time" => _serde::__private::Ok(__Field::__field1),
                                                        _ => _serde::__private::Ok(__Field::__ignore),
                                                    }
                                                }
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de> for __Field {
                                                #[inline]
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::Deserializer::deserialize_identifier(
                                                        __deserializer,
                                                        __FieldVisitor,
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private::PhantomData<
                                                    SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin,
                                                >,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        "struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_seq<__A>(
                                                    self,
                                                    mut __seq: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::SeqAccess<'de>,
                                                {
                                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                                        crate::SqlxTypesChronoNaiveDate,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    0usize,
                                                                    &"struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin with 2 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                                        crate::SqlxTypesChronoNaiveTime,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &"struct SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin with 2 elements",
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    _serde::__private::Ok(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::new_for_deserialize(__field0, __field1))
                                                }
                                                #[inline]
                                                fn visit_map<__A>(
                                                    self,
                                                    mut __map: __A,
                                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::MapAccess<'de>,
                                                {
                                                    let mut __field0: _serde::__private::Option<
                                                        crate::SqlxTypesChronoNaiveDate,
                                                    > = _serde::__private::None;
                                                    let mut __field1: _serde::__private::Option<
                                                        crate::SqlxTypesChronoNaiveTime,
                                                    > = _serde::__private::None;
                                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if _serde::__private::Option::is_some(&__field0) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                                            "naive_date",
                                                                        ),
                                                                    );
                                                                }
                                                                __field0 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        crate::SqlxTypesChronoNaiveDate,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field1 => {
                                                                if _serde::__private::Option::is_some(&__field1) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                                            "naive_time",
                                                                        ),
                                                                    );
                                                                }
                                                                __field1 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        crate::SqlxTypesChronoNaiveTime,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = _serde::de::MapAccess::next_value::<
                                                                    _serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        _serde::__private::Some(__field0) => __field0,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("naive_date")?
                                                        }
                                                    };
                                                    let __field1 = match __field1 {
                                                        _serde::__private::Some(__field1) => __field1,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("naive_time")?
                                                        }
                                                    };
                                                    _serde::__private::Ok(SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin::new_for_deserialize(__field0, __field1))
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["naive_date", "naive_time"];
                                            _serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                "SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin",
                                                FIELDS,
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<
                                                        SqlxTypesChronoNaiveDateTimeAsNotNullTimestampOrigin,
                                                    >,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        }),
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Derive,
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
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(
                            &impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        )),
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
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::DeriveOrImpl::Impl({
                            generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream)
                        }),
                    }
                };
                (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl)
            } else {
                (postgresql_crud_macros_common::DeriveOrImpl::Derive, postgresql_crud_macros_common::DeriveOrImpl::Derive)
            };
            let value_ident_inner_type_token_stream = quote::quote!{#value_snake_case: #ident_inner_type_token_stream};
            let ident_standart_not_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_origin_try_new_error_named_upper_camel_case = naming::parameter::SelfOriginTryNewErrorNamedUpperCamelCase::from_display(
                &ident_standart_not_null_upper_camel_case
            );
            let ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case = naming::parameter::SelfOriginTryNewForDeserializeErrorNamedUpperCamelCase::from_display(
                &ident_standart_not_null_upper_camel_case
            );
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
                            PostgresqlType::StdPrimitiveBoolAsBool => partial_ord_comma_token_stream,
                            PostgresqlType::StdStringStringAsText => partial_ord_comma_token_stream,
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesTimeTimeAsTime => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => partial_ord_comma_token_stream,
                            PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    };
                    let maybe_derive_ord_eq_token_stream = match &is_not_null_standart_can_be_primary_key {
                        IsNotNullStandartCanBePrimaryKey::True => quote::quote! {Ord, Eq,},
                        IsNotNullStandartCanBePrimaryKey::False => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_serde_serialize_token_stream = match &serde_serialize_derive_or_impl {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Serialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let maybe_derive_serde_deserialize_token_stream = match &serde_deserialize_derive_or_impl {
                        postgresql_crud_macros_common::DeriveOrImpl::Derive => quote::quote! {serde::Deserialize,},
                        postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => proc_macro2::TokenStream::new(),
                    };
                    let type_token_stream = {
                        let field_type_handle_token_stream = quote::quote!{#field_type_handle};
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => if let Ok(ref value) = postgresql_type_range_try_from_postgresql_type {
                                    wrap_into_sqlx_postgres_types_pg_range_stringified(
                                        &naming::parameter::SelfOriginUpperCamelCase::from_display(
                                            &generate_ident_stringified(&PostgresqlType::from(value), &not_null_or_nullable, &postgresql_type_pattern)
                                        )
                                    ).parse::<proc_macro2::TokenStream>().unwrap()
                                } else {
                                    field_type_handle_token_stream
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => field_type_handle_token_stream,
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: _ } => field_type_handle_token_stream,
                        }
                    };
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #maybe_derive_partial_ord_token_stream
                            #maybe_derive_ord_eq_token_stream
                            #maybe_derive_serde_serialize_token_stream
                            #maybe_derive_serde_deserialize_token_stream
                        )]
                        pub struct #ident_origin_upper_camel_case(#type_token_stream);
                    }
                };
                let contains_null_byte_upper_camel_case = naming::ContainsNullByteUpperCamelCase;
                let earlier_date_not_supported_upper_camel_case = naming::EarlierDateNotSupportedUpperCamelCase;
                let earliest_supported_date_snake_case = naming::EarliestSupportedDateSnakeCase;
                let naive_date_upper_camel_case = naming::NaiveDateUpperCamelCase;
                let naive_time_upper_camel_case = naming::NaiveTimeUpperCamelCase;
                let invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case = naming::InvalidHourOrMinuteOrSecondOrMicrosecondUpperCamelCase;
                let included_start_more_than_included_end_upper_camel_case = naming::IncludedStartMoreThanIncludedEndUpperCamelCase;
                let included_start_more_than_excluded_end_upper_camel_case = naming::IncludedStartMoreThanExcludedEndUpperCamelCase;
                let excluded_start_more_than_included_end_upper_camel_case = naming::ExcludedStartMoreThanIncludedEndUpperCamelCase;
                let excluded_start_more_than_excluded_end_upper_camel_case = naming::ExcludedStartMoreThanExcludedEndUpperCamelCase;
                let included_end_cannot_be_max_upper_camel_case = naming::IncludedEndCannotBeMaxUpperCamelCase;
                //todo after all ranges checks rename it
                enum TempRangeType {
                    SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
                    SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                }
                impl TempRangeType {
                    fn to_range_inner_postgresql_type(&self) -> PostgresqlType {
                        match &self {
                            TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => PostgresqlType::StdPrimitiveI32AsInt4,
                            TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => PostgresqlType::StdPrimitiveI64AsInt8,
                        }
                    }
                }
                let maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_token_stream = if let (
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    PostgresqlTypePattern::Standart,
                    Ok(postgresql_type_initialization_try_new)
                ) = (
                    &not_null_or_nullable,
                    &postgresql_type_pattern,
                    &postgresql_type_initialization_try_new_try_from_postgresql_type
                ) {
                    let content_token_stream = {
                        let generate_temp_range_type_error_variants_token_stream = |temp_range_type: &TempRangeType|{
                            //todo somehow convert it with from or try from
                            let range_inner_ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(
                                &generate_ident_standart_not_null_token_stream(&temp_range_type.to_range_inner_postgresql_type())
                            );
                            quote::quote! {
                                #included_start_more_than_included_end_upper_camel_case {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                #included_start_more_than_excluded_end_upper_camel_case {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                #excluded_start_more_than_included_end_upper_camel_case {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                #excluded_start_more_than_excluded_end_upper_camel_case {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                #included_end_cannot_be_max_upper_camel_case {
                                    #[eo_to_std_string_string_serialize_deserialize]
                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            }
                        };
                        match &postgresql_type_initialization_try_new {
                            PostgresqlTypeInitializationTryNew::StdStringStringAsText => {
                                quote::quote! {
                                    #contains_null_byte_upper_camel_case {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        #value_snake_case: #ident_inner_type_token_stream,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    }
                                }
                            },
                            PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => {
                                quote::quote! {
                                    #earlier_date_not_supported_upper_camel_case {
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        #value_snake_case: #std_string_string_token_stream,
                                        #[eo_to_std_string_string_serialize_deserialize]
                                        #earliest_supported_date_snake_case: #std_string_string_token_stream,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    }
                                }
                            },
                            PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                quote::quote! {
                                    #naive_date_upper_camel_case {
                                        #[eo_error_occurence]
                                        #error_snake_case: crate::SqlxTypesChronoNaiveDateTryNewErrorNamed,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #naive_time_upper_camel_case {
                                        #[eo_error_occurence]
                                        #error_snake_case: crate::SqlxTypesChronoNaiveTimeTryNewErrorNamed,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                }
                            },
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_temp_range_type_error_variants_token_stream(
                                &TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                            ),
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_temp_range_type_error_variants_token_stream(
                                &TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                            ),
                        }
                    };
                    quote::quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_standart_not_null_origin_try_new_error_named_upper_camel_case {
                            #content_token_stream
                        }
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_token_stream = if let (
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    PostgresqlTypePattern::Standart,
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(_)
                ) = (
                    &not_null_or_nullable,
                    &postgresql_type_pattern,
                    &serde_deserialize_derive_or_impl
                ) {
                    match &postgresql_type_deserialize {
                        PostgresqlTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                        PostgresqlTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                            PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(_) => proc_macro2::TokenStream::new(),
                            PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgresql_type_impl_try_new_for_deserialize) => {
                                let content_token_stream = match &postgresql_type_impl_try_new_for_deserialize {
                                    PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => {
                                        //todo reuse
                                        quote::quote! {
                                            #contains_null_byte_upper_camel_case {
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #value_snake_case: #ident_inner_type_token_stream,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    },
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                        quote::quote!{
                                            #invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #hour_snake_case: #std_primitive_u32_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #min_snake_case: #std_primitive_u32_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #sec_snake_case: #std_primitive_u32_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #micro_snake_case: #std_primitive_u32_token_stream,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    },
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                        quote::quote!{
                                            #invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #hour_snake_case: #std_primitive_u8_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #minute_snake_case: #std_primitive_u8_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #second_snake_case: #std_primitive_u8_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #microsecond_snake_case: #std_primitive_u32_token_stream,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #error_snake_case: #std_string_string_token_stream,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    },
                                };
                                quote::quote!{
                                    #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                                    pub enum #ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case {
                                        #content_token_stream
                                    }
                                }
                            },
                        }
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                let impl_ident_origin_token_stream = {
                    let pub_fn_new_or_try_new_token_stream = {
                        if let Ok(postgresql_type_initialization_try_new) = &postgresql_type_initialization_try_new_try_from_postgresql_type {
                            let content_token_stream = {
                                let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                                    quote::quote! {Ok(Self(match #value_snake_case {
                                        Some(#value_snake_case) => Some(match #type_token_stream::try_new(#value_snake_case) {
                                            Ok(#value_snake_case) => #value_snake_case,
                                            Err(#error_snake_case) => {
                                                return Err(#error_snake_case);
                                            },
                                        }),
                                        None => None
                                    }))}
                                };
                                let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                        Ok(Self({
                                            let mut #acc_snake_case = vec![];
                                            for #element_snake_case in #value_snake_case {
                                                match #type_token_stream::try_new(#element_snake_case) {
                                                    Ok(#value_snake_case) => {
                                                        #acc_snake_case.push(#value_snake_case);
                                                    },
                                                    Err(#error_snake_case) => {
                                                        return Err(#error_snake_case);
                                                    }
                                                }
                                            }
                                            #acc_snake_case
                                        }))
                                    },
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&type_token_stream),
                                };
                                match &postgresql_type_pattern {
                                    PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                            let generate_temp_range_check_token_stream = |temp_range_type: &TempRangeType|{
                                                //todo somehow convert it with from or try from
                                                let range_inner_ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(
                                                    &generate_ident_standart_not_null_token_stream(&temp_range_type.to_range_inner_postgresql_type())
                                                );
                                                let max_value_token_stream = match &temp_range_type {
                                                    TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => quote::quote!{std::primitive::i32::MAX},
                                                    TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote::quote!{std::primitive::i64::MAX},
                                                };
                                                quote::quote! {
                                                    let max = #max_value_token_stream;
                                                    let (#start_snake_case, #end_snake_case) = match (&#value_snake_case.#start_snake_case, &#value_snake_case.#end_snake_case) {
                                                        (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Included(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_more_than_included_end_upper_camel_case {
                                                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case),
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            if *#end_snake_case == max {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (
                                                                std::ops::Bound::Included(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case)),
                                                                std::ops::Bound::Included(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case))
                                                            )
                                                        },
                                                        (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_more_than_excluded_end_upper_camel_case {
                                                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case),
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (
                                                                std::ops::Bound::Included(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case)),
                                                                std::ops::Bound::Excluded(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case))
                                                            )
                                                        },
                                                        (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Unbounded) => {
                                                            (
                                                                std::ops::Bound::Included(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case)),
                                                                std::ops::Bound::Unbounded
                                                            )
                                                        },
                                                        (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Included(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_more_than_included_end_upper_camel_case {
                                                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case),
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            if *#end_snake_case == max {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (
                                                                std::ops::Bound::Excluded(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case)),
                                                                std::ops::Bound::Included(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case))
                                                            )
                                                        },
                                                        (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_more_than_excluded_end_upper_camel_case {
                                                                    #start_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case),
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (
                                                                std::ops::Bound::Excluded(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case)),
                                                                std::ops::Bound::Excluded(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case))
                                                            )
                                                        },
                                                        (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Unbounded) => {
                                                            (
                                                                std::ops::Bound::Excluded(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#start_snake_case)),
                                                                std::ops::Bound::Unbounded
                                                            )
                                                        },
                                                        (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_snake_case)) => {
                                                            if *#end_snake_case == max {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                                    #end_snake_case: #range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (
                                                                std::ops::Bound::Unbounded,
                                                                std::ops::Bound::Included(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case))
                                                            )
                                                        },
                                                        (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_snake_case)) => {
                                                            (
                                                                std::ops::Bound::Unbounded,
                                                                std::ops::Bound::Excluded(#range_inner_ident_standart_not_null_origin_upper_camel_case::new(*#end_snake_case))
                                                            )
                                                        },
                                                        (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => {
                                                            (
                                                                std::ops::Bound::Unbounded,
                                                                std::ops::Bound::Unbounded
                                                            )
                                                        },
                                                    };
                                                    Ok(Self(sqlx::postgres::types::PgRange {
                                                        #start_snake_case,
                                                        #end_snake_case
                                                    }))
                                                }
                                            };
                                            match &postgresql_type_initialization_try_new {
                                                PostgresqlTypeInitializationTryNew::StdStringStringAsText => {
                                                    quote::quote! {
                                                        if #value_snake_case.find('\0').is_some() {
                                                            Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#contains_null_byte_upper_camel_case {
                                                                #value_snake_case,
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            })
                                                        } else {
                                                            Ok(Self(#value_snake_case))
                                                        }
                                                    }
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => {
                                                    quote::quote! {
                                                        let #earliest_supported_date_snake_case = #field_type_standart_not_null::from_ymd_opt(-4713, 1, 1).unwrap();
                                                        if #value_snake_case > #earliest_supported_date_snake_case {
                                                            Ok(Self(#value_snake_case))
                                                        }
                                                        else {
                                                            Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                                                #value_snake_case: #value_snake_case.to_string(),
                                                                #earliest_supported_date_snake_case: #earliest_supported_date_snake_case.to_string(),
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            })
                                                        }
                                                    }
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                                    quote::quote! {
                                                        let date = match crate::SqlxTypesChronoNaiveDate::try_new(#value_snake_case.date()) {
                                                            Ok(#value_snake_case) => #value_snake_case,
                                                            Err(#error_snake_case) => {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#naive_date_upper_camel_case {
                                                                    #error_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                        };
                                                        let time = match crate::SqlxTypesChronoNaiveTime::try_new(#value_snake_case.time()) {
                                                            Ok(#value_snake_case) => #value_snake_case,
                                                            Err(#error_snake_case) => {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#naive_time_upper_camel_case {
                                                                    #error_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                        };
                                                        Ok(Self(#field_type_standart_not_null::new(
                                                            date.into(),
                                                            time.into()
                                                        )))
                                                    }
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_temp_range_check_token_stream(&TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_temp_range_check_token_stream(&TempRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                            }
                                        },
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                                    },
                                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_array_dimensions_initialization_token_stream(&{
                                        let (current_postgresql_type_pattern, current_not_null_or_nullable): (&PostgresqlTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&PostgresqlTypePattern::Standart, dimension1_not_null_or_nullable),
                                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                                        };
                                        generate_current_ident_origin_non_wrapping(current_postgresql_type_pattern, current_not_null_or_nullable)
                                    }),
                                }
                            };
                            quote::quote!{
                                pub fn try_new(#value_ident_inner_type_token_stream) -> Result<Self, #ident_standart_not_null_origin_try_new_error_named_upper_camel_case> {
                                    #content_token_stream
                                }
                            }
                        }
                        else {
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
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => if let Ok(ref value) = postgresql_type_range_try_from_postgresql_type {
                                            generate_pg_range_conversion_token_stream(
                                                &value_snake_case,
                                                &{
                                                    let range_postgresql_type_ident_origin = naming::parameter::SelfOriginUpperCamelCase::from_display(
                                                        &generate_ident_stringified(
                                                            &PostgresqlType::from(value),
                                                            &not_null_or_nullable,
                                                            &postgresql_type_pattern
                                                        )
                                                    );
                                                    quote::quote!{#range_postgresql_type_ident_origin::new(value)}
                                                }
                                            )
                                        }
                                        else {
                                            quote::quote! {#value_snake_case}
                                        },
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
                                pub fn new(value: #ident_inner_type_token_stream) -> Self {
                                    Self(#content_token_stream)
                                }
                            }
                        }
                    };
                    let maybe_fn_new_or_try_new_for_deserialize_token = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_type_deserialize {
                                PostgresqlTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                                PostgresqlTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                                    PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(postgresql_type_impl_new_for_deserialize) => {
                                        let parameters_token_stream = match &postgresql_type_impl_new_for_deserialize {
                                            PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                                quote::quote!{
                                                    naive_date: crate::SqlxTypesChronoNaiveDate,
                                                    naive_time: crate::SqlxTypesChronoNaiveTime
                                                }
                                            }
                                        };
                                        let content_token_stream = match &postgresql_type_impl_new_for_deserialize {
                                            PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                                quote::quote!{
                                                    Self(#field_type_standart_not_null::new(
                                                        naive_date.into(),
                                                        naive_time.into()
                                                    ))
                                                }
                                            }
                                        };
                                        quote::quote!{
                                            fn new_for_deserialize(#parameters_token_stream) -> Self {
                                                #content_token_stream
                                            }
                                        }
                                    },
                                    PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgresql_type_impl_try_new_for_deserialize) => {
                                        let parameters_token_stream = match &postgresql_type_impl_try_new_for_deserialize {
                                            PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => {
                                                quote::quote! {
                                                    #value_ident_inner_type_token_stream
                                                }
                                            },
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                quote::quote!{
                                                    #hour_snake_case: #std_primitive_u32_token_stream,
                                                    #min_snake_case: #std_primitive_u32_token_stream,
                                                    #sec_snake_case: #std_primitive_u32_token_stream,
                                                    #micro_snake_case: #std_primitive_u32_token_stream
                                                }
                                            },
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                quote::quote!{
                                                    #hour_snake_case: #std_primitive_u8_token_stream,
                                                    #minute_snake_case: #std_primitive_u8_token_stream,
                                                    #second_snake_case: #std_primitive_u8_token_stream,
                                                    #microsecond_snake_case: #std_primitive_u32_token_stream
                                                }
                                            },
                                        };
                                        let content_token_stream = match &postgresql_type_impl_try_new_for_deserialize {
                                            PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => {
                                                //todo reuse
                                                quote::quote! {
                                                    if #value_snake_case.find('\0').is_some() {
                                                        Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#contains_null_byte_upper_camel_case {
                                                            #value_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    } else {
                                                        Ok(Self(#value_snake_case))
                                                    }
                                                }
                                            },
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                quote::quote!{
                                                    match #field_type_standart_not_null::from_hms_micro_opt(
                                                        #hour_snake_case,
                                                        #min_snake_case,
                                                        #sec_snake_case,
                                                        #micro_snake_case,
                                                    ) {
                                                        Some(#value_snake_case) => Ok(Self(#value_snake_case)),
                                                        None => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                                            #hour_snake_case,
                                                            #min_snake_case,
                                                            #sec_snake_case,
                                                            #micro_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    }
                                                }
                                            },
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                quote::quote!{
                                                    match sqlx::types::time::Time::from_hms_micro(
                                                        #hour_snake_case,
                                                        #minute_snake_case,
                                                        #second_snake_case,
                                                        #microsecond_snake_case,
                                                    ) {
                                                        Ok(#value_snake_case) => Ok(Self(#value_snake_case)),
                                                        Err(#error_snake_case) => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                                            #hour_snake_case,
                                                            #minute_snake_case,
                                                            #second_snake_case,
                                                            #microsecond_snake_case,
                                                            #error_snake_case: #error_snake_case.to_string(),
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    }
                                                }
                                            },
                                        };
                                        quote::quote!{
                                            //todo reuse naming new, try_new, new_for_deserialize and try_new_for_deserialize
                                            fn try_new_for_deserialize(#parameters_token_stream) -> Result<Self, #ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case> {
                                                #content_token_stream
                                            }
                                        }
                                    },
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: _ } => proc_macro2::TokenStream::new(),
                    };
                    //todo maybe move to test module
                    let pub_fn_new_or_try_new_unwraped_for_test_token_stream = {
                        let content_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                            quote::quote! {try_new(#value_snake_case).unwrap()}
                        }
                        else {
                            quote::quote! {new(#value_snake_case)}
                        };
                        quote::quote! {
                            pub fn #new_or_try_new_unwraped_for_test_snake_case(#value_ident_inner_type_token_stream) -> Self {
                                Self::#content_token_stream
                            }
                        }
                    };
                    quote::quote! {
                        impl #ident_origin_upper_camel_case {
                            #pub_fn_new_or_try_new_token_stream
                            #maybe_fn_new_or_try_new_for_deserialize_token
                            #pub_fn_new_or_try_new_unwraped_for_test_token_stream
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
                            PostgresqlType::StdPrimitiveBoolAsBool => proc_macro2::TokenStream::new(),
                            PostgresqlType::StdStringStringAsText => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesTimeTimeAsTime => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => impl_is_string_empty_for_ident_origin_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => proc_macro2::TokenStream::new(),
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_impl_serde_serialize_for_ident_standart_not_null_origin_token_stream = match &serde_serialize_derive_or_impl {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => &proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(value) => value,
                };
                let maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_token_stream = match &serde_deserialize_derive_or_impl {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => &proc_macro2::TokenStream::new(),
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(value) => value,
                };
                let impl_std_fmt_display_for_ident_origin_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(formatter, "{self:?}")});
                let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &ident_origin_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {self.to_string()}
                );
                // fn std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream() -> proc_macro2::TokenStream {
                //     quote::quote! {std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)}
                // }
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_origin_upper_camel_case,
                    &{
                        let content_token_stream = match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let sqlx_postgres_types_pg_range_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = generate_qlx_postgres_types_pg_range_start_end_token_stream(
                                        &quote::quote! {std::ops::Bound::Included(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                                        &quote::quote! {std::ops::Bound::Excluded(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)}
                                    );
                                    let initialization_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                                        PostgresqlType::StdPrimitiveI16AsInt2
                                        | PostgresqlType::StdPrimitiveI32AsInt4
                                        | PostgresqlType::StdPrimitiveI64AsInt8
                                        | PostgresqlType::StdPrimitiveF32AsFloat4
                                        | PostgresqlType::StdPrimitiveF64AsFloat8
                                        | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                                        | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                                        | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => &core_default_default_default_token_stream,
                                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => &quote::quote! {#field_type_standart_not_null(#core_default_default_default_token_stream)},
                                        PostgresqlType::StdPrimitiveBoolAsBool
                                        | PostgresqlType::StdStringStringAsText => &core_default_default_default_token_stream,
                                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => &quote::quote! {vec![#core_default_default_default_token_stream]},
                                        PostgresqlType::SqlxTypesTimeTimeAsTime => &quote::quote!{
                                            #field_type_standart_not_null::from_hms_micro(
                                                0,
                                                0,
                                                0,
                                                0,
                                            ).unwrap()
                                        },
                                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => &{
                                            let double_dots_space_core_default_default_default_token_stream = generate_double_dot_space_tokens_token_stream(&core_default_default_default_token_stream);
                                            generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                                                &double_dots_space_core_default_default_default_token_stream,
                                                &double_dots_space_core_default_default_default_token_stream,
                                                &double_dots_space_core_default_default_default_token_stream,
                                            )
                                        },
                                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => &core_default_default_default_token_stream,
                                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => &core_default_default_default_token_stream,
                                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &quote::quote! {
                                            #field_type_standart_not_null::new(
                                                //todo maybe reuse naming
                                                <crate::SqlxTypesChronoNaiveDate as crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element().into(),
                                                <crate::SqlxTypesChronoNaiveTime as crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element().into()
                                            )
                                        },
                                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql
                                        | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => &core_default_default_default_token_stream,
                                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => &quote::quote! {
                                            sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_token_stream).unwrap())
                                        },
                                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => &core_default_default_default_token_stream,
                                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &sqlx_postgres_types_pg_range_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
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
                    }
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
                    &ident_origin_upper_camel_case,
                    &field_type_handle
                );
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = quote::quote! {
                    impl sqlx::Encode<'_, sqlx::Postgres> for #ident_origin_upper_camel_case {
                        fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
                        }
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                    &ident_origin_upper_camel_case,
                    &field_type_handle,
                    &{
                        let scopes_value_token_stream = quote::quote! {(#value_snake_case)};
                        let ok_self_scopes_value_token_stream = quote::quote! {Ok(Self #scopes_value_token_stream)};
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_type {
                                    PostgresqlType::StdPrimitiveI16AsInt2 |
                                    PostgresqlType::StdPrimitiveI32AsInt4 |
                                    PostgresqlType::StdPrimitiveI64AsInt8 |
                                    PostgresqlType::StdPrimitiveF32AsFloat4 |
                                    PostgresqlType::StdPrimitiveF64AsFloat8 |
                                    PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql |
                                    PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql |
                                    PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql |
                                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney |
                                    PostgresqlType::StdPrimitiveBoolAsBool |
                                    PostgresqlType::StdStringStringAsText |
                                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea |
                                    PostgresqlType::SqlxTypesChronoNaiveTimeAsTime |
                                    PostgresqlType::SqlxTypesTimeTimeAsTime |
                                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval |
                                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql |
                                    PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet |
                                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_value_token_stream,
                                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate |
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote::quote! {
                                        match Self::try_new #scopes_value_token_stream {
                                            Ok(#value_snake_case) => Ok(#value_snake_case),
                                            Err(#error_snake_case) => Err(std::boxed::Box::new(error)),
                                        }
                                    }
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => ok_self_scopes_value_token_stream
                            },
                            PostgresqlTypePattern::ArrayDimension1 {..} => ok_self_scopes_value_token_stream
                        }
                    }
                );
                let impl_sqlx_postgres_pg_has_array_type_for_ident_origin_token_stream = {
                    quote::quote! {
                        impl sqlx::postgres::PgHasArrayType for #ident_origin_upper_camel_case {
                            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                                <#field_type_standart_not_null as sqlx::postgres::PgHasArrayType>::array_type_info()
                            }
                        }
                    }
                };
                let impl_create_table_column_query_part_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_create_table_column_query_part_token_stream(
                    &ident_origin_upper_camel_case,
                    match &element.postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::False,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::False,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::False,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
                    },
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
                        PostgresqlType::StdPrimitiveBoolAsBool => &proc_macro2_token_stream_new,
                        PostgresqlType::StdStringStringAsText => &proc_macro2_token_stream_new,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &proc_macro2_token_stream_new,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &proc_macro2_token_stream_new,
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
                            PostgresqlType::StdPrimitiveBoolAsBool => "bool",
                            PostgresqlType::StdStringStringAsText => "text",
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => "bytea",
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => "time",
                            PostgresqlType::SqlxTypesTimeTimeAsTime => "time",
                            PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => "interval",
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => "date",
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => "timestamp",
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => "timestamptz",
                            PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => "uuid",
                            PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => "uuid",
                            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => "inet",
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => "macaddr",
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => "int4range",
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => "int8range",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => "daterange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => "tsrange",
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => "tstzrange",
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
                let maybe_impl_std_convert_from_ident_read_for_ident_origin_token_stream = match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::True => quote::quote! {
                        impl std::convert::From<#ident_standart_not_null_read_upper_camel_case> for #ident_origin_upper_camel_case {
                            fn from(#value_snake_case: #ident_standart_not_null_read_upper_camel_case) -> Self {
                                //todo reuse as crate::PostgresqlType
                                Self::new(<#ident_standart_not_null_upper_camel_case as crate::PostgresqlType>::into_inner(#value_snake_case))
                            }
                        }
                    },
                    IsNotNullStandartCanBePrimaryKey::False => proc_macro2::TokenStream::new()
                };
                quote::quote! {
                    #ident_origin_token_stream
                    #maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_token_stream
                    #maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_token_stream
                    #impl_ident_origin_token_stream
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
                    #impl_create_table_column_query_part_for_ident_origin_token_stream
                    #maybe_impl_std_convert_from_ident_read_for_ident_origin_token_stream
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
            let ident_standart_not_null_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_or_nullable_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_standart_not_null_or_nullable_upper_camel_case);
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
                    PostgresqlType::StdPrimitiveBoolAsBool => alias_token_stream,
                    PostgresqlType::StdStringStringAsText => alias_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => alias_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsTime => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => alias_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => alias_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => ident_create_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => alias_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => alias_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => alias_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => alias_token_stream,
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
                                #dimension_number_pagination_token_stream: crate::PaginationStartsWithOne
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
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
            let ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &{
                    let common_postgresql_type_filters = vec![
                        postgresql_crud_macros_common::PostgresqlTypeFilter::Equal {
                            ident: quote::quote!{#ident_origin_upper_camel_case}
                        }
                    ];
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => {
                            let greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan {
                                ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            let between = postgresql_crud_macros_common::PostgresqlTypeFilter::Between {
                                ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            let in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::In {
                                ident: quote::quote!{#ident_table_type_declaration_upper_camel_case}
                            };
                            let regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression;
                            let equal_to_encoded_string_representation = postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation;
                            let current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate;
                            let greater_than_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate;
                            let current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime;
                            let greater_than_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime;
                            let current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp;
                            let greater_than_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp;
                            let before = postgresql_crud_macros_common::PostgresqlTypeFilter::Before {
                                ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            // let bit_vec_position_equal = postgresql_crud_macros_common::PostgresqlTypeFilter::BitVecPositionEqual;
                            let common_standart_postgresql_type_filters = {
                                let vec = common_postgresql_type_filters.clone();
                                vec
                            };
                            let common_standart_postgresql_type_number_filters = {
                                let mut vec = common_standart_postgresql_type_filters.clone();
                                vec.push(greater_than.clone());
                                vec.push(between.clone());
                                vec.push(in_handle.clone());
                                vec
                            };
                            let (
                                where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            ) = {
                                let ranges_common_filter_vec = {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesWithinGivenRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesThatFullyContainTheGivenRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanIncludedLowerBound {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanExcludedUpperBound {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength);
                                    vec
                                };
                                (
                                    ranges_common_filter_vec.clone(),
                                    ranges_common_filter_vec.clone(),
                                    ranges_common_filter_vec.clone(),
                                    ranges_common_filter_vec.clone(),
                                    ranges_common_filter_vec.clone(),
                                )
                            };
                            match &postgresql_type {
                                PostgresqlType::StdPrimitiveI16AsInt2 => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI32AsInt4 => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI64AsInt8 => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveF32AsFloat4 => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveF64AsFloat8 => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => common_standart_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => common_standart_postgresql_type_number_filters,
                                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(in_handle.clone());
                                    vec
                                },
                                PostgresqlType::StdPrimitiveBoolAsBool => common_standart_postgresql_type_filters,
                                PostgresqlType::StdStringStringAsText => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(equal_to_encoded_string_representation.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_time.clone());
                                    vec.push(greater_than_current_time.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesTimeTimeAsTime => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_time.clone());
                                    vec.push(greater_than_current_time.clone());
                                    vec
                                },
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => common_standart_postgresql_type_filters,
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_date.clone());
                                    vec.push(greater_than_current_date.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_timestamp.clone());
                                    vec.push(greater_than_current_timestamp.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(before.clone());
                                    vec.push(between.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => common_standart_postgresql_type_filters,
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            }
                        },
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            let ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case = match &postgresql_type.can_be_nullable() {
                                CanBeNullable::True => quote::quote!{#ident_standart_not_null_or_nullable_table_type_declaration_upper_camel_case},
                                CanBeNullable::False => quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            let dimension_one_greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan {
                                ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            let dimension_one_between = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween {
                                ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            let dimension_one_in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn {
                                ident: ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case.clone()
                            };
                            let dimension_one_regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression;
                            let dimension_one_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentDate;
                            let dimension_one_greater_than_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate;
                            let dimension_one_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTime;
                            let dimension_one_greater_than_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime;
                            let dimension_one_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTimestamp;
                            let dimension_one_greater_than_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp;
                            let dimension_one_before = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore {
                                ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                            };
                            let common_array_dimension1_postgresql_type_filters = {
                                let mut vec = common_postgresql_type_filters.clone();
                                vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual {
                                    ident: ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case.clone()
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthMoreThan);
                                vec
                            };
                            let common_array_dimension1_postgresql_type_number_filters = {
                                let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                vec.push(dimension_one_greater_than.clone());
                                vec.push(dimension_one_between.clone());
                                vec.push(dimension_one_in_handle.clone());
                                vec
                            };
                            let (
                                where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            ) = {
                                let generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream = |postgresql_type_range: PostgresqlTypeRange| {
                                    let postgresql_type_from_postgresql_type_range = PostgresqlType::from(&postgresql_type_range);
                                    let range_element_ident_standart_not_null_token_stream = generate_ident_standart_not_null_token_stream(&postgresql_type_from_postgresql_type_range);
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesWithinGivenRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound {
                                        ident: quote::quote!{<#range_element_ident_standart_not_null_token_stream as crate::PostgresqlType>::Read}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound {
                                        ident: quote::quote!{<#range_element_ident_standart_not_null_token_stream as crate::PostgresqlType>::Read}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanIncludedLowerBound {
                                        ident: quote::quote!{<#range_element_ident_standart_not_null_token_stream as crate::PostgresqlType>::Read}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanExcludedUpperBound {
                                        ident: quote::quote!{<#range_element_ident_standart_not_null_token_stream as crate::PostgresqlType>::Read}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange {
                                        ident: quote::quote!{#ident_standart_not_null_table_type_declaration_upper_camel_case}
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRangeLength);
                                    vec
                                };
                                (
                                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI32AsInt4),
                                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI64AsInt8),
                                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate),
                                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                                )
                            };
                            match &postgresql_type {
                                PostgresqlType::StdPrimitiveI16AsInt2 => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI32AsInt4 => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI64AsInt8 => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveF32AsFloat4 => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveF64AsFloat8 => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => common_array_dimension1_postgresql_type_number_filters,
                                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_in_handle.clone());
                                    vec
                                },
                                PostgresqlType::StdPrimitiveBoolAsBool => common_array_dimension1_postgresql_type_filters,
                                PostgresqlType::StdStringStringAsText => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation);
                                    vec
                                },
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_time.clone());
                                    vec.push(dimension_one_greater_than_current_time.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesTimeTimeAsTime => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_time.clone());
                                    vec.push(dimension_one_greater_than_current_time.clone());
                                    vec
                                },
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => common_array_dimension1_postgresql_type_filters,
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_date.clone());
                                    vec.push(dimension_one_greater_than_current_date.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_timestamp.clone());
                                    vec.push(dimension_one_greater_than_current_timestamp.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_before.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => common_array_dimension1_postgresql_type_filters,
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            }
                        },
                    }
                }.iter().map(|element|element as &dyn postgresql_crud_macros_common::PostgresqlFilter).collect(),
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::False,
                &postgresql_crud_macros_common::IsQueryBindMutable::False
            );
            let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
            let ident_read_token_stream = {
                let ident_read_token_stream = {
                    let maybe_derive_partial_ord_ord_eq_token_stream = match &is_not_null_standart_can_be_primary_key {
                        IsNotNullStandartCanBePrimaryKey::True => quote::quote! {PartialOrd, Ord, Eq,},
                        IsNotNullStandartCanBePrimaryKey::False => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {
                        #[derive(
                            Debug,
                            Clone,
                            PartialEq,
                            #maybe_derive_partial_ord_ord_eq_token_stream
                            serde::Serialize,
                            serde::Deserialize
                        )]
                        pub struct #ident_read_upper_camel_case(#ident_origin_upper_camel_case);
                    }
                };
                let impl_ident_read_token_stream = {
                    let pub_fn_new_or_try_new_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                        quote::quote! {
                            pub fn try_new(#value_ident_inner_type_token_stream) -> Result<Self, #ident_standart_not_null_origin_try_new_error_named_upper_camel_case> {
                                match #ident_origin_upper_camel_case::try_new(#value_snake_case) {
                                    Ok(#value_snake_case) => Ok(Self(#value_snake_case)),
                                    Err(#error_snake_case) => Err(#error_snake_case)
                                }
                            }
                        }
                    }
                    else {
                        quote::quote! {
                            pub fn new(#value_ident_inner_type_token_stream) -> Self {
                                Self(#ident_origin_upper_camel_case::new(#value_snake_case))
                            }
                        }
                    };
                    //todo maybe put into test module?
                    let maybe_pub_fn_new_or_try_new_unwraped_for_test_token_stream = quote::quote!{
                        pub fn #new_or_try_new_unwraped_for_test_snake_case(#value_ident_inner_type_token_stream) -> Self {
                            Self(#ident_origin_upper_camel_case::#new_or_try_new_unwraped_for_test_snake_case(#value_snake_case))
                        }
                    };
                    //todo maybe need, maybe not
                    // let maybe_pub_fn_normalize_to_postgresql_range_token_stream = if let (
                    //     postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    //     PostgresqlTypePattern::Standart,
                    //     Ok(postgresql_type_range)
                    // ) = (
                    //     &not_null_or_nullable,
                    //     &postgresql_type_pattern,
                    //     &PostgresqlTypeRange::try_from(postgresql_type)
                    // ) {
                    //     let content_token_stream = match &postgresql_type_range {
                    //         PostgresqlTypeRange::StdPrimitiveI32AsInt4 => quote::quote!{

                    //         },
                    //         PostgresqlTypeRange::StdPrimitiveI64AsInt8 => quote::quote!{},
                    //         PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate => quote::quote!{},
                    //         PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote!{},
                    //         PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote!{},
                    //     };
                    //     quote::quote!{
                    //         pub fn normalize_to_postgresql_range(self) -> Self {
                    //             let start = match range.start {
                    //                 std::ops::Bound::Excluded(#value_snake_case) => std::ops::Bound::Included(#value_snake_case + 1),
                    //                 std::ops::Bound::Included(#value_snake_case) => std::ops::Bound::Included(#value_snake_case),
                    //                 std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    //             };
                    //             let end = match range.end {
                    //                 std::ops::Bound::Included(#value_snake_case) => std::ops::Bound::Excluded(#value_snake_case + 1),
                    //                 std::ops::Bound::Excluded(#value_snake_case) => std::ops::Bound::Excluded(#value_snake_case),
                    //                 std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    //             };
                    //             Self::new(sqlx::postgres::types::PgRange { start, end })
                    //         }
                    //     }
                    // } else {
                    //     proc_macro2::TokenStream::new()
                    // };
                    quote::quote! {
                        impl #ident_read_upper_camel_case {
                            #pub_fn_new_or_try_new_token_stream
                            #maybe_pub_fn_new_or_try_new_unwraped_for_test_token_stream
                            // #maybe_pub_fn_normalize_to_postgresql_range_token_stream
                        }
                    }
                };
                let impl_error_occurence_lib_to_std_string_string_for_ident_read_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &ident_read_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {self.0.to_string()}
                );
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_read_upper_camel_case,
                    &quote::quote! {Self(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                );
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = quote::quote! {
                    impl sqlx::Encode<'_, sqlx::Postgres> for #ident_read_upper_camel_case {
                        fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
                        }
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                    &ident_read_upper_camel_case,
                    &ident_origin_upper_camel_case,
                    &quote::quote! {Ok(Self(#value_snake_case))}
                );
                let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &ident_origin_upper_camel_case);
                let (
                    maybe_impl_postgresql_type_where_filter_for_ident_read_if_can_be_primary_key_token_stream,
                    maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream
                ) = if let (CanBePrimaryKey::True, postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&can_be_primary_key, &not_null_or_nullable, &postgresql_type_pattern) {
                    (
                        postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                            &quote::quote! {<'a>},
                            &ident_standart_not_null_read_upper_camel_case,
                            &proc_macro2::TokenStream::new(),
                            &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                            &postgresql_crud_macros_common::ColumnParameterUnderscore::False,
                            &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::True,
                            &{
                                let crate_query_part_error_named_checked_add_initialization_token_stream = postgresql_crud_macros_common::crate_query_part_error_named_checked_add_initialization_token_stream();
                                quote::quote! {
                                    match #increment_snake_case.checked_add(1) {
                                        Some(value) => {
                                            *#increment_snake_case = value;
                                            Ok(format!("({} = ${})", column, #increment_snake_case))
                                        },
                                        None => Err(#crate_query_part_error_named_checked_add_initialization_token_stream)
                                    }
                                }
                            },
                            &postgresql_crud_macros_common::IsQueryBindMutable::True,
                            &generate_typical_query_bind_token_stream(&naming::SelfSnakeCase),
                            &postgresql_crud_macros_common_import_path_crate,
                        ),
                        quote::quote! {
                            impl crate::PostgresqlTypePrimaryKey for #ident_standart_not_null_upper_camel_case {
                                type PrimaryKey = #ident_standart_not_null_read_upper_camel_case;
                            }
                        },
                    )
                } else {
                    (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new())
                };
                quote::quote! {
                    #ident_read_token_stream
                    #impl_ident_read_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_read_token_stream
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
                    #impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream
                    #maybe_impl_postgresql_type_where_filter_for_ident_read_if_can_be_primary_key_token_stream
                    #maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream
                }
            };
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let ident_read_inner_token_stream = quote::quote!{
                pub type #ident_read_inner_upper_camel_case = #ident_inner_type_token_stream;
            };
            let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
            let ident_update_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_update_upper_camel_case, &ident_origin_upper_camel_case);
            let impl_postgresql_type_for_ident_token_stream = {
                let generate_ok_std_string_string_from_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {Ok(#std_string_string_token_stream::from(#content_token_stream))}
                };
                let ok_std_string_string_from_default_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"default"});
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
                        PostgresqlType::StdPrimitiveBoolAsBool => typical,
                        PostgresqlType::StdStringStringAsText => typical,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => typical,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => typical,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => typical,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => typical,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => typical,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => typical,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => typical,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => uuid_generate_v4_initialized_by_postgresql,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => typical,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => typical,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => typical,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                    }
                };
                postgresql_crud_macros_common::generate_impl_postgresql_type_for_ident_token_stream(
                    &postgresql_crud_macros_common_import_path_crate,
                    &ident,
                    &ident_table_type_declaration_upper_camel_case,
                    &ident_create_upper_camel_case,
                    &postgresql_crud_macros_common::CreateQueryPartValueUnderscore::True,
                    &match &element.postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::True,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::True,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::True,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                    },
                    &query_part_create_token_stream,
                    &match &element.postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::True,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::True,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::True,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::True,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                    },
                    &match &element.postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                    },
                    &bind_value_to_query_create_token_stream,
                    &ident_select_upper_camel_case,
                    &match &element.postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => postgresql_crud_macros_common::SelectQueryPartValueUnderscore::True,
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: _ } => postgresql_crud_macros_common::SelectQueryPartValueUnderscore::False,
                    },
                    &match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => quote::quote! {#column_snake_case.to_string()},
                        PostgresqlTypePattern::ArrayDimension1 {..}
                        // |
                        // PostgresqlTypePattern::ArrayDimension2 {..} |
                        // PostgresqlTypePattern::ArrayDimension3 {..} |
                        // PostgresqlTypePattern::ArrayDimension4 {..}
                        => {
                            //todo change trait fn select_query_part( to Result String CheckedAdd
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
                    &ident_read_inner_upper_camel_case,
                    &{
                        let generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                            quote::quote!{
                                <#ident_standart_not_null_upper_camel_case as crate::PostgresqlType>::into_inner(
                                    #ident_standart_not_null_read_upper_camel_case(#content_token_stream)
                                )
                            }
                        };
                        let value_dot_zero_token_stream = quote::quote!{#value_snake_case.0};
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => if postgresql_type_range_try_from_postgresql_type_is_ok {
                                    // generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_dot_zero_token_stream)
                                    generate_pg_range_conversion_token_stream(
                                        &quote::quote!{#value_dot_zero_token_stream.0},
                                        &value_dot_zero_token_stream
                                    )
                                }
                                else {
                                    quote::quote! {#value_dot_zero_token_stream.0}
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_snake_case)
                                    }
                                    else {
                                        value_dot_zero_token_stream.clone()
                                    };
                                    quote::quote! {
                                        match #value_dot_zero_token_stream.0 {
                                            Some(#value_snake_case) => Some(#content_token_stream),
                                            None => None
                                        }
                                    }
                                },
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&element_snake_case)
                                    }
                                    else {
                                        quote::quote! {#element_snake_case.0}
                                    };
                                    quote::quote! {
                                        #value_dot_zero_token_stream.0.into_iter().map(|#element_snake_case|#content_token_stream).collect()
                                    }
                                },
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_snake_case)
                                    }
                                    else {
                                        value_dot_zero_token_stream.clone()
                                    };
                                    quote::quote! {
                                        #value_dot_zero_token_stream.0.into_iter().map(|#element_snake_case| match #element_snake_case.0 {
                                            Some(#value_snake_case) => Some(#content_token_stream),
                                            None => None
                                        }).collect()
                                    }
                                },
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&element_snake_case)
                                    }
                                    else {
                                        quote::quote! {#element_snake_case.0}
                                    };
                                    quote::quote! {
                                        match #value_dot_zero_token_stream.0 {
                                            Some(#value_snake_case) => Some(#value_dot_zero_token_stream.into_iter().map(|element|#content_token_stream).collect()),
                                            None => None
                                        }
                                    }
                                },
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_snake_case)
                                    }
                                    else {
                                        value_dot_zero_token_stream.clone()
                                    };
                                    quote::quote! {
                                        match #value_dot_zero_token_stream.0 {
                                            Some(#value_snake_case) => Some(#value_dot_zero_token_stream.into_iter().map(|#element_snake_case| match #element_snake_case.0 {
                                                Some(#value_snake_case) => Some(#content_token_stream),
                                                None => None
                                            }).collect()),
                                            None => None
                                        }
                                    }
                                },
                            },
                        }
                    },
                    &ident_update_upper_camel_case,
                    &postgresql_crud_macros_common::UpdateQueryPartValueUnderscore::True,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetAccumulatorUnderscore::True,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetTargetUnderscore::True,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetPathUnderscore::True,
                    &typical_query_part_token_stream,
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &typical_query_bind_token_stream,
                )
            };
            let impl_postgresql_type_test_cases_for_ident_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_type_test_cases_for_ident_token_stream(
                &postgresql_crud_macros_common_import_path_crate,
                &ident,
                &{
                    let ident_standart_not_null_as_postgresql_type_read_inner_token_stream = quote::quote!{
                        <#ident_standart_not_null_upper_camel_case as crate::PostgresqlType>::ReadInner
                    };
                    match &postgresql_type_pattern {
                        //todo additional values for test
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let generate_pgrange_test_cases_token_stream = |postgresql_type_range: &PostgresqlTypeRange|{
                                    let current_ident_standart_not_null = generate_ident_standart_not_null_token_stream(&PostgresqlType::from(postgresql_type_range));
                                    quote::quote!{
                                        let mut acc = vec![];
                                        for element in <#current_ident_standart_not_null as crate::tests::PostgresqlTypeTestCases>::#test_cases_snake_case() {
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(element.clone()), end: std::ops::Bound::Included(element.clone()) });
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(element.clone()), end: std::ops::Bound::Excluded(element.clone()) });
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(element.clone()), end: std::ops::Bound::Unbounded });

                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(element.clone()), end: std::ops::Bound::Included(element.clone()) });
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(element.clone()), end: std::ops::Bound::Excluded(element.clone()) });
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(element.clone()), end: std::ops::Bound::Unbounded });

                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(element.clone()) });
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(element.clone()) });
                                            acc.push(sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded });
                                        }
                                        acc
                                    }
                                };
                                match &postgresql_type {
                                    PostgresqlType::StdPrimitiveI16AsInt2 => quote::quote!{vec![std::primitive::i16::MIN, 0, std::primitive::i16::MAX]},
                                    PostgresqlType::StdPrimitiveI32AsInt4 => quote::quote!{vec![std::primitive::i32::MIN, 0, std::primitive::i32::MAX]},
                                    PostgresqlType::StdPrimitiveI64AsInt8 => quote::quote!{vec![std::primitive::i64::MIN, 0, std::primitive::i64::MAX]},
                                    PostgresqlType::StdPrimitiveF32AsFloat4 => quote::quote!{vec![
                                        std::primitive::f32::EPSILON,
                                        std::primitive::f32::MAX,
                                        std::primitive::f32::MIN,
                                        std::primitive::f32::MIN_POSITIVE,
                                        -1e30,
                                        -1e-30,
                                        -1.0,
                                        -0.0,
                                        0.0,
                                        1.0,
                                        3.1415,
                                        -3.1415,
                                        1e-30,
                                        1e30
                                    ]},
                                    PostgresqlType::StdPrimitiveF64AsFloat8 => quote::quote!{vec![
                                        std::primitive::f64::MAX,
                                        std::primitive::f64::MIN,
                                        std::primitive::f64::MIN_POSITIVE,
                                        -1e300,
                                        -1e-300,
                                        -1.0,
                                        -0.0,
                                        0.0,
                                        1.0,
                                        1e-300,
                                        1e300
                                    ]},
                                    PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => quote::quote!{vec![]},
                                    PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => quote::quote!{vec![]},
                                    PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => quote::quote!{vec![]},
                                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => quote::quote!{vec![
                                        #field_type_standart_not_null(std::primitive::i64::MIN),
                                        #field_type_standart_not_null(0),
                                        #field_type_standart_not_null(std::primitive::i64::MAX)
                                    ]},
                                    PostgresqlType::StdPrimitiveBoolAsBool => quote::quote!{vec![true, false]},
                                    PostgresqlType::StdStringStringAsText => quote::quote!{vec![
                                        "".to_string(), // empty
                                        "a".to_string(), // single character
                                        "Hello, world!".to_string(), // basic ASCII
                                        "   ".to_string(), // spaces only
                                        "\n\r\t".to_string(), // escape/control characters
                                        "1234567890".to_string(), // numeric string
                                        "😀".to_string(), // emoji
                                        "こんにちは".to_string(), // Japanese
                                        "🌍🚀✨ Rust 💖🦀".to_string(), // mixed emoji + text
                                        "a".repeat(1024), // long string (1 KB of 'a')
                                        "line1\nline2\nline3".to_string(), // multi-line
                                        String::from_utf8_lossy(&[0xF0, 0x9F, 0x92, 0x96]).to_string(), // 💖 as raw bytes
                                    ]},
                                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => quote::quote!{vec![
                                        vec![],
                                        (0u8..=255).collect(),
                                        vec![0; 1024],
                                        vec![0; 1024 * 1024 * 2],
                                    ]},
                                    PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => quote::quote!{vec![
                                        #field_type_standart_not_null::from_hms_micro_opt(
                                            0,
                                            0,
                                            0,
                                            0,
                                        ).unwrap(),
                                        #field_type_standart_not_null::from_hms_micro_opt(
                                            23,
                                            59,
                                            59,
                                            999_999,
                                        ).unwrap()
                                    ]},
                                    PostgresqlType::SqlxTypesTimeTimeAsTime => quote::quote!{vec![
                                        #field_type_standart_not_null::from_hms_micro(
                                            0,
                                            0,
                                            0,
                                            0,
                                        ).unwrap(),
                                        #field_type_standart_not_null::from_hms_micro(
                                            23,
                                            59,
                                            59,
                                            999_999,
                                        ).unwrap()
                                    ]},
                                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => quote::quote!{vec![
                                        sqlx::postgres::types::PgInterval { months: std::primitive::i32::MIN, days: std::primitive::i32::MIN, microseconds: std::primitive::i64::MIN },
                                        sqlx::postgres::types::PgInterval { months: std::primitive::i32::MAX, days: std::primitive::i32::MAX, microseconds: std::primitive::i64::MAX },
                                    ]},
                                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate => quote::quote!{vec![
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).unwrap(),
                                        sqlx::types::chrono::NaiveDate::MAX
                                    ]},
                                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote!{vec![
                                        sqlx::types::chrono::NaiveDateTime::new(
                                            sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).unwrap(),
                                            sqlx::types::chrono::NaiveTime::from_hms_micro_opt(
                                                0,
                                                0,
                                                0,
                                                0,
                                            ).unwrap()
                                        ),
                                        sqlx::types::chrono::NaiveDateTime::new(
                                            sqlx::types::chrono::NaiveDate::MAX,
                                            sqlx::types::chrono::NaiveTime::from_hms_micro_opt(
                                                23,
                                                59,
                                                59,
                                                999_999,
                                            ).unwrap()
                                        )
                                    ]},
                                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote!{vec![
                                        crate::SqlxTypesChronoDateTimeSqlxTypesChronoUtc::new(
                                            crate::ChronoNaiveDate::try_new(
                                                chrono::NaiveDate::from_ymd_opt(-4712, 12, 31).unwrap()
                                            ).unwrap().into(),
                                            crate::SqlxTypesChronoNaiveTime::try_new(
                                                crate::Hour::try_new(0).unwrap(),
                                                crate::Minute::try_new(0).unwrap(),
                                                crate::Second::try_new(0).unwrap(),
                                                crate::Microsecond::try_new(0).unwrap(),
                                            ).unwrap().into(),
                                        ),
                                        crate::SqlxTypesChronoDateTimeSqlxTypesChronoUtc::new(
                                            crate::ChronoNaiveDate::try_new(
                                                chrono::NaiveDate::MAX
                                            ).unwrap().into(),
                                            crate::SqlxTypesChronoNaiveTime::try_new(
                                                crate::Hour::try_new(23).unwrap(),
                                                crate::Minute::try_new(59).unwrap(),
                                                crate::Second::try_new(59).unwrap(),
                                                crate::Microsecond::try_new(999_999).unwrap(),
                                            ).unwrap().into(),
                                        ),
                                    ]},
                                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => quote::quote!{vec![]},
                                    PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => quote::quote!{vec![
                                        sqlx::types::Uuid::new_v4()
                                    ]},
                                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote!{vec![
                                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").unwrap(),
                                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").unwrap(),
                                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").unwrap(),
                                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").unwrap(),
                                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").unwrap(),
                                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").unwrap(),
                                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(std::net::Ipv4Addr::new(192, 168, 0, 0), 24).unwrap()),
                                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(std::net::Ipv4Addr::new(10, 0, 0, 0), 8).unwrap()),
                                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 32).unwrap()),
                                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::new(std::net::Ipv6Addr::LOCALHOST, 128).unwrap()),
                                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::new("2001:db8::".parse().unwrap(), 32).unwrap()),
                                    ]},
                                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote!{vec![
                                        sqlx::types::mac_address::MacAddress::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                                        sqlx::types::mac_address::MacAddress::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                                        sqlx::types::mac_address::MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                                        sqlx::types::mac_address::MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                                        sqlx::types::mac_address::MacAddress::new([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                                        sqlx::types::mac_address::MacAddress::new([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                                    ]},
                                    //todo maybe reuse Range logic? like wrap 
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => 
                                    //todo
                                    // generate_pgrange_test_cases_token_stream(&PostgresqlTypeRange::StdPrimitiveI32AsInt4),
                                    quote::quote!{vec![
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Excluded(20)},
                                        sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Excluded(10)},
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(10), end: std::ops::Bound::Included(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Included(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(10), end: std::ops::Bound::Excluded(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Unbounded },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Included(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(10), end: std::ops::Bound::Excluded(20) },
                                    ]},
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => 
                                    //todo
                                    // generate_pgrange_test_cases_token_stream(&PostgresqlTypeRange::StdPrimitiveI64AsInt8),
                                    quote::quote!{vec![
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Excluded(20)},
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(10), end: std::ops::Bound::Included(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Included(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(10), end: std::ops::Bound::Excluded(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Unbounded },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(10), end: std::ops::Bound::Included(20) },
                                        // sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(10), end: std::ops::Bound::Excluded(20) },
                                    ]},
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => 
                                    //todo
                                    // generate_pgrange_test_cases_token_stream(&PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate),
                                    quote::quote!{vec![
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDate::from_ymd_opt(2001, 1, 1).unwrap()),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDate::from_ymd_opt(1970, 1, 2).unwrap()),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap()),
                                        //     end: std::ops::Bound::Unbounded,
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Unbounded,
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDate::from_ymd_opt(1999, 12, 31).unwrap()),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Unbounded,
                                        //     end: std::ops::Bound::Unbounded,
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDate::from_ymd_opt(2024, 4, 4).unwrap()),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDate::from_ymd_opt(2024, 4, 4).unwrap()),
                                        // }
                                    ]},
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => 
                                    //todo
                                    // generate_pgrange_test_cases_token_stream(&PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                                    quote::quote!{vec![
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),//todo from_hms_micro_opt
                                        //     )),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 1).unwrap(),
                                        //     )),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2020, 5, 20).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                                        //     )),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2020, 5, 20).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                                        //     )),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                                        //     )),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                                        //     )),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
                                        //     )),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                                        //     )),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDateTime::new(
                                        //        sqlx::types::chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                                        //     )),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                                        //     )),
                                        // },
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Included(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_micro_opt(12, 34, 56, 123456).unwrap(),
                                        //     )),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::NaiveDateTime::new(
                                        //         sqlx::types::chrono::NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
                                        //         sqlx::types::chrono::NaiveTime::from_hms_micro_opt(12, 34, 56, 654321).unwrap(),
                                        //     )),
                                        // },
                                    ]},
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => 
                                    //todo
                                    // generate_pgrange_test_cases_token_stream(&PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                                    quote::quote!{vec![
                                        // sqlx::postgres::types::PgRange {
                                        //     start: std::ops::Bound::Excluded(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::MIN_UTC),
                                        //     end: std::ops::Bound::Excluded(sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::MAX_UTC),
                                        // }
                                    ]},
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote!{
                                let mut acc = <#ident_standart_not_null_upper_camel_case as crate::tests::PostgresqlTypeTestCases>::#test_cases_snake_case()
                                .into_iter()
                                .map(|element|Some(element))
                                .collect::<std::vec::Vec<
                                    std::option::Option<#ident_standart_not_null_as_postgresql_type_read_inner_token_stream>
                                >>();
                                acc.push(None);
                                acc
                            },
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => quote::quote!{
                                <#ident_standart_not_null_upper_camel_case as crate::tests::PostgresqlTypeTestCases>::#test_cases_snake_case()
                                .into_iter()
                                .map(|element|vec![element])
                                .collect::<std::vec::Vec<
                                    std::vec::Vec<#ident_standart_not_null_as_postgresql_type_read_inner_token_stream>
                                >>()
                            },
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => quote::quote!{
                                let mut acc = <#ident_standart_not_null_upper_camel_case as crate::tests::PostgresqlTypeTestCases>::#test_cases_snake_case()
                                .into_iter()
                                .map(|element|vec![Some(element)])
                                .collect::<std::vec::Vec<
                                    std::vec::Vec<
                                        std::option::Option<#ident_standart_not_null_as_postgresql_type_read_inner_token_stream>
                                    >
                                >>();
                                acc.push(vec![None]);
                                acc
                            },
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => quote::quote!{
                                let mut acc = <#ident_standart_not_null_upper_camel_case as crate::tests::PostgresqlTypeTestCases>::#test_cases_snake_case()
                                .into_iter()
                                .map(|element|Some(vec![element]))
                                .collect::<std::vec::Vec<
                                    std::option::Option<
                                        std::vec::Vec<#ident_standart_not_null_as_postgresql_type_read_inner_token_stream>
                                    >
                                >>();
                                acc.push(None);
                                acc
                            },
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => quote::quote!{
                                let mut acc = <#ident_standart_not_null_upper_camel_case as crate::tests::PostgresqlTypeTestCases>::#test_cases_snake_case()
                                .into_iter()
                                .map(|element|Some(vec![Some(element)]))
                                .collect::<std::vec::Vec<
                                    std::option::Option<
                                        std::vec::Vec<
                                            std::option::Option<#ident_standart_not_null_as_postgresql_type_read_inner_token_stream>
                                        >
                                    >
                                >>();
                                acc.push(None);
                                acc.push(Some(vec![None]));
                                acc
                            },

                        }
                    }
                }
            );
            let generated = quote::quote! {
                #ident_token_stream
                #ident_origin_token_stream
                #ident_table_type_declaration_token_stream
                #ident_create_token_stream
                #ident_select_token_stream
                #ident_where_element_token_stream
                #ident_read_token_stream
                #ident_read_inner_token_stream
                #ident_update_token_stream
                #impl_postgresql_type_for_ident_token_stream
                #impl_postgresql_type_test_cases_for_ident_token_stream
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
            //     // PostgresqlType::StdPrimitiveBoolAsBool,
            //     // PostgresqlType::StdStringStringAsText,
            //     // PostgresqlType::StdVecVecStdPrimitiveU8AsBytea,
            //     // PostgresqlType::SqlxTypesChronoNaiveTimeAsTime,
            //     // PostgresqlType::SqlxTypesTimeTimeAsTime,
            //     // PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval,
            //     // PostgresqlType::SqlxTypesChronoNaiveDateAsDate,
            //     // PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp,
            //     // PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
            //     // PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql,
            //     // PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient,
            //     // PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet,
            //     // PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
            //     // PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,

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
                    let field_ident = format!("column_{index}").parse::<proc_macro2::TokenStream>().unwrap();
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
        let columns_token_stream = columns_token_stream
            .into_iter()
            .map(|element| element.parse::<proc_macro2::TokenStream>().unwrap())
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            "GeneratePostgresqlTypesExample",
            &quote::quote! {
                struct GeneratePostgresqlTypesExample {
                    #(#columns_token_stream)*
                }
            },
        );
    }
    let generated = {
        let postgresql_type_array = postgresql_type_array.into_iter().map(|element| element.parse::<proc_macro2::TokenStream>().unwrap()).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#postgresql_type_array)*}
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTypes",
    //     &generated,
    // );
    generated.into()
}
