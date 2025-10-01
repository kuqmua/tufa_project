//todo why no bind id read_only_ids? maybe sql injection?
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
                    }
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
                        CanBeAnArrayElement::False => {
                            panic!("{cant_support_array_version_message}{value:#?}")
                        }
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
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    panic!("{cant_support_nullable_variants_message}{value:#?}")
                                }
                            },
                            CanBeAnArrayElement::False => {
                                panic!("{cant_support_array_version_message}{value:#?}")
                            }
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
                        CanBeNullable::True => postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
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
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &postgresql_type.can_be_an_array_element() {
                        CanBeAnArrayElement::True => match &postgresql_type.can_be_nullable() {
                            CanBeNullable::True => postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                                acc.push(PostgresqlTypeRecord {
                                    postgresql_type: postgresql_type.clone(),
                                    not_null_or_nullable,
                                    postgresql_type_pattern: postgresql_type_pattern.clone(),
                                });
                            }),
                            CanBeNullable::False => {
                                if let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &dimension1_not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                                        acc.push(PostgresqlTypeRecord {
                                            postgresql_type: postgresql_type.clone(),
                                            not_null_or_nullable,
                                            postgresql_type_pattern: PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable },
                                        });
                                    });
                                }
                            }
                        },
                        CanBeAnArrayElement::False => (),
                    },
                });
                acc
            })
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlTypesConfig {
        All,
        Concrete(std::vec::Vec<PostgresqlTypeRecord>),
    }
    #[derive(Debug)]
    enum PostgresqlTypeInitializationTryNew {
        StdStringStringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
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
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Err(()),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Err(()),
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Err(()),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Err(()),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
        }
    }
    impl std::convert::From<&PostgresqlTypeInitializationTryNew> for PostgresqlType {
        fn from(value: &PostgresqlTypeInitializationTryNew) -> PostgresqlType {
            match value {
                PostgresqlTypeInitializationTryNew::StdStringStringAsText => Self::StdStringStringAsText,
                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTimeAsTime,
                PostgresqlTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTimeAsTime,
                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PostgresqlTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            }
        }
    }
    #[derive(Debug)]
    enum PostgresqlTypeImplNewForDeserialize {
        SqlxPostgresTypesPgIntervalAsInterval,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    #[derive(Debug)]
    enum PostgresqlTypeImplTryNewForDeserialize {
        StdStringStringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
    }
    #[derive(Debug)]
    enum PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize {
        NewForDeserialize(PostgresqlTypeImplNewForDeserialize),
        TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize),
    }
    #[derive(Debug)]
    enum PostgresqlTypeDeserialize {
        Derive,
        ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize),
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
                PostgresqlType::StdStringStringAsText => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText)),
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => Self::Derive,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime)),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime)),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgIntervalAsInterval)),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate)),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp)),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz)),
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => Self::Derive,
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::Derive,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Derive,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::Derive,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range)),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range)),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange)),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange)),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange)),
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

        postgresql_type_record_vec.into_iter().fold(vec![], |mut acc, postgresql_type_record_element| {
            use postgresql_crud_macros_common::NotNullOrNullable;
            #[derive(Clone)]
            struct PostgresqlTypeRecordHandle {
                not_null_or_nullable: NotNullOrNullable,
                postgresql_type_pattern: PostgresqlTypePattern,
            }
            fn generate_postgresql_type_record_handle_vec(postgresql_type_record_handle: PostgresqlTypeRecordHandle) -> std::vec::Vec<PostgresqlTypeRecordHandle> {
                let generate_vec = |current_postgresql_type_record_handle: PostgresqlTypeRecordHandle| {
                    let mut acc = vec![];
                    for element in generate_postgresql_type_record_handle_vec(current_postgresql_type_record_handle) {
                        acc.push(element);
                    }
                    acc.push(postgresql_type_record_handle.clone());
                    acc
                };
                //same pattern was in generate_postgresql_types 21.05.2025
                match (&postgresql_type_record_handle.not_null_or_nullable, &postgresql_type_record_handle.postgresql_type_pattern) {
                    (NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) => {
                        vec![postgresql_type_record_handle]
                    }
                    (NotNullOrNullable::Nullable, PostgresqlTypePattern::Standart) => generate_vec(PostgresqlTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_type_pattern: PostgresqlTypePattern::Standart,
                    }),
                    (NotNullOrNullable::NotNull, PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
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
            })
            .into_iter()
            .for_each(|postgresql_type_record_handle_element| {
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
        })
    };
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
            let start_upper_camel_case = naming::StartUpperCamelCase;
            let end_upper_camel_case = naming::EndUpperCamelCase;
            let start_snake_case = naming::StartSnakeCase;
            let end_snake_case = naming::EndSnakeCase;
            let months_snake_case = naming::MonthsSnakeCase;
            let days_snake_case = naming::DaysSnakeCase;
            let microseconds_snake_case = naming::MicrosecondsSnakeCase;
            let as_upper_camel_case = naming::AsUpperCamelCase;
            let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
            let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
            let acc_snake_case = naming::AccSnakeCase;
            let option_update_snake_case = naming::OptionUpdateSnakeCase;
            let read_snake_case = naming::ReadSnakeCase;
            let hour_snake_case = naming::HourSnakeCase;
            let min_snake_case = naming::MinSnakeCase;
            let sec_snake_case = naming::SecSnakeCase;
            let micro_snake_case = naming::MicroSnakeCase;
            let minute_snake_case = naming::MinuteSnakeCase;
            let second_snake_case = naming::SecondSnakeCase;
            let microsecond_snake_case = naming::MicrosecondSnakeCase;
            let error_snake_case = naming::ErrorSnakeCase;
            let date_snake_case = naming::DateSnakeCase;
            let date_naive_snake_case = naming::DateNaiveSnakeCase;
            let time_snake_case = naming::TimeSnakeCase;
            let nanosecond_snake_case = naming::NanosecondSnakeCase;
            let included_upper_camel_case = naming::IncludedUpperCamelCase;
            let excluded_upper_camel_case = naming::ExcludedUpperCamelCase;
            let unbounded_upper_camel_case = naming::UnboundedUpperCamelCase;
            let new_snake_case = naming::NewSnakeCase;
            let try_new_snake_case = naming::TryNewSnakeCase;
            let try_new_for_deserialize_snake_case = naming::TryNewForDeserializeSnakeCase;
            let self_upper_camel_case = naming::SelfUpperCamelCase;
            let element_upper_camel_case = naming::ElementUpperCamelCase;
            let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
            let read_inner_vec_vec_snake_case = naming::ReadInnerVecVecSnakeCase;
            let create_vec_snake_case = naming::CreateVecSnakeCase;
            let create_snake_case = naming::CreateSnakeCase;

            let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
            let std_primitive_u32_token_stream = token_patterns::StdPrimitiveU32;
            let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
            let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
            let std_string_string_token_stream = token_patterns::StdStringString;

            let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
            let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

            let postgresql_crud_macros_common_import_path_postgresql_crud_common = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;

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
                    }
                };
                format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{not_null_or_nullable}{postgresql_part}")
            };
            let generate_ident_token_stream = |postgresql_type: &PostgresqlType, not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, postgresql_type_pattern: &PostgresqlTypePattern| generate_ident_stringified(postgresql_type, not_null_or_nullable, postgresql_type_pattern).parse::<proc_macro2::TokenStream>().unwrap();
            let ident = &generate_ident_token_stream(postgresql_type, not_null_or_nullable, postgresql_type_pattern);
            let generate_ident_standart_not_null_token_stream = |postgresql_type: &PostgresqlType| generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlTypePattern::Standart);
            let ident_standart_not_null_upper_camel_case = generate_ident_standart_not_null_token_stream(postgresql_type);
            let ident_token_stream = {
                quote::quote! {
                    #[derive(Debug)]
                    pub struct #ident;
                }
            };
            let generate_ident_standart_not_null_origin_token_stream = |postgresql_type: &PostgresqlType| naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_standart_not_null_token_stream(postgresql_type));
            let ident_standart_not_null_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(postgresql_type);
            let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
            let ident_standart_not_null_or_nullable_upper_camel_case = generate_ident_token_stream(postgresql_type, not_null_or_nullable, &PostgresqlTypePattern::Standart);

            let sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateAsDate);
            let sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoNaiveTimeAsTime);
            let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);

            let generate_ident_standart_not_null_origin_try_new_error_named_token_stream = |postgresql_type: &PostgresqlType| naming::parameter::SelfOriginTryNewErrorNamedUpperCamelCase::from_tokens(&generate_ident_standart_not_null_token_stream(postgresql_type));
            let sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateAsDate);
            let sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoNaiveTimeAsTime);
            let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
            let inner_type_standart_not_null_token_stream = {
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
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified = "sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>".to_string();
                    let sqlx_types_uuid_uuid_stringified = "sqlx::types::uuid::Uuid".to_string();
                    let sqlx_types_ipnetwork_ip_network_stringified = "sqlx::types::ipnetwork::IpNetwork".to_string();
                    let sqlx_types_mac_address_mac_address_stringified = "sqlx::types::mac_address::MacAddress".to_string();
                    match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => std_primitive_i16_stringified,
                        PostgresqlType::StdPrimitiveI32AsInt4 => std_primitive_i32_stringified,
                        PostgresqlType::StdPrimitiveI64AsInt8 => std_primitive_i64_stringified,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => std_primitive_f32_stringified,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => std_primitive_f64_stringified,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => std_primitive_i16_stringified,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => std_primitive_i32_stringified,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => std_primitive_i64_stringified,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => sqlx_postgres_types_pg_money_stringified,
                        PostgresqlType::StdPrimitiveBoolAsBool => std_primitive_bool_stringified,
                        PostgresqlType::StdStringStringAsText => std_string_string_stringified,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => std_vec_vec_std_primitive_u8_stringified,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => sqlx_types_chrono_naive_time_stringified,
                        PostgresqlType::SqlxTypesTimeTimeAsTime => sqlx_types_time_time_stringified,
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => sqlx_postgres_types_pg_interval_stringified,
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => sqlx_types_chrono_naive_date_stringified,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => sqlx_types_chrono_naive_date_time_stringified,
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => sqlx_types_uuid_uuid_stringified,
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => sqlx_types_uuid_uuid_stringified,
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => sqlx_types_ipnetwork_ip_network_stringified,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => sqlx_types_mac_address_mac_address_stringified,
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i32_stringified),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i64_stringified),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_stringified),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_time_stringified),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified),
                    }
                };
                value.parse::<proc_macro2::TokenStream>().unwrap()
            };
            let generate_current_ident_origin_non_wrapping = |current_postgresql_type_pattern: &PostgresqlTypePattern, current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable| naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(postgresql_type, current_not_null_or_nullable, current_postgresql_type_pattern));
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
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &inner_type_standart_not_null_token_stream,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                    },
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                        let (current_postgresql_type_pattern, current_not_null_or_nullable): (&PostgresqlTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&PostgresqlTypePattern::Standart, dimension1_not_null_or_nullable),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin(current_postgresql_type_pattern, current_not_null_or_nullable)
                    },
                }
            };
            let generate_typical_query_bind_token_stream = |content_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                    if let Err(error) = #query_snake_case.try_bind(#content_token_stream) {
                        return Err(error.to_string());
                    }
                    Ok(#query_snake_case)
                },
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                    let value = match #content_token_stream.0 {
                        Some(#value_snake_case) => Some(#value_snake_case),
                        None => None
                    };
                    if let Err(error) = #query_snake_case.try_bind(value) {
                        return Err(error.to_string());
                    }
                    Ok(#query_snake_case)
                },
            };
            let typical_query_bind_token_stream = generate_typical_query_bind_token_stream(&value_snake_case);

            let generate_double_dot_space_tokens_token_stream = |value: &dyn quote::ToTokens| {
                quote::quote! {: #value}
            };
            let generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream = |months_token_stream: &dyn quote::ToTokens, days_token_stream: &dyn quote::ToTokens, microseconds_token_stream: &dyn quote::ToTokens| {
                quote::quote! {#inner_type_standart_not_null_token_stream {
                    #months_snake_case #months_token_stream,
                    #days_snake_case #days_token_stream,
                    #microseconds_snake_case #microseconds_token_stream
                }}
            };
            let ident_inner_type_token_stream = match &element.postgresql_type_pattern {
                PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &inner_type_standart_not_null_token_stream,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&inner_type_standart_not_null_token_stream),
                },
                PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {#inner_type_standart_not_null_token_stream});
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
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
                quote::quote! {
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
                False,
            }
            let is_not_null_standart_can_be_primary_key = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart, CanBePrimaryKey::True) = (&not_null_or_nullable, &postgresql_type_pattern, &can_be_primary_key) {
                IsNotNullStandartCanBePrimaryKey::True
            } else {
                IsNotNullStandartCanBePrimaryKey::False
            };
            enum StartOrEnd {
                Start,
                End,
            }
            let generate_start_or_end_upper_camel_case = |start_or_end: &StartOrEnd| -> &dyn naming::StdFmtDisplayPlusQuoteToTokens {
                match &start_or_end {
                    StartOrEnd::Start => &start_upper_camel_case,
                    StartOrEnd::End => &end_upper_camel_case,
                }
            };
            let generate_start_or_end_snake_case = |start_or_end: &StartOrEnd| -> &dyn naming::StdFmtDisplayPlusQuoteToTokens {
                match &start_or_end {
                    StartOrEnd::Start => &start_snake_case,
                    StartOrEnd::End => &end_snake_case,
                }
            };
            let generate_tokens_as_crate_postgresql_type_token_stream = |ident_token_stream: &dyn quote::ToTokens| {
                quote::quote! {<#ident_token_stream as postgresql_crud_common::PostgresqlType>}
            };
            let (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl) = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&not_null_or_nullable, &postgresql_type_pattern) {
                let self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};
                enum ParameterNumber {
                    One,
                    Two,
                    Three,
                    Four,
                }
                impl ParameterNumber {
                    fn get_index(&self) -> std::primitive::usize {
                        match &self {
                            Self::One => 0,
                            Self::Two => 1,
                            Self::Three => 2,
                            Self::Four => 3,
                        }
                    }
                    fn get_vec_from_index_starting_with_zero(&self) -> std::vec::Vec<std::primitive::usize> {
                        (0..=self.get_index()).collect()
                    }
                }
                let parameter_number_one = ParameterNumber::One;
                let parameter_number_two = ParameterNumber::Two;
                let parameter_number_three = ParameterNumber::Three;
                let parameter_number_four = ParameterNumber::Four;
                let ident_standart_not_null_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_standart_not_null_upper_camel_case);
                let ident_standart_not_null_origin_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_standart_not_null_origin_upper_camel_case);
                let generate_std_ops_bound_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {std::ops::Bound<#type_token_stream>}
                };
                let std_ops_bound_std_primitive_i32_token_stream = generate_std_ops_bound_token_stream(&std_primitive_i32_token_stream);
                let std_ops_bound_std_primitive_i64_token_stream = generate_std_ops_bound_token_stream(&std_primitive_i64_token_stream);
                let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream = generate_std_ops_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case);
                let std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream = generate_std_ops_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case);
                let std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream = generate_std_ops_bound_token_stream(&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case);
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
                    let generate_serde_state_initialization_token_stream = |parameter_number: &ParameterNumber| {
                        let parameter_number_token_stream = {
                            let value = parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(|_| quote::quote! {+ 1});
                            quote::quote! {#(#value)*}
                        };
                        quote::quote! {
                            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_standart_not_null_origin_double_quotes_token_stream, false as std::primitive::usize #parameter_number_token_stream)?;
                        }
                    };
                    let serde_state_initialization_two_fields_token_stream = generate_serde_state_initialization_token_stream(&parameter_number_two);
                    let serde_state_initialization_three_fields_token_stream = generate_serde_state_initialization_token_stream(&parameter_number_three);
                    let serde_state_initialization_four_fields_token_stream = generate_serde_state_initialization_token_stream(&parameter_number_four);
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
                    let impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
                    let impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.to_string()}));

                    let generate_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens = |ident_token_stream: &dyn quote::ToTokens| {
                        let generate_serialize_field_match_std_ops_bound_token_stream = |start_or_end: &StartOrEnd| {
                            let start_or_end_token_stream = generate_start_or_end_snake_case(start_or_end);
                            generate_serialize_field_token_stream(
                                &start_or_end_token_stream,
                                &quote::quote! {
                                    &match self.0.#start_or_end_token_stream {
                                        std::ops::Bound::Included(#value_snake_case) => std::ops::Bound::Included(#ident_token_stream::#try_new_snake_case(#value_snake_case).unwrap()),
                                        std::ops::Bound::Excluded(#value_snake_case) => std::ops::Bound::Excluded(#ident_token_stream::#try_new_snake_case(#value_snake_case).unwrap()),
                                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                    }
                                },
                            )
                        };
                        let start_serialize_field_token_stream = generate_serialize_field_match_std_ops_bound_token_stream(&StartOrEnd::Start);
                        let end_serialize_field_token_stream = generate_serialize_field_match_std_ops_bound_token_stream(&StartOrEnd::End);
                        generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&quote::quote! {
                            #serde_state_initialization_two_fields_token_stream
                            #start_serialize_field_token_stream
                            #end_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
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
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.0}))),
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            let generate_field_inner_type_standart_not_null_token_stream_as_chrono_timelike_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                                quote::quote! {&(<#inner_type_standart_not_null_token_stream as chrono::Timelike>::#content_token_stream)}
                            };
                            let hour_serialize_field_token_stream = generate_serialize_field_token_stream(&hour_snake_case, &generate_field_inner_type_standart_not_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {hour(&self.0)}));
                            let min_serialize_field_token_stream = generate_serialize_field_token_stream(&min_snake_case, &generate_field_inner_type_standart_not_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {minute(&self.0)}));
                            let sec_serialize_field_token_stream = generate_serialize_field_token_stream(&sec_snake_case, &generate_field_inner_type_standart_not_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {second(&self.0)}));
                            let micro_serialize_field_token_stream = generate_serialize_field_token_stream(
                                &micro_snake_case,
                                &generate_field_inner_type_standart_not_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {
                                    #nanosecond_snake_case(&self.0) / 1000
                                }),
                            );
                            quote::quote! {
                                #serde_state_initialization_four_fields_token_stream
                                #hour_serialize_field_token_stream
                                #min_serialize_field_token_stream
                                #sec_serialize_field_token_stream
                                #micro_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            let generate_serialize_field_self_zero_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&self.0.#value()});
                            let hour_serialize_field_token_stream = generate_serialize_field_self_zero_token_stream(&hour_snake_case);
                            let minute_serialize_field_token_stream = generate_serialize_field_self_zero_token_stream(&minute_snake_case);
                            let second_serialize_field_token_stream = generate_serialize_field_self_zero_token_stream(&second_snake_case);
                            let microsecond_serialize_field_token_stream = generate_serialize_field_self_zero_token_stream(&microsecond_snake_case);
                            quote::quote! {
                                #serde_state_initialization_four_fields_token_stream
                                #hour_serialize_field_token_stream
                                #minute_serialize_field_token_stream
                                #second_serialize_field_token_stream
                                #microsecond_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
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
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            enum DateOrTime {
                                Date,
                                Time,
                            }
                            let generate_serialize_field_try_new_unwrap_token_stream = |date_or_time: &DateOrTime| {
                                let date_or_time_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &date_or_time {
                                    DateOrTime::Date => &date_snake_case,
                                    DateOrTime::Time => &time_snake_case,
                                };
                                generate_serialize_field_token_stream(&date_or_time_token_stream, &{
                                    let ident_token_stream: &dyn quote::ToTokens = match &date_or_time {
                                        DateOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case,
                                        DateOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case,
                                    };
                                    quote::quote! {&#ident_token_stream::#try_new_snake_case(self.0.#date_or_time_token_stream()).unwrap()}
                                })
                            };
                            let date_serialize_field_token_stream = generate_serialize_field_try_new_unwrap_token_stream(&DateOrTime::Date);
                            let time_serialize_field_token_stream = generate_serialize_field_try_new_unwrap_token_stream(&DateOrTime::Time);
                            quote::quote! {
                                #serde_state_initialization_two_fields_token_stream
                                #date_serialize_field_token_stream
                                #time_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                            enum DateNaiveOrTime {
                                Date,
                                Time,
                            }
                            let generate_serialize_field_try_new_unwrap_token_stream = |date_naive_or_time: &DateNaiveOrTime| {
                                let date_naive_or_time_token_stream: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &date_naive_snake_case,
                                    DateNaiveOrTime::Time => &time_snake_case,
                                };
                                generate_serialize_field_token_stream(&date_naive_or_time_token_stream, &{
                                    let ident_token_stream: &dyn quote::ToTokens = match &date_naive_or_time {
                                        DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case,
                                        DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case,
                                    };
                                    quote::quote! {&#ident_token_stream::#try_new_snake_case(self.0.#date_naive_or_time_token_stream()).unwrap()}
                                })
                            };
                            let date_naive_serialize_field_token_stream = generate_serialize_field_try_new_unwrap_token_stream(&DateNaiveOrTime::Date);
                            let time_serialize_field_token_stream = generate_serialize_field_try_new_unwrap_token_stream(&DateNaiveOrTime::Time);
                            quote::quote! {
                                #serde_state_initialization_two_fields_token_stream
                                #date_naive_serialize_field_token_stream
                                #time_serialize_field_token_stream
                                #serde_ser_serialize_struct_end_token_stream
                            }
                        })),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.bytes()}))),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case)),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case)),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case)),
                    }
                };
                let serde_deserialize_derive_or_impl = {
                    let struct_ident_double_quotes_token_stream = postgresql_crud_macros_common::generate_struct_ident_double_quotes_token_stream(&ident_origin_upper_camel_case);
                    let tuple_struct_ident_double_quotes_token_stream = postgresql_crud_macros_common::generate_tuple_struct_ident_double_quotes_token_stream(&ident_origin_upper_camel_case);
                    let struct_visitor_token_stream = quote::quote! {
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: serde::__private::PhantomData<#ident_standart_not_null_origin_upper_camel_case>,
                            lifetime: serde::__private::PhantomData<&'de ()>,
                        }
                    };
                    let start_end_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&start_snake_case, &end_snake_case];
                    let hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 4] = [&hour_snake_case, &min_snake_case, &sec_snake_case, &micro_snake_case];
                    let hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 4] = [&hour_snake_case, &minute_snake_case, &second_snake_case, &microsecond_snake_case];
                    let date_time_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&date_snake_case, &time_snake_case];
                    let date_naive_time_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&date_naive_snake_case, &time_snake_case];
                    let months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&months_snake_case, &days_snake_case, &microseconds_snake_case];
                    let serde_deserializer_deserialize_struct_visitor_token_stream = {
                        quote::quote! {
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #ident_standart_not_null_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<#ident_standart_not_null_origin_upper_camel_case>,
                                    lifetime: _serde::__private::PhantomData,
                                }
                            )
                        }
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
                    let generate_field_index_token_stream = |index: std::primitive::usize| format!("__{}{index}", naming::FieldSnakeCase).parse::<proc_macro2::TokenStream>().unwrap();
                    let (enum_field_two_token_stream, enum_field_three_token_stream, enum_field_four_token_stream) = {
                        let generate_enum_field_token_stream = |parameter_number: &ParameterNumber| {
                            let fields_token_stream = {
                                let fields_token_stream = parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(&generate_field_index_token_stream);
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
                        (generate_enum_field_token_stream(&parameter_number_two), generate_enum_field_token_stream(&parameter_number_three), generate_enum_field_token_stream(&parameter_number_four))
                    };
                    let (fn_expecting_struct_ident_double_quotes_token_stream, fn_expecting_tuple_struct_ident_double_quotes_token_stream, fn_expecting_field_identifier_token_stream) = {
                        let generate_fn_expecting_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                                    serde::__private::Formatter::write_str(__f, #content_token_stream)
                                }
                            }
                        };
                        (generate_fn_expecting_token_stream(&struct_ident_double_quotes_token_stream), generate_fn_expecting_token_stream(&tuple_struct_ident_double_quotes_token_stream), generate_fn_expecting_token_stream(&quote::quote! {"field identifier"}))
                    };
                    let field_0_token_stream = generate_field_index_token_stream(parameter_number_one.get_index());
                    let generate_serde_private_ok_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {serde::__private::Ok(#content_token_stream)}
                    };
                    let generate_serde_private_ok_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_serde_private_ok_token_stream(&quote::quote! {#ident_standart_not_null_origin_upper_camel_case(#content_token_stream)});
                    let match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream = quote::quote! {match #inner_type_standart_not_null_token_stream::try_parse(&#field_0_token_stream) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }};
                    let sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream = quote::quote! {#inner_type_standart_not_null_token_stream::#new_snake_case(#field_0_token_stream)};
                    let array_std_primitive_u8_6_token_stream = quote::quote! {[std::primitive::u8; 6]};
                    let (sqlx_types_chrono_naive_time_origin_try_new_for_deserialize, match_origin_try_new_for_deserialize_one_token_stream, match_origin_try_new_for_deserialize_two_token_stream, match_origin_try_new_for_deserialize_four_token_stream) = {
                        let generate_match_origin_try_new_for_deserialize_token_stream = |length: std::primitive::usize| {
                            let fields_token_stream = (1..=length).collect::<std::vec::Vec<_>>().into_iter().enumerate().map(|(index, _)| generate_field_index_token_stream(index));
                            quote::quote! {
                                match #ident_standart_not_null_origin_upper_camel_case::#try_new_for_deserialize_snake_case(#(#fields_token_stream),*) {
                                    Ok(#value_snake_case) => _serde::__private::Ok(#value_snake_case),
                                    Err(#error_snake_case) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                                }
                            }
                        };
                        (
                            generate_match_origin_try_new_for_deserialize_token_stream(hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array.len()),
                            generate_match_origin_try_new_for_deserialize_token_stream(1),
                            generate_match_origin_try_new_for_deserialize_token_stream(2),
                            generate_match_origin_try_new_for_deserialize_token_stream(4),
                        )
                    };
                    let (origin_new_for_deserialize_two_token_stream, origin_new_for_deserialize_three_token_stream) = {
                        let generate_origin_new_for_deserialize_token_stream = |length: std::primitive::usize| {
                            let fields_token_stream = (1..=length).collect::<std::vec::Vec<_>>().into_iter().enumerate().map(|(index, _)| generate_field_index_token_stream(index));
                            quote::quote! {
                                _serde::__private::Ok(#ident_standart_not_null_origin_upper_camel_case::new_for_deserialize(#(#fields_token_stream),*))
                            }
                        };
                        (generate_origin_new_for_deserialize_token_stream(2), generate_origin_new_for_deserialize_token_stream(3))
                    };
                    let (fn_visit_newtype_struct_pg_money_token_stream, fn_visit_newtype_struct_uuid_token_stream, fn_visit_newtype_struct_mac_address_token_stream, fn_visit_newtype_struct_text_token_stream, fn_visit_newtype_struct_sqlx_types_chrono_naive_date_token_stream) = {
                        let generate_fn_visit_newtype_struct_token_stream = |type_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                #[inline]
                                fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                                where
                                    __E: serde::Deserializer<'de>,
                                {
                                    let #field_0_token_stream = <#type_token_stream as serde::Deserialize>::deserialize(__e)?;
                                    #serde_private_ok_token_stream
                                }
                            }
                        };
                        (
                            generate_fn_visit_newtype_struct_token_stream(&std_primitive_i64_token_stream, &generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#inner_type_standart_not_null_token_stream(#field_0_token_stream)})),
                            generate_fn_visit_newtype_struct_token_stream(&std_string_string_token_stream, &generate_serde_private_ok_postgresql_type_token_stream(&match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream)),
                            generate_fn_visit_newtype_struct_token_stream(&array_std_primitive_u8_6_token_stream, &generate_serde_private_ok_postgresql_type_token_stream(&sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream)),
                            generate_fn_visit_newtype_struct_token_stream(&std_string_string_token_stream, &match_origin_try_new_for_deserialize_one_token_stream),
                            generate_fn_visit_newtype_struct_token_stream(&inner_type_standart_not_null_token_stream, &match_origin_try_new_for_deserialize_one_token_stream),
                        )
                    };
                    let generate_fields_serde_de_seq_access_next_element_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]| {
                        let error_message_token_stream = postgresql_crud_macros_common::generate_struct_ident_with_number_elements_double_quotes_token_stream(&ident_standart_not_null_origin_upper_camel_case, vec_token_stream.len());
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
                    let (
                        fn_visit_seq_pg_money_token_stream,
                        fn_visit_seq_sqlx_types_chrono_naive_time_token_stream,
                        fn_visit_seq_sqlx_types_uuid_uuid_token_stream,
                        fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream,
                        fn_visit_seq_std_string_string_token_stream,
                        fn_visit_seq_sqlx_types_time_time_token_stream,
                        fn_visit_seq_sqlx_types_chrono_naive_date_token_stream,
                        fn_visit_seq_sqlx_types_chrono_naive_date_time_token_stream,
                        fn_visit_seq_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                        fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                        fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream,
                    ) = {
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
                        (
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i64_token_stream]);
                                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote! {#inner_type_standart_not_null_token_stream(#field_0_token_stream)});
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #serde_private_ok_postgresql_type_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_u32_token_stream, &std_primitive_u32_token_stream, &std_primitive_u32_token_stream, &std_primitive_u32_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #sqlx_types_chrono_naive_time_origin_try_new_for_deserialize
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_string_string_token_stream]);
                                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #serde_private_ok_postgresql_type_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&array_std_primitive_u8_6_token_stream]);
                                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #serde_private_ok_postgresql_type_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_string_string_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #match_origin_try_new_for_deserialize_one_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_u8_token_stream, &std_primitive_u8_token_stream, &std_primitive_u8_token_stream, &std_primitive_u32_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #match_origin_try_new_for_deserialize_four_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&inner_type_standart_not_null_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #match_origin_try_new_for_deserialize_one_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case, &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #origin_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case, &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #origin_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #origin_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #origin_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #origin_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_ops_bound_std_primitive_i32_token_stream, &std_ops_bound_std_primitive_i32_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #match_origin_try_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_ops_bound_std_primitive_i64_token_stream, &std_ops_bound_std_primitive_i64_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #match_origin_try_new_for_deserialize_two_token_stream
                                }
                            }),
                            generate_fn_visit_seq_token_stream(&{
                                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[&std_primitive_i32_token_stream, &std_primitive_i32_token_stream, &std_primitive_i64_token_stream]);
                                quote::quote! {
                                    #fields_initialization_token_stream
                                    #origin_new_for_deserialize_three_token_stream
                                }
                            }),
                        )
                    };
                    let (fn_visit_u64_two_token_stream, fn_visit_u64_three_token_stream, fn_visit_u64_four_token_stream) = {
                        let generate_fn_visit_u64_token_stream = |parameter_number: &ParameterNumber| {
                            let fields_token_stream = {
                                parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(|element| {
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
                        (generate_fn_visit_u64_token_stream(&parameter_number_two), generate_fn_visit_u64_token_stream(&parameter_number_three), generate_fn_visit_u64_token_stream(&parameter_number_four))
                    };
                    let (fn_visit_str_value_start_end_token_stream, fn_visit_str_value_hour_min_sec_micro_token_stream, fn_visit_str_value_hour_minute_second_microsecond_token_stream, fn_visit_str_value_date_time_token_stream, fn_visit_str_value_date_naive_time_token_stream, fn_visit_str_value_months_days_microseconds_token_stream) = {
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
                            generate_fn_visit_str_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_str_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };
                    let (fn_visit_bytes_start_end_token_stream, fn_visit_bytes_hour_min_sec_micro_token_stream, fn_visit_bytes_hour_minute_second_microsecond_token_stream, fn_visit_bytes_date_time_token_stream, fn_visit_bytes_date_naive_time_token_stream, fn_visit_bytes_months_days_microseconds_token_stream) = {
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
                            generate_fn_visit_bytes_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                            generate_fn_visit_bytes_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
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
                        fn_visit_map_sqlx_types_chrono_naive_time_token_stream,
                        fn_visit_map_sqlx_types_time_time_token_stream,
                        fn_visit_map_sqlx_types_chrono_naive_date_time_token_stream,
                        fn_visit_map_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                        fn_visit_map_sqlx_postgres_types_pg_interval_token_stream,
                    ) = {
                        let generate_fn_visit_map_token_stream = |field_option_none_initialization_token_stream: &dyn quote::ToTokens, while_some_next_key_field_token_stream: &dyn quote::ToTokens, match_field_initialization_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
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
                            field_option_none_initialization_sqlx_types_chrono_naive_time_token_stream,
                            field_option_none_initialization_sqlx_types_time_time_token_stream,
                            field_option_none_initialization_sqlx_types_chrono_naive_date_time_token_stream,
                            field_option_none_initialization_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                            field_option_none_initialization_sqlx_postgres_types_pg_interval_token_stream,
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
                                generate_field_option_none_initialization_token_stream(&[&std_primitive_u32_token_stream, &std_primitive_u32_token_stream, &std_primitive_u32_token_stream, &std_primitive_u32_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_primitive_u8_token_stream, &std_primitive_u8_token_stream, &std_primitive_u8_token_stream, &std_primitive_u32_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case, &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case]),
                                generate_field_option_none_initialization_token_stream(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case, &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case]),
                                generate_field_option_none_initialization_token_stream(&[&std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_ops_bound_std_primitive_i32_token_stream, &std_ops_bound_std_primitive_i32_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_ops_bound_std_primitive_i64_token_stream, &std_ops_bound_std_primitive_i64_token_stream]),
                                generate_field_option_none_initialization_token_stream(&[&std_primitive_i32_token_stream, &std_primitive_i32_token_stream, &std_primitive_i64_token_stream]),
                            )
                        };
                        let (
                            while_some_next_key_field_sqlx_types_chrono_naive_time_token_stream,
                            while_some_next_key_field_sqlx_types_time_time_token_stream,
                            while_some_next_key_field_sqlx_types_chrono_naive_date_time_token_stream,
                            while_some_next_key_field_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                            while_some_next_key_field_sqlx_postgres_types_pg_interval_token_stream,
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
                                generate_while_some_next_key_field_token_stream(&[(&hour_snake_case, &std_primitive_u32_token_stream), (&min_snake_case, &std_primitive_u32_token_stream), (&sec_snake_case, &std_primitive_u32_token_stream), (&micro_snake_case, &std_primitive_u32_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&hour_snake_case, &std_primitive_u8_token_stream), (&minute_snake_case, &std_primitive_u8_token_stream), (&second_snake_case, &std_primitive_u8_token_stream), (&microsecond_snake_case, &std_primitive_u32_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&date_snake_case, &sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case), (&time_snake_case, &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case)]),
                                generate_while_some_next_key_field_token_stream(&[(&date_naive_snake_case, &sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case), (&time_snake_case, &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case)]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream), (&end_snake_case, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream), (&end_snake_case, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[
                                    (&start_snake_case, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream),
                                    (&end_snake_case, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_token_stream),
                                ]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_ops_bound_std_primitive_i32_token_stream), (&end_snake_case, &std_ops_bound_std_primitive_i32_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&start_snake_case, &std_ops_bound_std_primitive_i64_token_stream), (&end_snake_case, &std_ops_bound_std_primitive_i64_token_stream)]),
                                generate_while_some_next_key_field_token_stream(&[(&months_snake_case, &std_primitive_i32_token_stream), (&days_snake_case, &std_primitive_i32_token_stream), (&microseconds_snake_case, &std_primitive_i64_token_stream)]),
                            )
                        };
                        let (match_field_initialization_hour_min_sec_micro_token_stream, match_field_initialization_start_end_token_stream, match_field_initialization_hour_minute_second_microsecond_token_stream, match_field_initialization_date_time_token_stream, match_field_initialization_date_naive_time_token_stream, match_field_initialization_months_days_microseconds_token_stream) = {
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
                                generate_match_field_initialization_token_stream(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                                generate_match_field_initialization_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                            )
                        };
                        (
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_types_chrono_naive_time_token_stream,
                                &while_some_next_key_field_sqlx_types_chrono_naive_time_token_stream,
                                &match_field_initialization_hour_min_sec_micro_token_stream,
                                &sqlx_types_chrono_naive_time_origin_try_new_for_deserialize,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_types_time_time_token_stream,
                                &while_some_next_key_field_sqlx_types_time_time_token_stream,
                                &match_field_initialization_hour_minute_second_microsecond_token_stream,
                                &match_origin_try_new_for_deserialize_four_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_types_chrono_naive_date_time_token_stream,
                                &while_some_next_key_field_sqlx_types_chrono_naive_date_time_token_stream,
                                &match_field_initialization_date_time_token_stream,
                                &origin_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &while_some_next_key_field_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &match_field_initialization_date_naive_time_token_stream,
                                &origin_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &origin_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &origin_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &origin_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &match_origin_try_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                &match_field_initialization_start_end_token_stream,
                                &match_origin_try_new_for_deserialize_two_token_stream,
                            ),
                            generate_fn_visit_map_token_stream(
                                &field_option_none_initialization_sqlx_postgres_types_pg_interval_token_stream,
                                &while_some_next_key_field_sqlx_postgres_types_pg_interval_token_stream,
                                &match_field_initialization_months_days_microseconds_token_stream,
                                &origin_new_for_deserialize_three_token_stream,
                            ),
                        )
                    };
                    let (const_fields_start_end_token_stream, const_fields_sqlx_types_chrono_naive_time_token_stream, const_fields_sqlx_types_time_time_token_stream, const_fields_sqlx_types_chrono_naive_date_time_token_stream, const_fields_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, const_fields_sqlx_postgres_types_pg_interval_token_stream) = {
                        let generate_const_fields_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                            let field_names_token_stream = vec_token_stream.iter().map(|element| generate_quotes::double_quotes_token_stream(&element));
                            quote::quote! {
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[#(#field_names_token_stream),*];
                            }
                        };
                        (
                            generate_const_fields_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                            generate_const_fields_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
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
                        impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_time_token_stream,
                        impl_serde_de_visitor_for_visitor_pg_money_token_stream,
                        impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream,
                        impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream,
                        impl_serde_de_visitor_for_visitor_std_string_string_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_types_time_time_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_time_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                        impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_interval_token_stream,
                    ) = {
                        let generate_impl_serde_de_visitor_for_visitor_token_stream = |zero_token_stream: &dyn quote::ToTokens, first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                            generate_impl_serde_de_visitor_for_tokens_token_stream(
                                &quote::quote! {__Visitor<'de>},
                                &quote::quote! {
                                    type Value = #ident_standart_not_null_origin_upper_camel_case;
                                    #zero_token_stream
                                    #first_token_stream
                                    #second_token_stream
                                },
                            )
                        };
                        (
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_types_chrono_naive_time_token_stream, &fn_visit_map_sqlx_types_chrono_naive_time_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_tuple_struct_ident_double_quotes_token_stream, &fn_visit_newtype_struct_pg_money_token_stream, &fn_visit_seq_pg_money_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_tuple_struct_ident_double_quotes_token_stream, &fn_visit_newtype_struct_uuid_token_stream, &fn_visit_seq_sqlx_types_uuid_uuid_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_tuple_struct_ident_double_quotes_token_stream, &fn_visit_newtype_struct_mac_address_token_stream, &fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_tuple_struct_ident_double_quotes_token_stream, &fn_visit_newtype_struct_text_token_stream, &fn_visit_seq_std_string_string_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_types_time_time_token_stream, &fn_visit_map_sqlx_types_time_time_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_tuple_struct_ident_double_quotes_token_stream, &fn_visit_newtype_struct_sqlx_types_chrono_naive_date_token_stream, &fn_visit_seq_sqlx_types_chrono_naive_date_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_types_chrono_naive_date_time_token_stream, &fn_visit_map_sqlx_types_chrono_naive_date_time_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream, &fn_visit_map_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(
                                &fn_expecting_struct_ident_double_quotes_token_stream,
                                &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                                &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            ),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream),
                            generate_impl_serde_de_visitor_for_visitor_token_stream(&fn_expecting_struct_ident_double_quotes_token_stream, &fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream, &fn_visit_map_sqlx_postgres_types_pg_interval_token_stream),
                        )
                    };
                    let field_visitor_token_stream = quote::quote! {__FieldVisitor};
                    let struct_field_visitor_token_stream = quote::quote! {
                        #[doc(hidden)]
                        struct #field_visitor_token_stream;
                    };
                    let type_value_equal_underscore_field_semicolon_token_stream = quote::quote! {type Value = __Field;};
                    let (
                        impl_serde_de_visitor_for_field_visitor_token_stream_5a4f24ce_7a8e_4bcc_8f79_2494f79bcc08,
                        impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6,
                        impl_serde_de_visitor_for_field_visitor_token_stream_9b240c3e_a4af_4da1_a2ab_f1bab44b1df6,
                        impl_serde_de_visitor_for_field_visitor_token_stream_dc439ca1_8af1_4c4c_ab49_4e4fb15a41d3,
                        impl_serde_de_visitor_for_field_visitor_token_stream_8c733fe0_c816_4a0e_bb13_4c2d0cd2ded6,
                        impl_serde_de_visitor_for_field_visitor_token_stream_f702a411_b02b_4c90_aa7f_962a698612e7,
                    ) = {
                        let generate_impl_serde_de_visitor_for_field_visitor_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            let impl_serde_de_visitor_for_tokens_token_stream = generate_impl_serde_de_visitor_for_tokens_token_stream(&field_visitor_token_stream, &content_token_stream);
                            quote::quote! {
                                #struct_field_visitor_token_stream
                                #impl_serde_de_visitor_for_tokens_token_stream
                            }
                        };
                        (
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                #type_value_equal_underscore_field_semicolon_token_stream
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_four_token_stream
                                #fn_visit_str_value_hour_min_sec_micro_token_stream
                                #fn_visit_bytes_hour_min_sec_micro_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                #type_value_equal_underscore_field_semicolon_token_stream
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_two_token_stream
                                #fn_visit_str_value_start_end_token_stream
                                #fn_visit_bytes_start_end_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                #type_value_equal_underscore_field_semicolon_token_stream
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_four_token_stream
                                #fn_visit_str_value_hour_minute_second_microsecond_token_stream
                                #fn_visit_bytes_hour_minute_second_microsecond_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                #type_value_equal_underscore_field_semicolon_token_stream
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_two_token_stream
                                #fn_visit_str_value_date_time_token_stream
                                #fn_visit_bytes_date_time_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                #type_value_equal_underscore_field_semicolon_token_stream
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_two_token_stream
                                #fn_visit_str_value_date_naive_time_token_stream
                                #fn_visit_bytes_date_naive_time_token_stream
                            }),
                            generate_impl_serde_de_visitor_for_field_visitor_token_stream(&quote::quote! {
                                #type_value_equal_underscore_field_semicolon_token_stream
                                #fn_expecting_field_identifier_token_stream
                                #fn_visit_u64_three_token_stream
                                #fn_visit_str_value_months_days_microseconds_token_stream
                                #fn_visit_bytes_months_days_microseconds_token_stream
                            }),
                        )
                    };
                    let impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                        quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_uuid_uuid_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        }
                    });
                    match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI32AsInt4 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI64AsInt8 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveF32AsFloat4 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveF64AsFloat8 => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_pg_money_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        })),
                        PostgresqlType::StdPrimitiveBoolAsBool => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_std_string_string_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        })),
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_four_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_5a4f24ce_7a8e_4bcc_8f79_2494f79bcc08
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_time_token_stream
                            #const_fields_sqlx_types_chrono_naive_time_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxTypesTimeTimeAsTime => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_four_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_9b240c3e_a4af_4da1_a2ab_f1bab44b1df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_time_time_token_stream
                            #const_fields_sqlx_types_time_time_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        })),
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_dc439ca1_8af1_4c4c_ab49_4e4fb15a41d3
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_time_token_stream
                            #const_fields_sqlx_types_chrono_naive_date_time_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_8c733fe0_c816_4a0e_bb13_4c2d0cd2ded6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream
                            #const_fields_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream),
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_mac_address_mac_address_token_stream
                            #serde_deserializer_deserialize_newtype_struct_token_stream
                        })),
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_three_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f702a411_b02b_4c90_aa7f_962a698612e7
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_interval_token_stream
                            #const_fields_sqlx_postgres_types_pg_interval_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                            #enum_field_two_token_stream
                            #impl_serde_de_visitor_for_field_visitor_token_stream_f4d8cc33_bf35_4c13_a745_341364a68df6
                            #impl_serde_deserialize_for_field_token_stream
                            #struct_visitor_token_stream
                            #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream
                            #const_fields_start_end_token_stream
                            #serde_deserializer_deserialize_struct_visitor_token_stream
                        })),
                    }
                };
                (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl)
            } else {
                (postgresql_crud_macros_common::DeriveOrImpl::Derive, postgresql_crud_macros_common::DeriveOrImpl::Derive)
            };
            let value_ident_inner_type_token_stream = quote::quote! {#value_snake_case: #ident_inner_type_token_stream};
            let ident_standart_not_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_origin_try_new_error_named_upper_camel_case = naming::parameter::SelfOriginTryNewErrorNamedUpperCamelCase::from_display(&ident_standart_not_null_upper_camel_case);
            let ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case = naming::parameter::SelfOriginTryNewForDeserializeErrorNamedUpperCamelCase::from_display(&ident_standart_not_null_upper_camel_case);
            enum IntRangeType {
                SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
                SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
            }
            let int_range_type_to_range_inner_type_token_stream = |int_range_type: &IntRangeType| -> proc_macro2::TokenStream {
                match &int_range_type {
                    IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => quote::quote! {#std_primitive_i32_token_stream},
                    IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote::quote! {#std_primitive_i64_token_stream},
                }
            };
            let generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                    #content_token_stream,
                    sqlx::types::chrono::Utc
                )}
            };
            let generate_sqlx_types_chrono_naive_date_time_new_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {sqlx::types::chrono::NaiveDateTime::#new_snake_case(#content_token_stream)}
            };
            let generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {sqlx::types::time::Time::from_hms_micro(#content_token_stream).unwrap()}
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
                        let field_type_handle_token_stream = quote::quote! {#field_type_handle};
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => field_type_handle_token_stream,
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
                let date_upper_camel_case = naming::DateUpperCamelCase;
                let time_upper_camel_case = naming::TimeUpperCamelCase;
                let date_naive_upper_camel_case = naming::DateNaiveUpperCamelCase;
                let time_upper_camel_camel_case = naming::TimeUpperCamelCase;
                let start_upper_camel_case = naming::StartUpperCamelCase;
                let end_upper_camel_case = naming::EndUpperCamelCase;
                let invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case = naming::InvalidHourOrMinuteOrSecondOrMicrosecondUpperCamelCase;
                let nanosecond_precision_is_not_supported_upper_camel_case = naming::NanosecondPrecisionIsNotSupportedUpperCamelCase;
                let included_start_more_than_included_end_upper_camel_case = naming::IncludedStartMoreThanIncludedEndUpperCamelCase;
                let included_start_more_than_excluded_end_upper_camel_case = naming::IncludedStartMoreThanExcludedEndUpperCamelCase;
                let excluded_start_more_than_included_end_upper_camel_case = naming::ExcludedStartMoreThanIncludedEndUpperCamelCase;
                let excluded_start_more_than_excluded_end_upper_camel_case = naming::ExcludedStartMoreThanExcludedEndUpperCamelCase;
                let included_end_cannot_be_max_upper_camel_case = naming::IncludedEndCannotBeMaxUpperCamelCase;
                let generate_int_range_type_error_variants_token_stream = |int_range_type: &IntRangeType| {
                    let range_inner_type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                    quote::quote! {
                        #included_start_more_than_included_end_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            #start_snake_case: #range_inner_type_token_stream,
                            #[eo_to_std_string_string_serialize_deserialize]
                            #end_snake_case: #range_inner_type_token_stream,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                        #included_start_more_than_excluded_end_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            #start_snake_case: #range_inner_type_token_stream,
                            #[eo_to_std_string_string_serialize_deserialize]
                            #end_snake_case: #range_inner_type_token_stream,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                        #excluded_start_more_than_included_end_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            #start_snake_case: #range_inner_type_token_stream,
                            #[eo_to_std_string_string_serialize_deserialize]
                            #end_snake_case: #range_inner_type_token_stream,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                        #excluded_start_more_than_excluded_end_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            #start_snake_case: #range_inner_type_token_stream,
                            #[eo_to_std_string_string_serialize_deserialize]
                            #end_snake_case: #range_inner_type_token_stream,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                        #included_end_cannot_be_max_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            #end_snake_case: #range_inner_type_token_stream,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                    }
                };
                let nanosecond_precision_is_not_supported_variant_try_new_token_stream = quote::quote! {
                    #nanosecond_precision_is_not_supported_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #value_snake_case: #std_string_string_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                };
                let sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_token_stream = quote::quote! {
                    #earlier_date_not_supported_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #value_snake_case: #std_string_string_token_stream,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #earliest_supported_date_snake_case: #std_string_string_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                };
                let std_string_string_as_text_try_new_error_named_variants_token_stream = quote::quote! {
                    #contains_null_byte_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #value_snake_case: #ident_inner_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                };
                let maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_token_stream = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart, Ok(postgresql_type_initialization_try_new)) = (&not_null_or_nullable, &postgresql_type_pattern, &postgresql_type_initialization_try_new_try_from_postgresql_type) {
                    let content_token_stream: &dyn quote::ToTokens = {
                        match &postgresql_type_initialization_try_new {
                            PostgresqlTypeInitializationTryNew::StdStringStringAsText => &std_string_string_as_text_try_new_error_named_variants_token_stream,
                            PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => &nanosecond_precision_is_not_supported_variant_try_new_token_stream,
                            PostgresqlTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_variant_try_new_token_stream,
                            PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_token_stream,
                            PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => &quote::quote! {
                                #date_upper_camel_case {
                                    #[eo_error_occurence]
                                    #error_snake_case: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                #time_upper_camel_case {
                                    #[eo_error_occurence]
                                    #error_snake_case: #sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            PostgresqlTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &quote::quote! {
                                #date_naive_upper_camel_case {
                                    #[eo_error_occurence]
                                    #error_snake_case: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                                #time_upper_camel_camel_case {
                                    #[eo_error_occurence]
                                    #error_snake_case: #sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_upper_camel_case,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            },
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &{
                                let (start_variant_token_stream, end_variant_token_stream) = {
                                    let generate_variant_token_stream = |start_or_end: &StartOrEnd| {
                                        let start_or_end_upper_camel_case = generate_start_or_end_upper_camel_case(start_or_end);
                                        quote::quote! {
                                            #start_or_end_upper_camel_case {
                                                #[eo_error_occurence]
                                                #error_snake_case: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_upper_camel_case,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    };
                                    (generate_variant_token_stream(&StartOrEnd::Start), generate_variant_token_stream(&StartOrEnd::End))
                                };
                                quote::quote! {
                                    #start_variant_token_stream,
                                    #end_variant_token_stream,
                                }
                            },
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &{
                                let (start_variant_token_stream, end_variant_token_stream) = {
                                    let generate_variant_token_stream = |start_or_end: &StartOrEnd| {
                                        let start_or_end_upper_camel_case = generate_start_or_end_upper_camel_case(start_or_end);
                                        quote::quote! {
                                            #start_or_end_upper_camel_case {
                                                #[eo_error_occurence]
                                                #error_snake_case: #sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_error_named_upper_camel_case,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    };
                                    (generate_variant_token_stream(&StartOrEnd::Start), generate_variant_token_stream(&StartOrEnd::End))
                                };
                                quote::quote! {
                                    #start_variant_token_stream,
                                    #end_variant_token_stream
                                }
                            },
                            PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &{
                                let (start_variant_token_stream, end_variant_token_stream) = {
                                    let generate_variant_token_stream = |start_or_end: &StartOrEnd| {
                                        let start_or_end_upper_camel_case = generate_start_or_end_upper_camel_case(start_or_end);
                                        quote::quote! {
                                            #start_or_end_upper_camel_case {
                                                #[eo_error_occurence]
                                                #error_snake_case: #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_error_named_upper_camel_case,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    };
                                    (generate_variant_token_stream(&StartOrEnd::Start), generate_variant_token_stream(&StartOrEnd::End))
                                };
                                quote::quote! {
                                    #start_variant_token_stream,
                                    #end_variant_token_stream
                                }
                            },
                        }
                    };
                    quote::quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_standart_not_null_origin_try_new_error_named_upper_camel_case {
                            #content_token_stream
                        }
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_token_stream = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart, postgresql_crud_macros_common::DeriveOrImpl::Impl(_)) = (&not_null_or_nullable, &postgresql_type_pattern, &serde_deserialize_derive_or_impl) {
                    match &postgresql_type_deserialize {
                        PostgresqlTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                        PostgresqlTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                            PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(_) => proc_macro2::TokenStream::new(),
                            PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgresql_type_impl_try_new_for_deserialize) => {
                                let content_token_stream: &dyn quote::ToTokens = match &postgresql_type_impl_try_new_for_deserialize {
                                    PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => &std_string_string_as_text_try_new_error_named_variants_token_stream,
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => &{
                                        quote::quote! {
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
                                            },
                                            #nanosecond_precision_is_not_supported_variant_try_new_token_stream
                                        }
                                    },
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => &{
                                        quote::quote! {
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
                                            },
                                            #nanosecond_precision_is_not_supported_variant_try_new_token_stream
                                        }
                                    },
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_token_stream,
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                    PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                };
                                quote::quote! {
                                    #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                                    pub enum #ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case {
                                        #content_token_stream
                                    }
                                }
                            }
                        },
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let impl_ident_origin_token_stream = {
                    let pub_fn_new_or_try_new_token_stream = {
                        if let Ok(postgresql_type_initialization_try_new) = &postgresql_type_initialization_try_new_try_from_postgresql_type {
                            let content_token_stream = {
                                let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                                    quote::quote! {Ok(Self(match #value_snake_case {
                                        Some(#value_snake_case) => Some(match #type_token_stream::#try_new_snake_case(#value_snake_case) {
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
                                                match #type_token_stream::#try_new_snake_case(#element_snake_case) {
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
                                            let generate_int_range_check_token_stream = |int_range_type: &IntRangeType| {
                                                let max_value_token_stream = {
                                                    let type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                                                    quote::quote! {#type_token_stream::MAX}
                                                };
                                                quote::quote! {
                                                    let max = #max_value_token_stream;
                                                    let (#start_snake_case, #end_snake_case) = match (#value_snake_case.#start_snake_case, #value_snake_case.#end_snake_case) {
                                                        (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Included(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_more_than_included_end_upper_camel_case {
                                                                    #start_snake_case,
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            if #end_snake_case == max {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Included(#end_snake_case))
                                                        }
                                                        (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_more_than_excluded_end_upper_camel_case {
                                                                    #start_snake_case,
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case))
                                                        }
                                                        (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Unbounded) => (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Unbounded),
                                                        (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Included(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_more_than_included_end_upper_camel_case {
                                                                    #start_snake_case,
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            if #end_snake_case == max {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Included(#end_snake_case))
                                                        }
                                                        (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case)) => {
                                                            if #start_snake_case > #end_snake_case {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_more_than_excluded_end_upper_camel_case {
                                                                    #start_snake_case,
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case))
                                                        }
                                                        (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Unbounded) => (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Unbounded),
                                                        (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_snake_case)) => {
                                                            if #end_snake_case == max {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                                    #end_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_snake_case))
                                                        }
                                                        (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_snake_case)) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_snake_case)),
                                                        (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded),
                                                    };
                                                    Ok(Self(sqlx::postgres::types::PgRange { #start_snake_case, #end_snake_case }))
                                                }
                                            };
                                            let generate_ok_self_sqlx_postgres_types_pg_range_token_stream = |ident_origin_token_stream: &dyn quote::ToTokens| {
                                                quote::quote! {
                                                    let #value_snake_case = sqlx::postgres::types::PgRange {
                                                        #start_snake_case: match #value_snake_case.#start_snake_case {
                                                            std::ops::Bound::Included(#value_snake_case) => match #ident_origin_token_stream::#try_new_snake_case(#value_snake_case) {
                                                                Ok(#value_snake_case) => std::ops::Bound::Included(#value_snake_case.0),
                                                                Err(#error_snake_case) => {
                                                                    return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#start_upper_camel_case {
                                                                        #error_snake_case,
                                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                                    });
                                                                }
                                                            },
                                                            std::ops::Bound::Excluded(#value_snake_case) => match #ident_origin_token_stream::#try_new_snake_case(#value_snake_case) {
                                                                Ok(#value_snake_case) => std::ops::Bound::Excluded(#value_snake_case.0),
                                                                Err(#error_snake_case) => {
                                                                    return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#start_upper_camel_case {
                                                                        #error_snake_case,
                                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                                    });
                                                                }
                                                            },
                                                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                                        },
                                                        #end_snake_case: match #value_snake_case.#end_snake_case {
                                                            std::ops::Bound::Included(#value_snake_case) => match #ident_origin_token_stream::#try_new_snake_case(#value_snake_case) {
                                                                Ok(#value_snake_case) => std::ops::Bound::Included(#value_snake_case.0),
                                                                Err(#error_snake_case) => {
                                                                    return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#end_upper_camel_case {
                                                                        #error_snake_case,
                                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                                    });
                                                                }
                                                            },
                                                            std::ops::Bound::Excluded(#value_snake_case) => match #ident_origin_token_stream::#try_new_snake_case(#value_snake_case) {
                                                                Ok(#value_snake_case) => std::ops::Bound::Excluded(#value_snake_case.0),
                                                                Err(#error_snake_case) => {
                                                                    return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#end_upper_camel_case {
                                                                        #error_snake_case,
                                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                                    });
                                                                }
                                                            },
                                                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                                        },
                                                    };
                                                    Ok(Self(#value_snake_case))
                                                }
                                            };
                                            match &postgresql_type_initialization_try_new {
                                                PostgresqlTypeInitializationTryNew::StdStringStringAsText => quote::quote! {
                                                    if #value_snake_case.find('\0').is_some() {
                                                        Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#contains_null_byte_upper_camel_case {
                                                            #value_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    } else {
                                                        Ok(Self(#value_snake_case))
                                                    }
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => quote::quote! {
                                                    if <#inner_type_standart_not_null_token_stream as chrono::Timelike>::nanosecond(&#value_snake_case) % 1000 != 0 {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                            #value_snake_case: #value_snake_case.to_string(),
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    Ok(Self(#value_snake_case))
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => quote::quote! {
                                                    if #value_snake_case.nanosecond() % 1000 != 0 {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                            #value_snake_case: #value_snake_case.to_string(),
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    Ok(Self(#value_snake_case))
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => quote::quote! {
                                                    let #earliest_supported_date_snake_case = #inner_type_standart_not_null_token_stream::from_ymd_opt(-4713, 12, 31).unwrap();
                                                    if #value_snake_case >= #earliest_supported_date_snake_case {
                                                        Ok(Self(#value_snake_case))
                                                    }
                                                    else {
                                                        Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                                            #value_snake_case: #value_snake_case.to_string(),
                                                            #earliest_supported_date_snake_case: #earliest_supported_date_snake_case.to_string(),
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    }
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {
                                                    let #date_snake_case = match #sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case::#try_new_snake_case(
                                                        #value_snake_case.#date_snake_case()
                                                    ) {
                                                        Ok(#value_snake_case) => #value_snake_case,
                                                        Err(#error_snake_case) => {
                                                            return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#date_upper_camel_case {
                                                                #error_snake_case,
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            });
                                                        }
                                                    };
                                                    let #time_snake_case = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case::#try_new_snake_case(
                                                        #value_snake_case.#time_snake_case()
                                                    ) {
                                                        Ok(#value_snake_case) => #value_snake_case,
                                                        Err(#error_snake_case) => {
                                                            return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#time_upper_camel_case {
                                                                #error_snake_case,
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            });
                                                        }
                                                    };
                                                    Ok(Self(#inner_type_standart_not_null_token_stream::#new_snake_case(#date_snake_case.0, #time_snake_case.0)))
                                                },
                                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                                        #date_naive_snake_case.0,
                                                        #time_snake_case.0
                                                    }));
                                                    quote::quote! {
                                                        let #date_naive_snake_case = match #sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case::#try_new_snake_case(#value_snake_case.date_naive()) {
                                                            Ok(#value_snake_case) => #value_snake_case,
                                                            Err(#error_snake_case) => {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#date_naive_upper_camel_case {
                                                                    #error_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                        };
                                                        let #time_snake_case = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case::#try_new_snake_case(#value_snake_case.time()) {
                                                            Ok(#value_snake_case) => #value_snake_case,
                                                            Err(#error_snake_case) => {
                                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#time_upper_camel_camel_case {
                                                                    #error_snake_case,
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                        };
                                                        Ok(Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream))
                                                    }
                                                }
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_int_range_check_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_int_range_check_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => generate_ok_self_sqlx_postgres_types_pg_range_token_stream(&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case),
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => generate_ok_self_sqlx_postgres_types_pg_range_token_stream(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case),
                                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_ok_self_sqlx_postgres_types_pg_range_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case),
                                            }
                                        }
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
                            quote::quote! {
                                pub fn #try_new_snake_case(#value_ident_inner_type_token_stream) -> Result<Self, #ident_standart_not_null_origin_try_new_error_named_upper_camel_case> {
                                    #content_token_stream
                                }
                            }
                        } else {
                            let content_token_stream = {
                                let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                                    quote::quote! {match value {
                                        Some(value) => Some(#type_token_stream::#new_snake_case(value)),
                                        None => None
                                    }}
                                };
                                let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value.into_iter().map(|element|#type_token_stream::#new_snake_case(element)).collect()},
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&type_token_stream),
                                };
                                match &postgresql_type_pattern {
                                    PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                            if let Ok(ref value) = postgresql_type_range_try_from_postgresql_type {
                                                generate_pg_range_conversion_token_stream(&value_snake_case, &{
                                                    let range_postgresql_type_ident_origin = naming::parameter::SelfOriginUpperCamelCase::from_display(&generate_ident_stringified(&PostgresqlType::from(value), not_null_or_nullable, postgresql_type_pattern));
                                                    quote::quote! {#range_postgresql_type_ident_origin::#new_snake_case(value)}
                                                })
                                            } else {
                                                quote::quote! {#value_snake_case}
                                            }
                                        }
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
                            quote::quote! {
                                pub fn new(#value_ident_inner_type_token_stream) -> Self {
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
                                        let parameters_token_stream = {
                                            let generate_start_end_std_std_ops_bound_token_stream = |ident_token_stream: &dyn quote::ToTokens| {
                                                quote::quote! {
                                                    #start_snake_case: std::ops::Bound<#ident_token_stream>,
                                                    #end_snake_case: std::ops::Bound<#ident_token_stream>
                                                }
                                            };
                                            match &postgresql_type_impl_new_for_deserialize {
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgIntervalAsInterval => quote::quote! {
                                                    #months_snake_case: #std_primitive_i32_token_stream,
                                                    #days_snake_case: #std_primitive_i32_token_stream,
                                                    #microseconds_snake_case: #std_primitive_i64_token_stream,
                                                },
                                                PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {
                                                    #date_snake_case: #sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case,
                                                    #time_snake_case: #sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case
                                                },
                                                PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote! {
                                                    #date_naive_snake_case: #sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case,
                                                    #time_snake_case: #sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case,
                                                },
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => generate_start_end_std_std_ops_bound_token_stream(&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case),
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => generate_start_end_std_std_ops_bound_token_stream(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case),
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_start_end_std_std_ops_bound_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case),
                                            }
                                        };
                                        let content_token_stream = {
                                            let self_sqlx_postgres_types_pg_range_token_stream = quote::quote! {
                                                Self(sqlx::postgres::types::PgRange {
                                                    start: match start {
                                                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                                                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                                                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                                    },
                                                    end: match end {
                                                        std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                                                        std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                                                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                                    },
                                                })
                                            };
                                            match &postgresql_type_impl_new_for_deserialize {
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgIntervalAsInterval => quote::quote! {
                                                    Self(sqlx::postgres::types::PgInterval {
                                                        #months_snake_case,
                                                        #days_snake_case,
                                                        #microseconds_snake_case,
                                                    })
                                                },
                                                PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {
                                                    Self(#inner_type_standart_not_null_token_stream::#new_snake_case(#date_snake_case.0, #time_snake_case.0))
                                                },
                                                PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                                        #date_naive_snake_case.0,
                                                        #time_snake_case.0
                                                    }));
                                                    quote::quote! {Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream)}
                                                }
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => self_sqlx_postgres_types_pg_range_token_stream,
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => self_sqlx_postgres_types_pg_range_token_stream,
                                                PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => self_sqlx_postgres_types_pg_range_token_stream,
                                            }
                                        };
                                        quote::quote! {
                                            fn new_for_deserialize(#parameters_token_stream) -> Self {
                                                #content_token_stream
                                            }
                                        }
                                    }
                                    PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgresql_type_impl_try_new_for_deserialize) => {
                                        let parameters_token_stream = {
                                            let generate_value_pg_range_int_type_token_stream = |int_range_type: &IntRangeType| {
                                                let type_token_stream = {
                                                    let content_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                                                    quote::quote! {std::ops::Bound<#content_token_stream>}
                                                };
                                                quote::quote! {
                                                    #start_snake_case: #type_token_stream,
                                                    #end_snake_case: #type_token_stream
                                                }
                                            };
                                            match &postgresql_type_impl_try_new_for_deserialize {
                                                PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => {
                                                    quote::quote! {
                                                        #value_ident_inner_type_token_stream
                                                    }
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                    quote::quote! {
                                                        #hour_snake_case: #std_primitive_u32_token_stream,
                                                        #min_snake_case: #std_primitive_u32_token_stream,
                                                        #sec_snake_case: #std_primitive_u32_token_stream,
                                                        #micro_snake_case: #std_primitive_u32_token_stream
                                                    }
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                    quote::quote! {
                                                        #hour_snake_case: #std_primitive_u8_token_stream,
                                                        #minute_snake_case: #std_primitive_u8_token_stream,
                                                        #second_snake_case: #std_primitive_u8_token_stream,
                                                        #microsecond_snake_case: #std_primitive_u32_token_stream
                                                    }
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => {
                                                    quote::quote! {
                                                        #value_ident_inner_type_token_stream
                                                    }
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_value_pg_range_int_type_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_value_pg_range_int_type_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                            }
                                        };
                                        let content_token_stream = {
                                            let generate_self_match_try_new_token_stream = |parameters_token_stream: &dyn quote::ToTokens, match_error_variants_token_stream: &dyn quote::ToTokens| {
                                                quote::quote! {
                                                    match Self::#try_new_snake_case(#parameters_token_stream) {
                                                        Ok(#value_snake_case) => Ok(#value_snake_case),
                                                        Err(#error_snake_case) => match #error_snake_case {
                                                            #match_error_variants_token_stream
                                                        }
                                                    }
                                                }
                                            };
                                            let try_new_convert_pg_range_int_content_token_stream = generate_self_match_try_new_token_stream(
                                                &quote::quote! {sqlx::postgres::types::PgRange { #start_snake_case, #end_snake_case }},
                                                &quote::quote! {
                                                    #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_more_than_included_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#included_start_more_than_included_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    }),
                                                    #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_more_than_excluded_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#included_start_more_than_excluded_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    }),
                                                    #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_more_than_included_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#excluded_start_more_than_included_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    }),
                                                    #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_more_than_excluded_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#excluded_start_more_than_excluded_end_upper_camel_case {
                                                        #start_snake_case,
                                                        #end_snake_case,
                                                        code_occurence,
                                                    }),
                                                    #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                        #end_snake_case,
                                                        code_occurence,
                                                    } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                        #end_snake_case,
                                                        code_occurence,
                                                    }),
                                                },
                                            );
                                            match &postgresql_type_impl_try_new_for_deserialize {
                                                PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => {
                                                    let variant_token_stream = quote::quote! {
                                                        #contains_null_byte_upper_camel_case {
                                                            #value_snake_case,
                                                            code_occurence,
                                                        }
                                                    };
                                                    generate_self_match_try_new_token_stream(
                                                        &value_snake_case,
                                                        &quote::quote! {
                                                            #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#variant_token_stream => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#variant_token_stream),
                                                        },
                                                    )
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                    quote::quote! {
                                                        match #inner_type_standart_not_null_token_stream::from_hms_micro_opt(
                                                            #hour_snake_case,
                                                            #min_snake_case,
                                                            #sec_snake_case,
                                                            #micro_snake_case,
                                                        ) {
                                                            Some(#value_snake_case) => {
                                                                if <#inner_type_standart_not_null_token_stream as chrono::Timelike>::nanosecond(&#value_snake_case) % 1000 != 0 {
                                                                    return Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                                        #value_snake_case: #value_snake_case.to_string(),
                                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                                    });
                                                                }
                                                                Ok(Self(#value_snake_case))
                                                            },
                                                            None => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                                                #hour_snake_case,
                                                                #min_snake_case,
                                                                #sec_snake_case,
                                                                #micro_snake_case,
                                                                code_occurence: error_occurence_lib::code_occurence!(),
                                                            })
                                                        }
                                                    }
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                    quote::quote! {
                                                        match #inner_type_standart_not_null_token_stream::from_hms_micro(
                                                            #hour_snake_case,
                                                            #minute_snake_case,
                                                            #second_snake_case,
                                                            #microsecond_snake_case,
                                                        ) {
                                                            Ok(#value_snake_case) => {
                                                                if #value_snake_case.nanosecond() % 1000 != 0 {
                                                                    return Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                                        #value_snake_case: #value_snake_case.to_string(),
                                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                                    });
                                                                }
                                                                Ok(Self(#value_snake_case))
                                                            },
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
                                                }
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => generate_self_match_try_new_token_stream(
                                                    &value_snake_case,
                                                    &quote::quote! {
                                                        #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                                            #value_snake_case,
                                                            #earliest_supported_date_snake_case,
                                                            code_occurence,
                                                        } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                                            #value_snake_case,
                                                            #earliest_supported_date_snake_case,
                                                            code_occurence,
                                                        }),
                                                    },
                                                ),
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => try_new_convert_pg_range_int_content_token_stream,
                                                PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => try_new_convert_pg_range_int_content_token_stream,
                                            }
                                        };
                                        quote::quote! {
                                            fn #try_new_for_deserialize_snake_case(#parameters_token_stream) -> Result<Self, #ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case> {
                                                #content_token_stream
                                            }
                                        }
                                    }
                                },
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: _ } => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {
                        impl #ident_origin_upper_camel_case {
                            #pub_fn_new_or_try_new_token_stream
                            #maybe_fn_new_or_try_new_for_deserialize_token
                        }
                    }
                };
                let impl_std_convert_into_ident_origin_impl_new_or_try_new_value_type_for_ident_origin_token_stream = {
                    let content_token_stream = {
                        let self_dot_zero_token_stream = quote::quote!{#self_snake_case.0};
                        let element_dot_zero_token_stream = quote::quote!{#element_snake_case.0};
                        let generate_match_token_stream = |match_content_token_stream: &dyn quote::ToTokens, some_content_token_stream: &dyn quote::ToTokens|{
                            quote::quote! {
                                match #match_content_token_stream {
                                    Some(#value_snake_case) => Some(#value_snake_case.0#some_content_token_stream),
                                    None => None
                                }
                            }
                        };
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => self_dot_zero_token_stream.clone(),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_token_stream(
                                    &self_dot_zero_token_stream,
                                    &proc_macro2::TokenStream::new(),
                                )
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let dimension1_token_stream = match &dimension1_not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => element_dot_zero_token_stream.clone(),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_token_stream(
                                        &element_dot_zero_token_stream,
                                        &proc_macro2::TokenStream::new(),
                                    )
                                };
                                let into_iter_dimension1_token_stream = quote::quote!{.into_iter().map(|#element_snake_case|#dimension1_token_stream).collect()};
                                match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                        #self_dot_zero_token_stream #into_iter_dimension1_token_stream
                                    },
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_token_stream(
                                        &self_dot_zero_token_stream,
                                        &into_iter_dimension1_token_stream,
                                    )
                                }
                            },
                        }
                    };
                    quote::quote! {
                        impl std::convert::Into<#ident_inner_type_token_stream> for #ident_origin_upper_camel_case {
                            fn into(#self_snake_case) -> #ident_inner_type_token_stream {
                                #content_token_stream
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
                let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {self.to_string()});
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_origin_upper_camel_case, &{
                    let content_token_stream = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let pg_range_int_default_initialization_token_stream = quote::quote! {
                                    sqlx::postgres::types::PgRange {
                                        start: std::ops::Bound::Included(#core_default_default_default_token_stream),
                                        end: std::ops::Bound::Excluded(#core_default_default_default_token_stream),
                                    }
                                };
                                let generate_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| {
                                    quote::quote! {
                                        <#current_ident_token_stream as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                    }
                                };
                                let generate_sqlx_postgres_types_pg_range_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| {
                                    let current_ident_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = generate_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream(&current_ident_token_stream);
                                    quote::quote! {
                                        sqlx::postgres::types::PgRange {
                                            #start_snake_case: std::ops::Bound::Included(
                                                #current_ident_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0
                                            ),
                                            #end_snake_case: std::ops::Bound::Excluded(
                                                #current_ident_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0
                                            ),
                                        }
                                    }
                                };
                                let sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = generate_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream(&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case);
                                let sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = generate_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream(&sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case);
                                let initialization_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                                    PostgresqlType::StdPrimitiveI16AsInt2
                                    | PostgresqlType::StdPrimitiveI32AsInt4
                                    | PostgresqlType::StdPrimitiveI64AsInt8
                                    | PostgresqlType::StdPrimitiveF32AsFloat4
                                    | PostgresqlType::StdPrimitiveF64AsFloat8
                                    | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                                    | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                                    | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => &quote::quote! {#inner_type_standart_not_null_token_stream(#core_default_default_default_token_stream)},
                                    PostgresqlType::StdPrimitiveBoolAsBool | PostgresqlType::StdStringStringAsText => &core_default_default_default_token_stream,
                                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => &quote::quote! {vec![#core_default_default_default_token_stream]},
                                    PostgresqlType::SqlxTypesTimeTimeAsTime => &generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {0,0,0,0}),
                                    PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => &{
                                        let double_dots_space_core_default_default_default_token_stream = generate_double_dot_space_tokens_token_stream(&core_default_default_default_token_stream);
                                        generate_sqlx_postgres_types_pg_interval_field_type_pattern_token_stream(&double_dots_space_core_default_default_default_token_stream, &double_dots_space_core_default_default_default_token_stream, &double_dots_space_core_default_default_default_token_stream)
                                    },
                                    PostgresqlType::SqlxTypesChronoNaiveDateAsDate => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                        #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                        #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                    }),
                                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                        #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                        #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                    })),
                                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => &quote::quote! {
                                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_token_stream).unwrap())
                                    },
                                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => &core_default_default_default_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &pg_range_int_default_initialization_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &pg_range_int_default_initialization_token_stream,
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &generate_sqlx_postgres_types_pg_range_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(&sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case),
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &generate_sqlx_postgres_types_pg_range_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case),
                                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &generate_sqlx_postgres_types_pg_range_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case),
                                };
                                quote::quote! {#initialization_token_stream}
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                        },
                        PostgresqlTypePattern::ArrayDimension1 { .. } => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {vec![#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                        },
                    };
                    quote::quote! {Self(#content_token_stream)}
                });
                let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &field_type_handle);
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = quote::quote! {
                    impl sqlx::Encode<'_, sqlx::Postgres> for #ident_origin_upper_camel_case {
                        fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
                        }
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &field_type_handle, &{
                    let scopes_value_token_stream = quote::quote! {(#value_snake_case)};
                    let ok_self_scopes_value_token_stream = quote::quote! {Ok(Self #scopes_value_token_stream)};
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_type {
                                PostgresqlType::StdPrimitiveI16AsInt2
                                | PostgresqlType::StdPrimitiveI32AsInt4
                                | PostgresqlType::StdPrimitiveI64AsInt8
                                | PostgresqlType::StdPrimitiveF32AsFloat4
                                | PostgresqlType::StdPrimitiveF64AsFloat8
                                | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                                | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                                | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql
                                | PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney
                                | PostgresqlType::StdPrimitiveBoolAsBool
                                | PostgresqlType::StdStringStringAsText
                                | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                                | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
                                | PostgresqlType::SqlxTypesTimeTimeAsTime
                                | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
                                | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                                | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                                | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql
                                | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                                | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                                | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr
                                | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                                | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                                | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_value_token_stream,
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote::quote! {
                                    match Self::#try_new_snake_case #scopes_value_token_stream {
                                        Ok(#value_snake_case) => Ok(#value_snake_case),
                                        Err(#error_snake_case) => Err(std::boxed::Box::#new_snake_case(error)),
                                    }
                                },
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => ok_self_scopes_value_token_stream,
                        },
                        PostgresqlTypePattern::ArrayDimension1 { .. } => ok_self_scopes_value_token_stream,
                    }
                });
                let impl_sqlx_postgres_pg_has_array_type_for_ident_origin_token_stream = {
                    quote::quote! {
                        impl sqlx::postgres::PgHasArrayType for #ident_origin_upper_camel_case {
                            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                                <#inner_type_standart_not_null_token_stream as sqlx::postgres::PgHasArrayType>::array_type_info()
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
                            PostgresqlTypePattern::ArrayDimension1 { .. } => std::iter::repeat_n("[]", array_dimensions_number).collect::<String>(),
                        };
                        let maybe_constraint_part = match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => "".to_string(),
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => ",check (array_position({column},null) is null)".to_string(),
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => "".to_string(),
                            },
                        };
                        let maybe_primary_key_is_primary_key_token_stream = quote::quote! {crate::maybe_primary_key(is_primary_key)};
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
                                    format!(#format_handle_token_stream, #maybe_primary_key_is_primary_key_token_stream)
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
                                    format!(#format_handle_token_stream, #maybe_primary_key_is_primary_key_token_stream)
                                }
                            }
                        }
                    },
                );
                let maybe_impl_std_convert_from_ident_read_for_ident_origin_token_stream = match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::True => {
                        let ident_standart_not_null_as_crate_postgresql_type_token_stream = generate_tokens_as_crate_postgresql_type_token_stream(&ident_standart_not_null_upper_camel_case);
                        quote::quote! {
                            impl std::convert::From<#ident_standart_not_null_read_upper_camel_case> for #ident_origin_upper_camel_case {
                                fn from(#value_snake_case: #ident_standart_not_null_read_upper_camel_case) -> Self {
                                    Self::#new_snake_case(#ident_standart_not_null_as_crate_postgresql_type_token_stream::into_inner(#value_snake_case))
                                }
                            }
                        }
                    }
                    IsNotNullStandartCanBePrimaryKey::False => proc_macro2::TokenStream::new(),
                };
                quote::quote! {
                    #ident_origin_token_stream
                    #maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_token_stream
                    #maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_token_stream
                    #impl_ident_origin_token_stream
                    #impl_std_convert_into_ident_origin_impl_new_or_try_new_value_type_for_ident_origin_token_stream

                    #maybe_impl_is_string_empty_for_ident_origin_token_stream
                    #maybe_impl_serde_serialize_for_ident_standart_not_null_origin_token_stream
                    #maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_token_stream
                    #impl_std_fmt_display_for_ident_origin_token_stream
                    #impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream
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
                    let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_create_upper_camel_case, &quote::quote! {Self(#core_default_default_default_token_stream)});
                    quote::quote! {
                        #ident_create_token_stream
                        #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream
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
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            let mut arguments_token_stream = vec![];
                            for element in 1..=array_dimensions_number {
                                let dimension_number_pagination_token_stream = format!("dimension{element}_pagination").parse::<proc_macro2::TokenStream>().unwrap();
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
                let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                    &ident_select_upper_camel_case,
                    &match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => quote::quote! {#core_default_default_default_token_stream},
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            let mut arguments_token_stream = vec![];
                            for element in 1..=array_dimensions_number {
                                let dimension_number_pagination_token_stream = format!("dimension{element}_pagination").parse::<proc_macro2::TokenStream>().unwrap();
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
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
                    #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
                }
            };
            let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
            let ident_where_element_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &{
                    let common_postgresql_type_filters = vec![postgresql_crud_macros_common::PostgresqlTypeFilter::Equal { ident: quote::quote! {#ident_origin_upper_camel_case} }];
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => {
                            let greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThan {
                                ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            let between = postgresql_crud_macros_common::PostgresqlTypeFilter::Between {
                                ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            let in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::In { ident: quote::quote! {#ident_table_type_declaration_upper_camel_case} };
                            let regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::RegularExpression;
                            let equal_to_encoded_string_representation = postgresql_crud_macros_common::PostgresqlTypeFilter::EqualToEncodedStringRepresentation;
                            let current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentDate;
                            let greater_than_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentDate;
                            let current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTime;
                            let greater_than_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTime;
                            let current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::CurrentTimestamp;
                            let greater_than_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanCurrentTimestamp;
                            let before = postgresql_crud_macros_common::PostgresqlTypeFilter::Before {
                                ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            // let bit_vec_position_equal = postgresql_crud_macros_common::PostgresqlTypeFilter::BitVecPositionEqual;
                            let common_standart_postgresql_type_filters = { common_postgresql_type_filters.clone() };
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
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::FindRangesThatFullyContainTheGivenRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToLeftOfRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::StrictlyToRightOfRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::IncludedLowerBound {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::ExcludedUpperBound {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanIncludedLowerBound {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::GreaterThanExcludedUpperBound {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::OverlapWithRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::AdjacentWithRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::RangeLength);
                                    vec
                                };
                                (ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone())
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
                                }
                                PostgresqlType::StdPrimitiveBoolAsBool => common_standart_postgresql_type_filters,
                                PostgresqlType::StdStringStringAsText => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(equal_to_encoded_string_representation.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_time.clone());
                                    vec.push(greater_than_current_time.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesTimeTimeAsTime => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_time.clone());
                                    vec.push(greater_than_current_time.clone());
                                    vec
                                }
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => common_standart_postgresql_type_filters,
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_date.clone());
                                    vec.push(greater_than_current_date.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(between.clone());
                                    vec.push(current_timestamp.clone());
                                    vec.push(greater_than_current_timestamp.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(before.clone());
                                    vec.push(between.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => common_standart_postgresql_type_filters,
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                    let mut vec = common_standart_postgresql_type_filters.clone();
                                    vec.push(greater_than.clone());
                                    vec.push(regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            }
                        }
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            let ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case = match &postgresql_type.can_be_nullable() {
                                CanBeNullable::True => quote::quote! {#ident_standart_not_null_or_nullable_table_type_declaration_upper_camel_case},
                                CanBeNullable::False => quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            let dimension_one_greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan {
                                ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            let dimension_one_between = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween {
                                ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            let dimension_one_in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn {
                                ident: ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case.clone(),
                            };
                            let dimension_one_regular_expression = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneRegularExpression;
                            let dimension_one_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentDate;
                            let dimension_one_greater_than_current_date = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentDate;
                            let dimension_one_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTime;
                            let dimension_one_greater_than_current_time = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTime;
                            let dimension_one_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneCurrentTimestamp;
                            let dimension_one_greater_than_current_timestamp = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanCurrentTimestamp;
                            let dimension_one_before = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBefore {
                                ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                            };
                            let common_array_dimension1_postgresql_type_filters = {
                                let mut vec = common_postgresql_type_filters.clone();
                                vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual {
                                    ident: ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case.clone(),
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
                                    let range_element_ident_standart_not_null_as_crate_postgresql_type_read_token_stream = {
                                        let range_element_ident_standart_not_null_as_crate_postgresql_type_token_stream = generate_tokens_as_crate_postgresql_type_token_stream(&range_element_ident_standart_not_null_token_stream);
                                        quote::quote! {#range_element_ident_standart_not_null_as_crate_postgresql_type_token_stream::Read}
                                    };
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesWithinGivenRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToLeftOfRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneStrictlyToRightOfRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIncludedLowerBound {
                                        ident: range_element_ident_standart_not_null_as_crate_postgresql_type_read_token_stream.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneExcludedUpperBound {
                                        ident: range_element_ident_standart_not_null_as_crate_postgresql_type_read_token_stream.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanIncludedLowerBound {
                                        ident: range_element_ident_standart_not_null_as_crate_postgresql_type_read_token_stream.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThanExcludedUpperBound {
                                        ident: range_element_ident_standart_not_null_as_crate_postgresql_type_read_token_stream,
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneOverlapWithRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneAdjacentWithRange {
                                        ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
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
                                }
                                PostgresqlType::StdPrimitiveBoolAsBool => common_array_dimension1_postgresql_type_filters,
                                PostgresqlType::StdStringStringAsText => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation);
                                    vec
                                }
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_time.clone());
                                    vec.push(dimension_one_greater_than_current_time.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesTimeTimeAsTime => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_time.clone());
                                    vec.push(dimension_one_greater_than_current_time.clone());
                                    vec
                                }
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => common_array_dimension1_postgresql_type_filters,
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_date.clone());
                                    vec.push(dimension_one_greater_than_current_date.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec.push(dimension_one_current_timestamp.clone());
                                    vec.push(dimension_one_greater_than_current_timestamp.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_before.clone());
                                    vec.push(dimension_one_between.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => common_array_dimension1_postgresql_type_filters,
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                    let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                    vec.push(dimension_one_greater_than.clone());
                                    vec.push(dimension_one_regular_expression.clone());
                                    vec
                                }
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            }
                        }
                    }
                }
                .iter()
                .map(|element| element as &dyn postgresql_crud_macros_common::PostgresqlFilter)
                .collect(),
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::False,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
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
                            pub fn #try_new_snake_case(#value_ident_inner_type_token_stream) -> Result<Self, #ident_standart_not_null_origin_try_new_error_named_upper_camel_case> {
                                match #ident_origin_upper_camel_case::#try_new_snake_case(#value_snake_case) {
                                    Ok(#value_snake_case) => Ok(Self(#value_snake_case)),
                                    Err(#error_snake_case) => Err(#error_snake_case)
                                }
                            }
                        }
                    } else {
                        quote::quote! {
                            pub fn new(#value_ident_inner_type_token_stream) -> Self {
                                Self(#ident_origin_upper_camel_case::#new_snake_case(#value_snake_case))
                            }
                        }
                    };
                    quote::quote! {
                        impl #ident_read_upper_camel_case {
                            #pub_fn_new_or_try_new_token_stream
                        }
                    }
                };
                let impl_error_occurence_lib_to_std_string_string_for_ident_read_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_read_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {self.0.to_string()});
                let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream =
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
                let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = quote::quote! {
                    impl sqlx::Encode<'_, sqlx::Postgres> for #ident_read_upper_camel_case {
                        fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
                        }
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &ident_origin_upper_camel_case, &quote::quote! {Ok(Self(#value_snake_case))});
                let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &ident_origin_upper_camel_case);
                let (maybe_impl_postgresql_type_where_filter_for_ident_read_if_can_be_primary_key_token_stream, maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream) =
                    if let (CanBePrimaryKey::True, postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) = (&can_be_primary_key, &not_null_or_nullable, &postgresql_type_pattern) {
                        (
                            postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                                &quote::quote! {<'a>},
                                &ident_standart_not_null_read_upper_camel_case,
                                &proc_macro2::TokenStream::new(),
                                &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                                &postgresql_crud_macros_common::ColumnParameterUnderscore::False,
                                &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::True,
                                &{
                                    let postgresql_crud_common_query_part_error_named_checked_add_initialization_token_stream = postgresql_crud_macros_common::postgresql_crud_common_query_part_error_named_checked_add_initialization_token_stream();
                                    quote::quote! {
                                        match #increment_snake_case.checked_add(1) {
                                            Some(value) => {
                                                *#increment_snake_case = value;
                                                Ok(format!("({} = ${})", column, #increment_snake_case))
                                            },
                                            None => Err(#postgresql_crud_common_query_part_error_named_checked_add_initialization_token_stream)
                                        }
                                    }
                                },
                                &postgresql_crud_macros_common::IsQueryBindMutable::True,
                                &generate_typical_query_bind_token_stream(&naming::SelfSnakeCase),
                                &postgresql_crud_macros_common_import_path_postgresql_crud_common, //meeow
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
            let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
            let ident_read_only_ids_token_stream = {
                let ident_read_only_ids_token_stream = {
                    // let wrap_into_value_declaration_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
                    //     quote::quote!{#import_path::#value_upper_camel_case<#ident_token_stream>}
                    // };
                    //HERE
                    let content_token_stream = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &can_be_primary_key {
                                CanBePrimaryKey::True => quote::quote! {Value<#ident_read_upper_camel_case>},
                                CanBePrimaryKey::False => quote::quote! {Value<std::option::Option<()>>}
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {std::option::Option<Value<#ident_standart_not_null_read_only_ids_upper_camel_case>>}
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => 
                        // match &not_null_or_nullable {
                        //     postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {std::vec::Vec<Value<#ident_array_not_null_read_only_ids_upper_camel_case>>},
                        //     postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {std::option::Option<Value<#ident_array_nullable_read_only_ids_upper_camel_case>>}
                        // },
                        todo!()
                    };
                    quote::quote! {
                        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                        pub struct #ident_read_only_ids_upper_camel_case(#content_token_stream);
                    }
                };
                let impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream = {
                    quote::quote! {
                        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_read_only_ids_upper_camel_case {
                            fn decode(#value_snake_case: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                                match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(#value_snake_case) {
                                    Ok(#value_snake_case) => Ok(#value_snake_case.0),
                                    Err(#error_snake_case) => Err(#error_snake_case),
                                }
                            }
                        }
                    }
                };
                let impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream = {
                    quote::quote! {
                        impl sqlx::Type<sqlx::Postgres> for #ident_read_only_ids_upper_camel_case {
                            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                                <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
                            }
                            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                                <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
                            }
                        }
                    }
                };
                quote::quote! {
                    #ident_read_only_ids_token_stream
                    #impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream
                    #impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream
                }
            };
            // let ident_read_only_ids_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            //     &ident_read_only_ids_upper_camel_case,
            //     &if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
            //         && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
            //         && let CanBePrimaryKey::True = &can_be_primary_key
            //     {
            //         quote::quote! {#ident_read_upper_camel_case}
            //     } else {
            //         postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {()})
            //     },
            // );
            let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
            let ident_read_inner_token_stream = quote::quote! {
                pub type #ident_read_inner_upper_camel_case = #ident_inner_type_token_stream;
            };
            let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
            let ident_update_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_update_upper_camel_case, &ident_origin_upper_camel_case);
            let ident_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
            let ident_update_for_query_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_update_for_query_upper_camel_case, &ident_origin_upper_camel_case);
            let impl_postgresql_type_for_ident_token_stream = {
                let generate_ok_std_string_string_from_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {Ok(#std_string_string_token_stream::from(#content_token_stream))}
                };
                let ok_std_string_string_from_default_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"default"});
                let ok_std_string_string_from_uuid_generate_v4_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"uuid_generate_v4()"});
                let typical_query_part_token_stream = {
                    let acc_snake_case = naming::AccSnakeCase;
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                    let postgresql_crud_common_query_part_error_named_token_stream = postgresql_crud_macros_common::postgresql_crud_common_query_part_error_named_token_stream();
                    quote::quote! {
                        let mut #acc_snake_case = std::string::String::default();
                        match #increment_snake_case.checked_add(1) {
                            Some(#value_snake_case) => {
                                *#increment_snake_case = #value_snake_case;
                                #acc_snake_case.push_str(&format!(#format_handle_token_stream));
                            }
                            None => {
                                return Err(#postgresql_crud_common_query_part_error_named_token_stream::#checked_add_upper_camel_case {
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                        Ok(#acc_snake_case)
                    }
                };
                let ok_query_token_stream = quote::quote!{Ok(#query_snake_case)};
                type Handle<'a> = (&'a dyn quote::ToTokens, &'a dyn quote::ToTokens);
                let (query_part_create_token_stream, bind_value_to_query_create_token_stream): Handle = {
                    let typical: Handle = { (&typical_query_part_token_stream, &typical_query_bind_token_stream) };
                    let default_initialized_by_postgresql: Handle = (
                        &ok_std_string_string_from_default_token_stream,
                        &ok_query_token_stream
                    );
                    let uuid_generate_v4_initialized_by_postgresql: Handle = (
                        &ok_std_string_string_from_uuid_generate_v4_token_stream,
                        &ok_query_token_stream
                    );
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
                postgresql_crud_macros_common::generate_impl_postgresql_type_token_stream(
                    &postgresql_crud_macros_common_import_path_postgresql_crud_common,
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
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            //todo change trait fn select_query_part( to Result String CheckedAdd
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&{
                                let acc = std::iter::repeat_n("[{}:{}]", array_dimensions_number).collect::<String>();
                                format!("{{column}}{acc}")
                            });
                            let arguments_token_stream = {
                                let mut acc = vec![];
                                for element in 1..=array_dimensions_number {
                                    let dimension_number_pagination_token_stream = format!("dimension{element}_pagination").parse::<proc_macro2::TokenStream>().unwrap();
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
                    &{
                        let generate_ident_read_ident_origin_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {#ident_read_upper_camel_case(#ident_origin_upper_camel_case(#content_token_stream))}
                        };
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    if let Ok(postgresql_type_range) = &PostgresqlTypeRange::try_from(postgresql_type) {
                                        let generate_sqlx_postgres_types_pg_range_token_stream = |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                sqlx::postgres::types::PgRange{
                                                    #start_snake_case: std::ops::Bound::#start_token_stream,
                                                    #end_snake_case: std::ops::Bound::#end_token_stream
                                                }
                                            }
                                        };
                                        let included_start_token_stream = quote::quote! {#included_upper_camel_case(#start_snake_case)};
                                        let excluded_end_token_stream = quote::quote! {#excluded_upper_camel_case(#end_snake_case)};
                                        let included_end_token_stream = quote::quote! {#included_upper_camel_case(#end_snake_case)};
                                        let excluded_start_token_stream = quote::quote! {#excluded_upper_camel_case(#start_snake_case)};
                                        let sqlx_postgres_types_pg_range_excluded_excluded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&excluded_start_token_stream, &excluded_end_token_stream);
                                        let sqlx_postgres_types_pg_range_excluded_included_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&excluded_start_token_stream, &included_end_token_stream);
                                        let sqlx_postgres_types_pg_range_included_unbounded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &unbounded_upper_camel_case);
                                        let sqlx_postgres_types_pg_range_unbounded_excluded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &excluded_end_token_stream);
                                        let sqlx_postgres_types_pg_range_included_excluded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &excluded_end_token_stream);
                                        let sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &unbounded_upper_camel_case);
                                        let generate_range_match_token_stream = |included_included_token_stream: &dyn quote::ToTokens,
                                                                                 included_excluded_token_stream: &dyn quote::ToTokens,
                                                                                 included_unbounded_token_stream: &dyn quote::ToTokens,
                                                                                 excluded_included_token_stream: &dyn quote::ToTokens,
                                                                                 excluded_excluded_token_stream: &dyn quote::ToTokens,
                                                                                 excluded_unbounded_token_stream: &dyn quote::ToTokens,
                                                                                 unbounded_included_token_stream: &dyn quote::ToTokens,
                                                                                 unbounded_excluded_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                #ident_standart_not_null_read_upper_camel_case(#ident_standart_not_null_origin_upper_camel_case(match (#value_snake_case.0.0.#start_snake_case, #value_snake_case.0.0.#end_snake_case) {
                                                    (std::ops::Bound::#included_upper_camel_case(#start_snake_case), std::ops::Bound::#included_upper_camel_case(#end_snake_case)) => {
                                                        #included_included_token_stream
                                                    },
                                                    (std::ops::Bound::#included_upper_camel_case(#start_snake_case), std::ops::Bound::#excluded_upper_camel_case(#end_snake_case)) => {
                                                        #included_excluded_token_stream
                                                    },
                                                    (std::ops::Bound::#included_upper_camel_case(#start_snake_case), std::ops::Bound::#unbounded_upper_camel_case) => {
                                                        #included_unbounded_token_stream
                                                    },
                                                    (std::ops::Bound::#excluded_upper_camel_case(#start_snake_case), std::ops::Bound::#included_upper_camel_case(#end_snake_case)) => {
                                                        #excluded_included_token_stream
                                                    },
                                                    (std::ops::Bound::#excluded_upper_camel_case(#start_snake_case), std::ops::Bound::#excluded_upper_camel_case(#end_snake_case)) => {
                                                        #excluded_excluded_token_stream
                                                    },
                                                    (std::ops::Bound::#excluded_upper_camel_case(#start_snake_case), std::ops::Bound::#unbounded_upper_camel_case) => {
                                                        #excluded_unbounded_token_stream
                                                    },
                                                    (std::ops::Bound::#unbounded_upper_camel_case, std::ops::Bound::#included_upper_camel_case(#end_snake_case)) => {
                                                        #unbounded_included_token_stream
                                                    },
                                                    (std::ops::Bound::#unbounded_upper_camel_case, std::ops::Bound::#excluded_upper_camel_case(#end_snake_case)) => {
                                                        #unbounded_excluded_token_stream
                                                    },
                                                    (std::ops::Bound::#unbounded_upper_camel_case, std::ops::Bound::#unbounded_upper_camel_case) => #sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream,
                                                }))
                                            }
                                        };
                                        let generate_if_start_end_equal_token_stream = |true_token_stream: &dyn quote::ToTokens, false_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                if #start_snake_case == #end_snake_case {
                                                    #true_token_stream
                                                } else {
                                                    #false_token_stream
                                                }
                                            }
                                        };
                                        let if_equal_unbounded_unbounded_or_included_excluded_token_stream = generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &sqlx_postgres_types_pg_range_included_excluded_token_stream);
                                        let int_range_normalize_token_stream = {
                                            let generate_included_start_checked_add_token_stream = |id: &dyn std::fmt::Display| {
                                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&id);
                                                quote::quote! {#included_upper_camel_case(#start_snake_case.checked_add(1).expect(#format_handle_token_stream))}
                                            };
                                            let generate_excluded_end_checked_add_token_stream = |id: &dyn std::fmt::Display| {
                                                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&id);
                                                quote::quote! {#excluded_upper_camel_case(#end_snake_case.checked_add(1).expect(#format_handle_token_stream))}
                                            };
                                            let included_excluded_checked_add_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &generate_excluded_end_checked_add_token_stream(&"73fe1d32-6a04-4578-b251-15ed7009b47e"));
                                            let included_unbounded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &unbounded_upper_camel_case);
                                            let included_checked_add_excluded_checked_add_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&generate_included_start_checked_add_token_stream(&"586c25e8-c948-4399-a69f-2bda10d493e9"), &generate_excluded_end_checked_add_token_stream(&"cc8f4052-4923-4223-8e71-6c20d72cf1cb"));
                                            let included_checked_add_excluded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&generate_included_start_checked_add_token_stream(&"586bde76-a3cc-4118-bd9c-69155c03ae0c"), &excluded_end_token_stream);
                                            let included_checked_add_unbounded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&generate_included_start_checked_add_token_stream(&"cd851f25-3379-40e3-bf3e-84aff7e053b3"), &unbounded_upper_camel_case);
                                            let unbounded_excluded_checked_add_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &generate_excluded_end_checked_add_token_stream(&"68604d50-dd71-4d7b-8e19-38238e2f3631"));
                                            let unbounded_excluded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &excluded_end_token_stream);
                                            generate_range_match_token_stream(
                                                &included_excluded_checked_add_token_stream,
                                                &generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &sqlx_postgres_types_pg_range_included_excluded_token_stream),
                                                &included_unbounded_token_stream,
                                                &generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &included_checked_add_excluded_checked_add_token_stream),
                                                &generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &included_checked_add_excluded_token_stream),
                                                &included_checked_add_unbounded_token_stream,
                                                &unbounded_excluded_checked_add_token_stream,
                                                &unbounded_excluded_token_stream,
                                            )
                                        };
                                        let range_match_timestamp_and_timestamp_tz_token_stream = generate_range_match_token_stream(
                                            &generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &included_end_token_stream),
                                            &if_equal_unbounded_unbounded_or_included_excluded_token_stream,
                                            &sqlx_postgres_types_pg_range_included_unbounded_token_stream,
                                            &generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &sqlx_postgres_types_pg_range_excluded_included_token_stream),
                                            &generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &sqlx_postgres_types_pg_range_excluded_excluded_token_stream),
                                            &generate_sqlx_postgres_types_pg_range_token_stream(&excluded_start_token_stream, &unbounded_upper_camel_case),
                                            &generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &included_end_token_stream),
                                            &sqlx_postgres_types_pg_range_unbounded_excluded_token_stream,
                                        );
                                        match &postgresql_type_range {
                                            PostgresqlTypeRange::StdPrimitiveI32AsInt4 => int_range_normalize_token_stream,
                                            PostgresqlTypeRange::StdPrimitiveI64AsInt8 => int_range_normalize_token_stream,
                                            PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate => {
                                                let generate_dot_succ_opt_expect_token_stream = |id: &dyn std::fmt::Display| {
                                                    let id_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&id);
                                                    quote::quote! {.succ_opt().expect(#id_double_quotes_token_stream)}
                                                };
                                                let generate_included_start_succ_opt_token_stream = |id: &dyn std::fmt::Display| {
                                                    let dot_succ_opt_expect_token_stream = generate_dot_succ_opt_expect_token_stream(&id);
                                                    quote::quote! {#included_upper_camel_case(#start_snake_case #dot_succ_opt_expect_token_stream)}
                                                };
                                                let generate_excluded_end_succ_opt_token_stream = |id: &dyn std::fmt::Display| {
                                                    let dot_succ_opt_expect_token_stream = generate_dot_succ_opt_expect_token_stream(&id);
                                                    quote::quote! {#excluded_upper_camel_case(#end_snake_case #dot_succ_opt_expect_token_stream)}
                                                };
                                                generate_range_match_token_stream(
                                                    &generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &quote::quote! {#excluded_upper_camel_case(#end_snake_case.succ_opt().expect("9ebce3b4-4ca7-4ff5-8b7a-a3539125bba0"))}),
                                                    &if_equal_unbounded_unbounded_or_included_excluded_token_stream,
                                                    &sqlx_postgres_types_pg_range_included_unbounded_token_stream,
                                                    &generate_if_start_end_equal_token_stream(
                                                        &sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream,
                                                        &generate_sqlx_postgres_types_pg_range_token_stream(&generate_included_start_succ_opt_token_stream(&"98a0357b-d21a-4949-a101-c641528d2376"), &generate_excluded_end_succ_opt_token_stream(&"fe53a6b9-2d7e-4605-9f5a-7f5c21cc01e6")),
                                                    ),
                                                    &generate_if_start_end_equal_token_stream(&sqlx_postgres_types_pg_range_unbounded_unbounded_token_stream, &generate_sqlx_postgres_types_pg_range_token_stream(&generate_included_start_succ_opt_token_stream(&"d8a26635-c478-4a2a-acf4-bf1765702889"), &excluded_end_token_stream)),
                                                    &generate_sqlx_postgres_types_pg_range_token_stream(&generate_included_start_succ_opt_token_stream(&"9811c7c7-d7f5-4fb7-9d25-affb0bd4f5fb"), &unbounded_upper_camel_case),
                                                    &generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &generate_excluded_end_succ_opt_token_stream(&"d6288f19-0a24-42ad-9e69-36036d9f2c1d")),
                                                    &sqlx_postgres_types_pg_range_unbounded_excluded_token_stream,
                                                )
                                            }
                                            PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => range_match_timestamp_and_timestamp_tz_token_stream,
                                            PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => range_match_timestamp_and_timestamp_tz_token_stream,
                                        }
                                    } else {
                                        quote::quote! {#value_snake_case}
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_read_ident_origin_token_stream(&quote::quote! {
                                    match #value_snake_case.0.0 {
                                        Some(#value_snake_case) => Some(
                                            <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlType>::normalize(
                                                #ident_standart_not_null_read_upper_camel_case(#value_snake_case)
                                            ).0
                                        ),
                                        None => None
                                    }
                                }),
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => generate_ident_read_ident_origin_token_stream(&quote::quote! {
                                    #value_snake_case.0.0.into_iter().map(|#element_snake_case|{
                                        <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlType>::normalize(
                                            #ident_standart_not_null_read_upper_camel_case(#element_snake_case)
                                        ).0
                                    }).collect()
                                }),
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => generate_ident_read_ident_origin_token_stream(&{
                                    let ident_standart_nullable_upper_camel_case = generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::Nullable, &PostgresqlTypePattern::Standart);
                                    let ident_array_standart_nullable_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_standart_nullable_upper_camel_case);
                                    quote::quote! {
                                        #value_snake_case.0.0.into_iter().map(|#element_snake_case|{
                                            <#ident_standart_nullable_upper_camel_case as postgresql_crud_common::PostgresqlType>::normalize(
                                                #ident_array_standart_nullable_read_upper_camel_case(#element_snake_case)
                                            ).0
                                        }).collect()
                                    }
                                }),
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => generate_ident_read_ident_origin_token_stream(&{
                                    let ident_array_dimension1_not_null_not_null_upper_camel_case = generate_ident_token_stream(
                                        postgresql_type,
                                        &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        &PostgresqlTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        },
                                    );
                                    let ident_array_dimension1_not_null_not_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_array_dimension1_not_null_not_null_upper_camel_case);
                                    quote::quote! {
                                        match #value_snake_case.0.0 {
                                            Some(#value_snake_case) => Some(<#ident_array_dimension1_not_null_not_null_upper_camel_case as postgresql_crud_common::PostgresqlType>::normalize(
                                                #ident_array_dimension1_not_null_not_null_read_upper_camel_case(#value_snake_case),
                                            ).0),
                                            None => None,
                                        }
                                    }
                                }),
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => generate_ident_read_ident_origin_token_stream(&{
                                    let ident_array_dimension1_not_null_nullable_upper_camel_case = generate_ident_token_stream(
                                        postgresql_type,
                                        &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        &PostgresqlTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::Nullable,
                                        },
                                    );
                                    let ident_array_dimension1_not_null_nullable_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident_array_dimension1_not_null_nullable_upper_camel_case);
                                    quote::quote! {
                                        match #value_snake_case.0.0 {
                                            Some(#value_snake_case) => Some(<#ident_array_dimension1_not_null_nullable_upper_camel_case as postgresql_crud_common::PostgresqlType>::normalize(
                                                #ident_array_dimension1_not_null_nullable_read_upper_camel_case(#value_snake_case),
                                            ).0),
                                            None => None,
                                        }
                                    }
                                }),
                            },
                        }
                    },
                    &ident_read_only_ids_upper_camel_case,
                    &postgresql_crud_macros_common::SelectOnlyIdsIsPrimaryKeyUnderscore::False,
                    //todo reuse select_only_ids_query_part and select_only_updated_ids_query_part code
                    &{
                        let std_string_string_default_token_stream = quote::quote! {std::string::String::default()};
                        if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                            && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                            && let CanBePrimaryKey::True = &can_be_primary_key
                        {
                            quote::quote! {
                                if is_primary_key {
                                    format!("{column},")
                                }
                                else {
                                    #std_string_string_default_token_stream
                                }
                            }
                        } else {
                            std_string_string_default_token_stream
                        }
                    },
                    &ident_read_inner_upper_camel_case,
                    &{
                        let generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlType>::into_inner(
                                    #ident_standart_not_null_read_upper_camel_case(#content_token_stream)
                                )
                            }
                        };
                        let value_dot_zero_token_stream = quote::quote! {#value_snake_case.0};
                        let value_dot_zero_dot_zero_token_stream = quote::quote! {#value_dot_zero_token_stream.0};
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_pg_range_conversion_token_stream(&value_dot_zero_dot_zero_token_stream, &value_snake_case)
                                    } else {
                                        value_dot_zero_dot_zero_token_stream
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_snake_case)
                                    } else {
                                        value_dot_zero_token_stream.clone()
                                    };
                                    quote::quote! {
                                        match #value_dot_zero_dot_zero_token_stream {
                                            Some(#value_snake_case) => Some(#content_token_stream),
                                            None => None
                                        }
                                    }
                                }
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&element_snake_case)
                                    } else {
                                        quote::quote! {#element_snake_case.0}
                                    };
                                    quote::quote! {
                                        #value_dot_zero_dot_zero_token_stream.into_iter().map(|#element_snake_case|#content_token_stream).collect()
                                    }
                                }
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_snake_case)
                                    } else {
                                        value_dot_zero_token_stream.clone()
                                    };
                                    quote::quote! {
                                        #value_dot_zero_dot_zero_token_stream.into_iter().map(|#element_snake_case| match #element_snake_case.0 {
                                            Some(#value_snake_case) => Some(#content_token_stream),
                                            None => None
                                        }).collect()
                                    }
                                }
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&element_snake_case)
                                    } else {
                                        quote::quote! {#element_snake_case.0}
                                    };
                                    quote::quote! {
                                        match #value_dot_zero_dot_zero_token_stream {
                                            Some(#value_snake_case) => Some(#value_dot_zero_token_stream.into_iter().map(|element|#content_token_stream).collect()),
                                            None => None
                                        }
                                    }
                                }
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => {
                                    let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                        generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&value_snake_case)
                                    } else {
                                        value_dot_zero_token_stream.clone()
                                    };
                                    quote::quote! {
                                        match #value_dot_zero_dot_zero_token_stream {
                                            Some(#value_snake_case) => Some(#value_dot_zero_token_stream.into_iter().map(|#element_snake_case| match #element_snake_case.0 {
                                                Some(#value_snake_case) => Some(#content_token_stream),
                                                None => None
                                            }).collect()),
                                            None => None
                                        }
                                    }
                                }
                            },
                        }
                    },
                    &ident_update_upper_camel_case,
                    &ident_update_for_query_upper_camel_case,
                    &postgresql_crud_macros_common::UpdateQueryPartValueUnderscore::True,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetAccumulatorUnderscore::True,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetTargetUnderscore::True,
                    &postgresql_crud_macros_common::UpdateQueryPartJsonbSetPathUnderscore::True,
                    &typical_query_part_token_stream,
                    &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                    &typical_query_bind_token_stream,
                    &postgresql_crud_macros_common::SelectOnlyUpdatedIdsQueryPartIsPrimaryKeyUnderscore::False,
                    &{
                        let ok_std_string_string_default_token_stream = quote::quote! {Ok(std::string::String::default())};
                        if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                            && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                            && let CanBePrimaryKey::True = &can_be_primary_key
                        {
                            quote::quote! {
                                if is_primary_key {
                                    Ok(format!("{column},"))
                                }
                                else {
                                    #ok_std_string_string_default_token_stream
                                }
                            }
                        } else {
                            ok_std_string_string_default_token_stream
                        }
                    },
                    &postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                    &quote::quote!{Ok(#query_snake_case)}
                )
            };
            let impl_postgresql_type_test_cases_for_ident_token_stream = {
                let generate_read_or_update_new_or_try_new_unwraped_for_test_token_stream = |read_or_update: &postgresql_crud_macros_common::ReadOrUpdate| {
                    let read_or_update_upper_camel_case = read_or_update.upper_camel_case();
                    let content_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                        quote::quote! {#try_new_snake_case(#value_snake_case).unwrap()}
                    } else {
                        quote::quote! {#new_snake_case(#value_snake_case)}
                    };
                    quote::quote! {<#self_upper_camel_case::#element_upper_camel_case
                        as
                    #postgresql_crud_macros_common_import_path_postgresql_crud_common::#postgresql_type_upper_camel_case>::#read_or_update_upper_camel_case:: #content_token_stream}
                };
                enum IsNeedToUseInto {
                    True,
                    False
                }
                let generate_standart_not_null_test_case_handle_token_stream = |is_need_to_use_into: &IsNeedToUseInto|{
                    let generate_range_read_inner_vec_vec_token_stream = |
                        min_token_stream: &dyn quote::ToTokens,
                        negative_less_typical_token_stream: &dyn quote::ToTokens,
                        negative_more_typical_token_stream: &dyn quote::ToTokens,
                        near_zero_token_stream: &dyn quote::ToTokens,
                        positive_less_typical_token_stream: &dyn quote::ToTokens,
                        positive_more_typical_token_stream: &dyn quote::ToTokens,
                        max_token_stream: &dyn quote::ToTokens
                    | {
                        quote::quote! {
                            let min = #min_token_stream;
                            let negative_less_typical = #negative_less_typical_token_stream;
                            let negative_more_typical = #negative_more_typical_token_stream;
                            let near_zero = #near_zero_token_stream;
                            let positive_less_typical = #positive_less_typical_token_stream;
                            let positive_more_typical = #positive_more_typical_token_stream;
                            let max = #max_token_stream;
                            vec![
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min.clone()), end: std::ops::Bound::Included(min.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(negative_less_typical.clone()), end: std::ops::Bound::Included(negative_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(near_zero.clone()), end: std::ops::Bound::Included(near_zero.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(positive_less_typical.clone()), end: std::ops::Bound::Included(positive_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(max.clone()), end: std::ops::Bound::Included(max.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min.clone()), end: std::ops::Bound::Included(max.clone())},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min.clone()), end: std::ops::Bound::Excluded(min.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(negative_less_typical.clone()), end: std::ops::Bound::Excluded(negative_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(near_zero.clone()), end: std::ops::Bound::Excluded(near_zero.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(positive_less_typical.clone()), end: std::ops::Bound::Excluded(positive_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(max.clone()), end: std::ops::Bound::Excluded(max.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min.clone()), end: std::ops::Bound::Excluded(max.clone())},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(negative_less_typical.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(near_zero.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(positive_less_typical.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(max.clone()), end: std::ops::Bound::Unbounded},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min.clone()), end: std::ops::Bound::Included(min.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(negative_less_typical.clone()), end: std::ops::Bound::Included(negative_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(near_zero.clone()), end: std::ops::Bound::Included(near_zero.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(positive_less_typical.clone()), end: std::ops::Bound::Included(positive_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(max.clone()), end: std::ops::Bound::Included(max.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min.clone()), end: std::ops::Bound::Included(max.clone())},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min.clone()), end: std::ops::Bound::Excluded(min.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(negative_less_typical.clone()), end: std::ops::Bound::Excluded(negative_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(near_zero.clone()), end: std::ops::Bound::Excluded(near_zero.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(positive_less_typical.clone()), end: std::ops::Bound::Excluded(positive_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(max.clone()), end: std::ops::Bound::Excluded(max.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min.clone()), end: std::ops::Bound::Excluded(max.clone())},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(negative_less_typical.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(near_zero.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(positive_less_typical.clone()), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(max.clone()), end: std::ops::Bound::Unbounded},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(min.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(negative_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(near_zero.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(positive_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(max.clone())},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(min.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(negative_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(near_zero.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(positive_more_typical.clone())},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(max.clone())},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded},
                            ]
                        }
                    };
                    let generate_int_pgrange_read_inner_vec_vec_token_stream = |int_range_type: &IntRangeType| {
                        let range_inner_type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                        generate_range_read_inner_vec_vec_token_stream(&quote::quote! {#range_inner_type_token_stream::MIN}, &quote::quote! {-20}, &quote::quote! {-10}, &quote::quote! {0}, &quote::quote! {10}, &quote::quote! {20}, &quote::quote! {#range_inner_type_token_stream::MAX - 1})
                    };
                    let empty_vec_token_stream = quote::quote! {vec![]};
                    let (sqlx_types_chrono_naive_time_min_token_stream, sqlx_types_chrono_naive_time_ten_token_stream, sqlx_types_chrono_naive_time_twenty_token_stream, sqlx_types_chrono_naive_time_max_token_stream) = {
                        let generate_sqlx_types_chrono_naive_time_from_hms_micro_opt_token_stream = |parameters_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {sqlx::types::chrono::NaiveTime::from_hms_micro_opt(#parameters_token_stream).unwrap()}
                        };
                        (
                            generate_sqlx_types_chrono_naive_time_from_hms_micro_opt_token_stream(&quote::quote! {0,0,0,0}),
                            generate_sqlx_types_chrono_naive_time_from_hms_micro_opt_token_stream(&quote::quote! {10,10,10,10}),
                            generate_sqlx_types_chrono_naive_time_from_hms_micro_opt_token_stream(&quote::quote! {20,20,20,20}),
                            generate_sqlx_types_chrono_naive_time_from_hms_micro_opt_token_stream(&quote::quote! {23,59,59,999_999}),
                        )
                    };
                    let (
                        sqlx_types_chrono_naive_date_min_token_stream,
                        sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                        sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                        sqlx_types_chrono_naive_date_near_zero_token_stream,
                        sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                        sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                        sqlx_types_chrono_naive_date_max_token_stream,
                    ) = {
                        let generate_sqlx_types_chrono_naive_date_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {sqlx::types::chrono::NaiveDate::#content_token_stream}
                        };
                        let generate_from_ymd_opt_unwrap_token_stream = |parameters_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {from_ymd_opt(#parameters_token_stream).unwrap()}
                        };
                        (
                            generate_sqlx_types_chrono_naive_date_token_stream(&generate_from_ymd_opt_unwrap_token_stream(&quote::quote! {-4713, 12, 31})),
                            generate_sqlx_types_chrono_naive_date_token_stream(&generate_from_ymd_opt_unwrap_token_stream(&quote::quote! {-2000, 1, 1})),
                            generate_sqlx_types_chrono_naive_date_token_stream(&generate_from_ymd_opt_unwrap_token_stream(&quote::quote! {-1000, 1, 1})),
                            generate_sqlx_types_chrono_naive_date_token_stream(&generate_from_ymd_opt_unwrap_token_stream(&quote::quote! {0, 1, 1})),
                            generate_sqlx_types_chrono_naive_date_token_stream(&generate_from_ymd_opt_unwrap_token_stream(&quote::quote! {1000, 1, 1})),
                            generate_sqlx_types_chrono_naive_date_token_stream(&generate_from_ymd_opt_unwrap_token_stream(&quote::quote! {2000, 1, 1})),
                            generate_sqlx_types_chrono_naive_date_token_stream(&quote::quote! {MAX}),
                        )
                    };
                    let sqlx_types_chrono_naive_date_time_min_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_min_token_stream,
                        #sqlx_types_chrono_naive_time_min_token_stream
                    });
                    let sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                        #sqlx_types_chrono_naive_time_twenty_token_stream,
                    });
                    let sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                        #sqlx_types_chrono_naive_time_ten_token_stream,
                    });
                    let sqlx_types_chrono_naive_date_time_near_zero_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_near_zero_token_stream,
                        #sqlx_types_chrono_naive_time_min_token_stream
                    });
                    let sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                        #sqlx_types_chrono_naive_time_ten_token_stream,
                    });
                    let sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                        #sqlx_types_chrono_naive_time_twenty_token_stream,
                    });
                    let sqlx_types_chrono_naive_date_time_max_token_stream = generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                        #sqlx_types_chrono_naive_date_max_token_stream,
                        #sqlx_types_chrono_naive_time_max_token_stream
                    });

                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_min_token_stream);
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream);
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream);
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_near_zero_token_stream);
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream);
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream);
                    let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&sqlx_types_chrono_naive_date_time_max_token_stream);
                    //is_need_to_use_into
                    let generate_typical_test_cases_vec_token_stream = |value: &dyn quote::ToTokens|{
                        let content_token_stream = match &is_need_to_use_into {
                            IsNeedToUseInto::True => quote::quote!{.into()},
                            IsNeedToUseInto::False => proc_macro2::TokenStream::new(),
                        };
                        quote::quote!{postgresql_crud_common::#value()#content_token_stream}
                    };
                    match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_primitive_i16_test_cases_vec}),
                        PostgresqlType::StdPrimitiveI32AsInt4 => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_primitive_i32_test_cases_vec}),
                        PostgresqlType::StdPrimitiveI64AsInt8 => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_primitive_i64_test_cases_vec}),
                        PostgresqlType::StdPrimitiveF32AsFloat4 => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_primitive_f32_test_cases_vec}),
                        PostgresqlType::StdPrimitiveF64AsFloat8 => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_primitive_f64_test_cases_vec}),
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => empty_vec_token_stream,
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => empty_vec_token_stream,
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => empty_vec_token_stream,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => quote::quote! {
                            postgresql_crud_common::std_primitive_i64_test_cases_vec().into_iter().map(|#element_snake_case|
                                #inner_type_standart_not_null_token_stream(#element_snake_case)
                            ).collect::<std::vec::Vec<#inner_type_standart_not_null_token_stream>>()
                        },
                        PostgresqlType::StdPrimitiveBoolAsBool => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_primitive_bool_test_cases_vec}),
                        PostgresqlType::StdStringStringAsText => generate_typical_test_cases_vec_token_stream(&quote::quote!{std_string_string_test_cases_vec}),
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => quote::quote! {vec![
                            vec![],
                            (0u8..=255).collect(),
                            vec![0; 1024],
                            vec![0; 1024 * 1024 * 2],
                        ]},
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => quote::quote! {vec![
                            #sqlx_types_chrono_naive_time_min_token_stream,
                            #sqlx_types_chrono_naive_time_ten_token_stream,
                            #sqlx_types_chrono_naive_time_twenty_token_stream,
                            #sqlx_types_chrono_naive_time_max_token_stream,
                        ]},
                        PostgresqlType::SqlxTypesTimeTimeAsTime => {
                            let sqlx_types_time_time_from_hms_micro_min_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {0,0,0,0});
                            let sqlx_types_time_time_from_hms_micro_ten_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {10,10,10,10});
                            let sqlx_types_time_time_from_hms_micro_twenty_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {20,20,20,20});
                            let sqlx_types_time_time_from_hms_micro_max_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {23,59,59,999_999});
                            quote::quote! {vec![
                                #sqlx_types_time_time_from_hms_micro_min_unwrap_token_stream,
                                #sqlx_types_time_time_from_hms_micro_ten_unwrap_token_stream,
                                #sqlx_types_time_time_from_hms_micro_twenty_unwrap_token_stream,
                                #sqlx_types_time_time_from_hms_micro_max_unwrap_token_stream,
                            ]}
                        }
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => {
                            let min_token_stream = quote::quote!{MIN};
                            let max_token_stream = quote::quote!{MAX};
                            let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
                            let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
                            let std_primitive_i32_min_token_stream = quote::quote! {#std_primitive_i32_token_stream::#min_token_stream};
                            let std_primitive_i32_max_token_stream = quote::quote! {#std_primitive_i32_token_stream::#max_token_stream};
                            let generate_sqlx_postgres_types_pg_interval_token_stream = |
                                months_token_stream: &dyn quote::ToTokens,
                                days_token_stream: &dyn quote::ToTokens,
                                microseconds_token_stream: &dyn quote::ToTokens,
                            |quote::quote! {sqlx::postgres::types::PgInterval {
                                months: #months_token_stream,
                                days: #days_token_stream,
                                microseconds: #microseconds_token_stream
                            }};
                            let min_token_stream = generate_sqlx_postgres_types_pg_interval_token_stream(
                                &std_primitive_i32_min_token_stream,
                                &std_primitive_i32_min_token_stream,
                                &quote::quote! {#std_primitive_i64_token_stream::#min_token_stream}
                            );
                            let max_token_stream = generate_sqlx_postgres_types_pg_interval_token_stream(
                                &std_primitive_i32_max_token_stream,
                                &std_primitive_i32_max_token_stream,
                                &quote::quote! {#std_primitive_i64_token_stream::#max_token_stream}
                            );
                            quote::quote! {vec![
                                #min_token_stream,
                                #max_token_stream
                            ]}
                        },
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => quote::quote! {vec![
                            #sqlx_types_chrono_naive_date_min_token_stream,
                            #sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                            #sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                            #sqlx_types_chrono_naive_date_near_zero_token_stream,
                            #sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                            #sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                            #sqlx_types_chrono_naive_date_max_token_stream,
                        ]},
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {vec![
                            #sqlx_types_chrono_naive_date_time_min_token_stream,
                            #sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream,
                            #sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream,
                            #sqlx_types_chrono_naive_date_time_near_zero_token_stream,
                            #sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream,
                            #sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream,
                            #sqlx_types_chrono_naive_date_time_max_token_stream,
                        ]},
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote! {vec![
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_token_stream,
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_token_stream,
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_token_stream,
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_token_stream,
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_token_stream,
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_token_stream,
                            #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_token_stream,
                        ]},
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => quote::quote! {vec![]},
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => quote::quote! {vec![
                            sqlx::types::Uuid::new_v4()
                        ]},
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote! {vec![
                            <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").unwrap(),
                            <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").unwrap(),
                            <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").unwrap(),
                            <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").unwrap(),
                            <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").unwrap(),
                            <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").unwrap(),
                            sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(192, 168, 0, 0), 24).unwrap()),
                            sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(10, 0, 0, 0), 8).unwrap()),
                            sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(127, 0, 0, 1), 32).unwrap()),
                            sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_snake_case(std::net::Ipv6Addr::LOCALHOST, 128).unwrap()),
                            sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_snake_case("2001:db8::".parse().unwrap(), 32).unwrap()),
                        ]},
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {vec![
                            sqlx::types::mac_address::MacAddress::#new_snake_case([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                            sqlx::types::mac_address::MacAddress::#new_snake_case([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                            sqlx::types::mac_address::MacAddress::#new_snake_case([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                            sqlx::types::mac_address::MacAddress::#new_snake_case([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                            sqlx::types::mac_address::MacAddress::#new_snake_case([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                            sqlx::types::mac_address::MacAddress::#new_snake_case([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                        ]},
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_int_pgrange_read_inner_vec_vec_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_int_pgrange_read_inner_vec_vec_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => generate_range_read_inner_vec_vec_token_stream(
                            &sqlx_types_chrono_naive_date_min_token_stream,
                            &sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                            &sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                            &sqlx_types_chrono_naive_date_near_zero_token_stream,
                            &sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                            &sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                            &quote::quote! {#sqlx_types_chrono_naive_date_max_token_stream.pred_opt().unwrap()},
                        ),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => generate_range_read_inner_vec_vec_token_stream(
                            &sqlx_types_chrono_naive_date_time_min_token_stream,
                            &sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream,
                            &sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream,
                            &sqlx_types_chrono_naive_date_time_near_zero_token_stream,
                            &sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream,
                            &sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream,
                            &sqlx_types_chrono_naive_date_time_max_token_stream,
                        ),
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_range_read_inner_vec_vec_token_stream(
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_token_stream,
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_token_stream,
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_token_stream,
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_token_stream,
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_token_stream,
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_token_stream,
                            &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_token_stream,
                        ),
                    }
                };
                postgresql_crud_macros_common::generate_impl_postgresql_type_test_cases_for_ident_token_stream(
                    &quote::quote! {#[cfg(feature = "test-utils")]},
                    &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                    &ident_inner_type_token_stream,
                    &ident,
                    &match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream = generate_standart_not_null_test_case_handle_token_stream(&IsNeedToUseInto::True);
                                quote::quote! {vec![{#content_token_stream}]}
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                quote::quote! {
                                    let mut #acc_snake_case = vec![];
                                    for element0 in <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlTypeTestCases>::#read_inner_vec_vec_snake_case(&#read_only_ids_snake_case) {
                                        for element1 in element0 {
                                            #acc_snake_case.push(vec![Some(element1)]);
                                        }
                                    }
                                    #acc_snake_case.push(vec![None]);
                                    #acc_snake_case
                                }
                            }
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => quote::quote! {
                                let mut #acc_snake_case = vec![];
                                for element in <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlTypeTestCases>::#read_inner_vec_vec_snake_case(&#read_only_ids_snake_case) {
                                    #acc_snake_case.push(element);
                                }
                                vec![#acc_snake_case]
                            },
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => {
                                quote::quote! {
                                    let mut #acc_snake_case = vec![];
                                    for element in <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlTypeTestCases>::#read_inner_vec_vec_snake_case(&#read_only_ids_snake_case) {
                                        let mut current_acc = vec![];
                                        for current_element in element {
                                            current_acc.push(Some(current_element));
                                        }
                                        #acc_snake_case.push(current_acc);
                                    }
                                    vec![#acc_snake_case]
                                }
                            }
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => quote::quote! {
                                let mut #acc_snake_case = vec![];
                                for element in <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlTypeTestCases>::#read_inner_vec_vec_snake_case(&#read_only_ids_snake_case) {
                                    #acc_snake_case.push(Some(element));
                                }
                                vec![#acc_snake_case]
                            },
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => quote::quote! {
                                let mut #acc_snake_case = vec![];
                                for element0 in <#ident_standart_not_null_upper_camel_case as postgresql_crud_common::PostgresqlTypeTestCases>::#read_inner_vec_vec_snake_case(&#read_only_ids_snake_case) {
                                    let mut current_acc = vec![];
                                    for element1 in element0 {
                                        current_acc.push(Some(element1));
                                    }
                                    #acc_snake_case.push(Some(current_acc));
                                }
                                vec![#acc_snake_case]
                            },
                        },
                    },
                    &generate_read_or_update_new_or_try_new_unwraped_for_test_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Read),
                    &generate_read_or_update_new_or_try_new_unwraped_for_test_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Update),
                    &quote::quote! {todo!()},
                    &{
                        let ident_read_value_clone_token_stream = quote::quote! {#ident_read_upper_camel_case(value.clone())};
                        let none_token_stream = quote::quote! {None};
                        if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                            && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                        {
                            match &postgresql_type {
                                PostgresqlType::StdPrimitiveI16AsInt2 => none_token_stream,
                                PostgresqlType::StdPrimitiveI32AsInt4 => none_token_stream,
                                PostgresqlType::StdPrimitiveI64AsInt8 => none_token_stream,
                                PostgresqlType::StdPrimitiveF32AsFloat4 => none_token_stream,
                                PostgresqlType::StdPrimitiveF64AsFloat8 => none_token_stream,
                                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => ident_read_value_clone_token_stream,
                                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => ident_read_value_clone_token_stream,
                                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => ident_read_value_clone_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => none_token_stream,
                                PostgresqlType::StdPrimitiveBoolAsBool => none_token_stream,
                                PostgresqlType::StdStringStringAsText => none_token_stream,
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => none_token_stream,
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => none_token_stream,
                                PostgresqlType::SqlxTypesTimeTimeAsTime => none_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => none_token_stream,
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => none_token_stream,
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => none_token_stream,
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => none_token_stream,
                                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => ident_read_value_clone_token_stream,
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => none_token_stream,
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => none_token_stream,
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => none_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => none_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => none_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => none_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => none_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => none_token_stream,
                            }
                        } else {
                            none_token_stream
                        }
                    },
                    &if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                        && let CanBePrimaryKey::True = &can_be_primary_key
                    {
                        quote::quote! {
                            Some(postgresql_crud_common::Value {
                                value: value.clone()
                            })
                        }
                    } else {
                        quote::quote!{
                            Some(postgresql_crud_common::Value {
                                value: #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            })
                        }
                    },
                    &quote::quote!{match #option_update_snake_case {
                        Some(#value_snake_case) => #ident_read_upper_camel_case(#value_snake_case),
                        None => #read_snake_case
                    }},
                    &{
                        let generate_acc_content_token_stream = |not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, ident_token_stream: &dyn quote::ToTokens| {
                            let (
                                new_or_try_new_content_token_stream,
                                maybe_acc_push_none_token_stream
                            ) = match (&not_null_or_nullable, postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok()) {
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, true) => (
                                    quote::quote!{try_new(vec![#element_snake_case.into()]).expect("error adbae6b3-1542-4f81-89bf-48a9b895b488")},
                                    proc_macro2::TokenStream::new()
                                ),
                                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, false) => (
                                    quote::quote!{new(vec![#element_snake_case.into()])},
                                    proc_macro2::TokenStream::new()
                                ),
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, true) => (
                                    quote::quote!{try_new(Some(#element_snake_case.into())).expect("error b244d498-527d-4332-98c9-770d27e7af35")},
                                    quote::quote!{#acc_snake_case.push(<#ident as postgresql_crud_common::PostgresqlType>::Create::try_new(None).expect("error 31878971-17fc-4526-ab01-42c8332e641f"));}
                                ),
                                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, false) => (
                                    quote::quote!{new(Some(#element_snake_case.into()))},
                                    quote::quote!{#acc_snake_case.push(<#ident as postgresql_crud_common::PostgresqlType>::Create::new(None));}
                                ),
                            };
                            quote::quote! {
                                let mut #acc_snake_case = vec![];
                                for #element_snake_case in <#ident_token_stream as postgresql_crud_common::PostgresqlTypeTestCases>::#create_vec_snake_case() {
                                    #acc_snake_case.push(<#ident as postgresql_crud_common::PostgresqlType>::Create::#new_or_try_new_content_token_stream);
                                }
                                #maybe_acc_push_none_token_stream
                                #acc_snake_case
                            }
                        };
                        match &postgresql_type_pattern {
                            PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    let new_or_try_new_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                                        quote::quote!{try_new(#element_snake_case).expect("error 941bd15c-a751-45e7-8266-f17df4ee00aa")}
                                    }
                                    else {
                                        quote::quote!{new(#element_snake_case)}
                                    };
                                    match &can_be_primary_key {
                                        CanBePrimaryKey::True => quote::quote!{vec![]},//todo maybe wrong
                                        CanBePrimaryKey::False => {
                                            let content_token_stream = generate_standart_not_null_test_case_handle_token_stream(&IsNeedToUseInto::False);
                                            quote::quote!{
                                                #content_token_stream.into_iter().map(|#element_snake_case|
                                                    <#ident as postgresql_crud_common::PostgresqlType>::Create::#new_or_try_new_token_stream
                                                ).collect()
                                            }
                                        },
                                    }
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_acc_content_token_stream(
                                    &not_null_or_nullable,
                                    &generate_ident_token_stream(
                                        &postgresql_type,
                                        &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                        &PostgresqlTypePattern::Standart,
                                    )
                                )
                            },
                            PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_acc_content_token_stream(
                                &not_null_or_nullable,
                                &generate_ident_token_stream(
                                    &postgresql_type,
                                    &match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => dimension1_not_null_or_nullable.clone(),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                    },
                                    &match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => PostgresqlTypePattern::Standart,
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => PostgresqlTypePattern::ArrayDimension1 {
                                            dimension1_not_null_or_nullable: dimension1_not_null_or_nullable.clone()
                                        },
                                    },
                                )
                            )
                        }
                    },
                    &{
                        let content_token_stream = if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                            && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                            && let CanBePrimaryKey::True = &can_be_primary_key
                        {
                            quote::quote! {#read_only_ids_snake_case.expect("error 5f763c94-ada9-4459-b162-4b15e32c8b04")}//todo maybe remove expect
                        } else {
                            quote::quote!{#ident_read_upper_camel_case(#create_snake_case)}
                        };
                        quote::quote! {Some(postgresql_crud_common::Value { #value_snake_case: #content_token_stream })}
                    }
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
                #ident_read_only_ids_token_stream
                #ident_read_inner_token_stream
                #ident_update_token_stream
                #ident_update_for_query_token_stream
                #impl_postgresql_type_for_ident_token_stream
                #impl_postgresql_type_test_cases_for_ident_token_stream
            };
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
        let columns_token_stream = columns_token_stream.into_iter().map(|element| element.parse::<proc_macro2::TokenStream>().unwrap()).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
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
