use naming::{
    ArrayOfUcc, AsUcc, ColumnSc, ContainsNullByteUcc, CreateSc, DateNaiveSc, DateNaiveUcc, DateSc,
    DateUcc, DaysSc, EarlierDateNotSupportedUcc, EarliestSupportedDateSc, EndSc, EndUcc, EqualUcc,
    ErrorSc, ExcludedStartGreaterThanExcludedEndUcc, ExcludedStartGreaterThanIncludedEndUcc,
    ExcludedUcc, GenPostgresTypesModSc, HourSc, IncludedEndCannotBeMaxUcc,
    IncludedStartGreaterThanExcludedEndUcc, IncludedStartGreaterThanIncludedEndUcc, IncludedUcc,
    IncrementSc, InvalidHourOrMinuteOrSecondOrMicrosecondUcc, MicroSc, MicrosecondSc,
    MicrosecondsSc, MinSc, MinuteSc, MonthsSc, NanosecondPrecisionIsNotSupportedUcc, NanosecondSc,
    NewSc, OptionUpdateSc, OptionVecCreateSc, PostgresTypeUcc, QuerySc,
    ReadIntoTableTypeDeclarationSc, ReadOnlyIdsIntoReadSc, ReadOnlyIdsIntoTableTypeDeclarationSc,
    ReadOnlyIdsIntoUpdateSc, ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsSc,
    ReadOnlyIdsToTwoDimensionalVecReadInnerSc, ReadOnlyIdsUcc, ReadSc, ReadUcc, SecSc, SecondSc,
    SelfSc, SelfUcc, StartSc, StartUcc, StdFmtDisplayPlusQuoteToTokens, TableTypeDeclarationSc,
    TableTypeDeclarationUcc, TimeSc, TimeUcc, ToStdStringStringSc, TryNewForDeserializeSc,
    TryNewSc, UnboundedUcc, UpdateUcc, ValueSc, VecOfUcc,
    parameter::{
        SelfCreateUcc, SelfNotNullUcc, SelfOriginTryNewErrorNamedUcc,
        SelfOriginTryNewForDeserializeErrorNamedUcc, SelfOriginUcc, SelfReadInnerUcc,
        SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc, SelfTableTypeDeclarationUcc,
        SelfUpdateForQueryUcc, SelfUpdateUcc, SelfWhereUcc,
    },
};
use postgres_crud_macros_common::NotNullOrNullable;
use quote::quote;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::{
    fmt::{Display, Formatter, Result as StdFmtResult},
    iter::{once, repeat_n},
};
#[must_use]
pub fn gen_postgres_types(input_ts: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    #[allow(clippy::arbitrary_source_item_ordering)]
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
    impl From<&PostgresType> for RustTypeName {
        fn from(value: &PostgresType) -> Self {
            match &value {
                PostgresType::StdPrimitiveF32AsFloat4 => Self::StdPrimitiveF32,
                PostgresType::StdPrimitiveF64AsFloat8 => Self::StdPrimitiveF64,
                PostgresType::StdPrimitiveI16AsInt2 | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres => Self::StdPrimitiveI16,
                PostgresType::StdPrimitiveI32AsInt4 | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres => Self::StdPrimitiveI32,
                PostgresType::StdPrimitiveI64AsInt8 | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => Self::StdPrimitiveI64,
                PostgresType::SqlxPostgresTypesPgMoneyAsMoney => Self::SqlxPostgresTypesPgMoney,
                PostgresType::StdPrimitiveBoolAsBool => Self::StdPrimitiveBool,
                PostgresType::StdStringStringAsText => Self::StdStringString,
                PostgresType::StdVecVecStdPrimitiveU8AsBytea => Self::StdVecVecStdPrimitiveU8,
                PostgresType::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                PostgresType::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                PostgresType::SqlxPostgresTypesPgIntervalAsInterval => Self::SqlxPostgresTypesPgInterval,
                PostgresType::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::SqlxTypesUuidUuid,
                PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32,
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64,
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, strum_macros::Display)]
    enum PostgresTypeName {
        Int2,
        Int4,
        Int8,
        Float4,
        Float8,
        SmallSerialInitializedByPostgres,
        SerialInitializedByPostgres,
        BigSerialInitializedByPostgres,
        Money,
        Bool,
        Text,
        Bytea,
        Time,
        Interval,
        Date,
        Timestamp,
        TimestampTz,
        UuidV4InitializedByPostgres,
        UuidInitializedByClient,
        Inet,
        MacAddr,
        Int4Range,
        Int8Range,
        DateRange,
        TimestampRange,
        TimestampTzRange,
    }
    impl From<&PostgresType> for PostgresTypeName {
        fn from(value: &PostgresType) -> Self {
            match &value {
                PostgresType::StdPrimitiveI16AsInt2 => Self::Int2,
                PostgresType::StdPrimitiveI32AsInt4 => Self::Int4,
                PostgresType::StdPrimitiveI64AsInt8 => Self::Int8,
                PostgresType::StdPrimitiveF32AsFloat4 => Self::Float4,
                PostgresType::StdPrimitiveF64AsFloat8 => Self::Float8,
                PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres => Self::SmallSerialInitializedByPostgres,
                PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres => Self::SerialInitializedByPostgres,
                PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => Self::BigSerialInitializedByPostgres,
                PostgresType::SqlxPostgresTypesPgMoneyAsMoney => Self::Money,
                PostgresType::StdPrimitiveBoolAsBool => Self::Bool,
                PostgresType::StdStringStringAsText => Self::Text,
                PostgresType::StdVecVecStdPrimitiveU8AsBytea => Self::Bytea,
                PostgresType::SqlxTypesChronoNaiveTimeAsTime | PostgresType::SqlxTypesTimeTimeAsTime => Self::Time,
                PostgresType::SqlxPostgresTypesPgIntervalAsInterval => Self::Interval,
                PostgresType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => Self::UuidV4InitializedByPostgres,
                PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => Self::UuidInitializedByClient,
                PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Inet,
                PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::MacAddr,
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::Int4Range,
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::Int8Range,
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::DateRange,
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::TimestampRange,
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::TimestampTzRange,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
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
    enum PostgresType {
        StdPrimitiveI16AsInt2,
        StdPrimitiveI32AsInt4,
        StdPrimitiveI64AsInt8,
        StdPrimitiveF32AsFloat4,
        StdPrimitiveF64AsFloat8,
        StdPrimitiveI16AsSmallSerialInitializedByPostgres,
        StdPrimitiveI32AsSerialInitializedByPostgres,
        StdPrimitiveI64AsBigSerialInitializedByPostgres,
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
        SqlxTypesUuidUuidAsUuidV4InitializedByPostgres,
        SqlxTypesUuidUuidAsUuidInitializedByClient,
        SqlxTypesIpnetworkIpNetworkAsInet,
        SqlxTypesMacAddressMacAddressAsMacAddr,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    fn wrap_into_sqlx_postgres_types_pg_range_str(value: &dyn Display) -> String {
        format!("sqlx::postgres::types::PgRange<{value}>")
    }
    enum CanBeNullable {
        False,
        True,
    }
    enum CanBeAnArrayElement {
        False,
        True,
    }
    impl PostgresType {
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
                Self::StdPrimitiveI16AsSmallSerialInitializedByPostgres | Self::StdPrimitiveI32AsSerialInitializedByPostgres | Self::StdPrimitiveI64AsBigSerialInitializedByPostgres | Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => CanBeAnArrayElement::False,
            }
        }
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
                Self::StdPrimitiveI16AsSmallSerialInitializedByPostgres | Self::StdPrimitiveI32AsSerialInitializedByPostgres | Self::StdPrimitiveI64AsBigSerialInitializedByPostgres | Self::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => CanBeNullable::False,
            }
        }
    }
    impl quote::ToTokens for PostgresType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("cfefbb95-b0f4-44de-ba94-3e77e88daf0f")
                .to_tokens(tokens);
        }
    }
    impl From<&PostgresTypeRange> for PostgresType {
        fn from(value: &PostgresTypeRange) -> Self {
            match value {
                PostgresTypeRange::StdPrimitiveI32AsInt4 => Self::StdPrimitiveI32AsInt4,
                PostgresTypeRange::StdPrimitiveI64AsInt8 => Self::StdPrimitiveI64AsInt8,
                PostgresTypeRange::SqlxTypesChronoNaiveDateAsDate => {
                    Self::SqlxTypesChronoNaiveDateAsDate
                }
                PostgresTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                    Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                }
                PostgresTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                    Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum PostgresTypeRange {
        StdPrimitiveI32AsInt4,
        StdPrimitiveI64AsInt8,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
    }
    impl TryFrom<&PostgresType> for PostgresTypeRange {
        type Error = ();
        fn try_from(value: &PostgresType) -> Result<Self, Self::Error> {
            match &value {
                PostgresType::StdPrimitiveI16AsInt2
                | PostgresType::StdPrimitiveI32AsInt4
                | PostgresType::StdPrimitiveI64AsInt8
                | PostgresType::StdPrimitiveF32AsFloat4
                | PostgresType::StdPrimitiveF64AsFloat8
                | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                | PostgresType::StdPrimitiveBoolAsBool
                | PostgresType::StdStringStringAsText
                | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                | PostgresType::SqlxTypesTimeTimeAsTime
                | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                | PostgresType::SqlxTypesChronoNaiveDateAsDate
                | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres
                | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::StdPrimitiveI32AsInt4),
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::StdPrimitiveI64AsInt8),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
            }
        }
    }
    impl Display for PostgresTypeRange {
        fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
            write!(
                f,
                "{}",
                SelfNotNullUcc::from_display(&PostgresType::from(self))
            )
        }
    }
    impl quote::ToTokens for PostgresTypeRange {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("798ccb5a-e65b-4ae9-88cd-48c8e22d79d0")
                .to_tokens(tokens);
        }
    }
    // todo reuse it(move to postgres_macros_common) if sqlx devs will add nested array support
    #[allow(clippy::arbitrary_source_item_ordering)]
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
    enum PostgresTypePattern {
        Standart,
        ArrayDimension1 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
        },
        // sqlx does not support nested arrays yet. https://github.com/launchbadge/sqlx/issues/2280
        // ArrayDimension2 {
        //     dimension1_not_null_or_nullable: NotNullOrNullable,
        //     dimension2_not_null_or_nullable: NotNullOrNullable,
        // },
        // ArrayDimension3 {
        //     dimension1_not_null_or_nullable: NotNullOrNullable,
        //     dimension2_not_null_or_nullable: NotNullOrNullable,
        //     dimension3_not_null_or_nullable: NotNullOrNullable,
        // },
        // ArrayDimension4 {
        //     dimension1_not_null_or_nullable: NotNullOrNullable,
        //     dimension2_not_null_or_nullable: NotNullOrNullable,
        //     dimension3_not_null_or_nullable: NotNullOrNullable,
        //     dimension4_not_null_or_nullable: NotNullOrNullable,
        // },
    }
    impl PostgresTypePattern {
        const fn array_dimensions_number(&self) -> usize {
            match &self {
                Self::Standart => 0,
                Self::ArrayDimension1 { .. } => 1,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, serde::Serialize)]
    struct PostgresTypeRecord {
        postgres_type: PostgresType,
        not_null_or_nullable: NotNullOrNullable,
        postgres_type_pattern: PostgresTypePattern,
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        #[allow(
            unused_extern_crates,
            clippy::useless_attribute,
            clippy::arbitrary_source_item_ordering
        )]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PostgresTypeRecord {
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
                        __f: &mut Formatter<'_>,
                    ) -> _serde::__private228::fmt::Result {
                        Formatter::write_str(__f, "field identifier")
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
                            "postgres_type" => Ok(__Field::__field0),
                            "not_null_or_nullable" => Ok(__Field::__field1),
                            "postgres_type_pattern" => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"postgres_type" => Ok(__Field::__field0),
                            b"not_null_or_nullable" => Ok(__Field::__field1),
                            b"postgres_type_pattern" => Ok(__Field::__field2),
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
                    marker: _serde::__private228::PhantomData<PostgresTypeRecord>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PostgresTypeRecord;
                    fn expecting(
                        &self,
                        __f: &mut Formatter<'_>,
                    ) -> _serde::__private228::fmt::Result {
                        Formatter::write_str(__f, "struct PostgresTypeRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let Some(__field0) =
                            _serde::de::SeqAccess::next_element::<PostgresType>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct PostgresTypeRecord with 3 elements",
                            ));
                        };
                        let Some(__field1) =
                            _serde::de::SeqAccess::next_element::<NotNullOrNullable>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct PostgresTypeRecord with 3 elements",
                            ));
                        };
                        let Some(__field2) =
                            _serde::de::SeqAccess::next_element::<PostgresTypePattern>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct PostgresTypeRecord with 3 elements",
                            ));
                        };
                        match PostgresTypeRecord::try_from((__field0, __field1, __field2)) {
                            Ok(value) => Ok(value),
                            Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                        }
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: Option<PostgresType> = None;
                        let mut __field1: Option<NotNullOrNullable> = None;
                        let mut __field2: Option<PostgresTypePattern> = None;
                        while let Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if Option::is_some(&__field0) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "postgres_type",
                                            ),
                                        );
                                    }
                                    __field0 =
                                        Some(_serde::de::MapAccess::next_value::<PostgresType>(
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
                                    __field1 = Some(_serde::de::MapAccess::next_value::<
                                        NotNullOrNullable,
                                    >(
                                        &mut __map
                                    )?);
                                }
                                __Field::__field2 => {
                                    if Option::is_some(&__field2) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "postgres_type_pattern",
                                            ),
                                        );
                                    }
                                    __field2 =
                                        Some(_serde::de::MapAccess::next_value::<
                                            PostgresTypePattern,
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
                            None => _serde::__private228::de::missing_field("postgres_type")?,
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
                                _serde::__private228::de::missing_field("postgres_type_pattern")?
                            }
                        };
                        match PostgresTypeRecord::try_from((
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
                    "postgres_type",
                    "not_null_or_nullable",
                    "postgres_type_pattern",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PostgresTypeRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Self>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl TryFrom<(PostgresType, NotNullOrNullable, PostgresTypePattern)> for PostgresTypeRecord {
        type Error = String;
        fn try_from(
            value: (PostgresType, NotNullOrNullable, PostgresTypePattern),
        ) -> Result<Self, Self::Error> {
            let cant_support_nullable_variants_message = "cant support nullable variants: ";
            let cant_support_array_version_message = "cant support array_version: ";
            match &value.0.can_be_nullable() {
                CanBeNullable::False => {
                    if matches!(&value.1, NotNullOrNullable::Nullable) {
                        return Err(format!(
                            "{cant_support_nullable_variants_message}{value:#?}"
                        ));
                    }
                    match &value.2 {
                        PostgresTypePattern::Standart => Ok(Self {
                            postgres_type: value.0,
                            not_null_or_nullable: value.1,
                            postgres_type_pattern: value.2,
                        }),
                        PostgresTypePattern::ArrayDimension1 {
                            dimension1_not_null_or_nullable,
                        } => match &value.0.can_be_an_array_element() {
                            CanBeAnArrayElement::False => {
                                Err(format!("{cant_support_array_version_message}{value:#?}"))
                            }
                            CanBeAnArrayElement::True => match &dimension1_not_null_or_nullable {
                                NotNullOrNullable::NotNull => Ok(Self {
                                    postgres_type: value.0,
                                    not_null_or_nullable: value.1,
                                    postgres_type_pattern: value.2,
                                }),
                                NotNullOrNullable::Nullable => Err(format!(
                                    "{cant_support_nullable_variants_message}{value:#?}"
                                )),
                            },
                        },
                    }
                }
                CanBeNullable::True => match &value.2 {
                    PostgresTypePattern::Standart => Ok(Self {
                        postgres_type: value.0,
                        not_null_or_nullable: value.1,
                        postgres_type_pattern: value.2,
                    }),
                    PostgresTypePattern::ArrayDimension1 { .. } => {
                        match &value.0.can_be_an_array_element() {
                            CanBeAnArrayElement::False => {
                                Err(format!("{cant_support_array_version_message}{value:#?}"))
                            }
                            CanBeAnArrayElement::True => Ok(Self {
                                postgres_type: value.0,
                                not_null_or_nullable: value.1,
                                postgres_type_pattern: value.2,
                            }),
                        }
                    }
                },
            }
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GenPostgresTypesConfigVariant {
        All,
        Concrete(Vec<PostgresTypeRecord>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize)]
    struct GenPostgresJsonTypesConfig {
        postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_postgres_types: macros_helpers::ShouldWriteTokenStreamIntoFile,
        variant: GenPostgresTypesConfigVariant,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    enum PostgresTypeInitializationTryNew {
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
    impl TryFrom<&PostgresType> for PostgresTypeInitializationTryNew {
        type Error = ();
        fn try_from(value: &PostgresType) -> Result<Self, Self::Error> {
            match value {
                PostgresType::StdPrimitiveI16AsInt2
                | PostgresType::StdPrimitiveI32AsInt4
                | PostgresType::StdPrimitiveI64AsInt8
                | PostgresType::StdPrimitiveF32AsFloat4
                | PostgresType::StdPrimitiveF64AsFloat8
                | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                | PostgresType::StdPrimitiveBoolAsBool
                | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres
                | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PostgresType::StdStringStringAsText => Ok(Self::StdStringStringAsText),
                PostgresType::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                PostgresType::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                PostgresType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
        }
    }
    impl From<&PostgresTypeInitializationTryNew> for PostgresType {
        fn from(value: &PostgresTypeInitializationTryNew) -> Self {
            match value {
                PostgresTypeInitializationTryNew::StdStringStringAsText => Self::StdStringStringAsText,
                PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTimeAsTime,
                PostgresTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTimeAsTime,
                PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PostgresTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    enum PostgresTypeImplNewForDeserialize {
        SsqlxPostgresTypesPgIntervalAsInterval, //Ssqlx instead of Sqlx - just to fix clippy lint
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    enum PostgresTypeImplTryNewForDeserialize {
        StdStringStringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
    }
    #[derive(Debug)]
    enum PostgresTypeImplNewForDeserializeOrTryNewForDeserialize {
        NewForDeserialize(PostgresTypeImplNewForDeserialize),
        TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize),
    }
    #[derive(Debug)]
    enum PostgresTypeDeserialize {
        Derive,
        ImplNewForDeserializeOrTryNewForDeserialize(
            PostgresTypeImplNewForDeserializeOrTryNewForDeserialize,
        ),
    }
    impl From<&PostgresType> for PostgresTypeDeserialize {
        fn from(value: &PostgresType) -> Self {
            match value {
                PostgresType::StdPrimitiveI16AsInt2
                | PostgresType::StdPrimitiveI32AsInt4
                | PostgresType::StdPrimitiveI64AsInt8
                | PostgresType::StdPrimitiveF32AsFloat4
                | PostgresType::StdPrimitiveF64AsFloat8
                | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                | PostgresType::StdPrimitiveBoolAsBool
                | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres
                | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::Derive,
                PostgresType::StdStringStringAsText => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize::StdStringStringAsText)),
                PostgresType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime)),
                PostgresType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime)),
                PostgresType::SqlxPostgresTypesPgIntervalAsInterval => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresTypeImplNewForDeserialize::SsqlxPostgresTypesPgIntervalAsInterval)),
                PostgresType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate)),
                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp)),
                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz)),
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range)),
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range)),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange)),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange)),
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange)),
            }
        }
    }
    panic_location::panic_location();
    let gen_postgres_json_types_config =
        serde_json::from_str::<GenPostgresJsonTypesConfig>(&input_ts.to_string())
            .expect("80485f71-4e21-4166-94df-722326c36a29");
    let (columns_ts, postgres_type_array) = {
        let acc_5464fefe = match gen_postgres_json_types_config.variant {
            GenPostgresTypesConfigVariant::All => PostgresType::into_array().into_iter().fold(Vec::new(), |mut acc_4351207e, el_a897c529| {
                for el_a7126978 in PostgresTypePattern::into_array().into_iter().fold(Vec::new(), |mut acc_f806f6d2, el_8ae86bf2| {
                    match &el_8ae86bf2 {
                        PostgresTypePattern::Standart => {
                            acc_f806f6d2.push(el_8ae86bf2);
                        }
                        PostgresTypePattern::ArrayDimension1 { .. } => {
                            for el_6577bebd in NotNullOrNullable::into_array() {
                                acc_f806f6d2.push(PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: el_6577bebd });
                            }
                        }
                    }
                    acc_f806f6d2
                }) {
                    match &el_a7126978 {
                        PostgresTypePattern::Standart => match &el_a897c529.can_be_nullable() {
                            CanBeNullable::False => {
                                acc_4351207e.push(PostgresTypeRecord {
                                    postgres_type: el_a897c529.clone(),
                                    not_null_or_nullable: NotNullOrNullable::NotNull,
                                    postgres_type_pattern: el_a7126978,
                                });
                            },
                            CanBeNullable::True => NotNullOrNullable::into_array().into_iter().for_each(|el_a8753f2d| {
                                acc_4351207e.push(PostgresTypeRecord {
                                    postgres_type: el_a897c529.clone(),
                                    not_null_or_nullable: el_a8753f2d,
                                    postgres_type_pattern: el_a7126978.clone(),
                                });
                            }),
                        },
                        PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &el_a897c529.can_be_an_array_element() {
                            CanBeAnArrayElement::False => (),
                            CanBeAnArrayElement::True => match &el_a897c529.can_be_nullable() {
                                CanBeNullable::False => {
                                    if matches!(&dimension1_not_null_or_nullable, NotNullOrNullable::NotNull) {
                                        for el_8b51bcb4 in NotNullOrNullable::into_array() {
                                            acc_4351207e.push(PostgresTypeRecord {
                                                postgres_type: el_a897c529.clone(),
                                                not_null_or_nullable: el_8b51bcb4,
                                                postgres_type_pattern: PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable },
                                            });
                                        }
                                    }
                                },
                                CanBeNullable::True => NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                                    acc_4351207e.push(PostgresTypeRecord {
                                        postgres_type: el_a897c529.clone(),
                                        not_null_or_nullable,
                                        postgres_type_pattern: el_a7126978.clone(),
                                    });
                                }),
                            },
                        },
                    }
                }
                acc_4351207e
            }),
            GenPostgresTypesConfigVariant::Concrete(value) => value,
        };
        {
            let mut check_acc = Vec::new();
            for el_03c535a8 in &acc_5464fefe {
                if check_acc.contains(&el_03c535a8) {
                    panic!("536036f9-2511-4247-8463-6defbeb72f5c");
                } else {
                    check_acc.push(el_03c535a8);
                }
            }
        }
        acc_5464fefe
    }.into_iter()
    .fold(
        Vec::new(),
        |mut acc_0562629e, el_758fe97f| {
            #[derive(Clone)]
            struct PostgresTypeRecordHandle {
                not_null_or_nullable: NotNullOrNullable,
                postgres_type_pattern: PostgresTypePattern,
            }
            fn gen_postgres_type_record_handle_vec(
                postgres_type_record_handle: PostgresTypeRecordHandle,
            ) -> Vec<PostgresTypeRecordHandle> {
                let gen_vec = |
                    current_postgres_type_record_handle: PostgresTypeRecordHandle
                | gen_postgres_type_record_handle_vec(current_postgres_type_record_handle)
                .into_iter()
                .chain(once(postgres_type_record_handle.clone()))
                .collect();
                //same pattern was in gen_postgres_types 21.05.2025
                match (
                    &postgres_type_record_handle.not_null_or_nullable,
                    &postgres_type_record_handle.postgres_type_pattern,
                ) {
                    (NotNullOrNullable::NotNull, PostgresTypePattern::Standart) => {
                        vec![postgres_type_record_handle]
                    }
                    (NotNullOrNullable::Nullable, PostgresTypePattern::Standart) => {
                        gen_vec(PostgresTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgres_type_pattern: PostgresTypePattern::Standart,
                        })
                    }
                    (
                        NotNullOrNullable::NotNull,
                        PostgresTypePattern::ArrayDimension1 {
                            dimension1_not_null_or_nullable,
                        },
                    ) => gen_vec(PostgresTypeRecordHandle {
                        not_null_or_nullable: *dimension1_not_null_or_nullable,
                        postgres_type_pattern: PostgresTypePattern::Standart,
                    }),
                    (
                        NotNullOrNullable::Nullable,
                        PostgresTypePattern::ArrayDimension1 { .. },
                    ) => gen_vec(PostgresTypeRecordHandle {
                        not_null_or_nullable: NotNullOrNullable::NotNull,
                        postgres_type_pattern: postgres_type_record_handle
                            .postgres_type_pattern
                            .clone(),
                    }),
                }
            }
            for el_39ea25de in gen_postgres_type_record_handle_vec(PostgresTypeRecordHandle {
                not_null_or_nullable: el_758fe97f.not_null_or_nullable,
                postgres_type_pattern: el_758fe97f.postgres_type_pattern,
            }) {
                let value_88571cb8 = PostgresTypeRecord {
                    postgres_type: el_758fe97f.postgres_type.clone(),
                    not_null_or_nullable: el_39ea25de
                        .not_null_or_nullable,
                    postgres_type_pattern: el_39ea25de
                        .postgres_type_pattern,
                };
                if !acc_0562629e.contains(&value_88571cb8) {
                    acc_0562629e.push(value_88571cb8);
                }
            }
            acc_0562629e
        },
    )
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PostgresTypeRecord)>>()
    .par_iter()
    //.into_iter() //just for console prints ordering
    .map(|(index, element)| {
        enum PostgresTypeOrPostgresTypeTestCases {
            PostgresType,
            PostgresTypeTestCases,
        }
        enum CanBePrimaryKey {
            False,
            True,
        }
        enum IsNotNullStandartCanBePrimaryKey {
            False,
            True,
        }
        enum StartOrEnd {
            End,
            Start,
        }
        enum IntRangeType {
            SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range,
            SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range,
        }
        type Handle<'lifetime> = (&'lifetime dyn quote::ToTokens, &'lifetime dyn quote::ToTokens);
        fn gen_pg_range_conversion_ts(match_content_ts: &dyn quote::ToTokens, input_ts: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote! {
                sqlx::postgres::types::PgRange {
                    start: match #match_content_ts.start {
                        std::ops::Bound::Included(value_af65ccce) => std::ops::Bound::Included(#input_ts),
                        std::ops::Bound::Excluded(value_af65ccce) => std::ops::Bound::Excluded(#input_ts),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match #match_content_ts.end {
                        std::ops::Bound::Included(value_af65ccce) => std::ops::Bound::Included(#input_ts),
                        std::ops::Bound::Excluded(value_af65ccce) => std::ops::Bound::Excluded(#input_ts),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }
            }
        }
        let postgres_type = &element.postgres_type;
        let not_null_or_nullable = &element.not_null_or_nullable;
        let postgres_type_pattern = &element.postgres_type_pattern;
        let postgres_type_initialization_try_new_try_from_postgres_type = PostgresTypeInitializationTryNew::try_from(postgres_type);
        let postgres_type_deserialize = PostgresTypeDeserialize::from(postgres_type);

        let array_dimensions_number = postgres_type_pattern.array_dimensions_number();

        let postgres_type_range_try_from_postgres_type = PostgresTypeRange::try_from(postgres_type);
        let postgres_type_range_try_from_postgres_type_is_ok = postgres_type_range_try_from_postgres_type.is_ok();

        let std_primitive_u8_ts = token_patterns::StdPrimitiveU8;
        let std_primitive_i16_ts = token_patterns::StdPrimitiveI16;
        let std_primitive_u32_ts = token_patterns::StdPrimitiveU32;
        let std_primitive_i32_ts = token_patterns::StdPrimitiveI32;
        let std_primitive_i64_ts = token_patterns::StdPrimitiveI64;
        let std_primitive_f32_ts = token_patterns::StdPrimitiveF32;
        let std_string_string_ts = token_patterns::StdStringString;

        let core_default_default_default_ts = token_patterns::CoreDefaultDefaultDefault;
        let postgres_crud_common_default_option_some_vec_one_el_call_ts = token_patterns::PostgresCrudCommonDefaultOptionSomeVecOneElCall;
        let postgres_crud_common_default_option_some_vec_one_el_max_page_size_call_ts = token_patterns::PostgresCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall;
        let must_use_ts = token_patterns::MustUse;
        let allow_clippy_arbitrary_source_item_ordering_ts = token_patterns::AllowClippyArbitrarySourceItemOrdering;

        let import_path = postgres_crud_macros_common::ImportPath::PostgresCrudCommon;
        let import_path_non_primary_key_postgres_type_read_only_ids_ts = quote! {#import_path::NonPrimaryKeyPostgresTypeReadOnlyIds};
        let none_ts = quote!{None};
        let dot_clone_ts = quote!{.clone()};
        let maybe_dot_clone_ts: &dyn quote::ToTokens = if matches!(&postgres_type_pattern, PostgresTypePattern::Standart) &&
            matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
        {
            match &postgres_type {
                PostgresType::StdPrimitiveI16AsInt2 |
                PostgresType::StdPrimitiveI32AsInt4 |
                PostgresType::StdPrimitiveI64AsInt8 |
                PostgresType::StdPrimitiveF32AsFloat4 |
                PostgresType::StdPrimitiveF64AsFloat8 |
                PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                PostgresType::StdPrimitiveBoolAsBool |
                PostgresType::SqlxTypesChronoNaiveTimeAsTime | PostgresType::SqlxTypesTimeTimeAsTime |
                PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                PostgresType::SqlxTypesChronoNaiveDateAsDate |
                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &proc_macro2::TokenStream::new(),
                PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                PostgresType::StdStringStringAsText => &dot_clone_ts,
            }
        }
        else {
            &dot_clone_ts
        };

        let gen_import_path_value_initialization_ts = |content_ts: &dyn quote::ToTokens| postgres_crud_macros_common::gen_value_initialization_ts(&import_path, &content_ts);

        let gen_ident_str = |
            current_postgres_type: &PostgresType,
            current_not_null_or_nullable: &NotNullOrNullable,
            current_postgres_type_pattern: &PostgresTypePattern
        | {
            let rust_type_name = RustTypeName::from(current_postgres_type);
            let postgres_type_name = PostgresTypeName::from(current_postgres_type);
            let not_null_or_nullable_rust = current_not_null_or_nullable.rust();
            let (rust_part, postgres_part) = match &current_postgres_type_pattern {
                PostgresTypePattern::Standart => (format!("{rust_type_name}"), format!("{postgres_type_name}")),
                PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    (format!("{VecOfUcc}{d1_rust}{rust_type_name}"), format!("{ArrayOfUcc}{d1}{postgres_type_name}"))
                }
            };
            format!("{not_null_or_nullable_rust}{rust_part}{AsUcc}{current_not_null_or_nullable}{postgres_part}")
        };
        let gen_ident_ts = |
            current_postgres_type: &PostgresType,
            current_not_null_or_nullable: &NotNullOrNullable,
            current_postgres_type_pattern: &PostgresTypePattern
        | gen_ident_str(
            current_postgres_type,
            current_not_null_or_nullable,
            current_postgres_type_pattern
        ).parse::<proc_macro2::TokenStream>().expect("ff3eb7a6-8369-46fd-82f5-2afdf752365f");
        let ident = &gen_ident_ts(postgres_type, not_null_or_nullable, postgres_type_pattern);
        let gen_ident_standart_not_null_ts = |current_postgres_type: &PostgresType| gen_ident_ts(current_postgres_type, &NotNullOrNullable::NotNull, &PostgresTypePattern::Standart);
        let ident_standart_not_null_ucc = gen_ident_standart_not_null_ts(postgres_type);
        let ident_standart_nullable_ucc = gen_ident_ts(postgres_type, &NotNullOrNullable::Nullable, &PostgresTypePattern::Standart);
        let ident_array_not_null_ucc = gen_ident_ts(
            postgres_type,
            &NotNullOrNullable::NotNull,
            &PostgresTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: NotNullOrNullable::NotNull,
            },
        );
        let ident_array_nullable_ucc = gen_ident_ts(
            postgres_type,
            &NotNullOrNullable::NotNull,
            &PostgresTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable: NotNullOrNullable::Nullable,
            },
        );
        let gen_ts = |content_ts: &dyn quote::ToTokens, postgres_type_or_postgres_type_test_cases: &PostgresTypeOrPostgresTypeTestCases| {
            let trait_ts = match &postgres_type_or_postgres_type_test_cases {
                PostgresTypeOrPostgresTypeTestCases::PostgresType => quote! {PostgresType},
                PostgresTypeOrPostgresTypeTestCases::PostgresTypeTestCases => quote! {PostgresTypeTestCases},
            };
            quote! {<#content_ts as #import_path::#trait_ts>}
        };
        let gen_as_postgres_type_ts = |content_ts: &dyn quote::ToTokens| gen_ts(&content_ts, &PostgresTypeOrPostgresTypeTestCases::PostgresType);
        let gen_as_postgres_type_test_cases_ts = |content_ts: &dyn quote::ToTokens| gen_ts(&content_ts, &PostgresTypeOrPostgresTypeTestCases::PostgresTypeTestCases);
        let self_as_postgres_type_ts = gen_as_postgres_type_ts(&SelfUcc);
        let ident_standart_not_null_as_postgres_type_ts = gen_as_postgres_type_ts(&ident_standart_not_null_ucc);
        let ident_standart_nullable_as_postgres_type_ts = gen_as_postgres_type_ts(&ident_standart_nullable_ucc);
        let self_postgres_type_as_postgres_type_ts = gen_as_postgres_type_ts(&quote! {Self::#PostgresTypeUcc});

        let ident_standart_not_null_as_postgres_type_test_cases_ts = gen_as_postgres_type_test_cases_ts(&ident_standart_not_null_ucc);
        let ident_standart_nullable_as_postgres_type_test_cases_ts = gen_as_postgres_type_test_cases_ts(&ident_standart_nullable_ucc);
        let ident_array_not_null_as_postgres_type_test_cases_ts = gen_as_postgres_type_test_cases_ts(&ident_array_not_null_ucc);
        let ident_array_nullable_as_postgres_type_test_cases_ts = gen_as_postgres_type_test_cases_ts(&ident_array_nullable_ucc);

        let gen_ident_standart_not_null_origin_ts = |current_postgres_type: &PostgresType| SelfOriginUcc::from_tokens(
            &gen_ident_standart_not_null_ts(current_postgres_type)
        );
        let ident_standart_not_null_origin_ucc = gen_ident_standart_not_null_origin_ts(postgres_type);
        let ident_origin_ucc = SelfOriginUcc::from_tokens(&ident);
        let ident_standart_nullable_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_standart_nullable_ucc);

        let sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc = gen_ident_standart_not_null_origin_ts(&PostgresType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc = gen_ident_standart_not_null_origin_ts(&PostgresType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc = gen_ident_standart_not_null_origin_ts(&PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc = gen_ident_standart_not_null_origin_ts(&PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);

        let gen_ident_standart_not_null_origin_try_new_error_named_ts = |current_postgres_type: &PostgresType| SelfOriginTryNewErrorNamedUcc::from_tokens(
            &gen_ident_standart_not_null_ts(current_postgres_type)
        );
        let sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_ucc = gen_ident_standart_not_null_origin_try_new_error_named_ts(&PostgresType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_ucc = gen_ident_standart_not_null_origin_try_new_error_named_ts(&PostgresType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_error_named_ucc = gen_ident_standart_not_null_origin_try_new_error_named_ts(&PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_error_named_ucc = gen_ident_standart_not_null_origin_try_new_error_named_ts(&PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let inner_type_standart_not_null_ts = {
            let value = {
                let std_primitive_i16_str = "i16".to_owned();
                let std_primitive_i32_str = "i32".to_owned();
                let std_primitive_i64_str = "i64".to_owned();
                let std_primitive_f32_str = "f32".to_owned();
                let std_primitive_f64_str = "f64".to_owned();
                let sqlx_postgres_types_pg_money_str = "sqlx::postgres::types::PgMoney".to_owned();
                let std_primitive_bool_str = "bool".to_owned();
                let std_string_string_str = "String".to_owned();
                let std_vec_vec_std_primitive_u8_str = "Vec<u8>".to_owned();
                let sqlx_types_chrono_naive_date_str = "sqlx::types::chrono::NaiveDate".to_owned();
                let sqlx_types_chrono_naive_time_str = "sqlx::types::chrono::NaiveTime".to_owned();
                let sqlx_types_time_time_str = "sqlx::types::time::Time".to_owned();
                let sqlx_postgres_types_pg_interval_str = "sqlx::postgres::types::PgInterval".to_owned();
                let sqlx_types_chrono_naive_date_time_str = "sqlx::types::chrono::NaiveDateTime".to_owned();
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_str = "sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>".to_owned();
                let uuid_uuid_str = "uuid::Uuid".to_owned();
                let sqlx_types_ipnetwork_ip_network_str = "sqlx::types::ipnetwork::IpNetwork".to_owned();
                let sqlx_types_mac_address_mac_address_str = "sqlx::types::mac_address::MacAddress".to_owned();
                match &postgres_type {
                    PostgresType::StdPrimitiveF32AsFloat4 => std_primitive_f32_str,
                    PostgresType::StdPrimitiveF64AsFloat8 => std_primitive_f64_str,
                    PostgresType::StdPrimitiveI16AsInt2 | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres => std_primitive_i16_str,
                    PostgresType::StdPrimitiveI32AsInt4 | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres => std_primitive_i32_str,
                    PostgresType::StdPrimitiveI64AsInt8 | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => std_primitive_i64_str,
                    PostgresType::SqlxPostgresTypesPgMoneyAsMoney => sqlx_postgres_types_pg_money_str,
                    PostgresType::StdPrimitiveBoolAsBool => std_primitive_bool_str,
                    PostgresType::StdStringStringAsText => std_string_string_str,
                    PostgresType::StdVecVecStdPrimitiveU8AsBytea => std_vec_vec_std_primitive_u8_str,
                    PostgresType::SqlxTypesChronoNaiveTimeAsTime => sqlx_types_chrono_naive_time_str,
                    PostgresType::SqlxTypesTimeTimeAsTime => sqlx_types_time_time_str,
                    PostgresType::SqlxPostgresTypesPgIntervalAsInterval => sqlx_postgres_types_pg_interval_str,
                    PostgresType::SqlxTypesChronoNaiveDateAsDate => sqlx_types_chrono_naive_date_str,
                    PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => sqlx_types_chrono_naive_date_time_str,
                    PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_str,
                    PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => uuid_uuid_str,
                    PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => sqlx_types_ipnetwork_ip_network_str,
                    PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => sqlx_types_mac_address_mac_address_str,
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => wrap_into_sqlx_postgres_types_pg_range_str(&std_primitive_i32_str),
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => wrap_into_sqlx_postgres_types_pg_range_str(&std_primitive_i64_str),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => wrap_into_sqlx_postgres_types_pg_range_str(&sqlx_types_chrono_naive_date_str),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => wrap_into_sqlx_postgres_types_pg_range_str(&sqlx_types_chrono_naive_date_time_str),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => wrap_into_sqlx_postgres_types_pg_range_str(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_str),
                }
            };
            value.parse::<proc_macro2::TokenStream>().expect("2555843f-283f-4bc8-8c93-48e6fe68ae6a")
        };
        let gen_current_ident_origin_non_wrapping = |current_postgres_type_pattern: &PostgresTypePattern, current_not_null_or_nullable: &NotNullOrNullable| SelfOriginUcc::from_tokens(&gen_ident_ts(postgres_type, current_not_null_or_nullable, current_postgres_type_pattern));
        let field_type_handle: &dyn quote::ToTokens = {
            let gen_current_ident_origin = |current_postgres_type_pattern: &PostgresTypePattern, current_not_null_or_nullable: &NotNullOrNullable| {
                let value = gen_current_ident_origin_non_wrapping(current_postgres_type_pattern, current_not_null_or_nullable);
                match &not_null_or_nullable {
                    NotNullOrNullable::NotNull => postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(&value),
                    NotNullOrNullable::Nullable => postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&value),
                }
            };
            match &postgres_type_pattern {
                PostgresTypePattern::Standart => match &not_null_or_nullable {
                    NotNullOrNullable::NotNull => &inner_type_standart_not_null_ts,
                    NotNullOrNullable::Nullable => &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&ident_standart_not_null_origin_ucc),
                },
                PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let (current_postgres_type_pattern, current_not_null_or_nullable): (&PostgresTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => (&PostgresTypePattern::Standart, dimension1_not_null_or_nullable),
                        NotNullOrNullable::Nullable => (postgres_type_pattern, &NotNullOrNullable::NotNull),
                    };
                    gen_current_ident_origin(current_postgres_type_pattern, current_not_null_or_nullable)
                },
            }
        };
        let gen_typical_query_bind_ts = |content_ts: &dyn quote::ToTokens| match &not_null_or_nullable {
            NotNullOrNullable::NotNull => quote! {
                if let Err(er) = #QuerySc.try_bind(#content_ts) {
                    return Err(er.to_string());
                }
                Ok(#QuerySc)
            },
            NotNullOrNullable::Nullable => quote! {
                if let Err(er) = #QuerySc.try_bind(#content_ts.0.0) {
                    return Err(er.to_string());
                }
                Ok(#QuerySc)
            },
        };
        let typical_query_bind_ts = gen_typical_query_bind_ts(&ValueSc);
        let ident_inner_type_ts = match &element.postgres_type_pattern {
            PostgresTypePattern::Standart => match &not_null_or_nullable {
                NotNullOrNullable::NotNull => &inner_type_standart_not_null_ts,
                NotNullOrNullable::Nullable => &postgres_crud_macros_common::gen_std_option_option_tokens_declaration_ts(&inner_type_standart_not_null_ts),
            },
            PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote! {#inner_type_standart_not_null_ts});
                not_null_or_nullable.maybe_option_wrap(postgres_crud_macros_common::gen_std_vec_vec_tokens_declaration_ts(&dimension1_type))
            },
        };
        let can_be_primary_key = match &postgres_type {
            PostgresType::StdPrimitiveI16AsInt2
            | PostgresType::StdPrimitiveI32AsInt4
            | PostgresType::StdPrimitiveI64AsInt8
            | PostgresType::StdPrimitiveF32AsFloat4
            | PostgresType::StdPrimitiveF64AsFloat8
            | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
            | PostgresType::StdPrimitiveBoolAsBool
            | PostgresType::StdStringStringAsText
            | PostgresType::StdVecVecStdPrimitiveU8AsBytea
            | PostgresType::SqlxTypesChronoNaiveTimeAsTime
            | PostgresType::SqlxTypesTimeTimeAsTime
            | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
            | PostgresType::SqlxTypesChronoNaiveDateAsDate
            | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
            | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
            | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
            | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr
            | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
            | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
            | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
            | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
            | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePrimaryKey::False,
            PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => CanBePrimaryKey::True,
        };
        let is_standart_not_null = if matches!((&postgres_type_pattern, &not_null_or_nullable), (PostgresTypePattern::Standart, NotNullOrNullable::NotNull)) {
            postgres_crud_macros_common::IsStandartNotNull::True
        } else {
            postgres_crud_macros_common::IsStandartNotNull::False
        };
        let is_not_null_standart_can_be_primary_key = if matches!((&not_null_or_nullable, &postgres_type_pattern, &can_be_primary_key), (NotNullOrNullable::NotNull, PostgresTypePattern::Standart, CanBePrimaryKey::True)) {
            IsNotNullStandartCanBePrimaryKey::True
        } else {
            IsNotNullStandartCanBePrimaryKey::False
        };
        let gen_start_or_end_ucc = |start_or_end: &StartOrEnd| -> &dyn StdFmtDisplayPlusQuoteToTokens {
            match &start_or_end {
                StartOrEnd::End => &EndUcc,
                StartOrEnd::Start => &StartUcc,
            }
        };
        let gen_start_or_end_sc = |start_or_end: &StartOrEnd| -> &dyn StdFmtDisplayPlusQuoteToTokens {
            match &start_or_end {
                StartOrEnd::End => &EndSc,
                StartOrEnd::Start => &StartSc,
            }
        };
        let (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl) = if matches!(&is_standart_not_null, postgres_crud_macros_common::IsStandartNotNull::True) {
            #[allow(clippy::arbitrary_source_item_ordering)]
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
            let self_dot_zero_ts = quote! {#SelfSc.0};
            let parameter_number_one = ParameterNumber::One;
            let parameter_number_two = ParameterNumber::Two;
            let parameter_number_three = ParameterNumber::Three;
            let parameter_number_four = ParameterNumber::Four;
            let ident_standart_not_null_double_quotes_ts = gen_quotes::double_quotes_ts(&ident_standart_not_null_ucc);
            let ident_standart_not_null_origin_double_quotes_ts = gen_quotes::double_quotes_ts(&ident_standart_not_null_origin_ucc);
            let gen_std_ops_bound_ts = |type_ts: &dyn quote::ToTokens| {
                quote! {std::ops::Bound<#type_ts>}
            };
            let std_ops_bound_std_primitive_i32_ts = gen_std_ops_bound_ts(&std_primitive_i32_ts);
            let std_ops_bound_std_primitive_i64_ts = gen_std_ops_bound_ts(&std_primitive_i64_ts);
            let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts = gen_std_ops_bound_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc);
            let std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts = gen_std_ops_bound_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc);
            let std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts = gen_std_ops_bound_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc);
            let serde_serialize_derive_or_impl = {
                let gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens = |content_ts: &dyn quote::ToTokens| {
                    quote! {
                        #[allow(unused_qualifications)]
                        #[allow(clippy::absolute_paths)]
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        const _: () = {
                            #[allow(unused_extern_crates, clippy::useless_attribute)]
                            extern crate serde as _serde;
                            #[automatically_derived]
                            impl _serde::Serialize for #ident_standart_not_null_origin_ucc {
                                fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    #content_ts
                                }
                            }
                        };
                    }
                };
                let gen_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4 = |value_ts: &dyn quote::ToTokens| {
                    quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #ident_standart_not_null_origin_double_quotes_ts, &#self_dot_zero_ts #value_ts)}
                };
                let gen_serde_state_initialization_ts = |parameter_number: &ParameterNumber| {
                    let parameter_number_ts = {
                        let value = parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(|_| quote! {+ 1});
                        quote! {#(#value)*}
                    };
                    quote! {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_standart_not_null_origin_double_quotes_ts, usize::from(false) #parameter_number_ts)?;
                    }
                };
                let serde_state_initialization_two_fields_ts = gen_serde_state_initialization_ts(&parameter_number_two);
                let serde_state_initialization_three_fields_ts = gen_serde_state_initialization_ts(&parameter_number_three);
                let serde_state_initialization_four_fields_ts = gen_serde_state_initialization_ts(&parameter_number_four);
                let gen_serialize_field_ts = |field_name: &dyn Display, third_parameter_ts: &dyn quote::ToTokens| {
                    let field_name_double_quotes_ts = gen_quotes::double_quotes_ts(&field_name);
                    quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_double_quotes_ts, #third_parameter_ts)?;}
                };
                let serde_ser_serialize_struct_end_ts = quote! {_serde::ser::SerializeStruct::end(__serde_state)};
                let serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_ts = {
                    let gen_self_zero_tokens_ts = |value_ts: &dyn quote::ToTokens| {
                        quote! {&#self_dot_zero_ts.#value_ts}
                    };
                    let start_serialize_field_ts = gen_serialize_field_ts(&StartSc, &gen_self_zero_tokens_ts(&StartSc));
                    let end_serialize_field_ts = gen_serialize_field_ts(&EndSc, &gen_self_zero_tokens_ts(&EndSc));
                    quote! {
                        #serde_state_initialization_two_fields_ts
                        #start_serialize_field_ts
                        #end_serialize_field_ts
                        #serde_ser_serialize_struct_end_ts
                    }
                };
                let impl_serde_serialize_for_postgres_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_ts = gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_ts);
                let impl_serde_serialize_for_uuid_uuid_ts = gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&gen_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote! {.to_string()}));
                let gen_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens = |current_ident_ts: &dyn quote::ToTokens| {
                    let gen_serialize_field_match_std_ops_bound_ts = |start_or_end: &StartOrEnd| {
                        let start_or_end_ts = gen_start_or_end_sc(start_or_end);
                        gen_serialize_field_ts(
                            &start_or_end_ts,
                            &quote! {
                                &match self.0.#start_or_end_ts {
                                    std::ops::Bound::Included(#ValueSc) => std::ops::Bound::Included(#current_ident_ts::#TryNewSc(#ValueSc).map_err(_serde::ser::Error::custom)?),
                                    std::ops::Bound::Excluded(#ValueSc) => std::ops::Bound::Excluded(#current_ident_ts::#TryNewSc(#ValueSc).map_err(_serde::ser::Error::custom)?),
                                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                }
                            },
                        )
                    };
                    let start_serialize_field_ts = gen_serialize_field_match_std_ops_bound_ts(&StartOrEnd::Start);
                    let end_serialize_field_ts = gen_serialize_field_match_std_ops_bound_ts(&StartOrEnd::End);
                    gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&quote! {
                        #serde_state_initialization_two_fields_ts
                        #start_serialize_field_ts
                        #end_serialize_field_ts
                        #serde_ser_serialize_struct_end_ts
                    })
                };
                match &postgres_type {
                    PostgresType::StdPrimitiveI16AsInt2
                    | PostgresType::StdPrimitiveI32AsInt4
                    | PostgresType::StdPrimitiveI64AsInt8
                    | PostgresType::StdPrimitiveF32AsFloat4
                    | PostgresType::StdPrimitiveF64AsFloat8
                    | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                    | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                    | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                    | PostgresType::StdPrimitiveBoolAsBool
                    | PostgresType::StdStringStringAsText
                    | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                    | PostgresType::SqlxTypesChronoNaiveDateAsDate
                    | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => postgres_crud_macros_common::DeriveOrImpl::Derive,
                    PostgresType::SqlxPostgresTypesPgMoneyAsMoney => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&gen_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote! {.0}))),
                    PostgresType::SqlxTypesChronoNaiveTimeAsTime => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                        let gen_field_inner_type_standart_not_null_ts_as_chrono_timelike_ts = |content_ts: &dyn quote::ToTokens| {
                            quote! {&(<#inner_type_standart_not_null_ts as chrono::Timelike>::#content_ts)}
                        };
                        let hour_serialize_field_ts = gen_serialize_field_ts(&HourSc, &gen_field_inner_type_standart_not_null_ts_as_chrono_timelike_ts(&quote! {hour(&self.0)}));
                        let min_serialize_field_ts = gen_serialize_field_ts(&MinSc, &gen_field_inner_type_standart_not_null_ts_as_chrono_timelike_ts(&quote! {minute(&self.0)}));
                        let sec_serialize_field_ts = gen_serialize_field_ts(&SecSc, &gen_field_inner_type_standart_not_null_ts_as_chrono_timelike_ts(&quote! {second(&self.0)}));
                        let micro_serialize_field_ts = gen_serialize_field_ts(
                            &MicroSc,
                            &gen_field_inner_type_standart_not_null_ts_as_chrono_timelike_ts(&quote! {
                                #NanosecondSc(&self.0).checked_div(1000).expect("aea037b7-95ef-4616-b018-6f2ed1651928")
                            }),
                        );
                        quote! {
                            #serde_state_initialization_four_fields_ts
                            #hour_serialize_field_ts
                            #min_serialize_field_ts
                            #sec_serialize_field_ts
                            #micro_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PostgresType::SqlxTypesTimeTimeAsTime => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                        let gen_serialize_field_self_zero_ts = |value: &dyn StdFmtDisplayPlusQuoteToTokens| gen_serialize_field_ts(&value, &quote! {&self.0.#value()});
                        let hour_serialize_field_ts = gen_serialize_field_self_zero_ts(&HourSc);
                        let minute_serialize_field_ts = gen_serialize_field_self_zero_ts(&MinuteSc);
                        let second_serialize_field_ts = gen_serialize_field_self_zero_ts(&SecondSc);
                        let microsecond_serialize_field_ts = gen_serialize_field_self_zero_ts(&MicrosecondSc);
                        quote! {
                            #serde_state_initialization_four_fields_ts
                            #hour_serialize_field_ts
                            #minute_serialize_field_ts
                            #second_serialize_field_ts
                            #microsecond_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PostgresType::SqlxPostgresTypesPgIntervalAsInterval => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                        let gen_serialize_field_handle_ts = |value: &dyn StdFmtDisplayPlusQuoteToTokens| gen_serialize_field_ts(&value, &quote! {&#self_dot_zero_ts.#value});
                        let months_serialize_field_ts = gen_serialize_field_handle_ts(&MonthsSc);
                        let days_serialize_field_ts = gen_serialize_field_handle_ts(&DaysSc);
                        let microseconds_serialize_field_ts = gen_serialize_field_handle_ts(&MicrosecondsSc);
                        quote! {
                            #serde_state_initialization_three_fields_ts
                            #months_serialize_field_ts
                            #days_serialize_field_ts
                            #microseconds_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                        enum DateOrTime {
                            Date,
                            Time,
                        }
                        let gen_serialize_field_try_new_unwrap_ts = |date_or_time: &DateOrTime| {
                            let date_or_time_ts: &dyn StdFmtDisplayPlusQuoteToTokens = match &date_or_time {
                                DateOrTime::Date => &DateSc,
                                DateOrTime::Time => &TimeSc,
                            };
                            gen_serialize_field_ts(&date_or_time_ts, &{
                                let current_ident_ts: &dyn quote::ToTokens = match &date_or_time {
                                    DateOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                    DateOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc,
                                };
                                quote! {
                                    &match #current_ident_ts::#TryNewSc(self.0.#date_or_time_ts()) {
                                        Ok(value_b2ac0c33) => value_b2ac0c33,
                                        Err(error_2c555724) => {
                                            return Err(_serde::ser::Error::custom(error_2c555724));
                                        },
                                    }
                                }
                            })
                        };
                        let date_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateOrTime::Date);
                        let time_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateOrTime::Time);
                        quote! {
                            #serde_state_initialization_two_fields_ts
                            #date_serialize_field_ts
                            #time_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&{
                        enum DateNaiveOrTime {
                            Date,
                            Time,
                        }
                        let gen_serialize_field_try_new_unwrap_ts = |date_naive_or_time: &DateNaiveOrTime| {
                            let date_naive_or_time_ts: &dyn StdFmtDisplayPlusQuoteToTokens = match &date_naive_or_time {
                                DateNaiveOrTime::Date => &DateNaiveSc,
                                DateNaiveOrTime::Time => &TimeSc,
                            };
                            gen_serialize_field_ts(&date_naive_or_time_ts, &{
                                let current_ident_ts: &dyn quote::ToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                    DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc,
                                };
                                quote! {&#current_ident_ts::#TryNewSc(self.0.#date_naive_or_time_ts()).map_err(_serde::ser::Error::custom)?}
                            })
                        };
                        let date_naive_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateNaiveOrTime::Date);
                        let time_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateNaiveOrTime::Time);
                        quote! {
                            #serde_state_initialization_two_fields_ts
                            #date_naive_serialize_field_ts
                            #time_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgres_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_uuid_uuid_ts),
                    PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_tokens(&gen_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote! {.bytes()}))),
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgres_crud_macros_common::DeriveOrImpl::Impl(impl_serde_serialize_for_postgres_type_not_null_tokens_serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_ts),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc)),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc)),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_standart_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc)),
                }
            };
            let serde_deserialize_derive_or_impl = {
                enum ShouldAddDeLifetime {
                    False,
                    True,
                }
                let struct_ident_double_quotes_ts = postgres_crud_macros_common::gen_struct_ident_double_quotes_ts(&ident_origin_ucc);
                let tuple_struct_ident_double_quotes_ts = postgres_crud_macros_common::gen_tuple_struct_ident_double_quotes_ts(&ident_origin_ucc);
                let struct_visitor_ts = quote! {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private228::PhantomData<#ident_standart_not_null_origin_ucc>,
                        lifetime: serde::__private228::PhantomData<&'de ()>,
                    }
                };
                let start_end_std_fmt_display_plus_quote_to_tokens_array: [&dyn StdFmtDisplayPlusQuoteToTokens; 2] = [&StartSc, &EndSc];
                let hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array: [&dyn StdFmtDisplayPlusQuoteToTokens; 4] = [&HourSc, &MinSc, &SecSc, &MicroSc];
                let hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array: [&dyn StdFmtDisplayPlusQuoteToTokens; 4] = [&HourSc, &MinuteSc, &SecondSc, &MicrosecondSc];
                let date_time_std_fmt_display_plus_quote_to_tokens_array: [&dyn StdFmtDisplayPlusQuoteToTokens; 2] = [&DateSc, &TimeSc];
                let date_naive_time_std_fmt_display_plus_quote_to_tokens_array: [&dyn StdFmtDisplayPlusQuoteToTokens; 2] = [&DateNaiveSc, &TimeSc];
                let months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array: [&dyn StdFmtDisplayPlusQuoteToTokens; 3] = [&MonthsSc, &DaysSc, &MicrosecondsSc];
                let serde_deserializer_deserialize_struct_visitor_ts = {
                    quote! {
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #ident_standart_not_null_double_quotes_ts,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private228::PhantomData::<Self>,
                                lifetime: _serde::__private228::PhantomData,
                            }
                        )
                    }
                };
                let serde_deserializer_deserialize_newtype_struct_ts = quote! {
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        #ident_standart_not_null_origin_double_quotes_ts,
                        __Visitor {
                            marker: serde::__private228::PhantomData::<Self>,
                            lifetime: serde::__private228::PhantomData,
                        },
                    )
                };
                let gen_impl_serde_deserialize_for_tokens_ts = |content_ts: &dyn quote::ToTokens| {
                    quote! {
                        #[allow(unused_qualifications)]
                        #[allow(clippy::absolute_paths)]
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        const _: () = {
                            #[allow(unused_extern_crates, clippy::useless_attribute)]
                            extern crate serde as _serde;
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for #ident_standart_not_null_origin_ucc {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    #content_ts
                                }
                            }
                        };
                    }
                };
                let gen_field_index_ts = |index_52391f7d: usize| format!("__field{index_52391f7d}").parse::<proc_macro2::TokenStream>().expect("a4e1a63f-821b-4d35-823a-0a99efa9d1dc");
                let gen_field_index_value_ts = |index_7ef2fc7d: usize| format!("__field{index_7ef2fc7d}_value").parse::<proc_macro2::TokenStream>().expect("fa97be6c-6985-44f3-aec9-04adaf71dc8f");
                let (enum_field_two_ts, enum_field_three_ts, enum_field_four_ts) = {
                    let gen_enum_field_ts = |parameter_number: &ParameterNumber| {
                        let fields_ts = {
                            let fields_ts = parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(&gen_field_index_ts);
                            quote! {#(#fields_ts),*}
                        };
                        quote! {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                #fields_ts,
                                __ignore,
                            }
                        }
                    };
                    (gen_enum_field_ts(&parameter_number_two), gen_enum_field_ts(&parameter_number_three), gen_enum_field_ts(&parameter_number_four))
                };
                let (fn_expecting_struct_ident_double_quotes_ts, fn_expecting_tuple_struct_ident_double_quotes_ts, fn_expecting_field_identifier_ts) = {
                    let gen_fn_expecting_ts = |content_ts: &dyn quote::ToTokens| {
                        quote! {
                            fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                serde::__private228::Formatter::write_str(__f, #content_ts)
                            }
                        }
                    };
                    (gen_fn_expecting_ts(&struct_ident_double_quotes_ts), gen_fn_expecting_ts(&tuple_struct_ident_double_quotes_ts), gen_fn_expecting_ts(&quote! {"field identifier"}))
                };
                let field_0_value_ts = gen_field_index_value_ts(parameter_number_one.get_index());
                let gen_serde_private_ok_ts = |content_ts: &dyn quote::ToTokens| {
                    quote! {Ok(#content_ts)}
                };
                let gen_serde_private_ok_postgres_type_ts = |content_ts: &dyn quote::ToTokens| gen_serde_private_ok_ts(&quote! {#ident_standart_not_null_origin_ucc(#content_ts)});
                let match_uuid_uuid_field_type_try_parse_ts = quote! {match #inner_type_standart_not_null_ts::try_parse(&#field_0_value_ts) {
                    Ok(value_3c0b34fb) => value_3c0b34fb,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }};
                let sqlx_types_mac_address_mac_address_field_type_new_field_0_value_ts = quote! {#inner_type_standart_not_null_ts::#NewSc(#field_0_value_ts)};
                let array_std_primitive_u8_6_ts = quote! {[u8; 6]};
                let gen_vec_field_index_values_ts = |length: usize|{
                    let fields_ts = (1..=length).collect::<Vec<_>>().into_iter().enumerate().map(|(index_a8d5119e, _)| gen_field_index_value_ts(index_a8d5119e));
                    quote!{#(#fields_ts),*}
                };
                let (sqlx_types_chrono_naive_time_origin_try_new_for_deserialize, match_origin_try_new_for_deserialize_one_ts, match_origin_try_new_for_deserialize_two_ts, match_origin_try_new_for_deserialize_four_ts) = {
                    let gen_match_origin_try_new_for_deserialize_ts = |length: usize| {
                        let fields_ts = gen_vec_field_index_values_ts(length);
                        quote! {
                            match #ident_standart_not_null_origin_ucc::#TryNewForDeserializeSc(#fields_ts) {
                                Ok(value_e81dd4a5) => Ok(value_e81dd4a5),
                                Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                            }
                        }
                    };
                    (
                        gen_match_origin_try_new_for_deserialize_ts(hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array.len()),
                        gen_match_origin_try_new_for_deserialize_ts(1),
                        gen_match_origin_try_new_for_deserialize_ts(2),
                        gen_match_origin_try_new_for_deserialize_ts(4),
                    )
                };
                let (origin_new_for_deserialize_two_ts, origin_new_for_deserialize_three_ts) = {
                    let gen_origin_new_for_deserialize_ts = |length: usize| {
                        let fields_ts = gen_vec_field_index_values_ts(length);
                        quote! {
                            Ok(#ident_standart_not_null_origin_ucc::new_for_deserialize(#fields_ts))
                        }
                    };
                    (gen_origin_new_for_deserialize_ts(2), gen_origin_new_for_deserialize_ts(3))
                };
                let (fn_visit_newtype_struct_pg_money_ts, fn_visit_newtype_struct_uuid_ts, fn_visit_newtype_struct_mac_address_ts, fn_visit_newtype_struct_text_ts, fn_visit_newtype_struct_sqlx_types_chrono_naive_date_ts) = {
                    let gen_fn_visit_newtype_struct_ts = |type_ts: &dyn quote::ToTokens, serde_private_ok_ts: &dyn quote::ToTokens| {
                        quote! {
                            #[inline]
                            fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                            where
                                __E: serde::Deserializer<'de>,
                            {
                                let #field_0_value_ts = <#type_ts as serde::Deserialize>::deserialize(__e)?;
                                #serde_private_ok_ts
                            }
                        }
                    };
                    (
                        gen_fn_visit_newtype_struct_ts(&std_primitive_i64_ts, &gen_serde_private_ok_postgres_type_ts(&quote! {#inner_type_standart_not_null_ts(#field_0_value_ts)})),
                        gen_fn_visit_newtype_struct_ts(&std_string_string_ts, &gen_serde_private_ok_postgres_type_ts(&match_uuid_uuid_field_type_try_parse_ts)),
                        gen_fn_visit_newtype_struct_ts(&array_std_primitive_u8_6_ts, &gen_serde_private_ok_postgres_type_ts(&sqlx_types_mac_address_mac_address_field_type_new_field_0_value_ts)),
                        gen_fn_visit_newtype_struct_ts(&std_string_string_ts, &match_origin_try_new_for_deserialize_one_ts),
                        gen_fn_visit_newtype_struct_ts(&inner_type_standart_not_null_ts, &match_origin_try_new_for_deserialize_one_ts),
                    )
                };
                let gen_fields_serde_de_seq_access_next_el_initialization_ts = |vec_ts: &[&dyn quote::ToTokens]| {
                    let error_message_ts = postgres_crud_macros_common::gen_struct_ident_with_number_elements_double_quotes_ts(&ident_standart_not_null_origin_ucc, vec_ts.len());
                    let fields_initialization_ts = vec_ts.iter().enumerate().map(|(index_70b4dabd, el_9dc7f312)| {
                        let field_index_value_ts = gen_field_index_value_ts(index_70b4dabd);
                        let index_usize_ts = format!("{index_70b4dabd}usize").parse::<proc_macro2::TokenStream>().expect("ce15e6bf-cf71-42c3-9f6d-94d0f7ec6ede");
                        quote! {
                            let Some(#field_index_value_ts) = serde::de::SeqAccess::next_element::<#el_9dc7f312>(&mut __seq)? else {
                                return Err(serde::de::Error::invalid_length(#index_usize_ts, &#error_message_ts));
                            };
                        }
                    });
                    quote! {#(#fields_initialization_ts)*}
                };
                let (
                    fn_visit_seq_pg_money_ts,
                    fn_visit_seq_sqlx_types_chrono_naive_time_ts,
                    fn_visit_seq_uuid_uuid_ts,
                    fn_visit_seq_sqlx_types_mac_address_mac_address_ts,
                    fn_visit_seq_std_string_string_ts,
                    fn_visit_seq_sqlx_types_time_time_ts,
                    fn_visit_seq_sqlx_types_chrono_naive_date_ts,
                    fn_visit_seq_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_seq_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                    fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                    fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                    fn_visit_seq_sqlx_postgres_types_pg_interval_ts,
                ) = {
                    let gen_fn_visit_seq_ts = |content_ts: &dyn quote::ToTokens| {
                        quote! {
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::SeqAccess<'de>,
                            {
                                #content_ts
                            }
                        }
                    };
                    (
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_primitive_i64_ts]);
                            let serde_private_ok_postgres_type_ts = gen_serde_private_ok_postgres_type_ts(&quote! {#inner_type_standart_not_null_ts(#field_0_value_ts)});
                            quote! {
                                #fields_initialization_ts
                                #serde_private_ok_postgres_type_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_primitive_u32_ts, &std_primitive_u32_ts, &std_primitive_u32_ts, &std_primitive_u32_ts]);
                            quote! {
                                #fields_initialization_ts
                                #sqlx_types_chrono_naive_time_origin_try_new_for_deserialize
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_string_string_ts]);
                            let serde_private_ok_postgres_type_ts = gen_serde_private_ok_postgres_type_ts(&match_uuid_uuid_field_type_try_parse_ts);
                            quote! {
                                #fields_initialization_ts
                                #serde_private_ok_postgres_type_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&array_std_primitive_u8_6_ts]);
                            let serde_private_ok_postgres_type_ts = gen_serde_private_ok_postgres_type_ts(&sqlx_types_mac_address_mac_address_field_type_new_field_0_value_ts);
                            quote! {
                                #fields_initialization_ts
                                #serde_private_ok_postgres_type_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_string_string_ts]);
                            quote! {
                                #fields_initialization_ts
                                #match_origin_try_new_for_deserialize_one_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_primitive_u8_ts, &std_primitive_u8_ts, &std_primitive_u8_ts, &std_primitive_u32_ts]);
                            quote! {
                                #fields_initialization_ts
                                #match_origin_try_new_for_deserialize_four_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&inner_type_standart_not_null_ts]);
                            quote! {
                                #fields_initialization_ts
                                #match_origin_try_new_for_deserialize_one_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]);
                            quote! {
                                #fields_initialization_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]);
                            quote! {
                                #fields_initialization_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts]);
                            quote! {
                                #fields_initialization_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts]);
                            quote! {
                                #fields_initialization_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts]);
                            quote! {
                                #fields_initialization_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_ops_bound_std_primitive_i32_ts, &std_ops_bound_std_primitive_i32_ts]);
                            quote! {
                                #fields_initialization_ts
                                #match_origin_try_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_ops_bound_std_primitive_i64_ts, &std_ops_bound_std_primitive_i64_ts]);
                            quote! {
                                #fields_initialization_ts
                                #match_origin_try_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_initialization_ts = gen_fields_serde_de_seq_access_next_el_initialization_ts(&[&std_primitive_i32_ts, &std_primitive_i32_ts, &std_primitive_i64_ts]);
                            quote! {
                                #fields_initialization_ts
                                #origin_new_for_deserialize_three_ts
                            }
                        }),
                    )
                };
                let (fn_visit_u64_two_ts, fn_visit_u64_three_ts, fn_visit_u64_four_ts) = {
                    let gen_fn_visit_u64_ts = |parameter_number: &ParameterNumber| {
                        let fields_ts = {
                            parameter_number.get_vec_from_index_starting_with_zero().into_iter().map(|el_7298ebde| {
                                let index_variant_ts = format!("{el_7298ebde}u64").parse::<proc_macro2::TokenStream>().expect("5aee0393-2f04-42ca-87d6-bb4209d41ee1");
                                let field_index_ts = gen_field_index_ts(el_7298ebde);
                                quote! {#index_variant_ts => Ok(__Field::#field_index_ts)}
                            })
                        };
                        quote! {
                            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                            where
                                __E: serde::de::Error,
                            {
                                match __value {
                                    #(#fields_ts),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (gen_fn_visit_u64_ts(&parameter_number_two), gen_fn_visit_u64_ts(&parameter_number_three), gen_fn_visit_u64_ts(&parameter_number_four))
                };
                let (fn_visit_str_value_start_end_ts, fn_visit_str_value_hour_min_sec_micro_ts, fn_visit_str_value_hour_minute_second_microsecond_ts, fn_visit_str_value_date_time_ts, fn_visit_str_value_date_naive_time_ts, fn_visit_str_value_months_days_microseconds_ts) = {
                    let gen_fn_visit_str_ts = |vec_ts: &[&dyn StdFmtDisplayPlusQuoteToTokens]| {
                        let fields_ts = vec_ts.iter().enumerate().map(|(index_e1c5acfd, el_29343926)| {
                            let el_double_quotes_ts = gen_quotes::double_quotes_ts(&el_29343926);
                            let field_index_name_ts = gen_field_index_ts(index_e1c5acfd);
                            quote! {#el_double_quotes_ts => Ok(__Field::#field_index_name_ts)}
                        });
                        quote! {
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    #(#fields_ts),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (
                        gen_fn_visit_str_ts(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_str_ts(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_str_ts(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_str_ts(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_str_ts(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_str_ts(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    )
                };
                let (fn_visit_bytes_start_end_ts, fn_visit_bytes_hour_min_sec_micro_ts, fn_visit_bytes_hour_minute_second_microsecond_ts, fn_visit_bytes_date_time_ts, fn_visit_bytes_date_naive_time_ts, fn_visit_bytes_months_days_microseconds_ts) = {
                    let gen_fn_visit_bytes_ts = |vec_ts: &[&dyn StdFmtDisplayPlusQuoteToTokens]| {
                        let fields_ts = vec_ts.iter().enumerate().map(|(index_545c3b1e, el_1dbc37ab)| {
                            let b_el_double_quotes_ts = format!("b{}", gen_quotes::double_quotes_str(&el_1dbc37ab)).parse::<proc_macro2::TokenStream>().expect("c76c976b-9009-43d2-8d4b-1ec559b76008");
                            let field_index_name_ts = gen_field_index_ts(index_545c3b1e);
                            quote! {#b_el_double_quotes_ts => Ok(__Field::#field_index_name_ts)}
                        });
                        quote! {
                            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                            where
                                __E: serde::de::Error,
                            {
                                match __value {
                                    #(#fields_ts),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (
                        gen_fn_visit_bytes_ts(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_bytes_ts(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_bytes_ts(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_bytes_ts(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_bytes_ts(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                        gen_fn_visit_bytes_ts(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    )
                };
                let impl_serde_deserialize_for_field_ts = quote! {
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
                    fn_visit_map_sqlx_types_chrono_naive_time_ts,
                    fn_visit_map_sqlx_types_time_time_ts,
                    fn_visit_map_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_map_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                    fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                    fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                    fn_visit_map_sqlx_postgres_types_pg_interval_ts,
                ) = {
                    let gen_fn_visit_map_ts = |field_option_none_initialization_ts: &dyn quote::ToTokens, while_some_next_key_field_ts: &dyn quote::ToTokens, match_field_initialization_ts: &dyn quote::ToTokens, serde_private_ok_ts: &dyn quote::ToTokens| {
                        quote! {
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::MapAccess<'de>,
                            {
                                #field_option_none_initialization_ts
                                #while_some_next_key_field_ts
                                #match_field_initialization_ts
                                #serde_private_ok_ts
                            }
                        }
                    };
                    let (
                        field_option_none_initialization_sqlx_types_chrono_naive_time_ts,
                        field_option_none_initialization_sqlx_types_time_time_ts,
                        field_option_none_initialization_sqlx_types_chrono_naive_date_time_ts,
                        field_option_none_initialization_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                        field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                        field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                        field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                        field_option_none_initialization_sqlx_postgres_types_pg_interval_ts,
                    ) = {
                        let gen_field_option_none_initialization_ts = |vec_ts: &[&dyn quote::ToTokens]| {
                            let fields_initialization_ts = vec_ts.iter().enumerate().map(|(index_d9ee264a, el_de75f565)| {
                                let field_index_name_ts = gen_field_index_ts(index_d9ee264a);
                                quote! {let mut #field_index_name_ts: Option<#el_de75f565> = None;}
                            });
                            quote! {#(#fields_initialization_ts)*}
                        };
                        (
                            gen_field_option_none_initialization_ts(&[&std_primitive_u32_ts, &std_primitive_u32_ts, &std_primitive_u32_ts, &std_primitive_u32_ts]),
                            gen_field_option_none_initialization_ts(&[&std_primitive_u8_ts, &std_primitive_u8_ts, &std_primitive_u8_ts, &std_primitive_u32_ts]),
                            gen_field_option_none_initialization_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]),
                            gen_field_option_none_initialization_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]),
                            gen_field_option_none_initialization_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts]),
                            gen_field_option_none_initialization_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts]),
                            gen_field_option_none_initialization_ts(&[&std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts]),
                            gen_field_option_none_initialization_ts(&[&std_ops_bound_std_primitive_i32_ts, &std_ops_bound_std_primitive_i32_ts]),
                            gen_field_option_none_initialization_ts(&[&std_ops_bound_std_primitive_i64_ts, &std_ops_bound_std_primitive_i64_ts]),
                            gen_field_option_none_initialization_ts(&[&std_primitive_i32_ts, &std_primitive_i32_ts, &std_primitive_i64_ts]),
                        )
                    };
                    let (
                        while_some_next_key_field_sqlx_types_chrono_naive_time_ts,
                        while_some_next_key_field_sqlx_types_time_time_ts,
                        while_some_next_key_field_sqlx_types_chrono_naive_date_time_ts,
                        while_some_next_key_field_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                        while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                        while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                        while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                        while_some_next_key_field_sqlx_postgres_types_pg_interval_ts,
                    ) = {
                        let gen_while_some_next_key_field_ts = |vec_ts: &[(&dyn Display, &dyn quote::ToTokens)]| {
                            let fields_initialization_ts = vec_ts.iter().enumerate().map(|(index_2b1736c7, el_692238ce)| {
                                let field_name_double_quotes_ts = gen_quotes::double_quotes_str(&el_692238ce.0);
                                let field_type_ts = &el_692238ce.1;
                                let field_index_name_ts = gen_field_index_ts(index_2b1736c7);
                                quote! {
                                    __Field::#field_index_name_ts => {
                                        if Option::is_some(&#field_index_name_ts) {
                                            return Err(<__A::Error as serde::de::Error>::duplicate_field(#field_name_double_quotes_ts));
                                        }
                                        #field_index_name_ts = Some(serde::de::MapAccess::next_value::<#field_type_ts>(&mut __map)?);
                                    }
                                }
                            });
                            quote! {
                                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        #(#fields_initialization_ts)*
                                        __Field::__ignore => {
                                            let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                        }
                                    }
                                }
                            }
                        };
                        (
                            gen_while_some_next_key_field_ts(&[(&HourSc, &std_primitive_u32_ts), (&MinSc, &std_primitive_u32_ts), (&SecSc, &std_primitive_u32_ts), (&MicroSc, &std_primitive_u32_ts)]),
                            gen_while_some_next_key_field_ts(&[(&HourSc, &std_primitive_u8_ts), (&MinuteSc, &std_primitive_u8_ts), (&SecondSc, &std_primitive_u8_ts), (&MicrosecondSc, &std_primitive_u32_ts)]),
                            gen_while_some_next_key_field_ts(&[(&DateSc, &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc), (&TimeSc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc)]),
                            gen_while_some_next_key_field_ts(&[(&DateNaiveSc, &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc), (&TimeSc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc)]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts), (&EndSc, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts)]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts), (&EndSc, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts)]),
                            gen_while_some_next_key_field_ts(&[
                                (&StartSc, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts),
                                (&EndSc, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts),
                            ]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_std_primitive_i32_ts), (&EndSc, &std_ops_bound_std_primitive_i32_ts)]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_std_primitive_i64_ts), (&EndSc, &std_ops_bound_std_primitive_i64_ts)]),
                            gen_while_some_next_key_field_ts(&[(&MonthsSc, &std_primitive_i32_ts), (&DaysSc, &std_primitive_i32_ts), (&MicrosecondsSc, &std_primitive_i64_ts)]),
                        )
                    };
                    let (match_field_initialization_hour_min_sec_micro_ts, match_field_initialization_start_end_ts, match_field_initialization_hour_minute_second_microsecond_ts, match_field_initialization_date_time_ts, match_field_initialization_date_naive_time_ts, match_field_initialization_months_days_microseconds_ts) = {
                        let gen_match_field_initialization_ts = |vec_ts: &[&dyn StdFmtDisplayPlusQuoteToTokens]| {
                            let fields_initialization_ts = vec_ts.iter().enumerate().map(|(index_e1adef1a, el_f8a9e25b)| {
                                let field_name_double_quotes_ts = gen_quotes::double_quotes_str(&el_f8a9e25b);
                                let field_index_ts = gen_field_index_ts(index_e1adef1a);
                                let field_index_value_ts = gen_field_index_value_ts(index_e1adef1a);
                                quote! {
                                    let #field_index_value_ts = match #field_index_ts {
                                        Some(value_eeeb431b) => value_eeeb431b,
                                        None => serde::__private228::de::missing_field(#field_name_double_quotes_ts)?,
                                    };
                                }
                            });
                            quote! {#(#fields_initialization_ts)*}
                        };
                        (
                            gen_match_field_initialization_ts(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                            gen_match_field_initialization_ts(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                            gen_match_field_initialization_ts(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                            gen_match_field_initialization_ts(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                            gen_match_field_initialization_ts(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                            gen_match_field_initialization_ts(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                        )
                    };
                    (
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_types_chrono_naive_time_ts,
                            &while_some_next_key_field_sqlx_types_chrono_naive_time_ts,
                            &match_field_initialization_hour_min_sec_micro_ts,
                            &sqlx_types_chrono_naive_time_origin_try_new_for_deserialize,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_types_time_time_ts,
                            &while_some_next_key_field_sqlx_types_time_time_ts,
                            &match_field_initialization_hour_minute_second_microsecond_ts,
                            &match_origin_try_new_for_deserialize_four_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_types_chrono_naive_date_time_ts,
                            &while_some_next_key_field_sqlx_types_chrono_naive_date_time_ts,
                            &match_field_initialization_date_time_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &while_some_next_key_field_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &match_field_initialization_date_naive_time_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            &match_field_initialization_start_end_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            &match_field_initialization_start_end_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &match_field_initialization_start_end_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                            &while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                            &match_field_initialization_start_end_ts,
                            &match_origin_try_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                            &while_some_next_key_field_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                            &match_field_initialization_start_end_ts,
                            &match_origin_try_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_option_none_initialization_sqlx_postgres_types_pg_interval_ts,
                            &while_some_next_key_field_sqlx_postgres_types_pg_interval_ts,
                            &match_field_initialization_months_days_microseconds_ts,
                            &origin_new_for_deserialize_three_ts,
                        ),
                    )
                };
                let (const_fields_start_end_ts, const_fields_sqlx_types_chrono_naive_time_ts, const_fields_sqlx_types_time_time_ts, const_fields_sqlx_types_chrono_naive_date_time_ts, const_fields_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts, const_fields_sqlx_postgres_types_pg_interval_ts) = {
                    let gen_const_fields_ts = |vec_ts: &[&dyn StdFmtDisplayPlusQuoteToTokens]| {
                        let field_names_ts = vec_ts.iter().map(|el_391d76e4| gen_quotes::double_quotes_ts(&el_391d76e4));
                        quote! {
                            #[doc(hidden)]
                            const FIELDS: &[&str] = &[#(#field_names_ts),*];
                        }
                    };
                    (
                        gen_const_fields_ts(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        gen_const_fields_ts(&hour_min_sec_micro_std_fmt_display_plus_quote_to_tokens_array),
                        gen_const_fields_ts(&hour_minute_second_microsecond_std_fmt_display_plus_quote_to_tokens_array),
                        gen_const_fields_ts(&date_time_std_fmt_display_plus_quote_to_tokens_array),
                        gen_const_fields_ts(&date_naive_time_std_fmt_display_plus_quote_to_tokens_array),
                        gen_const_fields_ts(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    )
                };
                let gen_impl_serde_de_visitor_for_tokens_ts = |
                    should_add_de_lifetime: ShouldAddDeLifetime,
                    current_ident_ts: &dyn quote::ToTokens,
                    content_ts: &dyn quote::ToTokens
                | {
                    let (
                        maybe_impl_lifetime_ts,
                        maybe_visitor_lifetime_ts
                    ) = match should_add_de_lifetime{
                        ShouldAddDeLifetime::False => (
                            proc_macro2::TokenStream::new(),
                            quote!{<'_>},
                        ),
                        ShouldAddDeLifetime::True => (
                            quote!{<'de>},
                            quote!{<'de>},
                        ),
                    };
                    quote! {
                        impl #maybe_impl_lifetime_ts _serde::de::Visitor #maybe_visitor_lifetime_ts for #current_ident_ts {
                            #content_ts
                        }
                    }
                };
                let (
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_time_ts,
                    impl_serde_de_visitor_for_visitor_pg_money_ts,
                    impl_serde_de_visitor_for_visitor_uuid_uuid_ts,
                    impl_serde_de_visitor_for_visitor_mac_address_mac_address_ts,
                    impl_serde_de_visitor_for_visitor_std_string_string_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_time_time_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_time_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_interval_ts,
                ) = {
                    let gen_impl_serde_de_visitor_for_visitor_ts = |zero_ts: &dyn quote::ToTokens, first_ts: &dyn quote::ToTokens, second_ts: &dyn quote::ToTokens| {
                        gen_impl_serde_de_visitor_for_tokens_ts(
                            ShouldAddDeLifetime::True,
                            &quote! {__Visitor<'de>},
                            &quote! {
                                type Value = #ident_standart_not_null_origin_ucc;
                                #zero_ts
                                #first_ts
                                #second_ts
                            },
                        )
                    };
                    (
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_types_chrono_naive_time_ts, &fn_visit_map_sqlx_types_chrono_naive_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_double_quotes_ts, &fn_visit_newtype_struct_pg_money_ts, &fn_visit_seq_pg_money_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_double_quotes_ts, &fn_visit_newtype_struct_uuid_ts, &fn_visit_seq_uuid_uuid_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_double_quotes_ts, &fn_visit_newtype_struct_mac_address_ts, &fn_visit_seq_sqlx_types_mac_address_mac_address_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_double_quotes_ts, &fn_visit_newtype_struct_text_ts, &fn_visit_seq_std_string_string_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_types_time_time_ts, &fn_visit_map_sqlx_types_time_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_double_quotes_ts, &fn_visit_newtype_struct_sqlx_types_chrono_naive_date_ts, &fn_visit_seq_sqlx_types_chrono_naive_date_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_types_chrono_naive_date_time_ts, &fn_visit_map_sqlx_types_chrono_naive_date_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts, &fn_visit_map_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(
                            &fn_expecting_struct_ident_double_quotes_ts,
                            &fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_ts, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i32_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_ts, &fn_visit_map_sqlx_postgres_types_pg_range_sqlx_postgres_types_pg_range_std_primitive_i64_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_double_quotes_ts, &fn_visit_seq_sqlx_postgres_types_pg_interval_ts, &fn_visit_map_sqlx_postgres_types_pg_interval_ts),
                    )
                };
                let field_visitor_ts = quote! {__FieldVisitor};
                let type_value_equal_underscore_field_semicolon_ts = quote! {type Value = __Field;};
                let (
                    impl_serde_de_visitor_for_field_visitor_ts_5a4f24ce_7a8e_4bcc_8f79_2494f79bcc08,
                    impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33_bf35_4c13_a745_341364a68df6,
                    impl_serde_de_visitor_for_field_visitor_ts_9b240c3e_a4af_4da1_a2ab_f1bab44b1df6,
                    impl_serde_de_visitor_for_field_visitor_ts_dc439ca1_8af1_4c4c_ab49_4e4fb15a41d3,
                    impl_serde_de_visitor_for_field_visitor_ts_8c733fe0_c816_4a0e_bb13_4c2d0cd2ded6,
                    impl_serde_de_visitor_for_field_visitor_ts_f702a411_b02b_4c90_aa7f_962a698612e7,
                ) = {
                    let gen_impl_serde_de_visitor_for_field_visitor_ts = |content_ts: &dyn quote::ToTokens| {
                        let impl_serde_de_visitor_for_tokens_ts = gen_impl_serde_de_visitor_for_tokens_ts(
                            ShouldAddDeLifetime::False,
                            &field_visitor_ts,
                            &content_ts
                        );
                        quote! {
                            #[doc(hidden)]
                            struct #field_visitor_ts;
                            #impl_serde_de_visitor_for_tokens_ts
                        }
                    };
                    (
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_field_identifier_ts
                            #fn_visit_u64_four_ts
                            #fn_visit_str_value_hour_min_sec_micro_ts
                            #fn_visit_bytes_hour_min_sec_micro_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_field_identifier_ts
                            #fn_visit_u64_two_ts
                            #fn_visit_str_value_start_end_ts
                            #fn_visit_bytes_start_end_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_field_identifier_ts
                            #fn_visit_u64_four_ts
                            #fn_visit_str_value_hour_minute_second_microsecond_ts
                            #fn_visit_bytes_hour_minute_second_microsecond_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_field_identifier_ts
                            #fn_visit_u64_two_ts
                            #fn_visit_str_value_date_time_ts
                            #fn_visit_bytes_date_time_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_field_identifier_ts
                            #fn_visit_u64_two_ts
                            #fn_visit_str_value_date_naive_time_ts
                            #fn_visit_bytes_date_naive_time_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_field_identifier_ts
                            #fn_visit_u64_three_ts
                            #fn_visit_str_value_months_days_microseconds_ts
                            #fn_visit_bytes_months_days_microseconds_ts
                        }),
                    )
                };
                let impl_serde_deserialize_for_uuid_uuid_ts = gen_impl_serde_deserialize_for_tokens_ts(&{
                    quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_uuid_uuid_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    }
                });
                match &postgres_type {
                    PostgresType::StdPrimitiveI16AsInt2
                    | PostgresType::StdPrimitiveI32AsInt4
                    | PostgresType::StdPrimitiveI64AsInt8
                    | PostgresType::StdPrimitiveF32AsFloat4
                    | PostgresType::StdPrimitiveF64AsFloat8
                    | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                    | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                    | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                    | PostgresType::StdPrimitiveBoolAsBool
                    | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                    | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => postgres_crud_macros_common::DeriveOrImpl::Derive,
                    PostgresType::SqlxPostgresTypesPgMoneyAsMoney => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_pg_money_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PostgresType::StdStringStringAsText => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_std_string_string_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PostgresType::SqlxTypesChronoNaiveTimeAsTime => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_four_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_5a4f24ce_7a8e_4bcc_8f79_2494f79bcc08
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_time_ts
                        #const_fields_sqlx_types_chrono_naive_time_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxTypesTimeTimeAsTime => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_four_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_9b240c3e_a4af_4da1_a2ab_f1bab44b1df6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_time_time_ts
                        #const_fields_sqlx_types_time_time_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxTypesChronoNaiveDateAsDate => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_dc439ca1_8af1_4c4c_ab49_4e4fb15a41d3
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_time_ts
                        #const_fields_sqlx_types_chrono_naive_date_time_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_8c733fe0_c816_4a0e_bb13_4c2d0cd2ded6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts
                        #const_fields_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => postgres_crud_macros_common::DeriveOrImpl::Impl(impl_serde_deserialize_for_uuid_uuid_ts),
                    PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_mac_address_mac_address_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PostgresType::SqlxPostgresTypesPgIntervalAsInterval => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_three_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f702a411_b02b_4c90_aa7f_962a698612e7
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_interval_ts
                        #const_fields_sqlx_postgres_types_pg_interval_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33_bf35_4c13_a745_341364a68df6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i32_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33_bf35_4c13_a745_341364a68df6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_std_primitive_i64_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33_bf35_4c13_a745_341364a68df6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33_bf35_4c13_a745_341364a68df6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => postgres_crud_macros_common::DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33_bf35_4c13_a745_341364a68df6
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                }
            };
            (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl)
        } else {
            (postgres_crud_macros_common::DeriveOrImpl::Derive, postgres_crud_macros_common::DeriveOrImpl::Derive)
        };
        let value_ident_inner_type_ts = quote! {#ValueSc: #ident_inner_type_ts};
        let ident_standart_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_standart_not_null_origin_try_new_error_named_ucc = SelfOriginTryNewErrorNamedUcc::from_display(&ident_standart_not_null_ucc);
        let ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc = SelfOriginTryNewForDeserializeErrorNamedUcc::from_display(&ident_standart_not_null_ucc);
        let int_range_type_to_range_inner_type_ts = |int_range_type: &IntRangeType| -> proc_macro2::TokenStream {
            match &int_range_type {
                IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => quote! {#std_primitive_i32_ts},
                IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote! {#std_primitive_i64_ts},
            }
        };
        let gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = |content_ts: &dyn quote::ToTokens| {
            quote! {sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                #content_ts,
                sqlx::types::chrono::Utc
            )}
        };
        let gen_sqlx_types_chrono_naive_date_time_new_ts = |content_ts: &dyn quote::ToTokens| {
            quote! {sqlx::types::chrono::NaiveDateTime::#NewSc(#content_ts)}
        };
        let gen_sqlx_types_time_time_from_hms_micro_unwrap_ts = |content_ts: &dyn quote::ToTokens| {
            quote! {sqlx::types::time::Time::from_hms_micro(#content_ts).expect("7a1a18fa-c0cf-45e4-8b52-60f58a793c36")}
        };
        let gen_pub_const_new_or_pub_try_new_ts = |current_ident: &dyn quote::ToTokens| {
            let pub_fn_new_or_try_new_ts = if postgres_type_initialization_try_new_try_from_postgres_type.is_ok() {
                &macros_helpers::gen_pub_try_new_ts(
                    &value_ident_inner_type_ts,
                    &ident_standart_not_null_origin_try_new_error_named_ucc,
                    &quote! {
                        match #ident_origin_ucc::#TryNewSc(#ValueSc) {
                            Ok(value_0f9f1a61) => Ok(Self(value_0f9f1a61)),
                            Err(er) => Err(er)
                        }
                    },
                )
            } else {
                &{
                    let self_ident_origin_new_value_ts = quote! {Self(#ident_origin_ucc::#NewSc(#ValueSc))};
                    if matches!(&postgres_type_pattern, PostgresTypePattern::Standart)
                        && matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
                    {
                        macros_helpers::gen_pub_const_new_ts(
                            &must_use_ts,
                            &value_ident_inner_type_ts,
                            &self_ident_origin_new_value_ts
                        )
                    } else {
                        macros_helpers::gen_pub_new_ts(
                            &must_use_ts,
                            &value_ident_inner_type_ts,
                            &self_ident_origin_new_value_ts
                        )
                    }
                }
            };
            quote! {
                impl #current_ident {
                    #pub_fn_new_or_try_new_ts
                }
            }
        };
        let derive_copy = match &postgres_type_pattern {
            PostgresTypePattern::Standart => match &postgres_type {
                PostgresType::StdPrimitiveI16AsInt2 |
                PostgresType::StdPrimitiveI32AsInt4 |
                PostgresType::StdPrimitiveI64AsInt8 |
                PostgresType::StdPrimitiveF32AsFloat4 |
                PostgresType::StdPrimitiveF64AsFloat8 |
                PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                PostgresType::StdPrimitiveBoolAsBool |
                PostgresType::SqlxTypesChronoNaiveTimeAsTime |
                PostgresType::SqlxTypesTimeTimeAsTime |
                PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                PostgresType::SqlxTypesChronoNaiveDateAsDate |
                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::DeriveCopy::True,
                PostgresType::StdStringStringAsText |
                PostgresType::StdVecVecStdPrimitiveU8AsBytea => macros_helpers::DeriveCopy::False,
            },
            PostgresTypePattern::ArrayDimension1 { .. } => macros_helpers::DeriveCopy::False,
        };
        let sqlx_types_chrono_naive_time_min_function_ts = quote!{sqlx_types_chrono_naive_time_min};
        let sqlx_types_chrono_naive_time_ten_function_ts = quote!{sqlx_types_chrono_naive_time_ten};
        let sqlx_types_chrono_naive_time_twenty_function_ts = quote!{sqlx_types_chrono_naive_time_twenty};
        let sqlx_types_chrono_naive_time_max_function_ts = quote!{sqlx_types_chrono_naive_time_max};
        let sqlx_types_chrono_naive_date_min_function_ts = quote!{sqlx_types_chrono_naive_date_min};

        let sqlx_types_chrono_naive_date_negative_less_typical_function_ts = quote!{sqlx_types_chrono_naive_date_negative_less_typical};
        let sqlx_types_chrono_naive_date_negative_more_typical_function_ts = quote!{sqlx_types_chrono_naive_date_negative_more_typical};
        let sqlx_types_chrono_naive_date_near_zero_function_ts = quote!{sqlx_types_chrono_naive_date_near_zero};
        let sqlx_types_chrono_naive_date_positive_less_typical_function_ts = quote!{sqlx_types_chrono_naive_date_positive_less_typical};
        let sqlx_types_chrono_naive_date_positive_more_typical_function_ts = quote!{sqlx_types_chrono_naive_date_positive_more_typical};
        let sqlx_types_chrono_naive_date_max_function_ts = quote!{sqlx_types_chrono_naive_date_max};
        let sqlx_types_chrono_naive_date_max_pred_opt_expect_function_ts = quote!{sqlx_types_chrono_naive_date_max_pred_opt_expect};

        let ident_ts = {
            let ident_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .derive_partial_eq()
                .build_struct(
                    &ident,
                    &quote!{;},
                );
            // println!("@@@{}", ident_inner_type_ts);
            let maybe_impl_ident_ts = if matches!(&postgres_type_pattern, PostgresTypePattern::Standart) &&
                matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
            {
                enum IsConst {
                    False,
                    True,
                }
                let gen_inner_type_ts = |
                    is_const: IsConst,
                    name_ts: &dyn quote::ToTokens,
                    content_ts: &dyn quote::ToTokens
                |{
                    let maybe_const_ts = match is_const {
                        IsConst::False => proc_macro2::TokenStream::new(),
                        IsConst::True => quote!{const},
                    };
                    quote!{
                        #maybe_const_ts fn #name_ts() -> #ident_inner_type_ts {
                            #content_ts
                        }
                    }
                };
                let maybe_min_inner_type_ts = {
                    let gen_inner_type_ts_67fc7980 = |
                        is_const: IsConst,
                        content_ts_1ca2df79: &dyn quote::ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{min_inner_type},
                        &content_ts_1ca2df79
                    );
                    match &postgres_type {
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_67fc7980(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("000ddcc2-7057-4310-bbee-81c5fe6323c3")
                                }
                            )
                        ),
                        PostgresType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_67fc7980(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("f065e2b1-0e7b-4bc8-8fa6-49843c90ff7c")
                                }
                            )
                        ),
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateAsDate |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_more_than_min_inner_type_ts = {
                    let gen_inner_type_ts_6d89728a = |
                        is_const: IsConst,
                        content_ts_dcc22544: &dyn quote::ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{slightly_more_than_min_inner_type},
                        &content_ts_dcc22544
                    );
                    match &postgres_type {
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_6d89728a(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("9545a47c-6795-43a3-b8be-d92ab1cd6e40")
                                }
                            )
                        ),
                        PostgresType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_6d89728a(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("03f9561a-aee1-4bf9-8b36-3ba9440af108")
                                }
                            )
                        ),
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateAsDate |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_middle_inner_type_ts = {
                    let gen_inner_type_ts_23368199 = |
                        is_const: IsConst,
                        content_ts_645cff79: &dyn quote::ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{middle_inner_type},
                        &content_ts_645cff79
                    );
                    match &postgres_type {
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_23368199(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("0dafc3fc-83a1-40fe-965b-e3fba46a18c9")
                                }
                            )
                        ),
                        PostgresType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_23368199(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("d2ec329f-15ac-45b1-bbdc-51c08a23ad93")
                                }
                            )
                        ),
                        PostgresType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_inner_type_ts_23368199(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("a2f306ea-bf67-4743-9274-fbdcab3b8e22")
                                }
                            )
                        ),
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_more_than_middle_inner_type_ts = {
                    let gen_inner_type_ts_3a61c0b0 = |
                        is_const: IsConst,
                        content_ts_e09b85a8: &dyn quote::ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{slightly_more_than_middle_inner_type},
                        &content_ts_e09b85a8
                    );
                    match &postgres_type {
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_3a61c0b0(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("235276a7-7f04-4f8f-b4f2-084694243bf0")
                                }
                            )
                        ),
                        PostgresType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_3a61c0b0(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("6a3dbcaa-5004-4cc4-a53c-8b65cdfa064c")
                                }
                            )
                        ),
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateAsDate |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_max_inner_type_ts = {
                    let gen_inner_type_ts_32acb388 = |
                        is_const: IsConst,
                        content_ts_385694da: &dyn quote::ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{max_inner_type},
                        &content_ts_385694da
                    );
                    match &postgres_type {
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_32acb388(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("b217e3bf-f8d6-425d-85f9-f9610cc3ce3f")
                                }
                            )
                        ),
                        PostgresType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_inner_type_ts_32acb388(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveDate::MAX
                                }
                            )
                        ),
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxTypesTimeTimeAsTime |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_less_than_max_inner_type_ts = {
                    let gen_inner_type_ts_ddf0f630 = |
                        is_const: IsConst,
                        content_ts_5ca08aea: &dyn quote::ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{slightly_less_than_max_inner_type},
                        &content_ts_5ca08aea
                    );
                    match &postgres_type {
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_ddf0f630(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_998).expect("5d6cf475-56c8-4abf-a754-43f24caaafa5")
                                }
                            )
                        ),
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxTypesTimeTimeAsTime |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateAsDate |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_read_inner_initializations_ts = {
                    let gen_function_ts = |
                        name_ts: &proc_macro2::TokenStream,
                        content_ts_5dfcb210: &proc_macro2::TokenStream
                    |quote!{
                        const fn #name_ts() -> #ident_inner_type_ts {
                            #content_ts_5dfcb210
                        }
                    };
                    match &postgres_type {
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxTypesTimeTimeAsTime |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime => Some({
                            let content_ts_80e0683c = [
                                (&sqlx_types_chrono_naive_time_min_function_ts, &quote!{0,0,0,0}),
                                (&sqlx_types_chrono_naive_time_ten_function_ts, &quote!{10,10,10,10}),
                                (&sqlx_types_chrono_naive_time_twenty_function_ts, &quote!{20,20,20,20}),
                                (&sqlx_types_chrono_naive_time_max_function_ts, &quote!{23,59,59,999_999}),
                            ].iter().map(|(name_ts, parameters_ts)| quote! {
                                const fn #name_ts() -> #ident_inner_type_ts {
                                    #ident_inner_type_ts::from_hms_micro_opt(#parameters_ts).expect("149e01cc-429b-4783-bfc1-2908db0b801f")
                                }
                            }).collect::<Vec<proc_macro2::TokenStream>>();
                            quote!{#(#content_ts_80e0683c)*}
                        }),
                        PostgresType::SqlxTypesChronoNaiveDateAsDate => Some({
                            let content_ts_80e0683c = {
                                let gen_function_ident_inner_type_ts = |
                                    name_ts: &proc_macro2::TokenStream,
                                    content_ts_a29ab1c6: &proc_macro2::TokenStream
                                |gen_function_ts(
                                    name_ts,
                                    &quote!{#ident_inner_type_ts::#content_ts_a29ab1c6}
                                );
                                [
                                    gen_function_ident_inner_type_ts(
                                        &sqlx_types_chrono_naive_date_max_function_ts,
                                        &quote! { MAX }
                                    ),
                                    gen_function_ts(
                                        &sqlx_types_chrono_naive_date_max_pred_opt_expect_function_ts,
                                        &quote!{Self::#sqlx_types_chrono_naive_date_max_function_ts().pred_opt().expect("b7e16bf1-9e73-4a34-98b0-4e6e9e3d45fb")}
                                    )
                                ]
                                .into_iter()
                                .chain(
                                    [
                                        (&sqlx_types_chrono_naive_date_min_function_ts, &quote! { -4713, 12, 31 }),
                                        (&sqlx_types_chrono_naive_date_negative_less_typical_function_ts, &quote! { -2000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_negative_more_typical_function_ts, &quote! { -1000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_near_zero_function_ts, &quote! { 0, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_positive_less_typical_function_ts, &quote! { 1000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_positive_more_typical_function_ts, &quote! { 2000, 1, 1 }),
                                    ]
                                    .into_iter()
                                    .map(|(name_ts, parameters_ts)| {
                                        gen_function_ident_inner_type_ts(
                                            name_ts,
                                            &quote! {
                                                from_ymd_opt(#parameters_ts)
                                                    .expect("d25ee0e9-4a6b-4b20-b8e3-3f703e121088")
                                            }
                                        )
                                    })
                                ).collect::<Vec<proc_macro2::TokenStream>>()
                            };
                            quote!{#(#content_ts_80e0683c)*}
                        }),
                    }
                };
                if maybe_min_inner_type_ts.is_some() ||
                    maybe_slightly_more_than_min_inner_type_ts.is_some() ||
                    maybe_middle_inner_type_ts.is_some() ||
                    maybe_slightly_more_than_middle_inner_type_ts.is_some() ||
                    maybe_max_inner_type_ts.is_some() ||
                    maybe_slightly_less_than_max_inner_type_ts.is_some() ||
                    maybe_read_inner_initializations_ts.is_some()
                {
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        impl #ident {
                            #maybe_min_inner_type_ts
                            #maybe_slightly_more_than_min_inner_type_ts
                            #maybe_middle_inner_type_ts
                            #maybe_slightly_more_than_middle_inner_type_ts
                            #maybe_max_inner_type_ts
                            #maybe_slightly_less_than_max_inner_type_ts
                            #maybe_read_inner_initializations_ts
                        }
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote!{
                #ident_ts
                #maybe_impl_ident_ts
            }
        };
        let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
        let ident_origin_ts = {
            let ident_origin_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_eq_if(match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::False => macros_helpers::DeriveEq::False,
                    IsNotNullStandartCanBePrimaryKey::True => macros_helpers::DeriveEq::True,
                })
                .derive_partial_ord_if(match &is_standart_not_null {
                    postgres_crud_macros_common::IsStandartNotNull::False => macros_helpers::DerivePartialOrd::False,
                    postgres_crud_macros_common::IsStandartNotNull::True => match &postgres_type {
                        PostgresType::StdPrimitiveI16AsInt2
                        | PostgresType::StdPrimitiveI32AsInt4
                        | PostgresType::StdPrimitiveI64AsInt8
                        | PostgresType::StdPrimitiveF32AsFloat4
                        | PostgresType::StdPrimitiveF64AsFloat8
                        | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveBoolAsBool
                        | PostgresType::StdStringStringAsText
                        | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                        | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                        | PostgresType::SqlxTypesTimeTimeAsTime
                        | PostgresType::SqlxTypesChronoNaiveDateAsDate
                        | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => macros_helpers::DerivePartialOrd::True,
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                        | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                        | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                        | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                        | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::DerivePartialOrd::False,
                    },
                })
                .derive_ord_if(match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::False => macros_helpers::DeriveOrd::False,
                    IsNotNullStandartCanBePrimaryKey::True => macros_helpers::DeriveOrd::True,
                })
                .derive_serde_serialize_if(match &serde_serialize_derive_or_impl {
                    postgres_crud_macros_common::DeriveOrImpl::Derive => macros_helpers::DeriveSerdeSerialize::True,
                    postgres_crud_macros_common::DeriveOrImpl::Impl(_) => macros_helpers::DeriveSerdeSerialize::False,
                })
                .derive_serde_deserialize_if(match &serde_deserialize_derive_or_impl {
                    postgres_crud_macros_common::DeriveOrImpl::Derive => macros_helpers::DeriveSerdeDeserialize::True,
                    postgres_crud_macros_common::DeriveOrImpl::Impl(_) => macros_helpers::DeriveSerdeDeserialize::False,
                })
                .build_struct(
                    &ident_origin_ucc,
                    &quote!{(#field_type_handle);},
                );
            let gen_int_range_type_error_variants_ts = |int_range_type: &IntRangeType| {
                let range_inner_type_ts = int_range_type_to_range_inner_type_ts(int_range_type);
                quote! {
                    #IncludedStartGreaterThanIncludedEndUcc {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #EndSc: #range_inner_type_ts,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #IncludedStartGreaterThanExcludedEndUcc {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #EndSc: #range_inner_type_ts,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #ExcludedStartGreaterThanIncludedEndUcc {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #EndSc: #range_inner_type_ts,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #ExcludedStartGreaterThanExcludedEndUcc {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #EndSc: #range_inner_type_ts,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #IncludedEndCannotBeMaxUcc {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #EndSc: #range_inner_type_ts,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            };
            let nanosecond_precision_is_not_supported_variant_try_new_ts = quote! {
                #NanosecondPrecisionIsNotSupportedUcc {
                    #[eo_to_std_string_string_serialize_deserialize]
                    #ValueSc: #std_string_string_ts,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                }
            };
            let sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_ts = quote! {
                #EarlierDateNotSupportedUcc {
                    #[eo_to_std_string_string_serialize_deserialize]
                    #ValueSc: #std_string_string_ts,
                    #[eo_to_std_string_string_serialize_deserialize]
                    #EarliestSupportedDateSc: #std_string_string_ts,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                }
            };
            let std_string_string_as_text_try_new_error_named_variants_ts = quote! {
                #ContainsNullByteUcc {
                    #[eo_to_std_string_string_serialize_deserialize]
                    #ValueSc: #ident_inner_type_ts,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                }
            };
            let maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_ts = if matches!(&is_standart_not_null, postgres_crud_macros_common::IsStandartNotNull::True)
                && let Ok(postgres_type_initialization_try_new) = &postgres_type_initialization_try_new_try_from_postgres_type
            {
                let content_ts_d57d5de2 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_thiserror_error()
                    .derive_error_occurence_lib_error_occurence()
                    .build_enum(
                        &ident_standart_not_null_origin_try_new_error_named_ucc,
                        &{
                            let gen_start_end_ts = |content_ts: &dyn quote::ToTokens| {
                                let (start_variant_ts, end_variant_ts) = {
                                    let gen_variant_ts = |start_or_end: &StartOrEnd| {
                                        let start_or_end_ucc = gen_start_or_end_ucc(start_or_end);
                                        quote! {
                                            #start_or_end_ucc {
                                                #[eo_error_occurence]
                                                #ErrorSc: #content_ts,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            }
                                        }
                                    };
                                    (gen_variant_ts(&StartOrEnd::Start), gen_variant_ts(&StartOrEnd::End))
                                };
                                quote! {
                                    #start_variant_ts,
                                    #end_variant_ts,
                                }
                            };
                            let content_ts: &dyn quote::ToTokens = match &postgres_type_initialization_try_new {
                                PostgresTypeInitializationTryNew::StdStringStringAsText => &std_string_string_as_text_try_new_error_named_variants_ts,
                                PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime | PostgresTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_variant_try_new_ts,
                                PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_ts,
                                PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => &quote! {
                                    #DateUcc {
                                        #[eo_error_occurence]
                                        #ErrorSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_ucc,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #TimeUcc {
                                        #[eo_error_occurence]
                                        #ErrorSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_ucc,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                },
                                PostgresTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &quote! {
                                    #DateNaiveUcc {
                                        #[eo_error_occurence]
                                        #ErrorSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_ucc,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                    #TimeUcc {
                                        #[eo_error_occurence]
                                        #ErrorSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_error_named_ucc,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                    },
                                },
                                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &gen_int_range_type_error_variants_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &gen_int_range_type_error_variants_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_start_end_ts(
                                    &sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_error_named_ucc
                                ),
                                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_start_end_ts(
                                    &sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_error_named_ucc
                                ),
                                PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_start_end_ts(
                                    &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_error_named_ucc
                                ),
                            };
                            quote!{{#content_ts}}
                        }
                    );
                quote!{
                    #allow_clippy_arbitrary_source_item_ordering_ts
                    #content_ts_d57d5de2
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_ts = if matches!(&is_standart_not_null, postgres_crud_macros_common::IsStandartNotNull::True)
                && let postgres_crud_macros_common::DeriveOrImpl::Impl(_) = &serde_deserialize_derive_or_impl
            {
                match &postgres_type_deserialize {
                    PostgresTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                    PostgresTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgres_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgres_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                        PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(_) => proc_macro2::TokenStream::new(),
                        PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgres_type_impl_try_new_for_deserialize) => {
                            let content_ts_026f2a24 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_thiserror_error()
                            .derive_error_occurence_lib_error_occurence()
                            .build_enum(
                                &ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc,
                                &{
                                    let content_ts: &dyn quote::ToTokens = match &postgres_type_impl_try_new_for_deserialize {
                                        PostgresTypeImplTryNewForDeserialize::StdStringStringAsText => &std_string_string_as_text_try_new_error_named_variants_ts,
                                        PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => &quote! {
                                            #InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #HourSc: #std_primitive_u32_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #MinSc: #std_primitive_u32_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #SecSc: #std_primitive_u32_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #MicroSc: #std_primitive_u32_ts,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            },
                                            #nanosecond_precision_is_not_supported_variant_try_new_ts
                                        },
                                        PostgresTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => &quote! {
                                            #InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #HourSc: #std_primitive_u8_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #MinuteSc: #std_primitive_u8_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #SecondSc: #std_primitive_u8_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #MicrosecondSc: #std_primitive_u32_ts,
                                                #[eo_to_std_string_string_serialize_deserialize]
                                                #ErrorSc: #std_string_string_ts,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                            },
                                            #nanosecond_precision_is_not_supported_variant_try_new_ts
                                        },
                                        PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_named_variants_ts,
                                        PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => &gen_int_range_type_error_variants_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                        PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &gen_int_range_type_error_variants_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                    };
                                    quote!{{#content_ts}}
                                }
                            );
                            quote!{
                                #allow_clippy_arbitrary_source_item_ordering_ts
                                #content_ts_026f2a24
                            }
                        }
                    },
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_ident_origin_ts = {
                let fn_new_or_try_new_ts = postgres_type_initialization_try_new_try_from_postgres_type.as_ref().map_or_else(
                |()| {
                    let content_ts = {
                        let content_ts = {
                            let gen_match_option_ts = |type_ts: &dyn quote::ToTokens| {
                                quote! {value.map(#type_ts::#NewSc)}
                            };
                            let gen_array_dimensions_initialization_ts = |type_ts: &dyn quote::ToTokens| match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => quote! {value.into_iter().map(#type_ts::#NewSc).collect()},
                                NotNullOrNullable::Nullable => gen_match_option_ts(&type_ts),
                            };
                            match &postgres_type_pattern {
                                PostgresTypePattern::Standart => match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => {
                                        postgres_type_range_try_from_postgres_type.as_ref().map_or_else(
                                            |()| quote! {#ValueSc},
                                            |value_6ed98462| gen_pg_range_conversion_ts(
                                                &ValueSc,
                                                &{
                                                    let range_postgres_type_ident_origin = SelfOriginUcc::from_display(&gen_ident_str(&PostgresType::from(value_6ed98462), not_null_or_nullable, postgres_type_pattern));
                                                    quote! {#range_postgres_type_ident_origin::#NewSc(value_af65ccce)}
                                                }
                                            )
                                        )
                                    }
                                    NotNullOrNullable::Nullable => gen_match_option_ts(&ident_standart_not_null_origin_ucc),
                                },
                                PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => gen_array_dimensions_initialization_ts(&{
                                    let (current_postgres_type_pattern, current_not_null_or_nullable): (&PostgresTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                                        NotNullOrNullable::NotNull => (&PostgresTypePattern::Standart, dimension1_not_null_or_nullable),
                                        NotNullOrNullable::Nullable => (postgres_type_pattern, &NotNullOrNullable::NotNull),
                                    };
                                    gen_current_ident_origin_non_wrapping(current_postgres_type_pattern, current_not_null_or_nullable)
                                }),
                            }
                        };
                        quote! {Self(#content_ts)}
                    };
                    match &postgres_type_pattern {
                        PostgresTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => macros_helpers::gen_const_new_ts(
                                &must_use_ts,
                                &value_ident_inner_type_ts,
                                &content_ts
                            ),
                            NotNullOrNullable::Nullable => macros_helpers::gen_new_ts(
                                &must_use_ts,
                                &value_ident_inner_type_ts,
                                &content_ts
                            ),
                        },
                        PostgresTypePattern::ArrayDimension1 { .. } => macros_helpers::gen_new_ts(
                            &must_use_ts,
                            &value_ident_inner_type_ts,
                            &content_ts
                        ),
                    }
                },
                |postgres_type_initialization_try_new| {
                    let content_ts = {
                        let gen_match_option_ts = |type_ts: &dyn quote::ToTokens| {
                            quote! {Ok(Self(match #ValueSc {
                                Some(value_989d943e) => Some(match #type_ts::#TryNewSc(value_989d943e) {
                                    Ok(value_ea2a4a8c) => value_ea2a4a8c,
                                    Err(er) => {
                                        return Err(er);
                                    },
                                }),
                                None => None
                            }))}
                        };
                        let gen_array_dimensions_initialization_ts = |type_ts: &dyn quote::ToTokens| match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote! {
                                Ok(Self({
                                    let mut acc_4ce2782a = Vec::new();
                                    for el_de177578 in #ValueSc {
                                        match #type_ts::#TryNewSc(el_de177578) {
                                            Ok(value_a763a416) => {
                                                acc_4ce2782a.push(value_a763a416);
                                            },
                                            Err(er) => {
                                                return Err(er);
                                            }
                                        }
                                    }
                                    acc_4ce2782a
                                }))
                            },
                            NotNullOrNullable::Nullable => gen_match_option_ts(&type_ts),
                        };
                        match &postgres_type_pattern {
                            PostgresTypePattern::Standart => match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => {
                                    let gen_int_range_check_ts = |int_range_type: &IntRangeType| {
                                        let max_value_ts = {
                                            let type_ts = int_range_type_to_range_inner_type_ts(int_range_type);
                                            quote! {#type_ts::MAX}
                                        };
                                        quote! {
                                            let max = #max_value_ts;
                                            let (#StartSc, #EndSc) = match (#ValueSc.#StartSc, #ValueSc.#EndSc) {
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Included(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    if #EndSc == max {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#StartSc), std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Excluded(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#StartSc), std::ops::Bound::Excluded(#EndSc))
                                                }
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Unbounded) => (std::ops::Bound::Included(#StartSc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Included(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    if #EndSc == max {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Excluded(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Excluded(#EndSc))
                                                }
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Unbounded) => (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Included(#EndSc)) => {
                                                    if #EndSc == max {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Unbounded, std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#EndSc)) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#EndSc)),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded),
                                            };
                                            Ok(Self(sqlx::postgres::types::PgRange { #StartSc, #EndSc }))
                                        }
                                    };
                                    let gen_ok_self_sqlx_postgres_types_pg_range_ts = |current_ident_ts: &dyn quote::ToTokens| quote! {
                                        Ok(Self(sqlx::postgres::types::PgRange {
                                            #StartSc: match #ValueSc.#StartSc {
                                                std::ops::Bound::Included(included_value) => match #current_ident_ts::#TryNewSc(included_value) {
                                                    Ok(value_a9c1f658) => std::ops::Bound::Included(value_a9c1f658.0),
                                                    Err(er) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#StartUcc {
                                                            #ErrorSc: er,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Excluded(excluded_value) => match #current_ident_ts::#TryNewSc(excluded_value) {
                                                    Ok(value_f0ff8036) => std::ops::Bound::Excluded(value_f0ff8036.0),
                                                    Err(er) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#StartUcc {
                                                            #ErrorSc: er,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                            },
                                            #EndSc: match #ValueSc.#EndSc {
                                                std::ops::Bound::Included(included_value) => match #current_ident_ts::#TryNewSc(included_value) {
                                                    Ok(value_80168e2b) => std::ops::Bound::Included(value_80168e2b.0),
                                                    Err(er) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#EndUcc {
                                                            #ErrorSc: er,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Excluded(excluded_value) => match #current_ident_ts::#TryNewSc(excluded_value) {
                                                    Ok(value_05f87b70) => std::ops::Bound::Excluded(value_05f87b70.0),
                                                    Err(er) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#EndUcc {
                                                            #ErrorSc: er,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                            },
                                        }))
                                    };
                                    match &postgres_type_initialization_try_new {
                                        PostgresTypeInitializationTryNew::StdStringStringAsText => quote! {
                                            if #ValueSc.find('\0').is_some() {
                                                Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#ContainsNullByteUcc {
                                                    #ValueSc,
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                })
                                            } else {
                                                Ok(Self(#ValueSc))
                                            }
                                        },
                                        PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => quote! {
                                            if <#inner_type_standart_not_null_ts as chrono::Timelike>::nanosecond(&#ValueSc).checked_rem(1000).expect("7c8b4e12-8509-41e4-8769-0fe10aafd930") != 0 {
                                                return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                    #ValueSc: #ValueSc.to_string(),
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                            Ok(Self(#ValueSc))
                                        },
                                        PostgresTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => quote! {
                                            if #ValueSc.nanosecond().checked_rem(1000).expect("ce47524f-de07-4a01-a4c5-78d39398b922") != 0 {
                                                return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                    #ValueSc: #ValueSc.to_string(),
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                });
                                            }
                                            Ok(Self(#ValueSc))
                                        },
                                        PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => quote! {
                                            let #EarliestSupportedDateSc = #inner_type_standart_not_null_ts::from_ymd_opt(-4713, 12, 31).expect("9f6241e5-a3ce-4ade-b33c-37432d4cafd3");
                                            if #ValueSc >= #EarliestSupportedDateSc {
                                                Ok(Self(#ValueSc))
                                            }
                                            else {
                                                Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#EarlierDateNotSupportedUcc {
                                                    #ValueSc: #ValueSc.to_string(),
                                                    #EarliestSupportedDateSc: #EarliestSupportedDateSc.to_string(),
                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                })
                                            }
                                        },
                                        PostgresTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                            let #DateSc = match #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc::#TryNewSc(
                                                #ValueSc.#DateSc()
                                            ) {
                                                Ok(value_9be8eddb) => value_9be8eddb,
                                                Err(er) => {
                                                    return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#DateUcc {
                                                        #ErrorSc: er,
                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                    });
                                                }
                                            };
                                            let #TimeSc = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc::#TryNewSc(
                                                #ValueSc.#TimeSc()
                                            ) {
                                                Ok(value_993484ce) => value_993484ce,
                                                Err(er) => {
                                                    return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#TimeUcc {
                                                        #ErrorSc: er,
                                                        code_occurence: error_occurence_lib::code_occurence!(),
                                                    });
                                                }
                                            };
                                            Ok(Self(#inner_type_standart_not_null_ts::#NewSc(#DateSc.0, #TimeSc.0)))
                                        },
                                        PostgresTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                                #DateNaiveSc.0,
                                                #TimeSc.0
                                            }));
                                            quote! {
                                                let #DateNaiveSc = match #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc::#TryNewSc(#ValueSc.date_naive()) {
                                                    Ok(value_158945ad) => value_158945ad,
                                                    Err(er) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#DateNaiveUcc {
                                                            #ErrorSc: er,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                };
                                                let #TimeSc = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc::#TryNewSc(#ValueSc.time()) {
                                                    Ok(value_c5af739c) => value_c5af739c,
                                                    Err(er) => {
                                                        return Err(#ident_standart_not_null_origin_try_new_error_named_ucc::#TimeUcc {
                                                            #ErrorSc: er,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        });
                                                    }
                                                };
                                                Ok(Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts))
                                            }
                                        }
                                        PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => gen_int_range_check_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                        PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => gen_int_range_check_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                        PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_ok_self_sqlx_postgres_types_pg_range_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc),
                                        PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_ok_self_sqlx_postgres_types_pg_range_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc),
                                        PostgresTypeInitializationTryNew::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_ok_self_sqlx_postgres_types_pg_range_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc),
                                    }
                                }
                                NotNullOrNullable::Nullable => gen_match_option_ts(&ident_standart_not_null_origin_ucc),
                            },
                            PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => gen_array_dimensions_initialization_ts(&{
                                let (current_postgres_type_pattern, current_not_null_or_nullable): (&PostgresTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => (&PostgresTypePattern::Standart, dimension1_not_null_or_nullable),
                                    NotNullOrNullable::Nullable => (postgres_type_pattern, &NotNullOrNullable::NotNull),
                                };
                                gen_current_ident_origin_non_wrapping(current_postgres_type_pattern, current_not_null_or_nullable)
                            }),
                        }
                    };
                    quote! {
                        pub fn #TryNewSc(#value_ident_inner_type_ts) -> Result<Self, #ident_standart_not_null_origin_try_new_error_named_ucc> {
                            #content_ts
                        }
                    }
                });
                let maybe_fn_new_or_try_new_for_deserialize_token = match &postgres_type_pattern {
                    PostgresTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => match &postgres_type_deserialize {
                            PostgresTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                            PostgresTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(postgres_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &postgres_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                                PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(postgres_type_impl_new_for_deserialize) => {
                                    let parameters_ts = {
                                        let gen_start_end_std_std_ops_bound_ts = |current_ident_ts: &dyn quote::ToTokens| {
                                            quote! {
                                                #StartSc: std::ops::Bound<#current_ident_ts>,
                                                #EndSc: std::ops::Bound<#current_ident_ts>
                                            }
                                        };
                                        match &postgres_type_impl_new_for_deserialize {
                                            PostgresTypeImplNewForDeserialize::SsqlxPostgresTypesPgIntervalAsInterval => quote! {
                                                #MonthsSc: #std_primitive_i32_ts,
                                                #DaysSc: #std_primitive_i32_ts,
                                                #MicrosecondsSc: #std_primitive_i64_ts,
                                            },
                                            PostgresTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                                #DateSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                                #TimeSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc
                                            },
                                            PostgresTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote! {
                                                #DateNaiveSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                                #TimeSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc,
                                            },
                                            PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_start_end_std_std_ops_bound_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc),
                                            PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_start_end_std_std_ops_bound_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc),
                                            PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_start_end_std_std_ops_bound_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc),
                                        }
                                    };
                                    let content_ts = {
                                        let self_sqlx_postgres_types_pg_range_ts = quote! {
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
                                        match &postgres_type_impl_new_for_deserialize {
                                            PostgresTypeImplNewForDeserialize::SsqlxPostgresTypesPgIntervalAsInterval => quote! {
                                                Self(sqlx::postgres::types::PgInterval {
                                                    #MonthsSc,
                                                    #DaysSc,
                                                    #MicrosecondsSc,
                                                })
                                            },
                                            PostgresTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                                Self(#inner_type_standart_not_null_ts::#NewSc(#DateSc.0, #TimeSc.0))
                                            },
                                            PostgresTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                                    #DateNaiveSc.0,
                                                    #TimeSc.0
                                                }));
                                                quote! {Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts)}
                                            }
                                            PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange | PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange | PostgresTypeImplNewForDeserialize::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                                                self_sqlx_postgres_types_pg_range_ts
                                            }
                                        }
                                    };
                                    quote! {
                                        const fn new_for_deserialize(#parameters_ts) -> Self {
                                            #content_ts
                                        }
                                    }
                                }
                                PostgresTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(postgres_type_impl_try_new_for_deserialize) => {
                                    let parameters_ts = {
                                        let gen_value_pg_range_int_type_ts = |int_range_type: &IntRangeType| {
                                            let type_ts = {
                                                let content_ts = int_range_type_to_range_inner_type_ts(int_range_type);
                                                quote! {std::ops::Bound<#content_ts>}
                                            };
                                            quote! {
                                                start_9a8ef454: #type_ts,
                                                end_a14eb2b9: #type_ts
                                            }
                                        };
                                        match &postgres_type_impl_try_new_for_deserialize {
                                            PostgresTypeImplTryNewForDeserialize::StdStringStringAsText | PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => {
                                                quote! {value_356f2a0b: #ident_inner_type_ts}
                                            }
                                            PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                quote! {
                                                    #HourSc: #std_primitive_u32_ts,
                                                    #MinSc: #std_primitive_u32_ts,
                                                    #SecSc: #std_primitive_u32_ts,
                                                    #MicroSc: #std_primitive_u32_ts
                                                }
                                            }
                                            PostgresTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                quote! {
                                                    #HourSc: #std_primitive_u8_ts,
                                                    #MinuteSc: #std_primitive_u8_ts,
                                                    #SecondSc: #std_primitive_u8_ts,
                                                    #MicrosecondSc: #std_primitive_u32_ts
                                                }
                                            }
                                            PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => gen_value_pg_range_int_type_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                                            PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => gen_value_pg_range_int_type_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                                        }
                                    };
                                    let content_ts = {
                                        let gen_self_match_try_new_ts = |current_parameters_ts: &dyn quote::ToTokens, match_error_variants_ts: &dyn quote::ToTokens| {
                                            quote! {
                                                match Self::#TryNewSc(#current_parameters_ts) {
                                                    Ok(value_b318fc86) => Ok(value_b318fc86),
                                                    Err(er) => match er {
                                                        #match_error_variants_ts
                                                    }
                                                }
                                            }
                                        };
                                        let try_new_convert_pg_range_int_content_ts = gen_self_match_try_new_ts(
                                            &quote! {sqlx::postgres::types::PgRange { #StartSc: start_9a8ef454, #EndSc: end_a14eb2b9 }},
                                            &quote! {
                                                #ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    code_occurence,
                                                }),
                                                #ident_standart_not_null_origin_try_new_error_named_ucc::#IncludedEndCannotBeMaxUcc {
                                                    #EndSc,
                                                    code_occurence,
                                                } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#IncludedEndCannotBeMaxUcc {
                                                    #EndSc,
                                                    code_occurence,
                                                }),
                                            },
                                        );
                                        match &postgres_type_impl_try_new_for_deserialize {
                                            PostgresTypeImplTryNewForDeserialize::StdStringStringAsText => {
                                                let variant_ts = quote! {
                                                    #ContainsNullByteUcc {
                                                        #ValueSc,
                                                        code_occurence,
                                                    }
                                                };
                                                gen_self_match_try_new_ts(
                                                    &quote!{value_356f2a0b},
                                                    &quote! {
                                                        #ident_standart_not_null_origin_try_new_error_named_ucc::#variant_ts => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#variant_ts),
                                                    },
                                                )
                                            }
                                            PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                quote! {
                                                    match #inner_type_standart_not_null_ts::from_hms_micro_opt(
                                                        #HourSc,
                                                        #MinSc,
                                                        #SecSc,
                                                        #MicroSc,
                                                    ) {
                                                        Some(value_b143b9e1) => {
                                                            if <#inner_type_standart_not_null_ts as chrono::Timelike>::nanosecond(&value_b143b9e1).checked_rem(1000).expect("c0514180-cfe0-44e2-9dcf-ab41df7e11f3") != 0 {
                                                                return Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                                    #ValueSc: value_b143b9e1.to_string(),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            Ok(Self(value_b143b9e1))
                                                        },
                                                        None => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                            #HourSc,
                                                            #MinSc,
                                                            #SecSc,
                                                            #MicroSc,
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    }
                                                }
                                            }
                                            PostgresTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                quote! {
                                                    match #inner_type_standart_not_null_ts::from_hms_micro(
                                                        #HourSc,
                                                        #MinuteSc,
                                                        #SecondSc,
                                                        #MicrosecondSc,
                                                    ) {
                                                        Ok(value_9932d535) => {
                                                            if value_9932d535.nanosecond().checked_rem(1000).expect("0def33ce-99c1-4969-9f1d-6923319ccc5b") != 0 {
                                                                return Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                                    #ValueSc: value_9932d535.to_string(),
                                                                    code_occurence: error_occurence_lib::code_occurence!(),
                                                                });
                                                            }
                                                            Ok(Self(value_9932d535))
                                                        },
                                                        Err(er) => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                            #HourSc,
                                                            #MinuteSc,
                                                            #SecondSc,
                                                            #MicrosecondSc,
                                                            #ErrorSc: er.to_string(),
                                                            code_occurence: error_occurence_lib::code_occurence!(),
                                                        })
                                                    }
                                                }
                                            }
                                            PostgresTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => gen_self_match_try_new_ts(
                                                &quote!{value_356f2a0b},
                                                &quote! {
                                                    #ident_standart_not_null_origin_try_new_error_named_ucc::#EarlierDateNotSupportedUcc {
                                                        #ValueSc,
                                                        #EarliestSupportedDateSc,
                                                        code_occurence,
                                                    } => Err(#ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc::#EarlierDateNotSupportedUcc {
                                                        #ValueSc,
                                                        #EarliestSupportedDateSc,
                                                        code_occurence,
                                                    }),
                                                },
                                            ),
                                            PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresTypeImplTryNewForDeserialize::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => try_new_convert_pg_range_int_content_ts,
                                        }
                                    };
                                    quote! {
                                        fn #TryNewForDeserializeSc(#parameters_ts) -> Result<Self, #ident_standart_not_null_origin_try_new_for_deserialize_error_named_ucc> {
                                            #content_ts
                                        }
                                    }
                                }
                            },
                        },
                        NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    },
                    PostgresTypePattern::ArrayDimension1 { .. } => proc_macro2::TokenStream::new(),
                };
                quote! {
                    #allow_clippy_arbitrary_source_item_ordering_ts
                    impl #ident_origin_ucc {
                        #fn_new_or_try_new_ts
                        #maybe_fn_new_or_try_new_for_deserialize_token
                    }
                }
            };
            let impl_std_convert_from_ident_origin_for_ident_inner_type_ts = {
                let content_ts = {
                    let value_dot_zero = quote! {#ValueSc.0};
                    let gen_match_ts = |
                        match_content_ts: &dyn quote::ToTokens,
                        some_content_ts: &dyn quote::ToTokens,
                        some_value_ts: &dyn quote::ToTokens,
                    | quote! {
                        #match_content_ts.map(|#some_value_ts|#some_value_ts.0#some_content_ts)
                    };
                    match &postgres_type_pattern {
                        PostgresTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => value_dot_zero,
                            NotNullOrNullable::Nullable => gen_match_ts(
                                &value_dot_zero,
                                &proc_macro2::TokenStream::new(),
                                &quote!{value_6bfd70fa}
                            ),
                        },
                        PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                            let el_dot_zero_ts = quote! {el_6910aab7.0};
                            let dimension1_ts = match &dimension1_not_null_or_nullable {
                                NotNullOrNullable::NotNull => el_dot_zero_ts,
                                NotNullOrNullable::Nullable => gen_match_ts(
                                    &el_dot_zero_ts,
                                    &proc_macro2::TokenStream::new(),
                                    &quote!{value_1b8cbd77}
                                ),
                            };
                            let into_iter_dimension1_ts = quote! {.into_iter().map(|el_6910aab7|#dimension1_ts).collect()};
                            match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => quote! {
                                    #value_dot_zero #into_iter_dimension1_ts
                                },
                                NotNullOrNullable::Nullable => gen_match_ts(
                                    &value_dot_zero,
                                    &into_iter_dimension1_ts,
                                    &quote!{value_38cfcd24}
                                ),
                            }
                        }
                    }
                };
                quote! {
                    impl From<#ident_origin_ucc> for #ident_inner_type_ts {
                        fn from(#ValueSc: #ident_origin_ucc) -> Self {
                            #content_ts
                        }
                    }
                }
            };
            let maybe_impl_is_string_empty_for_ident_origin_ts = if matches!(&is_standart_not_null, postgres_crud_macros_common::IsStandartNotNull::True) {
                match &not_null_or_nullable {
                    NotNullOrNullable::NotNull => match &postgres_type {
                        PostgresType::StdPrimitiveI16AsInt2
                        | PostgresType::StdPrimitiveI32AsInt4
                        | PostgresType::StdPrimitiveI64AsInt8
                        | PostgresType::StdPrimitiveF32AsFloat4
                        | PostgresType::StdPrimitiveF64AsFloat8
                        | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                        | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                        | PostgresType::StdPrimitiveBoolAsBool
                        | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                        | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                        | PostgresType::SqlxTypesTimeTimeAsTime
                        | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                        | PostgresType::SqlxTypesChronoNaiveDateAsDate
                        | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                        | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                        PostgresType::StdStringStringAsText => postgres_crud_macros_common::gen_impl_crate_is_string_empty_for_ident_content_ts(
                            &ident_origin_ucc,
                            &quote! {self.0.clone().is_empty()},
                        ),
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => postgres_crud_macros_common::gen_impl_crate_is_string_empty_for_ident_content_ts(
                            &ident_origin_ucc,
                            &quote! {self.0.to_string().is_empty()},
                        ),
                    },
                    NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let maybe_impl_serde_serialize_for_ident_standart_not_null_origin_ts = match &serde_serialize_derive_or_impl {
                postgres_crud_macros_common::DeriveOrImpl::Derive => &proc_macro2::TokenStream::new(),
                postgres_crud_macros_common::DeriveOrImpl::Impl(value) => value,
            };
            let maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_ts = match &serde_deserialize_derive_or_impl {
                postgres_crud_macros_common::DeriveOrImpl::Derive => &proc_macro2::TokenStream::new(),
                postgres_crud_macros_common::DeriveOrImpl::Impl(value) => value,
            };
            let impl_std_fmt_display_for_ident_origin_ts = macros_helpers::gen_impl_std_fmt_display_ts(&proc_macro2::TokenStream::new(), &ident_origin_ucc, &proc_macro2::TokenStream::new(), &quote! {write!(f, "{self:?}")});
            let impl_error_occurence_lib_to_std_string_string_for_ident_origin_ts = macros_helpers::gen_impl_error_occurence_lib_to_std_string_string_ts(&proc_macro2::TokenStream::new(), &ident_origin_ucc, &proc_macro2::TokenStream::new(), &quote! {self.to_string()});
            let impl_default_option_some_vec_one_el_for_ident_origin_ts = postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_ts(&ident_origin_ucc, &{
                let content_ts = match &postgres_type_pattern {
                    PostgresTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => {
                            let pg_range_int_default_initialization_ts = quote! {
                                sqlx::postgres::types::PgRange {
                                    start: std::ops::Bound::Included(#core_default_default_default_ts),
                                    end: std::ops::Bound::Excluded(#core_default_default_default_ts),
                                }
                            };
                            let gen_as_default_option_some_vec_one_el_call_ts = |current_ident_ts: &dyn quote::ToTokens| {
                                quote! {
                                    <
                                        #current_ident_ts
                                        as
                                        #import_path::DefaultOptionSomeVecOneEl
                                    >::default_option_some_vec_one_el()
                                }
                            };
                            let gen_sqlx_postgres_types_pg_range_default_option_some_vec_one_el_ts = |current_ident_ts: &dyn quote::ToTokens| {
                                let current_ident_as_default_option_some_vec_one_el_call_ts = gen_as_default_option_some_vec_one_el_call_ts(&current_ident_ts);
                                quote! {
                                    sqlx::postgres::types::PgRange {
                                        #StartSc: std::ops::Bound::Included(
                                            #current_ident_as_default_option_some_vec_one_el_call_ts.0
                                        ),
                                        #EndSc: std::ops::Bound::Excluded(
                                            #current_ident_as_default_option_some_vec_one_el_call_ts.0
                                        ),
                                    }
                                }
                            };
                            let sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_option_some_vec_one_el_call_ts = gen_as_default_option_some_vec_one_el_call_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc);
                            let sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_option_some_vec_one_el_call_ts = gen_as_default_option_some_vec_one_el_call_ts(&sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc);
                            let initialization_ts: &dyn quote::ToTokens = match &postgres_type {
                                PostgresType::StdPrimitiveI16AsInt2
                                | PostgresType::StdPrimitiveI32AsInt4
                                | PostgresType::StdPrimitiveI64AsInt8
                                | PostgresType::StdPrimitiveF32AsFloat4
                                | PostgresType::StdPrimitiveF64AsFloat8
                                | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                                | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                                | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                                | PostgresType::StdPrimitiveBoolAsBool
                                | PostgresType::StdStringStringAsText
                                | PostgresType::SqlxTypesChronoNaiveDateAsDate
                                | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                                | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr
                                | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => &quote! {#field_type_handle::default()},
                                PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => &quote! {#ident_inner_type_ts::default()},
                                PostgresType::SqlxPostgresTypesPgMoneyAsMoney => &quote! {#inner_type_standart_not_null_ts(#core_default_default_default_ts)},
                                PostgresType::StdVecVecStdPrimitiveU8AsBytea => &quote! {vec![#core_default_default_default_ts]},
                                PostgresType::SqlxTypesTimeTimeAsTime => &gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {0,0,0,0}),
                                PostgresType::SqlxPostgresTypesPgIntervalAsInterval => &quote! {#inner_type_standart_not_null_ts {
                                    #MonthsSc: #core_default_default_default_ts,
                                    #DaysSc: #core_default_default_default_ts,
                                    #MicrosecondsSc: #core_default_default_default_ts
                                }},
                                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                    #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_option_some_vec_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_option_some_vec_one_el_call_ts.0,
                                }),
                                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                    #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_option_some_vec_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_option_some_vec_one_el_call_ts.0,
                                })),
                                PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => &quote! {
                                    sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default_ts).expect("9e9c9b57-1a39-4674-a112-5e009fcbab0f"))
                                },
                                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => &pg_range_int_default_initialization_ts,
                                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_sqlx_postgres_types_pg_range_default_option_some_vec_one_el_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc),
                                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_sqlx_postgres_types_pg_range_default_option_some_vec_one_el_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc),
                                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_sqlx_postgres_types_pg_range_default_option_some_vec_one_el_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc),
                            };
                            quote! {#initialization_ts}
                        }
                        NotNullOrNullable::Nullable => quote! {Some(#postgres_crud_common_default_option_some_vec_one_el_call_ts)},
                    },
                    PostgresTypePattern::ArrayDimension1 { .. } => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {vec![#postgres_crud_common_default_option_some_vec_one_el_call_ts]},
                        NotNullOrNullable::Nullable => quote! {Some(#postgres_crud_common_default_option_some_vec_one_el_call_ts)},
                    },
                };
                quote! {Self(#content_ts)}
            });
            let impl_sqlx_type_sqlx_postgres_for_ident_origin_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_origin_ucc, &field_type_handle);
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_origin_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_postgres_for_ident_origin_ts = postgres_crud_macros_common::gen_impl_sqlx_decode_sqlx_postgres_for_ident_ts(&ident_origin_ucc, &field_type_handle, &{
                let scopes_value_ts = quote! {(value_147c3532)};
                let ok_self_scopes_value_ts = quote! {Ok(Self #scopes_value_ts)};
                match &postgres_type_pattern {
                    PostgresTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => match &postgres_type {
                            PostgresType::StdPrimitiveI16AsInt2
                            | PostgresType::StdPrimitiveI32AsInt4
                            | PostgresType::StdPrimitiveI64AsInt8
                            | PostgresType::StdPrimitiveF32AsFloat4
                            | PostgresType::StdPrimitiveF64AsFloat8
                            | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                            | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                            | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                            | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                            | PostgresType::StdPrimitiveBoolAsBool
                            | PostgresType::StdStringStringAsText
                            | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                            | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                            | PostgresType::SqlxTypesTimeTimeAsTime
                            | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                            | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                            | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                            | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres
                            | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                            | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                            | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr
                            | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                            | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                            | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_value_ts,
                            PostgresType::SqlxTypesChronoNaiveDateAsDate | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => quote! {
                                match Self::#TryNewSc #scopes_value_ts {
                                    Ok(value_93eb5329) => Ok(value_93eb5329),
                                    Err(er) => Err(Box::#NewSc(er)),
                                }
                            },
                        },
                        NotNullOrNullable::Nullable => ok_self_scopes_value_ts,
                    },
                    PostgresTypePattern::ArrayDimension1 { .. } => ok_self_scopes_value_ts,
                }
            });
            let impl_sqlx_postgres_pg_has_array_type_for_ident_origin_ts = {
                quote! {
                    impl sqlx::postgres::PgHasArrayType for #ident_origin_ucc {
                        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                            <#inner_type_standart_not_null_ts as sqlx::postgres::PgHasArrayType>::array_type_info()
                        }
                    }
                }
            };
            let maybe_impl_std_convert_from_ident_read_for_ident_origin_ts = match &is_not_null_standart_can_be_primary_key {
                IsNotNullStandartCanBePrimaryKey::False => proc_macro2::TokenStream::new(),
                IsNotNullStandartCanBePrimaryKey::True => macros_helpers::gen_impl_std_convert_from_ts(&ident_standart_not_null_read_ucc, &ident_origin_ucc, &{
                    let ident_standart_not_null_as_crate_postgres_type_ts = gen_as_postgres_type_ts(&ident_standart_not_null_ucc);
                    quote! {Self::#NewSc(#ident_standart_not_null_as_crate_postgres_type_ts::into_inner(#ValueSc))}
                }),
            };
            quote! {
                #ident_origin_ts
                #maybe_pub_enum_ident_standart_not_null_origin_try_new_error_named_ts
                #maybe_pub_enum_ident_standart_not_null_origin_try_new_for_deserialize_error_named_ts
                #impl_ident_origin_ts
                #impl_std_convert_from_ident_origin_for_ident_inner_type_ts

                #maybe_impl_is_string_empty_for_ident_origin_ts
                #maybe_impl_serde_serialize_for_ident_standart_not_null_origin_ts
                #maybe_impl_serde_deserialize_for_ident_standart_not_null_origin_ts
                #impl_std_fmt_display_for_ident_origin_ts
                #impl_error_occurence_lib_to_std_string_string_for_ident_origin_ts
                #impl_default_option_some_vec_one_el_for_ident_origin_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_origin_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_origin_ts
                #impl_sqlx_decode_sqlx_postgres_for_ident_origin_ts
                #impl_sqlx_postgres_pg_has_array_type_for_ident_origin_ts
                #maybe_impl_std_convert_from_ident_read_for_ident_origin_ts
            }
        };
        let gen_pub_struct_tokens_ts = |current_ident_ts: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens, derive_default: macros_helpers::DeriveDefault| {
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
                    &current_ident_ts,
                    &content_ts
                )
        };
        let ident_origin_struct_content_ts = quote!{(#ident_origin_ucc);};
        let ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident);
        let ident_table_type_declaration_ts = {
            let ident_table_type_declaration_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_partial_ord_if(match &is_standart_not_null {
                    postgres_crud_macros_common::IsStandartNotNull::False => macros_helpers::DerivePartialOrd::False,
                    postgres_crud_macros_common::IsStandartNotNull::True => match &postgres_type {
                        PostgresType::StdPrimitiveI16AsInt2
                        | PostgresType::StdPrimitiveI32AsInt4
                        | PostgresType::StdPrimitiveI64AsInt8
                        | PostgresType::StdPrimitiveF32AsFloat4
                        | PostgresType::StdPrimitiveF64AsFloat8
                        | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres
                        | PostgresType::StdPrimitiveBoolAsBool
                        | PostgresType::StdStringStringAsText
                        | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                        | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                        | PostgresType::SqlxTypesTimeTimeAsTime
                        | PostgresType::SqlxTypesChronoNaiveDateAsDate
                        | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => macros_helpers::DerivePartialOrd::True,
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                        | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                        | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                        | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                        | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::DerivePartialOrd::False,
                    },
                })
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_table_type_declaration_ucc,
                    &ident_origin_struct_content_ts
                );
            let impl_ident_table_type_declaration_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_table_type_declaration_ucc);
            let impl_default_option_some_vec_one_el_for_ident_table_type_declaration_ts =
                postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_ts(&ident_table_type_declaration_ucc, &quote! {Self(#postgres_crud_common_default_option_some_vec_one_el_call_ts)});
            let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_table_type_declaration_ucc, &ident_origin_ucc);
            let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_table_type_declaration_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_postgres_for_ident_table_type_declaration_ts = postgres_crud_macros_common::gen_impl_sqlx_decode_sqlx_postgres_for_ident_ts(&ident_table_type_declaration_ucc, &ident_origin_ucc, &quote! {Ok(Self(value_147c3532))});
            //todo rewrite as dependency of PostgresType trait?
            let impl_postgres_type_equal_operator_for_ident_table_type_declaration_ts = postgres_crud_macros_common::impl_postgres_type_equal_operator_for_ident_ts(
                &import_path,
                &ident_table_type_declaration_ucc,
                //todo
                &{
                    let equal_ts = postgres_crud_macros_common::EqualOperatorHandle::Equal.to_tokens_path(&import_path);
                    let is_null_ts = postgres_crud_macros_common::EqualOperatorHandle::IsNull.to_tokens_path(&import_path);
                    match &postgres_type_pattern {
                        PostgresTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => equal_ts,
                            NotNullOrNullable::Nullable => quote! {
                                if self.0.0.is_some() {
                                    #equal_ts
                                }
                                else {
                                    #is_null_ts
                                }
                            },
                        },
                        PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => match &dimension1_not_null_or_nullable {
                                NotNullOrNullable::NotNull => equal_ts,
                                NotNullOrNullable::Nullable => {
                                    //todo thats not actually usefull coz nullable array comparison has different logic. need to refactor EqualOperatorHandle enum
                                    equal_ts
                                }
                            },
                            NotNullOrNullable::Nullable => quote! {
                                if self.0.0.is_some() {
                                    #equal_ts
                                }
                                else {
                                    #is_null_ts
                                }
                            },
                        },
                    }
                },
            );
            quote! {
                #ident_table_type_declaration_ts
                #impl_ident_table_type_declaration_ts
                #impl_default_option_some_vec_one_el_for_ident_table_type_declaration_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_ts
                #impl_sqlx_decode_sqlx_postgres_for_ident_table_type_declaration_ts
                #impl_postgres_type_equal_operator_for_ident_table_type_declaration_ts
            }
        };
        let ident_standart_not_null_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
        let ident_create_ts = {
            let ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy_if(derive_copy)
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_create_ucc,
                        &ident_origin_struct_content_ts
                    ),
                CanBePrimaryKey::True => gen_pub_struct_tokens_ts(&ident_create_ucc, &quote! {(());}, macros_helpers::DeriveDefault::False),
            };
            let maybe_impl_ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => gen_pub_const_new_or_pub_try_new_ts(&ident_create_ucc),
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
            };
            let impl_default_option_some_vec_one_el_for_ident_create_ts = postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_ts(&ident_create_ucc, &{
                let content_ts: &dyn quote::ToTokens = match &can_be_primary_key {
                    CanBePrimaryKey::False => &postgres_crud_common_default_option_some_vec_one_el_call_ts,
                    CanBePrimaryKey::True => &quote! {()},
                };
                quote! {Self(#content_ts)}
            });
            let maybe_impl_sqlx_encode_sqlx_postgres_for_ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_create_ucc, &quote! {#SelfSc.0}),
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
            };
            let maybe_impl_sqlx_type_sqlx_postgres_for_ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_create_ucc, &ident_origin_ucc),
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
            };
            quote! {
                #ident_create_ts
                #maybe_impl_ident_create_ts
                #impl_default_option_some_vec_one_el_for_ident_create_ts
                #maybe_impl_sqlx_encode_sqlx_postgres_for_ident_create_ts
                #maybe_impl_sqlx_type_sqlx_postgres_for_ident_create_ts
            }
        };
        let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
        let ident_select_ts = {
            let pub_struct_ident_select_ts = gen_pub_struct_tokens_ts(
                &ident_select_ucc,
                &match &postgres_type_pattern {
                    PostgresTypePattern::Standart => quote! {;},
                    PostgresTypePattern::ArrayDimension1 { .. } => {
                        let mut arguments_ts = Vec::new();
                        for el_9f432ae3 in 1..=array_dimensions_number {
                            let dimension_number_pagination_ts = format!("dimension{el_9f432ae3}_pagination").parse::<proc_macro2::TokenStream>().expect("af86f2d1-b00d-49ab-9ced-97a488d9dc5f");
                            arguments_ts.push(quote! {
                                #dimension_number_pagination_ts: postgres_types_common::PaginationStartsWithOne
                            });
                        }
                        quote! {{#(#arguments_ts),*}}
                    }
                },
                macros_helpers::DeriveDefault::True,
            );
            let (impl_default_option_some_vec_one_el_for_ident_select_ts, impl_default_option_some_vec_one_el_max_page_size_for_ident_select_ts) = {
                let gen_default_content_ts = |default_some_one_or_default_some_one_with_max_page_size: &postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| match &postgres_type_pattern {
                    PostgresTypePattern::Standart => quote! {Self},
                    PostgresTypePattern::ArrayDimension1 { .. } => {
                        let content_ts: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgres_crud_common_default_option_some_vec_one_el_call_ts,
                            postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgres_crud_common_default_option_some_vec_one_el_max_page_size_call_ts,
                        };
                        let mut arguments_ts = Vec::new();
                        for el_a227c2ba in 1..=array_dimensions_number {
                            let dimension_number_pagination_ts = format!("dimension{el_a227c2ba}_pagination").parse::<proc_macro2::TokenStream>().expect("e5250a98-89d6-4a58-90ea-39b04a708c1c");
                            arguments_ts.push(quote! {
                                #dimension_number_pagination_ts: #content_ts
                            });
                        }
                        quote! {Self {#(#arguments_ts),*}}
                    }
                };
                (
                    postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_ts(&ident_select_ucc, &gen_default_content_ts(&postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)),
                    postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_max_page_size_ts(&ident_select_ucc, &gen_default_content_ts(&postgres_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)),
                )
            };
            quote! {
                #pub_struct_ident_select_ts
                #impl_default_option_some_vec_one_el_for_ident_select_ts
                #impl_default_option_some_vec_one_el_max_page_size_for_ident_select_ts
            }
        };
        let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
        let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
        let ident_where_ts = postgres_crud_macros_common::gen_postgres_type_where_ts(
            &allow_clippy_arbitrary_source_item_ordering_ts,
            &{
                let common_postgres_type_filters = vec![postgres_crud_macros_common::PostgresTypeFilter::Equal { ident: quote! {#ident_table_type_declaration_ucc} }];
                match &postgres_type_pattern {
                    PostgresTypePattern::Standart => {
                        let greater_than = postgres_crud_macros_common::PostgresTypeFilter::GreaterThan {
                            ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                        };
                        let between = postgres_crud_macros_common::PostgresTypeFilter::Between {
                            ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                        };
                        let in_handle = postgres_crud_macros_common::PostgresTypeFilter::In { ident: quote! {#ident_table_type_declaration_ucc} };
                        let regular_expression = postgres_crud_macros_common::PostgresTypeFilter::RegularExpression;
                        let equal_to_encoded_string_representation = postgres_crud_macros_common::PostgresTypeFilter::EqualToEncodedStringRepresentation;
                        let current_date = postgres_crud_macros_common::PostgresTypeFilter::CurrentDate;
                        let greater_than_current_date = postgres_crud_macros_common::PostgresTypeFilter::GreaterThanCurrentDate;
                        let current_time = postgres_crud_macros_common::PostgresTypeFilter::CurrentTime;
                        let greater_than_current_time = postgres_crud_macros_common::PostgresTypeFilter::GreaterThanCurrentTime;
                        let current_timestamp = postgres_crud_macros_common::PostgresTypeFilter::CurrentTimestamp;
                        let greater_than_current_timestamp = postgres_crud_macros_common::PostgresTypeFilter::GreaterThanCurrentTimestamp;
                        let before = postgres_crud_macros_common::PostgresTypeFilter::Before {
                            ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                        };
                        // let bit_vec_position_equal = postgres_crud_macros_common::PostgresTypeFilter::BitVecPositionEqual;
                        let common_standart_postgres_type_filters = { common_postgres_type_filters };
                        let common_standart_postgres_type_number_filters = {
                            let mut vec = common_standart_postgres_type_filters.clone();
                            vec.push(greater_than.clone());
                            vec.push(between.clone());
                            vec.push(in_handle.clone());
                            vec
                        };
                        let (
                            where_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                            where_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ) = {
                            let ranges_common_filter_vec = {
                                let mut vec = common_standart_postgres_type_filters.clone();
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::FindRangesWithinGivenRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::FindRangesThatFullyContainTheGivenRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::StrictlyToLeftOfRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::StrictlyToRightOfRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::IncludedLowerBound {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::ExcludedUpperBound {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::GreaterThanIncludedLowerBound {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::GreaterThanExcludedUpperBound {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::OverlapWithRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::AdjacentWithRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::RangeLength);
                                vec
                            };
                            (ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec)
                        };
                        match &postgres_type {
                            PostgresType::StdPrimitiveI16AsInt2
                            | PostgresType::StdPrimitiveI32AsInt4
                            | PostgresType::StdPrimitiveI64AsInt8
                            | PostgresType::StdPrimitiveF32AsFloat4
                            | PostgresType::StdPrimitiveF64AsFloat8
                            | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                            | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                            | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => common_standart_postgres_type_number_filters,
                            PostgresType::SqlxPostgresTypesPgMoneyAsMoney => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(in_handle);
                                vec
                            }
                            PostgresType::StdVecVecStdPrimitiveU8AsBytea => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(equal_to_encoded_string_representation);
                                vec
                            }
                            PostgresType::SqlxTypesChronoNaiveTimeAsTime | PostgresType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(current_time);
                                vec.push(greater_than_current_time);
                                vec
                            }
                            PostgresType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(current_date);
                                vec.push(greater_than_current_date);
                                vec
                            }
                            PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(current_timestamp);
                                vec.push(greater_than_current_timestamp);
                                vec
                            }
                            PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(before);
                                vec.push(between);
                                vec
                            }
                            PostgresType::StdStringStringAsText | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(regular_expression);
                                vec
                            }
                            PostgresType::StdPrimitiveBoolAsBool | PostgresType::SqlxPostgresTypesPgIntervalAsInterval | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => common_standart_postgres_type_filters,
                            PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = common_standart_postgres_type_filters;
                                vec.push(greater_than);
                                vec.push(regular_expression);
                                vec
                            }
                            PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                            PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        }
                    }
                    PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                        let ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_ucc = {
                            let value = SelfTableTypeDeclarationUcc::from_tokens(&match &postgres_type.can_be_nullable() {
                                CanBeNullable::False => quote! {#ident_standart_not_null_ucc},
                                CanBeNullable::True => {
                                    let value = gen_ident_ts(postgres_type, not_null_or_nullable, &PostgresTypePattern::Standart);
                                    quote! {#value}
                                }
                            });
                            quote! {#value}
                        };
                        let dimension_one_greater_than = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThan {
                            ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                        };
                        let dimension_one_between = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneBetween {
                            ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                        };
                        let dimension_one_in_handle = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneIn {
                            ident: ident_standart_not_null_or_nullable_if_can_be_nullable_table_type_declaration_ucc,
                        };
                        let dimension_one_regular_expression = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneRegularExpression;
                        let dimension_one_current_date = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneCurrentDate;
                        let dimension_one_greater_than_current_date = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanCurrentDate;
                        let dimension_one_current_time = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneCurrentTime;
                        let dimension_one_greater_than_current_time = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanCurrentTime;
                        let dimension_one_current_timestamp = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneCurrentTimestamp;
                        let dimension_one_greater_than_current_timestamp = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanCurrentTimestamp;
                        let dimension_one_before = postgres_crud_macros_common::PostgresTypeFilter::DimensionOneBefore {
                            ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                        };
                        let common_array_dimension1_postgres_type_filters = {
                            let mut vec = common_postgres_type_filters;
                            vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneEqual {
                                ident: {
                                    let value = SelfTableTypeDeclarationUcc::from_tokens(&match &dimension1_not_null_or_nullable {
                                        NotNullOrNullable::NotNull => &ident_standart_not_null_ucc,
                                        NotNullOrNullable::Nullable => &ident_standart_nullable_ucc,
                                    });
                                    quote! {#value}
                                },
                            });
                            vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneLengthEqual);
                            vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneLengthGreaterThan);
                            vec
                        };
                        let common_array_dimension1_postgres_type_number_filters = {
                            let mut vec = common_array_dimension1_postgres_type_filters.clone();
                            vec.push(dimension_one_greater_than.clone());
                            vec.push(dimension_one_between.clone());
                            vec.push(dimension_one_in_handle.clone());
                            vec
                        };
                        let (
                            where_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                            where_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ) = {
                            let gen_where_sqlx_postgres_types_pg_range_filter_ts = |postgres_type_range: PostgresTypeRange| {
                                let postgres_type_from_postgres_type_range = PostgresType::from(&postgres_type_range);
                                let range_el_ident_standart_not_null_ts = gen_ident_standart_not_null_ts(&postgres_type_from_postgres_type_range);
                                let mut vec = common_array_dimension1_postgres_type_filters.clone();
                                let range_el_ident_standart_not_null_as_crate_postgres_type_read_ts = {
                                    let range_el_ident_standart_not_null_as_crate_postgres_type_ts = gen_as_postgres_type_ts(&range_el_ident_standart_not_null_ts);
                                    quote! {#range_el_ident_standart_not_null_as_crate_postgres_type_ts::Read}
                                };
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneFindRangesWithinGivenRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneFindRangesThatFullyContainTheGivenRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneStrictlyToLeftOfRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneStrictlyToRightOfRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneIncludedLowerBound {
                                    ident: range_el_ident_standart_not_null_as_crate_postgres_type_read_ts.clone(),
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneExcludedUpperBound {
                                    ident: range_el_ident_standart_not_null_as_crate_postgres_type_read_ts.clone(),
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanIncludedLowerBound {
                                    ident: range_el_ident_standart_not_null_as_crate_postgres_type_read_ts.clone(),
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneGreaterThanExcludedUpperBound {
                                    ident: range_el_ident_standart_not_null_as_crate_postgres_type_read_ts,
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneOverlapWithRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneAdjacentWithRange {
                                    ident: quote! {#ident_standart_not_null_table_type_declaration_ucc},
                                });
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneRangeLength);
                                vec
                            };
                            (
                                gen_where_sqlx_postgres_types_pg_range_filter_ts(PostgresTypeRange::StdPrimitiveI32AsInt4),
                                gen_where_sqlx_postgres_types_pg_range_filter_ts(PostgresTypeRange::StdPrimitiveI64AsInt8),
                                gen_where_sqlx_postgres_types_pg_range_filter_ts(PostgresTypeRange::SqlxTypesChronoNaiveDateAsDate),
                                gen_where_sqlx_postgres_types_pg_range_filter_ts(PostgresTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                                gen_where_sqlx_postgres_types_pg_range_filter_ts(PostgresTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                            )
                        };
                        match &postgres_type {
                            PostgresType::StdPrimitiveI16AsInt2
                            | PostgresType::StdPrimitiveI32AsInt4
                            | PostgresType::StdPrimitiveI64AsInt8
                            | PostgresType::StdPrimitiveF32AsFloat4
                            | PostgresType::StdPrimitiveF64AsFloat8
                            | PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres
                            | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres
                            | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => common_array_dimension1_postgres_type_number_filters,
                            PostgresType::SqlxPostgresTypesPgMoneyAsMoney => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_in_handle);
                                vec
                            }
                            PostgresType::StdVecVecStdPrimitiveU8AsBytea => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(postgres_crud_macros_common::PostgresTypeFilter::DimensionOneEqualToEncodedStringRepresentation);
                                vec
                            }
                            PostgresType::SqlxTypesChronoNaiveTimeAsTime | PostgresType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_between);
                                vec.push(dimension_one_current_time);
                                vec.push(dimension_one_greater_than_current_time);
                                vec
                            }
                            PostgresType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_between);
                                vec.push(dimension_one_current_date);
                                vec.push(dimension_one_greater_than_current_date);
                                vec
                            }
                            PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_between);
                                vec.push(dimension_one_current_timestamp);
                                vec.push(dimension_one_greater_than_current_timestamp);
                                vec
                            }
                            PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_before);
                                vec.push(dimension_one_between);
                                vec
                            }
                            PostgresType::StdStringStringAsText | PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_regular_expression);
                                vec
                            }
                            PostgresType::StdPrimitiveBoolAsBool | PostgresType::SqlxPostgresTypesPgIntervalAsInterval | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => common_array_dimension1_postgres_type_filters,
                            PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = common_array_dimension1_postgres_type_filters;
                                vec.push(dimension_one_greater_than);
                                vec.push(dimension_one_regular_expression);
                                vec
                            }
                            PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => where_sqlx_postgres_types_pg_range_std_primitive_i32_ts,
                            PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => where_sqlx_postgres_types_pg_range_std_primitive_i64_ts,
                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        }
                    }
                }
            }
            .iter()
            .map(|el_dde282a0| {
                let el_dde282a0_handle: &dyn postgres_crud_macros_common::PostgresFilter = el_dde282a0;
                el_dde282a0_handle
            })
            .collect(),
            &ident,
            &postgres_crud_macros_common::ShouldDeriveUtoipaToSchema::False,
            &postgres_crud_macros_common::ShouldDeriveSchemarsJsonSchema::False,
            &postgres_crud_macros_common::IsQueryBindMutable::False,
        );
        let ident_read_ts = {
            let ident_read_ts = {
                let (
                    derive_eq,
                    derive_partial_ord,
                    derive_ord
                ) = match &is_not_null_standart_can_be_primary_key {
                    IsNotNullStandartCanBePrimaryKey::False => (
                        macros_helpers::DeriveEq::False,
                        macros_helpers::DerivePartialOrd::False,
                        macros_helpers::DeriveOrd::False
                    ),
                    IsNotNullStandartCanBePrimaryKey::True => (
                        macros_helpers::DeriveEq::True,
                        macros_helpers::DerivePartialOrd::True,
                        macros_helpers::DeriveOrd::True
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
                        &ident_read_ucc,
                        &ident_origin_struct_content_ts
                    )
            };
            let impl_ident_read_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_read_ucc);
            let impl_error_occurence_lib_to_std_string_string_for_ident_read_ts = macros_helpers::gen_impl_error_occurence_lib_to_std_string_string_ts(&proc_macro2::TokenStream::new(), &ident_read_ucc, &proc_macro2::TokenStream::new(), &quote! {self.0.to_string()});
            let impl_crate_default_option_some_vec_one_el_for_ident_read_ts =
                postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_ts(&ident_read_ucc, &quote! {Self(#postgres_crud_common_default_option_some_vec_one_el_call_ts)});
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_read_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_postgres_for_ident_read_ts = postgres_crud_macros_common::gen_impl_sqlx_decode_sqlx_postgres_for_ident_ts(
                &ident_read_ucc,
                &ident_origin_ucc,
                &quote! {Ok(Self(value_147c3532))}
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_read_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_read_ucc, &ident_origin_ucc);
            let maybe_impl_postgres_type_where_filter_for_ident_read_if_can_be_primary_key_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
                postgres_crud_macros_common::impl_postgres_type_where_filter_for_ident_ts(
                    &quote! {<'lifetime>},
                    &ident_standart_not_null_read_ucc,
                    &proc_macro2::TokenStream::new(),
                    &postgres_crud_macros_common::IncrementParameterUnderscore::False,
                    &postgres_crud_macros_common::ColumnParameterUnderscore::False,
                    &postgres_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::True,
                    &quote! {
                        match #import_path::increment_checked_add_one_returning_increment(#IncrementSc) {
                            Ok(value_8da76391) => Ok(format!("({column} = ${value_8da76391})")),
                            Err(er) => Err(er)
                        }
                    },
                    &postgres_crud_macros_common::IsQueryBindMutable::True,
                    &gen_typical_query_bind_ts(&SelfSc),
                    &import_path,
                )
            } else {
                proc_macro2::TokenStream::new()
            };
            quote! {
                #ident_read_ts
                #impl_ident_read_ts
                #impl_error_occurence_lib_to_std_string_string_for_ident_read_ts
                #impl_crate_default_option_some_vec_one_el_for_ident_read_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_origin_ts
                #impl_sqlx_decode_sqlx_postgres_for_ident_read_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_read_ts
                #maybe_impl_postgres_type_where_filter_for_ident_read_if_can_be_primary_key_ts
            }
        };
        let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
        let ident_read_only_ids_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
            let ident_read_only_ids_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_read_only_ids_ucc,
                    &quote!{(#ident_read_ucc);},
                );
            let impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_ts = postgres_crud_macros_common::gen_impl_sqlx_decode_sqlx_postgres_for_ident_ts(
                &ident_read_only_ids_ucc,
                &ident_read_ucc,
                &quote! {Ok(Self(value_147c3532))}
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_read_only_ids_ucc, &ident_read_ucc);
            quote! {
                #ident_read_only_ids_ts
                #impl_sqlx_decode_sqlx_postgres_for_ident_read_only_ids_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_read_only_ids_ts
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let ident_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident);
        let ident_read_inner_ts = quote! {
            pub type #ident_read_inner_ucc = #ident_inner_type_ts;
        };
        let ident_update_ts = {
            let ident_update_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_update_ucc,
                    &ident_origin_struct_content_ts
                );
            let impl_ident_update_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_update_ucc);
            let impl_default_option_some_vec_one_el_for_ident_update_ts =
                postgres_crud_macros_common::gen_impl_postgres_crud_common_default_option_some_vec_one_el_ts(&ident_update_ucc, &quote! {Self(#postgres_crud_common_default_option_some_vec_one_el_call_ts)});
            let impl_error_occurence_lib_to_std_string_string_for_ident_update_ts = macros_helpers::gen_impl_error_occurence_lib_to_std_string_string_ts(&proc_macro2::TokenStream::new(), &ident_update_ucc, &proc_macro2::TokenStream::new(), &quote! {self.0.#ToStdStringStringSc()});
            quote! {
                #ident_update_ts
                #impl_ident_update_ts
                #impl_default_option_some_vec_one_el_for_ident_update_ts
                #impl_error_occurence_lib_to_std_string_string_for_ident_update_ts
            }
        };
        let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
        let ident_update_for_query_ts = {
            let ident_update_for_query_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_update_for_query_ucc,
                    &ident_origin_struct_content_ts
                );
            let impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_ts = postgres_crud_macros_common::gen_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_update_for_query_ucc, &ident_origin_ucc);
            let impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_ts = postgres_crud_macros_common::gen_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_update_for_query_ucc, &quote! {#SelfSc.0});
            let impl_std_convert_from_ident_update_for_ident_update_for_query_ts = macros_helpers::gen_impl_std_convert_from_ts(&ident_update_ucc, &ident_update_for_query_ucc, &quote! {Self(#ValueSc.0)});
            quote! {
                #ident_update_for_query_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_ts
                #impl_std_convert_from_ident_update_for_ident_update_for_query_ts
            }
        };
        let impl_postgres_type_for_ident_ts = {
            let gen_ok_std_string_string_from_tokens_ts = |content_ts: &dyn quote::ToTokens| {
                quote! {Ok(#std_string_string_ts::from(#content_ts))}
            };
            let ok_std_string_string_from_default_ts = gen_ok_std_string_string_from_tokens_ts(&quote! {"default"});
            let ok_std_string_string_from_uuid_generate_v4_ts = gen_ok_std_string_string_from_tokens_ts(&quote! {"uuid_generate_v4()"});
            let typical_query_part_ts = {
                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts(
                    &quote! {acc_c7df00f5, "${value_ba581e0f}"},
                    &postgres_crud_macros_common::gen_return_err_query_part_error_named_write_into_buffer_ts(import_path)
                );
                quote! {
                    let mut acc_c7df00f5 = String::default();
                    match #import_path::increment_checked_add_one_returning_increment(#IncrementSc) {
                        Ok(value_ba581e0f) => {
                            #if_write_is_err_ts
                        },
                        Err(er) => {
                            return Err(er);
                        }
                    }
                    Ok(acc_c7df00f5)
                }
            };
            let ok_query_ts = quote! {Ok(#QuerySc)};
            let (query_part_create_ts, bind_value_to_query_create_ts): Handle<'_> = {
                let typical: Handle<'_> = { (&typical_query_part_ts, &typical_query_bind_ts) };
                let default_initialized_by_postgres: Handle<'_> = (&ok_std_string_string_from_default_ts, &ok_query_ts);
                match &postgres_type {
                    PostgresType::StdPrimitiveI16AsInt2
                    | PostgresType::StdPrimitiveI32AsInt4
                    | PostgresType::StdPrimitiveI64AsInt8
                    | PostgresType::StdPrimitiveF32AsFloat4
                    | PostgresType::StdPrimitiveF64AsFloat8
                    | PostgresType::SqlxPostgresTypesPgMoneyAsMoney
                    | PostgresType::StdPrimitiveBoolAsBool
                    | PostgresType::StdStringStringAsText
                    | PostgresType::StdVecVecStdPrimitiveU8AsBytea
                    | PostgresType::SqlxTypesChronoNaiveTimeAsTime
                    | PostgresType::SqlxTypesTimeTimeAsTime
                    | PostgresType::SqlxPostgresTypesPgIntervalAsInterval
                    | PostgresType::SqlxTypesChronoNaiveDateAsDate
                    | PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                    | PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                    | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient
                    | PostgresType::SqlxTypesIpnetworkIpNetworkAsInet
                    | PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr
                    | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range
                    | PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range
                    | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                    | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                    | PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                    PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => default_initialized_by_postgres,
                    PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => (&ok_std_string_string_from_uuid_generate_v4_ts, &ok_query_ts),
                }
            };
            let select_only_ids_and_select_only_updated_ids_query_common_ts = {
                let format_handle_ts = gen_quotes::double_quotes_ts(&{
                    let column_comma = "{column},";
                    if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) { column_comma.to_owned() } else { format!("'{{{{\\\"value\\\": null}}}}'::jsonb as {column_comma}") }
                });
                quote! {Ok(format!(#format_handle_ts))}
            };
            postgres_crud_macros_common::gen_impl_postgres_type_ts(
                &import_path,
                &ident,
                &ident_table_type_declaration_ucc,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => postgres_crud_macros_common::IsPrimaryKeyUnderscore::True,
                    CanBePrimaryKey::True => postgres_crud_macros_common::IsPrimaryKeyUnderscore::False,
                },
                &{
                    let postgres_query_type = match &postgres_type {
                        PostgresType::StdPrimitiveI16AsInt2 => "int2",
                        PostgresType::StdPrimitiveI32AsInt4 => "int4",
                        PostgresType::StdPrimitiveI64AsInt8 => "int8",
                        PostgresType::StdPrimitiveF32AsFloat4 => "float4",
                        PostgresType::StdPrimitiveF64AsFloat8 => "float8",
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres => "smallserial",
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres => "serial",
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => "bigserial",
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney => "money",
                        PostgresType::StdPrimitiveBoolAsBool => "bool",
                        PostgresType::StdStringStringAsText => "text",
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea => "bytea",
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime | PostgresType::SqlxTypesTimeTimeAsTime => "time",
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval => "interval",
                        PostgresType::SqlxTypesChronoNaiveDateAsDate => "date",
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => "timestamp",
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => "timestamptz",
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => "uuid",
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => "inet",
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => "macaddr",
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => "int4range",
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => "int8range",
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => "daterange",
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => "tsrange",
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => "tstzrange",
                    };
                    let maybe_array_part = match &postgres_type_pattern {
                        PostgresTypePattern::Standart => String::new(),
                        PostgresTypePattern::ArrayDimension1 { .. } => repeat_n("[]", array_dimensions_number).collect::<String>(),
                    };
                    let maybe_constraint_part = match &postgres_type_pattern {
                        PostgresTypePattern::Standart => String::new(),
                        PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &dimension1_not_null_or_nullable {
                            NotNullOrNullable::NotNull => ",check (array_position({column},null) is null)".to_owned(),
                            NotNullOrNullable::Nullable => String::new(),
                        },
                    };
                    let maybe_primary_key_is_primary_key_ts = quote! {postgres_types_common::maybe_primary_key(is_primary_key)};
                    let column_postgres_query_type = format!("{{column}} {postgres_query_type}{maybe_array_part}{maybe_constraint_part}");
                    let column_postgres_query_type_not_null = format!("{{column}} {postgres_query_type}{maybe_array_part} not null{maybe_constraint_part}");
                    let space_additional_parameter = " {}";
                    match (&not_null_or_nullable, &can_be_primary_key) {
                        (NotNullOrNullable::NotNull, CanBePrimaryKey::False) => {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&column_postgres_query_type_not_null);
                            quote! {
                                format!(#format_handle_ts)
                            }
                        }
                        (NotNullOrNullable::NotNull, CanBePrimaryKey::True) => {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{column_postgres_query_type_not_null}{space_additional_parameter}"));
                            quote! {
                                format!(#format_handle_ts, #maybe_primary_key_is_primary_key_ts)
                            }
                        }
                        (NotNullOrNullable::Nullable, CanBePrimaryKey::False) => {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&column_postgres_query_type);
                            quote! {
                                format!(#format_handle_ts)
                            }
                        }
                        (NotNullOrNullable::Nullable, CanBePrimaryKey::True) => {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&format!("{column_postgres_query_type}{space_additional_parameter}"));
                            quote! {
                                format!(#format_handle_ts, #maybe_primary_key_is_primary_key_ts)
                            }
                        }
                    }
                },
                &ident_create_ucc,
                &postgres_crud_macros_common::CreateQueryPartValueUnderscore::True,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => postgres_crud_macros_common::CreateQueryPartIncrementUnderscore::False,
                    CanBePrimaryKey::True => postgres_crud_macros_common::CreateQueryPartIncrementUnderscore::True,
                },
                &query_part_create_ts,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => postgres_crud_macros_common::CreateQueryBindValueUnderscore::False,
                    CanBePrimaryKey::True => postgres_crud_macros_common::CreateQueryBindValueUnderscore::True,
                },
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => postgres_crud_macros_common::IsCreateQueryBindMutable::True,
                    CanBePrimaryKey::True => postgres_crud_macros_common::IsCreateQueryBindMutable::False,
                },
                &bind_value_to_query_create_ts,
                &ident_select_ucc,
                &match &element.postgres_type_pattern {
                    PostgresTypePattern::Standart => postgres_crud_macros_common::SelectQueryPartValueUnderscore::True,
                    PostgresTypePattern::ArrayDimension1 { .. } => postgres_crud_macros_common::SelectQueryPartValueUnderscore::False,
                },
                &{
                    let content_ts = match &postgres_type_pattern {
                        PostgresTypePattern::Standart => quote! {#ColumnSc.to_owned()},
                        PostgresTypePattern::ArrayDimension1 { .. } => {
                            let format_handle_ts = gen_quotes::double_quotes_ts(&{
                                let acc = repeat_n("[{}:{}]", array_dimensions_number).collect::<String>();
                                format!("{{column}}{acc}")
                            });
                            let arguments_ts = (1..=array_dimensions_number)
                            .map(|el_268f0f14| {
                                let dimension_number_pagination_ts = format!("dimension{el_268f0f14}_pagination")
                                .parse::<proc_macro2::TokenStream>()
                                .expect("6f2305ee-85e9-4dce-9a14-9e299586668a");
                                quote! {
                                    #ValueSc.#dimension_number_pagination_ts.start(),
                                    #ValueSc.#dimension_number_pagination_ts.end(),
                                }
                            });
                            quote! {format!(
                                #format_handle_ts,
                                #(#arguments_ts)*
                            )}
                        }
                    };
                    quote! {Ok(#content_ts)}
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &{
                    let gen_ident_read_ident_origin_ts = |content_ts: &dyn quote::ToTokens| {
                        quote! {#ident_read_ucc(#ident_origin_ucc(#content_ts))}
                    };
                    match &postgres_type_pattern {
                        PostgresTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => {
                                PostgresTypeRange::try_from(postgres_type).as_ref().map_or_else(
                                    |()| quote! {#ValueSc},
                                    |postgres_type_range| {
                                        let gen_sqlx_postgres_types_pg_range_ts = |start_ts: &dyn quote::ToTokens, end_ts: &dyn quote::ToTokens| {
                                            quote! {
                                                sqlx::postgres::types::PgRange{
                                                    #StartSc: std::ops::Bound::#start_ts,
                                                    #EndSc: std::ops::Bound::#end_ts
                                                }
                                            }
                                        };
                                        let included_start_ts = quote! {#IncludedUcc(#StartSc)};
                                        let excluded_end_ts = quote! {#ExcludedUcc(#EndSc)};
                                        let included_end_ts = quote! {#IncludedUcc(#EndSc)};
                                        let excluded_start_ts = quote! {#ExcludedUcc(#StartSc)};
                                        let sqlx_postgres_types_pg_range_excluded_excluded_ts = gen_sqlx_postgres_types_pg_range_ts(&excluded_start_ts, &excluded_end_ts);
                                        let sqlx_postgres_types_pg_range_excluded_included_ts = gen_sqlx_postgres_types_pg_range_ts(&excluded_start_ts, &included_end_ts);
                                        let sqlx_postgres_types_pg_range_included_unbounded_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_ts, &UnboundedUcc);
                                        let sqlx_postgres_types_pg_range_unbounded_excluded_ts = gen_sqlx_postgres_types_pg_range_ts(&UnboundedUcc, &excluded_end_ts);
                                        let sqlx_postgres_types_pg_range_included_excluded_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_ts, &excluded_end_ts);
                                        let sqlx_postgres_types_pg_range_unbounded_unbounded_ts = gen_sqlx_postgres_types_pg_range_ts(&UnboundedUcc, &UnboundedUcc);
                                        let gen_range_match_ts = |
                                            included_included_ts: &dyn quote::ToTokens,
                                            included_excluded_ts: &dyn quote::ToTokens,
                                            included_unbounded_ts: &dyn quote::ToTokens,
                                            excluded_included_ts: &dyn quote::ToTokens,
                                            excluded_excluded_ts: &dyn quote::ToTokens,
                                            excluded_unbounded_ts: &dyn quote::ToTokens,
                                            unbounded_included_ts: &dyn quote::ToTokens,
                                            unbounded_excluded_ts: &dyn quote::ToTokens
                                        | {
                                            quote! {
                                                #ident_standart_not_null_read_ucc(#ident_standart_not_null_origin_ucc(match (
                                                    #ValueSc.0.0.#StartSc,
                                                    #ValueSc.0.0.#EndSc
                                                ) {
                                                    (std::ops::Bound::#IncludedUcc(#StartSc), std::ops::Bound::#IncludedUcc(#EndSc)) => {
                                                        #included_included_ts
                                                    },
                                                    (std::ops::Bound::#IncludedUcc(#StartSc), std::ops::Bound::#ExcludedUcc(#EndSc)) => {
                                                        #included_excluded_ts
                                                    },
                                                    (std::ops::Bound::#IncludedUcc(#StartSc), std::ops::Bound::#UnboundedUcc) => {
                                                        #included_unbounded_ts
                                                    },
                                                    (std::ops::Bound::#ExcludedUcc(#StartSc), std::ops::Bound::#IncludedUcc(#EndSc)) => {
                                                        #excluded_included_ts
                                                    },
                                                    (std::ops::Bound::#ExcludedUcc(#StartSc), std::ops::Bound::#ExcludedUcc(#EndSc)) => {
                                                        #excluded_excluded_ts
                                                    },
                                                    (std::ops::Bound::#ExcludedUcc(#StartSc), std::ops::Bound::#UnboundedUcc) => {
                                                        #excluded_unbounded_ts
                                                    },
                                                    (std::ops::Bound::#UnboundedUcc, std::ops::Bound::#IncludedUcc(#EndSc)) => {
                                                        #unbounded_included_ts
                                                    },
                                                    (std::ops::Bound::#UnboundedUcc, std::ops::Bound::#ExcludedUcc(#EndSc)) => {
                                                        #unbounded_excluded_ts
                                                    },
                                                    (std::ops::Bound::#UnboundedUcc, std::ops::Bound::#UnboundedUcc) => #sqlx_postgres_types_pg_range_unbounded_unbounded_ts,
                                                }))
                                            }
                                        };
                                        let gen_if_start_end_equal_ts = |true_ts: &dyn quote::ToTokens, false_ts: &dyn quote::ToTokens| {
                                            quote! {
                                                if #StartSc == #EndSc {
                                                    #true_ts
                                                } else {
                                                    #false_ts
                                                }
                                            }
                                        };
                                        let if_equal_unbounded_unbounded_or_included_excluded_ts = gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &sqlx_postgres_types_pg_range_included_excluded_ts);
                                        let int_range_normalize_ts = {
                                            let (
                                                included_start_checked_add_ts,
                                                excluded_end_checked_add_ts
                                            ) = {
                                                let gen_checked_add_one_expect_ts = |first_ts: &dyn quote::ToTokens, second_ts: &dyn quote::ToTokens| {
                                                    quote! {#first_ts(#second_ts.checked_add(1).expect("0ec0992f-1d63-443f-b528-7fabfff31423"))}
                                                };
                                                (
                                                    gen_checked_add_one_expect_ts(&IncludedUcc, &StartSc),
                                                    gen_checked_add_one_expect_ts(&ExcludedUcc, &EndSc)
                                                )
                                            };
                                            let included_excluded_checked_add_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_ts, &excluded_end_checked_add_ts);
                                            let included_unbounded_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_ts, &UnboundedUcc);
                                            let included_checked_add_excluded_checked_add_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_checked_add_ts, &excluded_end_checked_add_ts);
                                            let included_checked_add_excluded_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_checked_add_ts, &excluded_end_ts);
                                            let included_checked_add_unbounded_ts = gen_sqlx_postgres_types_pg_range_ts(&included_start_checked_add_ts, &UnboundedUcc);
                                            let unbounded_excluded_checked_add_ts = gen_sqlx_postgres_types_pg_range_ts(&UnboundedUcc, &excluded_end_checked_add_ts);
                                            let unbounded_excluded_ts = gen_sqlx_postgres_types_pg_range_ts(&UnboundedUcc, &excluded_end_ts);
                                            gen_range_match_ts(
                                                &included_excluded_checked_add_ts,
                                                &gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &sqlx_postgres_types_pg_range_included_excluded_ts),
                                                &included_unbounded_ts,
                                                &gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_checked_add_ts),
                                                &gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_ts),
                                                &included_checked_add_unbounded_ts,
                                                &unbounded_excluded_checked_add_ts,
                                                &unbounded_excluded_ts,
                                            )
                                        };
                                        let range_match_timestamp_and_timestamp_tz_ts = gen_range_match_ts(
                                            &gen_sqlx_postgres_types_pg_range_ts(&included_start_ts, &included_end_ts),
                                            &if_equal_unbounded_unbounded_or_included_excluded_ts,
                                            &sqlx_postgres_types_pg_range_included_unbounded_ts,
                                            &gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &sqlx_postgres_types_pg_range_excluded_included_ts),
                                            &gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &sqlx_postgres_types_pg_range_excluded_excluded_ts),
                                            &gen_sqlx_postgres_types_pg_range_ts(&excluded_start_ts, &UnboundedUcc),
                                            &gen_sqlx_postgres_types_pg_range_ts(&UnboundedUcc, &included_end_ts),
                                            &sqlx_postgres_types_pg_range_unbounded_excluded_ts,
                                        );
                                        match &postgres_type_range {
                                            PostgresTypeRange::StdPrimitiveI32AsInt4 | PostgresTypeRange::StdPrimitiveI64AsInt8 => int_range_normalize_ts,
                                            PostgresTypeRange::SqlxTypesChronoNaiveDateAsDate => {
                                                let gen_dot_succ_opt_expect_ts = |id: &dyn Display| {
                                                    let id_double_quotes_ts = gen_quotes::double_quotes_ts(&id);
                                                    quote! {.succ_opt().expect(#id_double_quotes_ts)}
                                                };
                                                let gen_included_start_succ_opt_ts = |id: &dyn Display| {
                                                    let dot_succ_opt_expect_ts = gen_dot_succ_opt_expect_ts(&id);
                                                    quote! {#IncludedUcc(#StartSc #dot_succ_opt_expect_ts)}
                                                };
                                                let gen_excluded_end_succ_opt_ts = |id: &dyn Display| {
                                                    let dot_succ_opt_expect_ts = gen_dot_succ_opt_expect_ts(&id);
                                                    quote! {#ExcludedUcc(#EndSc #dot_succ_opt_expect_ts)}
                                                };
                                                gen_range_match_ts(
                                                    &gen_sqlx_postgres_types_pg_range_ts(&included_start_ts, &quote! {#ExcludedUcc(#EndSc.succ_opt().expect("9ebce3b4-4ca7-4ff5-8b7a-a3539125bba0"))}),
                                                    &if_equal_unbounded_unbounded_or_included_excluded_ts,
                                                    &sqlx_postgres_types_pg_range_included_unbounded_ts,
                                                    &gen_if_start_end_equal_ts(
                                                        &sqlx_postgres_types_pg_range_unbounded_unbounded_ts,
                                                        &gen_sqlx_postgres_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"98a0357b-d21a-4949-a101-c641528d2376"), &gen_excluded_end_succ_opt_ts(&"fe53a6b9-2d7e-4605-9f5a-7f5c21cc01e6")),
                                                    ),
                                                    &gen_if_start_end_equal_ts(&sqlx_postgres_types_pg_range_unbounded_unbounded_ts, &gen_sqlx_postgres_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"d8a26635-c478-4a2a-acf4-bf1765702889"), &excluded_end_ts)),
                                                    &gen_sqlx_postgres_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"9811c7c7-d7f5-4fb7-9d25-affb0bd4f5fb"), &UnboundedUcc),
                                                    &gen_sqlx_postgres_types_pg_range_ts(&UnboundedUcc, &gen_excluded_end_succ_opt_ts(&"d6288f19-0a24-42ad-9e69-36036d9f2c1d")),
                                                    &sqlx_postgres_types_pg_range_unbounded_excluded_ts,
                                                )
                                            }
                                            PostgresTypeRange::SqlxTypesChronoNaiveDateTimeAsTimestamp | PostgresTypeRange::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => range_match_timestamp_and_timestamp_tz_ts,
                                        }
                                    }
                                )
                            }
                            NotNullOrNullable::Nullable => gen_ident_read_ident_origin_ts(&quote! {
                                #ValueSc.0.0.map(
                                    |value_4561270e|
                                    <
                                        #ident_standart_not_null_ucc
                                        as
                                        #import_path::PostgresType
                                    >::normalize(
                                        #ident_standart_not_null_read_ucc(value_4561270e)
                                    ).0
                                )
                            }),
                        },
                        PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                            (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => gen_ident_read_ident_origin_ts(&quote! {
                                #ValueSc.0.0.into_iter().map(|el_7302af7b|{
                                    #ident_standart_not_null_as_postgres_type_ts::normalize(
                                        #ident_standart_not_null_read_ucc(el_7302af7b)
                                    ).0
                                }).collect()
                            }),
                            (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => gen_ident_read_ident_origin_ts(&{
                                let current_ident_ts = gen_ident_ts(postgres_type, &NotNullOrNullable::Nullable, &PostgresTypePattern::Standart);
                                let ident_array_standart_nullable_read_ucc = SelfReadUcc::from_tokens(&current_ident_ts);
                                quote! {
                                    #ValueSc.0.0.into_iter().map(|el_fc25e056|{
                                        #ident_standart_nullable_as_postgres_type_ts::normalize(
                                            #ident_array_standart_nullable_read_ucc(el_fc25e056)
                                        ).0
                                    }).collect()
                                }
                            }),
                            (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => gen_ident_read_ident_origin_ts(&{
                                let ident_array_dimension1_not_null_not_null_ucc = gen_ident_ts(
                                    postgres_type,
                                    &NotNullOrNullable::NotNull,
                                    &PostgresTypePattern::ArrayDimension1 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable::NotNull,
                                    },
                                );
                                let ident_array_dimension1_not_null_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_array_dimension1_not_null_not_null_ucc);
                                quote! {
                                    #ValueSc.0.0.map(|value_b4d912fb|
                                        <
                                            #ident_array_dimension1_not_null_not_null_ucc
                                            as
                                            #import_path::PostgresType
                                        >::normalize(
                                            #ident_array_dimension1_not_null_not_null_read_ucc(value_b4d912fb),
                                        ).0
                                    )
                                }
                            }),
                            (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => gen_ident_read_ident_origin_ts(&{
                                let ident_array_dimension1_not_null_nullable_ucc = gen_ident_ts(
                                    postgres_type,
                                    &NotNullOrNullable::NotNull,
                                    &PostgresTypePattern::ArrayDimension1 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable::Nullable,
                                    },
                                );
                                let ident_array_dimension1_not_null_nullable_read_ucc = SelfReadUcc::from_tokens(&ident_array_dimension1_not_null_nullable_ucc);
                                quote! {
                                    #ValueSc.0.0.map(
                                        |value_dd042db2|
                                        <
                                            #ident_array_dimension1_not_null_nullable_ucc
                                            as
                                            #import_path::PostgresType
                                        >::normalize(
                                            #ident_array_dimension1_not_null_nullable_read_ucc(value_dd042db2),
                                        ).0
                                    )
                                }
                            }),
                        },
                    }
                },
                &if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
                    quote! {#ident_read_only_ids_ucc}
                } else {
                    quote! {#import_path_non_primary_key_postgres_type_read_only_ids_ts}
                },
                &select_only_ids_and_select_only_updated_ids_query_common_ts,
                &ident_read_inner_ucc,
                &{
                    let gen_ident_standart_not_null_into_inner_ident_standart_not_null_read_ts = |content_ts: &dyn quote::ToTokens| {
                        quote! {
                            #ident_standart_not_null_as_postgres_type_ts::into_inner(
                                #ident_standart_not_null_read_ucc(#content_ts)
                            )
                        }
                    };
                    let value_dot_zero_ts = quote! {#ValueSc.0};
                    let value_dot_zero_dot_zero_ts = quote! {#value_dot_zero_ts.0};
                    match &postgres_type_pattern {
                        PostgresTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => {
                                if postgres_type_range_try_from_postgres_type_is_ok {
                                    gen_pg_range_conversion_ts(&value_dot_zero_dot_zero_ts, &quote!{value_af65ccce})
                                } else {
                                    value_dot_zero_dot_zero_ts
                                }
                            }
                            NotNullOrNullable::Nullable => {
                                let content_ts = if postgres_type_range_try_from_postgres_type_is_ok {
                                    gen_ident_standart_not_null_into_inner_ident_standart_not_null_read_ts(&quote!{value_bd169d3b})
                                } else {
                                    quote!{value_bd169d3b.0}
                                };
                                quote! {#value_dot_zero_dot_zero_ts.map(|value_bd169d3b| #content_ts)}
                            }
                        },
                        PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                            (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => {
                                let content_ts = if postgres_type_range_try_from_postgres_type_is_ok {
                                    gen_ident_standart_not_null_into_inner_ident_standart_not_null_read_ts(&quote!{el_f5e94f0c})
                                } else {
                                    quote! {el_f5e94f0c.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.into_iter().map(|el_f5e94f0c|#content_ts).collect()
                                }
                            }
                            (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => {
                                let content_ts = if postgres_type_range_try_from_postgres_type_is_ok {
                                    gen_ident_standart_not_null_into_inner_ident_standart_not_null_read_ts(&quote!{value_e9a6bd41})
                                } else {
                                    quote!{value_e9a6bd41.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.into_iter().map(|el_236259fc|
                                        el_236259fc.0.map(|value_e9a6bd41| #content_ts)
                                    ).collect()
                                }
                            }
                            (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => {
                                let content_ts = if postgres_type_range_try_from_postgres_type_is_ok {
                                    gen_ident_standart_not_null_into_inner_ident_standart_not_null_read_ts(&quote!{el_b37be63e})
                                } else {
                                    quote! {el_b37be63e.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.map(|value_47fb2e43|
                                        value_47fb2e43.0.into_iter().map(|el_b37be63e|#content_ts).collect()
                                    )
                                }
                            }
                            (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => {
                                let content_ts = if postgres_type_range_try_from_postgres_type_is_ok {
                                    gen_ident_standart_not_null_into_inner_ident_standart_not_null_read_ts(&quote!{value_e5c5f65c})
                                } else {
                                    quote!{value_e5c5f65c.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.map(|value_b1a259c4| value_b1a259c4.0.into_iter().map(|el_19a7e6d0|
                                        el_19a7e6d0.0.map(|value_e5c5f65c| #content_ts)
                                    ).collect())
                                }
                            }
                        },
                    }
                },
                &ident_update_ucc,
                &ident_update_for_query_ucc,
                &postgres_crud_macros_common::UpdateQueryPartValueUnderscore::True,
                &postgres_crud_macros_common::UpdateQueryPartJsonbSetAccumulatorUnderscore::True,
                &postgres_crud_macros_common::UpdateQueryPartJsonbSetTargetUnderscore::True,
                &postgres_crud_macros_common::UpdateQueryPartJsonbSetPathUnderscore::True,
                &typical_query_part_ts,
                &postgres_crud_macros_common::IsUpdateQueryBindMutable::True,
                &typical_query_bind_ts,
                &select_only_ids_and_select_only_updated_ids_query_common_ts,
                &postgres_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                &quote! {Ok(#QuerySc)},
            )
        };
        let impl_postgres_type_test_cases_for_ident_ts = {
            enum IsNeedToUseInto {
                False,
                True,
            }
            let gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts = |read_or_update: &postgres_crud_macros_common::ReadOrUpdate| {
                let read_or_update_ucc = read_or_update.ucc();
                let content_ts = if postgres_type_initialization_try_new_try_from_postgres_type.is_ok() {
                    quote! {#TryNewSc(#ValueSc).expect("69477d2f-1c78-4a08-bdb7-c84022352dee")}
                } else {
                    quote! {#NewSc(#ValueSc)}
                };
                quote! {<#SelfUcc::#PostgresTypeUcc
                    as
                #import_path::#PostgresTypeUcc>::#read_or_update_ucc:: #content_ts}
            };
            let gen_standart_not_null_test_case_handle_ts = |is_need_to_use_into: &IsNeedToUseInto| {
                let gen_range_read_only_ids_to_two_dimensional_vec_read_inner_ts =
                    |min_ts: &dyn quote::ToTokens, negative_less_typical_ts: &dyn quote::ToTokens, negative_more_typical_ts: &dyn quote::ToTokens, near_zero_ts: &dyn quote::ToTokens, positive_less_typical_ts: &dyn quote::ToTokens, positive_more_typical_ts: &dyn quote::ToTokens, max_ts: &dyn quote::ToTokens| {
                        quote! {{
                            let min = #min_ts;
                            let negative_less_typical = #negative_less_typical_ts;
                            let negative_more_typical = #negative_more_typical_ts;
                            let near_zero = #near_zero_ts;
                            let positive_less_typical = #positive_less_typical_ts;
                            let positive_more_typical = #positive_more_typical_ts;
                            let max = #max_ts;
                            vec![
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min), end: std::ops::Bound::Included(min)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(negative_less_typical), end: std::ops::Bound::Included(negative_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(near_zero), end: std::ops::Bound::Included(near_zero)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(positive_less_typical), end: std::ops::Bound::Included(positive_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(max), end: std::ops::Bound::Included(max)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min), end: std::ops::Bound::Included(max)},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min), end: std::ops::Bound::Excluded(min)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(negative_less_typical), end: std::ops::Bound::Excluded(negative_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(near_zero), end: std::ops::Bound::Excluded(near_zero)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(positive_less_typical), end: std::ops::Bound::Excluded(positive_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(max), end: std::ops::Bound::Excluded(max)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min), end: std::ops::Bound::Excluded(max)},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(min), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(negative_less_typical), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(near_zero), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(positive_less_typical), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Included(max), end: std::ops::Bound::Unbounded},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min), end: std::ops::Bound::Included(min)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(negative_less_typical), end: std::ops::Bound::Included(negative_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(near_zero), end: std::ops::Bound::Included(near_zero)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(positive_less_typical), end: std::ops::Bound::Included(positive_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(max), end: std::ops::Bound::Included(max)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min), end: std::ops::Bound::Included(max)},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min), end: std::ops::Bound::Excluded(min)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(negative_less_typical), end: std::ops::Bound::Excluded(negative_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(near_zero), end: std::ops::Bound::Excluded(near_zero)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(positive_less_typical), end: std::ops::Bound::Excluded(positive_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(max), end: std::ops::Bound::Excluded(max)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min), end: std::ops::Bound::Excluded(max)},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(min), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(negative_less_typical), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(near_zero), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(positive_less_typical), end: std::ops::Bound::Unbounded},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Excluded(max), end: std::ops::Bound::Unbounded},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(min)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(negative_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(near_zero)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(positive_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Included(max)},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(min)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(negative_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(near_zero)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(positive_more_typical)},
                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Excluded(max)},

                                sqlx::postgres::types::PgRange { start: std::ops::Bound::Unbounded, end: std::ops::Bound::Unbounded},
                            ]
                        }}
                    };
                let gen_int_pgrange_read_only_ids_to_two_dimensional_vec_read_inner_ts = |int_range_type: &IntRangeType| {
                    let range_inner_type_ts = int_range_type_to_range_inner_type_ts(int_range_type);
                    gen_range_read_only_ids_to_two_dimensional_vec_read_inner_ts(&quote! {#range_inner_type_ts::MIN}, &quote! {-20}, &quote! {-10}, &quote! {0}, &quote! {10}, &quote! {20}, &quote! {#range_inner_type_ts::MAX - 1})
                };
                let empty_vec_ts = quote! {Vec::new()};
                let gen_ident_standart_not_null_function_ts = |
                    ident_8b874ea5: &dyn quote::ToTokens,
                    content_ts: &dyn quote::ToTokens
                |quote!{#ident_8b874ea5::#content_ts()};
                let (
                    ident_sqlx_types_chrono_naive_time_min_ts,
                    ident_sqlx_types_chrono_naive_time_ten_ts,
                    ident_sqlx_types_chrono_naive_time_twenty_ts,
                    ident_sqlx_types_chrono_naive_time_max_ts
                ) = {
                    let gen_sqlx_types_chrono_naive_time_as_time_standart_not_null_function_ts = |
                        content_ts_fd88ca39: &dyn quote::ToTokens
                    |gen_ident_standart_not_null_function_ts(
                        &gen_ident_standart_not_null_ts(&PostgresType::SqlxTypesChronoNaiveTimeAsTime),
                        &content_ts_fd88ca39
                    );
                    (
                        gen_sqlx_types_chrono_naive_time_as_time_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_min_function_ts
                        ),
                        gen_sqlx_types_chrono_naive_time_as_time_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_ten_function_ts
                        ),
                        gen_sqlx_types_chrono_naive_time_as_time_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_twenty_function_ts
                        ),
                        gen_sqlx_types_chrono_naive_time_as_time_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_max_function_ts
                        )
                    )
                };
                let (
                    ident_sqlx_types_chrono_naive_date_min_ts,
                    ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                    ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                    ident_sqlx_types_chrono_naive_date_near_zero_ts,
                    ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                    ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                    ident_sqlx_types_chrono_naive_date_max_ts,
                    ident_sqlx_types_chrono_naive_date_max_pred_opt_expect_ts,
                ) = {
                    let sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts = |
                        content_ts_7c66f815: &dyn quote::ToTokens
                    |gen_ident_standart_not_null_function_ts(
                        &gen_ident_standart_not_null_ts(&PostgresType::SqlxTypesChronoNaiveDateAsDate),
                        &content_ts_7c66f815
                    );
                    (
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_min_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_negative_less_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_negative_more_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_near_zero_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_positive_less_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_positive_more_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_max_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_standart_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_max_pred_opt_expect_function_ts,
                        ),
                    )
                };
                let sqlx_types_chrono_naive_date_time_min_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_min_ts,
                    #ident_sqlx_types_chrono_naive_time_min_ts
                });
                let sqlx_types_chrono_naive_date_time_negative_less_typical_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                    #ident_sqlx_types_chrono_naive_time_twenty_ts,
                });
                let sqlx_types_chrono_naive_date_time_negative_more_typical_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                    #ident_sqlx_types_chrono_naive_time_ten_ts,
                });
                let sqlx_types_chrono_naive_date_time_near_zero_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_near_zero_ts,
                    #ident_sqlx_types_chrono_naive_time_min_ts
                });
                let sqlx_types_chrono_naive_date_time_positive_less_typical_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                    #ident_sqlx_types_chrono_naive_time_ten_ts,
                });
                let sqlx_types_chrono_naive_date_time_positive_more_typical_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                    #ident_sqlx_types_chrono_naive_time_twenty_ts,
                });
                let sqlx_types_chrono_naive_date_time_max_ts = gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                    #ident_sqlx_types_chrono_naive_date_max_ts,
                    #ident_sqlx_types_chrono_naive_time_max_ts
                });
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_min_ts);
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_negative_less_typical_ts);
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_negative_more_typical_ts);
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_near_zero_ts);
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_positive_less_typical_ts);
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_positive_more_typical_ts);
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&sqlx_types_chrono_naive_date_time_max_ts);
                let gen_typical_test_cases_vec_ts = |value: &dyn quote::ToTokens| {
                    let content_ts = match &is_need_to_use_into {
                        IsNeedToUseInto::True => quote! {.into()},
                        IsNeedToUseInto::False => proc_macro2::TokenStream::new(),
                    };
                    quote! {#import_path::#value()#content_ts}
                };
                match &postgres_type {
                    PostgresType::StdPrimitiveI16AsInt2 => gen_typical_test_cases_vec_ts(&quote! {std_primitive_i16_test_cases_vec}),
                    PostgresType::StdPrimitiveI32AsInt4 => gen_typical_test_cases_vec_ts(&quote! {std_primitive_i32_test_cases_vec}),
                    PostgresType::StdPrimitiveI64AsInt8 => gen_typical_test_cases_vec_ts(&quote! {std_primitive_i64_test_cases_vec}),
                    PostgresType::StdPrimitiveF32AsFloat4 => gen_typical_test_cases_vec_ts(&quote! {std_primitive_f32_test_cases_vec}),
                    PostgresType::StdPrimitiveF64AsFloat8 => gen_typical_test_cases_vec_ts(&quote! {std_primitive_f64_test_cases_vec}),
                    PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres | PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres | PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => empty_vec_ts,
                    PostgresType::SqlxPostgresTypesPgMoneyAsMoney => quote! {
                        #import_path::std_primitive_i64_test_cases_vec().into_iter().map(
                            #inner_type_standart_not_null_ts
                        ).collect::<Vec<#inner_type_standart_not_null_ts>>()
                    },
                    PostgresType::StdPrimitiveBoolAsBool => gen_typical_test_cases_vec_ts(&quote! {std_primitive_bool_test_cases_vec}),
                    PostgresType::StdStringStringAsText => gen_typical_test_cases_vec_ts(&quote! {std_string_string_test_cases_vec}),
                    PostgresType::StdVecVecStdPrimitiveU8AsBytea => quote! {vec![
                        Vec::new(),
                        (0u8..=255).collect(),
                        vec![0; 1024],
                        vec![0; 1024 * 1024 * 2],
                    ]},
                    PostgresType::SqlxTypesChronoNaiveTimeAsTime => {
                        let (
                            self_sqlx_types_chrono_naive_time_min_ts,
                            self_sqlx_types_chrono_naive_time_ten_ts,
                            self_sqlx_types_chrono_naive_time_twenty_ts,
                            self_sqlx_types_chrono_naive_time_max_ts,
                        ) = {
                            let gen_self_sqlx_types_chrono_naive_time_standart_not_null_function_ts = |content_ts_9d2b411e: &dyn quote::ToTokens|gen_ident_standart_not_null_function_ts(
                                &SelfUcc,
                                &content_ts_9d2b411e
                            );
                            (
                                gen_self_sqlx_types_chrono_naive_time_standart_not_null_function_ts(&sqlx_types_chrono_naive_time_min_function_ts),
                                gen_self_sqlx_types_chrono_naive_time_standart_not_null_function_ts(&sqlx_types_chrono_naive_time_ten_function_ts),
                                gen_self_sqlx_types_chrono_naive_time_standart_not_null_function_ts(&sqlx_types_chrono_naive_time_twenty_function_ts),
                                gen_self_sqlx_types_chrono_naive_time_standart_not_null_function_ts(&sqlx_types_chrono_naive_time_max_function_ts),
                            )
                        };
                        quote! {vec![
                            #self_sqlx_types_chrono_naive_time_min_ts,
                            #self_sqlx_types_chrono_naive_time_ten_ts,
                            #self_sqlx_types_chrono_naive_time_twenty_ts,
                            #self_sqlx_types_chrono_naive_time_max_ts,
                        ]}
                    },
                    PostgresType::SqlxTypesTimeTimeAsTime => {
                        let sqlx_types_time_time_from_hms_micro_min_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {0,0,0,0});
                        let sqlx_types_time_time_from_hms_micro_ten_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {10,10,10,10});
                        let sqlx_types_time_time_from_hms_micro_twenty_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {20,20,20,20});
                        let sqlx_types_time_time_from_hms_micro_max_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {23,59,59,999_999});
                        quote! {vec![
                            #sqlx_types_time_time_from_hms_micro_min_unwrap_ts,
                            #sqlx_types_time_time_from_hms_micro_ten_unwrap_ts,
                            #sqlx_types_time_time_from_hms_micro_twenty_unwrap_ts,
                            #sqlx_types_time_time_from_hms_micro_max_unwrap_ts,
                        ]}
                    }
                    PostgresType::SqlxPostgresTypesPgIntervalAsInterval => {
                        let min_ts = quote! {MIN};
                        let max_ts = quote! {MAX};
                        let std_primitive_i32_min_ts = quote! {#std_primitive_i32_ts::#min_ts};
                        let std_primitive_i32_max_ts = quote! {#std_primitive_i32_ts::#max_ts};
                        let gen_sqlx_postgres_types_pg_interval_ts = |months_ts: &dyn quote::ToTokens, days_ts: &dyn quote::ToTokens, microseconds_ts: &dyn quote::ToTokens| {
                            quote! {sqlx::postgres::types::PgInterval {
                                months: #months_ts,
                                days: #days_ts,
                                microseconds: #microseconds_ts
                            }}
                        };
                        let min_content_ts = gen_sqlx_postgres_types_pg_interval_ts(&std_primitive_i32_min_ts, &std_primitive_i32_min_ts, &quote! {#std_primitive_i64_ts::#min_ts});
                        let max_content_ts = gen_sqlx_postgres_types_pg_interval_ts(&std_primitive_i32_max_ts, &std_primitive_i32_max_ts, &quote! {#std_primitive_i64_ts::#max_ts});
                        quote! {vec![
                            #min_content_ts,
                            #max_content_ts
                        ]}
                    }
                    PostgresType::SqlxTypesChronoNaiveDateAsDate => {
                        let (
                            sqlx_types_chrono_naive_date_min_ts,
                            sqlx_types_chrono_naive_date_negative_less_typical_ts,
                            sqlx_types_chrono_naive_date_negative_more_typical_ts,
                            sqlx_types_chrono_naive_date_near_zero_ts,
                            sqlx_types_chrono_naive_date_positive_less_typical_ts,
                            sqlx_types_chrono_naive_date_positive_more_typical_ts,
                            sqlx_types_chrono_naive_date_max_ts
                        ) = {
                            let gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts = |content_ts_16bc2a50: &dyn quote::ToTokens|gen_ident_standart_not_null_function_ts(
                                &SelfUcc,
                                &content_ts_16bc2a50
                            );
                            (
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_min_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_negative_less_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_negative_more_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_near_zero_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_positive_less_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_positive_more_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_standart_not_null_function_ts(&sqlx_types_chrono_naive_date_max_function_ts)
                            )
                        };
                        quote! {vec![
                            #sqlx_types_chrono_naive_date_min_ts,
                            #sqlx_types_chrono_naive_date_negative_less_typical_ts,
                            #sqlx_types_chrono_naive_date_negative_more_typical_ts,
                            #sqlx_types_chrono_naive_date_near_zero_ts,
                            #sqlx_types_chrono_naive_date_positive_less_typical_ts,
                            #sqlx_types_chrono_naive_date_positive_more_typical_ts,
                            #sqlx_types_chrono_naive_date_max_ts,
                        ]}
                    },
                    PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {vec![
                        #sqlx_types_chrono_naive_date_time_min_ts,
                        #sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        #sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        #sqlx_types_chrono_naive_date_time_near_zero_ts,
                        #sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        #sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        #sqlx_types_chrono_naive_date_time_max_ts,
                    ]},
                    PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote! {vec![
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                    ]},
                    PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres => quote! {Vec::new()},
                    PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient => quote! {vec![
                        sqlx::types::Uuid::new_v4()
                    ]},
                    PostgresType::SqlxTypesIpnetworkIpNetworkAsInet => quote! {vec![
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").expect("478dbded-0912-4cb9-88e4-caddf5106628"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").expect("8af9e27e-8491-477d-821a-facc6e6344e3"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").expect("ba86505f-24fd-4f23-b2d0-3d873c357058"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").expect("32c744a0-38d5-45b6-a0b8-f744d7c7947e"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").expect("560815f8-60a6-42e2-9c9d-0edcbcc22457"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").expect("793db0ef-c8ea-4683-9782-34e304730d02"),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(std::net::Ipv4Addr::#NewSc(192, 168, 0, 0), 24).expect("c44934f2-335e-44b7-bb4d-0a91374b4a85")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(std::net::Ipv4Addr::#NewSc(10, 0, 0, 0), 8).expect("39e588d9-b32b-4611-a2f3-3ce500b93db0")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(std::net::Ipv4Addr::LOCALHOST, 32).expect("43fb25bd-03cd-44fe-bde8-dc92d8bafc71")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#NewSc(std::net::Ipv6Addr::LOCALHOST, 128).expect("b443be46-1805-4fda-b24b-71dba8d8b9d4")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#NewSc("2001:db8::".parse().expect("d4e6df27-fdb6-4e66-898c-abcfc41c5e49"), 32).expect("a7486c5e-6577-4b80-a3ec-097002698431")),
                    ]},
                    PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr => quote! {vec![
                        sqlx::types::mac_address::MacAddress::#NewSc([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                        sqlx::types::mac_address::MacAddress::#NewSc([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                        sqlx::types::mac_address::MacAddress::#NewSc([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                        sqlx::types::mac_address::MacAddress::#NewSc([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                        sqlx::types::mac_address::MacAddress::#NewSc([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                        sqlx::types::mac_address::MacAddress::#NewSc([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                    ]},
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range => gen_int_pgrange_read_only_ids_to_two_dimensional_vec_read_inner_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range),
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range => gen_int_pgrange_read_only_ids_to_two_dimensional_vec_read_inner_ts(&IntRangeType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_range_read_only_ids_to_two_dimensional_vec_read_inner_ts(
                        &ident_sqlx_types_chrono_naive_date_min_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_near_zero_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_max_pred_opt_expect_ts
                    ),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_range_read_only_ids_to_two_dimensional_vec_read_inner_ts(
                        &sqlx_types_chrono_naive_date_time_min_ts,
                        &sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_near_zero_ts,
                        &sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_max_ts,
                    ),
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_range_read_only_ids_to_two_dimensional_vec_read_inner_ts(
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                    ),
                }
            };
            let option_vec_create_ts = {
                let gen_some_acc_content_ts = |
                    current_not_null_or_nullable: &NotNullOrNullable,
                    current_ident_ts: &dyn quote::ToTokens,
                    additonal_content_ts: &dyn quote::ToTokens
                | {
                    let (new_or_try_new_content_ts, maybe_acc_push_none_ts) = match (&current_not_null_or_nullable, postgres_type_initialization_try_new_try_from_postgres_type.is_ok()) {
                        (NotNullOrNullable::NotNull, true) => (quote! {try_new(vec![el_0fd5865b.0.into()]).expect("adbae6b3-1542-4f81-89bf-48a9b895b488")}, proc_macro2::TokenStream::new()),
                        (NotNullOrNullable::NotNull, false) => (quote! {new(vec![el_0fd5865b.0.into()])}, proc_macro2::TokenStream::new()),
                        (NotNullOrNullable::Nullable, true) => (
                            quote! {try_new(Some(el_0fd5865b.0.into())).expect("b244d498-527d-4332-98c9-770d27e7af35")},
                            quote! {acc_0b59a062.push(#self_as_postgres_type_ts::Create::try_new(None).expect("31878971-17fc-4526-ab01-42c8332e641f"));},
                        ),
                        (NotNullOrNullable::Nullable, false) => (quote! {new(Some(el_0fd5865b.0.into()))}, quote! {acc_0b59a062.push(#self_as_postgres_type_ts::Create::new(None));}),
                    };
                    let ident_as_postgres_type_test_cases_ts = gen_as_postgres_type_test_cases_ts(&current_ident_ts);
                    quote! {Some({
                        let mut acc_0b59a062 = Vec::new();
                        for el_0fd5865b in #ident_as_postgres_type_test_cases_ts::#OptionVecCreateSc().unwrap_or(Vec::new()) {
                            acc_0b59a062.push(#self_as_postgres_type_ts::Create::#new_or_try_new_content_ts);
                        }
                        #maybe_acc_push_none_ts
                        #additonal_content_ts
                        acc_0b59a062
                    })}
                };
                match &postgres_type_pattern {
                    PostgresTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => match &can_be_primary_key {
                            CanBePrimaryKey::False => {
                                let content_ts = gen_standart_not_null_test_case_handle_ts(&IsNeedToUseInto::False);
                                let new_or_try_new_ts = {
                                    let self_as_postgres_type_create_ts = quote!{#self_as_postgres_type_ts::Create};
                                    if postgres_type_initialization_try_new_try_from_postgres_type.is_ok() {
                                        quote! {
                                            |el_043a7d30|#self_as_postgres_type_create_ts::try_new(
                                                el_043a7d30
                                            ).expect("941bd15c-a751-45e7-8266-f17df4ee00aa")
                                        }
                                    } else {
                                        quote! {#self_as_postgres_type_create_ts::#NewSc}
                                    }
                                };
                                quote! {Some(
                                    #content_ts.into_iter().map(
                                        #new_or_try_new_ts
                                    ).collect()
                                )}
                            }
                            CanBePrimaryKey::True => none_ts.clone(),
                        },
                        NotNullOrNullable::Nullable => gen_some_acc_content_ts(not_null_or_nullable, &gen_ident_ts(postgres_type, &NotNullOrNullable::NotNull, &PostgresTypePattern::Standart), &proc_macro2::TokenStream::new()),
                    },
                    PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => gen_some_acc_content_ts(
                        not_null_or_nullable,
                        &gen_ident_ts(
                            postgres_type,
                            &match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                            },
                            &match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => PostgresTypePattern::Standart,
                                NotNullOrNullable::Nullable => PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable },
                            },
                        ),
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => {
                                let content_ts: &dyn quote::ToTokens = match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => &ident_standart_not_null_as_postgres_type_test_cases_ts,
                                    NotNullOrNullable::Nullable => &ident_standart_nullable_as_postgres_type_test_cases_ts,
                                };
                                let (first_ts, second_ts, third_ts) = {
                                    let gen_new_or_try_new_ts = |current_content_ts: &dyn quote::ToTokens| {
                                        if postgres_type_initialization_try_new_try_from_postgres_type.is_ok() {
                                            quote! {try_new(#current_content_ts).expect("75ad9383-b257-4a0b-bd8d-c931950bf745")}
                                        } else {
                                            quote! {new(#current_content_ts)}
                                        }
                                    };
                                    let gen_vec_value_clone_zero_into_number_ts = |value: usize| {
                                        let number_ts = value.to_string().parse::<proc_macro2::TokenStream>().expect("50c87202-4038-4b27-85bd-c0593552bb89");
                                        //todo maybe correlate with .derive_copy_if()
                                        let current_maybe_dot_clone_ts: &dyn quote::ToTokens = match &postgres_type {
                                            PostgresType::StdPrimitiveI16AsInt2 |
                                            PostgresType::StdPrimitiveI32AsInt4 |
                                            PostgresType::StdPrimitiveI64AsInt8 |
                                            PostgresType::StdPrimitiveF32AsFloat4 |
                                            PostgresType::StdPrimitiveF64AsFloat8 |
                                            PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                                            PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                                            PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                                            PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                                            PostgresType::StdPrimitiveBoolAsBool |
                                            PostgresType::SqlxTypesChronoNaiveTimeAsTime | PostgresType::SqlxTypesTimeTimeAsTime |
                                            PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                                            PostgresType::SqlxTypesChronoNaiveDateAsDate |
                                            PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                                            PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                            PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                                            PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                                            PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                            PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                                            PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                            PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &proc_macro2::TokenStream::new(),
                                            PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                                            PostgresType::StdStringStringAsText => &dot_clone_ts,
                                        };
                                        quote! {vec![value_6465e8ae #current_maybe_dot_clone_ts.0.into(); #number_ts]}
                                    };
                                    (
                                        gen_new_or_try_new_ts(&quote! {
                                            #content_ts::#OptionVecCreateSc().unwrap_or(Vec::new())
                                            .into_iter()
                                            .map(|el_ffb375dd|el_ffb375dd.0.into())
                                            .collect()
                                        }),
                                        gen_new_or_try_new_ts(&gen_vec_value_clone_zero_into_number_ts(2)),
                                        gen_new_or_try_new_ts(&gen_vec_value_clone_zero_into_number_ts(1000)),
                                    )
                                };
                                quote! {
                                    acc_0b59a062.push(#self_as_postgres_type_ts::Create::#first_ts);
                                    if let Some(value_6465e8ae) = #content_ts::#OptionVecCreateSc().unwrap_or(Vec::new()).first() {
                                        acc_0b59a062.push(#self_as_postgres_type_ts::Create::#second_ts);
                                        acc_0b59a062.push(#self_as_postgres_type_ts::Create::#third_ts);
                                    }
                                }
                            }
                            NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                        },
                    ),
                }
            };
            let read_only_ids_to_two_dimensional_vec_read_inner_ts = {
                let gen_star_or_dot_clone_ts = |content_ts|match &postgres_type {
                    PostgresType::StdPrimitiveI16AsInt2 |
                    PostgresType::StdPrimitiveI32AsInt4 |
                    PostgresType::StdPrimitiveI64AsInt8 |
                    PostgresType::StdPrimitiveF32AsFloat4 |
                    PostgresType::StdPrimitiveF64AsFloat8 |
                    PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                    PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                    PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |
                    PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                    PostgresType::StdPrimitiveBoolAsBool |
                    PostgresType::SqlxTypesChronoNaiveTimeAsTime |
                    PostgresType::SqlxTypesTimeTimeAsTime |
                    PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                    PostgresType::SqlxTypesChronoNaiveDateAsDate |
                    PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                    PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                    PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres | PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                    PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                    PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                    PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => quote!{*#content_ts},
                    PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                    PostgresType::StdStringStringAsText => quote!{#content_ts.clone()}
                };
                match &postgres_type_pattern {
                    PostgresTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => {
                            let content_ts = gen_standart_not_null_test_case_handle_ts(&IsNeedToUseInto::True);
                            quote! {vec![{#content_ts}]}
                        }
                        NotNullOrNullable::Nullable => quote! {
                            #ident_standart_not_null_as_postgres_type_test_cases_ts::#ReadOnlyIdsToTwoDimensionalVecReadInnerSc(#ReadOnlyIdsSc)
                            .into_iter()
                            .flat_map(|el0| el0.into_iter().map(|el1| vec![Some(el1)]))
                            .chain(std::iter::once(vec![None]))
                            .collect()
                        },
                    },
                    PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => match &dimension1_not_null_or_nullable {
                            NotNullOrNullable::NotNull => {
                                let el_d27d1981_ts = gen_star_or_dot_clone_ts(&quote!{el_d27d1981});
                                quote! {
                                    let mut acc_abf96c9f = Vec::new();
                                    let read_only_ids_to_two_dimensional_vec_read_inner = #ident_standart_not_null_as_postgres_type_test_cases_ts::#ReadOnlyIdsToTwoDimensionalVecReadInnerSc(#ReadOnlyIdsSc);
                                    let option_additional = {
                                        let mut option_additional = None;
                                        for el_cb3f4b45 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                            if option_additional.is_some() {
                                                break;
                                            }
                                            for el_d27d1981 in el_cb3f4b45 {
                                                if option_additional.is_none() {
                                                    option_additional = Some((vec![
                                                        vec![#el_d27d1981_ts]],
                                                        vec![vec![#el_d27d1981_ts, #el_d27d1981_ts]
                                                    ]));
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
                                        for el_89e74982 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                            if el_89e74982.len() > 1 {
                                                has_len_greater_than_one = true;
                                                break;
                                            }
                                        }
                                        has_len_greater_than_one
                                    };
                                    for el_cb836246 in read_only_ids_to_two_dimensional_vec_read_inner {
                                        acc_abf96c9f.push(vec![el_cb836246]);
                                    }
                                    if let Some(value_e22f9ad2) = option_additional {
                                        if has_len_greater_than_one {
                                            acc_abf96c9f.push(value_e22f9ad2.0);
                                        }
                                        else {
                                            acc_abf96c9f.push(value_e22f9ad2.1);
                                        }
                                    }
                                    acc_abf96c9f
                                }
                            }
                            NotNullOrNullable::Nullable => {
                                let el_6b831e7c_ts = gen_star_or_dot_clone_ts(&quote!{el_6b831e7c});
                                quote! {
                                    let mut acc_68eba82f = Vec::new();
                                    let read_only_ids_to_two_dimensional_vec_read_inner = #ident_standart_nullable_as_postgres_type_test_cases_ts::#ReadOnlyIdsToTwoDimensionalVecReadInnerSc(#ReadOnlyIdsSc);
                                    let option_additional = {
                                        let mut option_additional = None;
                                        for el_b04183c6 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                            if option_additional.is_some() {
                                                break;
                                            }
                                            for el_6b831e7c in el_b04183c6 {
                                                if option_additional.is_none() {
                                                    option_additional = Some((
                                                        vec![vec![#el_6b831e7c_ts]],
                                                        vec![vec![#el_6b831e7c_ts, #el_6b831e7c_ts]]
                                                    ));
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                        }
                                        option_additional
                                    };
                                    let has_len_greater_than_one = read_only_ids_to_two_dimensional_vec_read_inner.len() > 1;
                                    acc_68eba82f.push(vec![
                                        read_only_ids_to_two_dimensional_vec_read_inner
                                        .into_iter()
                                        .flat_map(IntoIterator::into_iter)
                                        .collect()
                                    ]);
                                    if let Some(value_a0f0f172) = option_additional {
                                        if has_len_greater_than_one {
                                            acc_68eba82f.push(value_a0f0f172.0);
                                        }
                                        else {
                                            acc_68eba82f.push(value_a0f0f172.1);
                                        }
                                    }
                                    acc_68eba82f
                                }
                            }
                        },
                        NotNullOrNullable::Nullable => {
                            let content_ts = match &dimension1_not_null_or_nullable {
                                NotNullOrNullable::NotNull => &ident_array_not_null_as_postgres_type_test_cases_ts,
                                NotNullOrNullable::Nullable => &ident_array_nullable_as_postgres_type_test_cases_ts,
                            };
                            let el_31abc64a_ts = gen_star_or_dot_clone_ts(&quote!{el_31abc64a});
                            quote! {
                                let mut acc_5f7f59ac = Vec::new();
                                let read_only_ids_to_two_dimensional_vec_read_inner = #content_ts::#ReadOnlyIdsToTwoDimensionalVecReadInnerSc(#ReadOnlyIdsSc);
                                let option_additional = {
                                    let mut option_additional = None;
                                    for el_12a259ab in &read_only_ids_to_two_dimensional_vec_read_inner {
                                        if option_additional.is_some() {
                                            break;
                                        }
                                        for el_16a61773 in el_12a259ab {
                                            if option_additional.is_some() {
                                                break;
                                            }
                                            for el_31abc64a in el_16a61773 {
                                                if option_additional.is_none() {
                                                    option_additional = Some((
                                                        vec![Some(vec![#el_31abc64a_ts])],
                                                        vec![Some(vec![#el_31abc64a_ts, #el_31abc64a_ts])]
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
                                    for el_a177c6a3 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                        for el_aa72f570 in el_a177c6a3 {
                                            if el_aa72f570.len() > 1 {
                                                has_len_greater_than_one = true;
                                                break;
                                            }
                                        }
                                    }
                                    has_len_greater_than_one
                                };
                                acc_5f7f59ac.push(vec![Some(
                                    read_only_ids_to_two_dimensional_vec_read_inner
                                    .into_iter()
                                    .flatten()
                                    .flatten()
                                    .collect()
                                )]);
                                acc_5f7f59ac.push(vec![None]);
                                if let Some(value_3530786a) = option_additional {
                                    if has_len_greater_than_one {
                                        acc_5f7f59ac.push(value_3530786a.0);
                                    }
                                    else {
                                        acc_5f7f59ac.push(value_3530786a.1);
                                    }
                                }
                                acc_5f7f59ac
                            }
                        }
                    },
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&postgres_crud_macros_common::ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&postgres_crud_macros_common::ReadOrUpdate::Update);
            let update_to_read_only_ids_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
                quote! {
                    #ident_read_only_ids_ucc(#ident_read_ucc(#ValueSc.0 #maybe_dot_clone_ts))//todo its not correct. must be only for primary key but it for all types what van be primary key
                }
            } else {
                let value_initialization_ts = gen_import_path_value_initialization_ts(&none_ts);
                quote! {
                    #import_path_non_primary_key_postgres_type_read_only_ids_ts(#value_initialization_ts)
                }
            };
            let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = {
                //todo that is not correct for array of generated by postgres primary keys but maybe just need to remove this variants and thats it?
                let value_initialization_ts = gen_import_path_value_initialization_ts(&{
                    let content_ts: &dyn quote::ToTokens = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
                        &quote! {#ValueSc.0 #maybe_dot_clone_ts}
                    } else {
                        &postgres_crud_common_default_option_some_vec_one_el_call_ts
                    };
                    quote! {#self_postgres_type_as_postgres_type_ts::normalize(#content_ts)}
                });
                quote! {Some(#value_initialization_ts)}
            };
            let previous_read_merged_with_option_update_into_read_ts = quote! {
                #OptionUpdateSc.map_or(#ReadSc, |#ValueSc| #ident_read_ucc(#ValueSc.0))
            };
            let read_only_ids_merged_with_create_into_read_ts = {
                let content_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
                    quote! {#ReadOnlyIdsSc.0}
                } else {
                    quote! {#ident_read_ucc(#CreateSc.0)}
                };
                quote! {
                    #self_postgres_type_as_postgres_type_ts::normalize(#content_ts)
                }
            };
            let read_only_ids_merged_with_create_into_option_value_read_ts = {
                let value_initialization_ts = gen_import_path_value_initialization_ts(&quote! {
                    <Self as #import_path::PostgresTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoReadSc(
                        #ReadOnlyIdsSc,
                        #CreateSc
                    )
                });
                quote! {Some(#value_initialization_ts)}
            };
            let read_only_ids_merged_with_create_into_table_type_declaration_ts = {
                let content_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
                    quote! {#ReadOnlyIdsSc.0.0}
                } else {
                    quote! {#CreateSc.0}
                };
                quote! {#ident_table_type_declaration_ucc(#content_ts)}
            };
            //todo maybe it into function (not in proc macro)
            let read_only_ids_merged_with_create_into_where_equal_ts = {
                let content_ts = if matches!(&postgres_type_pattern, PostgresTypePattern::Standart)
                    && matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
                    && matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True)
                {
                    quote! {#ReadOnlyIdsSc.0.0}
                } else {
                    quote! {#CreateSc.0}
                };
                quote! {
                    #ident_where_ucc::#EqualUcc(where_filters::PostgresTypeWhereEqual {
                        logical_operator: #import_path::LogicalOperator::Or,
                        #ValueSc: #ident_table_type_declaration_ucc(#content_ts),
                    })
                }
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = quote! {
                #import_path::NotEmptyUniqueVec::try_new(vec![
                    #read_only_ids_merged_with_create_into_where_equal_ts
                ]).expect("4c08b551-1df7-4e5b-ae92-a700e0aded65")
            };
            let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts = none_ts.clone();
            let create_into_postgres_type_option_vec_where_dimension_one_equal_ts = match &postgres_type_pattern {
                PostgresTypePattern::Standart => none_ts.clone(),
                PostgresTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    let ident_standart_not_null_or_nullable_table_type_declaration_ucc: &dyn quote::ToTokens = match &dimension1_not_null_or_nullable {
                        NotNullOrNullable::NotNull => &ident_standart_not_null_table_type_declaration_ucc,
                        NotNullOrNullable::Nullable => &ident_standart_nullable_table_type_declaration_ucc,
                    };
                    let some_ts = {
                        let content_ts: &dyn quote::ToTokens = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => &quote! {#CreateSc.0.0},
                            NotNullOrNullable::Nullable => &quote! {value_09152b2e.0},
                        };
                        quote! {
                            match #import_path::NotEmptyUniqueVec::try_new({
                                let mut acc_74c71d5d = Vec::new();
                                for (index_7702518c, el_081d735b) in #content_ts.into_iter().enumerate() {
                                    acc_74c71d5d.push(
                                        #ident_where_ucc::DimensionOneEqual(
                                            where_filters::PostgresTypeWhereDimensionOneEqual {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                dimensions: where_filters::BoundedStdVecVec::try_from(
                                                    vec![
                                                        postgres_crud_common::NotZeroUnsignedPartOfStdPrimitiveI32::try_from(
                                                            i32::try_from(index_7702518c.checked_add(1)?).expect("5954966c-571a-4744-ba04-9806fc7e63c9")
                                                        ).expect("8d269b8f-41db-4fd9-b33a-e0c532593163")
                                                    ]
                                                ).expect("fe1e037f-70ce-4744-b34b-0413754e6fb0"),
                                                #ValueSc: #ident_standart_not_null_or_nullable_table_type_declaration_ucc(el_081d735b),
                                            }
                                        )
                                    );
                                }
                                acc_74c71d5d
                            }) {
                                Ok(value_2218be19) => Some(value_2218be19),
                                Err(error) => match error {
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("45c8de3c-965b-44a9-b1fd-b7148531da9e")
                                }
                            }
                        }
                    };
                    match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => some_ts,
                        NotNullOrNullable::Nullable => quote! {
                            match #CreateSc.0.0 {
                                Some(value_09152b2e) => #some_ts,
                                None => None
                            }
                        },
                    }
                }
            };
            let postgres_type_option_vec_where_greater_than_test_ts = {
                let greater_than = postgres_crud_common_and_macros_common::PostgresTypeGreaterThanVariant::GreaterThan;
                let not_greater_than = postgres_crud_common_and_macros_common::PostgresTypeGreaterThanVariant::NotGreaterThan;
                let equal_not_greater_than = postgres_crud_common_and_macros_common::PostgresTypeGreaterThanVariant::EqualNotGreaterThan;
                let gen_greater_than_test_ts = |greater_than_variant_ts: &postgres_crud_common_and_macros_common::PostgresTypeGreaterThanVariant, create_content_ts: &dyn quote::ToTokens, table_type_declaration_content_ts: &dyn quote::ToTokens| {
                    quote! {
                        #import_path::PostgresTypeGreaterThanTest {
                            variant: #import_path::PostgresTypeGreaterThanVariant::#greater_than_variant_ts,
                            create: #self_as_postgres_type_ts::Create::#create_content_ts,
                            greater_than: #self_as_postgres_type_ts::TableTypeDeclaration::#table_type_declaration_content_ts,
                        }
                    }
                };
                let gen_greater_than_test_new_new_ts =
                    |greater_than_variant_ts: &postgres_crud_common_and_macros_common::PostgresTypeGreaterThanVariant, create_ts: &dyn quote::ToTokens, greater_than_ts: &dyn quote::ToTokens| gen_greater_than_test_ts(greater_than_variant_ts, &quote! {new(#create_ts)}, &quote! {new(#greater_than_ts)});
                let gen_greater_than_test_try_new_try_new_ts = |greater_than_variant_ts: &postgres_crud_common_and_macros_common::PostgresTypeGreaterThanVariant, create_ts: &dyn quote::ToTokens, greater_than_ts: &dyn quote::ToTokens| {
                    gen_greater_than_test_ts(
                        greater_than_variant_ts,
                        &quote! {try_new(#create_ts).expect("8327c651-9a52-470f-b5ab-dd2680b2f5e1")},
                        &quote! {try_new(#greater_than_ts).expect("c369e6ea-4420-4087-b09a-88f0bbfcb2fe")},
                    )
                };
                let gen_greater_than_test_new_new_vec_ts = |
                    less_ts: &dyn quote::ToTokens,
                    less_with_more_ts: &dyn quote::ToTokens,
                    zero_ts: &dyn quote::ToTokens,
                    one_ts: &dyn quote::ToTokens, more_ts: &dyn quote::ToTokens, more_with_less_ts: &dyn quote::ToTokens| {
                    let greater_than_less_ts = gen_greater_than_test_new_new_ts(&greater_than, &less_with_more_ts, &less_ts);
                    let greater_than_zero_ts = gen_greater_than_test_new_new_ts(&greater_than, &one_ts, &zero_ts);
                    let greater_than_more_ts = gen_greater_than_test_new_new_ts(&greater_than, &more_ts, &more_with_less_ts);
                    let not_greater_than_less_ts = gen_greater_than_test_new_new_ts(&not_greater_than, &less_ts, &less_with_more_ts);
                    let not_greater_than_zero_ts = gen_greater_than_test_new_new_ts(&not_greater_than, &zero_ts, &one_ts);
                    let not_greater_than_more_ts = gen_greater_than_test_new_new_ts(&not_greater_than, &more_with_less_ts, &more_ts);
                    let equal_not_greater_than_less_ts = gen_greater_than_test_new_new_ts(&equal_not_greater_than, &less_ts, &less_ts);
                    let equal_not_greater_than_zero_ts = gen_greater_than_test_new_new_ts(&equal_not_greater_than, &zero_ts, &zero_ts);
                    let equal_not_greater_than_more_ts = gen_greater_than_test_new_new_ts(&equal_not_greater_than, &more_ts, &more_ts);
                    quote! {
                        #greater_than_less_ts,
                        #greater_than_zero_ts,
                        #greater_than_more_ts,
                        #not_greater_than_less_ts,
                        #not_greater_than_zero_ts,
                        #not_greater_than_more_ts,
                        #equal_not_greater_than_less_ts,
                        #equal_not_greater_than_zero_ts,
                        #equal_not_greater_than_more_ts
                    }
                };
                let gen_greater_than_test_try_new_try_new_vec_ts = |
                    less_ts: &dyn quote::ToTokens,
                    less_with_more_ts: &dyn quote::ToTokens,
                    zero_ts: &dyn quote::ToTokens,
                    one_ts: &dyn quote::ToTokens,
                    more_ts: &dyn quote::ToTokens,
                    more_with_less_ts: &dyn quote::ToTokens
                | {
                    let greater_than_less_ts = gen_greater_than_test_try_new_try_new_ts(&greater_than, &less_with_more_ts, &less_ts);
                    let greater_than_zero_ts = gen_greater_than_test_try_new_try_new_ts(&greater_than, &one_ts, &zero_ts);
                    let greater_than_more_ts = gen_greater_than_test_try_new_try_new_ts(&greater_than, &more_ts, &more_with_less_ts);
                    let not_greater_than_less_ts = gen_greater_than_test_try_new_try_new_ts(&not_greater_than, &less_ts, &less_with_more_ts);
                    let not_greater_than_zero_ts = gen_greater_than_test_try_new_try_new_ts(&not_greater_than, &zero_ts, &one_ts);
                    let not_greater_than_more_ts = gen_greater_than_test_try_new_try_new_ts(&not_greater_than, &more_with_less_ts, &more_ts);
                    let equal_not_greater_than_less_ts = gen_greater_than_test_try_new_try_new_ts(&equal_not_greater_than, &less_ts, &less_ts);
                    let equal_not_greater_than_zero_ts = gen_greater_than_test_try_new_try_new_ts(&equal_not_greater_than, &zero_ts, &zero_ts);
                    let equal_not_greater_than_more_ts = gen_greater_than_test_try_new_try_new_ts(&equal_not_greater_than, &more_ts, &more_ts);
                    quote! {
                        #greater_than_less_ts,
                        #greater_than_zero_ts,
                        #greater_than_more_ts,
                        #not_greater_than_less_ts,
                        #not_greater_than_zero_ts,
                        #not_greater_than_more_ts,
                        #equal_not_greater_than_less_ts,
                        #equal_not_greater_than_zero_ts,
                        #equal_not_greater_than_more_ts
                    }
                };
                match &postgres_type_pattern {
                    PostgresTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => {
                            let wrap_into_not_empty_unique_vec_ts = |content_ts: &dyn quote::ToTokens| quote! {Some(
                                #import_path::NotEmptyUniqueVec::try_new(vec![#content_ts]).expect("3ad4b6bf-ba8c-4b14-8745-b0d658e2bdd6")
                            )};
                            let sqlx_types_chrono_naive_time_as_time_standart_not_null_ts = &gen_ident_ts(
                                &PostgresType::SqlxTypesChronoNaiveTimeAsTime,
                                &NotNullOrNullable::NotNull,
                                &PostgresTypePattern::Standart
                            );
                            let sqlx_types_chrono_naive_date_as_date_standart_not_null_ts = &gen_ident_ts(
                                &PostgresType::SqlxTypesChronoNaiveDateAsDate,
                                &NotNullOrNullable::NotNull,
                                &PostgresTypePattern::Standart
                            );
                            match &postgres_type {
                                PostgresType::StdPrimitiveI16AsInt2 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#std_primitive_i16_ts::MIN},
                                    &quote!{#std_primitive_i16_ts::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#std_primitive_i16_ts::MAX},
                                    &quote!{#std_primitive_i16_ts::MAX - 1}
                                )),
                                PostgresType::StdPrimitiveI32AsInt4 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#std_primitive_i32_ts::MIN},
                                    &quote!{#std_primitive_i32_ts::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#std_primitive_i32_ts::MAX},
                                    &quote!{#std_primitive_i32_ts::MAX - 1}
                                )),
                                PostgresType::StdPrimitiveI64AsInt8 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#std_primitive_i64_ts::MIN},
                                    &quote!{#std_primitive_i64_ts::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#std_primitive_i64_ts::MAX},
                                    &quote!{#std_primitive_i64_ts::MAX - 1}
                                )),
                                PostgresType::StdPrimitiveF32AsFloat4 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#std_primitive_f32_ts::MIN},
                                    &quote!{#std_primitive_f32_ts::MIN.next_up()},
                                    &quote!{0.0},
                                    &quote!{1.0},
                                    &quote!{#std_primitive_f32_ts::MAX},
                                    &quote!{#std_primitive_f32_ts::MAX.next_down()}
                                )),
                                PostgresType::StdPrimitiveF64AsFloat8 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                //todo rust f64 != postgres float8
                                    &quote!{-2.0},
                                    &quote!{-2.0 + 1.0},
                                    &quote!{0.0},
                                    &quote!{1.0},
                                    &quote!{2.0},
                                    &quote!{2.0 - 1.0}
                                )),
                                PostgresType::SqlxTypesChronoNaiveTimeAsTime => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{Self::min_inner_type()},
                                    &quote!{Self::slightly_more_than_min_inner_type()},
                                    &quote!{Self::middle_inner_type()},
                                    &quote!{Self::slightly_more_than_middle_inner_type()},
                                    &quote!{Self::max_inner_type()},
                                    &quote!{Self::slightly_less_than_max_inner_type()},
                                )),
                                PostgresType::SqlxTypesTimeTimeAsTime => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{Self::min_inner_type()},
                                    &quote!{Self::slightly_more_than_min_inner_type()},
                                    &quote!{Self::middle_inner_type()},
                                    &quote!{Self::slightly_more_than_middle_inner_type()},
                                    &quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_999).expect("f3d895bb-64a0-47c5-819d-f31b9b5f4ba3")},
                                    &quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_998).expect("1e71f8c6-49a0-47cd-80e4-a4f92666af78")},
                                )),
                                PostgresType::SqlxTypesChronoNaiveDateAsDate => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 30)?},//todo not sure about this values. maybe reuse
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 31)?},
                                    &quote!{Self::middle_inner_type()},
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 2)?},
                                    &quote!{Self::max_inner_type()},
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(262_142, 12, 30)?},
                                )),
                                PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_standart_not_null_ts::min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_standart_not_null_ts::slightly_more_than_min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_standart_not_null_ts::middle_inner_type(),
                                        #sqlx_types_chrono_naive_time_as_time_standart_not_null_ts::min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_standart_not_null_ts::middle_inner_type(),
                                        #sqlx_types_chrono_naive_time_as_time_standart_not_null_ts::slightly_more_than_min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_standart_not_null_ts::max_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_standart_not_null_ts::slightly_less_than_max_inner_type()
                                    )},
                                )),
                                PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |//todo diffrent test logic for autogenerated?
                                PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |//todo diffrent test logic for autogenerated?
                                PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres |//todo diffrent test logic for autogenerated?
                                PostgresType::SqlxPostgresTypesPgMoneyAsMoney |
                                PostgresType::StdPrimitiveBoolAsBool |
                                PostgresType::StdStringStringAsText |
                                PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                                PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                                PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                                PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                                PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                                PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                                PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => none_ts.clone(),
                            }
                        }
                        NotNullOrNullable::Nullable => quote! {
                            <#ident_standart_not_null_ucc as #import_path::PostgresTypeTestCases>::postgres_type_option_vec_where_greater_than_test().map(
                                |el_e4af7fd9|
                                #import_path::NotEmptyUniqueVec::try_new(
                                    el_e4af7fd9
                                    .into_vec()
                                    .into_iter()
                                    .map(|el_504739e6| #import_path::PostgresTypeGreaterThanTest {
                                        variant: el_504739e6.variant,
                                        create: #ident_create_ucc(#ident_origin_ucc(Some(el_504739e6.create.0))),
                                        greater_than: #ident_table_type_declaration_ucc(#ident_origin_ucc(Some(el_504739e6.greater_than.0))),
                                    })
                                    .collect()
                                ).expect("63ce5df3-2b2b-4b2d-be70-0041e6a1cad2")
                            )
                        },
                    },
                    PostgresTypePattern::ArrayDimension1 { .. } => none_ts.clone(),
                }
            };
            let read_only_ids_merged_with_table_type_declaration_into_postgres_type_option_where_greater_than_ts = match &postgres_type_pattern {
                PostgresTypePattern::Standart => {
                    enum IsNeedToImplPostgresTypeGreaterThanTest {
                        False,
                        TrueFromCreate,
                        TrueFromReadOnlyIds,
                    }
                    enum CreateReadOnlyIds {
                        Create,
                        ReadOnlyIds,
                    }
                    let is_need_to_impl_greater_than_test = match &postgres_type {
                        PostgresType::StdPrimitiveI16AsInt2 |
                        PostgresType::StdPrimitiveI32AsInt4 |
                        PostgresType::StdPrimitiveI64AsInt8 |
                        PostgresType::StdPrimitiveF32AsFloat4 |
                        PostgresType::StdPrimitiveF64AsFloat8 |
                        PostgresType::SqlxTypesChronoNaiveTimeAsTime |
                        PostgresType::SqlxTypesTimeTimeAsTime |
                        PostgresType::SqlxTypesChronoNaiveDateAsDate |
                        PostgresType::SqlxTypesChronoNaiveDateTimeAsTimestamp => IsNeedToImplPostgresTypeGreaterThanTest::TrueFromCreate,
                        PostgresType::StdPrimitiveI16AsSmallSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI32AsSerialInitializedByPostgres |
                        PostgresType::StdPrimitiveI64AsBigSerialInitializedByPostgres => IsNeedToImplPostgresTypeGreaterThanTest::TrueFromReadOnlyIds,
                        PostgresType::SqlxPostgresTypesPgMoneyAsMoney |//todo why no support?
                        PostgresType::StdPrimitiveBoolAsBool |
                        PostgresType::StdStringStringAsText |
                        PostgresType::StdVecVecStdPrimitiveU8AsBytea |
                        PostgresType::SqlxPostgresTypesPgIntervalAsInterval |
                        PostgresType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |//todo why no support?
                        PostgresType::SqlxTypesUuidUuidAsUuidV4InitializedByPostgres |
                        PostgresType::SqlxTypesUuidUuidAsUuidInitializedByClient |
                        PostgresType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PostgresType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range |
                        PostgresType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PostgresType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => IsNeedToImplPostgresTypeGreaterThanTest::False,
                    };
                    let gen_some_ts = |value_476d047b: &CreateReadOnlyIds| match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => {
                            let content_ts = match &value_476d047b {
                                CreateReadOnlyIds::ReadOnlyIds => quote! {#ident_standart_not_null_table_type_declaration_ucc(#ReadOnlyIdsSc.0.0)},
                                CreateReadOnlyIds::Create => quote! {table_type_declaration},
                            };
                            quote! {Some(#ident_where_ucc::GreaterThan(
                                where_filters::PostgresTypeWhereGreaterThan {
                                    logical_operator: greater_than_variant.logical_operator(),
                                    #ValueSc: #content_ts,
                                }
                            ))}
                        }
                        NotNullOrNullable::Nullable => {
                            let content_ts = match &value_476d047b {
                                CreateReadOnlyIds::ReadOnlyIds => quote! {#ReadOnlyIdsSc.0},
                                CreateReadOnlyIds::Create => quote! {#TableTypeDeclarationSc.0.0},
                            };
                            quote! {
                                #content_ts.map(|el_886032ca| #ident_where_ucc::GreaterThan(where_filters::PostgresTypeWhereGreaterThan {
                                    logical_operator: greater_than_variant.logical_operator(),
                                    value: #ident_standart_not_null_table_type_declaration_ucc(el_886032ca),
                                }))
                            }
                        }
                    };
                    match &is_need_to_impl_greater_than_test {
                        IsNeedToImplPostgresTypeGreaterThanTest::TrueFromReadOnlyIds => gen_some_ts(&CreateReadOnlyIds::ReadOnlyIds),
                        IsNeedToImplPostgresTypeGreaterThanTest::TrueFromCreate => gen_some_ts(&CreateReadOnlyIds::Create),
                        IsNeedToImplPostgresTypeGreaterThanTest::False => none_ts.clone(),
                    }
                }
                PostgresTypePattern::ArrayDimension1 { .. } => none_ts.clone(),
            };
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_one_equal_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_two_equal_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_three_equal_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_four_equal_ts = none_ts.clone();
            let create_into_postgres_json_type_option_vec_where_length_equal_ts = none_ts.clone();
            let create_into_postgres_json_type_option_vec_where_length_greater_than_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_ts = none_ts;
            postgres_crud_macros_common::gen_impl_postgres_type_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &import_path,
                &ident_inner_type_ts,
                &ident,
                &option_vec_create_ts,
                &read_only_ids_to_two_dimensional_vec_read_inner_ts,
                &read_inner_into_read_with_new_or_try_new_unwraped_ts,
                &read_inner_into_update_with_new_or_try_new_unwraped_ts,
                &update_to_read_only_ids_ts,
                &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
                &previous_read_merged_with_option_update_into_read_ts,
                &read_only_ids_merged_with_create_into_read_ts,
                &read_only_ids_merged_with_create_into_option_value_read_ts,
                &read_only_ids_merged_with_create_into_table_type_declaration_ts,
                &read_only_ids_merged_with_create_into_where_equal_ts,
                &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
                &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts,
                &create_into_postgres_type_option_vec_where_dimension_one_equal_ts,
                &postgres_type_option_vec_where_greater_than_test_ts,
                &read_only_ids_merged_with_table_type_declaration_into_postgres_type_option_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_one_equal_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_two_equal_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_three_equal_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_dimension_four_equal_ts,
                &create_into_postgres_json_type_option_vec_where_length_equal_ts,
                &create_into_postgres_json_type_option_vec_where_length_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_between_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_in_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_regular_expression_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgres_json_type_option_vec_where_contains_el_regular_expression_ts,
            )
        };
        let maybe_impl_postgres_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
            let postgres_type_primary_key_ucc = naming::PostgresTypePrimaryKeyUcc;
            let value_as_read_only_ids_ts = quote! {#ValueSc: #self_as_postgres_type_ts::#ReadOnlyIdsUcc};
            quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                impl #import_path::#postgres_type_primary_key_ucc for #ident_standart_not_null_ucc {
                    type #PostgresTypeUcc = Self;
                    type #TableTypeDeclarationUcc = #ident_standart_not_null_table_type_declaration_ucc;
                    fn #ReadOnlyIdsIntoTableTypeDeclarationSc(#value_as_read_only_ids_ts) -> #self_as_postgres_type_ts::#TableTypeDeclarationUcc {
                        #ident_table_type_declaration_ucc(#ValueSc.0.0)
                    }
                    fn #ReadOnlyIdsIntoReadSc(#value_as_read_only_ids_ts) -> #self_as_postgres_type_ts::#ReadUcc {
                        #ValueSc.0
                    }
                    fn #ReadOnlyIdsIntoUpdateSc(#value_as_read_only_ids_ts) -> #self_as_postgres_type_ts::#UpdateUcc {
                        #ident_update_ucc(#ValueSc.0.0)
                    }
                    fn #ReadIntoTableTypeDeclarationSc(
                        #ValueSc: #self_as_postgres_type_ts::#ReadUcc
                    ) -> #self_as_postgres_type_ts::#TableTypeDeclarationUcc {
                        #ident_table_type_declaration_ucc(#ValueSc.0)
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let maybe_impl_postgres_type_not_primary_key_for_ident_ts = if matches!(&is_not_null_standart_can_be_primary_key, IsNotNullStandartCanBePrimaryKey::True) {
            proc_macro2::TokenStream::new()
        } else {
            postgres_crud_macros_common::gen_impl_postgres_type_not_primary_key_for_ident_ts(&import_path, &ident)
        };
        let generated = quote! {
            #ident_ts
            #ident_origin_ts
            #ident_table_type_declaration_ts
            #ident_create_ts
            #ident_select_ts
            #ident_where_ts
            #ident_read_ts
            #ident_read_only_ids_ts
            #ident_read_inner_ts
            #ident_update_ts
            #ident_update_for_query_ts
            #impl_postgres_type_for_ident_ts
            #impl_postgres_type_test_cases_for_ident_ts
            #maybe_impl_postgres_type_primary_key_for_ident_standart_not_null_if_can_be_primary_key_ts
            #maybe_impl_postgres_type_not_primary_key_for_ident_ts
        };
        (
            {
                let field_ident = format!("column_{index}").parse::<proc_macro2::TokenStream>().expect("2e15af68-48bd-4192-bd45-aacf8086d76b");
                quote! {
                    pub #field_ident: postgres_crud::postgres_type:: #ident,
                }
                .to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    macros_helpers::maybe_write_ts_into_file(
        gen_postgres_json_types_config
            .postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_types,
        "postgres_table_columns_using_postgres_types",
        &{
            let content_ts = columns_ts
                .into_iter()
                .map(|el_2e3fc869| {
                    el_2e3fc869
                        .parse::<proc_macro2::TokenStream>()
                        .expect("79ee6381-c845-4762-a6f6-1c6b38806535")
                })
                .collect::<Vec<proc_macro2::TokenStream>>();
            quote! {
                struct PostgresTableColumnsUsingPostgresTypes {
                    #(#content_ts)*
                }
            }
        },
        &macros_helpers::FormatWithCargofmt::True,
    );
    let generated = {
        let gen_postgres_types_mod_sc = GenPostgresTypesModSc;
        let content_ts = postgres_type_array
            .into_iter()
            .map(|el_f9569807| {
                el_f9569807
                    .parse::<proc_macro2::TokenStream>()
                    .expect("e0c9257d-e554-4147-8174-b431c364c1ac")
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #gen_postgres_types_mod_sc {
                #(#content_ts)*
            }
            pub use #gen_postgres_types_mod_sc::*;
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        gen_postgres_json_types_config.whole_content_write_into_gen_postgres_types,
        "gen_postgres_types",
        &generated,
        &macros_helpers::FormatWithCargofmt::True,
    );
    generated
}
