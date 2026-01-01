#[proc_macro]
pub fn generate_postgresql_types(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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
    impl From<&PostgresqlType> for RustTypeName {
        fn from(value: &PostgresqlType) -> Self {
            match &value {
                PostgresqlType::StdPrimitiveF32AsFloat4 => Self::StdPrimitiveF32,
                PostgresqlType::StdPrimitiveF64AsFloat8 => Self::StdPrimitiveF64,
                PostgresqlType::StdPrimitiveI16AsInt2 | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => Self::StdPrimitiveI16,
                PostgresqlType::StdPrimitiveI32AsInt4 | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => Self::StdPrimitiveI32,
                PostgresqlType::StdPrimitiveI64AsInt8 | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => Self::StdPrimitiveI64,
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
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::SqlxTypesUuidUuid,
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
    impl From<&PostgresqlType> for PostgresqlTypeName {
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
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime | PostgresqlType::SqlxTypesTimeTimeAsTime => Self::Time,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
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
        // SqlxTypesBigDecimalAsNumeric, remove coz dont know how to deserialize with scale i64
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
    fn wrap_into_sqlx_postgres_types_pg_range_stringified(value: &dyn std::fmt::Display) -> String {
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
        const fn can_be_nullable(&self) -> CanBeNullable {
            match &self {
                Self::StdPrimitiveI16AsInt2
                | Self::StdPrimitiveI32AsInt4
                | Self::StdPrimitiveI64AsInt8
                | Self::StdPrimitiveF32AsFloat4
                | Self::StdPrimitiveF64AsFloat8
                | Self::SqlxPostgresTypesPgMoneyAsMoney
                | Self::StdPrimitiveBoolAsBool
                | Self::StdStringStringAsText
                | Self::StdVecVecStdPrimitiveU8AsBytea
                | Self::SqlxTypesChronoNaiveTimeAsTime
                | Self::SqlxTypesTimeTimeAsTime
                | Self::SqlxPostgresTypesPgIntervalAsInterval
                | Self::SqlxTypesChronoNaiveDateAsDate
                | Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | Self::SqlxTypesUuidUuidAsUuidInitializedByClient
                | Self::SqlxTypesIpnetworkIpNetworkAsInet
                | Self::SqlxTypesMacAddressMacAddressAsMacAddr
                | Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                | Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeNullable::True,
                Self::StdPrimitiveI16AsSmallSerialInitializedByPostgresql | Self::StdPrimitiveI32AsSerialInitializedByPostgresql | Self::StdPrimitiveI64AsBigSerialInitializedByPostgresql | Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBeNullable::False,
            }
        }
        const fn can_be_an_array_element(&self) -> CanBeAnArrayElement {
            match &self {
                Self::StdPrimitiveI16AsInt2
                | Self::StdPrimitiveI32AsInt4
                | Self::StdPrimitiveI64AsInt8
                | Self::StdPrimitiveF32AsFloat4
                | Self::StdPrimitiveF64AsFloat8
                | Self::SqlxPostgresTypesPgMoneyAsMoney
                | Self::StdPrimitiveBoolAsBool
                | Self::StdStringStringAsText
                | Self::StdVecVecStdPrimitiveU8AsBytea
                | Self::SqlxTypesChronoNaiveTimeAsTime
                | Self::SqlxTypesTimeTimeAsTime
                | Self::SqlxPostgresTypesPgIntervalAsInterval
                | Self::SqlxTypesChronoNaiveDateAsDate
                | Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | Self::SqlxTypesUuidUuidAsUuidInitializedByClient
                | Self::SqlxTypesIpnetworkIpNetworkAsInet
                | Self::SqlxTypesMacAddressMacAddressAsMacAddr
                | Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                | Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeAnArrayElement::True,
                Self::StdPrimitiveI16AsSmallSerialInitializedByPostgresql | Self::StdPrimitiveI32AsSerialInitializedByPostgresql | Self::StdPrimitiveI64AsBigSerialInitializedByPostgresql | Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBeAnArrayElement::False,
            }
        }
    }
    impl quote::ToTokens for PostgresqlType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("cfefbb95-b0f4-44de-ba94-3e77e88daf0f")
                .to_tokens(tokens);
        }
    }
    impl From<&PostgresqlTypeRange> for PostgresqlType {
        fn from(value: &PostgresqlTypeRange) -> Self {
            match value {
                PostgresqlTypeRange::StdPrimitiveI32AsInt4 => Self::StdPrimitiveI32AsInt4,
                PostgresqlTypeRange::StdPrimitiveI64AsInt8 => Self::StdPrimitiveI64AsInt8,
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate => {
                    Self::SqlxTypesChronoNaiveDateAsDate
                }
                PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                    Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                }
                PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                    Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                }
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
    impl TryFrom<&PostgresqlType> for PostgresqlTypeRange {
        type Error = ();
        fn try_from(value: &PostgresqlType) -> Result<Self, Self::Error> {
            match &value {
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
                | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql
                | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::StdPrimitiveI32AsInt4),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::StdPrimitiveI64AsInt8),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
            }
        }
    }
    impl std::fmt::Display for PostgresqlTypeRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                naming::parameter::SelfNotNullUpperCamelCase::from_display(&PostgresqlType::from(
                    self
                ))
            )
        }
    }
    impl quote::ToTokens for PostgresqlTypeRange {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("798ccb5a-e65b-4ae9-88cd-48c8e22d79d0")
                .to_tokens(tokens);
        }
    }
    // todo reuse it(move to postgresql_macros_common) if sqlx devs will add nested array support
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
    enum PostgresqlTypePattern {
        Standart,
        ArrayDimension1 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
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
        const fn array_dimensions_number(&self) -> usize {
            match &self {
                Self::Standart => 0,
                Self::ArrayDimension1 { .. } => 1,
            }
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
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
                    fn expecting(
                        &self,
                        __f: &mut _serde::__private228::Formatter<'_>,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(__f, "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => Ok(__Field::__field0),
                            1u64 => Ok(__Field::__field1),
                            2u64 => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "postgresql_type" => Ok(__Field::__field0),
                            "not_null_or_nullable" => Ok(__Field::__field1),
                            "postgresql_type_pattern" => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"postgresql_type" => Ok(__Field::__field0),
                            b"not_null_or_nullable" => Ok(__Field::__field1),
                            b"postgresql_type_pattern" => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<PostgresqlTypeRecord>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PostgresqlTypeRecord;
                    fn expecting(
                        &self,
                        __f: &mut _serde::__private228::Formatter<'_>,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __f,
                            "struct PostgresqlTypeRecord",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let Some(__field0) =
                            _serde::de::SeqAccess::next_element::<PostgresqlType>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct PostgresqlTypeRecord with 3 elements",
                            ));
                        };
                        let Some(__field1) = _serde::de::SeqAccess::next_element::<
                            postgresql_crud_macros_common::NotNullOrNullable,
                        >(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct PostgresqlTypeRecord with 3 elements",
                            ));
                        };
                        let Some(__field2) = _serde::de::SeqAccess::next_element::<
                            PostgresqlTypePattern,
                        >(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct PostgresqlTypeRecord with 3 elements",
                            ));
                        };
                        match PostgresqlTypeRecord::try_from((__field0, __field1, __field2)) {
                            Ok(value) => Ok(value),
                            Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                        }
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: Option<PostgresqlType> = None;
                        let mut __field1: Option<postgresql_crud_macros_common::NotNullOrNullable> =
                            None;
                        let mut __field2: Option<PostgresqlTypePattern> = None;
                        while let Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if Option::is_some(&__field0) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "postgresql_type",
                                            ),
                                        );
                                    }
                                    __field0 =
                                        Some(_serde::de::MapAccess::next_value::<PostgresqlType>(
                                            &mut __map,
                                        )?);
                                }
                                __Field::__field1 => {
                                    if Option::is_some(&__field1) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "not_null_or_nullable",
                                            ),
                                        );
                                    }
                                    __field1 =
                                        Some(_serde::de::MapAccess::next_value::<
                                            postgresql_crud_macros_common::NotNullOrNullable,
                                        >(&mut __map)?);
                                }
                                __Field::__field2 => {
                                    if Option::is_some(&__field2) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "postgresql_type_pattern",
                                            ),
                                        );
                                    }
                                    __field2 =
                                        Some(_serde::de::MapAccess::next_value::<
                                            PostgresqlTypePattern,
                                        >(&mut __map)?);
                                }
                                __Field::__ignore => {
                                    let _: serde::de::IgnoredAny =
                                        _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                            &mut __map,
                                        )?;
                                }
                            }
                        }
                        let __field0_value = match __field0 {
                            Some(value) => value,
                            None => _serde::__private228::de::missing_field("postgresql_type")?,
                        };
                        let __field1_value = match __field1 {
                            Some(value) => value,
                            None => {
                                _serde::__private228::de::missing_field("not_null_or_nullable")?
                            }
                        };
                        let __field2_value = match __field2 {
                            Some(value) => value,
                            None => {
                                _serde::__private228::de::missing_field("postgresql_type_pattern")?
                            }
                        };
                        match PostgresqlTypeRecord::try_from((
                            __field0_value,
                            __field1_value,
                            __field2_value,
                        )) {
                            Ok(value) => Ok(value),
                            Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                        }
                    }
                }
                #[doc(hidden)]
                const FIELDS: &[&str] = &[
                    "postgresql_type",
                    "not_null_or_nullable",
                    "postgresql_type_pattern",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PostgresqlTypeRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Self>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl
        TryFrom<(
            PostgresqlType,
            postgresql_crud_macros_common::NotNullOrNullable,
            PostgresqlTypePattern,
        )> for PostgresqlTypeRecord
    {
        type Error = String;
        fn try_from(
            value: (
                PostgresqlType,
                postgresql_crud_macros_common::NotNullOrNullable,
                PostgresqlTypePattern,
            ),
        ) -> Result<Self, Self::Error> {
            let cant_support_nullable_variants_message = "cant support nullable variants: ";
            let cant_support_array_version_message = "cant support array_version: ";
            match &value.0.can_be_nullable() {
                CanBeNullable::True => match &value.2 {
                    PostgresqlTypePattern::Standart => Ok(Self {
                        postgresql_type: value.0,
                        not_null_or_nullable: value.1,
                        postgresql_type_pattern: value.2,
                    }),
                    PostgresqlTypePattern::ArrayDimension1 { .. } => {
                        match &value.0.can_be_an_array_element() {
                            CanBeAnArrayElement::True => Ok(Self {
                                postgresql_type: value.0,
                                not_null_or_nullable: value.1,
                                postgresql_type_pattern: value.2,
                            }),
                            CanBeAnArrayElement::False => {
                                Err(format!("{cant_support_array_version_message}{value:#?}"))
                            }
                        }
                    }
                },
                CanBeNullable::False => {
                    if let postgresql_crud_macros_common::NotNullOrNullable::Nullable = &value.1 {
                        return Err(format!(
                            "{cant_support_nullable_variants_message}{value:#?}"
                        ));
                    }
                    match &value.2 {
                        PostgresqlTypePattern::Standart => Ok(Self {
                            postgresql_type: value.0,
                            not_null_or_nullable: value.1,
                            postgresql_type_pattern: value.2,
                        }),
                        PostgresqlTypePattern::ArrayDimension1 {
                            dimension1_not_null_or_nullable,
                        } => match &value.0.can_be_an_array_element() {
                            CanBeAnArrayElement::True => match &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                    Ok(Self {
                                        postgresql_type: value.0,
                                        not_null_or_nullable: value.1,
                                        postgresql_type_pattern: value.2,
                                    })
                                }
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => Err(
                                    format!("{cant_support_nullable_variants_message}{value:#?}"),
                                ),
                            },
                            CanBeAnArrayElement::False => {
                                Err(format!("{cant_support_array_version_message}{value:#?}"))
                            }
                        },
                    }
                }
            }
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlTypesConfig {
        All,
        Concrete(Vec<PostgresqlTypeRecord>),
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
    impl TryFrom<&PostgresqlType> for PostgresqlTypeInitializationTryNew {
        type Error = ();
        fn try_from(value: &PostgresqlType) -> Result<Self, Self::Error> {
            match value {
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
                | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
                | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql
                | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresqlType::StdStringStringAsText => Ok(Self::StdStringStringAsText),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
        }
    }
    impl From<&PostgresqlTypeInitializationTryNew> for PostgresqlType {
        fn from(value: &PostgresqlTypeInitializationTryNew) -> Self {
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
        SsqlxPostgresTypesPgIntervalAsInterval, //Ssqlx instead of Sqlx - just to fix clippy lint
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
        ImplNewForDeserializeOrTryNewForDeserialize(
            PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize,
        ),
    }
    impl From<&PostgresqlType> for PostgresqlTypeDeserialize {
        fn from(value: &PostgresqlType) -> Self {
            match value {
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
                | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql
                | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::Derive,
                PostgresqlType::StdStringStringAsText => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText)),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime)),
                PostgresqlType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime)),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SsqlxPostgresTypesPgIntervalAsInterval)),
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate)),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp)),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz)),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range)),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range)),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange)),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange)),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange)),
            }
        }
    }
    use rayon::iter::IntoParallelRefIterator as _;
    use rayon::iter::ParallelIterator as _;
    panic_location::panic_location();
    let (columns_token_stream, postgresql_type_array) = {
        let current_acc = match serde_json::from_str::<GeneratePostgresqlTypesConfig>(
            &input_token_stream.to_string()
        ).expect("80485f71-4e21-4166-94df-722326c36a29") {
            GeneratePostgresqlTypesConfig::All => PostgresqlType::into_array().into_iter().fold(Vec::new(), |mut acc, postgresql_type| {
                let postgresql_type_pattern_all = PostgresqlTypePattern::into_array().into_iter().fold(Vec::new(), |mut current_acc, postgresql_type_pattern| {
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => {
                            current_acc.push(postgresql_type_pattern);
                        }
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            for dimension1_not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                current_acc.push(PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable });
                            }
                        }
                    }
                    current_acc
                });
                for postgresql_type_pattern in postgresql_type_pattern_all {
                    match &postgresql_type_pattern {
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
                                        for not_null_or_nullable in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                            acc.push(PostgresqlTypeRecord {
                                                postgresql_type: postgresql_type.clone(),
                                                not_null_or_nullable,
                                                postgresql_type_pattern: PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable },
                                            });
                                        }
                                    }
                                }
                            },
                            CanBeAnArrayElement::False => (),
                        },
                    }
                }
                acc
            }),
            GeneratePostgresqlTypesConfig::Concrete(value) => value,
        };
        {
            let mut check_acc = Vec::new();
            for element in &current_acc {
                if check_acc.contains(&element) {
                    panic!("536036f9-2511-4247-8463-6defbeb72f5c");
                } else {
                    check_acc.push(element);
                }
            }
        }
        current_acc
    }.into_iter()
    .fold(
        Vec::new(),
        |mut acc, postgresql_type_record_element| {
            use postgresql_crud_macros_common::NotNullOrNullable;
            #[derive(Clone)]
            struct PostgresqlTypeRecordHandle {
                not_null_or_nullable: NotNullOrNullable,
                postgresql_type_pattern: PostgresqlTypePattern,
            }
            fn generate_postgresql_type_record_handle_vec(
                postgresql_type_record_handle: PostgresqlTypeRecordHandle,
            ) -> Vec<PostgresqlTypeRecordHandle> {
                let generate_vec =
                    |current_postgresql_type_record_handle: PostgresqlTypeRecordHandle| {
                        let mut acc = Vec::new();
                        for element in generate_postgresql_type_record_handle_vec(
                            current_postgresql_type_record_handle,
                        ) {
                            acc.push(element);
                        }
                        acc.push(postgresql_type_record_handle.clone());
                        acc
                    };
                //same pattern was in generate_postgresql_types 21.05.2025
                match (
                    &postgresql_type_record_handle.not_null_or_nullable,
                    &postgresql_type_record_handle.postgresql_type_pattern,
                ) {
                    (NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart) => {
                        vec![postgresql_type_record_handle]
                    }
                    (NotNullOrNullable::Nullable, PostgresqlTypePattern::Standart) => {
                        generate_vec(PostgresqlTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_type_pattern: PostgresqlTypePattern::Standart,
                        })
                    }
                    (
                        NotNullOrNullable::NotNull,
                        PostgresqlTypePattern::ArrayDimension1 {
                            dimension1_not_null_or_nullable,
                        },
                    ) => generate_vec(PostgresqlTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgresql_type_pattern: PostgresqlTypePattern::Standart,
                    }),
                    (
                        NotNullOrNullable::Nullable,
                        PostgresqlTypePattern::ArrayDimension1 { .. },
                    ) => generate_vec(PostgresqlTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgresql_type_pattern: postgresql_type_record_handle
                            .postgresql_type_pattern
                            .clone(),
                    }),
                }
            }
            for postgresql_type_record_handle_element in generate_postgresql_type_record_handle_vec(PostgresqlTypeRecordHandle {
                not_null_or_nullable: postgresql_type_record_element.not_null_or_nullable,
                postgresql_type_pattern: postgresql_type_record_element.postgresql_type_pattern,
            }) {
                let postgresql_type_record = PostgresqlTypeRecord {
                    postgresql_type: postgresql_type_record_element.postgresql_type.clone(),
                    not_null_or_nullable: postgresql_type_record_handle_element
                        .not_null_or_nullable,
                    postgresql_type_pattern: postgresql_type_record_handle_element
                        .postgresql_type_pattern,
                };
                if !acc.contains(&postgresql_type_record) {
                    acc.push(postgresql_type_record);
                }
            }
            acc
        },
    )
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PostgresqlTypeRecord)>>()
    .par_iter()
    //.into_iter() //just for console prints ordering
    .map(|(index, element)| {
        enum PostgresqlTypeOrPostgresqlTypeTestCases {
            PostgresqlType,
            PostgresqlTypeTestCases,
        }
        enum CanBePrimaryKey {
            True,
            False,
        }
        enum IsStandartNotNull {
            True,
            False,
        }
        enum IsNotNullStandartCanBePrimaryKey {
            True,
            False,
        }
        enum StartOrEnd {
            Start,
            End,
        }
        enum IntRangeType {
            SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
            SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
        }
        type Handle<'lifetime> = (&'lifetime dyn quote::ToTokens, &'lifetime dyn quote::ToTokens);
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
        let postgresql_type = &element.postgresql_type;
        let not_null_or_nullable = &element.not_null_or_nullable;
        let postgresql_type_pattern = &element.postgresql_type_pattern;
        let postgresql_type_initialization_try_new_try_from_postgresql_type = PostgresqlTypeInitializationTryNew::try_from(postgresql_type);
        let postgresql_type_deserialize = PostgresqlTypeDeserialize::from(postgresql_type);

        let array_dimensions_number = postgresql_type_pattern.array_dimensions_number();

        let postgresql_type_range_try_from_postgresql_type = PostgresqlTypeRange::try_from(postgresql_type);
        let postgresql_type_range_try_from_postgresql_type_is_ok = postgresql_type_range_try_from_postgresql_type.is_ok();

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
        let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
        let read_only_ids_to_two_dimensional_vec_read_inner_snake_case = naming::ReadOnlyIdsToTwoDimensionalVecReadInnerSnakeCase;
        let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
        let create_snake_case = naming::CreateSnakeCase;
        let to_std_string_string_snake_case = naming::ToStdStringStringSnakeCase;
        let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
        let read_upper_camel_case = naming::ReadUpperCamelCase;
        let update_upper_camel_case = naming::UpdateUpperCamelCase;
        let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
        let table_type_declaration_snake_case = naming::TableTypeDeclarationSnakeCase;
        let read_only_ids_merged_with_create_into_read_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoReadSnakeCase;
        let read_only_ids_into_table_type_declaration_snake_case = naming::ReadOnlyIdsIntoTableTypeDeclarationSnakeCase;
        let read_only_ids_into_read_snake_case = naming::ReadOnlyIdsIntoReadSnakeCase;
        let read_only_ids_into_update_snake_case = naming::ReadOnlyIdsIntoUpdateSnakeCase;
        let read_into_table_type_declaration_snake_case = naming::ReadIntoTableTypeDeclarationSnakeCase;
        let equal_upper_camel_case = naming::EqualUpperCamelCase;

        let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
        let std_primitive_i16_token_stream = token_patterns::StdPrimitiveI16;
        let std_primitive_u32_token_stream = token_patterns::StdPrimitiveU32;
        let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
        let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
        let std_primitive_f32_token_stream = token_patterns::StdPrimitiveF32;
        let std_string_string_token_stream = token_patterns::StdStringString;

        let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeCall;

        let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
        let import_path_non_primary_key_postgresql_type_read_only_ids_token_stream = quote::quote! {#import_path::NonPrimaryKeyPostgresqlTypeReadOnlyIds};
        let none_token_stream = quote::quote!{None};

        let generate_import_path_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_value_initialization_token_stream(&import_path, &content_token_stream);

        let generate_ident_stringified = |
            current_postgresql_type: &PostgresqlType,
            current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
            current_postgresql_type_pattern: &PostgresqlTypePattern
        | {
            let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
            let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
            let rust_type_name = RustTypeName::from(current_postgresql_type);
            let postgresql_type_name = PostgresqlTypeName::from(current_postgresql_type);
            let not_null_or_nullable_rust = current_not_null_or_nullable.rust();
            let (rust_part, postgresql_part) = match &current_postgresql_type_pattern {
                PostgresqlTypePattern::Standart => (format!("{rust_type_name}"), format!("{postgresql_type_name}")),
                PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    (format!("{vec_of_upper_camel_case}{d1_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{postgresql_type_name}"))
                }
            };
            format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{current_not_null_or_nullable}{postgresql_part}")
        };
        let generate_ident_token_stream = |
            current_postgresql_type: &PostgresqlType,
            current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
            current_postgresql_type_pattern: &PostgresqlTypePattern
        | generate_ident_stringified(
            current_postgresql_type,
            current_not_null_or_nullable,
            current_postgresql_type_pattern
        ).parse::<proc_macro2::TokenStream>().expect("ff3eb7a6-8369-46fd-82f5-2afdf752365f");
        let ident = &generate_ident_token_stream(postgresql_type, not_null_or_nullable, postgresql_type_pattern);
        let generate_ident_standart_not_null_token_stream = |current_postgresql_type: &PostgresqlType| generate_ident_token_stream(current_postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlTypePattern::Standart);
        let ident_standart_not_null_upper_camel_case = generate_ident_standart_not_null_token_stream(postgresql_type);
        let ident_standart_nullable_upper_camel_case = generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::Nullable, &PostgresqlTypePattern::Standart);
        let ident_array_not_null_upper_camel_case = generate_ident_token_stream(
            postgresql_type,
            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            &PostgresqlTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            },
        );
        let ident_array_nullable_upper_camel_case = generate_ident_token_stream(
            postgresql_type,
            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
            &PostgresqlTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::Nullable,
            },
        );
        let generate_token_stream = |content_token_stream: &dyn quote::ToTokens, postgresql_type_or_postgresql_type_test_cases: &PostgresqlTypeOrPostgresqlTypeTestCases| {
            let trait_token_stream = match &postgresql_type_or_postgresql_type_test_cases {
                PostgresqlTypeOrPostgresqlTypeTestCases::PostgresqlType => quote::quote! {PostgresqlType},
                PostgresqlTypeOrPostgresqlTypeTestCases::PostgresqlTypeTestCases => quote::quote! {PostgresqlTypeTestCases},
            };
            quote::quote! {<#content_token_stream as #import_path::#trait_token_stream>}
        };
        let generate_as_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_token_stream(&content_token_stream, &PostgresqlTypeOrPostgresqlTypeTestCases::PostgresqlType);
        let generate_as_postgresql_type_test_cases_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_token_stream(&content_token_stream, &PostgresqlTypeOrPostgresqlTypeTestCases::PostgresqlTypeTestCases);
        let ident_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&ident);
        let self_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&self_upper_camel_case);
        let ident_standart_not_null_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&ident_standart_not_null_upper_camel_case);
        let ident_standart_nullable_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&ident_standart_nullable_upper_camel_case);
        let self_postgresql_type_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&quote::quote! {Self::#postgresql_type_upper_camel_case});

        let ident_standart_not_null_as_postgresql_type_test_cases_token_stream = generate_as_postgresql_type_test_cases_token_stream(&ident_standart_not_null_upper_camel_case);
        let ident_standart_nullable_as_postgresql_type_test_cases_token_stream = generate_as_postgresql_type_test_cases_token_stream(&ident_standart_nullable_upper_camel_case);
        let ident_array_not_null_as_postgresql_type_test_cases_token_stream = generate_as_postgresql_type_test_cases_token_stream(&ident_array_not_null_upper_camel_case);
        let ident_array_nullable_as_postgresql_type_test_cases_token_stream = generate_as_postgresql_type_test_cases_token_stream(&ident_array_nullable_upper_camel_case);
        let ident_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_copy()
            .build_struct(
                &ident,
                &quote::quote!{;},
            );
        let generate_ident_standart_not_null_origin_token_stream = |current_postgresql_type: &PostgresqlType| naming::parameter::SelfOriginUpperCamelCase::from_tokens(
            &generate_ident_standart_not_null_token_stream(current_postgresql_type)
        );
        let ident_standart_not_null_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(postgresql_type);
        let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
        let ident_standart_nullable_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_standart_nullable_upper_camel_case);

        let sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_upper_camel_case = generate_ident_standart_not_null_origin_token_stream(&PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);

        let generate_ident_standart_not_null_origin_try_new_error_named_token_stream = |current_postgresql_type: &PostgresqlType| naming::parameter::SelfOriginTryNewErrorNamedUpperCamelCase::from_tokens(
            &generate_ident_standart_not_null_token_stream(current_postgresql_type)
        );
        let sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_error_named_upper_camel_case = generate_ident_standart_not_null_origin_try_new_error_named_token_stream(&PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let inner_type_standart_not_null_token_stream = {
            let value = {
                let std_primitive_i16_stringified = "i16".to_owned();
                let std_primitive_i32_stringified = "i32".to_owned();
                let std_primitive_i64_stringified = "i64".to_owned();
                let std_primitive_f32_stringified = "f32".to_owned();
                let std_primitive_f64_stringified = "f64".to_owned();
                let sqlx_postgres_types_pg_money_stringified = "sqlx::postgres::types::PgMoney".to_owned();
                let std_primitive_bool_stringified = "bool".to_owned();
                let std_string_string_stringified = "String".to_owned();
                let std_vec_vec_std_primitive_u8_stringified = "Vec<u8>".to_owned();
                let sqlx_types_chrono_naive_date_stringified = "sqlx::types::chrono::NaiveDate".to_owned();
                let sqlx_types_chrono_naive_time_stringified = "sqlx::types::chrono::NaiveTime".to_owned();
                let sqlx_types_time_time_stringified = "sqlx::types::time::Time".to_owned();
                let sqlx_postgres_types_pg_interval_stringified = "sqlx::postgres::types::PgInterval".to_owned();
                let sqlx_types_chrono_naive_date_time_stringified = "sqlx::types::chrono::NaiveDateTime".to_owned();
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified = "sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>".to_owned();
                let sqlx_types_uuid_uuid_stringified = "sqlx::types::uuid::Uuid".to_owned();
                let sqlx_types_ipnetwork_ip_network_stringified = "sqlx::types::ipnetwork::IpNetwork".to_owned();
                let sqlx_types_mac_address_mac_address_stringified = "sqlx::types::mac_address::MacAddress".to_owned();
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveF32AsFloat4 => std_primitive_f32_stringified,
                    PostgresqlType::StdPrimitiveF64AsFloat8 => std_primitive_f64_stringified,
                    PostgresqlType::StdPrimitiveI16AsInt2 | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql => std_primitive_i16_stringified,
                    PostgresqlType::StdPrimitiveI32AsInt4 | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql => std_primitive_i32_stringified,
                    PostgresqlType::StdPrimitiveI64AsInt8 | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => std_primitive_i64_stringified,
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
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => sqlx_types_uuid_uuid_stringified,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => sqlx_types_ipnetwork_ip_network_stringified,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => sqlx_types_mac_address_mac_address_stringified,
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i32_stringified),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => wrap_into_sqlx_postgres_types_pg_range_stringified(&std_primitive_i64_stringified),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_stringified),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_naive_date_time_stringified),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => wrap_into_sqlx_postgres_types_pg_range_stringified(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_stringified),
                }
            };
            value.parse::<proc_macro2::TokenStream>().expect("2555843f-283f-4bc8-8c93-48e6fe68ae6a")
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
                if let Err(#error_snake_case) = #query_snake_case.try_bind(match #content_token_stream.0.0 {
                    Some(some_value) => Some(some_value),
                    None => None
                }) {
                    return Err(#error_snake_case.to_string());
                }
                Ok(#query_snake_case)
            },
        };
        let typical_query_bind_token_stream = generate_typical_query_bind_token_stream(&value_snake_case);
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
        let can_be_primary_key = match &postgresql_type {
            PostgresqlType::StdPrimitiveI16AsInt2
            | PostgresqlType::StdPrimitiveI32AsInt4
            | PostgresqlType::StdPrimitiveI64AsInt8
            | PostgresqlType::StdPrimitiveF32AsFloat4
            | PostgresqlType::StdPrimitiveF64AsFloat8
            | PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney
            | PostgresqlType::StdPrimitiveBoolAsBool
            | PostgresqlType::StdStringStringAsText
            | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
            | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
            | PostgresqlType::SqlxTypesTimeTimeAsTime
            | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
            | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
            | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
            | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
            | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
            | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr
            | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
            | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
            | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
            | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
            | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePrimaryKey::False,
            PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => CanBePrimaryKey::True,
        };
        let is_standart_not_null = if let (PostgresqlTypePattern::Standart, postgresql_crud_macros_common::NotNullOrNullable::NotNull) = (&postgresql_type_pattern, &not_null_or_nullable) {
            IsStandartNotNull::True
        } else {
            IsStandartNotNull::False
        };
        let is_not_null_standart_can_be_primary_key = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlTypePattern::Standart, CanBePrimaryKey::True) = (&not_null_or_nullable, &postgresql_type_pattern, &can_be_primary_key) {
            IsNotNullStandartCanBePrimaryKey::True
        } else {
            IsNotNullStandartCanBePrimaryKey::False
        };
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
        let (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl) = if let IsStandartNotNull::True = &is_standart_not_null {
            enum ParameterNumber {
                One,
                Two,
                Three,
                Four,
            }
            impl ParameterNumber {
                const fn get_index(&self) -> usize {
                    match &self {
                        Self::One => 0,
                        Self::Two => 1,
                        Self::Three => 2,
                        Self::Four => 3,
                    }
                }
                fn get_vec_from_index_starting_with_zero(&self) -> Vec<usize> {
                    (0..=self.get_index()).collect()
                }
            }
            let self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};
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
                                fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
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
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_standart_not_null_origin_double_quotes_token_stream, false as usize #parameter_number_token_stream)?;
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

                let generate_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens = |current_ident_token_stream: &dyn quote::ToTokens| {
                    let generate_serialize_field_match_std_ops_bound_token_stream = |start_or_end: &StartOrEnd| {
                        let start_or_end_token_stream = generate_start_or_end_snake_case(start_or_end);
                        generate_serialize_field_token_stream(
                            &start_or_end_token_stream,
                            &quote::quote! {
                                &match self.0.#start_or_end_token_stream {
                                    std::ops::Bound::Included(#value_snake_case) => std::ops::Bound::Included(#current_ident_token_stream::#try_new_snake_case(#value_snake_case).expect("c2e28cbe-e5cf-4300-a7ad-d6d67dc7c490")),
                                    std::ops::Bound::Excluded(#value_snake_case) => std::ops::Bound::Excluded(#current_ident_token_stream::#try_new_snake_case(#value_snake_case).expect("19d5d4f4-e686-4cff-b80e-8b37c50baf14")),
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
                    PostgresqlType::StdPrimitiveI16AsInt2
                    | PostgresqlType::StdPrimitiveI32AsInt4
                    | PostgresqlType::StdPrimitiveI64AsInt8
                    | PostgresqlType::StdPrimitiveF32AsFloat4
                    | PostgresqlType::StdPrimitiveF64AsFloat8
                    | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveBoolAsBool
                    | PostgresqlType::StdStringStringAsText
                    | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                    | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                    | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.0}))),
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
                                #nanosecond_snake_case(&self.0).checked_div(1000).expect("aea037b7-95ef-4616-b018-6f2ed1651928")
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
                        let generate_serialize_field_handle_token_stream = |value: &dyn naming::StdFmtDisplayPlusQuoteToTokens| generate_serialize_field_token_stream(&value, &quote::quote! {&#self_dot_zero_token_stream.#value});
                        let months_serialize_field_token_stream = generate_serialize_field_handle_token_stream(&months_snake_case);
                        let days_serialize_field_token_stream = generate_serialize_field_handle_token_stream(&days_snake_case);
                        let microseconds_serialize_field_token_stream = generate_serialize_field_handle_token_stream(&microseconds_snake_case);
                        quote::quote! {
                            #serde_state_initialization_three_fields_token_stream
                            #months_serialize_field_token_stream
                            #days_serialize_field_token_stream
                            #microseconds_serialize_field_token_stream
                            #serde_ser_serialize_struct_end_token_stream
                        }
                    })),
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
                                let current_ident_token_stream: &dyn quote::ToTokens = match &date_or_time {
                                    DateOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case,
                                    DateOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case,
                                };
                                quote::quote! {&#current_ident_token_stream::#try_new_snake_case(self.0.#date_or_time_token_stream()).expect("7efb7544-4454-4121-85fa-4b56d7afd21f")}
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
                                let current_ident_token_stream: &dyn quote::ToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_upper_camel_case,
                                    DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case,
                                };
                                quote::quote! {&#current_ident_token_stream::#try_new_snake_case(self.0.#date_naive_or_time_token_stream()).expect("4fd12604-816e-411a-8f51-6074e8ecd9d9")}
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
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream),
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote! {.bytes()}))),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgresql_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream),
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
                        marker: serde::__private228::PhantomData<#ident_standart_not_null_origin_upper_camel_case>,
                        lifetime: serde::__private228::PhantomData<&'de ()>,
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
                                marker: _serde::__private228::PhantomData::<#ident_standart_not_null_origin_upper_camel_case>,
                                lifetime: _serde::__private228::PhantomData,
                            }
                        )
                    }
                };
                let serde_deserializer_deserialize_newtype_struct_token_stream = quote::quote! {
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        #ident_standart_not_null_origin_double_quotes_token_stream,
                        __Visitor {
                            marker: serde::__private228::PhantomData::<Self>,
                            lifetime: serde::__private228::PhantomData,
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
                                ) -> Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    #content_token_stream
                                }
                            }
                        };
                    }
                };
                let generate_field_index_token_stream = |current_index: usize| format!("__{}{current_index}", naming::FieldSnakeCase).parse::<proc_macro2::TokenStream>().expect("a4e1a63f-821b-4d35-823a-0a99efa9d1dc");
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
                            fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                serde::__private228::Formatter::write_str(__f, #content_token_stream)
                            }
                        }
                    };
                    (generate_fn_expecting_token_stream(&struct_ident_double_quotes_token_stream), generate_fn_expecting_token_stream(&tuple_struct_ident_double_quotes_token_stream), generate_fn_expecting_token_stream(&quote::quote! {"field identifier"}))
                };
                let field_0_token_stream = generate_field_index_token_stream(parameter_number_one.get_index());
                let generate_serde_private_ok_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {Ok(#content_token_stream)}
                };
                let generate_serde_private_ok_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_serde_private_ok_token_stream(&quote::quote! {#ident_standart_not_null_origin_upper_camel_case(#content_token_stream)});
                let match_sqlx_types_uuid_uuid_field_type_try_parse_token_stream = quote::quote! {match #inner_type_standart_not_null_token_stream::try_parse(&#field_0_token_stream) {
                    Ok(ok_value) => ok_value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }};
                let sqlx_types_mac_address_mac_address_field_type_new_field_0_token_stream = quote::quote! {#inner_type_standart_not_null_token_stream::#new_snake_case(#field_0_token_stream)};
                let array_std_primitive_u8_6_token_stream = quote::quote! {[u8; 6]};
                let generate_vec_field_index_token_stream = |length: usize|{
                    let fields_token_stream = (1..=length).collect::<Vec<_>>().into_iter().enumerate().map(|(current_index, _)| generate_field_index_token_stream(current_index));
                    quote::quote!{#(#fields_token_stream),*}
                };
                let (sqlx_types_chrono_naive_time_origin_try_new_for_deserialize, match_origin_try_new_for_deserialize_one_token_stream, match_origin_try_new_for_deserialize_two_token_stream, match_origin_try_new_for_deserialize_four_token_stream) = {
                    let generate_match_origin_try_new_for_deserialize_token_stream = |length: usize| {
                        let fields_token_stream = generate_vec_field_index_token_stream(length);
                        quote::quote! {
                            match #ident_standart_not_null_origin_upper_camel_case::#try_new_for_deserialize_snake_case(#fields_token_stream) {
                                Ok(ok_value) => Ok(ok_value),
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
                    let generate_origin_new_for_deserialize_token_stream = |length: usize| {
                        let fields_token_stream = generate_vec_field_index_token_stream(length);
                        quote::quote! {
                            Ok(#ident_standart_not_null_origin_upper_camel_case::new_for_deserialize(#fields_token_stream))
                        }
                    };
                    (generate_origin_new_for_deserialize_token_stream(2), generate_origin_new_for_deserialize_token_stream(3))
                };
                let (fn_visit_newtype_struct_pg_money_token_stream, fn_visit_newtype_struct_uuid_token_stream, fn_visit_newtype_struct_mac_address_token_stream, fn_visit_newtype_struct_text_token_stream, fn_visit_newtype_struct_sqlx_types_chrono_naive_date_token_stream) = {
                    let generate_fn_visit_newtype_struct_token_stream = |type_token_stream: &dyn quote::ToTokens, serde_private_ok_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[inline]
                            fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
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
                    let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(current_index, current_element)| {
                        let field_index_token_stream = generate_field_index_token_stream(current_index);
                        let index_usize_token_stream = format!("{current_index}usize").parse::<proc_macro2::TokenStream>().expect("ce15e6bf-cf71-42c3-9f6d-94d0f7ec6ede");
                        quote::quote! {
                            let #field_index_token_stream = match serde::de::SeqAccess::next_element::<#current_element>(&mut __seq)? {
                                Some(__value) => __value,
                                None => {
                                    return Err(serde::de::Error::invalid_length(#index_usize_token_stream, &#error_message_token_stream));
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
                            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
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
                            parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(|current_element| {
                                let index_variant_token_stream = format!("{current_element}u64").parse::<proc_macro2::TokenStream>().expect("5aee0393-2f04-42ca-87d6-bb4209d41ee1");
                                let field_index_token_stream = generate_field_index_token_stream(current_element);
                                quote::quote! {#index_variant_token_stream => Ok(__Field::#field_index_token_stream)}
                            })
                        };
                        quote::quote! {
                            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                            where
                                __E: serde::de::Error,
                            {
                                match __value {
                                    #(#fields_token_stream),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (generate_fn_visit_u64_token_stream(&parameter_number_two), generate_fn_visit_u64_token_stream(&parameter_number_three), generate_fn_visit_u64_token_stream(&parameter_number_four))
                };
                let (fn_visit_str_value_start_end_token_stream, fn_visit_str_value_hour_min_sec_micro_token_stream, fn_visit_str_value_hour_minute_second_microsecond_token_stream, fn_visit_str_value_date_time_token_stream, fn_visit_str_value_date_naive_time_token_stream, fn_visit_str_value_months_days_microseconds_token_stream) = {
                    let generate_fn_visit_str_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]| {
                        let fields_token_stream = vec_token_stream.iter().enumerate().map(|(current_index, current_element)| {
                            let element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&current_element);
                            let field_index_name_token_stream = generate_field_index_token_stream(current_index);
                            quote::quote! {#element_double_quotes_token_stream => Ok(__Field::#field_index_name_token_stream)}
                        });
                        quote::quote! {
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    #(#fields_token_stream),*,
                                    _ => Ok(__Field::__ignore),
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
                        let fields_token_stream = vec_token_stream.iter().enumerate().map(|(current_index, current_element)| {
                            let b_element_double_quotes_token_stream = format!("b{}", generate_quotes::double_quotes_stringified(&current_element)).parse::<proc_macro2::TokenStream>().expect("c76c976b-9009-43d2-8d4b-1ec559b76008");
                            let field_index_name_token_stream = generate_field_index_token_stream(current_index);
                            quote::quote! {#b_element_double_quotes_token_stream => Ok(__Field::#field_index_name_token_stream)}
                        });
                        quote::quote! {
                            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                            where
                                __E: serde::de::Error,
                            {
                                match __value {
                                    #(#fields_token_stream),*,
                                    _ => Ok(__Field::__ignore),
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
                let impl_serde_deserialize_for_field_token_stream = quote::quote! {
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
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
                            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
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
                            let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(current_index, current_element)| {
                                let field_index_name_token_stream = generate_field_index_token_stream(current_index);
                                quote::quote! {let mut #field_index_name_token_stream: Option<#current_element> = None;}
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
                            let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(current_index, current_element)| {
                                let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&current_element.0);
                                let field_type_token_stream = &current_element.1;
                                let field_index_name_token_stream = generate_field_index_token_stream(current_index);
                                quote::quote! {
                                    __Field::#field_index_name_token_stream => {
                                        if Option::is_some(&#field_index_name_token_stream) {
                                            return Err(<__A::Error as serde::de::Error>::duplicate_field(#field_name_double_quotes_token_stream));
                                        }
                                        #field_index_name_token_stream = Some(serde::de::MapAccess::next_value::<#field_type_token_stream>(&mut __map)?);
                                    }
                                }
                            });
                            quote::quote! {
                                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        #(#fields_initialization_token_stream)*
                                        __Field::__ignore => {
                                            let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
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
                            let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(current_index, current_element)| {
                                let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&current_element);
                                let field_index_name_token_stream = generate_field_index_token_stream(current_index);
                                quote::quote! {
                                    let #field_index_name_token_stream = match #field_index_name_token_stream {
                                        Some(#field_index_name_token_stream) => #field_index_name_token_stream,
                                        None => serde::__private228::de::missing_field(#field_name_double_quotes_token_stream)?,
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
                        let field_names_token_stream = vec_token_stream.iter().map(|current_element| generate_quotes::double_quotes_token_stream(&current_element));
                        quote::quote! {
                            #[doc(hidden)]
                            const FIELDS: &[&str] = &[#(#field_names_token_stream),*];
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
                let generate_impl_serde_de_visitor_for_tokens_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl<'de> _serde::de::Visitor<'de> for #current_ident_token_stream {
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
                            #[doc(hidden)]
                            struct #field_visitor_token_stream;
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
                    PostgresqlType::StdPrimitiveI16AsInt2
                    | PostgresqlType::StdPrimitiveI32AsInt4
                    | PostgresqlType::StdPrimitiveI64AsInt8
                    | PostgresqlType::StdPrimitiveF32AsFloat4
                    | PostgresqlType::StdPrimitiveF64AsFloat8
                    | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql
                    | PostgresqlType::StdPrimitiveBoolAsBool
                    | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                    | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => postgresql_crud_macros_common::DeriveOrImpl::Derive,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                        #struct_visitor_token_stream
                        #impl_serde_de_visitor_for_visitor_pg_money_token_stream
                        #serde_deserializer_deserialize_newtype_struct_token_stream
                    })),
                    PostgresqlType::StdStringStringAsText => postgresql_crud_macros_common::DeriveOrImpl::Impl(generate_impl_serde_deserialize_for_tokens_token_stream(&quote::quote! {
                        #struct_visitor_token_stream
                        #impl_serde_de_visitor_for_visitor_std_string_string_token_stream
                        #serde_deserializer_deserialize_newtype_struct_token_stream
                    })),
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
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgresql_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream),
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
            quote::quote! {sqlx::types::time::Time::from_hms_micro(#content_token_stream).expect("7a1a18fa-c0cf-45e4-8b52-60f58a793c36")}
        };
        let generate_pub_const_new_or_pub_try_new_token_stream = |current_ident: &dyn quote::ToTokens| {
            let pub_fn_new_or_try_new_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                &macros_helpers::generate_pub_try_new_token_stream(
                    &value_ident_inner_type_token_stream,
                    &ident_standart_not_null_origin_try_new_error_named_upper_camel_case,
                    &quote::quote! {
                        match #ident_origin_upper_camel_case::#try_new_snake_case(#value_snake_case) {
                            Ok(ok_value) => Ok(Self(ok_value)),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                )
            } else {
                &{
                    let self_ident_origin_new_value_token_stream = quote::quote! {Self(#ident_origin_upper_camel_case::#new_snake_case(#value_snake_case))};
                    if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                        && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                    {
                        macros_helpers::generate_pub_const_new_token_stream(&value_ident_inner_type_token_stream, &self_ident_origin_new_value_token_stream)
                    } else {
                        macros_helpers::generate_pub_new_token_stream(&value_ident_inner_type_token_stream, &self_ident_origin_new_value_token_stream)
                    }
                }
            };
            quote::quote! {
                impl #current_ident {
                    #pub_fn_new_or_try_new_token_stream
                }
            }
        };
        let derive_copy = match &postgresql_type_pattern {
            PostgresqlTypePattern::Standart => match &postgresql_type {
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
                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime |
                PostgresqlType::SqlxTypesTimeTimeAsTime |
                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval |
                PostgresqlType::SqlxTypesChronoNaiveDateAsDate |
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql |
                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet |
                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr |
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::DeriveCopy::True,
                PostgresqlType::StdStringStringAsText |
                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => macros_helpers::DeriveCopy::False,
            },
            PostgresqlTypePattern::ArrayDimension1 { .. } => macros_helpers::DeriveCopy::False,
        };
        let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
        let ident_origin_token_stream = {
            let ident_origin_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_eq_if(match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::True => macros_helpers::DeriveEq::True,
                    IsNotNullStandartCanBePrimaryKey::False => macros_helpers::DeriveEq::False,
                })
                .derive_partial_ord_if(match &is_standart_not_null {
                    IsStandartNotNull::True => match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2
                        | PostgresqlType::StdPrimitiveI32AsInt4
                        | PostgresqlType::StdPrimitiveI64AsInt8
                        | PostgresqlType::StdPrimitiveF32AsFloat4
                        | PostgresqlType::StdPrimitiveF64AsFloat8
                        | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                        | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                        | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql
                        | PostgresqlType::StdPrimitiveBoolAsBool
                        | PostgresqlType::StdStringStringAsText
                        | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                        | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
                        | PostgresqlType::SqlxTypesTimeTimeAsTime
                        | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                        | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => macros_helpers::DerivePartialOrd::True,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney
                        | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
                        | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                        | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                        | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::DerivePartialOrd::False,
                    },
                    IsStandartNotNull::False => macros_helpers::DerivePartialOrd::False
                })
                .derive_ord_if(match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::True => macros_helpers::DeriveOrd::True,
                    IsNotNullStandartCanBePrimaryKey::False => macros_helpers::DeriveOrd::False,
                })
                .derive_serde_serialize_if(match &serde_serialize_derive_or_impl {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => macros_helpers::DeriveSerdeSerialize::True,
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => macros_helpers::DeriveSerdeSerialize::False,
                })
                .derive_serde_deserialize_if(match &serde_deserialize_derive_or_impl {
                    postgresql_crud_macros_common::DeriveOrImpl::Derive => macros_helpers::DeriveSerdeDeserialize::True,
                    postgresql_crud_macros_common::DeriveOrImpl::Impl(_) => macros_helpers::DeriveSerdeDeserialize::False,
                })
                .build_struct(
                    &ident_origin_upper_camel_case,
                    &quote::quote!{(#field_type_handle);},
                );
            let contains_null_byte_upper_camel_case = naming::ContainsNullByteUpperCamelCase;
            let earlier_date_not_supported_upper_camel_case = naming::EarlierDateNotSupportedUpperCamelCase;
            let earliest_supported_date_snake_case = naming::EarliestSupportedDateSnakeCase;
            let date_upper_camel_case = naming::DateUpperCamelCase;
            let time_upper_camel_case = naming::TimeUpperCamelCase;
            let date_naive_upper_camel_case = naming::DateNaiveUpperCamelCase;
            let time_upper_camel_camel_case = naming::TimeUpperCamelCase;
            let invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case = naming::InvalidHourOrMinuteOrSecondOrMicrosecondUpperCamelCase;
            let nanosecond_precision_is_not_supported_upper_camel_case = naming::NanosecondPrecisionIsNotSupportedUpperCamelCase;
            let included_start_greater_than_included_end_upper_camel_case = naming::IncludedStartGreaterThanIncludedEndUpperCamelCase;
            let included_start_greater_than_excluded_end_upper_camel_case = naming::IncludedStartGreaterThanExcludedEndUpperCamelCase;
            let excluded_start_greater_than_included_end_upper_camel_case = naming::ExcludedStartGreaterThanIncludedEndUpperCamelCase;
            let excluded_start_greater_than_excluded_end_upper_camel_case = naming::ExcludedStartGreaterThanExcludedEndUpperCamelCase;
            let included_end_cannot_be_max_upper_camel_case = naming::IncludedEndCannotBeMaxUpperCamelCase;
            let generate_int_range_type_error_variants_token_stream = |int_range_type: &IntRangeType| {
                let range_inner_type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                quote::quote! {
                    #included_start_greater_than_included_end_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #start_snake_case: #range_inner_type_token_stream,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #end_snake_case: #range_inner_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #included_start_greater_than_excluded_end_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #start_snake_case: #range_inner_type_token_stream,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #end_snake_case: #range_inner_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #excluded_start_greater_than_included_end_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #start_snake_case: #range_inner_type_token_stream,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #end_snake_case: #range_inner_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #excluded_start_greater_than_excluded_end_upper_camel_case {
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
            let maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_token_stream = if let IsStandartNotNull::True = &is_standart_not_null
                && let Ok(postgresql_type_initialization_try_new) = &postgresql_type_initialization_try_new_try_from_postgresql_type
            {
                macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_thiserror_error()
                    .derive_error_occurence_lib_error_occurence()
                    .build_enum(
                        &ident_standart_not_null_origin_try_new_error_named_upper_camel_case,
                        &{
                            let generate_start_end_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                                let (start_variant_token_stream, end_variant_token_stream) = {
                                    let generate_variant_token_stream = |start_or_end: &StartOrEnd| {
                                        let start_or_end_upper_camel_case = generate_start_or_end_upper_camel_case(start_or_end);
                                        quote::quote! {
                                            #start_or_end_upper_camel_case {
                                                #[eo_error_occurence]
                                                #error_snake_case: #content_token_stream,
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
                            };
                            let content_token_stream: &dyn quote::ToTokens = match &postgresql_type_initialization_try_new {
                                PostgresqlTypeInitializationTryNew::StdStringStringAsText => &std_string_string_as_text_try_new_error_named_variants_token_stream,
                                PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime | PostgresqlTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_variant_try_new_token_stream,
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
                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &generate_start_end_token_stream(
                                    &sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_upper_camel_case
                                ),
                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &generate_start_end_token_stream(
                                    &sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_error_named_upper_camel_case
                                ),
                                PostgresqlTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &generate_start_end_token_stream(
                                    &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_error_named_upper_camel_case
                                ),
                            };
                            quote::quote!{{#content_token_stream}}
                        }
                    )
            } else {
                proc_macro2::TokenStream::new()
            };
            let maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_token_stream = if let IsStandartNotNull::True = &is_standart_not_null
                && let postgresql_crud_macros_common::DeriveOrImpl::Impl(_) = &serde_deserialize_derive_or_impl
            {
                match &postgresql_type_deserialize {
                    PostgresqlTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                    PostgresqlTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                        PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(_) => proc_macro2::TokenStream::new(),
                        PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgresql_type_impl_try_new_for_deserialize) => macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_thiserror_error()
                            .derive_error_occurence_lib_error_occurence()
                            .build_enum(
                                &ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case,
                                &{
                                    let content_token_stream: &dyn quote::ToTokens = match &postgresql_type_impl_try_new_for_deserialize {
                                        PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText => &std_string_string_as_text_try_new_error_named_variants_token_stream,
                                        PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => &quote::quote! {
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
                                        },
                                        PostgresqlTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => &quote::quote! {
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
                                        },
                                        PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_token_stream,
                                        PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                        PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                    };
                                    quote::quote!{{#content_token_stream}}
                                }
                            )
                    },
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_ident_origin_token_stream = {
                let pub_fn_new_or_try_new_token_stream = postgresql_type_initialization_try_new_try_from_postgresql_type.as_ref().map_or_else(
                |()| {
                    let content_token_stream = {
                        let content_token_stream = {
                            let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                                quote::quote! {match value {
                                    Some(some_value) => Some(#type_token_stream::#new_snake_case(some_value)),
                                    None => None
                                }}
                            };
                            let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value.into_iter().map(#type_token_stream::#new_snake_case).collect()},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_option_token_stream(&type_token_stream),
                            };
                            match &postgresql_type_pattern {
                                PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                        postgresql_type_range_try_from_postgresql_type.as_ref().map_or_else(
                                            |()| quote::quote! {#value_snake_case},
                                            |value| generate_pg_range_conversion_token_stream(
                                                &value_snake_case,
                                                &{
                                                    let range_postgresql_type_ident_origin = naming::parameter::SelfOriginUpperCamelCase::from_display(&generate_ident_stringified(&PostgresqlType::from(value), not_null_or_nullable, postgresql_type_pattern));
                                                    quote::quote! {#range_postgresql_type_ident_origin::#new_snake_case(value)}
                                                }
                                            )
                                        )
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
                        quote::quote! {Self(#content_token_stream)}
                    };
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => macros_helpers::generate_const_new_token_stream(&value_ident_inner_type_token_stream, &content_token_stream),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => macros_helpers::generate_new_token_stream(&value_ident_inner_type_token_stream, &content_token_stream),
                        },
                        PostgresqlTypePattern::ArrayDimension1 { .. } => macros_helpers::generate_new_token_stream(&value_ident_inner_type_token_stream, &content_token_stream),
                    }
                },
                |postgresql_type_initialization_try_new| {
                    let content_token_stream = {
                        let generate_match_option_token_stream = |type_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {Ok(Self(match #value_snake_case {
                                Some(some_value) => Some(match #type_token_stream::#try_new_snake_case(some_value) {
                                    Ok(ok_value) => ok_value,
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
                                    let mut #acc_snake_case = Vec::new();
                                    for #element_snake_case in #value_snake_case {
                                        match #type_token_stream::#try_new_snake_case(#element_snake_case) {
                                            Ok(ok_value) => {
                                                #acc_snake_case.push(ok_value);
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
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_greater_than_included_end_upper_camel_case {
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
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_greater_than_excluded_end_upper_camel_case {
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
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_greater_than_included_end_upper_camel_case {
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
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_greater_than_excluded_end_upper_camel_case {
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
                                    let generate_ok_self_sqlx_postgres_types_pg_range_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| quote::quote! {
                                        Ok(Self(sqlx::postgres::types::PgRange {
                                            #start_snake_case: match #value_snake_case.#start_snake_case {
                                                std::ops::Bound::Included(included_value) => match #current_ident_token_stream::#try_new_snake_case(included_value) {
                                                    Ok(ok_value) => std::ops::Bound::Included(ok_value.0),
                                                    Err(#error_snake_case) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#start_upper_camel_case {
                                                            #error_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Excluded(excluded_value) => match #current_ident_token_stream::#try_new_snake_case(excluded_value) {
                                                    Ok(ok_value) => std::ops::Bound::Excluded(ok_value.0),
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
                                                std::ops::Bound::Included(included_value) => match #current_ident_token_stream::#try_new_snake_case(included_value) {
                                                    Ok(ok_value) => std::ops::Bound::Included(ok_value.0),
                                                    Err(#error_snake_case) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#end_upper_camel_case {
                                                            #error_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Excluded(excluded_value) => match #current_ident_token_stream::#try_new_snake_case(excluded_value) {
                                                    Ok(ok_value) => std::ops::Bound::Excluded(ok_value.0),
                                                    Err(#error_snake_case) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#end_upper_camel_case {
                                                            #error_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                            },
                                        }))
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
                                            if <#inner_type_standart_not_null_token_stream as chrono::Timelike>::nanosecond(&#value_snake_case).checked_rem(1000).expect("7c8b4e12-8509-41e4-8769-0fe10aafd930") != 0 {
                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                    #value_snake_case: #value_snake_case.to_string(),
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                            Ok(Self(#value_snake_case))
                                        },
                                        PostgresqlTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => quote::quote! {
                                            if #value_snake_case.nanosecond().checked_rem(1000).expect("ce47524f-de07-4a01-a4c5-78d39398b922") != 0 {
                                                return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                    #value_snake_case: #value_snake_case.to_string(),
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                            Ok(Self(#value_snake_case))
                                        },
                                        PostgresqlTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => quote::quote! {
                                            let #earliest_supported_date_snake_case = #inner_type_standart_not_null_token_stream::from_ymd_opt(-4713, 12, 31).expect("9f6241e5-a3ce-4ade-b33c-37432d4cafd3");
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
                                                Ok(ok_value) => ok_value,
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
                                                Ok(ok_value) => ok_value,
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
                                                    Ok(ok_value) => ok_value,
                                                    Err(#error_snake_case) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#date_naive_upper_camel_case {
                                                            #error_snake_case,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                };
                                                let #time_snake_case = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_upper_camel_case::#try_new_snake_case(#value_snake_case.time()) {
                                                    Ok(ok_value) => ok_value,
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
                });
                let maybe_fn_new_or_try_new_for_deserialize_token = match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_type_deserialize {
                            PostgresqlTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                            PostgresqlTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgresql_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                                PostgresqlTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(postgresql_type_impl_new_for_deserialize) => {
                                    let parameters_token_stream = {
                                        let generate_start_end_std_std_ops_bound_token_stream = |current_ident_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                #start_snake_case: std::ops::Bound<#current_ident_token_stream>,
                                                #end_snake_case: std::ops::Bound<#current_ident_token_stream>
                                            }
                                        };
                                        match &postgresql_type_impl_new_for_deserialize {
                                            PostgresqlTypeImplNewForDeserialize::SsqlxPostgresTypesPgIntervalAsInterval => quote::quote! {
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
                                            PostgresqlTypeImplNewForDeserialize::SsqlxPostgresTypesPgIntervalAsInterval => quote::quote! {
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
                                            PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange | PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange | PostgresqlTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                                                self_sqlx_postgres_types_pg_range_token_stream
                                            }
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
                                            PostgresqlTypeImplTryNewForDeserialize::StdStringStringAsText | PostgresqlTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => {
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
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_value_pg_range_int_type_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_value_pg_range_int_type_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                        }
                                    };
                                    let content_token_stream = {
                                        let generate_self_match_try_new_token_stream = |current_parameters_token_stream: &dyn quote::ToTokens, match_error_variants_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                match Self::#try_new_snake_case(#current_parameters_token_stream) {
                                                    Ok(ok_value) => Ok(ok_value),
                                                    Err(#error_snake_case) => match #error_snake_case {
                                                        #match_error_variants_token_stream
                                                    }
                                                }
                                            }
                                        };
                                        let try_new_convert_pg_range_int_content_token_stream = generate_self_match_try_new_token_stream(
                                            &quote::quote! {sqlx::postgres::types::PgRange { #start_snake_case, #end_snake_case }},
                                            &quote::quote! {
                                                #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_greater_than_included_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#included_start_greater_than_included_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#included_start_greater_than_excluded_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#included_start_greater_than_excluded_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_greater_than_included_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#excluded_start_greater_than_included_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_upper_camel_case::#excluded_start_greater_than_excluded_end_upper_camel_case {
                                                    #start_snake_case,
                                                    #end_snake_case,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#excluded_start_greater_than_excluded_end_upper_camel_case {
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
                                                        Some(some_value) => {
                                                            if <#inner_type_standart_not_null_token_stream as chrono::Timelike>::nanosecond(&some_value).checked_rem(1000).expect("c0514180-cfe0-44e2-9dcf-ab41df7e11f3") != 0 {
                                                                return Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                                    #value_snake_case: some_value.to_string(),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            Ok(Self(some_value))
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
                                                        Ok(ok_value) => {
                                                            if ok_value.nanosecond().checked_rem(1000).expect("0def33ce-99c1-4969-9f1d-6923319ccc5b") != 0 {
                                                                return Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                                    #value_snake_case: ok_value.to_string(),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            Ok(Self(ok_value))
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
                                            PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresqlTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => try_new_convert_pg_range_int_content_token_stream,
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
                    PostgresqlTypePattern::ArrayDimension1 { .. } => proc_macro2::TokenStream::new(),
                };
                quote::quote! {
                    impl #ident_origin_upper_camel_case {
                        #pub_fn_new_or_try_new_token_stream
                        #maybe_fn_new_or_try_new_for_deserialize_token
                    }
                }
            };
            let impl_std_convert_from_ident_origin_for_ident_inner_type_token_stream = {
                let content_token_stream = {
                    let value_dot_zero = quote::quote! {#value_snake_case.0};
                    let element_dot_zero_token_stream = quote::quote! {#element_snake_case.0};
                    let generate_match_token_stream = |match_content_token_stream: &dyn quote::ToTokens, some_content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            match #match_content_token_stream {
                                Some(some_value) => Some(some_value.0#some_content_token_stream),
                                None => None
                            }
                        }
                    };
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => value_dot_zero,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_token_stream(&value_dot_zero, &proc_macro2::TokenStream::new()),
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                            let dimension1_token_stream = match &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => element_dot_zero_token_stream,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_token_stream(&element_dot_zero_token_stream, &proc_macro2::TokenStream::new()),
                            };
                            let into_iter_dimension1_token_stream = quote::quote! {.into_iter().map(|#element_snake_case|#dimension1_token_stream).collect()};
                            match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                    #value_dot_zero #into_iter_dimension1_token_stream
                                },
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_token_stream(&value_dot_zero, &into_iter_dimension1_token_stream),
                            }
                        }
                    }
                };
                quote::quote! {
                    impl From<#ident_origin_upper_camel_case> for #ident_inner_type_token_stream {
                        fn from(#value_snake_case: #ident_origin_upper_camel_case) -> Self {
                            #content_token_stream
                        }
                    }
                }
            };
            let maybe_impl_is_string_empty_for_ident_origin_token_stream = if let IsStandartNotNull::True = &is_standart_not_null {
                let impl_is_string_empty_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case);
                match &not_null_or_nullable {
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
                        | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                        | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
                        | PostgresqlType::SqlxTypesTimeTimeAsTime
                        | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
                        | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                        | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                        | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                        PostgresqlType::StdStringStringAsText | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => impl_is_string_empty_for_ident_origin_token_stream,
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
            let impl_std_fmt_display_for_ident_origin_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(f, "{self:?}")});
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
                                    <
                                        #current_ident_token_stream
                                        as
                                        #import_path::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
                                    >::default_but_option_is_always_some_and_vec_always_contains_one_element()
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
                                | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql
                                | PostgresqlType::StdPrimitiveBoolAsBool
                                | PostgresqlType::StdStringStringAsText
                                | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                                | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
                                | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr
                                | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => &quote::quote! {#field_type_handle::default()},
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => &core_default_default_default_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => &quote::quote! {#inner_type_standart_not_null_token_stream(#core_default_default_default_token_stream)},
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => &quote::quote! {vec![#core_default_default_default_token_stream]},
                                PostgresqlType::SqlxTypesTimeTimeAsTime => &generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {0,0,0,0}),
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => &quote::quote! {#inner_type_standart_not_null_token_stream {
                                    #months_snake_case: #core_default_default_default_token_stream,
                                    #days_snake_case: #core_default_default_default_token_stream,
                                    #microseconds_snake_case: #core_default_default_default_token_stream
                                }},
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                    #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                    #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                }),
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                    #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                    #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream.0,
                                })),
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => &quote::quote! {
                                    sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_token_stream).expect("9e9c9b57-1a39-4674-a112-5e009fcbab0f"))
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &pg_range_int_default_initialization_token_stream,
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
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &quote::quote! {#self_snake_case.0});
            let impl_sqlx_decode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &field_type_handle, &{
                let scopes_ok_value_token_stream = quote::quote! {(ok_value)};
                let ok_self_scopes_ok_value_token_stream = quote::quote! {Ok(Self #scopes_ok_value_token_stream)};
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
                            | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_ok_value_token_stream,
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote::quote! {
                                match Self::#try_new_snake_case #scopes_ok_value_token_stream {
                                    Ok(ok_value0) => Ok(ok_value0),
                                    Err(#error_snake_case) => Err(Box::#new_snake_case(#error_snake_case)),
                                }
                            },
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => ok_self_scopes_ok_value_token_stream,
                    },
                    PostgresqlTypePattern::ArrayDimension1 { .. } => ok_self_scopes_ok_value_token_stream,
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
            let maybe_impl_std_convert_from_ident_read_for_ident_origin_token_stream = match &is_not_null_standart_can_be_primary_key {
                IsNotNullStandartCanBePrimaryKey::True => macros_helpers::generate_impl_std_convert_from_token_stream(&ident_standart_not_null_read_upper_camel_case, &ident_origin_upper_camel_case, &{
                    let ident_standart_not_null_as_crate_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&ident_standart_not_null_upper_camel_case);
                    quote::quote! {Self::#new_snake_case(#ident_standart_not_null_as_crate_postgresql_type_token_stream::into_inner(#value_snake_case))}
                }),
                IsNotNullStandartCanBePrimaryKey::False => proc_macro2::TokenStream::new(),
            };
            quote::quote! {
                #ident_origin_token_stream
                #maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_token_stream
                #maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_token_stream
                #impl_ident_origin_token_stream
                #impl_std_convert_from_ident_origin_for_ident_inner_type_token_stream

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
                #maybe_impl_std_convert_from_ident_read_for_ident_origin_token_stream
            }
        };
        let generate_pub_struct_tokens_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens, derive_default: macros_helpers::DeriveDefault| {
            macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_default_if(derive_default)
                .derive_clone()
                .derive_copy()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &current_ident_token_stream,
                    &content_token_stream
                )
        };
        let ident_origin_struct_content_token_stream = quote::quote!{(#ident_origin_upper_camel_case);};
        let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
        let ident_table_type_declaration_token_stream = {
            let ident_table_type_declaration_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_partial_ord_if(match &is_standart_not_null {
                    IsStandartNotNull::True => match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2
                        | PostgresqlType::StdPrimitiveI32AsInt4
                        | PostgresqlType::StdPrimitiveI64AsInt8
                        | PostgresqlType::StdPrimitiveF32AsFloat4
                        | PostgresqlType::StdPrimitiveF64AsFloat8
                        | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                        | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                        | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql
                        | PostgresqlType::StdPrimitiveBoolAsBool
                        | PostgresqlType::StdStringStringAsText
                        | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                        | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
                        | PostgresqlType::SqlxTypesTimeTimeAsTime
                        | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                        | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => macros_helpers::DerivePartialOrd::True,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney
                        | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
                        | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                        | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                        | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::DerivePartialOrd::False,
                    },
                    IsStandartNotNull::False => macros_helpers::DerivePartialOrd::False
                })
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_table_type_declaration_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_table_type_declaration_token_stream = generate_pub_const_new_or_pub_try_new_token_stream(&ident_table_type_declaration_upper_camel_case);
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_table_type_declaration_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &ident_origin_upper_camel_case);
            let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &quote::quote! {#self_snake_case.0});
            let impl_sqlx_decode_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &ident_origin_upper_camel_case, &quote::quote! {Ok(Self(ok_value))});
        //todo rewrite as dependency of PostgresqlType trait?
            let impl_postgresql_type_equal_operator_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::impl_postgresql_type_equal_operator_for_ident_token_stream(
                &import_path,
                &ident_table_type_declaration_upper_camel_case,
            //todo
                &{
                    let equal_token_stream = postgresql_crud_macros_common::EqualOperatorHandle::Equal.to_tokens_path(&import_path);
                    let is_null_token_stream = postgresql_crud_macros_common::EqualOperatorHandle::IsNull.to_tokens_path(&import_path);
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => equal_token_stream,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                if self.0.0.is_some() {
                                    #equal_token_stream
                                }
                                else {
                                    #is_null_token_stream
                                }
                            },
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &dimension1_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => equal_token_stream,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                //todo thats not actually usefull coz nullable array comparison has different logic. need to refactor EqualOperatorHandle enum
                                    equal_token_stream
                                }
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                if self.0.0.is_some() {
                                    #equal_token_stream
                                }
                                else {
                                    #is_null_token_stream
                                }
                            },
                        },
                    }
                },
            );
            quote::quote! {
                #ident_table_type_declaration_token_stream
                #impl_ident_table_type_declaration_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_token_stream
                #impl_sqlx_decode_sqlx_postgres_for_ident_table_type_declaration_token_stream
                #impl_postgresql_type_equal_operator_for_ident_table_type_declaration_token_stream
            }
        };
        let ident_standart_not_null_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
        let ident_create_token_stream = {
            let ident_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::True => generate_pub_struct_tokens_token_stream(&ident_create_upper_camel_case, &quote::quote! {(());}, macros_helpers::DeriveDefault::False),
                CanBePrimaryKey::False => macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy_if(derive_copy)
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_create_upper_camel_case,
                        &ident_origin_struct_content_token_stream
                    )
            };
            let maybe_impl_ident_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
                CanBePrimaryKey::False => generate_pub_const_new_or_pub_try_new_token_stream(&ident_create_upper_camel_case),
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_create_upper_camel_case, &{
                let content_token_stream: &dyn quote::ToTokens = match &can_be_primary_key {
                    CanBePrimaryKey::True => &quote::quote! {()},
                    CanBePrimaryKey::False => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                };
                quote::quote! {Self(#content_token_stream)}
            });
            let maybe_impl_sqlx_encode_sqlx_postgres_for_ident_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
                CanBePrimaryKey::False => postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_create_upper_camel_case, &quote::quote! {#self_snake_case.0}),
            };
            let maybe_impl_sqlx_type_sqlx_postgres_for_ident_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
                CanBePrimaryKey::False => postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_create_upper_camel_case, &ident_origin_upper_camel_case),
            };
            quote::quote! {
                #ident_create_token_stream
                #maybe_impl_ident_create_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream
                #maybe_impl_sqlx_encode_sqlx_postgres_for_ident_create_token_stream
                #maybe_impl_sqlx_type_sqlx_postgres_for_ident_create_token_stream
            }
        };
        let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
        let ident_select_token_stream = {
            let pub_struct_ident_select_token_stream = generate_pub_struct_tokens_token_stream(
                &ident_select_upper_camel_case,
                &match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => quote::quote! {;},
                    PostgresqlTypePattern::ArrayDimension1 { .. } => {
                        let mut arguments_token_stream = Vec::new();
                        for current_element in 1..=array_dimensions_number {
                            let dimension_number_pagination_token_stream = format!("dimension{current_element}_pagination").parse::<proc_macro2::TokenStream>().expect("af86f2d1-b00d-49ab-9ced-97a488d9dc5f");
                            arguments_token_stream.push(quote::quote! {
                                #dimension_number_pagination_token_stream: PaginationStartsWithOne
                            });
                        }
                        quote::quote! {{#(#arguments_token_stream),*}}
                    }
                },
                macros_helpers::DeriveDefault::True,
            );
            let (impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream, impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_ident_select_token_stream) = {
                let generate_default_content_token_stream = |default_some_one_or_default_some_one_with_max_page_size: &postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => quote::quote! {Self},
                    PostgresqlTypePattern::ArrayDimension1 { .. } => {
                        let content_token_stream: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream,
                        };
                        let mut arguments_token_stream = Vec::new();
                        for current_element in 1..=array_dimensions_number {
                            let dimension_number_pagination_token_stream = format!("dimension{current_element}_pagination").parse::<proc_macro2::TokenStream>().expect("e5250a98-89d6-4a58-90ea-39b04a708c1c");
                            arguments_token_stream.push(quote::quote! {
                                #dimension_number_pagination_token_stream: #content_token_stream
                            });
                        }
                        quote::quote! {Self {#(#arguments_token_stream),*}}
                    }
                };
                (
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &generate_default_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)),
                    postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(&ident_select_upper_camel_case, &generate_default_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)),
                )
            };
            quote::quote! {
                #pub_struct_ident_select_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_ident_select_token_stream
            }
        };
        let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
        let ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident);
        let ident_where_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_token_stream(
            &{
                let common_postgresql_type_filters = vec![postgresql_crud_macros_common::PostgresqlTypeFilter::Equal { ident: quote::quote! {#ident_table_type_declaration_upper_camel_case} }];
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
                        let common_standart_postgresql_type_filters = { common_postgresql_type_filters };
                        let common_standart_postgresql_type_number_filters = {
                            let mut vec = common_standart_postgresql_type_filters.clone();
                            vec.push(greater_than.clone());
                            vec.push(between.clone());
                            vec.push(in_handle.clone());
                            vec
                        };
                        let (
                            where_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                            where_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
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
                            (ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec)
                        };
                        match &postgresql_type {
                            PostgresqlType::StdPrimitiveI16AsInt2
                            | PostgresqlType::StdPrimitiveI32AsInt4
                            | PostgresqlType::StdPrimitiveI64AsInt8
                            | PostgresqlType::StdPrimitiveF32AsFloat4
                            | PostgresqlType::StdPrimitiveF64AsFloat8
                            | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                            | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                            | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => common_standart_postgresql_type_number_filters,
                            PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(in_handle);
                                vec
                            }
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(equal_to_encoded_string_representation);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime | PostgresqlType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(current_time);
                                vec.push(greater_than_current_time);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(current_date);
                                vec.push(greater_than_current_date);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(current_timestamp);
                                vec.push(greater_than_current_timestamp);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(before);
                                vec.push(between);
                                vec
                            }
                            PostgresqlType::StdStringStringAsText | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(regular_expression);
                                vec
                            }
                            PostgresqlType::StdPrimitiveBoolAsBool | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => common_standart_postgresql_type_filters,
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = common_standart_postgresql_type_filters;
                                vec.push(greater_than);
                                vec.push(regular_expression);
                                vec
                            }
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        }
                    }
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                        let ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case = {
                            let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&match &postgresql_type.can_be_nullable() {
                                CanBeNullable::True => {
                                    let value = generate_ident_token_stream(postgresql_type, not_null_or_nullable, &PostgresqlTypePattern::Standart);
                                    quote::quote! {#value}
                                }
                                CanBeNullable::False => quote::quote! {#ident_standart_not_null_upper_camel_case},
                            });
                            quote::quote! {#value}
                        };
                        let dimension_one_greater_than = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneGreaterThan {
                            ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                        };
                        let dimension_one_between = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneBetween {
                            ident: quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case},
                        };
                        let dimension_one_in_handle = postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneIn {
                            ident: ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_upper_camel_case,
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
                            let mut vec = common_postgresql_type_filters;
                            vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqual {
                                ident: {
                                    let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&match &dimension1_not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_standart_not_null_upper_camel_case,
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &ident_standart_nullable_upper_camel_case,
                                    });
                                    quote::quote! {#value}
                                },
                            });
                            vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthEqual);
                            vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneLengthGreaterThan);
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
                            where_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                            where_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        ) = {
                            let generate_where_sqlx_postgres_types_pg_range_filter_token_stream = |postgresql_type_range: PostgresqlTypeRange| {
                                let postgresql_type_from_postgresql_type_range = PostgresqlType::from(&postgresql_type_range);
                                let range_element_ident_standart_not_null_token_stream = generate_ident_standart_not_null_token_stream(&postgresql_type_from_postgresql_type_range);
                                let mut vec = common_array_dimension1_postgresql_type_filters.clone();
                                let range_element_ident_standart_not_null_as_crate_postgresql_type_read_token_stream = {
                                    let range_element_ident_standart_not_null_as_crate_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&range_element_ident_standart_not_null_token_stream);
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
                                generate_where_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI32AsInt4),
                                generate_where_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::StdPrimitiveI64AsInt8),
                                generate_where_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateAsDate),
                                generate_where_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                                generate_where_sqlx_postgres_types_pg_range_filter_token_stream(PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                            )
                        };
                        match &postgresql_type {
                            PostgresqlType::StdPrimitiveI16AsInt2
                            | PostgresqlType::StdPrimitiveI32AsInt4
                            | PostgresqlType::StdPrimitiveI64AsInt8
                            | PostgresqlType::StdPrimitiveF32AsFloat4
                            | PostgresqlType::StdPrimitiveF64AsFloat8
                            | PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql
                            | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql
                            | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => common_array_dimension1_postgresql_type_number_filters,
                            PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_in_handle);
                                vec
                            }
                            PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlTypeFilter::DimensionOneEqualToEncodedStringRepresentation);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoNaiveTimeAsTime | PostgresqlType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_between);
                                vec.push(dimension_one_current_time);
                                vec.push(dimension_one_greater_than_current_time);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_between);
                                vec.push(dimension_one_current_date);
                                vec.push(dimension_one_greater_than_current_date);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_between);
                                vec.push(dimension_one_current_timestamp);
                                vec.push(dimension_one_greater_than_current_timestamp);
                                vec
                            }
                            PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_before);
                                vec.push(dimension_one_between);
                                vec
                            }
                            PostgresqlType::StdStringStringAsText | PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_regular_expression);
                                vec
                            }
                            PostgresqlType::StdPrimitiveBoolAsBool | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => common_array_dimension1_postgresql_type_filters,
                            PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = common_array_dimension1_postgresql_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_regular_expression);
                                vec
                            }
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                            PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        }
                    }
                }
            }
            .iter()
            .map(|current_element| {
                let current_element_handle: &dyn postgresql_crud_macros_common::PostgresqlFilter = current_element;
                current_element_handle
            })
            .collect(),
            &ident,
            &postgresql_crud_macros_common::ShouldDeriveUtoipaToSchema::False,
            &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::False,
            &postgresql_crud_macros_common::IsQueryBindMutable::False,
        );
        let ident_read_token_stream = {
            let ident_read_token_stream = {
                let (
                    derive_eq,
                    derive_partial_ord,
                    derive_ord
                ) = match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::True => (
                        macros_helpers::DeriveEq::True,
                        macros_helpers::DerivePartialOrd::True,
                        macros_helpers::DeriveOrd::True
                    ),
                    IsNotNullStandartCanBePrimaryKey::False => (
                        macros_helpers::DeriveEq::False,
                        macros_helpers::DerivePartialOrd::False,
                        macros_helpers::DeriveOrd::False
                    ),
                };
                macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy_if(derive_copy)
                    .derive_partial_eq()
                    .derive_eq_if(derive_eq)
                    .derive_partial_ord_if(derive_partial_ord)
                    .derive_ord_if(derive_ord)
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_read_upper_camel_case,
                        &ident_origin_struct_content_token_stream
                    )
            };
            let impl_ident_read_token_stream = generate_pub_const_new_or_pub_try_new_token_stream(&ident_read_upper_camel_case);
            let impl_error_occurence_lib_to_std_string_string_for_ident_read_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_read_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {self.0.to_string()});
            let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &quote::quote! {#self_snake_case.0});
            let impl_sqlx_decode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                &ident_read_upper_camel_case,
                &ident_origin_upper_camel_case,
                &quote::quote! {Ok(Self(ok_value))}
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &ident_origin_upper_camel_case);
            let maybe_impl_postgresql_type_where_filter_for_ident_read_if_can_be_primary_key_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
                postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
                    &quote::quote! {<'lifetime>},
                    &ident_standart_not_null_read_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                    &postgresql_crud_macros_common::ColumnParameterUnderscore::False,
                    &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::True,
                    &quote::quote! {
                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                            Ok(ok_value) => Ok(format!("({column} = ${ok_value})")),
                            Err(#error_snake_case) => Err(#error_snake_case)
                        }
                    },
                    &postgresql_crud_macros_common::IsQueryBindMutable::True,
                    &generate_typical_query_bind_token_stream(&naming::SelfSnakeCase),
                    &import_path,
                )
            } else {
                proc_macro2::TokenStream::new()
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
            }
        };
        let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
        let ident_read_only_ids_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
            let ident_read_only_ids_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_read_only_ids_upper_camel_case,
                    &quote::quote!{(#ident_read_upper_camel_case);},
                );
            let impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
                &ident_read_only_ids_upper_camel_case,
                &ident_read_upper_camel_case,
                &quote::quote! {Ok(Self(ok_value))}
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_read_only_ids_upper_camel_case, &ident_read_upper_camel_case);
            quote::quote! {
                #ident_read_only_ids_token_stream
                #impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_token_stream
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
        let ident_read_inner_token_stream = quote::quote! {
            pub type #ident_read_inner_upper_camel_case = #ident_inner_type_token_stream;
        };
        let ident_update_token_stream = {
            let ident_update_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_update_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_update_token_stream = generate_pub_const_new_or_pub_try_new_token_stream(&ident_update_upper_camel_case);
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_update_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            let impl_error_occurence_lib_to_std_string_string_for_ident_update_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_update_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {self.0.#to_std_string_string_snake_case()});
            quote::quote! {
                #ident_update_token_stream
                #impl_ident_update_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_update_token_stream
            }
        };
        let ident_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
        let ident_update_for_query_token_stream = {
            let ident_update_for_query_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_update_for_query_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_update_for_query_upper_camel_case, &ident_origin_upper_camel_case);
            let impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_update_for_query_upper_camel_case, &quote::quote! {#self_snake_case.0});
            let impl_std_convert_from_ident_update_for_ident_update_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_update_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)});
            quote::quote! {
                #ident_update_for_query_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_token_stream
                #impl_std_convert_from_ident_update_for_ident_update_for_query_token_stream
            }
        };
        let impl_postgresql_type_for_ident_token_stream = {
            let generate_ok_std_string_string_from_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {Ok(#std_string_string_token_stream::from(#content_token_stream))}
            };
            let ok_std_string_string_from_default_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"default"});
            let ok_std_string_string_from_uuid_generate_v4_token_stream = generate_ok_std_string_string_from_tokens_token_stream(&quote::quote! {"uuid_generate_v4()"});
            let typical_query_part_token_stream = {
                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                    &quote::quote! {#acc_snake_case, "${ok_value}"},
                    &postgresql_crud_macros_common::generate_return_err_query_part_error_named_write_into_buffer_token_stream(import_path)
                );
                quote::quote! {
                    let mut #acc_snake_case = String::default();
                    match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                        Ok(ok_value) => {
                            #if_write_is_err_token_stream
                        },
                        Err(#error_snake_case) => {
                            return Err(#error_snake_case);
                        }
                    }
                    Ok(#acc_snake_case)
                }
            };
            let ok_query_token_stream = quote::quote! {Ok(#query_snake_case)};
            let (query_part_create_token_stream, bind_value_to_query_create_token_stream): Handle<'_> = {
                let typical: Handle<'_> = { (&typical_query_part_token_stream, &typical_query_bind_token_stream) };
                let default_initialized_by_postgresql: Handle<'_> = (&ok_std_string_string_from_default_token_stream, &ok_query_token_stream);
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsInt2
                    | PostgresqlType::StdPrimitiveI32AsInt4
                    | PostgresqlType::StdPrimitiveI64AsInt8
                    | PostgresqlType::StdPrimitiveF32AsFloat4
                    | PostgresqlType::StdPrimitiveF64AsFloat8
                    | PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney
                    | PostgresqlType::StdPrimitiveBoolAsBool
                    | PostgresqlType::StdStringStringAsText
                    | PostgresqlType::StdVecVecStdPrimitiveU8AsBytea
                    | PostgresqlType::SqlxTypesChronoNaiveTimeAsTime
                    | PostgresqlType::SqlxTypesTimeTimeAsTime
                    | PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval
                    | PostgresqlType::SqlxTypesChronoNaiveDateAsDate
                    | PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                    | PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                    | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient
                    | PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet
                    | PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr
                    | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                    | PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                    | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                    | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                    | PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                    PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => default_initialized_by_postgresql,
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => (&ok_std_string_string_from_uuid_generate_v4_token_stream, &ok_query_token_stream),
                }
            };
            let select_only_ids_and_select_only_updated_ids_query_common_token_stream = {
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&{
                    let column_comma = "{column},";
                    if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key { column_comma.to_owned() } else { format!("'{{{{\\\"value\\\": null}}}}'::jsonb as {column_comma}") }
                });
                quote::quote! {Ok(format!(#format_handle_token_stream))}
            };
            postgresql_crud_macros_common::generate_impl_postgresql_type_token_stream(
                &import_path,
                &ident,
                &ident_table_type_declaration_upper_camel_case,
                &match &can_be_primary_key {
                    CanBePrimaryKey::True => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::False,
                    CanBePrimaryKey::False => postgresql_crud_macros_common::IsPrimaryKeyUnderscore::True,
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
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime | PostgresqlType::SqlxTypesTimeTimeAsTime => "time",
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval => "interval",
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate => "date",
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => "timestamp",
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => "timestamptz",
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql | PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => "uuid",
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => "inet",
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => "macaddr",
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => "int4range",
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => "int8range",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => "daterange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => "tsrange",
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => "tstzrange",
                    };
                    let maybe_array_part = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => String::new(),
                        PostgresqlTypePattern::ArrayDimension1 { .. } => std::iter::repeat_n("[]", array_dimensions_number).collect::<String>(),
                    };
                    let maybe_constraint_part = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => String::new(),
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => ",check (array_position({column},null) is null)".to_owned(),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => String::new(),
                        },
                    };
                    let maybe_primary_key_is_primary_key_token_stream = quote::quote! {maybe_primary_key(is_primary_key)};
                    let column_postgresql_query_type = format!("{{column}} {postgresql_query_type}{maybe_array_part}{maybe_constraint_part}");
                    let column_postgresql_query_type_not_null = format!("{{column}} {postgresql_query_type}{maybe_array_part} not null{maybe_constraint_part}");
                    let space_additional_parameter = " {}";
                    match (&not_null_or_nullable, &can_be_primary_key) {
                        (postgresql_crud_macros_common::NotNullOrNullable::NotNull, CanBePrimaryKey::False) => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&column_postgresql_query_type_not_null);
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
                &ident_create_upper_camel_case,
                &postgresql_crud_macros_common::CreateQueryPartValueUnderscore::True,
                &match &can_be_primary_key {
                    CanBePrimaryKey::True => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::True,
                    CanBePrimaryKey::False => postgresql_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                },
                &query_part_create_token_stream,
                &match &can_be_primary_key {
                    CanBePrimaryKey::True => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::True,
                    CanBePrimaryKey::False => postgresql_crud_macros_common::CreateQueryBindValueUnderscore::False,
                },
                &match &can_be_primary_key {
                    CanBePrimaryKey::True => postgresql_crud_macros_common::IsCreateQueryBindMutable::False,
                    CanBePrimaryKey::False => postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
                },
                &bind_value_to_query_create_token_stream,
                &ident_select_upper_camel_case,
                &match &element.postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => postgresql_crud_macros_common::SelectQueryPartValueUnderscore::True,
                    PostgresqlTypePattern::ArrayDimension1 { .. } => postgresql_crud_macros_common::SelectQueryPartValueUnderscore::False,
                },
                &{
                    let content_token_stream = match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => quote::quote! {#column_snake_case.to_string()},
                        PostgresqlTypePattern::ArrayDimension1 { .. } => {
                            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&{
                                let acc = std::iter::repeat_n("[{}:{}]", array_dimensions_number).collect::<String>();
                                format!("{{column}}{acc}")
                            });
                            let arguments_token_stream = {
                                let mut acc = Vec::new();
                                for current_element in 1..=array_dimensions_number {
                                    let dimension_number_pagination_token_stream = format!("dimension{current_element}_pagination").parse::<proc_macro2::TokenStream>().expect("6f2305ee-85e9-4dce-9a14-9e299586668a");
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
                    };
                    quote::quote! {Ok(#content_token_stream)}
                },
                &ident_where_upper_camel_case,
                &ident_read_upper_camel_case,
                &{
                    let generate_ident_read_ident_origin_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {#ident_read_upper_camel_case(#ident_origin_upper_camel_case(#content_token_stream))}
                    };
                    match &postgresql_type_pattern {
                        PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                PostgresqlTypeRange::try_from(postgresql_type).as_ref().map_or_else(
                                    |()| quote::quote! {#value_snake_case},
                                    |postgresql_type_range| {
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
                                        let generate_range_match_token_stream = |
                                            included_included_token_stream: &dyn quote::ToTokens,
                                            included_excluded_token_stream: &dyn quote::ToTokens,
                                            included_unbounded_token_stream: &dyn quote::ToTokens,
                                            excluded_included_token_stream: &dyn quote::ToTokens,
                                            excluded_excluded_token_stream: &dyn quote::ToTokens,
                                            excluded_unbounded_token_stream: &dyn quote::ToTokens,
                                            unbounded_included_token_stream: &dyn quote::ToTokens,
                                            unbounded_excluded_token_stream: &dyn quote::ToTokens
                                        | {
                                            quote::quote! {
                                                #ident_standart_not_null_read_upper_camel_case(#ident_standart_not_null_origin_upper_camel_case(match (
                                                    #value_snake_case.0.0.#start_snake_case,
                                                    #value_snake_case.0.0.#end_snake_case
                                                ) {
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
                                            let (
                                                included_start_checked_add_token_stream,
                                                excluded_end_checked_add_token_stream
                                            ) = {
                                                let generate_checked_add_one_expect_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                                                    quote::quote! {#first_token_stream(#second_token_stream.checked_add(1).expect("0ec0992f-1d63-443f-b528-7fabfff31423"))}
                                                };
                                                (
                                                    generate_checked_add_one_expect_token_stream(&included_upper_camel_case, &start_snake_case),
                                                    generate_checked_add_one_expect_token_stream(&excluded_upper_camel_case, &end_snake_case)
                                                )
                                            };
                                            let included_excluded_checked_add_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &excluded_end_checked_add_token_stream);
                                            let included_unbounded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_token_stream, &unbounded_upper_camel_case);
                                            let included_checked_add_excluded_checked_add_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_checked_add_token_stream, &excluded_end_checked_add_token_stream);
                                            let included_checked_add_excluded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_checked_add_token_stream, &excluded_end_token_stream);
                                            let included_checked_add_unbounded_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&included_start_checked_add_token_stream, &unbounded_upper_camel_case);
                                            let unbounded_excluded_checked_add_token_stream = generate_sqlx_postgres_types_pg_range_token_stream(&unbounded_upper_camel_case, &excluded_end_checked_add_token_stream);
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
                                            PostgresqlTypeRange::StdPrimitiveI32AsInt4 | PostgresqlTypeRange::StdPrimitiveI64AsInt8 => int_range_normalize_token_stream,
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
                                            PostgresqlTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp | PostgresqlTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => range_match_timestamp_and_timestamp_tz_token_stream,
                                        }
                                    }
                                )
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_ident_read_ident_origin_token_stream(&quote::quote! {
                                match #value_snake_case.0.0 {
                                    Some(some_value) => Some(
                                        <
                                            #ident_standart_not_null_upper_camel_case
                                            as
                                            #import_path::PostgresqlType
                                        >::normalize(
                                            #ident_standart_not_null_read_upper_camel_case(some_value)
                                        ).0
                                    ),
                                    None => None
                                }
                            }),
                        },
                        PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::NotNull) => generate_ident_read_ident_origin_token_stream(&quote::quote! {
                                #value_snake_case.0.0.into_iter().map(|#element_snake_case|{
                                    #ident_standart_not_null_as_postgresql_type_token_stream::normalize(
                                        #ident_standart_not_null_read_upper_camel_case(#element_snake_case)
                                    ).0
                                }).collect()
                            }),
                            (postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => generate_ident_read_ident_origin_token_stream(&{
                                let current_ident_token_stream = generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::Nullable, &PostgresqlTypePattern::Standart);
                                let ident_array_standart_nullable_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&current_ident_token_stream);
                                quote::quote! {
                                    #value_snake_case.0.0.into_iter().map(|#element_snake_case|{
                                        #ident_standart_nullable_as_postgresql_type_token_stream::normalize(
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
                                        Some(some_value) => Some(<
                                            #ident_array_dimension1_not_null_not_null_upper_camel_case
                                            as
                                            #import_path::PostgresqlType
                                        >::normalize(
                                            #ident_array_dimension1_not_null_not_null_read_upper_camel_case(some_value),
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
                                        Some(some_value) => Some(<
                                            #ident_array_dimension1_not_null_nullable_upper_camel_case
                                            as
                                            #import_path::PostgresqlType
                                        >::normalize(
                                            #ident_array_dimension1_not_null_nullable_read_upper_camel_case(some_value),
                                        ).0),
                                        None => None,
                                    }
                                }
                            }),
                        },
                    }
                },
                &if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
                    quote::quote! {#ident_read_only_ids_upper_camel_case}
                } else {
                    quote::quote! {#import_path_non_primary_key_postgresql_type_read_only_ids_token_stream}
                },
                &select_only_ids_and_select_only_updated_ids_query_common_token_stream,
                &ident_read_inner_upper_camel_case,
                &{
                    let generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {
                            #ident_standart_not_null_as_postgresql_type_token_stream::into_inner(
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
                                    generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&quote::quote!{some_value})
                                } else {
                                    quote::quote!{some_value.0}
                                };
                                quote::quote! {
                                    match #value_dot_zero_dot_zero_token_stream {
                                        Some(some_value) => Some(#content_token_stream),
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
                                    generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&quote::quote!{some_value})
                                } else {
                                    quote::quote!{some_value.0}
                                };
                                quote::quote! {
                                    #value_dot_zero_dot_zero_token_stream.into_iter().map(|#element_snake_case| match #element_snake_case.0 {
                                        Some(some_value) => Some(#content_token_stream),
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
                                        Some(some_value) => Some(some_value.0.into_iter().map(|element|#content_token_stream).collect()),
                                        None => None
                                    }
                                }
                            }
                            (postgresql_crud_macros_common::NotNullOrNullable::Nullable, postgresql_crud_macros_common::NotNullOrNullable::Nullable) => {
                                let content_token_stream = if postgresql_type_range_try_from_postgresql_type_is_ok {
                                    generate_ident_standart_not_null_into_inner_ident_standart_not_null_read_token_stream(&quote::quote!{some_value1})
                                } else {
                                    quote::quote!{some_value1.0}
                                };
                                quote::quote! {
                                    match #value_dot_zero_dot_zero_token_stream {
                                        Some(some_value0) => Some(some_value0.0.into_iter().map(|#element_snake_case| match #element_snake_case.0 {
                                            Some(some_value1) => Some(#content_token_stream),
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
                &select_only_ids_and_select_only_updated_ids_query_common_token_stream,
                &postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                &quote::quote! {Ok(#query_snake_case)},
            )
        };
        let impl_postgresql_type_test_cases_for_ident_token_stream = {
            enum IsNeedToUseInto {
                True,
                False,
            }
            let generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream = |read_or_update: &postgresql_crud_macros_common::ReadOrUpdate| {
                let read_or_update_upper_camel_case = read_or_update.upper_camel_case();
                let content_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                    quote::quote! {#try_new_snake_case(#value_snake_case).expect("69477d2f-1c78-4a08-bdb7-c84022352dee")}
                } else {
                    quote::quote! {#new_snake_case(#value_snake_case)}
                };
                quote::quote! {<#self_upper_camel_case::#postgresql_type_upper_camel_case
                    as
                #import_path::#postgresql_type_upper_camel_case>::#read_or_update_upper_camel_case:: #content_token_stream}
            };
            let generate_standart_not_null_test_case_handle_token_stream = |is_need_to_use_into: &IsNeedToUseInto| {
                let generate_range_read_only_ids_to_two_dimensional_vec_read_inner_token_stream =
                    |min_token_stream: &dyn quote::ToTokens, negative_less_typical_token_stream: &dyn quote::ToTokens, negative_more_typical_token_stream: &dyn quote::ToTokens, near_zero_token_stream: &dyn quote::ToTokens, positive_less_typical_token_stream: &dyn quote::ToTokens, positive_more_typical_token_stream: &dyn quote::ToTokens, max_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {{
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
                        }}
                    };
                let generate_int_pgrange_read_only_ids_to_two_dimensional_vec_read_inner_token_stream = |int_range_type: &IntRangeType| {
                    let range_inner_type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                    generate_range_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(&quote::quote! {#range_inner_type_token_stream::MIN}, &quote::quote! {-20}, &quote::quote! {-10}, &quote::quote! {0}, &quote::quote! {10}, &quote::quote! {20}, &quote::quote! {#range_inner_type_token_stream::MAX - 1})
                };
                let empty_vec_token_stream = quote::quote! {Vec::new()};
                let (sqlx_types_chrono_naive_time_min_token_stream, sqlx_types_chrono_naive_time_ten_token_stream, sqlx_types_chrono_naive_time_twenty_token_stream, sqlx_types_chrono_naive_time_max_token_stream) = {
                    let generate_sqlx_types_chrono_naive_time_from_hms_micro_opt_token_stream = |parameters_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {sqlx::types::chrono::NaiveTime::from_hms_micro_opt(#parameters_token_stream).expect("d25ee0e9-4a6b-4b20-b8e3-3f703e121088")}
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
                        quote::quote! {from_ymd_opt(#parameters_token_stream).expect("f0b76e36-fe9c-4d97-9893-69aaf8a307f7")}
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
                let generate_typical_test_cases_vec_token_stream = |value: &dyn quote::ToTokens| {
                    let content_token_stream = match &is_need_to_use_into {
                        IsNeedToUseInto::True => quote::quote! {.into()},
                        IsNeedToUseInto::False => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {#import_path::#value()#content_token_stream}
                };
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsInt2 => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_primitive_i16_test_cases_vec}),
                    PostgresqlType::StdPrimitiveI32AsInt4 => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_primitive_i32_test_cases_vec}),
                    PostgresqlType::StdPrimitiveI64AsInt8 => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_primitive_i64_test_cases_vec}),
                    PostgresqlType::StdPrimitiveF32AsFloat4 => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_primitive_f32_test_cases_vec}),
                    PostgresqlType::StdPrimitiveF64AsFloat8 => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_primitive_f64_test_cases_vec}),
                    PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql | PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql | PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => empty_vec_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney => quote::quote! {
                        #import_path::std_primitive_i64_test_cases_vec().into_iter().map(|#element_snake_case|
                            #inner_type_standart_not_null_token_stream(#element_snake_case)
                        ).collect::<Vec<#inner_type_standart_not_null_token_stream>>()
                    },
                    PostgresqlType::StdPrimitiveBoolAsBool => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_primitive_bool_test_cases_vec}),
                    PostgresqlType::StdStringStringAsText => generate_typical_test_cases_vec_token_stream(&quote::quote! {std_string_string_test_cases_vec}),
                    PostgresqlType::StdVecVecStdPrimitiveU8AsBytea => quote::quote! {vec![
                        Vec::new(),
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
                        let min_token_stream = quote::quote! {MIN};
                        let max_token_stream = quote::quote! {MAX};
                        let std_primitive_i32_min_token_stream = quote::quote! {#std_primitive_i32_token_stream::#min_token_stream};
                        let std_primitive_i32_max_token_stream = quote::quote! {#std_primitive_i32_token_stream::#max_token_stream};
                        let generate_sqlx_postgres_types_pg_interval_token_stream = |months_token_stream: &dyn quote::ToTokens, days_token_stream: &dyn quote::ToTokens, microseconds_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {sqlx::postgres::types::PgInterval {
                                months: #months_token_stream,
                                days: #days_token_stream,
                                microseconds: #microseconds_token_stream
                            }}
                        };
                        let min_content_token_stream = generate_sqlx_postgres_types_pg_interval_token_stream(&std_primitive_i32_min_token_stream, &std_primitive_i32_min_token_stream, &quote::quote! {#std_primitive_i64_token_stream::#min_token_stream});
                        let max_content_token_stream = generate_sqlx_postgres_types_pg_interval_token_stream(&std_primitive_i32_max_token_stream, &std_primitive_i32_max_token_stream, &quote::quote! {#std_primitive_i64_token_stream::#max_token_stream});
                        quote::quote! {vec![
                            #min_content_token_stream,
                            #max_content_token_stream
                        ]}
                    }
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
                    PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql => quote::quote! {Vec::new()},
                    PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient => quote::quote! {vec![
                        sqlx::types::Uuid::new_v4()
                    ]},
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote! {vec![
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").expect("478dbded-0912-4cb9-88e4-caddf5106628"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").expect("8af9e27e-8491-477d-821a-facc6e6344e3"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").expect("ba86505f-24fd-4f23-b2d0-3d873c357058"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").expect("32c744a0-38d5-45b6-a0b8-f744d7c7947e"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").expect("560815f8-60a6-42e2-9c9d-0edcbcc22457"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").expect("793db0ef-c8ea-4683-9782-34e304730d02"),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(192, 168, 0, 0), 24).expect("c44934f2-335e-44b7-bb4d-0a91374b4a85")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(10, 0, 0, 0), 8).expect("39e588d9-b32b-4611-a2f3-3ce500b93db0")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(127, 0, 0, 1), 32).expect("43fb25bd-03cd-44fe-bde8-dc92d8bafc71")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_snake_case(std::net::Ipv6Addr::LOCALHOST, 128).expect("b443be46-1805-4fda-b24b-71dba8d8b9d4")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_snake_case("2001:db8::".parse().expect("d4e6df27-fdb6-4e66-898c-abcfc41c5e49"), 32).expect("a7486c5e-6577-4b80-a3ec-097002698431")),
                    ]},
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {vec![
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                    ]},
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => generate_int_pgrange_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => generate_int_pgrange_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => generate_range_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(
                        &sqlx_types_chrono_naive_date_min_token_stream,
                        &sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                        &sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                        &sqlx_types_chrono_naive_date_near_zero_token_stream,
                        &sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                        &sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                        &quote::quote! {#sqlx_types_chrono_naive_date_max_token_stream.pred_opt().expect("da1f311c-1ca1-41b2-9342-50f0b92567f2")},
                    ),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => generate_range_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(
                        &sqlx_types_chrono_naive_date_time_min_token_stream,
                        &sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_near_zero_token_stream,
                        &sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_max_token_stream,
                    ),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_range_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(
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
            let option_vec_create_token_stream = {
                let generate_some_acc_content_token_stream = |
                    current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
                    current_ident_token_stream: &dyn quote::ToTokens,
                    additonal_content_token_stream: &dyn quote::ToTokens
                | {
                    let (new_or_try_new_content_token_stream, maybe_acc_push_none_token_stream) = match (&current_not_null_or_nullable, postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok()) {
                        (postgresql_crud_macros_common::NotNullOrNullable::NotNull, true) => (quote::quote! {try_new(vec![#element_snake_case.0.into()]).expect("adbae6b3-1542-4f81-89bf-48a9b895b488")}, proc_macro2::TokenStream::new()),
                        (postgresql_crud_macros_common::NotNullOrNullable::NotNull, false) => (quote::quote! {new(vec![#element_snake_case.0.into()])}, proc_macro2::TokenStream::new()),
                        (postgresql_crud_macros_common::NotNullOrNullable::Nullable, true) => (
                            quote::quote! {try_new(Some(#element_snake_case.0.into())).expect("b244d498-527d-4332-98c9-770d27e7af35")},
                            quote::quote! {#acc_snake_case.push(#self_as_postgresql_type_token_stream::Create::try_new(None).expect("31878971-17fc-4526-ab01-42c8332e641f"));},
                        ),
                        (postgresql_crud_macros_common::NotNullOrNullable::Nullable, false) => (quote::quote! {new(Some(#element_snake_case.0.into()))}, quote::quote! {#acc_snake_case.push(#self_as_postgresql_type_token_stream::Create::new(None));}),
                    };
                    let ident_as_postgresql_type_test_cases_token_stream = generate_as_postgresql_type_test_cases_token_stream(&current_ident_token_stream);
                    quote::quote! {Some({
                        let mut #acc_snake_case = Vec::new();
                        for #element_snake_case in #ident_as_postgresql_type_test_cases_token_stream::#option_vec_create_snake_case().unwrap_or(Vec::new()) {
                            #acc_snake_case.push(#self_as_postgresql_type_token_stream::Create::#new_or_try_new_content_token_stream);
                        }
                        #maybe_acc_push_none_token_stream
                        #additonal_content_token_stream
                        #acc_snake_case
                    })}
                };
                match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &can_be_primary_key {
                            CanBePrimaryKey::True => none_token_stream.clone(),
                            CanBePrimaryKey::False => {
                                let content_token_stream = generate_standart_not_null_test_case_handle_token_stream(&IsNeedToUseInto::False);
                                let new_or_try_new_token_stream = if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                                    quote::quote! {try_new(#element_snake_case).expect("941bd15c-a751-45e7-8266-f17df4ee00aa")}
                                } else {
                                    quote::quote! {new(#element_snake_case)}
                                };
                                quote::quote! {Some(
                                    #content_token_stream.into_iter().map(|#element_snake_case|
                                        #self_as_postgresql_type_token_stream::Create::#new_or_try_new_token_stream
                                    ).collect()
                                )}
                            }
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_some_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(postgresql_type, &postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlTypePattern::Standart), &proc_macro2::TokenStream::new()),
                    },
                    PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_some_acc_content_token_stream(
                        not_null_or_nullable,
                        &generate_ident_token_stream(
                            postgresql_type,
                            &match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                            },
                            &match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => PostgresqlTypePattern::Standart,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable },
                            },
                        ),
                        &match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                                let content_token_stream: &dyn quote::ToTokens = match &dimension1_not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_standart_not_null_as_postgresql_type_test_cases_token_stream,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &ident_standart_nullable_as_postgresql_type_test_cases_token_stream,
                                };
                                let (first_token_stream, second_token_stream, third_token_stream) = {
                                    let generate_new_or_try_new_token_stream = |current_content_token_stream: &dyn quote::ToTokens| {
                                        if postgresql_type_initialization_try_new_try_from_postgresql_type.is_ok() {
                                            quote::quote! {try_new(#current_content_token_stream).expect("75ad9383-b257-4a0b-bd8d-c931950bf745")}
                                        } else {
                                            quote::quote! {new(#current_content_token_stream)}
                                        }
                                    };
                                    let generate_vec_value_clone_zero_into_number_token_stream = |value: usize| {
                                        let number_token_stream = value.to_string().parse::<proc_macro2::TokenStream>().expect("50c87202-4038-4b27-85bd-c0593552bb89");
                                        quote::quote! {vec![some_value.clone().0.into(); #number_token_stream]}
                                    };
                                    (
                                        generate_new_or_try_new_token_stream(&quote::quote! {
                                            #content_token_stream::#option_vec_create_snake_case().unwrap_or(Vec::new())
                                            .into_iter()
                                            .map(|#element_snake_case|#element_snake_case.0.into())
                                            .collect()
                                        }),
                                        generate_new_or_try_new_token_stream(&generate_vec_value_clone_zero_into_number_token_stream(2)),
                                        generate_new_or_try_new_token_stream(&generate_vec_value_clone_zero_into_number_token_stream(1000)),
                                    )
                                };
                                quote::quote! {
                                    #acc_snake_case.push(#ident_as_postgresql_type_token_stream::Create::#first_token_stream);
                                    if let Some(some_value) = #content_token_stream::#option_vec_create_snake_case().unwrap_or(Vec::new()).get(0) {
                                        #acc_snake_case.push(#ident_as_postgresql_type_token_stream::Create::#second_token_stream);
                                        #acc_snake_case.push(#ident_as_postgresql_type_token_stream::Create::#third_token_stream);
                                    }
                                }
                            }
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                    ),
                }
            };
            let read_only_ids_to_two_dimensional_vec_read_inner_token_stream = match &postgresql_type_pattern {
                PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                        let content_token_stream = generate_standart_not_null_test_case_handle_token_stream(&IsNeedToUseInto::True);
                        quote::quote! {vec![{#content_token_stream}]}
                    }
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                        let mut #acc_snake_case = Vec::new();
                        for element0 in #ident_standart_not_null_as_postgresql_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(#read_only_ids_snake_case) {
                            for element1 in element0 {
                                #acc_snake_case.push(vec![Some(element1)]);
                            }
                        }
                        #acc_snake_case.push(vec![None]);
                        #acc_snake_case
                    },
                },
                PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &dimension1_not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            quote::quote! {
                                let mut #acc_snake_case = Vec::new();
                                let read_only_ids_to_two_dimensional_vec_read_inner = #ident_standart_not_null_as_postgresql_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(#read_only_ids_snake_case);
                                let option_additional = {
                                    let mut option_additional = None;
                                    for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                        if option_additional.is_some() {
                                            break;
                                        }
                                        for element1 in element0 {
                                            if option_additional.is_none() {
                                                option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                                            }
                                            else {
                                                break;
                                            }
                                        }
                                    }
                                    option_additional
                                };
                                let has_len_greater_than_one = {
                                    let mut has_len_greater_than_one = false;
                                    for #element_snake_case in &read_only_ids_to_two_dimensional_vec_read_inner {
                                        if #element_snake_case.len() > 1 {
                                            has_len_greater_than_one = true;
                                            break;
                                        }
                                    }
                                    has_len_greater_than_one
                                };
                                for #element_snake_case in read_only_ids_to_two_dimensional_vec_read_inner {
                                    #acc_snake_case.push(vec![#element_snake_case]);
                                }
                                if let Some(some_value) = option_additional {
                                    if has_len_greater_than_one {
                                        #acc_snake_case.push(some_value.0);
                                    }
                                    if !has_len_greater_than_one {
                                        #acc_snake_case.push(some_value.1);
                                    }
                                }
                                #acc_snake_case
                            }
                        }
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            quote::quote! {
                                let mut #acc_snake_case = Vec::new();
                                let read_only_ids_to_two_dimensional_vec_read_inner = #ident_standart_nullable_as_postgresql_type_test_cases_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(#read_only_ids_snake_case);
                                let option_additional = {
                                    let mut option_additional = None;
                                    for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                        if option_additional.is_some() {
                                            break;
                                        }
                                        for element1 in element0 {
                                            if option_additional.is_none() {
                                                option_additional = Some((vec![vec![element1.clone()]], vec![vec![element1.clone(), element1.clone()]]));
                                            }
                                            else {
                                                break;
                                            }
                                        }
                                    }
                                    option_additional
                                };
                                let has_len_greater_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
                                #acc_snake_case.push({
                                    let mut #acc_snake_case = Vec::new();
                                    for element0 in read_only_ids_to_two_dimensional_vec_read_inner {
                                        for element1 in element0 {
                                            #acc_snake_case.push(element1);
                                        }
                                    }
                                    vec![#acc_snake_case]
                                });
                                if let Some(some_value) = option_additional {
                                    if has_len_greater_than_one {
                                        #acc_snake_case.push(some_value.0);
                                    }
                                    if !has_len_greater_than_one {
                                        #acc_snake_case.push(some_value.1);
                                    }
                                }
                                #acc_snake_case
                            }
                        }
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                        let content_token_stream = match &dimension1_not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_array_not_null_as_postgresql_type_test_cases_token_stream,
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => &ident_array_nullable_as_postgresql_type_test_cases_token_stream,
                        };
                        quote::quote! {
                            let mut #acc_snake_case = Vec::new();
                            let read_only_ids_to_two_dimensional_vec_read_inner = #content_token_stream::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(#read_only_ids_snake_case);
                            let option_additional = {
                                let mut option_additional = None;
                                for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                    if option_additional.is_some() {
                                        break;
                                    }
                                    for element1 in element0 {
                                        if option_additional.is_some() {
                                            break;
                                        }
                                        for element2 in element1 {
                                            if option_additional.is_none() {
                                                option_additional = Some((
                                                    vec![Some(vec![element2.clone()])],
                                                    vec![Some(vec![element2.clone(), element2.clone()])]
                                                ));
                                            }
                                            else {
                                                break;
                                            }
                                        }
                                    }
                                }
                                option_additional
                            };
                            let has_len_greater_than_one = {
                                let mut has_len_greater_than_one = false;
                                for element0 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                    for element1 in element0 {
                                        if element1.len() > 1 {
                                            has_len_greater_than_one = true;
                                            break;
                                        }
                                    }
                                }
                                has_len_greater_than_one
                            };
                            #acc_snake_case.push(vec![Some({
                                let mut #acc_snake_case = Vec::new();
                                for element0 in read_only_ids_to_two_dimensional_vec_read_inner.clone() {
                                    for element1 in element0 {
                                        for element2 in element1 {
                                            #acc_snake_case.push(element2);
                                        }
                                    }
                                }
                                #acc_snake_case
                            })]);
                            #acc_snake_case.push(vec![None]);
                            if let Some(some_value) = option_additional {
                                if has_len_greater_than_one {
                                    #acc_snake_case.push(some_value.0);
                                }
                                if !has_len_greater_than_one {
                                    #acc_snake_case.push(some_value.1);
                                }
                            }
                            #acc_snake_case
                        }
                    }
                },
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Update);
            let update_to_read_only_ids_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
                quote::quote! {
                    #ident_read_only_ids_upper_camel_case(#ident_read_upper_camel_case(#value_snake_case.0.clone()))//todo its not correct. must be only for primary key but it for all types what van be primary key
                }
            } else {
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&none_token_stream);
                quote::quote! {
                    #import_path_non_primary_key_postgresql_type_read_only_ids_token_stream(#value_initialization_token_stream)
                }
            };
            let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = {
            //todo that is not correct for array of generated by postgresql primary keys but maybe just need to remove this variants and thats it?
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                    let content_token_stream: &dyn quote::ToTokens = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
                        &quote::quote! {#value_snake_case.0.clone()}
                    } else {
                        &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    };
                    quote::quote! {#self_postgresql_type_as_postgresql_type_token_stream::normalize(#content_token_stream)}
                });
                quote::quote! {Some(#value_initialization_token_stream)}
            };
            let previous_read_merged_with_option_update_into_read_token_stream = quote::quote! {
                #option_update_snake_case.map_or(#read_snake_case, |#value_snake_case| #ident_read_upper_camel_case(#value_snake_case.0))
            };
            let read_only_ids_merged_with_create_into_read_token_stream = {
                let content_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
                    quote::quote! {#read_only_ids_snake_case.0}
                } else {
                    quote::quote! {#ident_read_upper_camel_case(#create_snake_case.0)}
                };
                quote::quote! {
                    #self_postgresql_type_as_postgresql_type_token_stream::normalize(#content_token_stream)
                }
            };
            let read_only_ids_merged_with_create_into_option_value_read_token_stream = {
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote! {
                    <Self as #import_path::PostgresqlTypeTestCases>::#read_only_ids_merged_with_create_into_read_snake_case(
                        #read_only_ids_snake_case,
                        #create_snake_case
                    )
                });
                quote::quote! {Some(#value_initialization_token_stream)}
            };
            let read_only_ids_merged_with_create_into_table_type_declaration_token_stream = {
                let content_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
                    quote::quote! {#read_only_ids_snake_case.0.0}
                } else {
                    quote::quote! {#create_snake_case.0}
                };
                quote::quote! {#ident_table_type_declaration_upper_camel_case(#content_token_stream)}
            };
            //todo maybe it into function (not in proc macro)
            let read_only_ids_merged_with_create_into_where_equal_token_stream = {
                let content_token_stream = if let PostgresqlTypePattern::Standart = &postgresql_type_pattern
                    && let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
                    && let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key
                {
                    quote::quote! {#read_only_ids_snake_case.0.0}
                } else {
                    quote::quote! {#create_snake_case.0}
                };
                quote::quote! {
                    #ident_where_upper_camel_case::#equal_upper_camel_case(where_filters::PostgresqlTypeWhereEqual {
                        logical_operator: #import_path::LogicalOperator::Or,
                        #value_snake_case: #ident_table_type_declaration_upper_camel_case(#content_token_stream),
                    })
                }
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream = quote::quote! {
                vec![#read_only_ids_merged_with_create_into_where_equal_token_stream]
            };
            let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream = none_token_stream.clone();
            let create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream = match &postgresql_type_pattern {
                PostgresqlTypePattern::Standart => none_token_stream.clone(),
                PostgresqlTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    let ident_standart_not_null_or_nullable_table_type_declaration_upper_camel_case: &dyn quote::ToTokens = match &dimension1_not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_standart_not_null_table_type_declaration_upper_camel_case,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => &ident_standart_nullable_table_type_declaration_upper_camel_case,
                    };
                    let some_token_stream = {
                        let content_token_stream: &dyn quote::ToTokens = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => &quote::quote! {#create_snake_case.0.0},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {some_value.0},
                        };
                        quote::quote! {
                            Some({
                                let mut #acc_snake_case = Vec::new();
                                for (i, #element_snake_case) in #content_token_stream.into_iter().enumerate() {
                                    let index = i.checked_add(1).expect("a6eeace9-fc74-48ad-af8c-673a01c3d0b4");
                                    #acc_snake_case.push(
                                        #ident_where_upper_camel_case::DimensionOneEqual(
                                            where_filters::PostgresqlTypeWhereDimensionOneEqual {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                dimensions: where_filters::BoundedStdVecVec::try_from(
                                                    vec![
                                                        postgresql_crud_common::NotZeroUnsignedPartOfStdPrimitiveI32::try_from(
                                                            i32::try_from(index).expect("5954966c-571a-4744-ba04-9806fc7e63c9")
                                                        ).expect("8d269b8f-41db-4fd9-b33a-e0c532593163")
                                                    ]
                                                ).expect("fe1e037f-70ce-4744-b34b-0413754e6fb0"),
                                                #value_snake_case: #ident_standart_not_null_or_nullable_table_type_declaration_upper_camel_case(#element_snake_case),
                                            }
                                        )
                                    );
                                }
                                #acc_snake_case
                            })
                        }
                    };
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => some_token_stream,
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            match #create_snake_case.0.0 {
                                Some(some_value) => #some_token_stream,
                                None => None
                            }
                        },
                    }
                }
            };
            let postgresql_type_option_vec_where_greater_than_test_token_stream = {
                let greater_than = postgresql_crud_common_and_macros_common::PostgresqlTypeGreaterThanVariant::GreaterThan;
                let not_greater_than = postgresql_crud_common_and_macros_common::PostgresqlTypeGreaterThanVariant::NotGreaterThan;
                let equal_not_greater_than = postgresql_crud_common_and_macros_common::PostgresqlTypeGreaterThanVariant::EqualNotGreaterThan;
                let generate_greater_than_test_token_stream = |greater_than_variant_token_stream: &postgresql_crud_common_and_macros_common::PostgresqlTypeGreaterThanVariant, create_content_token_stream: &dyn quote::ToTokens, table_type_declaration_content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        #import_path::PostgresqlTypeGreaterThanTest {
                            variant: #import_path::PostgresqlTypeGreaterThanVariant::#greater_than_variant_token_stream,
                            create: #self_as_postgresql_type_token_stream::Create::#create_content_token_stream,
                            greater_than: #self_as_postgresql_type_token_stream::TableTypeDeclaration::#table_type_declaration_content_token_stream,
                        }
                    }
                };
                let generate_greater_than_test_new_new_token_stream =
                    |greater_than_variant_token_stream: &postgresql_crud_common_and_macros_common::PostgresqlTypeGreaterThanVariant, create_token_stream: &dyn quote::ToTokens, greater_than_token_stream: &dyn quote::ToTokens| generate_greater_than_test_token_stream(greater_than_variant_token_stream, &quote::quote! {new(#create_token_stream)}, &quote::quote! {new(#greater_than_token_stream)});
                let generate_greater_than_test_try_new_try_new_token_stream = |greater_than_variant_token_stream: &postgresql_crud_common_and_macros_common::PostgresqlTypeGreaterThanVariant, create_token_stream: &dyn quote::ToTokens, greater_than_token_stream: &dyn quote::ToTokens| {
                    generate_greater_than_test_token_stream(
                        greater_than_variant_token_stream,
                        &quote::quote! {try_new(#create_token_stream).expect("8327c651-9a52-470f-b5ab-dd2680b2f5e1")},
                        &quote::quote! {try_new(#greater_than_token_stream).expect("c369e6ea-4420-4087-b09a-88f0bbfcb2fe")},
                    )
                };
                let generate_greater_than_test_new_new_vec_token_stream = |less_token_stream: &dyn quote::ToTokens, less_with_more_token_stream: &dyn quote::ToTokens, zero_token_stream: &dyn quote::ToTokens, one_token_stream: &dyn quote::ToTokens, more_token_stream: &dyn quote::ToTokens, more_with_less_token_stream: &dyn quote::ToTokens| {
                    let greater_than_less_token_stream = generate_greater_than_test_new_new_token_stream(&greater_than, &less_with_more_token_stream, &less_token_stream);
                    let greater_than_zero_token_stream = generate_greater_than_test_new_new_token_stream(&greater_than, &one_token_stream, &zero_token_stream);
                    let greater_than_more_token_stream = generate_greater_than_test_new_new_token_stream(&greater_than, &more_token_stream, &more_with_less_token_stream);
                    let not_greater_than_less_token_stream = generate_greater_than_test_new_new_token_stream(&not_greater_than, &less_token_stream, &less_with_more_token_stream);
                    let not_greater_than_zero_token_stream = generate_greater_than_test_new_new_token_stream(&not_greater_than, &zero_token_stream, &one_token_stream);
                    let not_greater_than_more_token_stream = generate_greater_than_test_new_new_token_stream(&not_greater_than, &more_with_less_token_stream, &more_token_stream);
                    let equal_not_greater_than_less_token_stream = generate_greater_than_test_new_new_token_stream(&equal_not_greater_than, &less_token_stream, &less_token_stream);
                    let equal_not_greater_than_zero_token_stream = generate_greater_than_test_new_new_token_stream(&equal_not_greater_than, &zero_token_stream, &zero_token_stream);
                    let equal_not_greater_than_more_token_stream = generate_greater_than_test_new_new_token_stream(&equal_not_greater_than, &more_token_stream, &more_token_stream);
                    quote::quote! {
                        #greater_than_less_token_stream,
                        #greater_than_zero_token_stream,
                        #greater_than_more_token_stream,
                        #not_greater_than_less_token_stream,
                        #not_greater_than_zero_token_stream,
                        #not_greater_than_more_token_stream,
                        #equal_not_greater_than_less_token_stream,
                        #equal_not_greater_than_zero_token_stream,
                        #equal_not_greater_than_more_token_stream
                    }
                };
                let generate_greater_than_test_try_new_try_new_vec_token_stream = |less_token_stream: &dyn quote::ToTokens, less_with_more_token_stream: &dyn quote::ToTokens, zero_token_stream: &dyn quote::ToTokens, one_token_stream: &dyn quote::ToTokens, more_token_stream: &dyn quote::ToTokens, more_with_less_token_stream: &dyn quote::ToTokens| {
                    let greater_than_less_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&greater_than, &less_with_more_token_stream, &less_token_stream);
                    let greater_than_zero_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&greater_than, &one_token_stream, &zero_token_stream);
                    let greater_than_more_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&greater_than, &more_token_stream, &more_with_less_token_stream);
                    let not_greater_than_less_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&not_greater_than, &less_token_stream, &less_with_more_token_stream);
                    let not_greater_than_zero_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&not_greater_than, &zero_token_stream, &one_token_stream);
                    let not_greater_than_more_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&not_greater_than, &more_with_less_token_stream, &more_token_stream);
                    let equal_not_greater_than_less_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&equal_not_greater_than, &less_token_stream, &less_token_stream);
                    let equal_not_greater_than_zero_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&equal_not_greater_than, &zero_token_stream, &zero_token_stream);
                    let equal_not_greater_than_more_token_stream = generate_greater_than_test_try_new_try_new_token_stream(&equal_not_greater_than, &more_token_stream, &more_token_stream);
                    quote::quote! {
                        #greater_than_less_token_stream,
                        #greater_than_zero_token_stream,
                        #greater_than_more_token_stream,
                        #not_greater_than_less_token_stream,
                        #not_greater_than_zero_token_stream,
                        #not_greater_than_more_token_stream,
                        #equal_not_greater_than_less_token_stream,
                        #equal_not_greater_than_zero_token_stream,
                        #equal_not_greater_than_more_token_stream
                    }
                };
                match &postgresql_type_pattern {
                    PostgresqlTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            let wrap_into_some_vec_token_stream = |content_token_stream: &dyn quote::ToTokens| quote::quote! {Some(vec![#content_token_stream])};
                            match &postgresql_type {
                                PostgresqlType::StdPrimitiveI16AsInt2 => wrap_into_some_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote!{#std_primitive_i16_token_stream::MIN},
                                    &quote::quote!{#std_primitive_i16_token_stream::MIN + 1},
                                    &quote::quote!{0},
                                    &quote::quote!{1},
                                    &quote::quote!{#std_primitive_i16_token_stream::MAX},
                                    &quote::quote!{#std_primitive_i16_token_stream::MAX - 1}
                                )),
                                PostgresqlType::StdPrimitiveI32AsInt4 => wrap_into_some_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote!{#std_primitive_i32_token_stream::MIN},
                                    &quote::quote!{#std_primitive_i32_token_stream::MIN + 1},
                                    &quote::quote!{0},
                                    &quote::quote!{1},
                                    &quote::quote!{#std_primitive_i32_token_stream::MAX},
                                    &quote::quote!{#std_primitive_i32_token_stream::MAX - 1}
                                )),
                                PostgresqlType::StdPrimitiveI64AsInt8 => wrap_into_some_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote!{#std_primitive_i64_token_stream::MIN},
                                    &quote::quote!{#std_primitive_i64_token_stream::MIN + 1},
                                    &quote::quote!{0},
                                    &quote::quote!{1},
                                    &quote::quote!{#std_primitive_i64_token_stream::MAX},
                                    &quote::quote!{#std_primitive_i64_token_stream::MAX - 1}
                                )),
                                PostgresqlType::StdPrimitiveF32AsFloat4 => wrap_into_some_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote!{#std_primitive_f32_token_stream::MIN},
                                    &quote::quote!{#std_primitive_f32_token_stream::MIN.next_up()},
                                    &quote::quote!{0.0},
                                    &quote::quote!{1.0},
                                    &quote::quote!{#std_primitive_f32_token_stream::MAX},
                                    &quote::quote!{#std_primitive_f32_token_stream::MAX.next_down()}
                                )),
                                PostgresqlType::StdPrimitiveF64AsFloat8 => wrap_into_some_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                //todo rust f64 != postgresql float8
                                    &quote::quote!{-2.0},
                                    &quote::quote!{-2.0 + 1.0},
                                    &quote::quote!{0.0},
                                    &quote::quote!{1.0},
                                    &quote::quote!{2.0},
                                    &quote::quote!{2.0 - 1.0}
                                )),
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsTime => wrap_into_some_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote!{sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("dbdd9f7a-85d8-424b-9124-1e93a559436b")},
                                    &quote::quote!{sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("d81d1eae-747f-416f-aa9d-288212b4ae73")},
                                    &quote::quote!{sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("be06a120-2bbf-445c-a242-1aaffa121af3")},
                                    &quote::quote!{sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("30d2c08a-82e2-4a5b-8d6b-b01bd58a00bd")},
                                    &quote::quote!{sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("bf537b26-f99c-4f1e-a7f5-df224bdfb4fc")},
                                    &quote::quote!{sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_998).expect("e7a13c77-47ba-4fe2-a08d-d257d1dbd2b8")},
                                )),
                                PostgresqlType::SqlxTypesTimeTimeAsTime => wrap_into_some_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote!{sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("d267f56d-c59b-4476-b7e4-1747aa9b4952")},
                                    &quote::quote!{sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("34bf1b76-006c-4555-b6d9-2f51c4eb9077")},
                                    &quote::quote!{sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("ff1e7a13-9b5c-4c22-afb5-71d083828bbf")},
                                    &quote::quote!{sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("126d0de8-116d-4620-9bb5-96eed23b99f5")},
                                    &quote::quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_999).expect("f3d895bb-64a0-47c5-819d-f31b9b5f4ba3")},
                                    &quote::quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_998).expect("1e71f8c6-49a0-47cd-80e4-a4f92666af78")},
                                )),
                                PostgresqlType::SqlxTypesChronoNaiveDateAsDate => wrap_into_some_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 30).expect("42b5d349-449f-499f-b456-f5670993a767")},//todo not sure about this values. maybe reuse
                                    &quote::quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 31).expect("8ccd5702-e11b-4b2f-bf6b-9245396412e3")},
                                    &quote::quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("4f4d7790-f760-4ea4-861e-e2b383585b1e")},
                                    &quote::quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 2).expect("6be0d837-eb33-45c7-b062-8dfac94f7edf")},
                                    &quote::quote!{sqlx::types::chrono::NaiveDate::MAX},
                                    &quote::quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(262_142, 12, 30).expect("3d6d30fc-20eb-42ba-8f1a-b67257f4eaeb")},
                                )),
                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => wrap_into_some_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).expect("22bd692f-3ba1-42a9-a053-4cfb3892db7d"),
                                        sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("ae30636e-1432-430e-9ffc-a7c7767f5a99")
                                    )},
                                    &quote::quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31).expect("44c79b13-f4f2-4246-b6eb-7841024e8e3d"),
                                        sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("053b3611-6094-42c0-ad56-eb4c5218b925")
                                    )},
                                    &quote::quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("f75ab479-3d69-41c4-8e0f-c3a02852466b"),
                                        sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("34db2475-c4ce-44d2-848b-84efadd2769f")
                                    )},
                                    &quote::quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("9081aea2-a9d3-4aa5-bb88-c188269f86f2"),
                                        sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("00db92a8-f1bb-427c-abcb-62c259589840")
                                    )},
                                    &quote::quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("f8dc04ce-285a-4b40-86c9-12d20fe90073")
                                    )},
                                    &quote::quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_998).expect("ab27057c-f47b-4999-bbf7-d0543eabb82e")
                                    )},
                                )),
                                PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql |//todo diffrent test logic for autogenerated?
                                PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql |//todo diffrent test logic for autogenerated?
                                PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql |//todo diffrent test logic for autogenerated?
                                PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney |
                                PostgresqlType::StdPrimitiveBoolAsBool |
                                PostgresqlType::StdStringStringAsText |
                                PostgresqlType::StdVecVecStdPrimitiveU8AsBytea |
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval |
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql |
                                PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet |
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => none_token_stream.clone(),
                            }
                        }
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                            match <#ident_standart_not_null_upper_camel_case as #import_path::PostgresqlTypeTestCases>::postgresql_type_option_vec_where_greater_than_test() {
                                Some(some_value) => Some(
                                    some_value
                                    .into_iter()
                                    .map(|#element_snake_case|#import_path::PostgresqlTypeGreaterThanTest {
                                        variant: #element_snake_case.variant,
                                        create: #ident_create_upper_camel_case(#ident_origin_upper_camel_case(Some(#element_snake_case.create.0))),
                                        greater_than: #ident_table_type_declaration_upper_camel_case(#ident_origin_upper_camel_case(Some(#element_snake_case.greater_than.0))),
                                    }).collect()
                                ),
                                None => None
                            }
                        },
                    },
                    PostgresqlTypePattern::ArrayDimension1 { .. } => none_token_stream.clone(),
                }
            };
            let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream = match &postgresql_type_pattern {
                PostgresqlTypePattern::Standart => {
                    enum IsNeedToImplPostgresqlTypeGreaterThanTest {
                        TrueFromReadOnlyIds,
                        TrueFromCreate,
                        False,
                    }
                    enum ReadOnlyIdsCreate {
                        ReadOnlyIds,
                        Create,
                    }
                    let is_need_to_impl_greater_than_test = match &postgresql_type {
                        PostgresqlType::StdPrimitiveI16AsInt2 |
                        PostgresqlType::StdPrimitiveI32AsInt4 |
                        PostgresqlType::StdPrimitiveI64AsInt8 |
                        PostgresqlType::StdPrimitiveF32AsFloat4 |
                        PostgresqlType::StdPrimitiveF64AsFloat8 |
                        PostgresqlType::SqlxTypesChronoNaiveTimeAsTime |
                        PostgresqlType::SqlxTypesTimeTimeAsTime |
                        PostgresqlType::SqlxTypesChronoNaiveDateAsDate |
                        PostgresqlType::SqlxTypesChronoNaiveDateTimeAsTimestamp => IsNeedToImplPostgresqlTypeGreaterThanTest::TrueFromCreate,
                        PostgresqlType::StdPrimitiveI16AsSmallSerialInitializedByPostgresql |
                        PostgresqlType::StdPrimitiveI32AsSerialInitializedByPostgresql |
                        PostgresqlType::StdPrimitiveI64AsBigSerialInitializedByPostgresql => IsNeedToImplPostgresqlTypeGreaterThanTest::TrueFromReadOnlyIds,
                        PostgresqlType::SqlxPostgresTypesPgMoneyAsMoney |//todo why no support?
                        PostgresqlType::StdPrimitiveBoolAsBool |
                        PostgresqlType::StdStringStringAsText |
                        PostgresqlType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresqlType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |//todo why no support?
                        PostgresqlType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql |
                        PostgresqlType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresqlType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresqlType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => IsNeedToImplPostgresqlTypeGreaterThanTest::False,
                    };
                    let generate_some_token_stream = |read_only_ids_create: &ReadOnlyIdsCreate| match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                            let content_token_stream = match &read_only_ids_create {
                                ReadOnlyIdsCreate::ReadOnlyIds => quote::quote! {#ident_standart_not_null_table_type_declaration_upper_camel_case(#read_only_ids_snake_case.0.0)},
                                ReadOnlyIdsCreate::Create => quote::quote! {table_type_declaration},
                            };
                            quote::quote! {
                                Some(
                                    #ident_where_upper_camel_case::GreaterThan(
                                        where_filters::PostgresqlTypeWhereGreaterThan {
                                            logical_operator: greater_than_variant.logical_operator(),
                                            #value_snake_case: #content_token_stream,
                                        }
                                    )
                                )
                            }
                        }
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                            let content_token_stream = match &read_only_ids_create {
                                ReadOnlyIdsCreate::ReadOnlyIds => quote::quote! {#read_only_ids_snake_case.0},
                                ReadOnlyIdsCreate::Create => quote::quote! {#table_type_declaration_snake_case.0.0},
                            };
                            quote::quote! {
                                match #content_token_stream {
                                    Some(some_value) => Some(
                                        #ident_where_upper_camel_case::GreaterThan(
                                            where_filters::PostgresqlTypeWhereGreaterThan {
                                                logical_operator: greater_than_variant.logical_operator(),
                                                #value_snake_case: #ident_standart_not_null_table_type_declaration_upper_camel_case(some_value),
                                            }
                                        )
                                    ),
                                    None => None
                                }
                            }
                        }
                    };
                    match &is_need_to_impl_greater_than_test {
                        IsNeedToImplPostgresqlTypeGreaterThanTest::TrueFromReadOnlyIds => generate_some_token_stream(&ReadOnlyIdsCreate::ReadOnlyIds),
                        IsNeedToImplPostgresqlTypeGreaterThanTest::TrueFromCreate => generate_some_token_stream(&ReadOnlyIdsCreate::Create),
                        IsNeedToImplPostgresqlTypeGreaterThanTest::False => none_token_stream.clone(),
                    }
                }
                PostgresqlTypePattern::ArrayDimension1 { .. } => none_token_stream.clone(),
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream = none_token_stream.clone();
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream = none_token_stream.clone();
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream = none_token_stream.clone();
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream = none_token_stream.clone();
            let create_into_postgresql_json_type_option_vec_where_length_equal_token_stream = none_token_stream.clone();
            let postgresql_json_type_option_vec_where_length_greater_than_test_token_stream = quote::quote! {todo!()};
            let create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream = none_token_stream;
            postgresql_crud_macros_common::generate_impl_postgresql_type_test_cases_for_ident_token_stream(
                &quote::quote! {#[cfg(feature = "test-utils")]},
                &import_path,
                &ident_inner_type_token_stream,
                &ident,
                &option_vec_create_token_stream,
                &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
                &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
                &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
                &update_to_read_only_ids_token_stream,
                &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                &previous_read_merged_with_option_update_into_read_token_stream,
                &read_only_ids_merged_with_create_into_read_token_stream,
                &read_only_ids_merged_with_create_into_option_value_read_token_stream,
                &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
                &read_only_ids_merged_with_create_into_where_equal_token_stream,
                &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
                &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream,
                &create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream,
                &postgresql_type_option_vec_where_greater_than_test_token_stream,
                &read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream,
                &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
                &postgresql_json_type_option_vec_where_length_greater_than_test_token_stream,
                &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
            )
        };
        let maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
            let postgresql_type_primary_key_upper_camel_case = naming::PostgresqlTypePrimaryKeyUpperCamelCase;
            let value_as_read_only_ids_token_stream = quote::quote! {#value_snake_case: #self_as_postgresql_type_token_stream::#read_only_ids_upper_camel_case};
            quote::quote! {
                impl #import_path::#postgresql_type_primary_key_upper_camel_case for #ident_standart_not_null_upper_camel_case {
                    type #postgresql_type_upper_camel_case = Self;
                    type #table_type_declaration_upper_camel_case = #ident_standart_not_null_table_type_declaration_upper_camel_case;
                    fn #read_only_ids_into_table_type_declaration_snake_case(#value_as_read_only_ids_token_stream) -> #self_as_postgresql_type_token_stream::#table_type_declaration_upper_camel_case {
                        #ident_table_type_declaration_upper_camel_case(#value_snake_case.0.0)
                    }
                    fn #read_only_ids_into_read_snake_case(#value_as_read_only_ids_token_stream) -> #self_as_postgresql_type_token_stream::#read_upper_camel_case {
                        #value_snake_case.0
                    }
                    fn #read_only_ids_into_update_snake_case(#value_as_read_only_ids_token_stream) -> #self_as_postgresql_type_token_stream::#update_upper_camel_case {
                        #ident_update_upper_camel_case(#value_snake_case.0.0)
                    }
                    fn #read_into_table_type_declaration_snake_case(
                        #value_snake_case: #self_as_postgresql_type_token_stream::#read_upper_camel_case
                    ) -> #self_as_postgresql_type_token_stream::#table_type_declaration_upper_camel_case {
                        #ident_table_type_declaration_upper_camel_case(#value_snake_case.0)
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let maybe_impl_postgresql_type_not_primary_key_for_ident_token_stream = if let IsNotNullStandartCanBePrimaryKey::True = &is_not_null_standart_can_be_primary_key {
            proc_macro2::TokenStream::new()
        } else {
            postgresql_crud_macros_common::generate_impl_postgresql_type_not_primary_key_for_ident_token_stream(&import_path, &ident)
        };
        let generated = quote::quote! {
            #ident_token_stream
            #ident_origin_token_stream
            #ident_table_type_declaration_token_stream
            #ident_create_token_stream
            #ident_select_token_stream
            #ident_where_token_stream
            #ident_read_token_stream
            #ident_read_only_ids_token_stream
            #ident_read_inner_token_stream
            #ident_update_token_stream
            #ident_update_for_query_token_stream
            #impl_postgresql_type_for_ident_token_stream
            #impl_postgresql_type_test_cases_for_ident_token_stream
            #maybe_impl_postgresql_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_token_stream
            #maybe_impl_postgresql_type_not_primary_key_for_ident_token_stream
        };
        (
            {
                let field_ident = format!("column_{index}").parse::<proc_macro2::TokenStream>().expect("2e15af68-48bd-4192-bd45-aacf8086d76b");
                quote::quote! {
                    pub #field_ident: postgresql_crud::postgresql_type:: #ident,
                }
                .to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    //this need only for better development experience
    if false {
        let content_token_stream = columns_token_stream
            .into_iter()
            .map(|element| {
                element
                    .parse::<proc_macro2::TokenStream>()
                    .expect("79ee6381-c845-4762-a6f6-1c6b38806535")
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        macros_helpers::write_token_stream_into_file(
            "GeneratePostgresqlTypesExample",
            &quote::quote! {
                struct GeneratePostgresqlTypesExample {
                    #(#content_token_stream)*
                }
            },
            &macros_helpers::FormatWithRustfmt::True,
        );
    }
    let generated = {
        let content_token_stream = postgresql_type_array
            .into_iter()
            .map(|element| {
                element
                    .parse::<proc_macro2::TokenStream>()
                    .expect("e0c9257d-e554-4147-8174-b431c364c1ac")
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    };
    // macros_helpers::write_token_stream_into_file(
    //     "GeneratePostgresqlTypes",
    //     &generated,
    //     &macros_helpers::FormatWithRustfmt::True
    // );
    generated.into()
}
