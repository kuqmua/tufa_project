#[proc_macro]
pub fn generate_postgresql_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    // #[derive(Debug, Clone, PartialEq, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    // struct PostgresqlType {
    //     rust_type_name: RustTypeName,
    //     postgresql_type_name: PostgresqlType
    // }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlType {
        StdPrimitiveI16AsPostgresqlInt2,
        StdPrimitiveI32AsPostgresqlInt4,
        StdPrimitiveI64AsPostgresqlInt8,
        StdPrimitiveF32AsPostgresqlFloat4,
        StdPrimitiveF64AsPostgresqlFloat8,
        StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql,
        StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql,
        StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql,
        SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
        SqlxTypesDecimalAsPostgresqlNumeric,
        SqlxTypesBigDecimalAsPostgresqlNumeric,
        StdPrimitiveBoolAsPostgresqlBool,
        StdStringStringAsPostgresqlCharN,
        StdStringStringAsPostgresqlVarchar,
        StdStringStringAsPostgresqlText,
        StdVecVecStdPrimitiveU8AsPostgresqlBytea,
        SqlxTypesChronoNaiveTimeAsPostgresqlTime,
        SqlxTypesTimeTimeAsPostgresqlTime,
        SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
        SqlxTypesTimeDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
        SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql,
        SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient,
        SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
        SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
        SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
        SqlxTypesBitVecAsPostgresqlBit,
        SqlxTypesBitVecAsPostgresqlVarbit,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange,
    }
    enum CanBeNullable {
        True,
        False,
    }
    impl PostgresqlType {
        fn can_be_nullable(&self) -> CanBeNullable {
            match &self {
                Self::StdPrimitiveI16AsPostgresqlInt2 => CanBeNullable::True,
                Self::StdPrimitiveI32AsPostgresqlInt4 => CanBeNullable::True,
                Self::StdPrimitiveI64AsPostgresqlInt8 => CanBeNullable::True,
                Self::StdPrimitiveF32AsPostgresqlFloat4 => CanBeNullable::True,
                Self::StdPrimitiveF64AsPostgresqlFloat8 => CanBeNullable::True,
                Self::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => CanBeNullable::False,
                Self::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => CanBeNullable::False,
                Self::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => CanBeNullable::False,
                Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => CanBeNullable::True,
                Self::SqlxTypesDecimalAsPostgresqlNumeric => CanBeNullable::True,
                Self::SqlxTypesBigDecimalAsPostgresqlNumeric => CanBeNullable::True,
                Self::StdPrimitiveBoolAsPostgresqlBool => CanBeNullable::True,
                Self::StdStringStringAsPostgresqlCharN => CanBeNullable::True,
                Self::StdStringStringAsPostgresqlVarchar => CanBeNullable::True,
                Self::StdStringStringAsPostgresqlText => CanBeNullable::True,
                Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime => CanBeNullable::True,
                Self::SqlxTypesTimeTimeAsPostgresqlTime => CanBeNullable::True,
                Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => CanBeNullable::True,
                Self::SqlxTypesTimeDateAsPostgresqlDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateAsPostgresqlDate => CanBeNullable::True,
                Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => CanBeNullable::True,
                Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => CanBeNullable::True,
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => CanBeNullable::True,
                Self::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => CanBeNullable::False,
                Self::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => CanBeNullable::True,
                Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => CanBeNullable::True,
                Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => CanBeNullable::True,
                Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => CanBeNullable::True,
                Self::SqlxTypesBitVecAsPostgresqlBit => CanBeNullable::True,
                Self::SqlxTypesBitVecAsPostgresqlVarbit => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => CanBeNullable::True,
                Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => CanBeNullable::True,
            }
        }
        fn field_type_stringified(&self) -> std::string::String {
            let std_primitive_i16_stringified = "std::primitive::i16".to_string();
            let std_primitive_i32_stringified = "std::primitive::i32".to_string();
            let std_primitive_i64_stringified = "std::primitive::i64".to_string();
            let std_primitive_f32_stringified = "std::primitive::f32".to_string();
            let std_primitive_f64_stringified = "std::primitive::f64".to_string();
            let sqlx_postgres_types_pg_money_stringified = "sqlx::postgres::types::PgMoney".to_string();
            let sqlx_types_decimal_stringified = "sqlx::types::Decimal".to_string();
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
            let sqlx_types_bit_vec_stringified = "sqlx::types::BitVec".to_string();
            let (
                sqlx_postgres_types_pg_range_std_primitive_i32_stringified,
                sqlx_postgres_types_pg_range_std_primitive_i64_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_time_date_stringified,
                sqlx_postgres_types_pg_range_sqlx_types_decimal_stringified,
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
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_decimal_stringified),
                    wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_big_decimal_stringified),
                )
            };
            match &self {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => std_primitive_i16_stringified,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => std_primitive_i32_stringified,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => std_primitive_i64_stringified,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => std_primitive_f32_stringified,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => std_primitive_f64_stringified,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => std_primitive_i16_stringified,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => std_primitive_i32_stringified,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => std_primitive_i64_stringified,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => sqlx_postgres_types_pg_money_stringified,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => sqlx_types_decimal_stringified,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => sqlx_types_big_decimal_stringified,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => std_primitive_bool_stringified,
                PostgresqlType::StdStringStringAsPostgresqlCharN => std_string_string_stringified,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => std_string_string_stringified,
                PostgresqlType::StdStringStringAsPostgresqlText => std_string_string_stringified,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => std_vec_vec_std_primitive_u8_stringified,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => sqlx_types_chrono_naive_time_stringified,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => sqlx_types_time_time_stringified,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => sqlx_postgres_types_pg_interval_stringified,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => sqlx_types_time_date_stringified,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => sqlx_types_chrono_naive_date_stringified,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => sqlx_types_chrono_naive_date_time_stringified,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => sqlx_types_time_primitive_date_time_stringified,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => sqlx_types_uuid_uuid_stringified,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => sqlx_types_uuid_uuid_stringified,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => sqlx_types_ipnetwork_ip_network_stringified,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => sqlx_types_ipnetwork_ip_network_stringified,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => sqlx_types_mac_address_mac_address_stringified,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => sqlx_types_bit_vec_stringified,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => sqlx_types_bit_vec_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => sqlx_postgres_types_pg_range_std_primitive_i32_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => sqlx_postgres_types_pg_range_std_primitive_i64_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => sqlx_postgres_types_pg_range_sqlx_types_decimal_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => sqlx_postgres_types_pg_range_sqlx_types_big_decimal_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => sqlx_postgres_types_pg_range_sqlx_types_time_date_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_stringified,
            }
        }
        fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
            self.field_type_stringified().parse::<proc_macro2::TokenStream>().unwrap()
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
                PostgresqlTypeRange::StdPrimitiveI32AsPostgresqlInt4 => Self::StdPrimitiveI32AsPostgresqlInt4,
                PostgresqlTypeRange::StdPrimitiveI64AsPostgresqlInt8 => Self::StdPrimitiveI64AsPostgresqlInt8,
                PostgresqlTypeRange::SqlxTypesDecimalAsPostgresqlNumeric => Self::SqlxTypesDecimalAsPostgresqlNumeric,
                PostgresqlTypeRange::SqlxTypesBigDecimalAsPostgresqlNumeric => Self::SqlxTypesBigDecimalAsPostgresqlNumeric,
                PostgresqlTypeRange::SqlxTypesTimeDateAsPostgresqlDate => Self::SqlxTypesTimeDateAsPostgresqlDate,
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsPostgresqlDate => Self::SqlxTypesChronoNaiveDateAsPostgresqlDate,
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
                PostgresqlTypeRange::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
            }
        }
    }
    enum PostgresqlTypeRange {
        StdPrimitiveI32AsPostgresqlInt4,
        StdPrimitiveI64AsPostgresqlInt8,
        SqlxTypesDecimalAsPostgresqlNumeric,
        SqlxTypesBigDecimalAsPostgresqlNumeric,
        SqlxTypesTimeDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    }
    impl std::convert::TryFrom<PostgresqlType> for PostgresqlTypeRange {
        type Error = ();
        fn try_from(value: PostgresqlType) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => Err(()),
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => Err(()),
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => Err(()),
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => Err(()),
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => Err(()),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => Err(()),
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => Err(()),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => Err(()),
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => Err(()),
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => Err(()),
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => Err(()),
                PostgresqlType::StdStringStringAsPostgresqlCharN => Err(()),
                PostgresqlType::StdStringStringAsPostgresqlVarchar => Err(()),
                PostgresqlType::StdStringStringAsPostgresqlText => Err(()),
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => Err(()),
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => Err(()),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => Err(()),
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => Err(()),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => Err(()),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => Err(()),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => Err(()),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => Err(()),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => Ok(Self::StdPrimitiveI32AsPostgresqlInt4),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => Ok(Self::StdPrimitiveI64AsPostgresqlInt8),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => Ok(Self::SqlxTypesDecimalAsPostgresqlNumeric),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => Ok(Self::SqlxTypesBigDecimalAsPostgresqlNumeric),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => Ok(Self::SqlxTypesTimeDateAsPostgresqlDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsPostgresqlDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => Ok(Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz),
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
    #[derive(Debug, PartialEq, serde::Deserialize)]
    enum PostgresqlTypePatternType {
        Standart,
        ArrayDimension1 {
            dimension1_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        },
        // ArrayDimension2 {
        //     dimension1_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension2_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        // },
        // ArrayDimension3 {
        //     dimension1_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension2_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension3_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        // },
        // ArrayDimension4 {
        //     dimension1_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension2_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension3_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension4_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        // },
        // ArrayDimension5 {
        //     dimension1_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension2_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension3_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension4_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        //     dimension5_postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        // },
    }
    //todo 
    // can i create a postgresql column array of not null int?
    // No, PostgreSQL does not natively support enforcing NOT NULL on individual elements of an array column.
    // Use a CHECK constraint to validate elements
    // CREATE TABLE my_table (
    //     my_column INTEGER[] NOT NULL,
    //     CONSTRAINT no_nulls_in_array CHECK (
    //         NOT EXISTS (
    //             SELECT 1 FROM unnest(my_column) AS x(val)
    //             WHERE val IS NULL
    //         )
    //     )
    // );

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct PostgresqlTypeRecord {
        postgresql_type: PostgresqlType,
        postgresql_type_not_null_or_nullable: postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable,
        postgresql_type_pattern_type: PostgresqlTypePatternType,
    }
    let postgresql_type_array = {
        //todo write function for gen all variants
        let vec = serde_json::from_str::<std::vec::Vec<PostgresqlTypeRecord>>(&input_token_stream.to_string())
        .expect("failed to get Config for generate_postgresql_type");
        let mut acc = vec![];
        for element in &vec {
            if acc.contains(&element) {
                panic!("not unique postgersql type provided: {element:#?}");
            }
            else {
                acc.push(&element);
            }
        }
        vec
    }.into_iter().map(|element|{
        let postgresql_type = &element.postgresql_type;
        let field_type = postgresql_type.field_type_token_stream();
        let postgresql_type_not_null_or_nullable = &element.postgresql_type_not_null_or_nullable;
        let postgresql_type_pattern_type = &element.postgresql_type_pattern_type;
        if let (CanBeNullable::False, postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable) = (&postgresql_type.can_be_nullable(), &postgresql_type_not_null_or_nullable) {
            panic!("type cannot be nullable")//todo maybe rewrite it somehow better?
        }
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
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;

        let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
        let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
        let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
        let std_string_string_token_stream = token_patterns::StdStringString;

        let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
        let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

        let postgresql_type_not_null_upper_camel_case = naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&postgresql_type);
        let postgresql_type_not_null_or_nullable_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = 

        // StdPrimitiveBool
        // NullableStdPrimitiveBool

        // ArrayOfStdPrimitiveBool
        // NullableArrayOfStdPrimitiveBool
        // ArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNullableStdPrimitiveBool

        // ArrayOfArrayOfStdPrimitiveBool
        // NullableArrayOfArrayOfStdPrimitiveBool
        // ArrayOfArrayOfNullableStdPrimitiveBool
        // NullableArrayOfArrayOfNullableStdPrimitiveBool
        // ArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNullableArrayOfNullableStdPrimitiveBool

        // ArrayOfArrayOfArrayOfStdPrimitiveBool
        // NullableArrayOfArrayOfArrayOfStdPrimitiveBool
        // ArrayOfArrayOfArrayOfNullableStdPrimitiveBool
        // NullableArrayOfArrayOfArrayOfNullableStdPrimitiveBool
        // ArrayOfArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NullableArrayOfArrayOfNullableArrayOfNullableStdPrimitiveBool
        // ArrayOfNullableArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNullableArrayOfNullableArrayOfNullableStdPrimitiveBool

/////////////////

        // NotNullStdPrimitiveBool
        // NullableStdPrimitiveBool

        // NotNullArrayOfNotNullStdPrimitiveBool
        // NullableArrayOfNotNullStdPrimitiveBool
        // NotNullArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNullableStdPrimitiveBool

        // NotNullArrayOfNotNullArrayOfNotNullStdPrimitiveBool
        // NullableArrayOfNotNullArrayOfNotNullStdPrimitiveBool
        // NotNullArrayOfNotNullArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNotNullArrayOfNullableStdPrimitiveBool
        // NotNullArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNullableArrayOfNullableStdPrimitiveBool

        // NotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullStdPrimitiveBool
        // NullableArrayOfNotNullArrayOfNotNullArrayOfNotNullStdPrimitiveBool
        // NotNullArrayOfNotNullArrayOfNotNullArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNotNullArrayOfNotNullArrayOfNullableStdPrimitiveBool
        // NotNullArrayOfNotNullArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNotNullArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NotNullArrayOfNullableArrayOfNullableArrayOfNullableStdPrimitiveBool
        // NullableArrayOfNullableArrayOfNullableArrayOfNullableStdPrimitiveBool



        //todo make all like Origin: StdPrimitiveI16AsPostgresqlInt2NotNullCreate into StdPrimitiveI16AsPostgresqlInt2CreateNotNull

        // StdPrimitiveI16AsPostgresqlInt2NotNull
        // StdPrimitiveI16AsPostgresqlInt2OriginNotNull
        // StdPrimitiveI16AsPostgresqlInt2NotNullTableTypeDeclaration
        // StdPrimitiveI16AsPostgresqlInt2NotNullCreate
        // StdPrimitiveI16AsPostgresqlInt2NotNullSelect
        // StdPrimitiveI16AsPostgresqlInt2NotNullWhereElement
        // StdPrimitiveI16AsPostgresqlInt2NotNullRead
        // StdPrimitiveI16AsPostgresqlInt2NotNullUpdate

        // StdPrimitiveI16AsPostgresqlInt2Nullable
        // StdPrimitiveI16AsPostgresqlInt2OriginNullable
        // StdPrimitiveI16AsPostgresqlInt2NullableTableTypeDeclaration
        // StdPrimitiveI16AsPostgresqlInt2NullableCreate
        // StdPrimitiveI16AsPostgresqlInt2NullableSelect
        // StdPrimitiveI16AsPostgresqlInt2NullableWhereElement
        // StdPrimitiveI16AsPostgresqlInt2NullableRead
        // StdPrimitiveI16AsPostgresqlInt2NullableUpdate




        ////////////////////////////////////////////
        //1 iteration need to be 
        
        // StdPrimitiveI16AsPostgresqlInt2NotNull
        // StdPrimitiveI16AsPostgresqlInt2OriginNotNull
        // StdPrimitiveI16AsPostgresqlInt2TableTypeDeclarationNotNull
        // StdPrimitiveI16AsPostgresqlInt2CreateNotNull
        // StdPrimitiveI16AsPostgresqlInt2SelectNotNull
        // StdPrimitiveI16AsPostgresqlInt2WhereElementNotNull
        // StdPrimitiveI16AsPostgresqlInt2ReadNotNull
        // StdPrimitiveI16AsPostgresqlInt2UpdateNotNull

        // StdPrimitiveI16AsPostgresqlInt2Nullable
        // StdPrimitiveI16AsPostgresqlInt2OriginNullable
        // StdPrimitiveI16AsPostgresqlInt2TableTypeDeclarationNullable
        // StdPrimitiveI16AsPostgresqlInt2CreateNullable
        // StdPrimitiveI16AsPostgresqlInt2SelectNullable
        // StdPrimitiveI16AsPostgresqlInt2WhereElementNullable
        // StdPrimitiveI16AsPostgresqlInt2ReadNullable
        // StdPrimitiveI16AsPostgresqlInt2UpdateNullable

        //2 iteration

        // StdPrimitiveI16AsPostgresqlInt2NotNull
        // StdPrimitiveI16AsPostgresqlInt2OriginNotNull
        // StdPrimitiveI16AsPostgresqlInt2TableTypeDeclarationNotNull
        // StdPrimitiveI16AsPostgresqlInt2CreateNotNull
        // StdPrimitiveI16AsPostgresqlInt2SelectNotNull
        // StdPrimitiveI16AsPostgresqlInt2WhereElementNotNull
        // StdPrimitiveI16AsPostgresqlInt2ReadNotNull
        // StdPrimitiveI16AsPostgresqlInt2UpdateNotNull

        // OptionStdPrimitiveI16AsPostgresqlInt2Nullable
        // OptionStdPrimitiveI16AsPostgresqlInt2OriginNullable
        // OptionStdPrimitiveI16AsPostgresqlInt2TableTypeDeclarationNullable
        // OptionStdPrimitiveI16AsPostgresqlInt2CreateNullable
        // OptionStdPrimitiveI16AsPostgresqlInt2SelectNullable
        // OptionStdPrimitiveI16AsPostgresqlInt2WhereElementNullable
        // OptionStdPrimitiveI16AsPostgresqlInt2ReadNullable
        // OptionStdPrimitiveI16AsPostgresqlInt2UpdateNullable

        //3 iteration

        // NotNullInt2
        // NullableInt2

        // NotNullArrayOfNotNullInt2
        // NullableArrayOfNotNullInt2
        // NotNullArrayOfNullableInt2
        // NullableArrayOfNullableInt2


        // NotNullArrayOfNotNullArrayOfNotNullInt2
        // NullableArrayOfNotNullArrayOfNotNullInt2
        // NotNullArrayOfNotNullArrayOfNullableInt2
        // NullableArrayOfNotNullArrayOfNullableInt2
        // NotNullArrayOfNotNullArrayOfNotNullInt2


        

        // match (&postgresql_type_not_null_or_nullable, &postgresql_type_pattern_type) {
        //     (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull, PostgresqlTypePatternType::Standart) => &naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&postgresql_type),
        //     (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull, PostgresqlTypePatternType::ArrayDimension1 {
        //         dimension1_postgresql_type_not_null_or_nullable,
        //     }) => &naming::parameter::SelfNotNullUpperCamelCase::from_tokens(
        //         &naming::parameter::VecOfSelfUpperCamelCase::from_tokens(&postgresql_type)
        //     ),
        //     // VecStdPrimitiveBool
        //     (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable, PostgresqlTypePatternType::Standart) => &naming::parameter::SelfNullableUpperCamelCase::from_tokens(&postgresql_type),
        //     (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable, PostgresqlTypePatternType::ArrayDimension1 {
        //         dimension1_postgresql_type_not_null_or_nullable,
        //     }) => &naming::parameter::SelfNullableUpperCamelCase::from_tokens(
        //         &postgresql_type
        //     ),
        // };
        match &postgresql_type_not_null_or_nullable {
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => &naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&postgresql_type),
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => &naming::parameter::SelfNullableUpperCamelCase::from_tokens(&postgresql_type),
        };
        let postgresql_type_not_null_or_nullable_token_stream = {
            quote::quote! {
                #[derive(Debug)]
                pub struct #postgresql_type_not_null_or_nullable_upper_camel_case;
            }
        };
        let postgresql_type_origin_not_null_upper_camel_case = naming::parameter::SelfOriginNotNullUpperCamelCase::from_tokens(&postgresql_type);
        let postgresql_type_origin_not_null_or_nullable_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_not_null_or_nullable {
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => &postgresql_type_origin_not_null_upper_camel_case,
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => &naming::parameter::SelfOriginNullableUpperCamelCase::from_tokens(&postgresql_type),
        };
        let field_type_handle: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => &field_type,
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&postgresql_type_origin_not_null_upper_camel_case)
        };
        let postgresql_type_origin_not_null_or_nullable_token_stream = {
            let partial_ord_comma_token_stream = quote::quote! {PartialOrd,};
            let maybe_derive_partial_ord_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &partial_ord_comma_token_stream,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &partial_ord_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &partial_ord_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &partial_ord_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => &partial_ord_comma_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &partial_ord_comma_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => &proc_macro2_token_stream_new,
            };
            let serde_serialize_comma_token_stream = quote::quote! {serde::Serialize,};
            let maybe_derive_serde_serialize_token_stream: &dyn quote::ToTokens = match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => &serde_serialize_comma_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => &serde_serialize_comma_token_stream,
            };
            let serde_deserialize_comma_token_stream = quote::quote! {serde::Deserialize,};
            let maybe_derive_serde_deserialize_token_stream = match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => &serde_deserialize_comma_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => &serde_deserialize_comma_token_stream,
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
                pub struct #postgresql_type_origin_not_null_or_nullable_upper_camel_case(pub #field_type_handle);
            }
        };
        let maybe_impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream = {
            let impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream = postgresql_crud_macros_common::generate_impl_crate_is_empty_for_ident_token_stream(
                &postgresql_type_origin_not_null_or_nullable_upper_camel_case
            );
            match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlCharN => impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => proc_macro2::TokenStream::new(),
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
            }
        };

        let sqlx_types_time_primitive_date_time_as_postgresql_timestamp = PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp;
        let sqlx_types_time_date_as_postgresql_date = PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate;
        let sqlx_types_big_decimal_as_postgresql_numeric = PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric;

        let sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_field_type_token_stream = PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp.field_type_token_stream();
        let sqlx_types_time_primitive_date_time_as_postgresql_timestamp_field_type_token_stream = sqlx_types_time_primitive_date_time_as_postgresql_timestamp.field_type_token_stream();
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_field_type_token_stream = PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz.field_type_token_stream();
        let sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_field_type_token_stream = PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz.field_type_token_stream();
        let sqlx_types_chrono_naive_date_as_postgresql_date_field_type_token_stream = PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate.field_type_token_stream();
        let sqlx_types_time_date_as_postgresql_date_field_type_token_stream = sqlx_types_time_date_as_postgresql_date.field_type_token_stream();
        let sqlx_types_decimal_as_postgresql_numeric_field_type_token_stream = PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric.field_type_token_stream();
        let sqlx_types_big_decimal_as_postgresql_numeric_field_type_token_stream = sqlx_types_big_decimal_as_postgresql_numeric.field_type_token_stream();

        let sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream = naming::parameter::SelfOriginNotNullUpperCamelCase::from_display(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp);
        let sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream = naming::parameter::SelfOriginNotNullUpperCamelCase::from_display(&sqlx_types_time_date_as_postgresql_date);
        let sqlx_types_big_decimal_as_postgresql_numeric_origin_not_null_upper_camel_case_token_stream = naming::parameter::SelfOriginNotNullUpperCamelCase::from_display(&sqlx_types_big_decimal_as_postgresql_numeric);

        let sqlx_postgres_types_pg_money_field_type_token_stream = PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney.field_type_token_stream();
        let sqlx_types_uuid_uuid_field_type_token_stream = PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient.field_type_token_stream();
        let sqlx_types_mac_address_mac_address_field_type_token_stream = PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr.field_type_token_stream();
        let sqlx_types_bit_vec_field_type_token_stream = PostgresqlType::SqlxTypesBitVecAsPostgresqlBit.field_type_token_stream();
        let sqlx_postgres_types_pg_interval_field_type_token_stream = PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval.field_type_token_stream();

        let std_vec_vec_std_primitive_bool_token_stream = quote::quote! {std::vec::Vec<std::primitive::bool>};
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

        let self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};

        let maybe_impl_try_new_for_postgresql_type_origin_not_null_token_stream = {
            let impl_try_new_for_sqlx_types_time_date_token_stream = {
                let postgresql_type_not_null_try_new_error_named_upper_camel_case = naming::parameter::SelfNotNullTryNewErrorNamedUpperCamelCase::from_tokens(&postgresql_type);
                let from_calendar_date_upper_camel_case = naming::FromCalendarDateUpperCamelCase;
                let less_than_minimum_postgresql_value_upper_camel_case = naming::LessThanMinimumPostgresqlValueUpperCamelCase;
                let postgresql_type_not_null_try_new_error_named_token_stream = {
                    let error_content_token_stream = quote::quote! {
                        #[eo_to_std_string_string_serialize_deserialize]
                        value: #std_string_string_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    };
                    quote::quote! {
                        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #postgresql_type_not_null_try_new_error_named_upper_camel_case {
                            #from_calendar_date_upper_camel_case {
                                #error_content_token_stream
                            },
                            #less_than_minimum_postgresql_value_upper_camel_case {
                                #error_content_token_stream
                            },
                        }
                    }
                };
                let impl_postgresql_type_not_null_try_new_token_stream = {
                    let error_content_token_stream = quote::quote! {
                        value: format!("{value:?}"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    };
                    quote::quote! {
                        impl #postgresql_type_origin_not_null_upper_camel_case {
                            fn try_new(
                                #year_snake_case: std::primitive::i32,
                                #month_snake_case: #time_month_token_stream,
                                #day_snake_case: std::primitive::u8,
                            ) -> Result<Self, #postgresql_type_not_null_try_new_error_named_upper_camel_case> {
                                match #sqlx_types_time_date_as_postgresql_date_field_type_token_stream::from_calendar_date(
                                    #year_snake_case,
                                    #month_snake_case,
                                    #day_snake_case,
                                ) {
                                    Ok(value) => {
                                        //postgresql having minimum value "year": -4712, "month": 1, "day": 1. maximum "year": 5874897, "month": 12, "day": 31. but library type does not impl that correctly(in type max is 9999)
                                        let minimum = #sqlx_types_time_date_as_postgresql_date_field_type_token_stream::from_calendar_date(
                                            -4713,
                                            #time_month_token_stream::December,
                                            31,
                                        ).unwrap();
                                        if minimum > value {
                                            Err(#postgresql_type_not_null_try_new_error_named_upper_camel_case::#less_than_minimum_postgresql_value_upper_camel_case {
                                                #error_content_token_stream
                                            })
                                        }
                                        else {
                                            Ok(Self(value))
                                        }
                                    },
                                    Err(value) => Err(#postgresql_type_not_null_try_new_error_named_upper_camel_case::#from_calendar_date_upper_camel_case {
                                        #error_content_token_stream
                                    })
                                }
                            }
                        }
                    }
                };
                quote::quote! {
                    #postgresql_type_not_null_try_new_error_named_token_stream
                    #impl_postgresql_type_not_null_try_new_token_stream
                }
            };
            match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => impl_try_new_for_sqlx_types_time_date_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => proc_macro2::TokenStream::new(),
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
            }
        };
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
        let postgresql_type_not_null_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type_not_null_upper_camel_case);

        let generate_match_std_collections_bound_token_stream = |match_token_stream: &dyn quote::ToTokens, init_token_stream: &dyn quote::ToTokens| {
            quote::quote! {match #match_token_stream {
                std::collections::Bound::Included(#value_snake_case) => std::collections::Bound::Included(#init_token_stream),
                std::collections::Bound::Excluded(#value_snake_case) => std::collections::Bound::Excluded(#init_token_stream),
                std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
            }}
        };

        let maybe_impl_serde_serialize_for_postgresql_type_origin_not_null_token_stream = {
            let generate_impl_serde_serialize_for_postgresql_type_not_null_tokens = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl _serde::Serialize for #postgresql_type_origin_not_null_upper_camel_case {
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
                quote::quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #postgresql_type_not_null_double_quotes_token_stream, &#self_dot_zero_token_stream #value_token_stream)}
            };
            let generate_serde_state_initialization_token_stream = |parameter_number: ParameterNumber| {
                let parameter_number_token_stream = {
                    let value = parameter_number.get_vec_from_index_starting_with_one().into_iter().map(|_| quote::quote! {+ 1});
                    quote::quote! {#(#value)*}
                };
                quote::quote! {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #postgresql_type_not_null_double_quotes_token_stream, false as std::primitive::usize #parameter_number_token_stream)?;
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
            let generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25 = |type_token_stream: &dyn quote::ToTokens, is_need_to_be_cloned: std::primitive::bool| {
                let maybe_clone_token_stream: &dyn quote::ToTokens = if is_need_to_be_cloned { &quote::quote! {.clone()} } else { &proc_macro2_token_stream_new };
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
            let impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.to_string()}));
            let impl_serde_serialize_for_sqlx_types_bit_vec_token_stream = generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&quote::quote! {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    #postgresql_type_not_null_double_quotes_token_stream,
                    &#self_dot_zero_token_stream.iter().collect::<#std_vec_vec_std_primitive_bool_token_stream>(),
                )
            });
            match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.0})),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&{
                        let digits_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::DigitsSnakeCase, &quote::quote! {&#crate_postgresql_type_postgresql_type_num_bigint_big_int_token_stream(bigint)});
                        let scale_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::ScaleSnakeCase, &quote::quote! {&exponent});
                        quote::quote! {
                            let (bigint, exponent) = #self_dot_zero_token_stream.clone().into_bigint_and_exponent();
                            #serde_state_initialization_two_fields_token_stream
                            #digits_serialize_field_token_stream
                            #scale_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    }),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&{
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
                    }),
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&{
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
                    }),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.bytes()})),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => impl_serde_serialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => impl_serde_serialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => {
                        generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_big_decimal_as_postgresql_numeric_origin_not_null_upper_camel_case_token_stream, true))
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => {
                        generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream, false))
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => {
                        generate_impl_serde_serialize_for_postgresql_type_not_null_tokens(&generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream, false))
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream,
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
            }
        };

        let generate_std_collections_bound_token_stream = |type_token_stream: &dyn quote::ToTokens| {
            quote::quote! {std::collections::Bound<#type_token_stream>}
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
        let maybe_impl_serde_deserialize_for_postgresql_type_origin_not_null_token_stream = {
            let struct_ident_double_quotes_token_stream = postgresql_crud_macros_common::generate_struct_ident_double_quotes_token_stream(&postgresql_type);
            let postgresql_type_visitor_upper_camel_case = naming::parameter::SelfVisitorUpperCamelCase::from_tokens(&postgresql_type);

            let struct_visitor_token_stream = quote::quote! {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: serde::__private::PhantomData<#postgresql_type_origin_not_null_upper_camel_case>,
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
                            #postgresql_type_not_null_double_quotes_token_stream,
                            FIELDS,
                            #content_token_stream
                        )
                    }
                };
                (
                    generate_serde_deserializer_deserialize_struct_visitor_token_stream(&quote::quote! {
                        __Visitor {
                            marker: _serde::__private::PhantomData::<#postgresql_type_origin_not_null_upper_camel_case>,
                            lifetime: _serde::__private::PhantomData,
                        }
                    }),
                    generate_serde_deserializer_deserialize_struct_visitor_token_stream(&postgresql_type_visitor_upper_camel_case),
                )
            };

            let serde_deserializer_deserialize_newtype_struct_token_stream = quote::quote! {
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    #postgresql_type_not_null_double_quotes_token_stream,
                    __Visitor {
                        marker: serde::__private::PhantomData::<#postgresql_type_origin_not_null_upper_camel_case>,
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
                        impl<'de> _serde::Deserialize<'de> for #postgresql_type_origin_not_null_upper_camel_case {
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
                        fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(__formatter, #content_token_stream)
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
            let generate_serde_private_ok_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_serde_private_ok_token_stream(&quote::quote! {#postgresql_type_origin_not_null_upper_camel_case(#content_token_stream)});

            let match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream = quote::quote! {match #sqlx_types_uuid_uuid_field_type_token_stream::try_parse(&#field_0_token_stream) {
                Ok(value) => value,
                Err(error) => {
                    return Err(serde::de::Error::custom(error));
                }
            }};
            let sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream = quote::quote! {#sqlx_types_mac_address_mac_address_field_type_token_stream::new(#field_0_token_stream)};
            let array_std_primitive_u8_6_token_stream = quote::quote! {[std::primitive::u8; 6]};
            let sqlx_types_bit_vec_field_type_set_token_stream = quote::quote! {{
                let mut bit_vec = #sqlx_types_bit_vec_field_type_token_stream::from_elem(#field_0_token_stream.len(), false);
                #field_0_token_stream.into_iter().enumerate().for_each(|(index, element)|{
                    bit_vec.set(index, element);
                });
                bit_vec
            }};

            let (fn_visit_newtype_struct_pg_money_token_stream, fn_visit_newtype_struct_uuid_token_stream, fn_visit_newtype_struct_mac_address_token_stream, fn_visit_newtype_struct_bit_vec_token_stream) = {
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
                    generate_fn_visit_newtype_struct_token_stream(&std_vec_vec_std_primitive_bool_token_stream, &sqlx_types_bit_vec_field_type_set_token_stream),
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

            let serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#sqlx_types_big_decimal_as_postgresql_numeric_field_type_token_stream::new(
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
            let fn_visit_seq_sqlx_types_time_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i32_token_stream, &time_month_token_stream, &std_primitive_u8_token_stream]);
                quote::quote! {
                    #fields_initialization_token_stream
                    match #postgresql_type_origin_not_null_upper_camel_case::try_new(#field_0_token_stream, #field_1_token_stream, #field_2_token_stream) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
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
                    let std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_field_type_token_stream);
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
                    let std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                        &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_field_type_token_stream);
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
                    let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_field_type_token_stream);
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
                    let std_collections_bound_sqlx_types_chrono_naive_date_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_as_postgresql_date_field_type_token_stream);
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
                    let std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream = generate_std_collections_bound_token_stream(&sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let token_stream = generate_std_collections_bound_token_stream(&sqlx_types_decimal_as_postgresql_numeric_field_type_token_stream);
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&token_stream, &token_stream])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote! {
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let token_stream = generate_std_collections_bound_token_stream(&sqlx_types_big_decimal_as_postgresql_numeric_origin_not_null_upper_camel_case_token_stream);
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
            let fn_visit_seq_sqlx_types_bit_vec_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_vec_vec_std_primitive_bool_token_stream]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_types_bit_vec_field_type_set_token_stream);
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
                std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                std_collections_bound_sqlx_types_decimal_token_stream,
                std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream,
            ) = {
                (
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null_upper_camel_case_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_chrono_naive_date_as_postgresql_date_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_time_date_as_postgresql_date_not_null_upper_camel_case_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_decimal_as_postgresql_numeric_field_type_token_stream),
                    generate_std_collections_bound_token_stream(&sqlx_types_big_decimal_as_postgresql_numeric_origin_not_null_upper_camel_case_token_stream),
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
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
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
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
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
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                            &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_chrono_naive_date_token_stream, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_decimal_token_stream, &std_collections_bound_sqlx_types_decimal_token_stream]),
                        generate_field_option_none_initialization_token_stream(&[&std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream]),
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
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
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
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream),
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
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream),
                        ]),
                        generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_collections_bound_sqlx_types_decimal_token_stream), (&end_snake_case, &std_collections_bound_sqlx_types_decimal_token_stream)]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream),
                        ]),
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

                let match_postgresql_type_try_new_field0_field1_field2_token_stream = quote::quote! {
                    match #postgresql_type_origin_not_null_upper_camel_case::try_new(#field_0_token_stream, #field_1_token_stream, #field_2_token_stream) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
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
                        &match_postgresql_type_try_new_field0_field1_field2_token_stream,
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
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
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
                            fn visit_map<V>(self, mut map: V) -> Result<#postgresql_type_origin_not_null_upper_camel_case, V::Error>
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
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream,
                impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream,
                impl_serde_de_visitor_for_visitor_bit_vec_token_stream,
            ) = {
                let generate_impl_serde_de_visitor_for_visitor_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_origin_not_null_upper_camel_case;
                            #fn_expecting_struct_ident_double_quotes_token_stream
                            #first_token_stream
                            #second_token_stream
                        }
                    }
                };
                (
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_pg_money_token_stream, &fn_visit_seq_pg_money_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_types_big_decimal_token_stream, &fn_visit_map_sqlx_types_big_decimal_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_types_time_date_token_stream, &fn_visit_map_sqlx_types_time_date_token_stream),
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
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_uuid_token_stream, &fn_visit_seq_sqlx_types_uuid_uuid_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_mac_address_token_stream, &fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream),
                    generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_visit_newtype_struct_bit_vec_token_stream, &fn_visit_seq_sqlx_types_bit_vec_token_stream),
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
                            type Value = #postgresql_type_origin_not_null_upper_camel_case;
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
            let impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote! {
                    #struct_visitor_token_stream
                    #impl_serde_de_visitor_for_visitor_bit_vec_token_stream
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
            ///////////////////////
            match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_pg_money_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_8ae918a4_5464_4f56_8078_ab475f269079
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_big_decimal_token_stream
                            #const_fields_sqlx_types_big_decimal_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        }
                    }),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2::TokenStream::new(),
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2::TokenStream::new(),
                    // default deserialize impl can cause an postgresql error "date of out range". pub const fn from_ordinal_date( do it too. if u want to check it just use sqlx::types::time::Date::MIN
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #enum_field_three_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_77c8b6d8_4ac3_4551_8498_36b9d77317f2
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_time_date_token_stream
                            #const_fields_sqlx_types_time_date_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2::TokenStream::new(),
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        }
                    }),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #field_months_days_microseconds_token_stream
                            #impl_serde_deserialize_for_field_sqlx_postgres_types_pg_interval_token_stream
                            #impl_serde_de_visitor_for_ident_visitor_sqlx_postgres_types_pg_interval_token_stream
                            #const_fields_sqlx_postgres_types_pg_interval_token_stream
                            #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                        }
                    }),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_or_i64_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => {
                        generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => {
                        generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        }
                    }),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => {
                        generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => {
                        generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream)
                    }
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => {
                        generate_impl_serde_deserialize_for_tokens_2a45b124_f34d_4526_b85d_52516d6a5486_token_stream(&impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream)
                    }
                },
            }
        };
        let impl_std_fmt_display_for_postgresql_type_origin_not_null_or_nullable_token_stream =
            macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &postgresql_type_origin_not_null_or_nullable_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(formatter, "{self:?}")});
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_origin_not_null_or_nullable_token_stream =
            macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &postgresql_type_origin_not_null_or_nullable_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {self.to_string()});

        let sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream = quote::quote! {
            #sqlx_types_time_date_as_postgresql_date_field_type_token_stream::from_ordinal_date(
                #core_default_default_default_token_stream,
                1,
            ).unwrap()//todo
        };

        let sqlx_types_time_primitive_date_time_new_token_stream = quote::quote! {#sqlx_types_time_primitive_date_time_as_postgresql_timestamp_field_type_token_stream::new(
            #sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
            #sqlx_types_time_time_midnight_token_stream,
        )};
        // fn std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream() -> proc_macro2::TokenStream {
        //     quote::quote! {std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)}
        // }

        let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_origin_not_null_or_nullable_token_stream = match &postgresql_type_not_null_or_nullable {
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&postgresql_type_origin_not_null_or_nullable_upper_camel_case, &{
                let generate_sqlx_postgres_types_pg_range_token_stream =
                    |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| generate_qlx_postgres_types_pg_range_start_end_token_stream(&quote::quote! {std::ops::Bound::Included(#start_token_stream)}, &quote::quote! {std::ops::Bound::Excluded(#end_token_stream)});
                let sqlx_postgres_types_pg_range_core_default_default_default_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&core_default_default_default_token_stream, &core_default_default_default_token_stream);
                let initialization_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2
                    | PostgresqlType::StdPrimitiveI32AsPostgresqlInt4
                    | PostgresqlType::StdPrimitiveI64AsPostgresqlInt8
                    | PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4
                    | PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8
                    | PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &quote::quote! {#sqlx_postgres_types_pg_money_field_type_token_stream(#core_default_default_default_token_stream)},
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric
                    | PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric
                    | PostgresqlType::StdPrimitiveBoolAsPostgresqlBool
                    | PostgresqlType::StdStringStringAsPostgresqlCharN
                    | PostgresqlType::StdStringStringAsPostgresqlVarchar
                    | PostgresqlType::StdStringStringAsPostgresqlText => &core_default_default_default_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &quote::quote! {vec![#core_default_default_default_token_stream]},
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &quote::quote! {#sqlx_types_time_time_midnight_token_stream},
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &{
                        let double_dots_space_core_default_default_default_token_stream = generate_double_dot_space_tokens_token_stream(&core_default_default_default_token_stream);
                        generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(
                            &double_dots_space_core_default_default_default_token_stream,
                            &double_dots_space_core_default_default_default_token_stream,
                            &double_dots_space_core_default_default_default_token_stream,
                        )
                    },
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate | PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &core_default_default_default_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &core_default_default_default_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &sqlx_types_time_primitive_date_time_new_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz
                    | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz
                    | PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql
                    | PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &core_default_default_default_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &quote::quote! {
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_token_stream).unwrap())
                    },
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &core_default_default_default_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit | PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &quote::quote! {{
                        let mut value = #sqlx_types_bit_vec_field_type_token_stream::new();
                        value.push(false);
                        value
                    }},
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &generate_sqlx_postgres_types_pg_range_token_stream(
                        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                    ),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &generate_sqlx_postgres_types_pg_range_token_stream(&sqlx_types_time_primitive_date_time_new_token_stream, &sqlx_types_time_primitive_date_time_new_token_stream),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                };
                quote::quote! {Self(#initialization_token_stream)}
            }),
            postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &postgresql_type_origin_not_null_or_nullable_upper_camel_case,
                &quote::quote! {Self(
                    Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                )},
            ),
        };
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_origin_not_null_or_nullable_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&postgresql_type_origin_not_null_or_nullable_upper_camel_case, &field_type_handle);
        let impl_sqlx_encode_sqlx_postgres_for_postgresql_type_origin_not_null_or_nullable_token_stream = {
            let self_snake_case = naming::SelfSnakeCase;
            quote::quote! {
                impl sqlx::Encode<'_, sqlx::Postgres> for #postgresql_type_origin_not_null_or_nullable_upper_camel_case {
                    fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
                        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
                    }
                }
            }
        };
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_origin_not_null_or_nullable_token_stream =
            postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&postgresql_type_origin_not_null_or_nullable_upper_camel_case, &field_type_handle, &quote::quote! {Ok(Self(#value_snake_case))});
        let impl_sqlx_postgres_pg_has_array_type_for_postgresql_type_origin_not_null_or_nullable_token_stream = quote::quote! {
            impl sqlx::postgres::PgHasArrayType for #postgresql_type_origin_not_null_or_nullable_upper_camel_case {
                fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                    <#field_type as sqlx::postgres::PgHasArrayType>::array_type_info()
                }
            }
        };
        let crate_query_part_error_named_token_stream = postgresql_crud_macros_common::crate_query_part_error_named_token_stream();
        enum CanBePrimaryKey {
            True,
            False,
        }
        let can_be_primary_key = match &postgresql_type {
            PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => CanBePrimaryKey::True,
            PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => CanBePrimaryKey::True,
            PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => CanBePrimaryKey::True,
            PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => CanBePrimaryKey::False,
            PostgresqlType::StdStringStringAsPostgresqlCharN => CanBePrimaryKey::False,
            PostgresqlType::StdStringStringAsPostgresqlVarchar => CanBePrimaryKey::False,
            PostgresqlType::StdStringStringAsPostgresqlText => CanBePrimaryKey::False,
            PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => CanBePrimaryKey::True,
            PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => CanBePrimaryKey::False,
            PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => CanBePrimaryKey::False,
            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => CanBePrimaryKey::False,
        };
        let generate_typical_query_bind_token_stream = |content_token_stream: &dyn quote::ToTokens|{
            match &postgresql_type_not_null_or_nullable {
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull => quote::quote! {
                    #query_snake_case = #query_snake_case.bind(#content_token_stream);
                    #query_snake_case
                },
                postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable => quote::quote! {
                    #query_snake_case = #query_snake_case.bind(match #content_token_stream .0 {
                        Some(#value_snake_case) => Some(#value_snake_case),
                        None => None
                    });
                    #query_snake_case
                },
            }
        };
        let typical_query_bind_token_stream = generate_typical_query_bind_token_stream(&value_snake_case);
        let postgresql_crud_macros_common_import_path_crate = postgresql_crud_macros_common::ImportPath::Crate;
        let maybe_impl_postgresql_type_where_filter_for_postgresql_type_origin_not_null_or_nullable_if_can_be_primary_key_token_stream = if let (CanBePrimaryKey::True, postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull) = (&can_be_primary_key, &postgresql_type_not_null_or_nullable) {
            postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                &quote::quote! {<'a>},
                &postgresql_type_origin_not_null_upper_camel_case,
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
            )
        } else {
            proc_macro2::TokenStream::new()
        };
        let maybe_impl_postgresql_type_primary_key_for_postgresql_type_not_null_or_nullable_if_can_be_primary_key_token_stream = if let (CanBePrimaryKey::True, postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull) = (&can_be_primary_key, &postgresql_type_not_null_or_nullable) {
            quote::quote!{
                impl crate::PostgresqlTypePrimaryKey for #postgresql_type_not_null_upper_camel_case {
                    type PrimaryKey = #postgresql_type_origin_not_null_upper_camel_case;
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let impl_create_table_column_query_part_for_postgresql_type_origin_not_null_or_nullable_token_stream = {
            let fixed_length_snake_case = naming::FixedLengthSnakeCase;
            let fixed_length_parameter_token_stream = {
                let postgresql_type_length_upper_camel_case = naming::parameter::SelfLengthUpperCamelCase::from_tokens(&postgresql_type);
                quote::quote! {, #fixed_length_snake_case: #postgresql_type_length_upper_camel_case}
            };
            postgresql_crud_macros_common::generate_create_table_column_query_part_token_stream(
                &postgresql_type_origin_not_null_or_nullable_upper_camel_case,
                &match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => &proc_macro2_token_stream_new,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => &proc_macro2_token_stream_new,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => &fixed_length_parameter_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => &fixed_length_parameter_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => &proc_macro2_token_stream_new,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &fixed_length_parameter_token_stream,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &fixed_length_parameter_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => &proc_macro2_token_stream_new,
                },
                &{
                    let postgresql_query_type = match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => "int2",
                        PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => "int4",
                        PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => "int8",
                        PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => "float4",
                        PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => "float8",
                        PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => "smallserial",
                        PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => "serial",
                        PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => "bigserial",
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => "money",
                        PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => "numeric",
                        PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => "numeric",
                        PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => "bool",
                        PostgresqlType::StdStringStringAsPostgresqlCharN => &format!("char({{{fixed_length_snake_case}}})"),
                        PostgresqlType::StdStringStringAsPostgresqlVarchar => &format!("varchar({{{fixed_length_snake_case}}})"),
                        PostgresqlType::StdStringStringAsPostgresqlText => "text",
                        PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => "bytea",
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => "time",
                        PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => "time",
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => "interval",
                        PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => "date",
                        PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => "date",
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => "timestamp",
                        PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => "timestamp",
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => "timestamptz",
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => "timestamptz",
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => "uuid",
                        PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => "uuid",
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => "inet",
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => "cidr",
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => "macaddr",
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => &format!("bit({{{fixed_length_snake_case}}})"),
                        PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &format!("bit varying({{{fixed_length_snake_case}}})"),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => "int4range",
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => "int8range",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => "numrange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => "numrange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => "daterange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => "daterange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => "tsrange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => "tsrange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => "tstzrange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => "tstzrange",
                    };
                    let crate_maybe_primary_key_is_primary_key_token_stream = quote::quote! {crate::maybe_primary_key(is_primary_key)};
                    let column_postgresql_query_type = format!("{{column}} {postgresql_query_type}");
                    let column_postgresql_query_type_not_null = format!("{column_postgresql_query_type} not null");
                    let space_additional_parameter = " {}";
                    match (&postgresql_type_not_null_or_nullable, &can_be_primary_key) {
                        (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull, CanBePrimaryKey::False) => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type_not_null.to_string());
                            quote::quote! {
                                format!(#format_handle_token_stream)
                            }
                        }
                        (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull, CanBePrimaryKey::True) => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{column_postgresql_query_type_not_null}{space_additional_parameter}"));
                            quote::quote! {
                                format!(#format_handle_token_stream, #crate_maybe_primary_key_is_primary_key_token_stream)
                            }
                        }
                        (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable, CanBePrimaryKey::False) => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type);
                            quote::quote! {
                                format!(#format_handle_token_stream)
                            }
                        }
                        (postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::Nullable, CanBePrimaryKey::True) => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{column_postgresql_query_type}{space_additional_parameter}"));
                            quote::quote! {
                                format!(#format_handle_token_stream, #crate_maybe_primary_key_is_primary_key_token_stream)
                            }
                        }
                    }
                },
            )
        };
        fn generate_pub_struct_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens, impl_default: std::primitive::bool, impl_deserialize: std::primitive::bool) -> proc_macro2::TokenStream {
            let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
            let maybe_impl_default_token_stream: &dyn quote::ToTokens = if impl_default { &quote::quote! {Default,} } else { &proc_macro2_token_stream_new };
            let maybe_impl_serde_deserialize_token_stream: &dyn quote::ToTokens = if impl_deserialize { &quote::quote! {serde::Deserialize,} } else { &proc_macro2_token_stream_new };
            quote::quote! {
                #[derive(
                    Debug,
                    #maybe_impl_default_token_stream
                    Clone,
                    PartialEq,
                    serde::Serialize,
                    #maybe_impl_serde_deserialize_token_stream
                )]
                pub struct #ident_token_stream #content_token_stream
            }
        }
        let postgresql_type_not_null_or_nullable_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
        let postgresql_type_not_null_or_nullable_table_type_declaration_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            &postgresql_type_not_null_or_nullable_table_type_declaration_upper_camel_case,
            &postgresql_type_origin_not_null_or_nullable_upper_camel_case
        );
        let postgresql_type_not_null_or_nullable_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
        let postgresql_type_not_null_or_nullable_create_token_stream = {
            let postgresql_type_not_null_or_nullable_create_token_stream = {
                let postgresql_type_not_null_or_nullable_create_token_stream = generate_pub_struct_tokens_token_stream(&postgresql_type_not_null_or_nullable_create_upper_camel_case, &quote::quote! {(());}, false, true);
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_create_token_stream =
                    postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_create_upper_camel_case, &quote::quote! {Self(#core_default_default_default_token_stream)});
                quote::quote! {
                    #postgresql_type_not_null_or_nullable_create_token_stream
                    #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_create_token_stream
                }
            };
            let alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_type_not_null_or_nullable_create_upper_camel_case, &postgresql_type_origin_not_null_or_nullable_upper_camel_case);
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => alias_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => alias_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => alias_token_stream,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => alias_token_stream,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => alias_token_stream,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => postgresql_type_not_null_or_nullable_create_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => postgresql_type_not_null_or_nullable_create_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => postgresql_type_not_null_or_nullable_create_token_stream,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => alias_token_stream,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => alias_token_stream,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => alias_token_stream,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => alias_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlCharN => alias_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => alias_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlText => alias_token_stream,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => alias_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => alias_token_stream,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => alias_token_stream,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => alias_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => alias_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => alias_token_stream,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => alias_token_stream,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => alias_token_stream,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => alias_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => postgresql_type_not_null_or_nullable_create_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => alias_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => alias_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => alias_token_stream,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => alias_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => alias_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => alias_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => alias_token_stream,
            }
        };
        let postgresql_type_not_null_or_nullable_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
        let postgresql_type_not_null_or_nullable_select_token_stream = {
            let pub_struct_postgresql_type_not_null_or_nullable_select_token_stream = generate_pub_struct_tokens_token_stream(&postgresql_type_not_null_or_nullable_select_upper_camel_case, &quote::quote! {;}, true, true);
            let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_select_token_stream =
                postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&postgresql_type_not_null_or_nullable_select_upper_camel_case, &core_default_default_default_token_stream);
            quote::quote! {
                #pub_struct_postgresql_type_not_null_or_nullable_select_token_stream
                #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_not_null_or_nullable_select_token_stream
            }
        };
        let postgresql_type_not_null_or_nullable_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
        let postgresql_type_not_null_or_nullable_read_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            &postgresql_type_not_null_or_nullable_read_upper_camel_case,
            &postgresql_type_origin_not_null_or_nullable_upper_camel_case
        );
        let postgresql_type_not_null_or_nullable_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
        let postgresql_type_not_null_or_nullable_update_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            &postgresql_type_not_null_or_nullable_update_upper_camel_case,
            &postgresql_type_origin_not_null_or_nullable_upper_camel_case
        );
        let postgresql_type_not_null_or_nullable_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&postgresql_type_not_null_or_nullable_upper_camel_case);
        let postgresql_type_not_null_or_nullable_where_element_token_stream = {
            let generate_postgresql_type_not_null_or_nullable_where_element_token_stream = |variants: &std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter>| {
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    variants,
                    |is_relevant_only_for_not_null: std::primitive::bool| {
                        if is_relevant_only_for_not_null {
                            &postgresql_type_origin_not_null_upper_camel_case
                        } else {
                            &postgresql_type_origin_not_null_or_nullable_upper_camel_case
                        }
                    },
                    &postgresql_type_not_null_or_nullable_upper_camel_case,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::False,
                    &postgresql_crud_macros_common::IsQueryBindMutable::True,
                )
            };

            let equal = postgresql_crud_macros_common::PostgresqlTypeFilter::Equal;
            let greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan;
            let between = postgresql_crud_macros_common::PostgresqlTypeFilter::Between;
            let in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::In;
            let case_sensitive_regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::CaseSensitiveRegularExpression;
            let case_insensitive_regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::CaseInsensitiveRegularExpression;
            let length_more_than = postgresql_crud_macros_common::PostgresqlTypeFilter::LengthMoreThan;
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

            let where_element_number_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &in_handle]);
            let where_element_sqlx_postgres_types_pg_money_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &in_handle]);
            let where_element_sqlx_types_decimal_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between]);
            let where_element_sqlx_types_big_decimal_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between]);
            let where_element_bool_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal]);
            let where_element_std_string_string_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
            let where_element_std_vec_vec_std_primitive_u8_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &length_more_than, &equal_to_encoded_string_representation]);
            let where_element_sqlx_types_chrono_naive_time_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_time, &greater_than_current_time]);
            let where_element_sqlx_types_time_time_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_time, &greater_than_current_time]);
            let where_element_sqlx_postgres_types_pg_interval_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal]);
            let where_element_sqlx_types_time_date_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_date, &greater_than_current_date]);
            let where_element_sqlx_types_chrono_naive_date_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_date, &greater_than_current_date]);
            let where_element_sqlx_types_chrono_naive_date_time_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_timestamp, &greater_than_current_timestamp]);
            let where_element_sqlx_types_time_primitive_date_time_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &between, &current_timestamp, &greater_than_current_timestamp]);
            let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &before, &between]);
            let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &before, &between]);
            let where_element_sqlx_types_uuid_uuid_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
            let where_element_sqlx_types_ipnetwork_ip_network_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal]);
            let where_element_sqlx_types_mac_address_mac_address_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![&equal, &greater_than, &case_sensitive_regular_expression, &case_insensitive_regular_expression]);
            let where_element_sqlx_types_bit_vec_token_stream = generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&vec![
                &equal,
                // &bit_vec_position_equal
            ]);
            let (
                where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
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
                            PostgresqlTypeRange::StdPrimitiveI32AsPostgresqlInt4 => should_impl_postgresql_type_range_length_true,
                            PostgresqlTypeRange::StdPrimitiveI64AsPostgresqlInt8 => should_impl_postgresql_type_range_length_true,
                            PostgresqlTypeRange::SqlxTypesDecimalAsPostgresqlNumeric => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesBigDecimalAsPostgresqlNumeric => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesTimeDateAsPostgresqlDate => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsPostgresqlDate => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => should_impl_postgresql_type_range_length_false,
                            PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => should_impl_postgresql_type_range_length_false,
                        }
                    };
                    generate_postgresql_type_not_null_or_nullable_where_element_token_stream(&{
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
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI32AsPostgresqlInt4),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI64AsPostgresqlInt8),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesDecimalAsPostgresqlNumeric),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesBigDecimalAsPostgresqlNumeric),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesTimeDateAsPostgresqlDate),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsPostgresqlDate),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz),
                    generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz)
                )
            };
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => where_element_number_token_stream,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => where_element_sqlx_postgres_types_pg_money_token_stream,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => where_element_sqlx_types_decimal_token_stream,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => where_element_sqlx_types_big_decimal_token_stream,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => where_element_bool_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlCharN => where_element_std_string_string_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => where_element_std_string_string_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlText => where_element_std_string_string_token_stream,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => where_element_std_vec_vec_std_primitive_u8_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => where_element_sqlx_types_chrono_naive_time_token_stream,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => where_element_sqlx_types_time_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => where_element_sqlx_postgres_types_pg_interval_token_stream,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => where_element_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => where_element_sqlx_types_chrono_naive_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => where_element_sqlx_types_chrono_naive_date_time_token_stream,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => where_element_sqlx_types_time_primitive_date_time_token_stream,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => where_element_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => where_element_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => where_element_sqlx_types_mac_address_mac_address_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => where_element_sqlx_types_bit_vec_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => where_element_sqlx_types_bit_vec_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
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
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => typical,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => typical,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => typical,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => typical,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => typical,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => default_initialized_by_postgresql,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => default_initialized_by_postgresql,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => default_initialized_by_postgresql,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => typical,
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => typical,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => typical,
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => typical,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => typical,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => typical,
                    PostgresqlType::StdStringStringAsPostgresqlText => typical,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => typical,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => typical,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => typical,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => typical,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => typical,
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => typical,
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => typical,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => typical,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => typical,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => typical,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => uuid_generate_v4_initialized_by_postgresql,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => typical,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => typical,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => typical,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => typical,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => typical,
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRange => typical,
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRange => typical,
                }
            };
            postgresql_crud_macros_common::generate_impl_postgresql_type_for_ident_token_stream(
                &postgresql_crud_macros_common_import_path_crate,
                &postgresql_type_not_null_or_nullable_upper_camel_case,
                &postgresql_type_not_null_or_nullable_table_type_declaration_upper_camel_case,
                &postgresql_type_not_null_or_nullable_create_upper_camel_case,
                &query_part_create_token_stream,
                &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                &bind_value_to_query_create_token_stream,
                &postgresql_type_not_null_or_nullable_select_upper_camel_case,
                &quote::quote! {#column_snake_case.to_string()},
                &postgresql_type_not_null_or_nullable_where_element_upper_camel_case,
                &postgresql_type_not_null_or_nullable_read_upper_camel_case,
                &postgresql_type_not_null_or_nullable_update_upper_camel_case,
                &typical_query_part_token_stream,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                &typical_query_bind_token_stream,
            )
        };
        let generated = quote::quote! {
            #postgresql_type_not_null_or_nullable_token_stream
            #postgresql_type_origin_not_null_or_nullable_token_stream
            #maybe_impl_is_empty_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #maybe_impl_try_new_for_postgresql_type_origin_not_null_token_stream
            #maybe_impl_serde_serialize_for_postgresql_type_origin_not_null_token_stream
            #maybe_impl_serde_deserialize_for_postgresql_type_origin_not_null_token_stream
            #impl_std_fmt_display_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #impl_sqlx_encode_sqlx_postgres_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #impl_sqlx_postgres_pg_has_array_type_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #maybe_impl_postgresql_type_where_filter_for_postgresql_type_origin_not_null_or_nullable_if_can_be_primary_key_token_stream
            #maybe_impl_postgresql_type_primary_key_for_postgresql_type_not_null_or_nullable_if_can_be_primary_key_token_stream
            #impl_create_table_column_query_part_for_postgresql_type_origin_not_null_or_nullable_token_stream
            #postgresql_type_not_null_or_nullable_table_type_declaration_token_stream
            #postgresql_type_not_null_or_nullable_create_token_stream
            #postgresql_type_not_null_or_nullable_select_token_stream
            #postgresql_type_not_null_or_nullable_where_element_token_stream
            #postgresql_type_not_null_or_nullable_read_token_stream
            #postgresql_type_not_null_or_nullable_update_token_stream
            #impl_postgresql_type_for_ident_token_stream
        };
        // if let (
        //     PostgresqlType::StdPrimitiveI16AsPostgresqlInt2,
        //     postgresql_crud_macros_common::PostgresqlTypeNotNullOrNullable::NotNull,
        //     PostgresqlTypePatternType::Origin
        // ) = (
        //     &postgresql_type,
        //     &postgresql_type_not_null_or_nullable,
        //     &postgresql_type_pattern_type
        // ) {
        //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //         "PostgresqlTypeTokens",
        //         &generated,
        //     );
        // }
        generated
    });
    let generated = quote::quote! {#(#postgresql_type_array)*};
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlTypeTokens",
    //     &generated,
    // );
    generated.into()
}
