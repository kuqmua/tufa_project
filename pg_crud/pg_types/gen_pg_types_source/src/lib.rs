use enum_extension_lib::EnumExtension;
use gen_quotes::{dq_str, dq_ts};
use macros_helpers::{
    DeriveCopy, DeriveDefault, DeriveEq, DeriveOrd, DerivePartialOrd, DeriveSerdeDeserialize,
    DeriveSerdeSerialize, FormatWithCargofmt, ShouldWriteTokenStreamIntoFile,
    StructOrEnumDeriveTsStreamBuilder, gen_const_new_ts, gen_if_write_is_err_ts,
    gen_impl_display_ts, gen_impl_from_ts, gen_impl_to_err_string_ts, gen_new_ts,
    gen_pub_const_new_ts, gen_pub_new_ts, gen_pub_try_new_ts, maybe_write_ts_into_file,
};
use naming::{
    ArrOfUcc, AsUcc, ColumnSc, ContainsNullByteUcc, CreateSc, DateNaiveSc, DateNaiveUcc, DateSc,
    DateUcc, DaysSc, DisplayPlusToTokens, EarlierDateNotSupportedUcc, EarliestSupportedDateSc,
    EndSc, EndUcc, EqualUcc, ErSc, ExcludedStartGreaterThanExcludedEndUcc,
    ExcludedStartGreaterThanIncludedEndUcc, ExcludedUcc, GenPgTypesModSc, HourSc,
    IncludedEndCannotBeMaxUcc, IncludedStartGreaterThanExcludedEndUcc,
    IncludedStartGreaterThanIncludedEndUcc, IncludedUcc, IncrSc,
    InvalidHourOrMinuteOrSecondOrMicrosecondUcc, MaxSc, MicroSc, MicrosecondSc, MicrosecondsSc,
    MinSc, MinuteSc, MonthsSc, NanosecondPrecisionIsNotSupportedUcc, NanosecondSc, NearZeroSc,
    NegativeLessTypicalSc, NegativeMoreTypicalSc, NewSc, OptUpdateSc, OptVecCreateSc,
    PgTypePrimaryKeyUcc, PgTypeUcc, PositiveLessTypicalSc, PositiveMoreTypicalSc, QuerySc,
    ReadIntoTableTypeSc, ReadOnlyIdsIntoReadSc, ReadOnlyIdsIntoTableTypeSc,
    ReadOnlyIdsIntoUpdateSc, ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsSc,
    ReadOnlyIdsToTwoDimalVecReadInnerSc, ReadOnlyIdsUcc, ReadSc, ReadUcc, SecSc, SecondSc, SelfSc,
    SelfUcc, StartSc, StartUcc, TableTypeSc, TableTypeUcc, TimeSc, TimeUcc, ToErrStringSc,
    TryNewForDeserializeSc, TryNewSc, UnboundedUcc, UpdateUcc, ValueSc, VecOfUcc,
    param::{
        SelfCreateUcc, SelfNotNullUcc, SelfOriginTryNewErUcc, SelfOriginTryNewForDeserializeErUcc,
        SelfOriginUcc, SelfReadInnerUcc, SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc,
        SelfTableTypeUcc, SelfUpdateForQueryUcc, SelfUpdateUcc, SelfWhereUcc,
    },
};
use panic_location::panic_location;
use pg_crud_common_and_macros_common::PgTypeGreaterThanVrt;
use pg_crud_macros_common::{
    ColumnParamUnderscore, CreateQueryBindValueUnderscore, CreateQueryPartIncrUnderscore,
    CreateQueryPartValueUnderscore, DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, DeriveOrImpl,
    EqualOperatorHandle, ImportPath, IncrParamUnderscore, IsCreateQueryBindMutable,
    IsNeedToAddLogicalOperatorUnderscore, IsNullable, IsPrimaryKeyUnderscore, IsQueryBindMutable,
    IsSelectOnlyUpdatedIdsQueryBindMutable, IsStdrtNotNull, IsUpdateQueryBindMutable, PgFilter,
    PgTypeFilter, ReadOrUpdate, SelectQueryPartValueUnderscore, ShouldDeriveSchemarsJsonSchema,
    ShouldDeriveUtoipaToSchema, UpdateQueryPartJsonbSetAccumulatorUnderscore,
    UpdateQueryPartJsonbSetPathUnderscore, UpdateQueryPartJsonbSetTargetUnderscore,
    UpdateQueryPartValueUnderscore, gen_impl_crate_is_string_empty_for_ident_ts,
    gen_impl_pg_crud_common_default_opt_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts,
    gen_impl_pg_type_not_primary_key_for_ident_ts, gen_impl_pg_type_test_cases_for_ident_ts,
    gen_impl_pg_type_ts, gen_impl_sqlx_decode_sqlx_pg_for_ident_ts,
    gen_impl_sqlx_encode_sqlx_pg_for_ident_ts, gen_impl_sqlx_type_for_ident_ts,
    gen_opt_type_decl_ts, gen_pg_type_where_ts, gen_return_err_query_part_er_write_into_buffer_ts,
    gen_struct_ident_dq_ts, gen_struct_ident_with_nbr_els_dq_ts, gen_tuple_struct_ident_dq_ts,
    gen_value_init_ts, gen_vec_tokens_decl_ts, impl_pg_type_equal_operator_for_ident_ts,
    impl_pg_type_where_filter_for_ident_ts,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    fmt::{Display, Formatter, Result as StdFmtResult},
    iter::{once, repeat_n},
};
use strum_macros::{Display as StrumDisplay, EnumIter};
use token_patterns::{
    AllowClippyArbitrarySourceItemOrdering, CoreDefault, F32, I16, I32, I64, MustUse,
    PgCrudCommonDefaultOptSomeVecOneElCall, PgCrudCommonDefaultOptSomeVecOneElMaxPageSizeCall,
    StringTs, U8, U32,
};
#[must_use]
pub fn gen_pg_types(input_ts: &Ts2) -> Ts2 {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, StrumDisplay)]
    enum RustTypeName {
        I16,
        I32,
        I64,
        F32,
        F64,
        SqlxPgTypesPgMoney,
        Bool,
        String,
        StdVecVecU8,
        SqlxTypesChronoNaiveTime,
        SqlxTypesTimeTime,
        SqlxPgTypesPgInterval,
        SqlxTypesChronoNaiveDate,
        SqlxTypesChronoNaiveDateTime,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        SqlxTypesUuidUuid,
        SqlxTypesIpnetworkIpNetwork,
        SqlxTypesMacAddressMacAddress,
        SqlxPgTypesPgRangeI32,
        SqlxPgTypesPgRangeI64,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
        SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    }
    impl From<&PgType> for RustTypeName {
        fn from(v: &PgType) -> Self {
            match &v {
                PgType::F32AsFloat4 => Self::F32,
                PgType::F64AsFloat8 => Self::F64,
                PgType::I16AsInt2 | PgType::I16AsSmallSerialInitByPg => Self::I16,
                PgType::I32AsInt4 | PgType::I32AsSerialInitByPg => Self::I32,
                PgType::I64AsInt8 | PgType::I64AsBigSerialInitByPg => Self::I64,
                PgType::SqlxPgTypesPgMoneyAsMoney => Self::SqlxPgTypesPgMoney,
                PgType::BoolAsBool => Self::Bool,
                PgType::StringAsText => Self::String,
                PgType::StdVecVecU8AsBytea => Self::StdVecVecU8,
                PgType::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                PgType::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::SqlxPgTypesPgInterval,
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => Self::SqlxTypesUuidUuid,
                PgType::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32,
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, StrumDisplay)]
    enum PgTypeName {
        Int2,
        Int4,
        Int8,
        Float4,
        Float8,
        SmallSerialInitByPg,
        SerialInitByPg,
        BigSerialInitByPg,
        Money,
        Bool,
        Text,
        Bytea,
        Time,
        Interval,
        Date,
        Timestamp,
        TimestampTz,
        UuidV4InitByPg,
        UuidInitByClient,
        Inet,
        MacAddr,
        Int4Range,
        Int8Range,
        DateRange,
        TimestampRange,
        TimestampTzRange,
    }
    impl From<&PgType> for PgTypeName {
        fn from(v: &PgType) -> Self {
            match &v {
                PgType::I16AsInt2 => Self::Int2,
                PgType::I32AsInt4 => Self::Int4,
                PgType::I64AsInt8 => Self::Int8,
                PgType::F32AsFloat4 => Self::Float4,
                PgType::F64AsFloat8 => Self::Float8,
                PgType::I16AsSmallSerialInitByPg => Self::SmallSerialInitByPg,
                PgType::I32AsSerialInitByPg => Self::SerialInitByPg,
                PgType::I64AsBigSerialInitByPg => Self::BigSerialInitByPg,
                PgType::SqlxPgTypesPgMoneyAsMoney => Self::Money,
                PgType::BoolAsBool => Self::Bool,
                PgType::StringAsText => Self::Text,
                PgType::StdVecVecU8AsBytea => Self::Bytea,
                PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => Self::Time,
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::Interval,
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => Self::UuidV4InitByPg,
                PgType::SqlxTypesUuidUuidAsUuidInitByClient => Self::UuidInitByClient,
                PgType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Inet,
                PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::MacAddr,
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::Int4Range,
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::Int8Range,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::DateRange,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::TimestampRange,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::TimestampTzRange,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, Serialize, Deserialize, StrumDisplay, EnumIter, EnumExtension,
    )]
    enum PgType {
        I16AsInt2,
        I32AsInt4,
        I64AsInt8,
        F32AsFloat4,
        F64AsFloat8,
        I16AsSmallSerialInitByPg,
        I32AsSerialInitByPg,
        I64AsBigSerialInitByPg,
        SqlxPgTypesPgMoneyAsMoney,
        // SqlxTypesBigDecimalAsNumeric, remove coz dont know how to deserialize with scale i64
        BoolAsBool,
        StringAsText,
        StdVecVecU8AsBytea,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxPgTypesPgIntervalAsInterval,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxTypesUuidUuidAsUuidV4InitByPg,
        SqlxTypesUuidUuidAsUuidInitByClient,
        SqlxTypesIpnetworkIpNetworkAsInet,
        SqlxTypesMacAddressMacAddressAsMacAddr,
        SqlxPgTypesPgRangeI32AsInt4Range,
        SqlxPgTypesPgRangeI64AsInt8Range,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    fn wrap_into_sqlx_pg_types_pg_range_str(v: &dyn Display) -> String {
        format!("sqlx::postgres::types::PgRange<{v}>")
    }
    enum CanBeNullable {
        False,
        True,
    }
    enum CanBeAnArrEl {
        False,
        True,
    }
    impl PgType {
        const fn can_be_an_arr_el(&self) -> CanBeAnArrEl {
            match &self {
                Self::I16AsInt2
                | Self::I32AsInt4
                | Self::I64AsInt8
                | Self::F32AsFloat4
                | Self::F64AsFloat8
                | Self::SqlxPgTypesPgMoneyAsMoney
                | Self::BoolAsBool
                | Self::StringAsText
                | Self::StdVecVecU8AsBytea
                | Self::SqlxTypesChronoNaiveTimeAsTime
                | Self::SqlxTypesTimeTimeAsTime
                | Self::SqlxPgTypesPgIntervalAsInterval
                | Self::SqlxTypesChronoNaiveDateAsDate
                | Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | Self::SqlxTypesUuidUuidAsUuidInitByClient
                | Self::SqlxTypesIpnetworkIpNetworkAsInet
                | Self::SqlxTypesMacAddressMacAddressAsMacAddr
                | Self::SqlxPgTypesPgRangeI32AsInt4Range
                | Self::SqlxPgTypesPgRangeI64AsInt8Range
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeAnArrEl::True,
                Self::I16AsSmallSerialInitByPg | Self::I32AsSerialInitByPg | Self::I64AsBigSerialInitByPg | Self::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBeAnArrEl::False,
            }
        }
        const fn can_be_nullable(&self) -> CanBeNullable {
            match &self {
                Self::I16AsInt2
                | Self::I32AsInt4
                | Self::I64AsInt8
                | Self::F32AsFloat4
                | Self::F64AsFloat8
                | Self::SqlxPgTypesPgMoneyAsMoney
                | Self::BoolAsBool
                | Self::StringAsText
                | Self::StdVecVecU8AsBytea
                | Self::SqlxTypesChronoNaiveTimeAsTime
                | Self::SqlxTypesTimeTimeAsTime
                | Self::SqlxPgTypesPgIntervalAsInterval
                | Self::SqlxTypesChronoNaiveDateAsDate
                | Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | Self::SqlxTypesUuidUuidAsUuidInitByClient
                | Self::SqlxTypesIpnetworkIpNetworkAsInet
                | Self::SqlxTypesMacAddressMacAddressAsMacAddr
                | Self::SqlxPgTypesPgRangeI32AsInt4Range
                | Self::SqlxPgTypesPgRangeI64AsInt8Range
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeNullable::True,
                Self::I16AsSmallSerialInitByPg | Self::I32AsSerialInitByPg | Self::I64AsBigSerialInitByPg | Self::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBeNullable::False,
            }
        }
    }
    impl ToTokens for PgType {
        fn to_tokens(&self, tokens: &mut Ts2) {
            self.to_string()
                .parse::<Ts2>()
                .expect("cfefbb95")
                .to_tokens(tokens);
        }
    }
    impl From<&Range> for PgType {
        fn from(v: &Range) -> Self {
            match v {
                Range::I32AsInt4 => Self::I32AsInt4,
                Range::I64AsInt8 => Self::I64AsInt8,
                Range::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                Range::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                    Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                }
                Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                    Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum Range {
        I32AsInt4,
        I64AsInt8,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
    }
    impl TryFrom<&PgType> for Range {
        type Error = ();
        fn try_from(v: &PgType) -> Result<Self, Self::Error> {
            match &v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StringAsText
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesChronoNaiveTimeAsTime
                | PgType::SqlxTypesTimeTimeAsTime
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesChronoNaiveDateAsDate
                | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::I32AsInt4),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::I64AsInt8),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
            }
        }
    }
    impl Display for Range {
        fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
            write!(f, "{}", SelfNotNullUcc::from_display(&PgType::from(self)))
        }
    }
    impl ToTokens for Range {
        fn to_tokens(&self, tokens: &mut Ts2) {
            self.to_string()
                .parse::<Ts2>()
                .expect("798ccb5a")
                .to_tokens(tokens);
        }
    }
    // todo reuse it(move to pg_macros_common) if sqlx devs will add nested arr support
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, Serialize, Deserialize, StrumDisplay, EnumIter, EnumExtension,
    )]
    enum PgTypePattern {
        Stdrt,
        ArrDim1 { dim1_is_nullable: IsNullable },
        // sqlx does not support nested arrs yet. https://github.com/launchbadge/sqlx/issues/2280
        // ArrDim2 {
        //     dim1_is_nullable: IsNullable,
        //     dim2_is_nullable: IsNullable,
        // },
        // ArrDim3 {
        //     dim1_is_nullable: IsNullable,
        //     dim2_is_nullable: IsNullable,
        //     dim3_is_nullable: IsNullable,
        // },
        // ArrDim4 {
        //     dim1_is_nullable: IsNullable,
        //     dim2_is_nullable: IsNullable,
        //     dim3_is_nullable: IsNullable,
        //     dim4_is_nullable: IsNullable,
        // },
    }
    impl PgTypePattern {
        const fn arr_dims_nbr(&self) -> usize {
            match &self {
                Self::Stdrt => 0,
                Self::ArrDim1 { .. } => 1,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, Serialize)]
    struct PgTypeRecord {
        pg_type: PgType,
        is_nullable: IsNullable,
        pg_type_pattern: PgTypePattern,
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
        impl<'de> _serde::Deserialize<'de> for PgTypeRecord {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    f0,
                    f1,
                    f2,
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
                    fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match v {
                            0u64 => Ok(__Field::f0),
                            1u64 => Ok(__Field::f1),
                            2u64 => Ok(__Field::f2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match v {
                            "pg_type" => Ok(__Field::f0),
                            "is_nullable" => Ok(__Field::f1),
                            "pg_type_pattern" => Ok(__Field::f2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match v {
                            b"pg_type" => Ok(__Field::f0),
                            b"is_nullable" => Ok(__Field::f1),
                            b"pg_type_pattern" => Ok(__Field::f2),
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
                    marker: _serde::__private228::PhantomData<PgTypeRecord>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PgTypeRecord;
                    fn expecting(
                        &self,
                        __f: &mut Formatter<'_>,
                    ) -> _serde::__private228::fmt::Result {
                        Formatter::write_str(__f, "struct PgTypeRecord")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let Some(f0) = _serde::de::SeqAccess::next_element::<PgType>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct PgTypeRecord with 3 els",
                            ));
                        };
                        let Some(f1) =
                            _serde::de::SeqAccess::next_element::<IsNullable>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct PgTypeRecord with 3 els",
                            ));
                        };
                        let Some(f2) =
                            _serde::de::SeqAccess::next_element::<PgTypePattern>(&mut __seq)?
                        else {
                            return Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct PgTypeRecord with 3 els",
                            ));
                        };
                        match PgTypeRecord::try_from((f0, f1, f2)) {
                            Ok(v) => Ok(v),
                            Err(er) => Err(serde::de::Error::custom(format!("{er:?}"))),
                        }
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut f0: Option<PgType> = None;
                        let mut f1: Option<IsNullable> = None;
                        let mut f2: Option<PgTypePattern> = None;
                        while let Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::f0 => {
                                    if Option::is_some(&f0) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pg_type",
                                            ),
                                        );
                                    }
                                    f0 = Some(_serde::de::MapAccess::next_value::<PgType>(
                                        &mut __map,
                                    )?);
                                }
                                __Field::f1 => {
                                    if Option::is_some(&f1) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "is_nullable",
                                            ),
                                        );
                                    }
                                    f1 = Some(_serde::de::MapAccess::next_value::<IsNullable>(
                                        &mut __map,
                                    )?);
                                }
                                __Field::f2 => {
                                    if Option::is_some(&f2) {
                                        return Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pg_type_pattern",
                                            ),
                                        );
                                    }
                                    f2 = Some(_serde::de::MapAccess::next_value::<PgTypePattern>(
                                        &mut __map,
                                    )?);
                                }
                                __Field::__ignore => {
                                    let _: serde::de::IgnoredAny =
                                        _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                            &mut __map,
                                        )?;
                                }
                            }
                        }
                        let f0_value = match f0 {
                            Some(v) => v,
                            None => _serde::__private228::de::missing_field("pg_type")?,
                        };
                        let f1_value = match f1 {
                            Some(v) => v,
                            None => _serde::__private228::de::missing_field("is_nullable")?,
                        };
                        let f2_value = match f2 {
                            Some(v) => v,
                            None => _serde::__private228::de::missing_field("pg_type_pattern")?,
                        };
                        match PgTypeRecord::try_from((f0_value, f1_value, f2_value)) {
                            Ok(v) => Ok(v),
                            Err(er) => Err(serde::de::Error::custom(format!("{er:?}"))),
                        }
                    }
                }
                #[doc(hidden)]
                const FIELDS: &[&str] = &["pg_type", "is_nullable", "pg_type_pattern"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PgTypeRecord",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Self>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl TryFrom<(PgType, IsNullable, PgTypePattern)> for PgTypeRecord {
        type Error = String;
        fn try_from(v: (PgType, IsNullable, PgTypePattern)) -> Result<Self, Self::Error> {
            let cant_support_nullable_vrts_message = "cant support nullable vrts: ";
            let cant_support_arr_version_message = "cant support arr_version: ";
            match &v.0.can_be_nullable() {
                CanBeNullable::False => {
                    if matches!(&v.1, IsNullable::True) {
                        return Err(format!("{cant_support_nullable_vrts_message}{v:#?}"));
                    }
                    match &v.2 {
                        PgTypePattern::Stdrt => Ok(Self {
                            pg_type: v.0,
                            is_nullable: v.1,
                            pg_type_pattern: v.2,
                        }),
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => {
                            match &v.0.can_be_an_arr_el() {
                                CanBeAnArrEl::False => {
                                    Err(format!("{cant_support_arr_version_message}{v:#?}"))
                                }
                                CanBeAnArrEl::True => match &dim1_is_nullable {
                                    IsNullable::False => Ok(Self {
                                        pg_type: v.0,
                                        is_nullable: v.1,
                                        pg_type_pattern: v.2,
                                    }),
                                    IsNullable::True => {
                                        Err(format!("{cant_support_nullable_vrts_message}{v:#?}"))
                                    }
                                },
                            }
                        }
                    }
                }
                CanBeNullable::True => match &v.2 {
                    PgTypePattern::Stdrt => Ok(Self {
                        pg_type: v.0,
                        is_nullable: v.1,
                        pg_type_pattern: v.2,
                    }),
                    PgTypePattern::ArrDim1 { .. } => match &v.0.can_be_an_arr_el() {
                        CanBeAnArrEl::False => {
                            Err(format!("{cant_support_arr_version_message}{v:#?}"))
                        }
                        CanBeAnArrEl::True => Ok(Self {
                            pg_type: v.0,
                            is_nullable: v.1,
                            pg_type_pattern: v.2,
                        }),
                    },
                },
            }
        }
    }
    #[derive(Debug, serde::Deserialize)]
    enum GenPgTypesConfigVrt {
        All,
        Concrete(Vec<PgTypeRecord>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize)]
    struct GenPgJsonTypesConfig {
        pg_table_columns_write_into_file: ShouldWriteTokenStreamIntoFile,
        whole_write_into_file: ShouldWriteTokenStreamIntoFile,
        vrt: GenPgTypesConfigVrt,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    enum PgTypeInitTryNew {
        StringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxPgTypesPgRangeI32AsInt4Range,
        SqlxPgTypesPgRangeI64AsInt8Range,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    impl TryFrom<&PgType> for PgTypeInitTryNew {
        type Error = ();
        fn try_from(v: &PgType) -> Result<Self, Self::Error> {
            match v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PgType::StringAsText => Ok(Self::StringAsText),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                PgType::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                PgType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::SqlxPgTypesPgRangeI32AsInt4Range),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::SqlxPgTypesPgRangeI64AsInt8Range),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
        }
    }
    impl From<&PgTypeInitTryNew> for PgType {
        fn from(v: &PgTypeInitTryNew) -> Self {
            match v {
                PgTypeInitTryNew::StringAsText => Self::StringAsText,
                PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTimeAsTime,
                PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTimeAsTime,
                PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32AsInt4Range,
                PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64AsInt8Range,
                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    enum PgTypeImplNewForDeserialize {
        SsqlxPgTypesPgIntervalAsInterval, //Ssqlx instead of Sqlx - just to fix clippy lint
        SqlxTypesChronoNaiveDateTimeAsTimestamp,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    enum PgTypeImplTryNewForDeserialize {
        StringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxPgTypesPgRangeI32AsInt4Range,
        SqlxPgTypesPgRangeI64AsInt8Range,
    }
    #[derive(Debug)]
    enum PgTypeImplNewForDeserializeOrTryNewForDeserialize {
        NewForDeserialize(PgTypeImplNewForDeserialize),
        TryNewForDeserialize(PgTypeImplTryNewForDeserialize),
    }
    #[derive(Debug)]
    enum PgTypeDeserialize {
        Derive,
        ImplNewForDeserializeOrTryNewForDeserialize(
            PgTypeImplNewForDeserializeOrTryNewForDeserialize,
        ),
    }
    impl From<&PgType> for PgTypeDeserialize {
        fn from(v: &PgType) -> Self {
            match v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::Derive,
                PgType::StringAsText => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PgTypeImplTryNewForDeserialize::StringAsText)),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime)),
                PgType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PgTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime)),
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PgTypeImplNewForDeserialize::SsqlxPgTypesPgIntervalAsInterval)),
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate)),
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PgTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp)),
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PgTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz)),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI32AsInt4Range)),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI64AsInt8Range)),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange)),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange)),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDeserialize(PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange)),
            }
        }
    }
    panic_location();
    let gen_pg_json_types_config =
        from_str::<GenPgJsonTypesConfig>(&input_ts.to_string()).expect("80485f71");
    let (columns_ts, pg_type_arr) = {
        let acc = match gen_pg_json_types_config.vrt {
            GenPgTypesConfigVrt::All => PgType::into_arr().into_iter().fold(Vec::new(), |mut acc0, el| {
                for el0 in PgTypePattern::into_arr().into_iter().fold(Vec::new(), |mut acc1, el1| {
                    match &el1 {
                        PgTypePattern::Stdrt => {
                            acc1.push(el1);
                        }
                        PgTypePattern::ArrDim1 { .. } => {
                            for el2 in IsNullable::into_arr() {
                                acc1.push(PgTypePattern::ArrDim1 { dim1_is_nullable: el2 });
                            }
                        }
                    }
                    acc1
                }) {
                    match &el0 {
                        PgTypePattern::Stdrt => match &el.can_be_nullable() {
                            CanBeNullable::False => {
                                acc0.push(PgTypeRecord {
                                    pg_type: el.clone(),
                                    is_nullable: IsNullable::False,
                                    pg_type_pattern: el0,
                                });
                            },
                            CanBeNullable::True => IsNullable::into_arr().into_iter().for_each(|el1| {
                                acc0.push(PgTypeRecord {
                                    pg_type: el.clone(),
                                    is_nullable: el1,
                                    pg_type_pattern: el0.clone(),
                                });
                            }),
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => match &el.can_be_an_arr_el() {
                            CanBeAnArrEl::False => (),
                            CanBeAnArrEl::True => match &el.can_be_nullable() {
                                CanBeNullable::False => {
                                    if matches!(&dim1_is_nullable, IsNullable::False) {
                                        for el1 in IsNullable::into_arr() {
                                            acc0.push(PgTypeRecord {
                                                pg_type: el.clone(),
                                                is_nullable: el1,
                                                pg_type_pattern: PgTypePattern::ArrDim1 { dim1_is_nullable: *dim1_is_nullable },
                                            });
                                        }
                                    }
                                },
                                CanBeNullable::True => IsNullable::into_arr().into_iter().for_each(|is_nullable| {
                                    acc0.push(PgTypeRecord {
                                        pg_type: el.clone(),
                                        is_nullable,
                                        pg_type_pattern: el0.clone(),
                                    });
                                }),
                            },
                        },
                    }
                }
                acc0
            }),
            GenPgTypesConfigVrt::Concrete(v) => v,
        };
        {
            let mut check_acc = Vec::new();
            for el in &acc {
                if check_acc.contains(&el) {
                    panic!("536036f9");
                } else {
                    check_acc.push(el);
                }
            }
        }
        acc
    }.into_iter()
    .fold(
        Vec::new(),
        |mut acc, el| {
            #[derive(Clone)]
            struct PgTypeRecordHandle {
                is_nullable: IsNullable,
                pg_type_pattern: PgTypePattern,
            }
            fn gen_pg_type_record_handle_vec(
                pg_type_record_handle: PgTypeRecordHandle,
            ) -> Vec<PgTypeRecordHandle> {
                let gen_vec = |
                    pg_type_record_handle_7ca933c5: PgTypeRecordHandle
                | gen_pg_type_record_handle_vec(pg_type_record_handle_7ca933c5)
                .into_iter()
                .chain(once(pg_type_record_handle.clone()))
                .collect();
                //same pattern was in gen_pg_types 21.05.2025
                match (
                    &pg_type_record_handle.is_nullable,
                    &pg_type_record_handle.pg_type_pattern,
                ) {
                    (IsNullable::False, PgTypePattern::Stdrt) => {
                        vec![pg_type_record_handle]
                    }
                    (IsNullable::True, PgTypePattern::Stdrt) => {
                        gen_vec(PgTypeRecordHandle {
                            is_nullable: IsNullable::False,
                            pg_type_pattern: PgTypePattern::Stdrt,
                        })
                    }
                    (
                        IsNullable::False,
                        PgTypePattern::ArrDim1 {
                            dim1_is_nullable,
                        },
                    ) => gen_vec(PgTypeRecordHandle {
                        is_nullable: *dim1_is_nullable,
                        pg_type_pattern: PgTypePattern::Stdrt,
                    }),
                    (
                        IsNullable::True,
                        PgTypePattern::ArrDim1 { .. },
                    ) => gen_vec(PgTypeRecordHandle {
                        is_nullable: IsNullable::False,
                        pg_type_pattern: pg_type_record_handle
                            .pg_type_pattern
                            .clone(),
                    }),
                }
            }
            for el0 in gen_pg_type_record_handle_vec(PgTypeRecordHandle {
                is_nullable: el.is_nullable,
                pg_type_pattern: el.pg_type_pattern,
            }) {
                let v_88571cb8 = PgTypeRecord {
                    pg_type: el.pg_type.clone(),
                    is_nullable: el0
                        .is_nullable,
                    pg_type_pattern: el0
                        .pg_type_pattern,
                };
                if !acc.contains(&v_88571cb8) {
                    acc.push(v_88571cb8);
                }
            }
            acc
        },
    )
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PgTypeRecord)>>()
    .par_iter()
    //.into_iter() //just for console prints ordering
    .map(|(index, el)| {
        enum PgTypeOrPgTypeTestCases {
            PgType,
            PgTypeTestCases,
        }
        enum CanBePrimaryKey {
            False,
            True,
        }
        enum IsNotNullStdrtCanBePrimaryKey {
            False,
            True,
        }
        enum StartOrEnd {
            End,
            Start,
        }
        enum IntRangeType {
            SqlxPgTypesPgRangeI32AsInt4Range,
            SqlxPgTypesPgRangeI64AsInt8Range,
        }
        type Handle<'lifetime> = (&'lifetime dyn ToTokens, &'lifetime dyn ToTokens);
        fn gen_pg_range_conversion_ts(match_ts: &dyn ToTokens, input_ts: &dyn ToTokens) -> Ts2 {
            quote! {
                sqlx::postgres::types::PgRange {
                    start: match #match_ts.start {
                        std::ops::Bound::Included(v_af65ccce) => std::ops::Bound::Included(#input_ts),
                        std::ops::Bound::Excluded(v_af65ccce) => std::ops::Bound::Excluded(#input_ts),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                    end: match #match_ts.end {
                        std::ops::Bound::Included(v_af65ccce) => std::ops::Bound::Included(#input_ts),
                        std::ops::Bound::Excluded(v_af65ccce) => std::ops::Bound::Excluded(#input_ts),
                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                    },
                }
            }
        }
        let pg_type = &el.pg_type;
        let is_nullable = &el.is_nullable;
        let pg_type_pattern = &el.pg_type_pattern;
        let pg_type_init_try_new_try_from_pg_type = PgTypeInitTryNew::try_from(pg_type);
        let pg_type_deserialize = PgTypeDeserialize::from(pg_type);
        let arr_dims_nbr = pg_type_pattern.arr_dims_nbr();
        let range_try_from_pg_type = Range::try_from(pg_type);
        let range_try_from_pg_type_is_ok = range_try_from_pg_type.is_ok();
        let import_path = ImportPath::PgCrudCommon;
        let import_path_non_primary_key_pg_type_read_only_ids_ts = quote! {#import_path::NonPrimaryKeyPgTypeReadOnlyIds};
        let none_ts = quote!{None};
        let dot_clone_ts = quote!{.clone()};
        let maybe_dot_clone_ts: &dyn ToTokens = if matches!(&pg_type_pattern, PgTypePattern::Stdrt) &&
            matches!(&is_nullable, IsNullable::False) && !matches!(
                pg_type,
                PgType::StdVecVecU8AsBytea | PgType::StringAsText
            )
        {
            &Ts2::new()
        } else {
            &dot_clone_ts
        };
        let gen_import_path_value_init_ts = |ts: &dyn ToTokens| gen_value_init_ts(&import_path, &ts);
        let gen_ident_str = |
            pg_type_73b7c8af: &PgType,
            is_nullable_a5a792df: &IsNullable,
            pg_type_pattern_4f1e15f0: &PgTypePattern
        | {
            let rust_type_name = RustTypeName::from(pg_type_73b7c8af);
            let pg_type_name = PgTypeName::from(pg_type_73b7c8af);
            let is_nullable_rust = is_nullable_a5a792df.rust();
            let not_null_or_nullable_str = is_nullable_a5a792df.not_null_or_nullable_str();
            let (rust_part, pg_part) = match &pg_type_pattern_4f1e15f0 {
                PgTypePattern::Stdrt => (format!("{rust_type_name}"), format!("{pg_type_name}")),
                PgTypePattern::ArrDim1 { dim1_is_nullable } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    (format!("{VecOfUcc}{d1_rust}{rust_type_name}"), format!("{ArrOfUcc}{d1}{pg_type_name}"))
                }
            };
            format!("{is_nullable_rust}{rust_part}{AsUcc}{not_null_or_nullable_str}{pg_part}")
        };
        let gen_ident_ts = |
            pg_type_f8ca5f3f: &PgType,
            is_nullable_ea26dfba: &IsNullable,
            pg_type_pattern_b0eedab6: &PgTypePattern
        | gen_ident_str(
            pg_type_f8ca5f3f,
            is_nullable_ea26dfba,
            pg_type_pattern_b0eedab6
        ).parse::<Ts2>().expect("ff3eb7a6");
        let ident = &gen_ident_ts(pg_type, is_nullable, pg_type_pattern);
        let gen_ident_stdrt_not_null_ts = |pg_type_60cf140e: &PgType| gen_ident_ts(pg_type_60cf140e, &IsNullable::False, &PgTypePattern::Stdrt);
        let ident_stdrt_not_null_ucc = gen_ident_stdrt_not_null_ts(pg_type);
        let ident_stdrt_nullable_ucc = gen_ident_ts(pg_type, &IsNullable::True, &PgTypePattern::Stdrt);
        let ident_arr_not_null_ucc = gen_ident_ts(
            pg_type,
            &IsNullable::False,
            &PgTypePattern::ArrDim1 {
                dim1_is_nullable: IsNullable::False,
            },
        );
        let ident_arr_nullable_ucc = gen_ident_ts(
            pg_type,
            &IsNullable::False,
            &PgTypePattern::ArrDim1 {
                dim1_is_nullable: IsNullable::True,
            },
        );
        let gen_ts = |ts: &dyn ToTokens, pg_type_or_pg_type_test_cases: &PgTypeOrPgTypeTestCases| {
            let trait_ts = match &pg_type_or_pg_type_test_cases {
                PgTypeOrPgTypeTestCases::PgType => quote! {PgType},
                PgTypeOrPgTypeTestCases::PgTypeTestCases => quote! {PgTypeTestCases},
            };
            quote! {<#ts as #import_path::#trait_ts>}
        };
        let gen_as_pg_type_ts = |ts: &dyn ToTokens| gen_ts(&ts, &PgTypeOrPgTypeTestCases::PgType);
        let gen_as_pg_type_test_cases_ts = |ts: &dyn ToTokens| gen_ts(&ts, &PgTypeOrPgTypeTestCases::PgTypeTestCases);
        let self_as_pg_type_ts = gen_as_pg_type_ts(&SelfUcc);
        let ident_stdrt_not_null_as_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_not_null_ucc);
        let ident_stdrt_nullable_as_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_nullable_ucc);
        let self_pg_type_as_pg_type_ts = gen_as_pg_type_ts(&quote! {Self::#PgTypeUcc});
        let ident_stdrt_not_null_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_stdrt_not_null_ucc);
        let ident_stdrt_nullable_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_stdrt_nullable_ucc);
        let ident_arr_not_null_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_arr_not_null_ucc);
        let ident_arr_nullable_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_arr_nullable_ucc);
        let gen_ident_stdrt_not_null_origin_ts = |pg_type_1faa6188: &PgType| SelfOriginUcc::from_tokens(
            &gen_ident_stdrt_not_null_ts(pg_type_1faa6188)
        );
        let ident_stdrt_not_null_origin_ucc = gen_ident_stdrt_not_null_origin_ts(pg_type);
        let ident_origin_ucc = SelfOriginUcc::from_tokens(&ident);
        let ident_stdrt_nullable_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident_stdrt_nullable_ucc);
        let sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc = gen_ident_stdrt_not_null_origin_ts(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc = gen_ident_stdrt_not_null_origin_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc = gen_ident_stdrt_not_null_origin_ts(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc = gen_ident_stdrt_not_null_origin_ts(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let gen_ident_stdrt_not_null_origin_try_new_er_ts = |pg_type_454846c8: &PgType| SelfOriginTryNewErUcc::from_tokens(
            &gen_ident_stdrt_not_null_ts(pg_type_454846c8)
        );
        let sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_er_ucc = gen_ident_stdrt_not_null_origin_try_new_er_ts(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_er_ucc = gen_ident_stdrt_not_null_origin_try_new_er_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_er_ucc = gen_ident_stdrt_not_null_origin_try_new_er_ts(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_er_ucc = gen_ident_stdrt_not_null_origin_try_new_er_ts(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let inner_type_stdrt_not_null_ts = {
            let value = {
                let i16_str = "i16".to_owned();
                let i32_str = "i32".to_owned();
                let i64_str = "i64".to_owned();
                let f32_str = "f32".to_owned();
                let f64_str = "f64".to_owned();
                let sqlx_pg_types_pg_money_str = "sqlx::postgres::types::PgMoney".to_owned();
                let bool_str = "bool".to_owned();
                let string_str = "String".to_owned();
                let vec_u8_str = "Vec<u8>".to_owned();
                let sqlx_types_chrono_naive_date_str = "sqlx::types::chrono::NaiveDate".to_owned();
                let sqlx_types_chrono_naive_time_str = "sqlx::types::chrono::NaiveTime".to_owned();
                let sqlx_types_time_time_str = "sqlx::types::time::Time".to_owned();
                let sqlx_pg_types_pg_interval_str = "sqlx::postgres::types::PgInterval".to_owned();
                let sqlx_types_chrono_naive_date_time_str = "sqlx::types::chrono::NaiveDateTime".to_owned();
                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_str = "sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>".to_owned();
                let uuid_uuid_str = "uuid::Uuid".to_owned();
                let sqlx_types_ipnetwork_ip_network_str = "sqlx::types::ipnetwork::IpNetwork".to_owned();
                let sqlx_types_mac_address_mac_address_str = "sqlx::types::mac_address::MacAddress".to_owned();
                match &pg_type {
                    PgType::F32AsFloat4 => f32_str,
                    PgType::F64AsFloat8 => f64_str,
                    PgType::I16AsInt2 | PgType::I16AsSmallSerialInitByPg => i16_str,
                    PgType::I32AsInt4 | PgType::I32AsSerialInitByPg => i32_str,
                    PgType::I64AsInt8 | PgType::I64AsBigSerialInitByPg => i64_str,
                    PgType::SqlxPgTypesPgMoneyAsMoney => sqlx_pg_types_pg_money_str,
                    PgType::BoolAsBool => bool_str,
                    PgType::StringAsText => string_str,
                    PgType::StdVecVecU8AsBytea => vec_u8_str,
                    PgType::SqlxTypesChronoNaiveTimeAsTime => sqlx_types_chrono_naive_time_str,
                    PgType::SqlxTypesTimeTimeAsTime => sqlx_types_time_time_str,
                    PgType::SqlxPgTypesPgIntervalAsInterval => sqlx_pg_types_pg_interval_str,
                    PgType::SqlxTypesChronoNaiveDateAsDate => sqlx_types_chrono_naive_date_str,
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => sqlx_types_chrono_naive_date_time_str,
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => sqlx_types_chrono_date_time_sqlx_types_chrono_utc_str,
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => uuid_uuid_str,
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet => sqlx_types_ipnetwork_ip_network_str,
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => sqlx_types_mac_address_mac_address_str,
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => wrap_into_sqlx_pg_types_pg_range_str(&i32_str),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => wrap_into_sqlx_pg_types_pg_range_str(&i64_str),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => wrap_into_sqlx_pg_types_pg_range_str(&sqlx_types_chrono_naive_date_str),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => wrap_into_sqlx_pg_types_pg_range_str(&sqlx_types_chrono_naive_date_time_str),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => wrap_into_sqlx_pg_types_pg_range_str(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_str),
                }
            };
            value.parse::<Ts2>().expect("2555843f")
        };
        let gen_ident_origin_non_wrapping_8ad5380a = |pg_type_pattern_94bed782: &PgTypePattern, is_nullable_28c189b3: &IsNullable| SelfOriginUcc::from_tokens(&gen_ident_ts(pg_type, is_nullable_28c189b3, pg_type_pattern_94bed782));
        let ft_handle: &dyn ToTokens = {
            match &pg_type_pattern {
                PgTypePattern::Stdrt => match &is_nullable {
                    IsNullable::False => &inner_type_stdrt_not_null_ts,
                    IsNullable::True => &gen_opt_type_decl_ts(&ident_stdrt_not_null_origin_ucc),
                },
                PgTypePattern::ArrDim1 { dim1_is_nullable } => &{
                    let (pg_type_pattern_7790d04a, is_nullable_86d888a6): (&PgTypePattern, &IsNullable) = match &is_nullable {
                        IsNullable::False => (&PgTypePattern::Stdrt, dim1_is_nullable),
                        IsNullable::True => (pg_type_pattern, &IsNullable::False),
                    };
                    let value = gen_ident_origin_non_wrapping_8ad5380a(pg_type_pattern_7790d04a, is_nullable_86d888a6);
                    match &is_nullable {
                        IsNullable::False => gen_vec_tokens_decl_ts(&value),
                        IsNullable::True => gen_opt_type_decl_ts(&value),
                    }
                },
            }
        };
        let gen_typical_query_bind_ts = |ts: &dyn ToTokens| match &is_nullable {
            IsNullable::False => quote! {
                if let Err(er) = #QuerySc.try_bind(#ts) {
                    return Err(er.to_string());
                }
                Ok(#QuerySc)
            },
            IsNullable::True => quote! {
                if let Err(er) = #QuerySc.try_bind(#ts.0.0) {
                    return Err(er.to_string());
                }
                Ok(#QuerySc)
            },
        };
        let typical_query_bind_ts = gen_typical_query_bind_ts(&ValueSc);
        let ident_inner_type_ts = match &el.pg_type_pattern {
            PgTypePattern::Stdrt => match &is_nullable {
                IsNullable::False => &inner_type_stdrt_not_null_ts,
                IsNullable::True => &gen_opt_type_decl_ts(&inner_type_stdrt_not_null_ts),
            },
            PgTypePattern::ArrDim1 { dim1_is_nullable } => &{
                let dim1_type = dim1_is_nullable.maybe_opt_wrap(quote! {#inner_type_stdrt_not_null_ts});
                is_nullable.maybe_opt_wrap(gen_vec_tokens_decl_ts(&dim1_type))
            },
        };
        let can_be_primary_key = match &pg_type {
            PgType::I16AsInt2
            | PgType::I32AsInt4
            | PgType::I64AsInt8
            | PgType::F32AsFloat4
            | PgType::F64AsFloat8
            | PgType::SqlxPgTypesPgMoneyAsMoney
            | PgType::BoolAsBool
            | PgType::StringAsText
            | PgType::StdVecVecU8AsBytea
            | PgType::SqlxTypesChronoNaiveTimeAsTime
            | PgType::SqlxTypesTimeTimeAsTime
            | PgType::SqlxPgTypesPgIntervalAsInterval
            | PgType::SqlxTypesChronoNaiveDateAsDate
            | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
            | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            | PgType::SqlxTypesUuidUuidAsUuidInitByClient
            | PgType::SqlxTypesIpnetworkIpNetworkAsInet
            | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
            | PgType::SqlxPgTypesPgRangeI32AsInt4Range
            | PgType::SqlxPgTypesPgRangeI64AsInt8Range
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePrimaryKey::False,
            PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBePrimaryKey::True,
        };
        let is_stdrt_not_null = if matches!((&pg_type_pattern, &is_nullable), (PgTypePattern::Stdrt, IsNullable::False)) {
            IsStdrtNotNull::True
        } else {
            IsStdrtNotNull::False
        };
        let is_not_null_stdrt_can_be_primary_key = if matches!((&is_nullable, &pg_type_pattern, &can_be_primary_key), (IsNullable::False, PgTypePattern::Stdrt, CanBePrimaryKey::True)) {
            IsNotNullStdrtCanBePrimaryKey::True
        } else {
            IsNotNullStdrtCanBePrimaryKey::False
        };
        let gen_start_or_end_ucc = |start_or_end: &StartOrEnd| -> &dyn DisplayPlusToTokens {
            match &start_or_end {
                StartOrEnd::End => &EndUcc,
                StartOrEnd::Start => &StartUcc,
            }
        };
        let gen_start_or_end_sc = |start_or_end: &StartOrEnd| -> &dyn DisplayPlusToTokens {
            match &start_or_end {
                StartOrEnd::End => &EndSc,
                StartOrEnd::Start => &StartSc,
            }
        };
        let (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl) = if matches!(&is_stdrt_not_null, IsStdrtNotNull::True) {
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum ParamNbr {
                One,
                Two,
                Three,
                Four,
            }
            impl ParamNbr {
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
            let param_nbr_one = ParamNbr::One;
            let param_nbr_two = ParamNbr::Two;
            let param_nbr_three = ParamNbr::Three;
            let param_nbr_four = ParamNbr::Four;
            let ident_stdrt_not_null_dq_ts = dq_ts(&ident_stdrt_not_null_ucc);
            let ident_stdrt_not_null_origin_dq_ts = dq_ts(&ident_stdrt_not_null_origin_ucc);
            let gen_std_ops_bound_ts = |type_ts: &dyn ToTokens| {
                quote! {std::ops::Bound<#type_ts>}
            };
            let std_ops_bound_i32_ts = gen_std_ops_bound_ts(&I32);
            let std_ops_bound_i64_ts = gen_std_ops_bound_ts(&I64);
            let std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts = gen_std_ops_bound_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc);
            let std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts = gen_std_ops_bound_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc);
            let std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts = gen_std_ops_bound_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc);
            let serde_serialize_derive_or_impl = {
                let gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens = |ts: &dyn ToTokens| {
                    quote! {
                        #[allow(unused_qualifications)]
                        #[allow(clippy::absolute_paths)]
                        #AllowClippyArbitrarySourceItemOrdering
                        const _: () = {
                            #[allow(unused_extern_crates, clippy::useless_attribute)]
                            extern crate serde as _serde;
                            #[automatically_derived]
                            impl _serde::Serialize for #ident_stdrt_not_null_origin_ucc {
                                fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    #ts
                                }
                            }
                        };
                    }
                };
                let gen_serde_serialize_content_b5af560e = |value_ts: &dyn ToTokens| {
                    quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #ident_stdrt_not_null_origin_dq_ts, &#self_dot_zero_ts #value_ts)}
                };
                let gen_serde_state_init_ts = |param_nbr: &ParamNbr| {
                    let param_nbr_ts = {
                        let value = param_nbr.get_vec_from_index_starting_with_zero().into_iter().map(|_| quote! {+ 1});
                        quote! {#(#value)*}
                    };
                    quote! {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_stdrt_not_null_origin_dq_ts, usize::from(false) #param_nbr_ts)?;
                    }
                };
                let serde_state_init_two_fields_ts = gen_serde_state_init_ts(&param_nbr_two);
                let serde_state_init_three_fields_ts = gen_serde_state_init_ts(&param_nbr_three);
                let serde_state_init_four_fields_ts = gen_serde_state_init_ts(&param_nbr_four);
                let gen_serialize_field_ts = |field_name: &dyn Display, third_param_ts: &dyn ToTokens| {
                    let field_name_dq_ts = dq_ts(&field_name);
                    quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_dq_ts, #third_param_ts)?;}
                };
                let serde_ser_serialize_struct_end_ts = quote! {_serde::ser::SerializeStruct::end(__serde_state)};
                let serde_serialize_content_e5bb5640_ts = {
                    let gen_self_zero_tokens_ts = |value_ts: &dyn ToTokens| {
                        quote! {&#self_dot_zero_ts.#value_ts}
                    };
                    let start_serialize_field_ts = gen_serialize_field_ts(&StartSc, &gen_self_zero_tokens_ts(&StartSc));
                    let end_serialize_field_ts = gen_serialize_field_ts(&EndSc, &gen_self_zero_tokens_ts(&EndSc));
                    quote! {
                        #serde_state_init_two_fields_ts
                        #start_serialize_field_ts
                        #end_serialize_field_ts
                        #serde_ser_serialize_struct_end_ts
                    }
                };
                let impl_serde_serialize_for_pg_type_not_null_tokens_serde_serialize_content_e5bb5640_ts = gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&serde_serialize_content_e5bb5640_ts);
                let impl_serde_serialize_for_uuid_uuid_ts = gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&gen_serde_serialize_content_b5af560e(&quote! {.to_string()}));
                let gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_start_end_range_tokens = |ident_ts_ba0919c4: &dyn ToTokens| {
                    let gen_serialize_field_match_std_ops_bound_ts = |start_or_end: &StartOrEnd| {
                        let start_or_end_ts = gen_start_or_end_sc(start_or_end);
                        gen_serialize_field_ts(
                            &start_or_end_ts,
                            &quote! {
                                &match self.0.#start_or_end_ts {
                                    std::ops::Bound::Included(#ValueSc) => std::ops::Bound::Included(#ident_ts_ba0919c4::#TryNewSc(#ValueSc).map_err(_serde::ser::Error::custom)?),
                                    std::ops::Bound::Excluded(#ValueSc) => std::ops::Bound::Excluded(#ident_ts_ba0919c4::#TryNewSc(#ValueSc).map_err(_serde::ser::Error::custom)?),
                                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                }
                            },
                        )
                    };
                    let start_serialize_field_ts = gen_serialize_field_match_std_ops_bound_ts(&StartOrEnd::Start);
                    let end_serialize_field_ts = gen_serialize_field_match_std_ops_bound_ts(&StartOrEnd::End);
                    gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&quote! {
                        #serde_state_init_two_fields_ts
                        #start_serialize_field_ts
                        #end_serialize_field_ts
                        #serde_ser_serialize_struct_end_ts
                    })
                };
                let gen_b5af560e_ts = |ts: &dyn ToTokens|{
                    DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(
                        &gen_serde_serialize_content_b5af560e(&ts)
                    ))
                };
                match &pg_type {
                    PgType::I16AsInt2
                    | PgType::I32AsInt4
                    | PgType::I64AsInt8
                    | PgType::F32AsFloat4
                    | PgType::F64AsFloat8
                    | PgType::I16AsSmallSerialInitByPg
                    | PgType::I32AsSerialInitByPg
                    | PgType::I64AsBigSerialInitByPg
                    | PgType::BoolAsBool
                    | PgType::StringAsText
                    | PgType::StdVecVecU8AsBytea
                    | PgType::SqlxTypesChronoNaiveDateAsDate
                    | PgType::SqlxTypesIpnetworkIpNetworkAsInet => DeriveOrImpl::Derive,
                    PgType::SqlxPgTypesPgMoneyAsMoney => gen_b5af560e_ts(&quote! {.0}),
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_b5af560e_ts(&quote! {.bytes()}),
                    PgType::SqlxTypesChronoNaiveTimeAsTime => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&{
                        let gen_field_inner_type_stdrt_not_null_ts_as_chrono_timelike_ts = |ts: &dyn ToTokens| {
                            quote! {&(<#inner_type_stdrt_not_null_ts as chrono::Timelike>::#ts)}
                        };
                        let hour_serialize_field_ts = gen_serialize_field_ts(&HourSc, &gen_field_inner_type_stdrt_not_null_ts_as_chrono_timelike_ts(&quote! {hour(&self.0)}));
                        let min_serialize_field_ts = gen_serialize_field_ts(&MinSc, &gen_field_inner_type_stdrt_not_null_ts_as_chrono_timelike_ts(&quote! {minute(&self.0)}));
                        let sec_serialize_field_ts = gen_serialize_field_ts(&SecSc, &gen_field_inner_type_stdrt_not_null_ts_as_chrono_timelike_ts(&quote! {second(&self.0)}));
                        let micro_serialize_field_ts = gen_serialize_field_ts(
                            &MicroSc,
                            &gen_field_inner_type_stdrt_not_null_ts_as_chrono_timelike_ts(&quote! {
                                #NanosecondSc(&self.0).checked_div(1000).expect("aea037b7")
                            }),
                        );
                        quote! {
                            #serde_state_init_four_fields_ts
                            #hour_serialize_field_ts
                            #min_serialize_field_ts
                            #sec_serialize_field_ts
                            #micro_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PgType::SqlxTypesTimeTimeAsTime => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&{
                        let gen_serialize_field_self_zero_ts = |v: &dyn DisplayPlusToTokens| gen_serialize_field_ts(&v, &quote! {&self.0.#v()});
                        let hour_serialize_field_ts = gen_serialize_field_self_zero_ts(&HourSc);
                        let minute_serialize_field_ts = gen_serialize_field_self_zero_ts(&MinuteSc);
                        let second_serialize_field_ts = gen_serialize_field_self_zero_ts(&SecondSc);
                        let microsecond_serialize_field_ts = gen_serialize_field_self_zero_ts(&MicrosecondSc);
                        quote! {
                            #serde_state_init_four_fields_ts
                            #hour_serialize_field_ts
                            #minute_serialize_field_ts
                            #second_serialize_field_ts
                            #microsecond_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PgType::SqlxPgTypesPgIntervalAsInterval => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&{
                        let gen_serialize_field_handle_ts = |v: &dyn DisplayPlusToTokens| gen_serialize_field_ts(&v, &quote! {&#self_dot_zero_ts.#v});
                        let months_serialize_field_ts = gen_serialize_field_handle_ts(&MonthsSc);
                        let days_serialize_field_ts = gen_serialize_field_handle_ts(&DaysSc);
                        let microseconds_serialize_field_ts = gen_serialize_field_handle_ts(&MicrosecondsSc);
                        quote! {
                            #serde_state_init_three_fields_ts
                            #months_serialize_field_ts
                            #days_serialize_field_ts
                            #microseconds_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&{
                        enum DateOrTime {
                            Date,
                            Time,
                        }
                        let gen_serialize_field_try_new_unwrap_ts = |date_or_time: &DateOrTime| {
                            let date_or_time_ts: &dyn DisplayPlusToTokens = match &date_or_time {
                                DateOrTime::Date => &DateSc,
                                DateOrTime::Time => &TimeSc,
                            };
                            gen_serialize_field_ts(&date_or_time_ts, &{
                                let ident_ts_203ac73c: &dyn ToTokens = match &date_or_time {
                                    DateOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                    DateOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc,
                                };
                                quote! {
                                    &match #ident_ts_203ac73c::#TryNewSc(self.0.#date_or_time_ts()) {
                                        Ok(v_b2ac0c33) => v_b2ac0c33,
                                        Err(er) => {
                                            return Err(_serde::ser::Error::custom(er));
                                        },
                                    }
                                }
                            })
                        };
                        let date_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateOrTime::Date);
                        let time_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateOrTime::Time);
                        quote! {
                            #serde_state_init_two_fields_ts
                            #date_serialize_field_ts
                            #time_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_tokens(&{
                        enum DateNaiveOrTime {
                            Date,
                            Time,
                        }
                        let gen_serialize_field_try_new_unwrap_ts = |date_naive_or_time: &DateNaiveOrTime| {
                            let date_naive_or_time_ts: &dyn DisplayPlusToTokens = match &date_naive_or_time {
                                DateNaiveOrTime::Date => &DateNaiveSc,
                                DateNaiveOrTime::Time => &TimeSc,
                            };
                            gen_serialize_field_ts(&date_naive_or_time_ts, &{
                                let ident_ts_d3be24b2: &dyn ToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                    DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc,
                                };
                                quote! {&#ident_ts_d3be24b2::#TryNewSc(self.0.#date_naive_or_time_ts()).map_err(_serde::ser::Error::custom)?}
                            })
                        };
                        let date_naive_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateNaiveOrTime::Date);
                        let time_serialize_field_ts = gen_serialize_field_try_new_unwrap_ts(&DateNaiveOrTime::Time);
                        quote! {
                            #serde_state_init_two_fields_ts
                            #date_naive_serialize_field_ts
                            #time_serialize_field_ts
                            #serde_ser_serialize_struct_end_ts
                        }
                    })),
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => DeriveOrImpl::Impl(impl_serde_serialize_for_uuid_uuid_ts),
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => DeriveOrImpl::Impl(impl_serde_serialize_for_pg_type_not_null_tokens_serde_serialize_content_e5bb5640_ts),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc)),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc)),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DeriveOrImpl::Impl(gen_impl_serde_serialize_for_ident_stdrt_not_null_origin_start_end_range_tokens(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc)),
                }
            };
            let serde_deserialize_derive_or_impl = {
                enum ShouldAddDeLifetime {
                    False,
                    True,
                }
                let struct_ident_dq_ts = gen_struct_ident_dq_ts(&ident_origin_ucc);
                let tuple_struct_ident_dq_ts = gen_tuple_struct_ident_dq_ts(&ident_origin_ucc);
                let struct_visitor_ts = quote! {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private228::PhantomData<#ident_stdrt_not_null_origin_ucc>,
                        lifetime: serde::__private228::PhantomData<&'de ()>,
                    }
                };
                let start_end_display_plus_to_tokens_arr: [&dyn DisplayPlusToTokens; 2] = [&StartSc, &EndSc];
                let hour_min_sec_micro_display_plus_to_tokens_arr: [&dyn DisplayPlusToTokens; 4] = [&HourSc, &MinSc, &SecSc, &MicroSc];
                let hour_minute_second_microsecond_display_plus_to_tokens_arr: [&dyn DisplayPlusToTokens; 4] = [&HourSc, &MinuteSc, &SecondSc, &MicrosecondSc];
                let date_time_display_plus_to_tokens_arr: [&dyn DisplayPlusToTokens; 2] = [&DateSc, &TimeSc];
                let date_naive_time_display_plus_to_tokens_arr: [&dyn DisplayPlusToTokens; 2] = [&DateNaiveSc, &TimeSc];
                let months_days_microseconds_display_plus_to_tokens_arr: [&dyn DisplayPlusToTokens; 3] = [&MonthsSc, &DaysSc, &MicrosecondsSc];
                let serde_deserializer_deserialize_struct_visitor_ts = {
                    quote! {
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #ident_stdrt_not_null_dq_ts,
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
                        #ident_stdrt_not_null_origin_dq_ts,
                        __Visitor {
                            marker: serde::__private228::PhantomData::<Self>,
                            lifetime: serde::__private228::PhantomData,
                        },
                    )
                };
                let gen_impl_serde_deserialize_for_tokens_ts = |ts: &dyn ToTokens| {
                    quote! {
                        #[allow(unused_qualifications)]
                        #[allow(clippy::absolute_paths)]
                        #AllowClippyArbitrarySourceItemOrdering
                        const _: () = {
                            #[allow(unused_extern_crates, clippy::useless_attribute)]
                            extern crate serde as _serde;
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for #ident_stdrt_not_null_origin_ucc {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    #ts
                                }
                            }
                        };
                    }
                };
                let gen_field_index_ts = |index_52391f7d: usize| format!("f{index_52391f7d}").parse::<Ts2>().expect("a4e1a63f");
                let gen_field_index_value_ts = |index_7ef2fc7d: usize| format!("f{index_7ef2fc7d}_value").parse::<Ts2>().expect("fa97be6c");
                let (enum_field_two_ts, enum_field_three_ts, enum_field_four_ts) = {
                    let gen_enum_field_ts = |param_nbr: &ParamNbr| {
                        let fields_ts = {
                            let fields_ts = param_nbr.get_vec_from_index_starting_with_zero().into_iter().map(&gen_field_index_ts);
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
                    (gen_enum_field_ts(&param_nbr_two), gen_enum_field_ts(&param_nbr_three), gen_enum_field_ts(&param_nbr_four))
                };
                let (fn_expecting_struct_ident_dq_ts, fn_expecting_tuple_struct_ident_dq_ts, fn_expecting_fi_ts) = {
                    let gen_fn_expecting_ts = |ts: &dyn ToTokens| {
                        quote! {
                            fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                serde::__private228::Formatter::write_str(__f, #ts)
                            }
                        }
                    };
                    (gen_fn_expecting_ts(&struct_ident_dq_ts), gen_fn_expecting_ts(&tuple_struct_ident_dq_ts), gen_fn_expecting_ts(&quote! {"field identifier"}))
                };
                let field_0_value_ts = gen_field_index_value_ts(param_nbr_one.get_index());
                let gen_serde_private_ok_ts = |ts: &dyn ToTokens| {
                    quote! {Ok(#ts)}
                };
                let gen_serde_private_ok_pg_type_ts = |ts: &dyn ToTokens| gen_serde_private_ok_ts(&quote! {#ident_stdrt_not_null_origin_ucc(#ts)});
                let match_uuid_uuid_ft_try_parse_ts = quote! {match #inner_type_stdrt_not_null_ts::try_parse(&#field_0_value_ts) {
                    Ok(v_3c0b34fb) => v_3c0b34fb,
                    Err(er) => {
                        return Err(serde::de::Error::custom(er));
                    }
                }};
                let sqlx_types_mac_address_mac_address_ft_new_field_0_value_ts = quote! {#inner_type_stdrt_not_null_ts::#NewSc(#field_0_value_ts)};
                let arr_u8_6_ts = quote! {[u8; 6]};
                let gen_vec_field_index_ts = |length: usize|{
                    let fields_ts = (1..=length).collect::<Vec<_>>().into_iter().enumerate().map(|(index_a8d5119e, _)| gen_field_index_value_ts(index_a8d5119e));
                    quote!{#(#fields_ts),*}
                };
                let (sqlx_types_chrono_naive_time_origin_try_new_for_deserialize, match_origin_try_new_for_deserialize_one_ts, match_origin_try_new_for_deserialize_two_ts, match_origin_try_new_for_deserialize_four_ts) = {
                    let gen_match_origin_try_new_for_deserialize_ts = |length: usize| {
                        let fields_ts = gen_vec_field_index_ts(length);
                        quote! {
                            match #ident_stdrt_not_null_origin_ucc::#TryNewForDeserializeSc(#fields_ts) {
                                Ok(v_e81dd4a5) => Ok(v_e81dd4a5),
                                Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                            }
                        }
                    };
                    (
                        gen_match_origin_try_new_for_deserialize_ts(hour_min_sec_micro_display_plus_to_tokens_arr.len()),
                        gen_match_origin_try_new_for_deserialize_ts(1),
                        gen_match_origin_try_new_for_deserialize_ts(2),
                        gen_match_origin_try_new_for_deserialize_ts(4),
                    )
                };
                let (origin_new_for_deserialize_two_ts, origin_new_for_deserialize_three_ts) = {
                    let gen_origin_new_for_deserialize_ts = |length: usize| {
                        let fields_ts = gen_vec_field_index_ts(length);
                        quote! {
                            Ok(#ident_stdrt_not_null_origin_ucc::new_for_deserialize(#fields_ts))
                        }
                    };
                    (gen_origin_new_for_deserialize_ts(2), gen_origin_new_for_deserialize_ts(3))
                };
                let (fn_visit_newtype_struct_pg_money_ts, fn_visit_newtype_struct_uuid_ts, fn_visit_newtype_struct_mac_address_ts, fn_visit_newtype_struct_text_ts, fn_visit_newtype_struct_sqlx_types_chrono_naive_date_ts) = {
                    let gen_fn_visit_newtype_struct_ts = |type_ts: &dyn ToTokens, serde_private_ok_ts: &dyn ToTokens| {
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
                        gen_fn_visit_newtype_struct_ts(&I64, &gen_serde_private_ok_pg_type_ts(&quote! {#inner_type_stdrt_not_null_ts(#field_0_value_ts)})),
                        gen_fn_visit_newtype_struct_ts(&StringTs, &gen_serde_private_ok_pg_type_ts(&match_uuid_uuid_ft_try_parse_ts)),
                        gen_fn_visit_newtype_struct_ts(&arr_u8_6_ts, &gen_serde_private_ok_pg_type_ts(&sqlx_types_mac_address_mac_address_ft_new_field_0_value_ts)),
                        gen_fn_visit_newtype_struct_ts(&StringTs, &match_origin_try_new_for_deserialize_one_ts),
                        gen_fn_visit_newtype_struct_ts(&inner_type_stdrt_not_null_ts, &match_origin_try_new_for_deserialize_one_ts),
                    )
                };
                let gen_fields_serde_de_seq_access_next_el_init_ts = |vec_ts: &[&dyn ToTokens]| {
                    let er_message_ts = gen_struct_ident_with_nbr_els_dq_ts(&ident_stdrt_not_null_origin_ucc, vec_ts.len());
                    let fields_init_ts = vec_ts.iter().enumerate().map(|(index_70b4dabd, el0)| {
                        let field_index_value_ts = gen_field_index_value_ts(index_70b4dabd);
                        let index_usize_ts = format!("{index_70b4dabd}usize").parse::<Ts2>().expect("ce15e6bf");
                        quote! {
                            let Some(#field_index_value_ts) = serde::de::SeqAccess::next_element::<#el0>(&mut __seq)? else {
                                return Err(serde::de::Error::invalid_length(#index_usize_ts, &#er_message_ts));
                            };
                        }
                    });
                    quote! {#(#fields_init_ts)*}
                };
                let (
                    fn_visit_seq_pg_money_ts,
                    fn_visit_seq_sqlx_types_chrono_naive_time_ts,
                    fn_visit_seq_uuid_uuid_ts,
                    fn_visit_seq_sqlx_types_mac_address_mac_address_ts,
                    fn_visit_seq_string_ts,
                    fn_visit_seq_sqlx_types_time_time_ts,
                    fn_visit_seq_sqlx_types_chrono_naive_date_ts,
                    fn_visit_seq_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_seq_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_seq_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                    fn_visit_seq_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_seq_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_seq_sqlx_pg_types_pg_range_i32_ts,
                    fn_visit_seq_sqlx_pg_types_pg_range_i64_ts,
                    fn_visit_seq_sqlx_pg_types_pg_interval_ts,
                ) = {
                    let gen_fn_visit_seq_ts = |ts: &dyn ToTokens| {
                        quote! {
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::SeqAccess<'de>,
                            {
                                #ts
                            }
                        }
                    };
                    (
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&I64]);
                            let serde_private_ok_pg_type_ts = gen_serde_private_ok_pg_type_ts(&quote! {#inner_type_stdrt_not_null_ts(#field_0_value_ts)});
                            quote! {
                                #fields_init_ts
                                #serde_private_ok_pg_type_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&U32, &U32, &U32, &U32]);
                            quote! {
                                #fields_init_ts
                                #sqlx_types_chrono_naive_time_origin_try_new_for_deserialize
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&StringTs]);
                            let serde_private_ok_pg_type_ts = gen_serde_private_ok_pg_type_ts(&match_uuid_uuid_ft_try_parse_ts);
                            quote! {
                                #fields_init_ts
                                #serde_private_ok_pg_type_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&arr_u8_6_ts]);
                            let serde_private_ok_pg_type_ts = gen_serde_private_ok_pg_type_ts(&sqlx_types_mac_address_mac_address_ft_new_field_0_value_ts);
                            quote! {
                                #fields_init_ts
                                #serde_private_ok_pg_type_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&StringTs]);
                            quote! {
                                #fields_init_ts
                                #match_origin_try_new_for_deserialize_one_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&U8, &U8, &U8, &U32]);
                            quote! {
                                #fields_init_ts
                                #match_origin_try_new_for_deserialize_four_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&inner_type_stdrt_not_null_ts]);
                            quote! {
                                #fields_init_ts
                                #match_origin_try_new_for_deserialize_one_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]);
                            quote! {
                                #fields_init_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]);
                            quote! {
                                #fields_init_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts]);
                            quote! {
                                #fields_init_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts]);
                            quote! {
                                #fields_init_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts]);
                            quote! {
                                #fields_init_ts
                                #origin_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&std_ops_bound_i32_ts, &std_ops_bound_i32_ts]);
                            quote! {
                                #fields_init_ts
                                #match_origin_try_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&std_ops_bound_i64_ts, &std_ops_bound_i64_ts]);
                            quote! {
                                #fields_init_ts
                                #match_origin_try_new_for_deserialize_two_ts
                            }
                        }),
                        gen_fn_visit_seq_ts(&{
                            let fields_init_ts = gen_fields_serde_de_seq_access_next_el_init_ts(&[&I32, &I32, &I64]);
                            quote! {
                                #fields_init_ts
                                #origin_new_for_deserialize_three_ts
                            }
                        }),
                    )
                };
                let (fn_visit_u64_two_ts, fn_visit_u64_three_ts, fn_visit_u64_four_ts) = {
                    let gen_fn_visit_u64_ts = |param_nbr: &ParamNbr| {
                        let fields_ts = {
                            param_nbr.get_vec_from_index_starting_with_zero().into_iter().map(|el0| {
                                let index_vrt_ts = format!("{el0}u64").parse::<Ts2>().expect("5aee0393");
                                let field_index_ts = gen_field_index_ts(el0);
                                quote! {#index_vrt_ts => Ok(__Field::#field_index_ts)}
                            })
                        };
                        quote! {
                            fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                            where
                                __E: serde::de::Error,
                            {
                                match v {
                                    #(#fields_ts),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (gen_fn_visit_u64_ts(&param_nbr_two), gen_fn_visit_u64_ts(&param_nbr_three), gen_fn_visit_u64_ts(&param_nbr_four))
                };
                let (fn_visit_str_value_start_end_ts, fn_visit_str_value_hour_min_sec_micro_ts, fn_visit_str_value_hour_minute_second_microsecond_ts, fn_visit_str_value_date_time_ts, fn_visit_str_value_date_naive_time_ts, fn_visit_str_value_months_days_microseconds_ts) = {
                    let gen_fn_visit_str_ts = |vec_ts: &[&dyn DisplayPlusToTokens]| {
                        let fields_ts = vec_ts.iter().enumerate().map(|(index_e1c5acfd, el0)| {
                            let el0_dq_ts = dq_ts(&el0);
                            let field_index_name_ts = gen_field_index_ts(index_e1c5acfd);
                            quote! {#el0_dq_ts => Ok(__Field::#field_index_name_ts)}
                        });
                        quote! {
                            fn visit_str<__E>(
                                self,
                                v: &str,
                            ) -> Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match v {
                                    #(#fields_ts),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (
                        gen_fn_visit_str_ts(&start_end_display_plus_to_tokens_arr),
                        gen_fn_visit_str_ts(&hour_min_sec_micro_display_plus_to_tokens_arr),
                        gen_fn_visit_str_ts(&hour_minute_second_microsecond_display_plus_to_tokens_arr),
                        gen_fn_visit_str_ts(&date_time_display_plus_to_tokens_arr),
                        gen_fn_visit_str_ts(&date_naive_time_display_plus_to_tokens_arr),
                        gen_fn_visit_str_ts(&months_days_microseconds_display_plus_to_tokens_arr),
                    )
                };
                let (fn_visit_bytes_start_end_ts, fn_visit_bytes_hour_min_sec_micro_ts, fn_visit_bytes_hour_minute_second_microsecond_ts, fn_visit_bytes_date_time_ts, fn_visit_bytes_date_naive_time_ts, fn_visit_bytes_months_days_microseconds_ts) = {
                    let gen_fn_visit_bytes_ts = |vec_ts: &[&dyn DisplayPlusToTokens]| {
                        let fields_ts = vec_ts.iter().enumerate().map(|(index_545c3b1e, el0)| {
                            let b_el0_dq_ts = format!("b{}", dq_str(&el0)).parse::<Ts2>().expect("c76c976b");
                            let field_index_name_ts = gen_field_index_ts(index_545c3b1e);
                            quote! {#b_el0_dq_ts => Ok(__Field::#field_index_name_ts)}
                        });
                        quote! {
                            fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                            where
                                __E: serde::de::Error,
                            {
                                match v {
                                    #(#fields_ts),*,
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                    };
                    (
                        gen_fn_visit_bytes_ts(&start_end_display_plus_to_tokens_arr),
                        gen_fn_visit_bytes_ts(&hour_min_sec_micro_display_plus_to_tokens_arr),
                        gen_fn_visit_bytes_ts(&hour_minute_second_microsecond_display_plus_to_tokens_arr),
                        gen_fn_visit_bytes_ts(&date_time_display_plus_to_tokens_arr),
                        gen_fn_visit_bytes_ts(&date_naive_time_display_plus_to_tokens_arr),
                        gen_fn_visit_bytes_ts(&months_days_microseconds_display_plus_to_tokens_arr),
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
                    fn_visit_map_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                    fn_visit_map_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                    fn_visit_map_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    fn_visit_map_sqlx_pg_types_pg_range_sqlx_pg_types_pg_range_i32_ts,
                    fn_visit_map_sqlx_pg_types_pg_range_sqlx_pg_types_pg_range_i64_ts,
                    fn_visit_map_sqlx_pg_types_pg_interval_ts,
                ) = {
                    let gen_fn_visit_map_ts = |field_opt_none_init_ts: &dyn ToTokens, while_some_next_key_field_ts: &dyn ToTokens, match_field_init_ts: &dyn ToTokens, serde_private_ok_ts: &dyn ToTokens| {
                        quote! {
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                            where
                                __A: serde::de::MapAccess<'de>,
                            {
                                #field_opt_none_init_ts
                                #while_some_next_key_field_ts
                                #match_field_init_ts
                                #serde_private_ok_ts
                            }
                        }
                    };
                    let (
                        field_opt_none_init_sqlx_types_chrono_naive_time_ts,
                        field_opt_none_init_sqlx_types_time_time_ts,
                        field_opt_none_init_sqlx_types_chrono_naive_date_time_ts,
                        field_opt_none_init_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        field_opt_none_init_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                        field_opt_none_init_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                        field_opt_none_init_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        field_opt_none_init_sqlx_pg_types_pg_range_i32_ts,
                        field_opt_none_init_sqlx_pg_types_pg_range_i64_ts,
                        field_opt_none_init_sqlx_pg_types_pg_interval_ts,
                    ) = {
                        let gen_field_opt_none_init_ts = |vec_ts: &[&dyn ToTokens]| {
                            let fields_init_ts = vec_ts.iter().enumerate().map(|(index_d9ee264a, el0)| {
                                let field_index_name_ts = gen_field_index_ts(index_d9ee264a);
                                quote! {let mut #field_index_name_ts: Option<#el0> = None;}
                            });
                            quote! {#(#fields_init_ts)*}
                        };
                        (
                            gen_field_opt_none_init_ts(&[&U32, &U32, &U32, &U32]),
                            gen_field_opt_none_init_ts(&[&U8, &U8, &U8, &U32]),
                            gen_field_opt_none_init_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]),
                            gen_field_opt_none_init_ts(&[&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc]),
                            gen_field_opt_none_init_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts]),
                            gen_field_opt_none_init_ts(&[&std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts]),
                            gen_field_opt_none_init_ts(&[&std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts]),
                            gen_field_opt_none_init_ts(&[&std_ops_bound_i32_ts, &std_ops_bound_i32_ts]),
                            gen_field_opt_none_init_ts(&[&std_ops_bound_i64_ts, &std_ops_bound_i64_ts]),
                            gen_field_opt_none_init_ts(&[&I32, &I32, &I64]),
                        )
                    };
                    let (
                        while_some_next_key_field_sqlx_types_chrono_naive_time_ts,
                        while_some_next_key_field_sqlx_types_time_time_ts,
                        while_some_next_key_field_sqlx_types_chrono_naive_date_time_ts,
                        while_some_next_key_field_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        while_some_next_key_field_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                        while_some_next_key_field_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                        while_some_next_key_field_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        while_some_next_key_field_sqlx_pg_types_pg_range_i32_ts,
                        while_some_next_key_field_sqlx_pg_types_pg_range_i64_ts,
                        while_some_next_key_field_sqlx_pg_types_pg_interval_ts,
                    ) = {
                        let gen_while_some_next_key_field_ts = |vec_ts: &[(&dyn Display, &dyn ToTokens)]| {
                            let fields_init_ts = vec_ts.iter().enumerate().map(|(index_2b1736c7, el0)| {
                                let field_name_dq_ts = dq_str(&el0.0);
                                let ft_ts = &el0.1;
                                let field_index_name_ts = gen_field_index_ts(index_2b1736c7);
                                quote! {
                                    __Field::#field_index_name_ts => {
                                        if Option::is_some(&#field_index_name_ts) {
                                            return Err(<__A::Error as serde::de::Error>::duplicate_field(#field_name_dq_ts));
                                        }
                                        #field_index_name_ts = Some(serde::de::MapAccess::next_value::<#ft_ts>(&mut __map)?);
                                    }
                                }
                            });
                            quote! {
                                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        #(#fields_init_ts)*
                                        __Field::__ignore => {
                                            let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                        }
                                    }
                                }
                            }
                        };
                        (
                            gen_while_some_next_key_field_ts(&[(&HourSc, &U32), (&MinSc, &U32), (&SecSc, &U32), (&MicroSc, &U32)]),
                            gen_while_some_next_key_field_ts(&[(&HourSc, &U8), (&MinuteSc, &U8), (&SecondSc, &U8), (&MicrosecondSc, &U32)]),
                            gen_while_some_next_key_field_ts(&[(&DateSc, &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc), (&TimeSc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc)]),
                            gen_while_some_next_key_field_ts(&[(&DateNaiveSc, &sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc), (&TimeSc, &sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc)]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts), (&EndSc, &std_ops_bound_sqlx_types_chrono_naive_date_as_not_null_date_origin_ts)]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts), (&EndSc, &std_ops_bound_sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ts)]),
                            gen_while_some_next_key_field_ts(&[
                                (&StartSc, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts),
                                (&EndSc, &std_ops_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ts),
                            ]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_i32_ts), (&EndSc, &std_ops_bound_i32_ts)]),
                            gen_while_some_next_key_field_ts(&[(&StartSc, &std_ops_bound_i64_ts), (&EndSc, &std_ops_bound_i64_ts)]),
                            gen_while_some_next_key_field_ts(&[(&MonthsSc, &I32), (&DaysSc, &I32), (&MicrosecondsSc, &I64)]),
                        )
                    };
                    let (match_field_init_hour_min_sec_micro_ts, match_field_init_start_end_ts, match_field_init_hour_minute_second_microsecond_ts, match_field_init_date_time_ts, match_field_init_date_naive_time_ts, match_field_init_months_days_microseconds_ts) = {
                        let gen_match_field_init_ts = |vec_ts: &[&dyn DisplayPlusToTokens]| {
                            let fields_init_ts = vec_ts.iter().enumerate().map(|(index_e1adef1a, el0)| {
                                let field_name_dq_ts = dq_str(&el0);
                                let field_index_ts = gen_field_index_ts(index_e1adef1a);
                                let field_index_value_ts = gen_field_index_value_ts(index_e1adef1a);
                                quote! {
                                    let #field_index_value_ts = match #field_index_ts {
                                        Some(v_eeeb431b) => v_eeeb431b,
                                        None => serde::__private228::de::missing_field(#field_name_dq_ts)?,
                                    };
                                }
                            });
                            quote! {#(#fields_init_ts)*}
                        };
                        (
                            gen_match_field_init_ts(&hour_min_sec_micro_display_plus_to_tokens_arr),
                            gen_match_field_init_ts(&start_end_display_plus_to_tokens_arr),
                            gen_match_field_init_ts(&hour_minute_second_microsecond_display_plus_to_tokens_arr),
                            gen_match_field_init_ts(&date_time_display_plus_to_tokens_arr),
                            gen_match_field_init_ts(&date_naive_time_display_plus_to_tokens_arr),
                            gen_match_field_init_ts(&months_days_microseconds_display_plus_to_tokens_arr),
                        )
                    };
                    (
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_types_chrono_naive_time_ts,
                            &while_some_next_key_field_sqlx_types_chrono_naive_time_ts,
                            &match_field_init_hour_min_sec_micro_ts,
                            &sqlx_types_chrono_naive_time_origin_try_new_for_deserialize,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_types_time_time_ts,
                            &while_some_next_key_field_sqlx_types_time_time_ts,
                            &match_field_init_hour_minute_second_microsecond_ts,
                            &match_origin_try_new_for_deserialize_four_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_types_chrono_naive_date_time_ts,
                            &while_some_next_key_field_sqlx_types_chrono_naive_date_time_ts,
                            &match_field_init_date_time_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &while_some_next_key_field_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &match_field_init_date_naive_time_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            &while_some_next_key_field_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            &match_field_init_start_end_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            &while_some_next_key_field_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            &match_field_init_start_end_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &while_some_next_key_field_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &match_field_init_start_end_ts,
                            &origin_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_pg_types_pg_range_i32_ts,
                            &while_some_next_key_field_sqlx_pg_types_pg_range_i32_ts,
                            &match_field_init_start_end_ts,
                            &match_origin_try_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_pg_types_pg_range_i64_ts,
                            &while_some_next_key_field_sqlx_pg_types_pg_range_i64_ts,
                            &match_field_init_start_end_ts,
                            &match_origin_try_new_for_deserialize_two_ts,
                        ),
                        gen_fn_visit_map_ts(
                            &field_opt_none_init_sqlx_pg_types_pg_interval_ts,
                            &while_some_next_key_field_sqlx_pg_types_pg_interval_ts,
                            &match_field_init_months_days_microseconds_ts,
                            &origin_new_for_deserialize_three_ts,
                        ),
                    )
                };
                let (const_fields_start_end_ts, const_fields_sqlx_types_chrono_naive_time_ts, const_fields_sqlx_types_time_time_ts, const_fields_sqlx_types_chrono_naive_date_time_ts, const_fields_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts, const_fields_sqlx_pg_types_pg_interval_ts) = {
                    let gen_const_fields_ts = |vec_ts: &[&dyn DisplayPlusToTokens]| {
                        let field_names_ts = vec_ts.iter().map(|el0| dq_ts(&el0));
                        quote! {
                            #[doc(hidden)]
                            const FIELDS: &[&str] = &[#(#field_names_ts),*];
                        }
                    };
                    (
                        gen_const_fields_ts(&start_end_display_plus_to_tokens_arr),
                        gen_const_fields_ts(&hour_min_sec_micro_display_plus_to_tokens_arr),
                        gen_const_fields_ts(&hour_minute_second_microsecond_display_plus_to_tokens_arr),
                        gen_const_fields_ts(&date_time_display_plus_to_tokens_arr),
                        gen_const_fields_ts(&date_naive_time_display_plus_to_tokens_arr),
                        gen_const_fields_ts(&months_days_microseconds_display_plus_to_tokens_arr),
                    )
                };
                let gen_impl_serde_de_visitor_for_tokens_ts = |
                    should_add_de_lifetime: ShouldAddDeLifetime,
                    ident_ts_2ecd936e: &dyn ToTokens,
                    ts: &dyn ToTokens
                | {
                    let (
                        maybe_impl_lifetime_ts,
                        maybe_visitor_lifetime_ts
                    ) = match should_add_de_lifetime{
                        ShouldAddDeLifetime::False => (
                            Ts2::new(),
                            quote!{<'_>},
                        ),
                        ShouldAddDeLifetime::True => (
                            quote!{<'de>},
                            quote!{<'de>},
                        ),
                    };
                    quote! {
                        impl #maybe_impl_lifetime_ts _serde::de::Visitor #maybe_visitor_lifetime_ts for #ident_ts_2ecd936e {
                            #ts
                        }
                    }
                };
                let (
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_time_ts,
                    impl_serde_de_visitor_for_visitor_pg_money_ts,
                    impl_serde_de_visitor_for_visitor_uuid_uuid_ts,
                    impl_serde_de_visitor_for_visitor_mac_address_mac_address_ts,
                    impl_serde_de_visitor_for_visitor_string_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_time_time_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_time_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_i32_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_i64_ts,
                    impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_interval_ts,
                ) = {
                    let gen_impl_serde_de_visitor_for_visitor_ts = |zero_ts: &dyn ToTokens, first_ts: &dyn ToTokens, second_ts: &dyn ToTokens| {
                        gen_impl_serde_de_visitor_for_tokens_ts(
                            ShouldAddDeLifetime::True,
                            &quote! {__Visitor<'de>},
                            &quote! {
                                type Value = #ident_stdrt_not_null_origin_ucc;
                                #zero_ts
                                #first_ts
                                #second_ts
                            },
                        )
                    };
                    (
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_types_chrono_naive_time_ts, &fn_visit_map_sqlx_types_chrono_naive_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_dq_ts, &fn_visit_newtype_struct_pg_money_ts, &fn_visit_seq_pg_money_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_dq_ts, &fn_visit_newtype_struct_uuid_ts, &fn_visit_seq_uuid_uuid_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_dq_ts, &fn_visit_newtype_struct_mac_address_ts, &fn_visit_seq_sqlx_types_mac_address_mac_address_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_dq_ts, &fn_visit_newtype_struct_text_ts, &fn_visit_seq_string_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_types_time_time_ts, &fn_visit_map_sqlx_types_time_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_tuple_struct_ident_dq_ts, &fn_visit_newtype_struct_sqlx_types_chrono_naive_date_ts, &fn_visit_seq_sqlx_types_chrono_naive_date_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_types_chrono_naive_date_time_ts, &fn_visit_map_sqlx_types_chrono_naive_date_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts, &fn_visit_map_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts, &fn_visit_map_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts, &fn_visit_map_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(
                            &fn_expecting_struct_ident_dq_ts,
                            &fn_visit_seq_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                            &fn_visit_map_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_pg_types_pg_range_i32_ts, &fn_visit_map_sqlx_pg_types_pg_range_sqlx_pg_types_pg_range_i32_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_pg_types_pg_range_i64_ts, &fn_visit_map_sqlx_pg_types_pg_range_sqlx_pg_types_pg_range_i64_ts),
                        gen_impl_serde_de_visitor_for_visitor_ts(&fn_expecting_struct_ident_dq_ts, &fn_visit_seq_sqlx_pg_types_pg_interval_ts, &fn_visit_map_sqlx_pg_types_pg_interval_ts),
                    )
                };
                let field_visitor_ts = quote! {__FieldVisitor};
                let type_value_equal_underscore_field_semicolon_ts = quote! {type Value = __Field;};
                let (
                    impl_serde_de_visitor_for_field_visitor_ts_5a4f24ce,
                    impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33,
                    impl_serde_de_visitor_for_field_visitor_ts_9b240c3e,
                    impl_serde_de_visitor_for_field_visitor_ts_dc439ca1,
                    impl_serde_de_visitor_for_field_visitor_ts_8c733fe0,
                    impl_serde_de_visitor_for_field_visitor_ts_f702a411,
                ) = {
                    let gen_impl_serde_de_visitor_for_field_visitor_ts = |ts: &dyn ToTokens| {
                        let impl_serde_de_visitor_for_tokens_ts = gen_impl_serde_de_visitor_for_tokens_ts(
                            ShouldAddDeLifetime::False,
                            &field_visitor_ts,
                            &ts
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
                            #fn_expecting_fi_ts
                            #fn_visit_u64_four_ts
                            #fn_visit_str_value_hour_min_sec_micro_ts
                            #fn_visit_bytes_hour_min_sec_micro_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_fi_ts
                            #fn_visit_u64_two_ts
                            #fn_visit_str_value_start_end_ts
                            #fn_visit_bytes_start_end_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_fi_ts
                            #fn_visit_u64_four_ts
                            #fn_visit_str_value_hour_minute_second_microsecond_ts
                            #fn_visit_bytes_hour_minute_second_microsecond_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_fi_ts
                            #fn_visit_u64_two_ts
                            #fn_visit_str_value_date_time_ts
                            #fn_visit_bytes_date_time_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_fi_ts
                            #fn_visit_u64_two_ts
                            #fn_visit_str_value_date_naive_time_ts
                            #fn_visit_bytes_date_naive_time_ts
                        }),
                        gen_impl_serde_de_visitor_for_field_visitor_ts(&quote! {
                            #type_value_equal_underscore_field_semicolon_ts
                            #fn_expecting_fi_ts
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
                match &pg_type {
                    PgType::I16AsInt2
                    | PgType::I32AsInt4
                    | PgType::I64AsInt8
                    | PgType::F32AsFloat4
                    | PgType::F64AsFloat8
                    | PgType::I16AsSmallSerialInitByPg
                    | PgType::I32AsSerialInitByPg
                    | PgType::I64AsBigSerialInitByPg
                    | PgType::BoolAsBool
                    | PgType::StdVecVecU8AsBytea
                    | PgType::SqlxTypesIpnetworkIpNetworkAsInet => DeriveOrImpl::Derive,
                    PgType::SqlxPgTypesPgMoneyAsMoney => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_pg_money_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PgType::StringAsText => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_string_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PgType::SqlxTypesChronoNaiveTimeAsTime => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_four_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_5a4f24ce
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_time_ts
                        #const_fields_sqlx_types_chrono_naive_time_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxTypesTimeTimeAsTime => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_four_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_9b240c3e
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_time_time_ts
                        #const_fields_sqlx_types_time_time_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxTypesChronoNaiveDateAsDate => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_dc439ca1
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_naive_date_time_ts
                        #const_fields_sqlx_types_chrono_naive_date_time_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_8c733fe0
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts
                        #const_fields_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => DeriveOrImpl::Impl(impl_serde_deserialize_for_uuid_uuid_ts),
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_mac_address_mac_address_ts
                        #serde_deserializer_deserialize_newtype_struct_ts
                    })),
                    PgType::SqlxPgTypesPgIntervalAsInterval => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_three_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f702a411
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_interval_ts
                        #const_fields_sqlx_pg_types_pg_interval_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_i32_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_i64_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DeriveOrImpl::Impl(gen_impl_serde_deserialize_for_tokens_ts(&quote! {
                        #enum_field_two_ts
                        #impl_serde_de_visitor_for_field_visitor_ts_f4d8cc33
                        #impl_serde_deserialize_for_field_ts
                        #struct_visitor_ts
                        #impl_serde_de_visitor_for_visitor_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts
                        #const_fields_start_end_ts
                        #serde_deserializer_deserialize_struct_visitor_ts
                    })),
                }
            };
            (serde_serialize_derive_or_impl, serde_deserialize_derive_or_impl)
        } else {
            (DeriveOrImpl::Derive, DeriveOrImpl::Derive)
        };
        let value_ident_inner_type_ts = quote! {#ValueSc: #ident_inner_type_ts};
        let ident_stdrt_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_stdrt_not_null_origin_try_new_er_ucc = SelfOriginTryNewErUcc::from_display(&ident_stdrt_not_null_ucc);
        let ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc = SelfOriginTryNewForDeserializeErUcc::from_display(&ident_stdrt_not_null_ucc);
        let int_range_type_to_range_inner_type_ts = |int_range_type: &IntRangeType| -> Ts2 {
            match &int_range_type {
                IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range => quote! {#I32},
                IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range => quote! {#I64},
            }
        };
        let gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = |ts: &dyn ToTokens| {
            quote! {sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                #ts,
                sqlx::types::chrono::Utc
            )}
        };
        let gen_sqlx_types_chrono_naive_date_time_new_ts = |ts: &dyn ToTokens| {
            quote! {sqlx::types::chrono::NaiveDateTime::#NewSc(#ts)}
        };
        let gen_sqlx_types_time_time_from_hms_micro_unwrap_ts = |ts: &dyn ToTokens| {
            quote! {sqlx::types::time::Time::from_hms_micro(#ts).expect("7a1a18fa")}
        };
        let gen_pub_const_new_or_pub_try_new_ts = |ident_d755cf8f: &dyn ToTokens| {
            let pub_fn_new_or_try_new_ts = if pg_type_init_try_new_try_from_pg_type.is_ok() {
                &gen_pub_try_new_ts(
                    &value_ident_inner_type_ts,
                    &ident_stdrt_not_null_origin_try_new_er_ucc,
                    &quote! {
                        match #ident_origin_ucc::#TryNewSc(#ValueSc) {
                            Ok(v_0f9f1a61) => Ok(Self(v_0f9f1a61)),
                            Err(er) => Err(er)
                        }
                    },
                )
            } else {
                &{
                    let self_ident_origin_new_value_ts = quote! {Self(#ident_origin_ucc::#NewSc(#ValueSc))};
                    if matches!(&pg_type_pattern, PgTypePattern::Stdrt)
                        && matches!(&is_nullable, IsNullable::False)
                    {
                        gen_pub_const_new_ts(
                            &MustUse,
                            &value_ident_inner_type_ts,
                            &self_ident_origin_new_value_ts
                        )
                    } else {
                        gen_pub_new_ts(
                            &MustUse,
                            &value_ident_inner_type_ts,
                            &self_ident_origin_new_value_ts
                        )
                    }
                }
            };
            quote! {
                impl #ident_d755cf8f {
                    #pub_fn_new_or_try_new_ts
                }
            }
        };
        let derive_copy = match &pg_type_pattern {
            PgTypePattern::Stdrt => match &pg_type {
                PgType::I16AsInt2 |
                PgType::I32AsInt4 |
                PgType::I64AsInt8 |
                PgType::F32AsFloat4 |
                PgType::F64AsFloat8 |
                PgType::I16AsSmallSerialInitByPg |
                PgType::I32AsSerialInitByPg |
                PgType::I64AsBigSerialInitByPg |
                PgType::SqlxPgTypesPgMoneyAsMoney |
                PgType::BoolAsBool |
                PgType::SqlxTypesChronoNaiveTimeAsTime |
                PgType::SqlxTypesTimeTimeAsTime |
                PgType::SqlxPgTypesPgIntervalAsInterval |
                PgType::SqlxTypesChronoNaiveDateAsDate |
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DeriveCopy::True,
                PgType::StringAsText |
                PgType::StdVecVecU8AsBytea => DeriveCopy::False,
            },
            PgTypePattern::ArrDim1 { .. } => DeriveCopy::False,
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
            let ident_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .derive_partial_eq()
                .build_struct(
                    &ident,
                    &Ts2::new(),
                    &quote!{;},
                );
            // println!("@@@{}", ident_inner_type_ts);
            let maybe_impl_ident_ts = if matches!(&pg_type_pattern, PgTypePattern::Stdrt) &&
                matches!(&is_nullable, IsNullable::False)
            {
                enum IsConst {
                    False,
                    True,
                }
                let gen_inner_type_ts = |
                    is_const: IsConst,
                    name_ts: &dyn ToTokens,
                    ts: &dyn ToTokens
                |{
                    let maybe_const_ts = match is_const {
                        IsConst::False => Ts2::new(),
                        IsConst::True => quote!{const},
                    };
                    quote!{
                        #maybe_const_ts fn #name_ts() -> #ident_inner_type_ts {
                            #ts
                        }
                    }
                };
                let maybe_min_inner_type_ts = {
                    let gen_inner_type_ts_67fc7980 = |
                        is_const: IsConst,
                        ts_1ca2df79: &dyn ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{min_inner_type},
                        &ts_1ca2df79
                    );
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_67fc7980(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("000ddcc2")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_67fc7980(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("f065e2b1")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_more_than_min_inner_type_ts = {
                    let gen_inner_type_ts_6d89728a = |
                        is_const: IsConst,
                        ts_dcc22544: &dyn ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{slightly_more_than_min_inner_type},
                        &ts_dcc22544
                    );
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_6d89728a(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("9545a47c")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_6d89728a(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("03f9561a")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_middle_inner_type_ts = {
                    let gen_inner_type_ts_23368199 = |
                        is_const: IsConst,
                        ts_645cff79: &dyn ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{middle_inner_type},
                        &ts_645cff79
                    );
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_23368199(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("0dafc3fc")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_23368199(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("d2ec329f")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_inner_type_ts_23368199(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("a2f306ea")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_more_than_middle_inner_type_ts = {
                    let gen_inner_type_ts_3a61c0b0 = |
                        is_const: IsConst,
                        ts_e09b85a8: &dyn ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{slightly_more_than_middle_inner_type},
                        &ts_e09b85a8
                    );
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_3a61c0b0(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("235276a7")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inner_type_ts_3a61c0b0(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("6a3dbcaa")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_max_inner_type_ts = {
                    let gen_inner_type_ts_32acb388 = |
                        is_const: IsConst,
                        ts_385694da: &dyn ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{max_inner_type},
                        &ts_385694da
                    );
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_32acb388(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("b217e3bf")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_inner_type_ts_32acb388(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveDate::MAX
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_less_than_max_inner_type_ts = {
                    let gen_inner_type_ts_ddf0f630 = |
                        is_const: IsConst,
                        ts_5ca08aea: &dyn ToTokens
                    |gen_inner_type_ts(
                        is_const,
                        &quote!{slightly_less_than_max_inner_type},
                        &ts_5ca08aea
                    );
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inner_type_ts_ddf0f630(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_998).expect("5d6cf475")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_read_inner_inits_ts = {
                    let gen_function_ts = |
                        name_ts: &Ts2,
                        ts_5dfcb210: &Ts2
                    |quote!{
                        const fn #name_ts() -> #ident_inner_type_ts {
                            #ts_5dfcb210
                        }
                    };
                    match &pg_type {
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some({
                            let ts_80e0683c = [
                                (&sqlx_types_chrono_naive_time_min_function_ts, &quote!{0,0,0,0}),
                                (&sqlx_types_chrono_naive_time_ten_function_ts, &quote!{10,10,10,10}),
                                (&sqlx_types_chrono_naive_time_twenty_function_ts, &quote!{20,20,20,20}),
                                (&sqlx_types_chrono_naive_time_max_function_ts, &quote!{23,59,59,999_999}),
                            ].iter().map(|(name_ts, params_ts)| quote! {
                                const fn #name_ts() -> #ident_inner_type_ts {
                                    #ident_inner_type_ts::from_hms_micro_opt(#params_ts).expect("149e01cc")
                                }
                            }).collect::<Vec<Ts2>>();
                            quote!{#(#ts_80e0683c)*}
                        }),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some({
                            let ts_80e0683c = {
                                let gen_function_ident_inner_type_ts = |
                                    name_ts: &Ts2,
                                    ts_a29ab1c6: &Ts2
                                |gen_function_ts(
                                    name_ts,
                                    &quote!{#ident_inner_type_ts::#ts_a29ab1c6}
                                );
                                [
                                    gen_function_ident_inner_type_ts(
                                        &sqlx_types_chrono_naive_date_max_function_ts,
                                        &quote! { MAX }
                                    ),
                                    gen_function_ts(
                                        &sqlx_types_chrono_naive_date_max_pred_opt_expect_function_ts,
                                        &quote!{Self::#sqlx_types_chrono_naive_date_max_function_ts().pred_opt().expect("b7e16bf1")}
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
                                    .map(|(name_ts, params_ts)| {
                                        gen_function_ident_inner_type_ts(
                                            name_ts,
                                            &quote! {
                                                from_ymd_opt(#params_ts)
                                                    .expect("d25ee0e9")
                                            }
                                        )
                                    })
                                ).collect::<Vec<Ts2>>()
                            };
                            quote!{#(#ts_80e0683c)*}
                        }),
                    }
                };
                if maybe_min_inner_type_ts.is_some() ||
                    maybe_slightly_more_than_min_inner_type_ts.is_some() ||
                    maybe_middle_inner_type_ts.is_some() ||
                    maybe_slightly_more_than_middle_inner_type_ts.is_some() ||
                    maybe_max_inner_type_ts.is_some() ||
                    maybe_slightly_less_than_max_inner_type_ts.is_some() ||
                    maybe_read_inner_inits_ts.is_some()
                {
                    quote!{
                        #AllowClippyArbitrarySourceItemOrdering
                        impl #ident {
                            #maybe_min_inner_type_ts
                            #maybe_slightly_more_than_min_inner_type_ts
                            #maybe_middle_inner_type_ts
                            #maybe_slightly_more_than_middle_inner_type_ts
                            #maybe_max_inner_type_ts
                            #maybe_slightly_less_than_max_inner_type_ts
                            #maybe_read_inner_inits_ts
                        }
                    }
                }
                else {
                    Ts2::new()
                }
            }
            else {
                Ts2::new()
            };
            quote!{
                #ident_ts
                #maybe_impl_ident_ts
            }
        };
        let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
        let ident_origin_ts = {
            let ident_origin_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_eq_if(match &is_not_null_stdrt_can_be_primary_key {
                    IsNotNullStdrtCanBePrimaryKey::False => DeriveEq::False,
                    IsNotNullStdrtCanBePrimaryKey::True => DeriveEq::True,
                })
                .derive_partial_ord_if(match &is_stdrt_not_null {
                    IsStdrtNotNull::False => DerivePartialOrd::False,
                    IsStdrtNotNull::True => match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitByPg
                        | PgType::I32AsSerialInitByPg
                        | PgType::I64AsBigSerialInitByPg
                        | PgType::BoolAsBool
                        | PgType::StringAsText
                        | PgType::StdVecVecU8AsBytea
                        | PgType::SqlxTypesChronoNaiveTimeAsTime
                        | PgType::SqlxTypesTimeTimeAsTime
                        | PgType::SqlxTypesChronoNaiveDateAsDate
                        | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => DerivePartialOrd::True,
                        PgType::SqlxPgTypesPgMoneyAsMoney
                        | PgType::SqlxPgTypesPgIntervalAsInterval
                        | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                        | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DerivePartialOrd::False,
                    },
                })
                .derive_ord_if(match &is_not_null_stdrt_can_be_primary_key {
                    IsNotNullStdrtCanBePrimaryKey::False => DeriveOrd::False,
                    IsNotNullStdrtCanBePrimaryKey::True => DeriveOrd::True,
                })
                .derive_serde_serialize_if(match &serde_serialize_derive_or_impl {
                    DeriveOrImpl::Derive => DeriveSerdeSerialize::True,
                    DeriveOrImpl::Impl(_) => DeriveSerdeSerialize::False,
                })
                .derive_serde_deserialize_if(match &serde_deserialize_derive_or_impl {
                    DeriveOrImpl::Derive => DeriveSerdeDeserialize::True,
                    DeriveOrImpl::Impl(_) => DeriveSerdeDeserialize::False,
                })
                .build_struct(
                    &ident_origin_ucc,
                    &Ts2::new(),
                    &quote!{(#ft_handle);},
                );
            let gen_int_range_type_er_vrts_ts = |int_range_type: &IntRangeType| {
                let range_inner_type_ts = int_range_type_to_range_inner_type_ts(int_range_type);
                quote! {
                    #IncludedStartGreaterThanIncludedEndUcc {
                        #[eo_to_err_string_serde]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_err_string_serde]
                        #EndSc: #range_inner_type_ts,
                        loc: location_lib::loc::Loc,
                    },
                    #IncludedStartGreaterThanExcludedEndUcc {
                        #[eo_to_err_string_serde]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_err_string_serde]
                        #EndSc: #range_inner_type_ts,
                        loc: location_lib::loc::Loc,
                    },
                    #ExcludedStartGreaterThanIncludedEndUcc {
                        #[eo_to_err_string_serde]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_err_string_serde]
                        #EndSc: #range_inner_type_ts,
                        loc: location_lib::loc::Loc,
                    },
                    #ExcludedStartGreaterThanExcludedEndUcc {
                        #[eo_to_err_string_serde]
                        #StartSc: #range_inner_type_ts,
                        #[eo_to_err_string_serde]
                        #EndSc: #range_inner_type_ts,
                        loc: location_lib::loc::Loc,
                    },
                    #IncludedEndCannotBeMaxUcc {
                        #[eo_to_err_string_serde]
                        #EndSc: #range_inner_type_ts,
                        loc: location_lib::loc::Loc,
                    },
                }
            };
            let nanosecond_precision_is_not_supported_vrt_try_new_ts = quote! {
                #NanosecondPrecisionIsNotSupportedUcc {
                    #[eo_to_err_string_serde]
                    #ValueSc: #StringTs,
                    loc: location_lib::loc::Loc,
                }
            };
            let sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts = quote! {
                #EarlierDateNotSupportedUcc {
                    #[eo_to_err_string_serde]
                    #ValueSc: #StringTs,
                    #[eo_to_err_string_serde]
                    #EarliestSupportedDateSc: #StringTs,
                    loc: location_lib::loc::Loc,
                }
            };
            let string_as_text_try_new_er_vrts_ts = quote! {
                #ContainsNullByteUcc {
                    #[eo_to_err_string_serde]
                    #ValueSc: #ident_inner_type_ts,
                    loc: location_lib::loc::Loc,
                }
            };
            let maybe_pub_enum_ident_stdrt_not_null_origin_try_new_er_ts = if matches!(&is_stdrt_not_null, IsStdrtNotNull::True)
                && let Ok(pg_type_init_try_new) = &pg_type_init_try_new_try_from_pg_type
            {
                let ts_d57d5de2 = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_thiserror_error()
                    .derive_location_lib_location()
                    .build_enum(
                        &ident_stdrt_not_null_origin_try_new_er_ucc,
                        &Ts2::new(),
                        &{
                            let gen_start_end_ts = |ts: &dyn ToTokens| {
                                let (start_vrt_ts, end_vrt_ts) = {
                                    let gen_vrt_ts = |start_or_end: &StartOrEnd| {
                                        let start_or_end_ucc = gen_start_or_end_ucc(start_or_end);
                                        quote! {
                                            #start_or_end_ucc {
                                                #[eo_location]
                                                #ErSc: #ts,
                                                loc: location_lib::loc::Loc,
                                            }
                                        }
                                    };
                                    (gen_vrt_ts(&StartOrEnd::Start), gen_vrt_ts(&StartOrEnd::End))
                                };
                                quote! {
                                    #start_vrt_ts,
                                    #end_vrt_ts,
                                }
                            };
                            let ts: &dyn ToTokens = match &pg_type_init_try_new {
                                PgTypeInitTryNew::StringAsText => &string_as_text_try_new_er_vrts_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime | PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_vrt_try_new_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => &quote! {
                                    #DateUcc {
                                        #[eo_location]
                                        #ErSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_er_ucc,
                                        loc: location_lib::loc::Loc,
                                    },
                                    #TimeUcc {
                                        #[eo_location]
                                        #ErSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_er_ucc,
                                        loc: location_lib::loc::Loc,
                                    },
                                },
                                PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &quote! {
                                    #DateNaiveUcc {
                                        #[eo_location]
                                        #ErSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_er_ucc,
                                        loc: location_lib::loc::Loc,
                                    },
                                    #TimeUcc {
                                        #[eo_location]
                                        #ErSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_try_new_er_ucc,
                                        loc: location_lib::loc::Loc,
                                    },
                                },
                                PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_start_end_ts(
                                    &sqlx_types_chrono_naive_date_as_not_null_date_origin_try_new_er_ucc
                                ),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_start_end_ts(
                                    &sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_try_new_er_ucc
                                ),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_start_end_ts(
                                    &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_try_new_er_ucc
                                ),
                            };
                            quote!{{#ts}}
                        }
                    );
                quote!{
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_d57d5de2
                }
            } else {
                Ts2::new()
            };
            let maybe_pub_enum_ident_stdrt_not_null_origin_try_new_for_deserialize_er_ts = if matches!(&is_stdrt_not_null, IsStdrtNotNull::True)
                && let DeriveOrImpl::Impl(_) = &serde_deserialize_derive_or_impl
            {
                match &pg_type_deserialize {
                    PgTypeDeserialize::Derive => Ts2::new(),
                    PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(pg_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &pg_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                        PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(_) => Ts2::new(),
                        PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(pg_type_impl_try_new_for_deserialize) => {
                            let ts_026f2a24 = StructOrEnumDeriveTsStreamBuilder::new()
                            .make_pub()
                            .derive_debug()
                            .derive_serde_serialize()
                            .derive_serde_deserialize()
                            .derive_thiserror_error()
                            .derive_location_lib_location()
                            .build_enum(
                                &ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc,
                                &Ts2::new(),
                                &{
                                    let ts: &dyn ToTokens = match &pg_type_impl_try_new_for_deserialize {
                                        PgTypeImplTryNewForDeserialize::StringAsText => &string_as_text_try_new_er_vrts_ts,
                                        PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => &quote! {
                                            #InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                #[eo_to_err_string_serde]
                                                #HourSc: #U32,
                                                #[eo_to_err_string_serde]
                                                #MinSc: #U32,
                                                #[eo_to_err_string_serde]
                                                #SecSc: #U32,
                                                #[eo_to_err_string_serde]
                                                #MicroSc: #U32,
                                                loc: location_lib::loc::Loc,
                                            },
                                            #nanosecond_precision_is_not_supported_vrt_try_new_ts
                                        },
                                        PgTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => &quote! {
                                            #InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                #[eo_to_err_string_serde]
                                                #HourSc: #U8,
                                                #[eo_to_err_string_serde]
                                                #MinuteSc: #U8,
                                                #[eo_to_err_string_serde]
                                                #SecondSc: #U8,
                                                #[eo_to_err_string_serde]
                                                #MicrosecondSc: #U32,
                                                #[eo_to_err_string_serde]
                                                #ErSc: #StringTs,
                                                loc: location_lib::loc::Loc,
                                            },
                                            #nanosecond_precision_is_not_supported_vrt_try_new_ts
                                        },
                                        PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts,
                                        PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI32AsInt4Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI64AsInt8Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                    };
                                    quote!{{#ts}}
                                }
                            );
                            quote!{
                                #AllowClippyArbitrarySourceItemOrdering
                                #ts_026f2a24
                            }
                        }
                    },
                }
            } else {
                Ts2::new()
            };
            let impl_ident_origin_ts = {
                let fn_new_or_try_new_ts = pg_type_init_try_new_try_from_pg_type.as_ref().map_or_else(
                |()| {
                    let ts = {
                        let ts = {
                            let gen_match_opt_ts = |type_ts: &dyn ToTokens| {
                                quote! {value.map(#type_ts::#NewSc)}
                            };
                            let gen_arr_dims_init_ts = |type_ts: &dyn ToTokens| match &is_nullable {
                                IsNullable::False => quote! {value.into_iter().map(#type_ts::#NewSc).collect()},
                                IsNullable::True => gen_match_opt_ts(&type_ts),
                            };
                            match &pg_type_pattern {
                                PgTypePattern::Stdrt => match &is_nullable {
                                    IsNullable::False => {
                                        range_try_from_pg_type.as_ref().map_or_else(
                                            |()| quote! {#ValueSc},
                                            |v_6ed98462| gen_pg_range_conversion_ts(
                                                &ValueSc,
                                                &{
                                                    let range_pg_type_ident_origin = SelfOriginUcc::from_display(&gen_ident_str(&PgType::from(v_6ed98462), is_nullable, pg_type_pattern));
                                                    quote! {#range_pg_type_ident_origin::#NewSc(v_af65ccce)}
                                                }
                                            )
                                        )
                                    }
                                    IsNullable::True => gen_match_opt_ts(&ident_stdrt_not_null_origin_ucc),
                                },
                                PgTypePattern::ArrDim1 { dim1_is_nullable } => gen_arr_dims_init_ts(&{
                                    let (pg_type_pattern_ce191343, is_nullable_b772ed8a): (&PgTypePattern, &IsNullable) = match &is_nullable {
                                        IsNullable::False => (&PgTypePattern::Stdrt, dim1_is_nullable),
                                        IsNullable::True => (pg_type_pattern, &IsNullable::False),
                                    };
                                    gen_ident_origin_non_wrapping_8ad5380a(pg_type_pattern_ce191343, is_nullable_b772ed8a)
                                }),
                            }
                        };
                        quote! {Self(#ts)}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nullable {
                            IsNullable::False => gen_const_new_ts(
                                &MustUse,
                                &value_ident_inner_type_ts,
                                &ts
                            ),
                            IsNullable::True => gen_new_ts(
                                &MustUse,
                                &value_ident_inner_type_ts,
                                &ts
                            ),
                        },
                        PgTypePattern::ArrDim1 { .. } => gen_new_ts(
                            &MustUse,
                            &value_ident_inner_type_ts,
                            &ts
                        ),
                    }
                },
                |pg_type_init_try_new| {
                    let ts = {
                        let gen_match_opt_ts = |type_ts: &dyn ToTokens| {
                            quote! {Ok(Self(match #ValueSc {
                                Some(v_989d943e) => Some(match #type_ts::#TryNewSc(v_989d943e) {
                                    Ok(v_ea2a4a8c) => v_ea2a4a8c,
                                    Err(er) => {
                                        return Err(er);
                                    },
                                }),
                                None => None
                            }))}
                        };
                        let gen_arr_dims_init_ts = |type_ts: &dyn ToTokens| match &is_nullable {
                            IsNullable::False => quote! {
                                Ok(Self({
                                    let mut acc_4ce2782a = Vec::new();
                                    for el_de177578 in #ValueSc {
                                        match #type_ts::#TryNewSc(el_de177578) {
                                            Ok(v_a763a416) => {
                                                acc_4ce2782a.push(v_a763a416);
                                            },
                                            Err(er) => {
                                                return Err(er);
                                            }
                                        }
                                    }
                                    acc_4ce2782a
                                }))
                            },
                            IsNullable::True => gen_match_opt_ts(&type_ts),
                        };
                        match &pg_type_pattern {
                            PgTypePattern::Stdrt => match &is_nullable {
                                IsNullable::False => {
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
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                    if #EndSc == max {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#StartSc), std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Excluded(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#StartSc), std::ops::Bound::Excluded(#EndSc))
                                                }
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Unbounded) => (std::ops::Bound::Included(#StartSc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Included(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                    if #EndSc == max {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Excluded(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Excluded(#EndSc))
                                                }
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Unbounded) => (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Included(#EndSc)) => {
                                                    if #EndSc == max {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            loc: location_lib::loc!(),
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
                                    let gen_ok_self_sqlx_pg_types_pg_range_ts = |ident_ts_430fc374: &dyn ToTokens| quote! {
                                        Ok(Self(sqlx::postgres::types::PgRange {
                                            #StartSc: match #ValueSc.#StartSc {
                                                std::ops::Bound::Included(included_value) => match #ident_ts_430fc374::#TryNewSc(included_value) {
                                                    Ok(v_a9c1f658) => std::ops::Bound::Included(v_a9c1f658.0),
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#StartUcc {
                                                            #ErSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Excluded(excluded_value) => match #ident_ts_430fc374::#TryNewSc(excluded_value) {
                                                    Ok(v_f0ff8036) => std::ops::Bound::Excluded(v_f0ff8036.0),
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#StartUcc {
                                                            #ErSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                            },
                                            #EndSc: match #ValueSc.#EndSc {
                                                std::ops::Bound::Included(included_value) => match #ident_ts_430fc374::#TryNewSc(included_value) {
                                                    Ok(v_80168e2b) => std::ops::Bound::Included(v_80168e2b.0),
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#EndUcc {
                                                            #ErSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Excluded(excluded_value) => match #ident_ts_430fc374::#TryNewSc(excluded_value) {
                                                    Ok(v_05f87b70) => std::ops::Bound::Excluded(v_05f87b70.0),
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#EndUcc {
                                                            #ErSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                },
                                                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                            },
                                        }))
                                    };
                                    match &pg_type_init_try_new {
                                        PgTypeInitTryNew::StringAsText => quote! {
                                            if #ValueSc.find('\0').is_some() {
                                                Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#ContainsNullByteUcc {
                                                    #ValueSc,
                                                    loc: location_lib::loc!(),
                                                })
                                            } else {
                                                Ok(Self(#ValueSc))
                                            }
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime => quote! {
                                            if <#inner_type_stdrt_not_null_ts as chrono::Timelike>::nanosecond(&#ValueSc).checked_rem(1000).expect("7c8b4e12") != 0 {
                                                return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                    #ValueSc: #ValueSc.to_string(),
                                                    loc: location_lib::loc!(),
                                                });
                                            }
                                            Ok(Self(#ValueSc))
                                        },
                                        PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => quote! {
                                            if #ValueSc.nanosecond().checked_rem(1000).expect("ce47524f") != 0 {
                                                return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                    #ValueSc: #ValueSc.to_string(),
                                                    loc: location_lib::loc!(),
                                                });
                                            }
                                            Ok(Self(#ValueSc))
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => quote! {
                                            let #EarliestSupportedDateSc = #inner_type_stdrt_not_null_ts::from_ymd_opt(-4713, 12, 31).expect("9f6241e5");
                                            if #ValueSc >= #EarliestSupportedDateSc {
                                                Ok(Self(#ValueSc))
                                            }
                                            else {
                                                Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#EarlierDateNotSupportedUcc {
                                                    #ValueSc: #ValueSc.to_string(),
                                                    #EarliestSupportedDateSc: #EarliestSupportedDateSc.to_string(),
                                                    loc: location_lib::loc!(),
                                                })
                                            }
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                            let #DateSc = match #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc::#TryNewSc(
                                                #ValueSc.#DateSc()
                                            ) {
                                                Ok(v_9be8eddb) => v_9be8eddb,
                                                Err(er) => {
                                                    return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#DateUcc {
                                                        #ErSc,
                                                        loc: location_lib::loc!(),
                                                    });
                                                }
                                            };
                                            let #TimeSc = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc::#TryNewSc(
                                                #ValueSc.#TimeSc()
                                            ) {
                                                Ok(v_993484ce) => v_993484ce,
                                                Err(er) => {
                                                    return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#TimeUcc {
                                                        #ErSc,
                                                        loc: location_lib::loc!(),
                                                    });
                                                }
                                            };
                                            Ok(Self(#inner_type_stdrt_not_null_ts::#NewSc(#DateSc.0, #TimeSc.0)))
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                                #DateNaiveSc.0,
                                                #TimeSc.0
                                            }));
                                            quote! {
                                                let #DateNaiveSc = match #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc::#TryNewSc(#ValueSc.date_naive()) {
                                                    Ok(v_158945ad) => v_158945ad,
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#DateNaiveUcc {
                                                            #ErSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                };
                                                let #TimeSc = match #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc::#TryNewSc(#ValueSc.time()) {
                                                    Ok(v_c5af739c) => v_c5af739c,
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_not_null_origin_try_new_er_ucc::#TimeUcc {
                                                            #ErSc,
                                                            loc: location_lib::loc!(),
                                                        });
                                                    }
                                                };
                                                Ok(Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts))
                                            }
                                        }
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => gen_int_range_check_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => gen_int_range_check_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc),
                                    }
                                }
                                IsNullable::True => gen_match_opt_ts(&ident_stdrt_not_null_origin_ucc),
                            },
                            PgTypePattern::ArrDim1 { dim1_is_nullable } => gen_arr_dims_init_ts(&{
                                let (pg_type_pattern_fb8e939d, is_nullable_104968f1): (&PgTypePattern, &IsNullable) = match &is_nullable {
                                    IsNullable::False => (&PgTypePattern::Stdrt, dim1_is_nullable),
                                    IsNullable::True => (pg_type_pattern, &IsNullable::False),
                                };
                                gen_ident_origin_non_wrapping_8ad5380a(pg_type_pattern_fb8e939d, is_nullable_104968f1)
                            }),
                        }
                    };
                    quote! {
                        pub fn #TryNewSc(#value_ident_inner_type_ts) -> Result<Self, #ident_stdrt_not_null_origin_try_new_er_ucc> {
                            #ts
                        }
                    }
                });
                let maybe_fn_new_or_try_new_for_deserialize_token = match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nullable {
                        IsNullable::False => match &pg_type_deserialize {
                            PgTypeDeserialize::Derive => Ts2::new(),
                            PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDeserialize(pg_type_impl_new_for_deserialize_or_try_new_for_deserialize) => match &pg_type_impl_new_for_deserialize_or_try_new_for_deserialize {
                                PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize(pg_type_impl_new_for_deserialize) => {
                                    let params_ts = {
                                        let gen_start_end_std_std_ops_bound_ts = |ident_ts_46d9ac26: &dyn ToTokens| {
                                            quote! {
                                                #StartSc: std::ops::Bound<#ident_ts_46d9ac26>,
                                                #EndSc: std::ops::Bound<#ident_ts_46d9ac26>
                                            }
                                        };
                                        match &pg_type_impl_new_for_deserialize {
                                            PgTypeImplNewForDeserialize::SsqlxPgTypesPgIntervalAsInterval => quote! {
                                                #MonthsSc: #I32,
                                                #DaysSc: #I32,
                                                #MicrosecondsSc: #I64,
                                            },
                                            PgTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                                #DateSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                                #TimeSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc
                                            },
                                            PgTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote! {
                                                #DateNaiveSc: #sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc,
                                                #TimeSc: #sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc,
                                            },
                                            PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_start_end_std_std_ops_bound_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc),
                                            PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_start_end_std_std_ops_bound_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc),
                                            PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_start_end_std_std_ops_bound_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc),
                                        }
                                    };
                                    let ts = {
                                        let self_sqlx_pg_types_pg_range_ts = quote! {
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
                                        match &pg_type_impl_new_for_deserialize {
                                            PgTypeImplNewForDeserialize::SsqlxPgTypesPgIntervalAsInterval => quote! {
                                                Self(sqlx::postgres::types::PgInterval {
                                                    #MonthsSc,
                                                    #DaysSc,
                                                    #MicrosecondsSc,
                                                })
                                            },
                                            PgTypeImplNewForDeserialize::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                                Self(#inner_type_stdrt_not_null_ts::#NewSc(#DateSc.0, #TimeSc.0))
                                            },
                                            PgTypeImplNewForDeserialize::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                                let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                                    #DateNaiveSc.0,
                                                    #TimeSc.0
                                                }));
                                                quote! {Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts)}
                                            }
                                            PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange | PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange | PgTypeImplNewForDeserialize::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                                                self_sqlx_pg_types_pg_range_ts
                                            }
                                        }
                                    };
                                    quote! {
                                        const fn new_for_deserialize(#params_ts) -> Self {
                                            #ts
                                        }
                                    }
                                }
                                PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(pg_type_impl_try_new_for_deserialize) => {
                                    let params_ts = {
                                        let gen_value_pg_range_int_type_ts = |int_range_type: &IntRangeType| {
                                            let type_ts = {
                                                let ts = int_range_type_to_range_inner_type_ts(int_range_type);
                                                quote! {std::ops::Bound<#ts>}
                                            };
                                            quote! {
                                                start_9a8ef454: #type_ts,
                                                end_a14eb2b9: #type_ts
                                            }
                                        };
                                        match &pg_type_impl_try_new_for_deserialize {
                                            PgTypeImplTryNewForDeserialize::StringAsText | PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => {
                                                quote! {v_356f2a0b: #ident_inner_type_ts}
                                            }
                                            PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                quote! {
                                                    #HourSc: #U32,
                                                    #MinSc: #U32,
                                                    #SecSc: #U32,
                                                    #MicroSc: #U32
                                                }
                                            }
                                            PgTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                quote! {
                                                    #HourSc: #U8,
                                                    #MinuteSc: #U8,
                                                    #SecondSc: #U8,
                                                    #MicrosecondSc: #U32
                                                }
                                            }
                                            PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI32AsInt4Range => gen_value_pg_range_int_type_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                            PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI64AsInt8Range => gen_value_pg_range_int_type_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                        }
                                    };
                                    let ts = {
                                        let gen_self_match_try_new_ts = |params_ts_04a82119: &dyn ToTokens, match_er_vrts_ts: &dyn ToTokens| {
                                            quote! {
                                                match Self::#TryNewSc(#params_ts_04a82119) {
                                                    Ok(v_b318fc86) => Ok(v_b318fc86),
                                                    Err(er) => match er {
                                                        #match_er_vrts_ts
                                                    }
                                                }
                                            }
                                        };
                                        let try_new_convert_pg_range_int_ts = gen_self_match_try_new_ts(
                                            &quote! {sqlx::postgres::types::PgRange { #StartSc: start_9a8ef454, #EndSc: end_a14eb2b9 }},
                                            &quote! {
                                                #ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                } => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                }),
                                                #ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                } => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                }),
                                                #ident_stdrt_not_null_origin_try_new_er_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                } => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                }),
                                                #ident_stdrt_not_null_origin_try_new_er_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                } => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                    #StartSc,
                                                    #EndSc,
                                                    loc,
                                                }),
                                                #ident_stdrt_not_null_origin_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                    #EndSc,
                                                    loc,
                                                } => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                    #EndSc,
                                                    loc,
                                                }),
                                            },
                                        );
                                        match &pg_type_impl_try_new_for_deserialize {
                                            PgTypeImplTryNewForDeserialize::StringAsText => {
                                                let vrt_ts = quote! {
                                                    #ContainsNullByteUcc {
                                                        #ValueSc,
                                                        loc,
                                                    }
                                                };
                                                gen_self_match_try_new_ts(
                                                    &quote!{v_356f2a0b},
                                                    &quote! {
                                                        #ident_stdrt_not_null_origin_try_new_er_ucc::#vrt_ts => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#vrt_ts),
                                                    },
                                                )
                                            }
                                            PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime => {
                                                quote! {
                                                    match #inner_type_stdrt_not_null_ts::from_hms_micro_opt(
                                                        #HourSc,
                                                        #MinSc,
                                                        #SecSc,
                                                        #MicroSc,
                                                    ) {
                                                        Some(v_b143b9e1) => {
                                                            if <#inner_type_stdrt_not_null_ts as chrono::Timelike>::nanosecond(&v_b143b9e1).checked_rem(1000).expect("c0514180") != 0 {
                                                                return Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                                    #ValueSc: v_b143b9e1.to_string(),
                                                                    loc: location_lib::loc!(),
                                                                });
                                                            }
                                                            Ok(Self(v_b143b9e1))
                                                        },
                                                        None => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                            #HourSc,
                                                            #MinSc,
                                                            #SecSc,
                                                            #MicroSc,
                                                            loc: location_lib::loc!(),
                                                        })
                                                    }
                                                }
                                            }
                                            PgTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime => {
                                                quote! {
                                                    match #inner_type_stdrt_not_null_ts::from_hms_micro(
                                                        #HourSc,
                                                        #MinuteSc,
                                                        #SecondSc,
                                                        #MicrosecondSc,
                                                    ) {
                                                        Ok(v_9932d535) => {
                                                            if v_9932d535.nanosecond().checked_rem(1000).expect("0def33ce") != 0 {
                                                                return Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                                    #ValueSc: v_9932d535.to_string(),
                                                                    loc: location_lib::loc!(),
                                                                });
                                                            }
                                                            Ok(Self(v_9932d535))
                                                        },
                                                        Err(er) => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                                            #HourSc,
                                                            #MinuteSc,
                                                            #SecondSc,
                                                            #MicrosecondSc,
                                                            #ErSc: er.to_string(),
                                                            loc: location_lib::loc!(),
                                                        })
                                                    }
                                                }
                                            }
                                            PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate => gen_self_match_try_new_ts(
                                                &quote!{v_356f2a0b},
                                                &quote! {
                                                    #ident_stdrt_not_null_origin_try_new_er_ucc::#EarlierDateNotSupportedUcc {
                                                        #ValueSc,
                                                        #EarliestSupportedDateSc,
                                                        loc,
                                                    } => Err(#ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc::#EarlierDateNotSupportedUcc {
                                                        #ValueSc,
                                                        #EarliestSupportedDateSc,
                                                        loc,
                                                    }),
                                                },
                                            ),
                                            PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI32AsInt4Range | PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI64AsInt8Range => try_new_convert_pg_range_int_ts,
                                        }
                                    };
                                    quote! {
                                        fn #TryNewForDeserializeSc(#params_ts) -> Result<Self, #ident_stdrt_not_null_origin_try_new_for_deserialize_er_ucc> {
                                            #ts
                                        }
                                    }
                                }
                            },
                        },
                        IsNullable::True => Ts2::new(),
                    },
                    PgTypePattern::ArrDim1 { .. } => Ts2::new(),
                };
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    impl #ident_origin_ucc {
                        #fn_new_or_try_new_ts
                        #maybe_fn_new_or_try_new_for_deserialize_token
                    }
                }
            };
            let impl_from_ident_origin_for_ident_inner_type_ts = {
                let ts = {
                    let value_dot_zero = quote! {#ValueSc.0};
                    let gen_match_ts = |
                        match_ts: &dyn ToTokens,
                        some_ts: &dyn ToTokens,
                        some_value_ts: &dyn ToTokens,
                    | quote! {
                        #match_ts.map(|#some_value_ts|#some_value_ts.0#some_ts)
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nullable {
                            IsNullable::False => value_dot_zero,
                            IsNullable::True => gen_match_ts(
                                &value_dot_zero,
                                &Ts2::new(),
                                &quote!{v_6bfd70fa}
                            ),
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => {
                            let el_dot_zero_ts = quote! {el_6910aab7.0};
                            let dim1_ts = match &dim1_is_nullable {
                                IsNullable::False => el_dot_zero_ts,
                                IsNullable::True => gen_match_ts(
                                    &el_dot_zero_ts,
                                    &Ts2::new(),
                                    &quote!{v_1b8cbd77}
                                ),
                            };
                            let into_iter_dim1_ts = quote! {.into_iter().map(|el_6910aab7|#dim1_ts).collect()};
                            match &is_nullable {
                                IsNullable::False => quote! {
                                    #value_dot_zero #into_iter_dim1_ts
                                },
                                IsNullable::True => gen_match_ts(
                                    &value_dot_zero,
                                    &into_iter_dim1_ts,
                                    &quote!{v_38cfcd24}
                                ),
                            }
                        }
                    }
                };
                quote! {
                    impl From<#ident_origin_ucc> for #ident_inner_type_ts {
                        fn from(#ValueSc: #ident_origin_ucc) -> Self {
                            #ts
                        }
                    }
                }
            };
            let maybe_impl_is_string_empty_for_ident_origin_ts = if matches!(&is_stdrt_not_null, IsStdrtNotNull::True) {
                match &is_nullable {
                    IsNullable::False => match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitByPg
                        | PgType::I32AsSerialInitByPg
                        | PgType::I64AsBigSerialInitByPg
                        | PgType::SqlxPgTypesPgMoneyAsMoney
                        | PgType::BoolAsBool
                        | PgType::StdVecVecU8AsBytea
                        | PgType::SqlxTypesChronoNaiveTimeAsTime
                        | PgType::SqlxTypesTimeTimeAsTime
                        | PgType::SqlxPgTypesPgIntervalAsInterval
                        | PgType::SqlxTypesChronoNaiveDateAsDate
                        | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                        | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ts2::new(),
                        PgType::StringAsText => gen_impl_crate_is_string_empty_for_ident_ts(
                            &ident_origin_ucc,
                            &quote! {self.0.clone().is_empty()},
                        ),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_impl_crate_is_string_empty_for_ident_ts(
                            &ident_origin_ucc,
                            &quote! {self.0.to_string().is_empty()},
                        ),
                    },
                    IsNullable::True => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            let maybe_impl_serde_serialize_for_ident_stdrt_not_null_origin_ts = match &serde_serialize_derive_or_impl {
                DeriveOrImpl::Derive => &Ts2::new(),
                DeriveOrImpl::Impl(v) => v,
            };
            let maybe_impl_serde_deserialize_for_ident_stdrt_not_null_origin_ts = match &serde_deserialize_derive_or_impl {
                DeriveOrImpl::Derive => &Ts2::new(),
                DeriveOrImpl::Impl(v) => v,
            };
            let impl_display_for_ident_origin_ts = gen_impl_display_ts(&Ts2::new(), &ident_origin_ucc, &Ts2::new(), &quote! {write!(f, "{self:?}")});
            let impl_location_lib_to_err_string_for_ident_origin_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_origin_ucc, &Ts2::new(), &quote! {self.to_string()});
            let impl_default_opt_some_vec_one_el_for_ident_origin_ts = gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_origin_ucc, &{
                let ts = match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let pg_range_int_default_init_ts = quote! {
                                sqlx::postgres::types::PgRange {
                                    start: std::ops::Bound::Included(#CoreDefault),
                                    end: std::ops::Bound::Excluded(#CoreDefault),
                                }
                            };
                            let gen_as_default_opt_some_vec_one_el_call_ts = |ident_ts_87626b85: &dyn ToTokens| {
                                quote! {
                                    <
                                        #ident_ts_87626b85
                                        as
                                        #import_path::DefaultOptSomeVecOneEl
                                    >::default_opt_some_vec_one_el()
                                }
                            };
                            let gen_sqlx_pg_types_pg_range_default_opt_some_vec_one_el_ts = |ident_ts_a0b3ffd9: &dyn ToTokens| {
                                let ident_as_default_opt_some_vec_one_el_call_ts_d8b3f916 = gen_as_default_opt_some_vec_one_el_call_ts(&ident_ts_a0b3ffd9);
                                quote! {
                                    sqlx::postgres::types::PgRange {
                                        #StartSc: std::ops::Bound::Included(
                                            #ident_as_default_opt_some_vec_one_el_call_ts_d8b3f916.0
                                        ),
                                        #EndSc: std::ops::Bound::Excluded(
                                            #ident_as_default_opt_some_vec_one_el_call_ts_d8b3f916.0
                                        ),
                                    }
                                }
                            };
                            let sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_opt_some_vec_one_el_call_ts = gen_as_default_opt_some_vec_one_el_call_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc);
                            let sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_opt_some_vec_one_el_call_ts = gen_as_default_opt_some_vec_one_el_call_ts(&sqlx_types_chrono_naive_time_as_not_null_time_origin_ucc);
                            let init_ts: &dyn ToTokens = match &pg_type {
                                PgType::I16AsInt2
                                | PgType::I32AsInt4
                                | PgType::I64AsInt8
                                | PgType::F32AsFloat4
                                | PgType::F64AsFloat8
                                | PgType::I16AsSmallSerialInitByPg
                                | PgType::I32AsSerialInitByPg
                                | PgType::I64AsBigSerialInitByPg
                                | PgType::BoolAsBool
                                | PgType::StringAsText
                                | PgType::SqlxTypesChronoNaiveDateAsDate
                                | PgType::SqlxTypesChronoNaiveTimeAsTime
                                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => &quote! {#ft_handle::default()},
                                PgType::SqlxTypesUuidUuidAsUuidInitByClient => &quote! {#ident_inner_type_ts::default()},
                                PgType::SqlxPgTypesPgMoneyAsMoney => &quote! {#inner_type_stdrt_not_null_ts(#CoreDefault)},
                                PgType::StdVecVecU8AsBytea => &quote! {vec![#CoreDefault]},
                                PgType::SqlxTypesTimeTimeAsTime => &gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {0,0,0,0}),
                                PgType::SqlxPgTypesPgIntervalAsInterval => &quote! {#inner_type_stdrt_not_null_ts {
                                    #MonthsSc: #CoreDefault,
                                    #DaysSc: #CoreDefault,
                                    #MicrosecondsSc: #CoreDefault
                                }},
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                    #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_opt_some_vec_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_opt_some_vec_one_el_call_ts.0,
                                }),
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                    #sqlx_types_chrono_naive_date_as_not_null_date_origin_as_default_opt_some_vec_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_not_null_time_origin_as_default_opt_some_vec_one_el_call_ts.0,
                                })),
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet => &quote! {
                                    sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(core::net::Ipv4Addr::UNSPECIFIED, #CoreDefault).expect("9e9c9b57"))
                                },
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => &pg_range_int_default_init_ts,
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_sqlx_pg_types_pg_range_default_opt_some_vec_one_el_ts(&sqlx_types_chrono_naive_date_as_not_null_date_origin_ucc),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_sqlx_pg_types_pg_range_default_opt_some_vec_one_el_ts(&sqlx_types_chrono_naive_date_time_as_not_null_timestamp_origin_ucc),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_sqlx_pg_types_pg_range_default_opt_some_vec_one_el_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_not_null_timestamptz_origin_ucc),
                            };
                            quote! {#init_ts}
                        }
                        IsNullable::True => quote! {Some(#PgCrudCommonDefaultOptSomeVecOneElCall)},
                    },
                    PgTypePattern::ArrDim1 { .. } => match &is_nullable {
                        IsNullable::False => quote! {vec![#PgCrudCommonDefaultOptSomeVecOneElCall]},
                        IsNullable::True => quote! {Some(#PgCrudCommonDefaultOptSomeVecOneElCall)},
                    },
                };
                quote! {Self(#ts)}
            });
            let impl_sqlx_type_for_ident_origin_ts = gen_impl_sqlx_type_for_ident_ts(&ident_origin_ucc, &ft_handle);
            let impl_sqlx_encode_sqlx_pg_for_ident_origin_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_origin_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_pg_for_ident_origin_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(&ident_origin_ucc, &ft_handle, &{
                let scopes_v_ts = quote! {(v)};
                let ok_self_scopes_v_ts = quote! {Ok(Self #scopes_v_ts)};
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nullable {
                        IsNullable::False => match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::F64AsFloat8
                            | PgType::I16AsSmallSerialInitByPg
                            | PgType::I32AsSerialInitByPg
                            | PgType::I64AsBigSerialInitByPg
                            | PgType::SqlxPgTypesPgMoneyAsMoney
                            | PgType::BoolAsBool
                            | PgType::StringAsText
                            | PgType::StdVecVecU8AsBytea
                            | PgType::SqlxTypesChronoNaiveTimeAsTime
                            | PgType::SqlxTypesTimeTimeAsTime
                            | PgType::SqlxPgTypesPgIntervalAsInterval
                            | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                            | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                            | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                            | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                            | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                            | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_v_ts,
                            PgType::SqlxTypesChronoNaiveDateAsDate | PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote! {
                                match Self::#TryNewSc #scopes_v_ts {
                                    Ok(v_93eb5329) => Ok(v_93eb5329),
                                    Err(er) => Err(Box::#NewSc(er)),
                                }
                            },
                        },
                        IsNullable::True => ok_self_scopes_v_ts,
                    },
                    PgTypePattern::ArrDim1 { .. } => ok_self_scopes_v_ts,
                }
            });
            let impl_sqlx_pg_pg_has_arr_type_for_ident_origin_ts = {
                quote! {
                    impl sqlx::postgres::PgHasArrayType for #ident_origin_ucc {
                        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                            <#inner_type_stdrt_not_null_ts as sqlx::postgres::PgHasArrayType>::array_type_info()
                        }
                    }
                }
            };
            let maybe_impl_from_ident_read_for_ident_origin_ts = match &is_not_null_stdrt_can_be_primary_key {
                IsNotNullStdrtCanBePrimaryKey::False => Ts2::new(),
                IsNotNullStdrtCanBePrimaryKey::True => gen_impl_from_ts(&ident_stdrt_not_null_read_ucc, &ident_origin_ucc, &{
                    let ident_stdrt_not_null_as_crate_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_not_null_ucc);
                    quote! {Self::#NewSc(#ident_stdrt_not_null_as_crate_pg_type_ts::into_inner(#ValueSc))}
                }),
            };
            quote! {
                #ident_origin_ts
                #maybe_pub_enum_ident_stdrt_not_null_origin_try_new_er_ts
                #maybe_pub_enum_ident_stdrt_not_null_origin_try_new_for_deserialize_er_ts
                #impl_ident_origin_ts
                #impl_from_ident_origin_for_ident_inner_type_ts
                #maybe_impl_is_string_empty_for_ident_origin_ts
                #maybe_impl_serde_serialize_for_ident_stdrt_not_null_origin_ts
                #maybe_impl_serde_deserialize_for_ident_stdrt_not_null_origin_ts
                #impl_display_for_ident_origin_ts
                #impl_location_lib_to_err_string_for_ident_origin_ts
                #impl_default_opt_some_vec_one_el_for_ident_origin_ts
                #impl_sqlx_type_for_ident_origin_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_origin_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_origin_ts
                #impl_sqlx_pg_pg_has_arr_type_for_ident_origin_ts
                #maybe_impl_from_ident_read_for_ident_origin_ts
            }
        };
        let gen_pub_struct_tokens_ts = |ident_ts_46b769df: &dyn ToTokens, ts: &dyn ToTokens, derive_default: DeriveDefault| {
            StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_default_if(derive_default)
                .derive_clone()
                .derive_copy()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_ts_46b769df,
                    &Ts2::new(),
                    &ts
                )
        };
        let ident_origin_struct_ts = quote!{(#ident_origin_ucc);};
        let ident_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident);
        let ident_table_type_ts = {
            let ident_table_type_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_partial_ord_if(match &is_stdrt_not_null {
                    IsStdrtNotNull::False => DerivePartialOrd::False,
                    IsStdrtNotNull::True => match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitByPg
                        | PgType::I32AsSerialInitByPg
                        | PgType::I64AsBigSerialInitByPg
                        | PgType::BoolAsBool
                        | PgType::StringAsText
                        | PgType::StdVecVecU8AsBytea
                        | PgType::SqlxTypesChronoNaiveTimeAsTime
                        | PgType::SqlxTypesTimeTimeAsTime
                        | PgType::SqlxTypesChronoNaiveDateAsDate
                        | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => DerivePartialOrd::True,
                        PgType::SqlxPgTypesPgMoneyAsMoney
                        | PgType::SqlxPgTypesPgIntervalAsInterval
                        | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                        | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DerivePartialOrd::False,
                    },
                })
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_table_type_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_ts
                );
            let impl_ident_table_type_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_table_type_ucc);
            let impl_default_opt_some_vec_one_el_for_ident_table_type_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_table_type_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            let impl_sqlx_type_for_ident_table_type_ts = gen_impl_sqlx_type_for_ident_ts(&ident_table_type_ucc, &ident_origin_ucc);
            let impl_sqlx_encode_sqlx_pg_for_ident_table_type_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_table_type_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_pg_for_ident_table_type_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(&ident_table_type_ucc, &ident_origin_ucc, &quote! {Ok(Self(v))});
            //todo rewrite as dependency of PgType trait?
            let impl_pg_type_equal_operator_for_ident_table_type_ts = impl_pg_type_equal_operator_for_ident_ts(
                &import_path,
                &ident_table_type_ucc,
                //todo
                &{
                    let equal_ts = EqualOperatorHandle::Equal.to_tokens_path(&import_path);
                    let is_null_ts = EqualOperatorHandle::IsNull.to_tokens_path(&import_path);
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nullable {
                            IsNullable::False => equal_ts,
                            IsNullable::True => quote! {
                                if self.0.0.is_some() {
                                    #equal_ts
                                }
                                else {
                                    #is_null_ts
                                }
                            },
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => match &is_nullable {
                            IsNullable::False => match &dim1_is_nullable {
                                IsNullable::False => equal_ts,
                                IsNullable::True => {
                                    //todo thats not actually usefull coz nullable arr comparison has different logic. need to refactor EqualOperatorHandle enum
                                    equal_ts
                                }
                            },
                            IsNullable::True => quote! {
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
                #ident_table_type_ts
                #impl_ident_table_type_ts
                #impl_default_opt_some_vec_one_el_for_ident_table_type_ts
                #impl_sqlx_type_for_ident_table_type_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_table_type_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_table_type_ts
                #impl_pg_type_equal_operator_for_ident_table_type_ts
            }
        };
        let ident_stdrt_not_null_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
        let ident_create_ts = {
            let ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone()
                    .derive_copy_if(derive_copy)
                    .derive_partial_eq()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_struct(
                        &ident_create_ucc,
                        &Ts2::new(),
                        &ident_origin_struct_ts
                    ),
                CanBePrimaryKey::True => gen_pub_struct_tokens_ts(&ident_create_ucc, &quote! {(());}, DeriveDefault::False),
            };
            let maybe_impl_ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => gen_pub_const_new_or_pub_try_new_ts(&ident_create_ucc),
                CanBePrimaryKey::True => Ts2::new(),
            };
            let impl_default_opt_some_vec_one_el_for_ident_create_ts = gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_create_ucc, &{
                let ts: &dyn ToTokens = match &can_be_primary_key {
                    CanBePrimaryKey::False => &PgCrudCommonDefaultOptSomeVecOneElCall,
                    CanBePrimaryKey::True => &quote! {()},
                };
                quote! {Self(#ts)}
            });
            let maybe_impl_sqlx_encode_sqlx_pg_for_ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_create_ucc, &quote! {#SelfSc.0}),
                CanBePrimaryKey::True => Ts2::new(),
            };
            let maybe_impl_sqlx_type_for_ident_create_ts = match &can_be_primary_key {
                CanBePrimaryKey::False => gen_impl_sqlx_type_for_ident_ts(&ident_create_ucc, &ident_origin_ucc),
                CanBePrimaryKey::True => Ts2::new(),
            };
            quote! {
                #ident_create_ts
                #maybe_impl_ident_create_ts
                #impl_default_opt_some_vec_one_el_for_ident_create_ts
                #maybe_impl_sqlx_encode_sqlx_pg_for_ident_create_ts
                #maybe_impl_sqlx_type_for_ident_create_ts
            }
        };
        let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
        let ident_select_ts = {
            let pub_struct_ident_select_ts = gen_pub_struct_tokens_ts(
                &ident_select_ucc,
                &match &pg_type_pattern {
                    PgTypePattern::Stdrt => quote! {;},
                    PgTypePattern::ArrDim1 { .. } => {
                        let mut arguments_ts = Vec::new();
                        for el0 in 1..=arr_dims_nbr {
                            let dim_nbr_pagination_ts = format!("dim{el0}_pagination").parse::<Ts2>().expect("af86f2d1");
                            arguments_ts.push(quote! {
                                #dim_nbr_pagination_ts: pg_types_common::PaginationStartsWithOne
                            });
                        }
                        quote! {{#(#arguments_ts),*}}
                    }
                },
                DeriveDefault::True,
            );
            let (impl_default_opt_some_vec_one_el_for_ident_select_ts, impl_default_opt_some_vec_one_el_max_page_size_for_ident_select_ts) = {
                let gen_default_ts = |default_some_one_or_default_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| match &pg_type_pattern {
                    PgTypePattern::Stdrt => quote! {Self},
                    PgTypePattern::ArrDim1 { .. } => {
                        let ts: &dyn ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudCommonDefaultOptSomeVecOneElCall,
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudCommonDefaultOptSomeVecOneElMaxPageSizeCall,
                        };
                        let mut arguments_ts = Vec::new();
                        for el0 in 1..=arr_dims_nbr {
                            let dim_nbr_pagination_ts = format!("dim{el0}_pagination").parse::<Ts2>().expect("e5250a98");
                            arguments_ts.push(quote! {
                                #dim_nbr_pagination_ts: #ts
                            });
                        }
                        quote! {Self {#(#arguments_ts),*}}
                    }
                };
                (
                    gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_select_ucc, &gen_default_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)),
                    gen_impl_pg_crud_common_default_opt_some_vec_one_el_max_page_size_ts(&ident_select_ucc, &gen_default_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)),
                )
            };
            quote! {
                #pub_struct_ident_select_ts
                #impl_default_opt_some_vec_one_el_for_ident_select_ts
                #impl_default_opt_some_vec_one_el_max_page_size_for_ident_select_ts
            }
        };
        let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
        let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
        let ident_where_ts = gen_pg_type_where_ts(
            &AllowClippyArbitrarySourceItemOrdering,
            &{
                let common_pg_type_filters = vec![PgTypeFilter::Equal { ident: quote! {#ident_table_type_ucc} }];
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => {
                        let greater_than = PgTypeFilter::GreaterThan {
                            ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                        };
                        let between = PgTypeFilter::Between {
                            ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                        };
                        let in_handle = PgTypeFilter::In { ident: quote! {#ident_table_type_ucc} };
                        let regex = PgTypeFilter::Regex;
                        let equal_to_encoded_string_representation = PgTypeFilter::EqualToEncodedStringRepresentation;
                        let date_9c6d41ca = PgTypeFilter::CurrentDate;
                        let greater_than_current_date = PgTypeFilter::GreaterThanCurrentDate;
                        let time_49c41c1c = PgTypeFilter::CurrentTime;
                        let greater_than_current_time = PgTypeFilter::GreaterThanCurrentTime;
                        let timestamp_ad2e556b = PgTypeFilter::CurrentTimestamp;
                        let greater_than_current_timestamp = PgTypeFilter::GreaterThanCurrentTimestamp;
                        let before = PgTypeFilter::Before {
                            ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                        };
                        // let bit_vec_position_equal = PgTypeFilter::BitVecPositionEqual;
                        let common_stdrt_pg_type_filters = { common_pg_type_filters };
                        let common_stdrt_pg_type_nbr_filters = {
                            let mut vec = common_stdrt_pg_type_filters.clone();
                            vec.push(greater_than.clone());
                            vec.push(between.clone());
                            vec.push(in_handle.clone());
                            vec
                        };
                        let (
                            where_sqlx_pg_types_pg_range_i32_ts,
                            where_sqlx_pg_types_pg_range_i64_ts,
                            where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            where_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ) = {
                            let ranges_common_filter_vec = {
                                let mut vec = common_stdrt_pg_type_filters.clone();
                                vec.push(PgTypeFilter::FindRangesWithinGivenRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::FindRangesThatFullyContainTheGivenRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::StrictlyToLeftOfRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::StrictlyToRightOfRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::IncludedLowerBound {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::ExcludedUpperBound {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::GreaterThanIncludedLowerBound {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::GreaterThanExcludedUpperBound {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::OverlapWithRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::AdjacentWithRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::RangeLength);
                                vec
                            };
                            (ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec.clone(), ranges_common_filter_vec)
                        };
                        match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::F64AsFloat8
                            | PgType::I16AsSmallSerialInitByPg
                            | PgType::I32AsSerialInitByPg
                            | PgType::I64AsBigSerialInitByPg => common_stdrt_pg_type_nbr_filters,
                            PgType::SqlxPgTypesPgMoneyAsMoney => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(in_handle);
                                vec
                            }
                            PgType::StdVecVecU8AsBytea => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(equal_to_encoded_string_representation);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(time_49c41c1c);
                                vec.push(greater_than_current_time);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(date_9c6d41ca);
                                vec.push(greater_than_current_date);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(greater_than);
                                vec.push(between);
                                vec.push(timestamp_ad2e556b);
                                vec.push(greater_than_current_timestamp);
                                vec
                            }
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(before);
                                vec.push(between);
                                vec
                            }
                            PgType::StringAsText | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(regex);
                                vec
                            }
                            PgType::BoolAsBool | PgType::SqlxPgTypesPgIntervalAsInterval | PgType::SqlxTypesIpnetworkIpNetworkAsInet => common_stdrt_pg_type_filters,
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = common_stdrt_pg_type_filters;
                                vec.push(greater_than);
                                vec.push(regex);
                                vec
                            }
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => where_sqlx_pg_types_pg_range_i32_ts,
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => where_sqlx_pg_types_pg_range_i64_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        }
                    }
                    PgTypePattern::ArrDim1 { dim1_is_nullable } => {
                        let ident_stdrt_is_nullable_if_can_be_nullable_table_type_ucc = {
                            let value = SelfTableTypeUcc::from_tokens(&match &pg_type.can_be_nullable() {
                                CanBeNullable::False => quote! {#ident_stdrt_not_null_ucc},
                                CanBeNullable::True => {
                                    let value = gen_ident_ts(pg_type, is_nullable, &PgTypePattern::Stdrt);
                                    quote! {#value}
                                }
                            });
                            quote! {#value}
                        };
                        let dim_one_greater_than = PgTypeFilter::DimOneGreaterThan {
                            ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                        };
                        let dim_one_between = PgTypeFilter::DimOneBetween {
                            ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                        };
                        let dim_one_in_handle = PgTypeFilter::DimOneIn {
                            ident: ident_stdrt_is_nullable_if_can_be_nullable_table_type_ucc,
                        };
                        let dim_one_regex = PgTypeFilter::DimOneRegex;
                        let dim_one_current_date = PgTypeFilter::DimOneCurrentDate;
                        let dim_one_greater_than_current_date = PgTypeFilter::DimOneGreaterThanCurrentDate;
                        let dim_one_current_time = PgTypeFilter::DimOneCurrentTime;
                        let dim_one_greater_than_current_time = PgTypeFilter::DimOneGreaterThanCurrentTime;
                        let dim_one_current_timestamp = PgTypeFilter::DimOneCurrentTimestamp;
                        let dim_one_greater_than_current_timestamp = PgTypeFilter::DimOneGreaterThanCurrentTimestamp;
                        let dim_one_before = PgTypeFilter::DimOneBefore {
                            ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                        };
                        let common_arr_dim1_pg_type_filters = {
                            let mut vec = common_pg_type_filters;
                            vec.push(PgTypeFilter::DimOneEqual {
                                ident: {
                                    let value = SelfTableTypeUcc::from_tokens(&match &dim1_is_nullable {
                                        IsNullable::False => &ident_stdrt_not_null_ucc,
                                        IsNullable::True => &ident_stdrt_nullable_ucc,
                                    });
                                    quote! {#value}
                                },
                            });
                            vec.push(PgTypeFilter::DimOneLengthEqual);
                            vec.push(PgTypeFilter::DimOneLengthGreaterThan);
                            vec
                        };
                        let common_arr_dim1_pg_type_nbr_filters = {
                            let mut vec = common_arr_dim1_pg_type_filters.clone();
                            vec.push(dim_one_greater_than.clone());
                            vec.push(dim_one_between.clone());
                            vec.push(dim_one_in_handle.clone());
                            vec
                        };
                        let (
                            where_sqlx_pg_types_pg_range_i32_ts,
                            where_sqlx_pg_types_pg_range_i64_ts,
                            where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            where_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ) = {
                            let gen_where_sqlx_pg_types_pg_range_filter_ts = |range: Range| {
                                let pg_type_from_range = PgType::from(&range);
                                let range_el_ident_stdrt_not_null_ts = gen_ident_stdrt_not_null_ts(&pg_type_from_range);
                                let mut vec = common_arr_dim1_pg_type_filters.clone();
                                let range_el_ident_stdrt_not_null_as_crate_pg_type_read_ts = {
                                    let range_el_ident_stdrt_not_null_as_crate_pg_type_ts = gen_as_pg_type_ts(&range_el_ident_stdrt_not_null_ts);
                                    quote! {#range_el_ident_stdrt_not_null_as_crate_pg_type_ts::Read}
                                };
                                vec.push(PgTypeFilter::DimOneFindRangesWithinGivenRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::DimOneFindRangesThatFullyContainTheGivenRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::DimOneStrictlyToLeftOfRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::DimOneStrictlyToRightOfRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::DimOneIncludedLowerBound {
                                    ident: range_el_ident_stdrt_not_null_as_crate_pg_type_read_ts.clone(),
                                });
                                vec.push(PgTypeFilter::DimOneExcludedUpperBound {
                                    ident: range_el_ident_stdrt_not_null_as_crate_pg_type_read_ts.clone(),
                                });
                                vec.push(PgTypeFilter::DimOneGreaterThanIncludedLowerBound {
                                    ident: range_el_ident_stdrt_not_null_as_crate_pg_type_read_ts.clone(),
                                });
                                vec.push(PgTypeFilter::DimOneGreaterThanExcludedUpperBound {
                                    ident: range_el_ident_stdrt_not_null_as_crate_pg_type_read_ts,
                                });
                                vec.push(PgTypeFilter::DimOneOverlapWithRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::DimOneAdjacentWithRange {
                                    ident: quote! {#ident_stdrt_not_null_table_type_ucc},
                                });
                                vec.push(PgTypeFilter::DimOneRangeLength);
                                vec
                            };
                            (
                                gen_where_sqlx_pg_types_pg_range_filter_ts(Range::I32AsInt4),
                                gen_where_sqlx_pg_types_pg_range_filter_ts(Range::I64AsInt8),
                                gen_where_sqlx_pg_types_pg_range_filter_ts(Range::SqlxTypesChronoNaiveDateAsDate),
                                gen_where_sqlx_pg_types_pg_range_filter_ts(Range::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                                gen_where_sqlx_pg_types_pg_range_filter_ts(Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                            )
                        };
                        match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::F64AsFloat8
                            | PgType::I16AsSmallSerialInitByPg
                            | PgType::I32AsSerialInitByPg
                            | PgType::I64AsBigSerialInitByPg => common_arr_dim1_pg_type_nbr_filters,
                            PgType::SqlxPgTypesPgMoneyAsMoney => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_in_handle);
                                vec
                            }
                            PgType::StdVecVecU8AsBytea => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(PgTypeFilter::DimOneEqualToEncodedStringRepresentation);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_between);
                                vec.push(dim_one_current_time);
                                vec.push(dim_one_greater_than_current_time);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_between);
                                vec.push(dim_one_current_date);
                                vec.push(dim_one_greater_than_current_date);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_between);
                                vec.push(dim_one_current_timestamp);
                                vec.push(dim_one_greater_than_current_timestamp);
                                vec
                            }
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_before);
                                vec.push(dim_one_between);
                                vec
                            }
                            PgType::StringAsText | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_regex);
                                vec
                            }
                            PgType::BoolAsBool | PgType::SqlxPgTypesPgIntervalAsInterval | PgType::SqlxTypesIpnetworkIpNetworkAsInet => common_arr_dim1_pg_type_filters,
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = common_arr_dim1_pg_type_filters;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_regex);
                                vec
                            }
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => where_sqlx_pg_types_pg_range_i32_ts,
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => where_sqlx_pg_types_pg_range_i64_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => where_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => where_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        }
                    }
                }
            }
            .iter()
            .map(|el0| {
                let el1: &dyn PgFilter = el0;
                el1
            })
            .collect(),
            &ident,
            &ShouldDeriveUtoipaToSchema::False,
            &ShouldDeriveSchemarsJsonSchema::False,
            &IsQueryBindMutable::False,
        );
        let ident_read_ts = {
            let ident_read_ts = {
                let (
                    derive_eq,
                    derive_partial_ord,
                    derive_ord
                ) = match &is_not_null_stdrt_can_be_primary_key {
                    IsNotNullStdrtCanBePrimaryKey::False => (
                        DeriveEq::False,
                        DerivePartialOrd::False,
                        DeriveOrd::False
                    ),
                    IsNotNullStdrtCanBePrimaryKey::True => (
                        DeriveEq::True,
                        DerivePartialOrd::True,
                        DeriveOrd::True
                    ),
                };
                StructOrEnumDeriveTsStreamBuilder::new()
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
                        &Ts2::new(),
                        &ident_origin_struct_ts
                    )
            };
            let impl_ident_read_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_read_ucc);
            let impl_location_lib_to_err_string_for_ident_read_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_read_ucc, &Ts2::new(), &quote! {self.0.to_string()});
            let impl_crate_default_opt_some_vec_one_el_for_ident_read_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_read_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            let impl_sqlx_encode_sqlx_pg_for_ident_origin_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_read_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_read_ucc,
                &ident_origin_ucc,
                &quote! {Ok(Self(v))}
            );
            let impl_sqlx_type_for_ident_read_ts = gen_impl_sqlx_type_for_ident_ts(&ident_read_ucc, &ident_origin_ucc);
            let maybe_impl_pg_type_where_filter_for_ident_read_if_can_be_primary_key_ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
                impl_pg_type_where_filter_for_ident_ts(
                    &quote! {<'lifetime>},
                    &ident_stdrt_not_null_read_ucc,
                    &Ts2::new(),
                    &IncrParamUnderscore::False,
                    &ColumnParamUnderscore::False,
                    &IsNeedToAddLogicalOperatorUnderscore::True,
                    &quote! {
                        match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                            Ok(v_8da76391) => Ok(format!("({column} = ${v_8da76391})")),
                            Err(er) => Err(er)
                        }
                    },
                    &IsQueryBindMutable::True,
                    &gen_typical_query_bind_ts(&SelfSc),
                    &import_path,
                )
            } else {
                Ts2::new()
            };
            quote! {
                #ident_read_ts
                #impl_ident_read_ts
                #impl_location_lib_to_err_string_for_ident_read_ts
                #impl_crate_default_opt_some_vec_one_el_for_ident_read_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_origin_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_read_ts
                #impl_sqlx_type_for_ident_read_ts
                #maybe_impl_pg_type_where_filter_for_ident_read_if_can_be_primary_key_ts
            }
        };
        let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
        let ident_read_only_ids_ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
            let ident_read_only_ids_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_read_only_ids_ucc,
                    &Ts2::new(),
                    &quote!{(#ident_read_ucc);},
                );
            let impl_sqlx_decode_sqlx_pg_for_ident_read_only_ids_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_read_only_ids_ucc,
                &ident_read_ucc,
                &quote! {Ok(Self(v))}
            );
            let impl_sqlx_type_for_ident_read_only_ids_ts = gen_impl_sqlx_type_for_ident_ts(&ident_read_only_ids_ucc, &ident_read_ucc);
            quote! {
                #ident_read_only_ids_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_read_only_ids_ts
                #impl_sqlx_type_for_ident_read_only_ids_ts
            }
        } else {
            Ts2::new()
        };
        let ident_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident);
        let ident_read_inner_ts = quote! {
            pub type #ident_read_inner_ucc = #ident_inner_type_ts;
        };
        let ident_update_ts = {
            let ident_update_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_update_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_ts
                );
            let impl_ident_update_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_update_ucc);
            let impl_default_opt_some_vec_one_el_for_ident_update_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_update_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            let impl_location_lib_to_err_string_for_ident_update_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_update_ucc, &Ts2::new(), &quote! {self.0.#ToErrStringSc()});
            quote! {
                #ident_update_ts
                #impl_ident_update_ts
                #impl_default_opt_some_vec_one_el_for_ident_update_ts
                #impl_location_lib_to_err_string_for_ident_update_ts
            }
        };
        let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
        let ident_update_for_query_ts = {
            let ident_update_for_query_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(
                    &ident_update_for_query_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_ts
                );
            let impl_sqlx_type_for_ident_update_for_query_ts = gen_impl_sqlx_type_for_ident_ts(&ident_update_for_query_ucc, &ident_origin_ucc);
            let impl_sqlx_encode_sqlx_pg_for_ident_update_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_update_for_query_ucc, &quote! {#SelfSc.0});
            let impl_from_ident_update_for_ident_update_for_query_ts = gen_impl_from_ts(&ident_update_ucc, &ident_update_for_query_ucc, &quote! {Self(#ValueSc.0)});
            quote! {
                #ident_update_for_query_ts
                #impl_sqlx_type_for_ident_update_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_update_for_query_ts
                #impl_from_ident_update_for_ident_update_for_query_ts
            }
        };
        let impl_pg_type_for_ident_ts = {
            let gen_ok_string_from_tokens_ts = |ts: &dyn ToTokens| {
                quote! {Ok(#StringTs::from(#ts))}
            };
            let ok_string_from_default_ts = gen_ok_string_from_tokens_ts(&quote! {"default"});
            let ok_string_from_uuid_generate_v4_ts = gen_ok_string_from_tokens_ts(&quote! {"uuid_generate_v4()"});
            let typical_query_part_ts = {
                let if_write_is_err_ts = gen_if_write_is_err_ts(
                    &quote! {acc_c7df00f5, "${v_ba581e0f}"},
                    &gen_return_err_query_part_er_write_into_buffer_ts(import_path)
                );
                quote! {
                    let mut acc_c7df00f5 = String::default();
                    match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                        Ok(v_ba581e0f) => {
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
                let default_init_by_pg: Handle<'_> = (&ok_string_from_default_ts, &ok_query_ts);
                match &pg_type {
                    PgType::I16AsInt2
                    | PgType::I32AsInt4
                    | PgType::I64AsInt8
                    | PgType::F32AsFloat4
                    | PgType::F64AsFloat8
                    | PgType::SqlxPgTypesPgMoneyAsMoney
                    | PgType::BoolAsBool
                    | PgType::StringAsText
                    | PgType::StdVecVecU8AsBytea
                    | PgType::SqlxTypesChronoNaiveTimeAsTime
                    | PgType::SqlxTypesTimeTimeAsTime
                    | PgType::SqlxPgTypesPgIntervalAsInterval
                    | PgType::SqlxTypesChronoNaiveDateAsDate
                    | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                    | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                    | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                    | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                    | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                    | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                    | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                    PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg => default_init_by_pg,
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => (&ok_string_from_uuid_generate_v4_ts, &ok_query_ts),
                }
            };
            let select_only_ids_and_select_only_updated_ids_query_common_ts = {
                let format_ts = dq_ts(&{
                    let column_comma = "{column},";
                    if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) { column_comma.to_owned() } else { format!("'{{{{\\\"value\\\": null}}}}'::jsonb as {column_comma}") }
                });
                quote! {Ok(format!(#format_ts))}
            };
            gen_impl_pg_type_ts(
                &import_path,
                &ident,
                &ident_table_type_ucc,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => IsPrimaryKeyUnderscore::True,
                    CanBePrimaryKey::True => IsPrimaryKeyUnderscore::False,
                },
                &{
                    let pg_query_type = match &pg_type {
                        PgType::I16AsInt2 => "int2",
                        PgType::I32AsInt4 => "int4",
                        PgType::I64AsInt8 => "int8",
                        PgType::F32AsFloat4 => "float4",
                        PgType::F64AsFloat8 => "float8",
                        PgType::I16AsSmallSerialInitByPg => "smallserial",
                        PgType::I32AsSerialInitByPg => "serial",
                        PgType::I64AsBigSerialInitByPg => "bigserial",
                        PgType::SqlxPgTypesPgMoneyAsMoney => "money",
                        PgType::BoolAsBool => "bool",
                        PgType::StringAsText => "text",
                        PgType::StdVecVecU8AsBytea => "bytea",
                        PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => "time",
                        PgType::SqlxPgTypesPgIntervalAsInterval => "interval",
                        PgType::SqlxTypesChronoNaiveDateAsDate => "date",
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => "timestamp",
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => "timestamptz",
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => "uuid",
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet => "inet",
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => "macaddr",
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range => "int4range",
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range => "int8range",
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => "daterange",
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => "tsrange",
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => "tstzrange",
                    };
                    let maybe_arr_part = match &pg_type_pattern {
                        PgTypePattern::Stdrt => String::new(),
                        PgTypePattern::ArrDim1 { .. } => repeat_n("[]", arr_dims_nbr).collect::<String>(),
                    };
                    let maybe_constraint_part = match &pg_type_pattern {
                        PgTypePattern::Stdrt => String::new(),
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => match &dim1_is_nullable {
                            IsNullable::False => ",check (arr_position({column},null) is null)".to_owned(),
                            IsNullable::True => String::new(),
                        },
                    };
                    let maybe_primary_key_is_primary_key_ts = quote! {pg_types_common::maybe_primary_key(is_primary_key)};
                    let column_pg_query_type = format!("{{column}} {pg_query_type}{maybe_arr_part}{maybe_constraint_part}");
                    let column_pg_query_type_not_null = format!("{{column}} {pg_query_type}{maybe_arr_part} not null{maybe_constraint_part}");
                    let space_additional_param = " {}";
                    match (&is_nullable, &can_be_primary_key) {
                        (IsNullable::False, CanBePrimaryKey::False) => {
                            let format_ts = dq_ts(&column_pg_query_type_not_null);
                            quote! {
                                format!(#format_ts)
                            }
                        }
                        (IsNullable::False, CanBePrimaryKey::True) => {
                            let format_ts = dq_ts(&format!("{column_pg_query_type_not_null}{space_additional_param}"));
                            quote! {
                                format!(#format_ts, #maybe_primary_key_is_primary_key_ts)
                            }
                        }
                        (IsNullable::True, CanBePrimaryKey::False) => {
                            let format_ts = dq_ts(&column_pg_query_type);
                            quote! {
                                format!(#format_ts)
                            }
                        }
                        (IsNullable::True, CanBePrimaryKey::True) => {
                            let format_ts = dq_ts(&format!("{column_pg_query_type}{space_additional_param}"));
                            quote! {
                                format!(#format_ts, #maybe_primary_key_is_primary_key_ts)
                            }
                        }
                    }
                },
                &ident_create_ucc,
                &CreateQueryPartValueUnderscore::True,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => CreateQueryPartIncrUnderscore::False,
                    CanBePrimaryKey::True => CreateQueryPartIncrUnderscore::True,
                },
                &query_part_create_ts,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => CreateQueryBindValueUnderscore::False,
                    CanBePrimaryKey::True => CreateQueryBindValueUnderscore::True,
                },
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => IsCreateQueryBindMutable::True,
                    CanBePrimaryKey::True => IsCreateQueryBindMutable::False,
                },
                &bind_value_to_query_create_ts,
                &ident_select_ucc,
                &match &el.pg_type_pattern {
                    PgTypePattern::Stdrt => SelectQueryPartValueUnderscore::True,
                    PgTypePattern::ArrDim1 { .. } => SelectQueryPartValueUnderscore::False,
                },
                &{
                    let ts = match &pg_type_pattern {
                        PgTypePattern::Stdrt => quote! {#ColumnSc.to_owned()},
                        PgTypePattern::ArrDim1 { .. } => {
                            let format_ts = dq_ts(&{
                                let acc = repeat_n("[{}:{}]", arr_dims_nbr).collect::<String>();
                                format!("{{column}}{acc}")
                            });
                            let arguments_ts = (1..=arr_dims_nbr)
                            .map(|el0| {
                                let dim_nbr_pagination_ts = format!("dim{el0}_pagination")
                                .parse::<Ts2>()
                                .expect("6f2305ee");
                                quote! {
                                    #ValueSc.#dim_nbr_pagination_ts.start(),
                                    #ValueSc.#dim_nbr_pagination_ts.end(),
                                }
                            });
                            quote! {format!(
                                #format_ts,
                                #(#arguments_ts)*
                            )}
                        }
                    };
                    quote! {Ok(#ts)}
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &{
                    let gen_ident_read_ident_origin_ts = |ts: &dyn ToTokens| {
                        quote! {#ident_read_ucc(#ident_origin_ucc(#ts))}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nullable {
                            IsNullable::False => {
                                Range::try_from(pg_type).as_ref().map_or_else(
                                    |()| quote! {#ValueSc},
                                    |range| {
                                        let gen_sqlx_pg_types_pg_range_ts = |start_ts: &dyn ToTokens, end_ts: &dyn ToTokens| {
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
                                        let sqlx_pg_types_pg_range_excluded_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&excluded_start_ts, &excluded_end_ts);
                                        let sqlx_pg_types_pg_range_excluded_included_ts = gen_sqlx_pg_types_pg_range_ts(&excluded_start_ts, &included_end_ts);
                                        let sqlx_pg_types_pg_range_included_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &UnboundedUcc);
                                        let sqlx_pg_types_pg_range_unbounded_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&UnboundedUcc, &excluded_end_ts);
                                        let sqlx_pg_types_pg_range_included_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &excluded_end_ts);
                                        let sqlx_pg_types_pg_range_unbounded_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&UnboundedUcc, &UnboundedUcc);
                                        let gen_range_match_ts = |
                                            included_included_ts: &dyn ToTokens,
                                            included_excluded_ts: &dyn ToTokens,
                                            included_unbounded_ts: &dyn ToTokens,
                                            excluded_included_ts: &dyn ToTokens,
                                            excluded_excluded_ts: &dyn ToTokens,
                                            excluded_unbounded_ts: &dyn ToTokens,
                                            unbounded_included_ts: &dyn ToTokens,
                                            unbounded_excluded_ts: &dyn ToTokens
                                        | {
                                            quote! {
                                                #ident_stdrt_not_null_read_ucc(#ident_stdrt_not_null_origin_ucc(match (
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
                                                    (std::ops::Bound::#UnboundedUcc, std::ops::Bound::#UnboundedUcc) => #sqlx_pg_types_pg_range_unbounded_unbounded_ts,
                                                }))
                                            }
                                        };
                                        let gen_if_start_end_equal_ts = |true_ts: &dyn ToTokens, false_ts: &dyn ToTokens| {
                                            quote! {
                                                if #StartSc == #EndSc {
                                                    #true_ts
                                                } else {
                                                    #false_ts
                                                }
                                            }
                                        };
                                        let if_equal_unbounded_unbounded_or_included_excluded_ts = gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_included_excluded_ts);
                                        let int_range_normalize_ts = {
                                            let (
                                                included_start_checked_add_ts,
                                                excluded_end_checked_add_ts
                                            ) = {
                                                let gen_checked_add_one_expect_ts = |first_ts: &dyn ToTokens, second_ts: &dyn ToTokens| {
                                                    quote! {#first_ts(#second_ts.checked_add(1).expect("0ec0992f"))}
                                                };
                                                (
                                                    gen_checked_add_one_expect_ts(&IncludedUcc, &StartSc),
                                                    gen_checked_add_one_expect_ts(&ExcludedUcc, &EndSc)
                                                )
                                            };
                                            let included_excluded_checked_add_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &excluded_end_checked_add_ts);
                                            let included_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &UnboundedUcc);
                                            let included_checked_add_excluded_checked_add_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_checked_add_ts, &excluded_end_checked_add_ts);
                                            let included_checked_add_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_checked_add_ts, &excluded_end_ts);
                                            let included_checked_add_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_checked_add_ts, &UnboundedUcc);
                                            let unbounded_excluded_checked_add_ts = gen_sqlx_pg_types_pg_range_ts(&UnboundedUcc, &excluded_end_checked_add_ts);
                                            let unbounded_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&UnboundedUcc, &excluded_end_ts);
                                            gen_range_match_ts(
                                                &included_excluded_checked_add_ts,
                                                &gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_included_excluded_ts),
                                                &included_unbounded_ts,
                                                &gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_checked_add_ts),
                                                &gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_ts),
                                                &included_checked_add_unbounded_ts,
                                                &unbounded_excluded_checked_add_ts,
                                                &unbounded_excluded_ts,
                                            )
                                        };
                                        let range_match_timestamp_and_timestamp_tz_ts = gen_range_match_ts(
                                            &gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &included_end_ts),
                                            &if_equal_unbounded_unbounded_or_included_excluded_ts,
                                            &sqlx_pg_types_pg_range_included_unbounded_ts,
                                            &gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_excluded_included_ts),
                                            &gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_excluded_excluded_ts),
                                            &gen_sqlx_pg_types_pg_range_ts(&excluded_start_ts, &UnboundedUcc),
                                            &gen_sqlx_pg_types_pg_range_ts(&UnboundedUcc, &included_end_ts),
                                            &sqlx_pg_types_pg_range_unbounded_excluded_ts,
                                        );
                                        match &range {
                                            Range::I32AsInt4 | Range::I64AsInt8 => int_range_normalize_ts,
                                            Range::SqlxTypesChronoNaiveDateAsDate => {
                                                let gen_dot_succ_opt_expect_ts = |id: &dyn Display| {
                                                    let id_dq_ts = dq_ts(&id);
                                                    quote! {.succ_opt().expect(#id_dq_ts)}
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
                                                    &gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &quote! {#ExcludedUcc(#EndSc.succ_opt().expect("9ebce3b4"))}),
                                                    &if_equal_unbounded_unbounded_or_included_excluded_ts,
                                                    &sqlx_pg_types_pg_range_included_unbounded_ts,
                                                    &gen_if_start_end_equal_ts(
                                                        &sqlx_pg_types_pg_range_unbounded_unbounded_ts,
                                                        &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"98a0357b-d21a-4949-a101-c641528d2376"), &gen_excluded_end_succ_opt_ts(&"fe53a6b9-2d7e-4605-9f5a-7f5c21cc01e6")),
                                                    ),
                                                    &gen_if_start_end_equal_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"d8a26635-c478-4a2a-acf4-bf1765702889"), &excluded_end_ts)),
                                                    &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"9811c7c7-d7f5-4fb7-9d25-affb0bd4f5fb"), &UnboundedUcc),
                                                    &gen_sqlx_pg_types_pg_range_ts(&UnboundedUcc, &gen_excluded_end_succ_opt_ts(&"d6288f19-0a24-42ad-9e69-36036d9f2c1d")),
                                                    &sqlx_pg_types_pg_range_unbounded_excluded_ts,
                                                )
                                            }
                                            Range::SqlxTypesChronoNaiveDateTimeAsTimestamp | Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => range_match_timestamp_and_timestamp_tz_ts,
                                        }
                                    }
                                )
                            }
                            IsNullable::True => gen_ident_read_ident_origin_ts(&quote! {
                                #ValueSc.0.0.map(
                                    |v_4561270e|
                                    <
                                        #ident_stdrt_not_null_ucc
                                        as
                                        #import_path::PgType
                                    >::normalize(
                                        #ident_stdrt_not_null_read_ucc(v_4561270e)
                                    ).0
                                )
                            }),
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => match (&is_nullable, &dim1_is_nullable) {
                            (IsNullable::False, IsNullable::False) => gen_ident_read_ident_origin_ts(&quote! {
                                #ValueSc.0.0.into_iter().map(|el|{
                                    #ident_stdrt_not_null_as_pg_type_ts::normalize(
                                        #ident_stdrt_not_null_read_ucc(el)
                                    ).0
                                }).collect()
                            }),
                            (IsNullable::False, IsNullable::True) => gen_ident_read_ident_origin_ts(&{
                                let ident_ts_e4c5a2a3 = gen_ident_ts(pg_type, &IsNullable::True, &PgTypePattern::Stdrt);
                                let ident_arr_stdrt_nullable_read_ucc = SelfReadUcc::from_tokens(&ident_ts_e4c5a2a3);
                                quote! {
                                    #ValueSc.0.0.into_iter().map(|el|{
                                        #ident_stdrt_nullable_as_pg_type_ts::normalize(
                                            #ident_arr_stdrt_nullable_read_ucc(el)
                                        ).0
                                    }).collect()
                                }
                            }),
                            (IsNullable::True, IsNullable::False) => gen_ident_read_ident_origin_ts(&{
                                let ident_arr_dim1_not_null_not_null_ucc = gen_ident_ts(
                                    pg_type,
                                    &IsNullable::False,
                                    &PgTypePattern::ArrDim1 {
                                        dim1_is_nullable: IsNullable::False,
                                    },
                                );
                                let ident_arr_dim1_not_null_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_arr_dim1_not_null_not_null_ucc);
                                quote! {
                                    #ValueSc.0.0.map(|v_b4d912fb|
                                        <
                                            #ident_arr_dim1_not_null_not_null_ucc
                                            as
                                            #import_path::PgType
                                        >::normalize(
                                            #ident_arr_dim1_not_null_not_null_read_ucc(v_b4d912fb),
                                        ).0
                                    )
                                }
                            }),
                            (IsNullable::True, IsNullable::True) => gen_ident_read_ident_origin_ts(&{
                                let ident_arr_dim1_not_null_nullable_ucc = gen_ident_ts(
                                    pg_type,
                                    &IsNullable::False,
                                    &PgTypePattern::ArrDim1 {
                                        dim1_is_nullable: IsNullable::True,
                                    },
                                );
                                let ident_arr_dim1_not_null_nullable_read_ucc = SelfReadUcc::from_tokens(&ident_arr_dim1_not_null_nullable_ucc);
                                quote! {
                                    #ValueSc.0.0.map(
                                        |v_dd042db2|
                                        <
                                            #ident_arr_dim1_not_null_nullable_ucc
                                            as
                                            #import_path::PgType
                                        >::normalize(
                                            #ident_arr_dim1_not_null_nullable_read_ucc(v_dd042db2),
                                        ).0
                                    )
                                }
                            }),
                        },
                    }
                },
                &if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
                    quote! {#ident_read_only_ids_ucc}
                } else {
                    quote! {#import_path_non_primary_key_pg_type_read_only_ids_ts}
                },
                &select_only_ids_and_select_only_updated_ids_query_common_ts,
                &ident_read_inner_ucc,
                &{
                    let gen_ident_stdrt_not_null_into_inner_ident_stdrt_not_null_read_ts = |ts: &dyn ToTokens| {
                        quote! {
                            #ident_stdrt_not_null_as_pg_type_ts::into_inner(
                                #ident_stdrt_not_null_read_ucc(#ts)
                            )
                        }
                    };
                    let value_dot_zero_ts = quote! {#ValueSc.0};
                    let value_dot_zero_dot_zero_ts = quote! {#value_dot_zero_ts.0};
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nullable {
                            IsNullable::False => {
                                if range_try_from_pg_type_is_ok {
                                    gen_pg_range_conversion_ts(&value_dot_zero_dot_zero_ts, &quote!{v_af65ccce})
                                } else {
                                    value_dot_zero_dot_zero_ts
                                }
                            }
                            IsNullable::True => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_not_null_into_inner_ident_stdrt_not_null_read_ts(&quote!{v_bd169d3b})
                                } else {
                                    quote!{v_bd169d3b.0}
                                };
                                quote! {#value_dot_zero_dot_zero_ts.map(|v_bd169d3b| #ts)}
                            }
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nullable } => match (&is_nullable, &dim1_is_nullable) {
                            (IsNullable::False, IsNullable::False) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_not_null_into_inner_ident_stdrt_not_null_read_ts(&quote!{el_f5e94f0c})
                                } else {
                                    quote! {el_f5e94f0c.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.into_iter().map(|el_f5e94f0c|#ts).collect()
                                }
                            }
                            (IsNullable::False, IsNullable::True) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_not_null_into_inner_ident_stdrt_not_null_read_ts(&quote!{v_e9a6bd41})
                                } else {
                                    quote!{v_e9a6bd41.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.into_iter().map(|el_236259fc|
                                        el_236259fc.0.map(|v_e9a6bd41| #ts)
                                    ).collect()
                                }
                            }
                            (IsNullable::True, IsNullable::False) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_not_null_into_inner_ident_stdrt_not_null_read_ts(&quote!{el_b37be63e})
                                } else {
                                    quote! {el_b37be63e.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.map(|v_47fb2e43|
                                        v_47fb2e43.0.into_iter().map(|el_b37be63e|#ts).collect()
                                    )
                                }
                            }
                            (IsNullable::True, IsNullable::True) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_not_null_into_inner_ident_stdrt_not_null_read_ts(&quote!{v_e5c5f65c})
                                } else {
                                    quote!{v_e5c5f65c.0}
                                };
                                quote! {
                                    #value_dot_zero_dot_zero_ts.map(|v_b1a259c4| v_b1a259c4.0.into_iter().map(|el_19a7e6d0|
                                        el_19a7e6d0.0.map(|v_e5c5f65c| #ts)
                                    ).collect())
                                }
                            }
                        },
                    }
                },
                &ident_update_ucc,
                &ident_update_for_query_ucc,
                &UpdateQueryPartValueUnderscore::True,
                &UpdateQueryPartJsonbSetAccumulatorUnderscore::True,
                &UpdateQueryPartJsonbSetTargetUnderscore::True,
                &UpdateQueryPartJsonbSetPathUnderscore::True,
                &typical_query_part_ts,
                &IsUpdateQueryBindMutable::True,
                &typical_query_bind_ts,
                &select_only_ids_and_select_only_updated_ids_query_common_ts,
                &IsSelectOnlyUpdatedIdsQueryBindMutable::False,
                &quote! {Ok(#QuerySc)},
            )
        };
        let impl_pg_type_test_cases_for_ident_ts = {
            enum IsNeedToUseInto {
                False,
                True,
            }
            let gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts = |read_or_update: &ReadOrUpdate| {
                let read_or_update_ucc = read_or_update.ucc();
                let ts = if pg_type_init_try_new_try_from_pg_type.is_ok() {
                    quote! {#TryNewSc(#ValueSc).expect("69477d2f")}
                } else {
                    quote! {#NewSc(#ValueSc)}
                };
                quote! {<#SelfUcc::#PgTypeUcc
                    as
                #import_path::#PgTypeUcc>::#read_or_update_ucc:: #ts}
            };
            let gen_stdrt_not_null_test_case_handle_ts = |is_need_to_use_into: &IsNeedToUseInto| {
                let gen_range_read_only_ids_to_two_dimal_vec_read_inner_ts =
                    |min_ts: &dyn ToTokens, negative_less_typical_ts: &dyn ToTokens, negative_more_typical_ts: &dyn ToTokens, near_zero_ts: &dyn ToTokens, positive_less_typical_ts: &dyn ToTokens, positive_more_typical_ts: &dyn ToTokens, max_ts: &dyn ToTokens| {
                        enum Bnd<'lifetime> {
                            Excl(&'lifetime dyn ToTokens),
                            Incl(&'lifetime dyn ToTokens),
                            Unb,
                        }
                        let ts_08778f0f = [
                            (Bnd::Incl(&MinSc),Bnd::Incl(&MinSc)),
                            (Bnd::Incl(&NegativeLessTypicalSc),Bnd::Incl(&NegativeMoreTypicalSc)),
                            (Bnd::Incl(&NearZeroSc), Bnd::Incl(&NearZeroSc)),
                            (Bnd::Incl(&PositiveLessTypicalSc), Bnd::Incl(&PositiveMoreTypicalSc)),
                            (Bnd::Incl(&MaxSc), Bnd::Incl(&MaxSc)),
                            (Bnd::Incl(&MinSc), Bnd::Incl(&MaxSc)),
                            (Bnd::Incl(&MinSc), Bnd::Excl(&MinSc)),
                            (Bnd::Incl(&NegativeLessTypicalSc), Bnd::Excl(&NegativeMoreTypicalSc)),
                            (Bnd::Incl(&NearZeroSc), Bnd::Excl(&NearZeroSc)),
                            (Bnd::Incl(&PositiveLessTypicalSc), Bnd::Excl(&PositiveMoreTypicalSc)),
                            (Bnd::Incl(&MaxSc), Bnd::Excl(&MaxSc)),
                            (Bnd::Incl(&MinSc), Bnd::Excl(&MaxSc)),
                            (Bnd::Incl(&MinSc), Bnd::Unb),
                            (Bnd::Incl(&NegativeLessTypicalSc), Bnd::Unb),
                            (Bnd::Incl(&NearZeroSc), Bnd::Unb),
                            (Bnd::Incl(&PositiveLessTypicalSc), Bnd::Unb),
                            (Bnd::Incl(&MaxSc), Bnd::Unb),
                            (Bnd::Excl(&MinSc), Bnd::Incl(&MinSc)),
                            (Bnd::Excl(&NegativeLessTypicalSc), Bnd::Incl(&NegativeMoreTypicalSc)),
                            (Bnd::Excl(&NearZeroSc), Bnd::Incl(&NearZeroSc)),
                            (Bnd::Excl(&PositiveLessTypicalSc), Bnd::Incl(&PositiveMoreTypicalSc)),
                            (Bnd::Excl(&MaxSc), Bnd::Incl(&MaxSc)),
                            (Bnd::Excl(&MinSc), Bnd::Incl(&MaxSc)),
                            (Bnd::Excl(&MinSc), Bnd::Excl(&MinSc)),
                            (Bnd::Excl(&NegativeLessTypicalSc), Bnd::Excl(&NegativeMoreTypicalSc)),
                            (Bnd::Excl(&NearZeroSc), Bnd::Excl(&NearZeroSc)),
                            (Bnd::Excl(&PositiveLessTypicalSc), Bnd::Excl(&PositiveMoreTypicalSc)),
                            (Bnd::Excl(&MaxSc), Bnd::Excl(&MaxSc)),
                            (Bnd::Excl(&MinSc), Bnd::Excl(&MaxSc)),
                            (Bnd::Excl(&MinSc), Bnd::Unb),
                            (Bnd::Excl(&NegativeLessTypicalSc), Bnd::Unb),
                            (Bnd::Excl(&NearZeroSc), Bnd::Unb),
                            (Bnd::Excl(&PositiveLessTypicalSc), Bnd::Unb),
                            (Bnd::Excl(&MaxSc), Bnd::Unb),
                            (Bnd::Unb, Bnd::Incl(&MinSc)),
                            (Bnd::Unb, Bnd::Incl(&NegativeMoreTypicalSc)),
                            (Bnd::Unb, Bnd::Incl(&NearZeroSc)),
                            (Bnd::Unb, Bnd::Incl(&PositiveMoreTypicalSc)),
                            (Bnd::Unb, Bnd::Incl(&MaxSc)),
                            (Bnd::Unb, Bnd::Excl(&MinSc)),
                            (Bnd::Unb, Bnd::Excl(&NegativeMoreTypicalSc)),
                            (Bnd::Unb, Bnd::Excl(&NearZeroSc)),
                            (Bnd::Unb, Bnd::Excl(&PositiveMoreTypicalSc)),
                            (Bnd::Unb, Bnd::Excl(&MaxSc)),
                            (Bnd::Unb, Bnd::Unb),
                        ]
                        .into_iter()
                        .map(|(start, end)|{
                            let (start_ts,end_ts) = {
                                let gen_bound_ts = |bnd: Bnd<'_>|{
                                    let ts = match bnd {
                                        Bnd::Excl(ts) => quote!{Excluded(#ts)},
                                        Bnd::Incl(ts) => quote!{Included(#ts)},
                                        Bnd::Unb => quote!{Unbounded},
                                    };
                                    quote::quote!{std::ops::Bound::#ts}
                                };
                                (gen_bound_ts(start), gen_bound_ts(end))
                            };
                            quote!{sqlx::postgres::types::PgRange { start: #start_ts, end: #end_ts}}
                        });
                        quote!{{
                            let #MinSc = #min_ts;
                            let #MaxSc = #max_ts;
                            let #NegativeLessTypicalSc = #negative_less_typical_ts;
                            let #NegativeMoreTypicalSc = #negative_more_typical_ts;
                            let #NearZeroSc = #near_zero_ts;
                            let #PositiveLessTypicalSc = #positive_less_typical_ts;
                            let #PositiveMoreTypicalSc = #positive_more_typical_ts;
                            vec![#(#ts_08778f0f),*]
                        }}
                    };
                let gen_int_pgrange_read_only_ids_to_two_dimal_vec_read_inner_ts = |int_range_type: &IntRangeType| {
                    let range_inner_type_ts = int_range_type_to_range_inner_type_ts(int_range_type);
                    gen_range_read_only_ids_to_two_dimal_vec_read_inner_ts(&quote! {#range_inner_type_ts::MIN}, &quote! {-20}, &quote! {-10}, &quote! {0}, &quote! {10}, &quote! {20}, &quote! {#range_inner_type_ts::MAX - 1})
                };
                let empty_vec_ts = quote! {Vec::new()};
                let gen_ident_stdrt_not_null_function_ts = |
                    ident_8b874ea5: &dyn ToTokens,
                    ts: &dyn ToTokens
                |quote!{#ident_8b874ea5::#ts()};
                let (
                    ident_sqlx_types_chrono_naive_time_min_ts,
                    ident_sqlx_types_chrono_naive_time_ten_ts,
                    ident_sqlx_types_chrono_naive_time_twenty_ts,
                    ident_sqlx_types_chrono_naive_time_max_ts
                ) = {
                    let gen_sqlx_types_chrono_naive_time_as_time_stdrt_not_null_function_ts = |
                        ts_fd88ca39: &dyn ToTokens
                    |gen_ident_stdrt_not_null_function_ts(
                        &gen_ident_stdrt_not_null_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime),
                        &ts_fd88ca39
                    );
                    (
                        gen_sqlx_types_chrono_naive_time_as_time_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_min_function_ts
                        ),
                        gen_sqlx_types_chrono_naive_time_as_time_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_ten_function_ts
                        ),
                        gen_sqlx_types_chrono_naive_time_as_time_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_time_twenty_function_ts
                        ),
                        gen_sqlx_types_chrono_naive_time_as_time_stdrt_not_null_function_ts(
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
                    let sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts = |
                        ts_7c66f815: &dyn ToTokens
                    |gen_ident_stdrt_not_null_function_ts(
                        &gen_ident_stdrt_not_null_ts(&PgType::SqlxTypesChronoNaiveDateAsDate),
                        &ts_7c66f815
                    );
                    (
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_min_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_negative_less_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_negative_more_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_near_zero_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_positive_less_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_positive_more_typical_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_max_function_ts,
                        ),
                        sqlx_types_chrono_naive_date_as_date_stdrt_not_null_function_ts(
                            &sqlx_types_chrono_naive_date_max_pred_opt_expect_function_ts,
                        ),
                    )
                };
                let (
                    sqlx_types_chrono_naive_date_time_min_ts,
                    sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                    sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                    sqlx_types_chrono_naive_date_time_near_zero_ts,
                    sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                    sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                    sqlx_types_chrono_naive_date_time_max_ts,
                ) = {
                    let gen_naive = |date: &dyn ToTokens, time: &dyn ToTokens| {
                        gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! { #date, #time })
                    };
                    (
                        gen_naive(&ident_sqlx_types_chrono_naive_date_min_ts, &ident_sqlx_types_chrono_naive_time_min_ts),
                        gen_naive(&ident_sqlx_types_chrono_naive_date_negative_less_typical_ts, &ident_sqlx_types_chrono_naive_time_twenty_ts),
                        gen_naive(&ident_sqlx_types_chrono_naive_date_negative_more_typical_ts, &ident_sqlx_types_chrono_naive_time_ten_ts),
                        gen_naive(&ident_sqlx_types_chrono_naive_date_near_zero_ts, &ident_sqlx_types_chrono_naive_time_min_ts),
                        gen_naive(&ident_sqlx_types_chrono_naive_date_positive_less_typical_ts, &ident_sqlx_types_chrono_naive_time_ten_ts),
                        gen_naive(&ident_sqlx_types_chrono_naive_date_positive_more_typical_ts, &ident_sqlx_types_chrono_naive_time_twenty_ts),
                        gen_naive(&ident_sqlx_types_chrono_naive_date_max_ts, &ident_sqlx_types_chrono_naive_time_max_ts),
                    )
                };
                let (
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                ) = {
                    let gen_utc = |naive: &dyn ToTokens| gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(naive);
                    (
                        gen_utc(&sqlx_types_chrono_naive_date_time_min_ts),
                        gen_utc(&sqlx_types_chrono_naive_date_time_negative_less_typical_ts),
                        gen_utc(&sqlx_types_chrono_naive_date_time_negative_more_typical_ts),
                        gen_utc(&sqlx_types_chrono_naive_date_time_near_zero_ts),
                        gen_utc(&sqlx_types_chrono_naive_date_time_positive_less_typical_ts),
                        gen_utc(&sqlx_types_chrono_naive_date_time_positive_more_typical_ts),
                        gen_utc(&sqlx_types_chrono_naive_date_time_max_ts),
                    )
                };
                let gen_typical_test_cases_vec_ts = |v: &dyn ToTokens| {
                    let ts = match &is_need_to_use_into {
                        IsNeedToUseInto::True => quote! {.into()},
                        IsNeedToUseInto::False => Ts2::new(),
                    };
                    quote! {#import_path::#v()#ts}
                };
                match &pg_type {
                    PgType::I16AsInt2 => gen_typical_test_cases_vec_ts(&quote! {i16_test_cases_vec}),
                    PgType::I32AsInt4 => gen_typical_test_cases_vec_ts(&quote! {i32_test_cases_vec}),
                    PgType::I64AsInt8 => gen_typical_test_cases_vec_ts(&quote! {i64_test_cases_vec}),
                    PgType::F32AsFloat4 => gen_typical_test_cases_vec_ts(&quote! {f32_test_cases_vec}),
                    PgType::F64AsFloat8 => gen_typical_test_cases_vec_ts(&quote! {f64_test_cases_vec}),
                    PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg => empty_vec_ts,
                    PgType::SqlxPgTypesPgMoneyAsMoney => quote! {
                        #import_path::i64_test_cases_vec().into_iter().map(
                            #inner_type_stdrt_not_null_ts
                        ).collect::<Vec<#inner_type_stdrt_not_null_ts>>()
                    },
                    PgType::BoolAsBool => gen_typical_test_cases_vec_ts(&quote! {bool_test_cases_vec}),
                    PgType::StringAsText => gen_typical_test_cases_vec_ts(&quote! {string_test_cases_vec}),
                    PgType::StdVecVecU8AsBytea => quote! {vec![
                        Vec::new(),
                        (0u8..=255).collect(),
                        vec![0; 1024],
                        vec![0; 1024 * 1024 * 2],
                    ]},
                    PgType::SqlxTypesChronoNaiveTimeAsTime => {
                        let (
                            self_sqlx_types_chrono_naive_time_min_ts,
                            self_sqlx_types_chrono_naive_time_ten_ts,
                            self_sqlx_types_chrono_naive_time_twenty_ts,
                            self_sqlx_types_chrono_naive_time_max_ts,
                        ) = {
                            let gen_self_sqlx_types_chrono_naive_time_stdrt_not_null_function_ts = |ts_9d2b411e: &dyn ToTokens|gen_ident_stdrt_not_null_function_ts(
                                &SelfUcc,
                                &ts_9d2b411e
                            );
                            (
                                gen_self_sqlx_types_chrono_naive_time_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_time_min_function_ts),
                                gen_self_sqlx_types_chrono_naive_time_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_time_ten_function_ts),
                                gen_self_sqlx_types_chrono_naive_time_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_time_twenty_function_ts),
                                gen_self_sqlx_types_chrono_naive_time_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_time_max_function_ts),
                            )
                        };
                        quote! {vec![
                            #self_sqlx_types_chrono_naive_time_min_ts,
                            #self_sqlx_types_chrono_naive_time_ten_ts,
                            #self_sqlx_types_chrono_naive_time_twenty_ts,
                            #self_sqlx_types_chrono_naive_time_max_ts,
                        ]}
                    },
                    PgType::SqlxTypesTimeTimeAsTime => {
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
                    PgType::SqlxPgTypesPgIntervalAsInterval => {
                        let min_ts = quote! {MIN};
                        let max_ts = quote! {MAX};
                        let i32_min_ts = quote! {#I32::#min_ts};
                        let i32_max_ts = quote! {#I32::#max_ts};
                        let gen_sqlx_pg_types_pg_interval_ts = |months_ts: &dyn ToTokens, days_ts: &dyn ToTokens, microseconds_ts: &dyn ToTokens| {
                            quote! {sqlx::postgres::types::PgInterval {
                                months: #months_ts,
                                days: #days_ts,
                                microseconds: #microseconds_ts
                            }}
                        };
                        let min_ts_cfc64470 = gen_sqlx_pg_types_pg_interval_ts(&i32_min_ts, &i32_min_ts, &quote! {#I64::#min_ts});
                        let max_ts_dea6df94 = gen_sqlx_pg_types_pg_interval_ts(&i32_max_ts, &i32_max_ts, &quote! {#I64::#max_ts});
                        quote! {vec![
                            #min_ts_cfc64470,
                            #max_ts_dea6df94
                        ]}
                    }
                    PgType::SqlxTypesChronoNaiveDateAsDate => {
                        let (
                            sqlx_types_chrono_naive_date_min_ts,
                            sqlx_types_chrono_naive_date_negative_less_typical_ts,
                            sqlx_types_chrono_naive_date_negative_more_typical_ts,
                            sqlx_types_chrono_naive_date_near_zero_ts,
                            sqlx_types_chrono_naive_date_positive_less_typical_ts,
                            sqlx_types_chrono_naive_date_positive_more_typical_ts,
                            sqlx_types_chrono_naive_date_max_ts
                        ) = {
                            let gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts = |ts_16bc2a50: &dyn ToTokens|gen_ident_stdrt_not_null_function_ts(
                                &SelfUcc,
                                &ts_16bc2a50
                            );
                            (
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_min_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_negative_less_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_negative_more_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_near_zero_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_positive_less_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_positive_more_typical_function_ts),
                                gen_self_sqlx_types_chrono_naive_date_stdrt_not_null_function_ts(&sqlx_types_chrono_naive_date_max_function_ts)
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
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {vec![
                        #sqlx_types_chrono_naive_date_time_min_ts,
                        #sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        #sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        #sqlx_types_chrono_naive_date_time_near_zero_ts,
                        #sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        #sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        #sqlx_types_chrono_naive_date_time_max_ts,
                    ]},
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote! {vec![
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                    ]},
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => quote! {Vec::new()},
                    PgType::SqlxTypesUuidUuidAsUuidInitByClient => quote! {vec![
                        sqlx::types::Uuid::new_v4()
                    ]},
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet => quote! {vec![
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").expect("478dbded"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").expect("8af9e27e"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").expect("ba86505f"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").expect("32c744a0"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").expect("560815f8"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").expect("793db0ef"),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(std::net::Ipv4Addr::#NewSc(192, 168, 0, 0), 24).expect("c44934f2")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(std::net::Ipv4Addr::#NewSc(10, 0, 0, 0), 8).expect("39e588d9")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(std::net::Ipv4Addr::LOCALHOST, 32).expect("43fb25bd")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#NewSc(std::net::Ipv6Addr::LOCALHOST, 128).expect("b443be46")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#NewSc("2001:db8::".parse().expect("d4e6df27"), 32).expect("a7486c5e")),
                    ]},
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote! {vec![
                        sqlx::types::mac_address::MacAddress::#NewSc([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                        sqlx::types::mac_address::MacAddress::#NewSc([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                        sqlx::types::mac_address::MacAddress::#NewSc([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                        sqlx::types::mac_address::MacAddress::#NewSc([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                        sqlx::types::mac_address::MacAddress::#NewSc([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                        sqlx::types::mac_address::MacAddress::#NewSc([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                    ]},
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_int_pgrange_read_only_ids_to_two_dimal_vec_read_inner_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_int_pgrange_read_only_ids_to_two_dimal_vec_read_inner_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_range_read_only_ids_to_two_dimal_vec_read_inner_ts(
                        &ident_sqlx_types_chrono_naive_date_min_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_near_zero_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_max_pred_opt_expect_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_range_read_only_ids_to_two_dimal_vec_read_inner_ts(
                        &sqlx_types_chrono_naive_date_time_min_ts,
                        &sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_near_zero_ts,
                        &sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_max_ts,
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_range_read_only_ids_to_two_dimal_vec_read_inner_ts(
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
            let opt_vec_create_ts = {
                let gen_some_acc_ts = |
                    is_nullable_27c3e340: &IsNullable,
                    ident_ts_3fadfa9d: &dyn ToTokens,
                    additonal_ts: &dyn ToTokens
                | {
                    let (new_or_try_new_ts, maybe_acc_push_none_ts) = match (&is_nullable_27c3e340, pg_type_init_try_new_try_from_pg_type.is_ok()) {
                        (IsNullable::False, true) => (quote! {try_new(vec![el_0fd5865b.0.into()]).expect("adbae6b3")}, Ts2::new()),
                        (IsNullable::False, false) => (quote! {new(vec![el_0fd5865b.0.into()])}, Ts2::new()),
                        (IsNullable::True, true) => (
                            quote! {try_new(Some(el_0fd5865b.0.into())).expect("b244d498")},
                            quote! {acc_0b59a062.push(#self_as_pg_type_ts::Create::try_new(None).expect("31878971"));},
                        ),
                        (IsNullable::True, false) => (quote! {new(Some(el_0fd5865b.0.into()))}, quote! {acc_0b59a062.push(#self_as_pg_type_ts::Create::new(None));}),
                    };
                    let ident_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_ts_3fadfa9d);
                    quote! {Some({
                        let mut acc_0b59a062 = Vec::new();
                        for el_0fd5865b in #ident_as_pg_type_test_cases_ts::#OptVecCreateSc().unwrap_or(Vec::new()) {
                            acc_0b59a062.push(#self_as_pg_type_ts::Create::#new_or_try_new_ts);
                        }
                        #maybe_acc_push_none_ts
                        #additonal_ts
                        acc_0b59a062
                    })}
                };
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nullable {
                        IsNullable::False => match &can_be_primary_key {
                            CanBePrimaryKey::False => {
                                let ts = gen_stdrt_not_null_test_case_handle_ts(&IsNeedToUseInto::False);
                                let new_or_try_new_ts = {
                                    let self_as_pg_type_create_ts = quote!{#self_as_pg_type_ts::Create};
                                    if pg_type_init_try_new_try_from_pg_type.is_ok() {
                                        quote! {
                                            |el_043a7d30|#self_as_pg_type_create_ts::try_new(
                                                el_043a7d30
                                            ).expect("941bd15c")
                                        }
                                    } else {
                                        quote! {#self_as_pg_type_create_ts::#NewSc}
                                    }
                                };
                                quote! {Some(
                                    #ts.into_iter().map(
                                        #new_or_try_new_ts
                                    ).collect()
                                )}
                            }
                            CanBePrimaryKey::True => none_ts.clone(),
                        },
                        IsNullable::True => gen_some_acc_ts(is_nullable, &gen_ident_ts(pg_type, &IsNullable::False, &PgTypePattern::Stdrt), &Ts2::new()),
                    },
                    PgTypePattern::ArrDim1 { dim1_is_nullable } => gen_some_acc_ts(
                        is_nullable,
                        &gen_ident_ts(
                            pg_type,
                            &match &is_nullable {
                                IsNullable::False => *dim1_is_nullable,
                                IsNullable::True => IsNullable::False,
                            },
                            &match &is_nullable {
                                IsNullable::False => PgTypePattern::Stdrt,
                                IsNullable::True => PgTypePattern::ArrDim1 { dim1_is_nullable: *dim1_is_nullable },
                            },
                        ),
                        &match &is_nullable {
                            IsNullable::False => {
                                let ts: &dyn ToTokens = match &dim1_is_nullable {
                                    IsNullable::False => &ident_stdrt_not_null_as_pg_type_test_cases_ts,
                                    IsNullable::True => &ident_stdrt_nullable_as_pg_type_test_cases_ts,
                                };
                                let (first_ts, second_ts, third_ts) = {
                                    let gen_new_or_try_new_ts = |ts_68722004: &dyn ToTokens| {
                                        if pg_type_init_try_new_try_from_pg_type.is_ok() {
                                            quote! {try_new(#ts_68722004).expect("75ad9383")}
                                        } else {
                                            quote! {new(#ts_68722004)}
                                        }
                                    };
                                    let gen_vec_value_clone_zero_into_nbr_ts = |v: usize| {
                                        let nbr_ts = v.to_string().parse::<Ts2>().expect("50c87202");
                                        //todo maybe correlate with .derive_copy_if()
                                        let maybe_dot_clone_ts_060db18f: &dyn ToTokens = match &pg_type {
                                            PgType::I16AsInt2 |
                                            PgType::I32AsInt4 |
                                            PgType::I64AsInt8 |
                                            PgType::F32AsFloat4 |
                                            PgType::F64AsFloat8 |
                                            PgType::I16AsSmallSerialInitByPg |
                                            PgType::I32AsSerialInitByPg |
                                            PgType::I64AsBigSerialInitByPg |
                                            PgType::SqlxPgTypesPgMoneyAsMoney |
                                            PgType::BoolAsBool |
                                            PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime |
                                            PgType::SqlxPgTypesPgIntervalAsInterval |
                                            PgType::SqlxTypesChronoNaiveDateAsDate |
                                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                                            PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                            PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                                            PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &Ts2::new(),
                                            PgType::StdVecVecU8AsBytea |
                                            PgType::StringAsText => &dot_clone_ts,
                                        };
                                        quote! {vec![v_6465e8ae #maybe_dot_clone_ts_060db18f.0.into(); #nbr_ts]}
                                    };
                                    (
                                        gen_new_or_try_new_ts(&quote! {
                                            #ts::#OptVecCreateSc().unwrap_or(Vec::new())
                                            .into_iter()
                                            .map(|el_ffb375dd|el_ffb375dd.0.into())
                                            .collect()
                                        }),
                                        gen_new_or_try_new_ts(&gen_vec_value_clone_zero_into_nbr_ts(2)),
                                        gen_new_or_try_new_ts(&gen_vec_value_clone_zero_into_nbr_ts(1000)),
                                    )
                                };
                                quote! {
                                    acc_0b59a062.push(#self_as_pg_type_ts::Create::#first_ts);
                                    if let Some(v_6465e8ae) = #ts::#OptVecCreateSc().unwrap_or(Vec::new()).first() {
                                        acc_0b59a062.push(#self_as_pg_type_ts::Create::#second_ts);
                                        acc_0b59a062.push(#self_as_pg_type_ts::Create::#third_ts);
                                    }
                                }
                            }
                            IsNullable::True => Ts2::new(),
                        },
                    ),
                }
            };
            let read_only_ids_to_two_dimal_vec_read_inner_ts = {
                let gen_star_or_dot_clone_ts = |ts|match &pg_type {
                    PgType::I16AsInt2 |
                    PgType::I32AsInt4 |
                    PgType::I64AsInt8 |
                    PgType::F32AsFloat4 |
                    PgType::F64AsFloat8 |
                    PgType::I16AsSmallSerialInitByPg |
                    PgType::I32AsSerialInitByPg |
                    PgType::I64AsBigSerialInitByPg |
                    PgType::SqlxPgTypesPgMoneyAsMoney |
                    PgType::BoolAsBool |
                    PgType::SqlxTypesChronoNaiveTimeAsTime |
                    PgType::SqlxTypesTimeTimeAsTime |
                    PgType::SqlxPgTypesPgIntervalAsInterval |
                    PgType::SqlxTypesChronoNaiveDateAsDate |
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => quote!{*#ts},
                    PgType::StdVecVecU8AsBytea |
                    PgType::StringAsText => quote!{#ts.clone()}
                };
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let ts = gen_stdrt_not_null_test_case_handle_ts(&IsNeedToUseInto::True);
                            quote! {vec![{#ts}]}
                        }
                        IsNullable::True => quote! {
                            #ident_stdrt_not_null_as_pg_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(#ReadOnlyIdsSc)
                            .into_iter()
                            .flat_map(|el0| el0.into_iter().map(|el1| vec![Some(el1)]))
                            .chain(std::iter::once(vec![None]))
                            .collect()
                        },
                    },
                    PgTypePattern::ArrDim1 { dim1_is_nullable } => match &is_nullable {
                        IsNullable::False => match &dim1_is_nullable {
                            IsNullable::False => {
                                let el_d27d1981_ts = gen_star_or_dot_clone_ts(&quote!{el_d27d1981});
                                quote! {
                                    let mut acc_abf96c9f = Vec::new();
                                    let read_only_ids_to_two_dimal_vec_read_inner = #ident_stdrt_not_null_as_pg_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(#ReadOnlyIdsSc);
                                    let opt_additional = {
                                        let mut opt_additional = None;
                                        for el_cb3f4b45 in &read_only_ids_to_two_dimal_vec_read_inner {
                                            if opt_additional.is_some() {
                                                break;
                                            }
                                            for el_d27d1981 in el_cb3f4b45 {
                                                if opt_additional.is_none() {
                                                    opt_additional = Some((vec![
                                                        vec![#el_d27d1981_ts]],
                                                        vec![vec![#el_d27d1981_ts, #el_d27d1981_ts]
                                                    ]));
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                        }
                                        opt_additional
                                    };
                                    let has_len_greater_than_one = {
                                        let mut has_len_greater_than_one = false;
                                        for el_89e74982 in &read_only_ids_to_two_dimal_vec_read_inner {
                                            if el_89e74982.len() > 1 {
                                                has_len_greater_than_one = true;
                                                break;
                                            }
                                        }
                                        has_len_greater_than_one
                                    };
                                    for el_cb836246 in read_only_ids_to_two_dimal_vec_read_inner {
                                        acc_abf96c9f.push(vec![el_cb836246]);
                                    }
                                    if let Some(v_e22f9ad2) = opt_additional {
                                        if has_len_greater_than_one {
                                            acc_abf96c9f.push(v_e22f9ad2.0);
                                        }
                                        else {
                                            acc_abf96c9f.push(v_e22f9ad2.1);
                                        }
                                    }
                                    acc_abf96c9f
                                }
                            }
                            IsNullable::True => {
                                let el_6b831e7c_ts = gen_star_or_dot_clone_ts(&quote!{el_6b831e7c});
                                quote! {
                                    let mut acc_68eba82f = Vec::new();
                                    let read_only_ids_to_two_dimal_vec_read_inner = #ident_stdrt_nullable_as_pg_type_test_cases_ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(#ReadOnlyIdsSc);
                                    let opt_additional = {
                                        let mut opt_additional = None;
                                        for el_b04183c6 in &read_only_ids_to_two_dimal_vec_read_inner {
                                            if opt_additional.is_some() {
                                                break;
                                            }
                                            for el_6b831e7c in el_b04183c6 {
                                                if opt_additional.is_none() {
                                                    opt_additional = Some((
                                                        vec![vec![#el_6b831e7c_ts]],
                                                        vec![vec![#el_6b831e7c_ts, #el_6b831e7c_ts]]
                                                    ));
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                        }
                                        opt_additional
                                    };
                                    let has_len_greater_than_one = read_only_ids_to_two_dimal_vec_read_inner.len() > 1;
                                    acc_68eba82f.push(vec![
                                        read_only_ids_to_two_dimal_vec_read_inner
                                        .into_iter()
                                        .flat_map(IntoIterator::into_iter)
                                        .collect()
                                    ]);
                                    if let Some(v_a0f0f172) = opt_additional {
                                        if has_len_greater_than_one {
                                            acc_68eba82f.push(v_a0f0f172.0);
                                        }
                                        else {
                                            acc_68eba82f.push(v_a0f0f172.1);
                                        }
                                    }
                                    acc_68eba82f
                                }
                            }
                        },
                        IsNullable::True => {
                            let ts = match &dim1_is_nullable {
                                IsNullable::False => &ident_arr_not_null_as_pg_type_test_cases_ts,
                                IsNullable::True => &ident_arr_nullable_as_pg_type_test_cases_ts,
                            };
                            let el_31abc64a_ts = gen_star_or_dot_clone_ts(&quote!{el_31abc64a});
                            quote! {
                                let mut acc_5f7f59ac = Vec::new();
                                let read_only_ids_to_two_dimal_vec_read_inner = #ts::#ReadOnlyIdsToTwoDimalVecReadInnerSc(#ReadOnlyIdsSc);
                                let opt_additional = {
                                    let mut opt_additional = None;
                                    for el_12a259ab in &read_only_ids_to_two_dimal_vec_read_inner {
                                        if opt_additional.is_some() {
                                            break;
                                        }
                                        for el_16a61773 in el_12a259ab {
                                            if opt_additional.is_some() {
                                                break;
                                            }
                                            for el_31abc64a in el_16a61773 {
                                                if opt_additional.is_none() {
                                                    opt_additional = Some((
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
                                    opt_additional
                                };
                                let has_len_greater_than_one = {
                                    let mut has_len_greater_than_one = false;
                                    for el_a177c6a3 in &read_only_ids_to_two_dimal_vec_read_inner {
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
                                    read_only_ids_to_two_dimal_vec_read_inner
                                    .into_iter()
                                    .flatten()
                                    .flatten()
                                    .collect()
                                )]);
                                acc_5f7f59ac.push(vec![None]);
                                if let Some(v_3530786a) = opt_additional {
                                    if has_len_greater_than_one {
                                        acc_5f7f59ac.push(v_3530786a.0);
                                    }
                                    else {
                                        acc_5f7f59ac.push(v_3530786a.1);
                                    }
                                }
                                acc_5f7f59ac
                            }
                        }
                    },
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&ReadOrUpdate::Update);
            let update_to_read_only_ids_ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
                quote! {
                    #ident_read_only_ids_ucc(#ident_read_ucc(#ValueSc.0 #maybe_dot_clone_ts))//todo its not correct. must be only for primary key but it for all types what van be primary key
                }
            } else {
                let value_init_ts = gen_import_path_value_init_ts(&none_ts);
                quote! {
                    #import_path_non_primary_key_pg_type_read_only_ids_ts(#value_init_ts)
                }
            };
            let read_only_ids_to_opt_value_read_default_opt_some_vec_one_el_ts = {
                //todo that is not correct for arr of generated by pg primary keys but maybe just need to remove this vrts and thats it?
                let value_init_ts = gen_import_path_value_init_ts(&{
                    let ts: &dyn ToTokens = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
                        &quote! {#ValueSc.0 #maybe_dot_clone_ts}
                    } else {
                        &PgCrudCommonDefaultOptSomeVecOneElCall
                    };
                    quote! {#self_pg_type_as_pg_type_ts::normalize(#ts)}
                });
                quote! {Some(#value_init_ts)}
            };
            let previous_read_merged_with_opt_update_into_read_ts = quote! {
                #OptUpdateSc.map_or(#ReadSc, |#ValueSc| #ident_read_ucc(#ValueSc.0))
            };
            let read_only_ids_merged_with_create_into_read_ts = {
                let ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
                    quote! {#ReadOnlyIdsSc.0}
                } else {
                    quote! {#ident_read_ucc(#CreateSc.0)}
                };
                quote! {
                    #self_pg_type_as_pg_type_ts::normalize(#ts)
                }
            };
            let read_only_ids_merged_with_create_into_opt_value_read_ts = {
                let value_init_ts = gen_import_path_value_init_ts(&quote! {
                    <Self as #import_path::PgTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoReadSc(
                        #ReadOnlyIdsSc,
                        #CreateSc
                    )
                });
                quote! {Some(#value_init_ts)}
            };
            let read_only_ids_merged_with_create_into_table_type_ts = {
                let ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
                    quote! {#ReadOnlyIdsSc.0.0}
                } else {
                    quote! {#CreateSc.0}
                };
                quote! {#ident_table_type_ucc(#ts)}
            };
            //todo maybe it into function (not in proc macro)
            let read_only_ids_merged_with_create_into_where_equal_ts = {
                let ts = if matches!(&pg_type_pattern, PgTypePattern::Stdrt)
                    && matches!(&is_nullable, IsNullable::False)
                    && matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True)
                {
                    quote! {#ReadOnlyIdsSc.0.0}
                } else {
                    quote! {#CreateSc.0}
                };
                quote! {
                    #ident_where_ucc::#EqualUcc(where_filters::PgTypeWhereEqual {
                        logical_operator: #import_path::LogicalOperator::Or,
                        #ValueSc: #ident_table_type_ucc(#ts),
                    })
                }
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = quote! {
                #import_path::NotEmptyUniqueVec::try_new(vec![
                    #read_only_ids_merged_with_create_into_where_equal_ts
                ]).expect("4c08b551")
            };
            let read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts = none_ts.clone();
            let create_into_pg_type_opt_vec_where_dim_one_equal_ts = match &pg_type_pattern {
                PgTypePattern::Stdrt => none_ts.clone(),
                PgTypePattern::ArrDim1 { dim1_is_nullable } => {
                    let ident_stdrt_is_nullable_table_type_ucc: &dyn ToTokens = match &dim1_is_nullable {
                        IsNullable::False => &ident_stdrt_not_null_table_type_ucc,
                        IsNullable::True => &ident_stdrt_nullable_table_type_ucc,
                    };
                    let some_ts = {
                        let ts: &dyn ToTokens = match &is_nullable {
                            IsNullable::False => &quote! {#CreateSc.0.0},
                            IsNullable::True => &quote! {v_09152b2e.0},
                        };
                        quote! {
                            match #import_path::NotEmptyUniqueVec::try_new({
                                let mut acc_74c71d5d = Vec::new();
                                for (index_7702518c, el_081d735b) in #ts.into_iter().enumerate() {
                                    acc_74c71d5d.push(
                                        #ident_where_ucc::DimOneEqual(
                                            where_filters::PgTypeWhereDimOneEqual {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                dims: where_filters::BoundedVec::try_from(
                                                    vec![
                                                        pg_crud_common::NotZeroUnsignedPartOfI32::try_from(
                                                            i32::try_from(index_7702518c.checked_add(1)?).expect("5954966c")
                                                        ).expect("8d269b8f")
                                                    ]
                                                ).expect("fe1e037f"),
                                                #ValueSc: #ident_stdrt_is_nullable_table_type_ucc(el_081d735b),
                                            }
                                        )
                                    );
                                }
                                acc_74c71d5d
                            }) {
                                Ok(v_2218be19) => Some(v_2218be19),
                                Err(er) => match er {
                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("45c8de3c")
                                }
                            }
                        }
                    };
                    match &is_nullable {
                        IsNullable::False => some_ts,
                        IsNullable::True => quote! {
                            match #CreateSc.0.0 {
                                Some(v_09152b2e) => #some_ts,
                                None => None
                            }
                        },
                    }
                }
            };
            let pg_type_opt_vec_where_greater_than_test_ts = {
                let greater_than = PgTypeGreaterThanVrt::GreaterThan;
                let not_greater_than = PgTypeGreaterThanVrt::NotGreaterThan;
                let equal_not_greater_than = PgTypeGreaterThanVrt::EqualNotGreaterThan;
                let gen_greater_than_test_ts = |greater_than_vrt_ts: &PgTypeGreaterThanVrt, create_ts: &dyn ToTokens, table_type_ts: &dyn ToTokens| {
                    quote! {
                        #import_path::PgTypeGreaterThanTest {
                            vrt: #import_path::PgTypeGreaterThanVrt::#greater_than_vrt_ts,
                            create: #self_as_pg_type_ts::Create::#create_ts,
                            greater_than: #self_as_pg_type_ts::TableType::#table_type_ts,
                        }
                    }
                };
                let gen_greater_than_test_new_new_ts =
                    |greater_than_vrt_ts: &PgTypeGreaterThanVrt, create_ts: &dyn ToTokens, greater_than_ts: &dyn ToTokens| gen_greater_than_test_ts(greater_than_vrt_ts, &quote! {new(#create_ts)}, &quote! {new(#greater_than_ts)});
                let gen_greater_than_test_try_new_try_new_ts = |greater_than_vrt_ts: &PgTypeGreaterThanVrt, create_ts: &dyn ToTokens, greater_than_ts: &dyn ToTokens| {
                    gen_greater_than_test_ts(
                        greater_than_vrt_ts,
                        &quote! {try_new(#create_ts).expect("8327c651")},
                        &quote! {try_new(#greater_than_ts).expect("c369e6ea")},
                    )
                };
                let gen_greater_than_test_new_new_vec_ts = |
                    less_ts: &dyn ToTokens,
                    less_with_more_ts: &dyn ToTokens,
                    zero_ts: &dyn ToTokens,
                    one_ts: &dyn ToTokens, more_ts: &dyn ToTokens, more_with_less_ts: &dyn ToTokens| {
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
                    less_ts: &dyn ToTokens,
                    less_with_more_ts: &dyn ToTokens,
                    zero_ts: &dyn ToTokens,
                    one_ts: &dyn ToTokens,
                    more_ts: &dyn ToTokens,
                    more_with_less_ts: &dyn ToTokens
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
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let wrap_into_not_empty_unique_vec_ts = |ts: &dyn ToTokens| quote! {Some(
                                #import_path::NotEmptyUniqueVec::try_new(vec![#ts]).expect("3ad4b6bf")
                            )};
                            let sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts = &gen_ident_ts(
                                &PgType::SqlxTypesChronoNaiveTimeAsTime,
                                &IsNullable::False,
                                &PgTypePattern::Stdrt
                            );
                            let sqlx_types_chrono_naive_date_as_date_stdrt_not_null_ts = &gen_ident_ts(
                                &PgType::SqlxTypesChronoNaiveDateAsDate,
                                &IsNullable::False,
                                &PgTypePattern::Stdrt
                            );
                            match &pg_type {
                                PgType::I16AsInt2 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#I16::MIN},
                                    &quote!{#I16::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#I16::MAX},
                                    &quote!{#I16::MAX - 1}
                                )),
                                PgType::I32AsInt4 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#I32::MIN},
                                    &quote!{#I32::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#I32::MAX},
                                    &quote!{#I32::MAX - 1}
                                )),
                                PgType::I64AsInt8 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#I64::MIN},
                                    &quote!{#I64::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#I64::MAX},
                                    &quote!{#I64::MAX - 1}
                                )),
                                PgType::F32AsFloat4 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#F32::MIN},
                                    &quote!{#F32::MIN.next_up()},
                                    &quote!{0.0},
                                    &quote!{1.0},
                                    &quote!{#F32::MAX},
                                    &quote!{#F32::MAX.next_down()}
                                )),
                                PgType::F64AsFloat8 => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                //todo rust f64 != pg float8
                                    &quote!{-2.0},
                                    &quote!{-2.0 + 1.0},
                                    &quote!{0.0},
                                    &quote!{1.0},
                                    &quote!{2.0},
                                    &quote!{2.0 - 1.0}
                                )),
                                PgType::SqlxTypesChronoNaiveTimeAsTime => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{Self::min_inner_type()},
                                    &quote!{Self::slightly_more_than_min_inner_type()},
                                    &quote!{Self::middle_inner_type()},
                                    &quote!{Self::slightly_more_than_middle_inner_type()},
                                    &quote!{Self::max_inner_type()},
                                    &quote!{Self::slightly_less_than_max_inner_type()},
                                )),
                                PgType::SqlxTypesTimeTimeAsTime => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{Self::min_inner_type()},
                                    &quote!{Self::slightly_more_than_min_inner_type()},
                                    &quote!{Self::middle_inner_type()},
                                    &quote!{Self::slightly_more_than_middle_inner_type()},
                                    &quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_999).expect("f3d895bb")},
                                    &quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_998).expect("1e71f8c6")},
                                )),
                                PgType::SqlxTypesChronoNaiveDateAsDate => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 30)?},//todo not sure about this values. maybe reuse
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 31)?},
                                    &quote!{Self::middle_inner_type()},
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 2)?},
                                    &quote!{Self::max_inner_type()},
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(262_142, 12, 30)?},
                                )),
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => wrap_into_not_empty_unique_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts::min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts::slightly_more_than_min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_stdrt_not_null_ts::middle_inner_type(),
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts::min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_stdrt_not_null_ts::middle_inner_type(),
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts::slightly_more_than_min_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts::max_inner_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_not_null_ts::slightly_less_than_max_inner_type()
                                    )},
                                )),
                                PgType::I16AsSmallSerialInitByPg |//todo diffrent test logic for autogenerated?
                                PgType::I32AsSerialInitByPg |//todo diffrent test logic for autogenerated?
                                PgType::I64AsBigSerialInitByPg |//todo diffrent test logic for autogenerated?
                                PgType::SqlxPgTypesPgMoneyAsMoney |
                                PgType::BoolAsBool |
                                PgType::StringAsText |
                                PgType::StdVecVecU8AsBytea |
                                PgType::SqlxPgTypesPgIntervalAsInterval |
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                                PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                                PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                                PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => none_ts.clone(),
                            }
                        }
                        IsNullable::True => quote! {
                            <#ident_stdrt_not_null_ucc as #import_path::PgTypeTestCases>::pg_type_opt_vec_where_greater_than_test().map(
                                |el_e4af7fd9|
                                #import_path::NotEmptyUniqueVec::try_new(
                                    el_e4af7fd9
                                    .into_vec()
                                    .into_iter()
                                    .map(|el_504739e6| #import_path::PgTypeGreaterThanTest {
                                        vrt: el_504739e6.vrt,
                                        create: #ident_create_ucc(#ident_origin_ucc(Some(el_504739e6.create.0))),
                                        greater_than: #ident_table_type_ucc(#ident_origin_ucc(Some(el_504739e6.greater_than.0))),
                                    })
                                    .collect()
                                ).expect("63ce5df3")
                            )
                        },
                    },
                    PgTypePattern::ArrDim1 { .. } => none_ts.clone(),
                }
            };
            let read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_ts = match &pg_type_pattern {
                PgTypePattern::Stdrt => {
                    enum IsNeedToImplPgTypeGreaterThanTest {
                        False,
                        TrueFromCreate,
                        TrueFromReadOnlyIds,
                    }
                    enum CreateReadOnlyIds {
                        Create,
                        ReadOnlyIds,
                    }
                    let is_need_to_impl_greater_than_test = match &pg_type {
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::SqlxTypesChronoNaiveTimeAsTime |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => IsNeedToImplPgTypeGreaterThanTest::TrueFromCreate,
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg => IsNeedToImplPgTypeGreaterThanTest::TrueFromReadOnlyIds,
                        PgType::SqlxPgTypesPgMoneyAsMoney |//todo why no support?
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |//todo why no support?
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => IsNeedToImplPgTypeGreaterThanTest::False,
                    };
                    let gen_some_ts = |v_476d047b: &CreateReadOnlyIds| match &is_nullable {
                        IsNullable::False => {
                            let ts = match &v_476d047b {
                                CreateReadOnlyIds::ReadOnlyIds => quote! {#ident_stdrt_not_null_table_type_ucc(#ReadOnlyIdsSc.0.0)},
                                CreateReadOnlyIds::Create => quote! {table_type},
                            };
                            quote! {Some(#ident_where_ucc::GreaterThan(
                                where_filters::PgTypeWhereGreaterThan {
                                    logical_operator: greater_than_vrt.logical_operator(),
                                    #ValueSc: #ts,
                                }
                            ))}
                        }
                        IsNullable::True => {
                            let ts = match &v_476d047b {
                                CreateReadOnlyIds::ReadOnlyIds => quote! {#ReadOnlyIdsSc.0},
                                CreateReadOnlyIds::Create => quote! {#TableTypeSc.0.0},
                            };
                            quote! {
                                #ts.map(|el_886032ca| #ident_where_ucc::GreaterThan(where_filters::PgTypeWhereGreaterThan {
                                    logical_operator: greater_than_vrt.logical_operator(),
                                    value: #ident_stdrt_not_null_table_type_ucc(el_886032ca),
                                }))
                            }
                        }
                    };
                    match &is_need_to_impl_greater_than_test {
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromReadOnlyIds => gen_some_ts(&CreateReadOnlyIds::ReadOnlyIds),
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromCreate => gen_some_ts(&CreateReadOnlyIds::Create),
                        IsNeedToImplPgTypeGreaterThanTest::False => none_ts.clone(),
                    }
                }
                PgTypePattern::ArrDim1 { .. } => none_ts.clone(),
            };
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts = none_ts.clone();
            let create_into_pg_json_type_opt_vec_where_length_equal_ts = none_ts.clone();
            let create_into_pg_json_type_opt_vec_where_length_greater_than_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts = none_ts.clone();
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts = none_ts;
            gen_impl_pg_type_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &import_path,
                &ident_inner_type_ts,
                &ident,
                &opt_vec_create_ts,
                &read_only_ids_to_two_dimal_vec_read_inner_ts,
                &read_inner_into_read_with_new_or_try_new_unwraped_ts,
                &read_inner_into_update_with_new_or_try_new_unwraped_ts,
                &update_to_read_only_ids_ts,
                &read_only_ids_to_opt_value_read_default_opt_some_vec_one_el_ts,
                &previous_read_merged_with_opt_update_into_read_ts,
                &read_only_ids_merged_with_create_into_read_ts,
                &read_only_ids_merged_with_create_into_opt_value_read_ts,
                &read_only_ids_merged_with_create_into_table_type_ts,
                &read_only_ids_merged_with_create_into_where_equal_ts,
                &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
                &read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts,
                &create_into_pg_type_opt_vec_where_dim_one_equal_ts,
                &pg_type_opt_vec_where_greater_than_test_ts,
                &read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts,
                &create_into_pg_json_type_opt_vec_where_length_equal_ts,
                &create_into_pg_json_type_opt_vec_where_length_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts,
            )
        };
        let maybe_impl_pg_type_primary_key_for_ident_stdrt_not_null_if_can_be_primary_key_ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
            let value_as_read_only_ids_ts = quote! {#ValueSc: #self_as_pg_type_ts::#ReadOnlyIdsUcc};
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                impl #import_path::#PgTypePrimaryKeyUcc for #ident_stdrt_not_null_ucc {
                    type #PgTypeUcc = Self;
                    type #TableTypeUcc = #ident_stdrt_not_null_table_type_ucc;
                    fn #ReadOnlyIdsIntoTableTypeSc(#value_as_read_only_ids_ts) -> #self_as_pg_type_ts::#TableTypeUcc {
                        #ident_table_type_ucc(#ValueSc.0.0)
                    }
                    fn #ReadOnlyIdsIntoReadSc(#value_as_read_only_ids_ts) -> #self_as_pg_type_ts::#ReadUcc {
                        #ValueSc.0
                    }
                    fn #ReadOnlyIdsIntoUpdateSc(#value_as_read_only_ids_ts) -> #self_as_pg_type_ts::#UpdateUcc {
                        #ident_update_ucc(#ValueSc.0.0)
                    }
                    fn #ReadIntoTableTypeSc(
                        #ValueSc: #self_as_pg_type_ts::#ReadUcc
                    ) -> #self_as_pg_type_ts::#TableTypeUcc {
                        #ident_table_type_ucc(#ValueSc.0)
                    }
                }
            }
        } else {
            Ts2::new()
        };
        let maybe_impl_pg_type_not_primary_key_for_ident_ts = if matches!(&is_not_null_stdrt_can_be_primary_key, IsNotNullStdrtCanBePrimaryKey::True) {
            Ts2::new()
        } else {
            gen_impl_pg_type_not_primary_key_for_ident_ts(&import_path, &ident)
        };
        let generated = quote! {
            #ident_ts
            #ident_origin_ts
            #ident_table_type_ts
            #ident_create_ts
            #ident_select_ts
            #ident_where_ts
            #ident_read_ts
            #ident_read_only_ids_ts
            #ident_read_inner_ts
            #ident_update_ts
            #ident_update_for_query_ts
            #impl_pg_type_for_ident_ts
            #impl_pg_type_test_cases_for_ident_ts
            #maybe_impl_pg_type_primary_key_for_ident_stdrt_not_null_if_can_be_primary_key_ts
            #maybe_impl_pg_type_not_primary_key_for_ident_ts
        };
        (
            {
                let fi = format!("column_{index}").parse::<Ts2>().expect("2e15af68");
                quote! {
                    pub #fi: pg_crud::pg_type:: #ident,
                }
                .to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    maybe_write_ts_into_file(
        gen_pg_json_types_config.pg_table_columns_write_into_file,
        "pg_table_columns_using_pg_types",
        &{
            let ts = columns_ts
                .into_iter()
                .map(|el_2e3fc869| el_2e3fc869.parse::<Ts2>().expect("79ee6381"))
                .collect::<Vec<Ts2>>();
            quote! {
                struct PgTableColumnsUsingPgTypes {
                    #(#ts)*
                }
            }
        },
        &FormatWithCargofmt::True,
    );
    let generated = {
        let ts = pg_type_arr
            .into_iter()
            .map(|el_f9569807| el_f9569807.parse::<Ts2>().expect("e0c9257d"))
            .collect::<Vec<Ts2>>();
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #GenPgTypesModSc {
                #(#ts)*
            }
            pub use #GenPgTypesModSc::*;
        }
    };
    maybe_write_ts_into_file(
        gen_pg_json_types_config.whole_write_into_file,
        "gen_pg_types",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
