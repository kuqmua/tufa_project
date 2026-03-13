use enum_extension_lib::EnumExtension;
use gen_quotes::dq_ts;
use macros_helpers::gen_impl_try_from_ts;
use macros_helpers::{
    DCopy, DDefault, DEq, DOrd, DPartialOrd, DSerdeDeserialize, DSerdeSerialize, DTsBuilder,
    FormatWithCargofmt, ShouldWriteTsIntoFile, gen_const_new_ts, gen_if_write_is_err_ts,
    gen_impl_display_ts, gen_impl_from_ts, gen_impl_to_err_string_ts, gen_new_ts,
    gen_pub_const_new_ts, gen_pub_new_ts, gen_pub_try_new_ts, mb_write_ts_into_file,
};
use naming::{
    ArrOfUcc, AsUcc, ColSc, ContainsNullByteUcc, CrSc, DateNaiveSc, DateNaiveUcc, DateSc, DateUcc,
    DaysSc, DisplayPlusToTokens, EarlierDateNotSupportedUcc, EarliestSupportedDateSc, EndSc,
    EndUcc, EqUcc, ErSc, ExcludedStartGreaterThanExcludedEndUcc,
    ExcludedStartGreaterThanIncludedEndUcc, ExcludedUcc, GenPgTypesModSc, HourSc,
    IncludedEndCannotBeMaxUcc, IncludedStartGreaterThanExcludedEndUcc,
    IncludedStartGreaterThanIncludedEndUcc, IncludedUcc, IncrSc,
    InvalidHourOrMinuteOrSecondOrMicrosecondUcc, MaxSc, MicroSc, MicrosecondSc, MicrosecondsSc,
    MinSc, MinuteSc, MonthsSc, NanosecondPrecisionIsNotSupportedUcc, NanosecondSc, NearZeroSc,
    NegativeLessTypicalSc, NegativeMoreTypicalSc, NewSc, NotUuidUcc, OptUpdSc, OptVecCrSc,
    PgTypePkUcc, PgTypeUcc, PositiveLessTypicalSc, PositiveMoreTypicalSc, QuerySc,
    RdIdsAndCrIntoRdSc, RdIdsIntoRdSc, RdIdsIntoTtSc, RdIdsIntoUpdSc, RdIdsSc,
    RdIdsTo2DimsVecRdInnSc, RdIdsUcc, RdIntoTtSc, RdSc, RdUcc, SecSc, SecondSc, SelfSc, SelfUcc,
    StartSc, StartUcc, TimeSc, TimeUcc, ToErrStringSc, TryNewSc, TtSc, TtUcc, UnboundedUcc, UpdUcc,
    VSc, VecOfUcc,
    prm::{
        SelfCrUcc, SelfNnUcc, SelfOrgnTryNewErUcc, SelfOrgnTryNewForDeErUcc, SelfOrgnUcc,
        SelfRdIdsUcc, SelfRdInnUcc, SelfRdUcc, SelfSelUcc, SelfTtUcc, SelfUpdForQueryUcc,
        SelfUpdUcc, SelfWhUcc,
    },
};
use optml::Optml;
use panic_loc::panic_loc;
use pg_crud_cmn_and_macros_cmn::PgTypeGreaterThanVrt;
use pg_crud_macros_cmn::{
    AddOprtrUndrscr, ColPrmUndrscr, CrQbValueUndrscr, CrQpIncrUndrscr, CrQpValueUndrscr,
    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, DeriveOrImpl, EqOprtrH, Import, IncrPrmUndrscr,
    IsCrQbMut, IsNl, IsPkUndrscr, IsQbMut, IsSelOnlyUpddIdsQbMut, IsStdrtNn, IsUpdQbMut, PgFlt,
    PgTypeFlt, RdOrUpd, SelQpValueUndrscr, ShouldDSchemarsJsonSchema, ShouldDeriveUtoipaToSchema,
    UpdQpJsonbSetAccumulatorUndrscr, UpdQpJsonbSetPathUndrscr, UpdQpJsonbSetTargetUndrscr,
    UpdQpValueUndrscr, gen_impl_crate_is_string_empty_for_ident_ts,
    gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts,
    gen_impl_pg_crud_cmn_dflt_some_one_el_ts, gen_impl_pg_type_not_pk_for_ident_ts,
    gen_impl_pg_type_test_cases_for_ident_ts, gen_impl_pg_type_ts,
    gen_impl_sqlx_decode_sqlx_pg_for_ident_ts, gen_impl_sqlx_encode_sqlx_pg_for_ident_ts,
    gen_impl_sqlx_type_for_ident_ts, gen_opt_type_dcl_ts, gen_pg_type_wh_ts,
    gen_return_err_qp_er_write_into_buffer_ts, gen_v_init_ts, gen_vec_tokens_dcl_ts,
    impl_pg_type_eq_oprtr_for_ident_ts, impl_pg_type_wh_flt_for_ident_ts,
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
    AllowClippyArbitrarySrcItemOrdering, CoreDefault, F32, I16, I32, I64, MustUse,
    PgCrudCmnDfltSomeOneElCall, PgCrudCmnDfltSomeOneElMaxPageSizeCall, StringTs, U8, U32,
};
#[must_use]
pub fn gen_pg_types(input_ts: &Ts2) -> Ts2 {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, StrumDisplay, Optml)]
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
    #[derive(Debug, StrumDisplay, Optml)]
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
        Debug,
        Clone,
        PartialEq,
        Serialize,
        Deserialize,
        StrumDisplay,
        EnumIter,
        EnumExtension,
        Optml,
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
    enum CanBeNl {
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
        const fn can_be_nl(&self) -> CanBeNl {
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
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeNl::True,
                Self::I16AsSmallSerialInitByPg | Self::I32AsSerialInitByPg | Self::I64AsBigSerialInitByPg | Self::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBeNl::False,
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
            write!(f, "{}", SelfNnUcc::from_display(&PgType::from(self)))
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
    // todo reuse it(move to pg_macros_cmn) if sqlx devs will add nested arr support
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Serialize,
        Deserialize,
        StrumDisplay,
        EnumIter,
        EnumExtension,
        Optml,
    )]
    enum PgTypePattern {
        Stdrt,
        ArrDim1 { dim1_is_nl: IsNl },
        // sqlx does not support nested arrs yet. https://github.com/launchbadge/sqlx/issues/2280
        // ArrDim2 {
        //     dim1_is_nl: IsNl,
        //     dim2_is_nl: IsNl,
        // },
        // ArrDim3 {
        //     dim1_is_nl: IsNl,
        //     dim2_is_nl: IsNl,
        //     dim3_is_nl: IsNl,
        // },
        // ArrDim4 {
        //     dim1_is_nl: IsNl,
        //     dim2_is_nl: IsNl,
        //     dim3_is_nl: IsNl,
        //     dim4_is_nl: IsNl,
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
    #[derive(Debug, PartialEq, Serialize, Deserialize, Optml)]
    #[serde(try_from = "PgTypeRecordRaw")]
    struct PgTypeRecord {
        pg_type: PgType,
        is_nl: IsNl,
        pg_type_pattern: PgTypePattern,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize, Optml)]
    struct PgTypeRecordRaw {
        pg_type: PgType,
        is_nl: IsNl,
        pg_type_pattern: PgTypePattern,
    }
    impl TryFrom<PgTypeRecordRaw> for PgTypeRecord {
        type Error = String;
        fn try_from(v: PgTypeRecordRaw) -> Result<Self, Self::Error> {
            let cant_supp_nl_vrts_msg = "cant support nl vrts: ";
            let cant_supp_arr_ver_msg = "cant support arr_version: ";
            match &v.pg_type.can_be_nl() {
                CanBeNl::False => {
                    if matches!(&v.is_nl, IsNl::True) {
                        return Err(format!("{cant_supp_nl_vrts_msg}{v:#?}"));
                    }
                    match &v.pg_type_pattern {
                        PgTypePattern::Stdrt => Ok(Self {
                            pg_type: v.pg_type,
                            is_nl: v.is_nl,
                            pg_type_pattern: v.pg_type_pattern,
                        }),
                        PgTypePattern::ArrDim1 { dim1_is_nl } => {
                            match &v.pg_type.can_be_an_arr_el() {
                                CanBeAnArrEl::False => {
                                    Err(format!("{cant_supp_arr_ver_msg}{v:#?}"))
                                }
                                CanBeAnArrEl::True => match &dim1_is_nl {
                                    IsNl::False => Ok(Self {
                                        pg_type: v.pg_type,
                                        is_nl: v.is_nl,
                                        pg_type_pattern: v.pg_type_pattern,
                                    }),
                                    IsNl::True => Err(format!("{cant_supp_nl_vrts_msg}{v:#?}")),
                                },
                            }
                        }
                    }
                }
                CanBeNl::True => match &v.pg_type_pattern {
                    PgTypePattern::Stdrt => Ok(Self {
                        pg_type: v.pg_type,
                        is_nl: v.is_nl,
                        pg_type_pattern: v.pg_type_pattern,
                    }),
                    PgTypePattern::ArrDim1 { .. } => match &v.pg_type.can_be_an_arr_el() {
                        CanBeAnArrEl::False => Err(format!("{cant_supp_arr_ver_msg}{v:#?}")),
                        CanBeAnArrEl::True => Ok(Self {
                            pg_type: v.pg_type,
                            is_nl: v.is_nl,
                            pg_type_pattern: v.pg_type_pattern,
                        }),
                    },
                },
            }
        }
    }
    #[derive(Debug, serde::Deserialize, Optml)]
    enum GenPgTypesConfigVrt {
        All,
        Concrete(Vec<PgTypeRecord>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, Optml)]
    struct GenPgJsonsConfig {
        vrt: GenPgTypesConfigVrt,
        pg_tbl_cols_write_into_file: ShouldWriteTsIntoFile,
        whole_write_into_file: ShouldWriteTsIntoFile,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Optml)]
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
    #[derive(Debug, Optml)]
    enum PgTypeImplTryNewForDe {
        StringAsText,
        SqlxTypesChronoNaiveTimeAsTime,
        SqlxTypesTimeTimeAsTime,
        SqlxTypesChronoNaiveDateAsDate,
        SqlxPgTypesPgRangeI32AsInt4Range,
        SqlxPgTypesPgRangeI64AsInt8Range,
        SqlxTypesUuidUuidAsUuidV4InitByPg,
        SqlxTypesUuidUuidAsUuidInitByClient,
    }
    #[derive(Debug, Optml)]
    enum PgTypeImplNewForDeserializeOrTryNewForDe {
        NewForDeserialize,
        TryNewForDe(PgTypeImplTryNewForDe),
    }
    #[derive(Debug, Optml)]
    enum PgTypeDeserialize {
        Derive,
        ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe),
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
                PgType::StringAsText => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::StringAsText)),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime)),
                PgType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime)),
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate)),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range)),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range)),
                PgType::SqlxPgTypesPgIntervalAsInterval |
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize),
            }
        }
    }
    panic_loc();
    let gen_pg_json_config = from_str::<GenPgJsonsConfig>(&input_ts.to_string()).expect("80485f71");
    let (cols_ts, pg_type_arr) = {
        let acc = match gen_pg_json_config.vrt {
            GenPgTypesConfigVrt::All => PgType::into_arr().into_iter().fold(Vec::new(), |mut acc0, el| {
                for el0 in PgTypePattern::into_arr().into_iter().fold(Vec::new(), |mut acc1, el1| {
                    match &el1 {
                        PgTypePattern::Stdrt => {
                            acc1.push(el1);
                        }
                        PgTypePattern::ArrDim1 { .. } => {
                            for el2 in IsNl::into_arr() {
                                acc1.push(PgTypePattern::ArrDim1 { dim1_is_nl: el2 });
                            }
                        }
                    }
                    acc1
                }) {
                    match &el0 {
                        PgTypePattern::Stdrt => match &el.can_be_nl() {
                            CanBeNl::False => {
                                acc0.push(PgTypeRecord {
                                    pg_type: el.clone(),
                                    is_nl: IsNl::False,
                                    pg_type_pattern: el0,
                                });
                            },
                            CanBeNl::True => IsNl::into_arr().into_iter().for_each(|el1| {
                                acc0.push(PgTypeRecord {
                                    pg_type: el.clone(),
                                    is_nl: el1,
                                    pg_type_pattern: el0.clone(),
                                });
                            }),
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nl } => match &el.can_be_an_arr_el() {
                            CanBeAnArrEl::False => (),
                            CanBeAnArrEl::True => match &el.can_be_nl() {
                                CanBeNl::False => {
                                    if matches!(&dim1_is_nl, IsNl::False) {
                                        for el1 in IsNl::into_arr() {
                                            acc0.push(PgTypeRecord {
                                                pg_type: el.clone(),
                                                is_nl: el1,
                                                pg_type_pattern: PgTypePattern::ArrDim1 { dim1_is_nl: *dim1_is_nl },
                                            });
                                        }
                                    }
                                },
                                CanBeNl::True => IsNl::into_arr().into_iter().for_each(|is_nl| {
                                    acc0.push(PgTypeRecord {
                                        pg_type: el.clone(),
                                        is_nl,
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
            #[derive(Clone, Optml)]
            struct PgTypeRecordH {
                is_nl: IsNl,
                pg_type_pattern: PgTypePattern,
            }
            fn gen_pg_type_record_h_vec(
                pg_type_record_h: PgTypeRecordH,
            ) -> Vec<PgTypeRecordH> {
                let gen_vec = |
                    pg_type_record_h_7ca933c5: PgTypeRecordH
                | gen_pg_type_record_h_vec(pg_type_record_h_7ca933c5)
                .into_iter()
                .chain(once(pg_type_record_h.clone()))
                .collect();
                //same pattern was in gen_pg_types 21.05.2025
                match (
                    &pg_type_record_h.is_nl,
                    &pg_type_record_h.pg_type_pattern,
                ) {
                    (IsNl::False, PgTypePattern::Stdrt) => {
                        vec![pg_type_record_h]
                    }
                    (IsNl::True, PgTypePattern::Stdrt) => {
                        gen_vec(PgTypeRecordH {
                            is_nl: IsNl::False,
                            pg_type_pattern: PgTypePattern::Stdrt,
                        })
                    }
                    (
                        IsNl::False,
                        PgTypePattern::ArrDim1 {
                            dim1_is_nl,
                        },
                    ) => gen_vec(PgTypeRecordH {
                        is_nl: *dim1_is_nl,
                        pg_type_pattern: PgTypePattern::Stdrt,
                    }),
                    (
                        IsNl::True,
                        PgTypePattern::ArrDim1 { .. },
                    ) => gen_vec(PgTypeRecordH {
                        is_nl: IsNl::False,
                        pg_type_pattern: pg_type_record_h
                            .pg_type_pattern
                            .clone(),
                    }),
                }
            }
            for el0 in gen_pg_type_record_h_vec(PgTypeRecordH {
                is_nl: el.is_nl,
                pg_type_pattern: el.pg_type_pattern,
            }) {
                let v_88571cb8 = PgTypeRecord {
                    pg_type: el.pg_type.clone(),
                    is_nl: el0
                        .is_nl,
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
    .map(|(i, el)| {
        enum PgTypeOrPgTypeTestCases {
            PgType,
            PgTypeTestCases,
        }
        enum CanBePk {
            False,
            True,
        }
        enum IsNnStdrtCanBePk {
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
        type H<'lt> = (&'lt dyn ToTokens, &'lt dyn ToTokens);
        fn gen_pg_range_conversion_ts(match_ts: &dyn ToTokens, input_ts: &dyn ToTokens) -> Ts2 {
            let arms_ts = quote! {
                std::ops::Bound::Included(v_af65ccce) => std::ops::Bound::Included(#input_ts),
                std::ops::Bound::Excluded(v_af65ccce) => std::ops::Bound::Excluded(#input_ts),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            };
            quote! {
                sqlx::postgres::types::PgRange {
                    start: match #match_ts.start { #arms_ts },
                    end: match #match_ts.end { #arms_ts },
                }
            }
        }
        let pg_type = &el.pg_type;
        let is_nl = &el.is_nl;
        let pg_type_pattern = &el.pg_type_pattern;
        let pg_type_init_try_new_try_from_pg_type = PgTypeInitTryNew::try_from(pg_type);
        let pg_type_deserialize = PgTypeDeserialize::from(pg_type);
        let arr_dims_nbr = pg_type_pattern.arr_dims_nbr();
        let range_try_from_pg_type = Range::try_from(pg_type);
        let range_try_from_pg_type_is_ok = range_try_from_pg_type.is_ok();
        let import = Import::PgCrudCmn;
        let import_non_pk_pg_type_rd_ids_ts = quote! {#import::NonPkPgTypeRdIds};
        let none_ts = quote!{None};
        let dot_clone_ts = quote!{.clone()};
        let mb_dot_clone_ts: &dyn ToTokens = if matches!(&pg_type_pattern, PgTypePattern::Stdrt) &&
            matches!(&is_nl, IsNl::False) && !matches!(
                pg_type,
                PgType::StdVecVecU8AsBytea | PgType::StringAsText
            )
        {
            &Ts2::new()
        } else {
            &dot_clone_ts
        };
        let gen_v_init_ts0 = |ts: &dyn ToTokens| gen_v_init_ts(&import, &ts);
        let gen_ident_str = |
            pg_type_73b7c8af: &PgType,
            is_nl_a5a792df: &IsNl,
            pg_type_pattern_4f1e15f0: &PgTypePattern
        | {
            let rust_type_name = RustTypeName::from(pg_type_73b7c8af);
            let pg_type_name = PgTypeName::from(pg_type_73b7c8af);
            let is_nl_rust = is_nl_a5a792df.rust();
            let nn_or_nl_str = is_nl_a5a792df.nn_or_nl_str();
            let (rust_part, pg_part) = match &pg_type_pattern_4f1e15f0 {
                PgTypePattern::Stdrt => (format!("{rust_type_name}"), format!("{pg_type_name}")),
                PgTypePattern::ArrDim1 { dim1_is_nl } => {
                    let d1 = dim1_is_nl.nn_or_nl_str();
                    let d1_rust = dim1_is_nl.rust();
                    (format!("{VecOfUcc}{d1_rust}{rust_type_name}"), format!("{ArrOfUcc}{d1}{pg_type_name}"))
                }
            };
            format!("{is_nl_rust}{rust_part}{AsUcc}{nn_or_nl_str}{pg_part}")
        };
        let gen_ident_ts = |
            pg_type_f8ca5f3f: &PgType,
            is_nl_ea26dfba: &IsNl,
            pg_type_pattern_b0eedab6: &PgTypePattern
        | gen_ident_str(
            pg_type_f8ca5f3f,
            is_nl_ea26dfba,
            pg_type_pattern_b0eedab6
        ).parse::<Ts2>().expect("ff3eb7a6");
        let ident = &gen_ident_ts(pg_type, is_nl, pg_type_pattern);
        let gen_ident_stdrt_nn_ts = |v: &PgType| gen_ident_ts(v, &IsNl::False, &PgTypePattern::Stdrt);
        let ident_stdrt_nn_ucc = gen_ident_stdrt_nn_ts(pg_type);
        let ident_stdrt_nl_ucc = gen_ident_ts(pg_type, &IsNl::True, &PgTypePattern::Stdrt);
        let ident_arr_nn_ucc = gen_ident_ts(
            pg_type,
            &IsNl::False,
            &PgTypePattern::ArrDim1 {
                dim1_is_nl: IsNl::False,
            },
        );
        let ident_arr_nl_ucc = gen_ident_ts(
            pg_type,
            &IsNl::False,
            &PgTypePattern::ArrDim1 {
                dim1_is_nl: IsNl::True,
            },
        );
        let gen_as_trait_ts = |ts: &dyn ToTokens, pg_type_or_pg_type_test_cases: &PgTypeOrPgTypeTestCases| {
            let trait_ts = match &pg_type_or_pg_type_test_cases {
                PgTypeOrPgTypeTestCases::PgType => quote! {PgType},
                PgTypeOrPgTypeTestCases::PgTypeTestCases => quote! {PgTypeTestCases},
            };
            quote! {<#ts as #import::#trait_ts>}
        };
        let gen_as_pg_type_ts = |ts: &dyn ToTokens| gen_as_trait_ts(&ts, &PgTypeOrPgTypeTestCases::PgType);
        let gen_as_pg_type_test_cases_ts = |ts: &dyn ToTokens| gen_as_trait_ts(&ts, &PgTypeOrPgTypeTestCases::PgTypeTestCases);
        let self_as_pg_type_ts = gen_as_pg_type_ts(&SelfUcc);
        let ident_stdrt_nn_as_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_nn_ucc);
        let ident_stdrt_nl_as_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_nl_ucc);
        let self_pg_type_as_pg_type_ts = gen_as_pg_type_ts(&quote! {Self::#PgTypeUcc});
        let ident_stdrt_nn_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_stdrt_nn_ucc);
        let ident_stdrt_nl_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_stdrt_nl_ucc);
        let ident_arr_nn_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_arr_nn_ucc);
        let ident_arr_nl_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_arr_nl_ucc);
        let gen_ident_stdrt_nn_orgn_ts = |pg_type_1faa6188: &PgType| SelfOrgnUcc::from_tokens(
            &gen_ident_stdrt_nn_ts(pg_type_1faa6188)
        );
        let ident_stdrt_nn_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(pg_type);
        let ident_orgn_ucc = SelfOrgnUcc::from_tokens(&ident);
        let ident_stdrt_nl_tt_ucc = SelfTtUcc::from_tokens(&ident_stdrt_nl_ucc);
        let sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let gen_ident_stdrt_nn_orgn_try_new_er_ts = |pg_type_454846c8: &PgType| SelfOrgnTryNewErUcc::from_tokens(
            &gen_ident_stdrt_nn_ts(pg_type_454846c8)
        );
        let sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_nn_time_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let inn_type_stdrt_nn_ts = {
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
        }.parse::<Ts2>().expect("2555843f");
        let gen_ident_orgn_non_wrapping_8ad5380a = |pg_type_pattern_94bed782: &PgTypePattern, is_nl_28c189b3: &IsNl| SelfOrgnUcc::from_tokens(&gen_ident_ts(pg_type, is_nl_28c189b3, pg_type_pattern_94bed782));
        let dim1_ident_orgn_ts = |dim1_is_nl: &IsNl| {
            let (pat, inl): (&PgTypePattern, &IsNl) = match &is_nl {
                IsNl::False => (&PgTypePattern::Stdrt, dim1_is_nl),
                IsNl::True => (pg_type_pattern, &IsNl::False),
            };
            gen_ident_orgn_non_wrapping_8ad5380a(pat, inl)
        };
        let ft_h: &dyn ToTokens = {
            match &pg_type_pattern {
                PgTypePattern::Stdrt => match &is_nl {
                    IsNl::False => &inn_type_stdrt_nn_ts,
                    IsNl::True => &gen_opt_type_dcl_ts(&ident_stdrt_nn_orgn_ucc),
                },
                PgTypePattern::ArrDim1 { dim1_is_nl } => &{
                    let v = dim1_ident_orgn_ts(dim1_is_nl);
                    match &is_nl {
                        IsNl::False => gen_vec_tokens_dcl_ts(&v),
                        IsNl::True => gen_opt_type_dcl_ts(&v),
                    }
                },
            }
        };
        let gen_typical_qb_ts = |ts: &dyn ToTokens| match &is_nl {
            IsNl::False => quote! {
                if let Err(er) = #QuerySc.try_bind(#ts) {
                    return Err(er.to_string());
                }
                Ok(#QuerySc)
            },
            IsNl::True => quote! {
                if let Err(er) = #QuerySc.try_bind(#ts.0.0) {
                    return Err(er.to_string());
                }
                Ok(#QuerySc)
            },
        };
        let typical_qb_ts = gen_typical_qb_ts(&VSc);
        let ident_inn_type_ts = match &el.pg_type_pattern {
            PgTypePattern::Stdrt => match &is_nl {
                IsNl::False => &inn_type_stdrt_nn_ts,
                IsNl::True => &gen_opt_type_dcl_ts(&inn_type_stdrt_nn_ts),
            },
            PgTypePattern::ArrDim1 { dim1_is_nl } => &{
                let dim1_type = dim1_is_nl.mb_opt_wrap(quote! {#inn_type_stdrt_nn_ts});
                is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim1_type))
            },
        };
        let can_be_pk = match &pg_type {
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
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePk::False,
            PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBePk::True,
        };
        let is_stdrt_nn = if matches!((&pg_type_pattern, &is_nl), (PgTypePattern::Stdrt, IsNl::False)) {
            IsStdrtNn::True
        } else {
            IsStdrtNn::False
        };
        let is_nn_stdrt_can_be_pk = if matches!((&is_nl, &pg_type_pattern, &can_be_pk), (IsNl::False, PgTypePattern::Stdrt, CanBePk::True)) {
            IsNnStdrtCanBePk::True
        } else {
            IsNnStdrtCanBePk::False
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
        let (ser_derive_or_impl, de_derive_or_impl) = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum PrmNbr {
                Two,
                Three,
                Four,
            }
            impl PrmNbr {
                const fn get_i(&self) -> usize {
                    match &self {
                        Self::Two => 1,
                        Self::Three => 2,
                        Self::Four => 3,
                    }
                }
                fn get_vec_from_i_starting_with_zero(&self) -> Vec<usize> {
                    (0..=self.get_i()).collect()
                }
            }
            let self_dot_zero_ts = quote! {#SelfSc.0};
            let prm_nbr_two = PrmNbr::Two;
            let prm_nbr_three = PrmNbr::Three;
            let prm_nbr_four = PrmNbr::Four;
            let ident_stdrt_nn_orgn_dq_ts = dq_ts(&ident_stdrt_nn_orgn_ucc);
            (
                {
                    let gen_impl_ser_for_ident_stdrt_nn_orgn_tokens = |ts: &dyn ToTokens| {
                        quote! {
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #AllowClippyArbitrarySrcItemOrdering
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl _serde::Serialize for #ident_stdrt_nn_orgn_ucc {
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
                    let gen_ser_cnt_b5af560e = |ts: &dyn ToTokens| {
                        quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #ident_stdrt_nn_orgn_dq_ts, &#self_dot_zero_ts #ts)}
                    };
                    let gen_serde_state_init_ts = |prm_nbr: &PrmNbr| {
                        let prm_nbr_ts = {
                            let ts = prm_nbr.get_vec_from_i_starting_with_zero().into_iter().map(|_| quote! {+ 1});
                            quote! {#(#ts)*}
                        };
                        quote! {
                            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_stdrt_nn_orgn_dq_ts, usize::from(false) #prm_nbr_ts)?;
                        }
                    };
                    let serde_state_init_two_fields_ts = gen_serde_state_init_ts(&prm_nbr_two);
                    let serde_state_init_three_fields_ts = gen_serde_state_init_ts(&prm_nbr_three);
                    let serde_state_init_four_fields_ts = gen_serde_state_init_ts(&prm_nbr_four);
                    let gen_ser_field_ts = |field_name: &dyn Display, third_prm_ts: &dyn ToTokens| {
                        let field_name_dq_ts = dq_ts(&field_name);
                        quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_dq_ts, #third_prm_ts)?;}
                    };
                    let serde_ser_ser_struct_end_ts = quote! {_serde::ser::SerializeStruct::end(__serde_state)};
                    let ser_cnt_e5bb5640_ts = {
                        let gen_self_zero_tokens_ts = |ts: &dyn ToTokens| {
                            quote! {&#self_dot_zero_ts.#ts}
                        };
                        let start_ser_field_ts = gen_ser_field_ts(&StartSc, &gen_self_zero_tokens_ts(&StartSc));
                        let end_ser_field_ts = gen_ser_field_ts(&EndSc, &gen_self_zero_tokens_ts(&EndSc));
                        quote! {
                            #serde_state_init_two_fields_ts
                            #start_ser_field_ts
                            #end_ser_field_ts
                            #serde_ser_ser_struct_end_ts
                        }
                    };
                    let impl_ser_for_pg_type_nn_tokens_ser_cnt_e5bb5640_ts = gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&ser_cnt_e5bb5640_ts);
                    let impl_ser_for_uuid_uuid_ts = gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&gen_ser_cnt_b5af560e(&quote! {.to_string()}));
                    let gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens = |ts: &dyn ToTokens| {
                        let gen_ser_field_match_std_ops_bound_ts = |start_or_end: &StartOrEnd| {
                            let start_or_end_ts = gen_start_or_end_sc(start_or_end);
                            gen_ser_field_ts(
                                &start_or_end_ts,
                                &quote! {
                                    &match self.0.#start_or_end_ts {
                                        std::ops::Bound::Included(v_7d755c7c) => std::ops::Bound::Included(#ts::#TryNewSc(v_7d755c7c).map_err(_serde::ser::Error::custom)?),
                                        std::ops::Bound::Excluded(v_cfbe64e9) => std::ops::Bound::Excluded(#ts::#TryNewSc(v_cfbe64e9).map_err(_serde::ser::Error::custom)?),
                                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                    }
                                },
                            )
                        };
                        let start_ser_field_ts = gen_ser_field_match_std_ops_bound_ts(&StartOrEnd::Start);
                        let end_ser_field_ts = gen_ser_field_match_std_ops_bound_ts(&StartOrEnd::End);
                        gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&quote! {
                            #serde_state_init_two_fields_ts
                            #start_ser_field_ts
                            #end_ser_field_ts
                            #serde_ser_ser_struct_end_ts
                        })
                    };
                    let gen_b5af560e_ts = |ts: &dyn ToTokens|{
                        DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(
                            &gen_ser_cnt_b5af560e(&ts)
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
                        PgType::SqlxTypesChronoNaiveTimeAsTime => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            let gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts = |ts: &dyn ToTokens| {
                                quote! {&(<#inn_type_stdrt_nn_ts as chrono::Timelike>::#ts)}
                            };
                            let hour_ser_field_ts = gen_ser_field_ts(&HourSc, &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote! {hour(&self.0)}));
                            let min_ser_field_ts = gen_ser_field_ts(&MinSc, &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote! {minute(&self.0)}));
                            let sec_ser_field_ts = gen_ser_field_ts(&SecSc, &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote! {second(&self.0)}));
                            let micro_ser_field_ts = gen_ser_field_ts(
                                &MicroSc,
                                &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote! {
                                    #NanosecondSc(&self.0).checked_div(1000).expect("aea037b7")
                                }),
                            );
                            quote! {
                                #serde_state_init_four_fields_ts
                                #hour_ser_field_ts
                                #min_ser_field_ts
                                #sec_ser_field_ts
                                #micro_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        })),
                        PgType::SqlxTypesTimeTimeAsTime => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            let gen_ser_field_self_zero_ts = |v: &dyn DisplayPlusToTokens| gen_ser_field_ts(&v, &quote! {&self.0.#v()});
                            let hour_ser_field_ts = gen_ser_field_self_zero_ts(&HourSc);
                            let minute_ser_field_ts = gen_ser_field_self_zero_ts(&MinuteSc);
                            let second_ser_field_ts = gen_ser_field_self_zero_ts(&SecondSc);
                            let microsecond_ser_field_ts = gen_ser_field_self_zero_ts(&MicrosecondSc);
                            quote! {
                                #serde_state_init_four_fields_ts
                                #hour_ser_field_ts
                                #minute_ser_field_ts
                                #second_ser_field_ts
                                #microsecond_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        })),
                        PgType::SqlxPgTypesPgIntervalAsInterval => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            let gen_ser_field_h_ts = |v: &dyn DisplayPlusToTokens| gen_ser_field_ts(&v, &quote! {&#self_dot_zero_ts.#v});
                            let months_ser_field_ts = gen_ser_field_h_ts(&MonthsSc);
                            let days_ser_field_ts = gen_ser_field_h_ts(&DaysSc);
                            let microseconds_ser_field_ts = gen_ser_field_h_ts(&MicrosecondsSc);
                            quote! {
                                #serde_state_init_three_fields_ts
                                #months_ser_field_ts
                                #days_ser_field_ts
                                #microseconds_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        })),
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            enum DateOrTime {
                                Date,
                                Time,
                            }
                            let gen_ser_field_try_new_unwrap_ts = |date_or_time: &DateOrTime| {
                                let date_or_time_ts: &dyn DisplayPlusToTokens = match &date_or_time {
                                    DateOrTime::Date => &DateSc,
                                    DateOrTime::Time => &TimeSc,
                                };
                                gen_ser_field_ts(&date_or_time_ts, &{
                                    let ident_ts_203ac73c: &dyn ToTokens = match &date_or_time {
                                        DateOrTime::Date => &sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc,
                                        DateOrTime::Time => &sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc,
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
                            let date_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateOrTime::Date);
                            let time_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateOrTime::Time);
                            quote! {
                                #serde_state_init_two_fields_ts
                                #date_ser_field_ts
                                #time_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        })),
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            enum DateNaiveOrTime {
                                Date,
                                Time,
                            }
                            let gen_ser_field_try_new_unwrap_ts = |date_naive_or_time: &DateNaiveOrTime| {
                                let date_naive_or_time_ts: &dyn DisplayPlusToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &DateNaiveSc,
                                    DateNaiveOrTime::Time => &TimeSc,
                                };
                                gen_ser_field_ts(&date_naive_or_time_ts, &{
                                    let ident_ts_d3be24b2: &dyn ToTokens = match &date_naive_or_time {
                                        DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc,
                                        DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc,
                                    };
                                    quote! {&#ident_ts_d3be24b2::#TryNewSc(self.0.#date_naive_or_time_ts()).map_err(_serde::ser::Error::custom)?}
                                })
                            };
                            let date_naive_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateNaiveOrTime::Date);
                            let time_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateNaiveOrTime::Time);
                            quote! {
                                #serde_state_init_two_fields_ts
                                #date_naive_ser_field_ts
                                #time_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        })),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => DeriveOrImpl::Impl(impl_ser_for_uuid_uuid_ts),
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => DeriveOrImpl::Impl(impl_ser_for_pg_type_nn_tokens_ser_cnt_e5bb5640_ts),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc)),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens(&sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc)),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DeriveOrImpl::Impl(gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc)),
                    }
                },
                DeriveOrImpl::Derive
            )
        } else {
            (DeriveOrImpl::Derive, DeriveOrImpl::Derive)
        };
        let v_ident_inn_type_ts = quote! {#VSc: #ident_inn_type_ts};
        let ident_stdrt_nn_rd_ucc = SelfRdUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_stdrt_nn_orgn_try_new_er_ucc = SelfOrgnTryNewErUcc::from_display(&ident_stdrt_nn_ucc);
        let ident_stdrt_nn_orgn_try_new_for_de_er_ucc = SelfOrgnTryNewForDeErUcc::from_display(&ident_stdrt_nn_ucc);
        let int_range_type_to_range_inn_type_ts = |int_range_type: &IntRangeType| -> Ts2 {
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
        let gen_pub_const_new_or_pub_try_new_ts = |ts: &dyn ToTokens| {
            let pub_fn_new_or_try_new_ts = if pg_type_init_try_new_try_from_pg_type.is_ok() {
                &gen_pub_try_new_ts(
                    &Ts2::new(),
                    &v_ident_inn_type_ts,
                    &ident_stdrt_nn_orgn_try_new_er_ucc,
                    &quote! {
                        match #ident_orgn_ucc::#TryNewSc(#VSc) {
                            Ok(v_0f9f1a61) => Ok(Self(v_0f9f1a61)),
                            Err(er) => Err(er)
                        }
                    },
                )
            } else {
                &{
                    let self_ident_orgn_new_v_ts = quote! {Self(#ident_orgn_ucc::#NewSc(#VSc))};
                    if matches!(&pg_type_pattern, PgTypePattern::Stdrt)
                        && matches!(&is_nl, IsNl::False)
                    {
                        gen_pub_const_new_ts(
                            &MustUse,
                            &v_ident_inn_type_ts,
                            &self_ident_orgn_new_v_ts
                        )
                    } else {
                        gen_pub_new_ts(
                            &MustUse,
                            &v_ident_inn_type_ts,
                            &self_ident_orgn_new_v_ts
                        )
                    }
                }
            };
            quote! {
                impl #ts {
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
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DCopy::True,
                PgType::StringAsText |
                PgType::StdVecVecU8AsBytea => DCopy::False,
            },
            PgTypePattern::ArrDim1 { .. } => DCopy::False,
        };
        let sqlx_types_chrono_naive_time_min_fn_ts = quote!{sqlx_types_chrono_naive_time_min};
        let sqlx_types_chrono_naive_time_ten_fn_ts = quote!{sqlx_types_chrono_naive_time_ten};
        let sqlx_types_chrono_naive_time_twenty_fn_ts = quote!{sqlx_types_chrono_naive_time_twenty};
        let sqlx_types_chrono_naive_time_max_fn_ts = quote!{sqlx_types_chrono_naive_time_max};
        let sqlx_types_chrono_naive_date_min_fn_ts = quote!{sqlx_types_chrono_naive_date_min};
        let sqlx_types_chrono_naive_date_negative_less_typical_fn_ts = quote!{sqlx_types_chrono_naive_date_negative_less_typical};
        let sqlx_types_chrono_naive_date_negative_more_typical_fn_ts = quote!{sqlx_types_chrono_naive_date_negative_more_typical};
        let sqlx_types_chrono_naive_date_near_zero_fn_ts = quote!{sqlx_types_chrono_naive_date_near_zero};
        let sqlx_types_chrono_naive_date_positive_less_typical_fn_ts = quote!{sqlx_types_chrono_naive_date_positive_less_typical};
        let sqlx_types_chrono_naive_date_positive_more_typical_fn_ts = quote!{sqlx_types_chrono_naive_date_positive_more_typical};
        let sqlx_types_chrono_naive_date_max_fn_ts = quote!{sqlx_types_chrono_naive_date_max};
        let sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_ts = quote!{sqlx_types_chrono_naive_date_max_pred_opt_expect};
        let ident_ts = {
            let ident_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .build_struct(
                    &Ts2::new(),
                    &ident,
                    &Ts2::new(),
                    &quote!{;},
                );
            // println!("@@@{}", ident_inn_type_ts);
            let mb_impl_ident_ts = if matches!(&pg_type_pattern, PgTypePattern::Stdrt) &&
                matches!(&is_nl, IsNl::False)
            {
                enum IsConst {
                    False,
                    True,
                }
                let gen_inn_type_ts = |
                    is_const: IsConst,
                    name_ts: &dyn ToTokens,
                    ts: &dyn ToTokens
                |{
                    let mb_const_ts = match is_const {
                        IsConst::False => Ts2::new(),
                        IsConst::True => quote!{const},
                    };
                    quote!{
                        #mb_const_ts fn #name_ts() -> #ident_inn_type_ts {
                            #ts
                        }
                    }
                };
                let mb_min_inn_type_ts = {
                    let gen_inn_type_ts_67fc7980 = |is_const: IsConst, ts: &dyn ToTokens| gen_inn_type_ts(is_const, &quote!{min_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inn_type_ts_67fc7980(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("000ddcc2")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inn_type_ts_67fc7980(
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
                let mb_slightly_more_than_min_inn_type_ts = {
                    let gen_inn_type_ts_6d89728a = |is_const: IsConst, ts: &dyn ToTokens| gen_inn_type_ts(is_const, &quote!{slightly_more_than_min_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inn_type_ts_6d89728a(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("9545a47c")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inn_type_ts_6d89728a(
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
                let mb_middle_inn_type_ts = {
                    let gen_inn_type_ts_23368199 = |is_const: IsConst, ts: &dyn ToTokens| gen_inn_type_ts(is_const, &quote!{middle_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inn_type_ts_23368199(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("0dafc3fc")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inn_type_ts_23368199(
                                IsConst::False,
                                &quote!{
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("d2ec329f")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_inn_type_ts_23368199(
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
                let mb_slightly_more_than_middle_inn_type_ts = {
                    let gen_inn_type_ts_3a61c0b0 = |is_const: IsConst, ts: &dyn ToTokens| gen_inn_type_ts(is_const, &quote!{slightly_more_than_middle_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inn_type_ts_3a61c0b0(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("235276a7")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_inn_type_ts_3a61c0b0(
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
                let mb_max_inn_type_ts = {
                    let gen_inn_type_ts_32acb388 = |is_const: IsConst, ts: &dyn ToTokens| gen_inn_type_ts(is_const, &quote!{max_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inn_type_ts_32acb388(
                                IsConst::True,
                                &quote!{
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("b217e3bf")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_inn_type_ts_32acb388(
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
                let mb_slightly_less_than_max_inn_type_ts = {
                    let gen_inn_type_ts_ddf0f630 = |is_const: IsConst, ts: &dyn ToTokens| gen_inn_type_ts(is_const, &quote!{slightly_less_than_max_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_inn_type_ts_ddf0f630(
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
                let mb_rd_inn_inits_ts = {
                    let gen_fn_ts = |
                        name_ts: &Ts2,
                        ts_5dfcb210: &Ts2
                    |quote!{
                        const fn #name_ts() -> #ident_inn_type_ts {
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
                                (&sqlx_types_chrono_naive_time_min_fn_ts, &quote!{0,0,0,0}),
                                (&sqlx_types_chrono_naive_time_ten_fn_ts, &quote!{10,10,10,10}),
                                (&sqlx_types_chrono_naive_time_twenty_fn_ts, &quote!{20,20,20,20}),
                                (&sqlx_types_chrono_naive_time_max_fn_ts, &quote!{23,59,59,999_999}),
                            ].iter().map(|(name_ts, prms_ts)| quote! {
                                const fn #name_ts() -> #ident_inn_type_ts {
                                    #ident_inn_type_ts::from_hms_micro_opt(#prms_ts).expect("149e01cc")
                                }
                            }).collect::<Vec<Ts2>>();
                            quote!{#(#ts_80e0683c)*}
                        }),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some({
                            let ts_80e0683c = {
                                let gen_fn_ident_inn_type_ts = |
                                    name_ts: &Ts2,
                                    ts_a29ab1c6: &Ts2
                                |gen_fn_ts(
                                    name_ts,
                                    &quote!{#ident_inn_type_ts::#ts_a29ab1c6}
                                );
                                [
                                    gen_fn_ident_inn_type_ts(
                                        &sqlx_types_chrono_naive_date_max_fn_ts,
                                        &quote! { MAX }
                                    ),
                                    gen_fn_ts(
                                        &sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_ts,
                                        &quote!{Self::#sqlx_types_chrono_naive_date_max_fn_ts().pred_opt().expect("b7e16bf1")}
                                    )
                                ]
                                .into_iter()
                                .chain(
                                    [
                                        (&sqlx_types_chrono_naive_date_min_fn_ts, &quote! { -4713, 12, 31 }),
                                        (&sqlx_types_chrono_naive_date_negative_less_typical_fn_ts, &quote! { -2000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_negative_more_typical_fn_ts, &quote! { -1000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_near_zero_fn_ts, &quote! { 0, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_positive_less_typical_fn_ts, &quote! { 1000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_positive_more_typical_fn_ts, &quote! { 2000, 1, 1 }),
                                    ]
                                    .into_iter()
                                    .map(|(name_ts, prms_ts)| {
                                        gen_fn_ident_inn_type_ts(
                                            name_ts,
                                            &quote! {
                                                from_ymd_opt(#prms_ts)
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
                if mb_min_inn_type_ts.is_some() ||
                    mb_slightly_more_than_min_inn_type_ts.is_some() ||
                    mb_middle_inn_type_ts.is_some() ||
                    mb_slightly_more_than_middle_inn_type_ts.is_some() ||
                    mb_max_inn_type_ts.is_some() ||
                    mb_slightly_less_than_max_inn_type_ts.is_some() ||
                    mb_rd_inn_inits_ts.is_some()
                {
                    quote!{
                        #AllowClippyArbitrarySrcItemOrdering
                        impl #ident {
                            #mb_min_inn_type_ts
                            #mb_slightly_more_than_min_inn_type_ts
                            #mb_middle_inn_type_ts
                            #mb_slightly_more_than_middle_inn_type_ts
                            #mb_max_inn_type_ts
                            #mb_slightly_less_than_max_inn_type_ts
                            #mb_rd_inn_inits_ts
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
                #mb_impl_ident_ts
            }
        };
        let sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts = SelfOrgnUcc::from_tokens(&gen_ident_stdrt_nn_ts(&PgType::SqlxTypesChronoNaiveDateAsDate));
        let ident_upd_ucc = SelfUpdUcc::from_tokens(&ident);
        let ident_orgn_ts = {
            let ident_orgn_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_eq_if(match &is_nn_stdrt_can_be_pk {
                    IsNnStdrtCanBePk::False => DEq::False,
                    IsNnStdrtCanBePk::True => DEq::True,
                })
                .d_partial_ord_if(match &is_stdrt_nn {
                    IsStdrtNn::False => DPartialOrd::False,
                    IsStdrtNn::True => match &pg_type {
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
                        | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => DPartialOrd::True,
                        PgType::SqlxPgTypesPgMoneyAsMoney
                        | PgType::SqlxPgTypesPgIntervalAsInterval
                        | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                        | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DPartialOrd::False,
                    },
                })
                .d_ord_if(match &is_nn_stdrt_can_be_pk {
                    IsNnStdrtCanBePk::False => DOrd::False,
                    IsNnStdrtCanBePk::True => DOrd::True,
                })
                .d_serde_serialize_if(match &ser_derive_or_impl {
                    DeriveOrImpl::Derive => DSerdeSerialize::True,
                    DeriveOrImpl::Impl(_) => DSerdeSerialize::False,
                })
                .d_serde_deserialize_if(match &de_derive_or_impl {
                    DeriveOrImpl::Derive => DSerdeDeserialize::True,
                    DeriveOrImpl::Impl(_) => DSerdeDeserialize::False,
                })
                .build_struct(
                    &if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                        let gen_serde_from_ts = |ts: &dyn ToTokens|quote!{#[serde(from = #ts)]};
                        let gen_serde_try_from_ts = |ts: &dyn ToTokens|quote!{#[serde(try_from = #ts)]};
                        match &pg_type {
                            PgType::I16AsInt2 |
                            PgType::I32AsInt4 |
                            PgType::I64AsInt8 |
                            PgType::F32AsFloat4 |
                            PgType::F64AsFloat8 |
                            PgType::I16AsSmallSerialInitByPg |
                            PgType::I32AsSerialInitByPg |
                            PgType::I64AsBigSerialInitByPg |
                            PgType::BoolAsBool |
                            PgType::StdVecVecU8AsBytea |
                            PgType::SqlxTypesIpnetworkIpNetworkAsInet => Ts2::new(),
                            PgType::SqlxPgTypesPgMoneyAsMoney => gen_serde_from_ts(&quote!{"i64"}),
                            PgType::SqlxTypesChronoNaiveTimeAsTime => gen_serde_try_from_ts(&quote!{"(u32,u32,u32,u32)"}),
                            PgType::SqlxTypesTimeTimeAsTime => gen_serde_try_from_ts(&quote!{"(u8,u8,u8,u32)"}),
                            PgType::SqlxPgTypesPgIntervalAsInterval => gen_serde_from_ts(&quote!{"(i32,i32,i64)"}),
                            PgType::SqlxTypesChronoNaiveDateAsDate => gen_serde_try_from_ts(&quote!{"sqlx::types::chrono::NaiveDate"}),
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => gen_serde_from_ts(&dq_ts(&format!("({sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts},SqlxTypesChronoNaiveTimeAsNnTimeOrgn)"))),
                            PgType::StringAsText |
                            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                            PgType::SqlxTypesUuidUuidAsUuidInitByClient => quote!{#[serde(try_from = "String")]},
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote!{#[serde(from = "[u8; 6]")]},
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => quote!{#[serde(try_from = "(std::ops::Bound<i32>,std::ops::Bound<i32>)")]},
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote!{#[serde(try_from = "(std::ops::Bound<i64>,std::ops::Bound<i64>)")]},
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                                let bound = format!("std::ops::Bound<{sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts}>");
                                let ts = dq_ts(&format!("({bound},{bound})"));
                                quote!{#[serde(from = #ts)]}
                            },
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => quote!{#[serde(from = "(std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNnTimestampOrgn>,std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNnTimestampOrgn>)")]},//todo reuse name
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => quote!{#[serde(from = "(std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzOrgn>,std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzOrgn>)")]},//todo reuse name
                        }
                    }
                    else {
                        Ts2::new()
                    },
                    &ident_orgn_ucc,
                    &Ts2::new(),
                    &quote!{(#ft_h);},
                );
            let gen_loc_var_ts = |name_ts: &dyn ToTokens, ts: &dyn ToTokens|quote! {
                #name_ts {
                    loc: loc_lib::loc::Loc,
                    #ts
                }
            };
            let gen_int_range_type_er_vrts_ts = |int_range_type: &IntRangeType| {
                let range_inn_type_ts = int_range_type_to_range_inn_type_ts(int_range_type);
                let (
                    included_start_greater_than_included_end_ts,
                    included_start_greater_than_excluded_end_ts,
                    excluded_start_greater_than_included_end_ts,
                    excluded_start_greater_than_excluded_end_ts
                ) = {
                    let gen_ts = |ts: &dyn ToTokens|gen_loc_var_ts(
                        &ts,
                        &quote!{
                            #[eo_to_err_string_serde]
                            #StartSc: #range_inn_type_ts,
                            #[eo_to_err_string_serde]
                            #EndSc: #range_inn_type_ts,
                        }
                    );
                    (
                        gen_ts(&IncludedStartGreaterThanIncludedEndUcc),
                        gen_ts(&IncludedStartGreaterThanExcludedEndUcc),
                        gen_ts(&ExcludedStartGreaterThanIncludedEndUcc),
                        gen_ts(&ExcludedStartGreaterThanExcludedEndUcc)
                    )
                };
                let included_end_cannot_be_max_ucc_ts = gen_loc_var_ts(
                    &IncludedEndCannotBeMaxUcc,
                    &quote!{
                        #[eo_to_err_string_serde]
                        #EndSc: #range_inn_type_ts,
                    }
                );
                quote! {
                    #included_start_greater_than_included_end_ts,
                    #included_start_greater_than_excluded_end_ts,
                    #excluded_start_greater_than_included_end_ts,
                    #excluded_start_greater_than_excluded_end_ts,
                    #included_end_cannot_be_max_ucc_ts,
                }
            };
            let nanosecond_precision_is_not_supported_vrt_try_new_ts = gen_loc_var_ts(
                &NanosecondPrecisionIsNotSupportedUcc,
                &quote!{
                    #[eo_to_err_string_serde]
                    #VSc: #StringTs,
                }
            );
            let sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts = gen_loc_var_ts(
                &EarlierDateNotSupportedUcc,
                &quote!{
                    #[eo_to_err_string_serde]
                    value: #StringTs,
                    #[eo_to_err_string_serde]
                    #EarliestSupportedDateSc: #StringTs,
                }
            );
            let string_as_text_try_new_er_vrts_ts = gen_loc_var_ts(
                &ContainsNullByteUcc,
                &quote!{
                    #[eo_to_err_string_serde]
                    #VSc: #ident_inn_type_ts,
                }
            );
            let uuid_as_uuid_v4_as_string_try_new_er_vrts_ts = gen_loc_var_ts(
                &NotUuidUcc,
                &quote!{
                    #[eo_to_err_string_serde]
                    #VSc: String,
                }
            );
            let mb_pub_enum_ident_stdrt_nn_orgn_try_new_er_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True)
                && let Ok(pg_type_init_try_new) = &pg_type_init_try_new_try_from_pg_type
            {
                let ts_d57d5de2 = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_thiserror_error()
                    .d_loc_lib_location()
                    .build_enum(
                        &Ts2::new(),
                        &ident_stdrt_nn_orgn_try_new_er_ucc,
                        &Ts2::new(),
                        &{
                            let gen_ts = |ts: &dyn ToTokens| {
                                let (start_vrt_ts, end_vrt_ts) = {
                                    let gen_vrt_ts = |start_or_end: &StartOrEnd| gen_loc_var_ts(
                                        &gen_start_or_end_ucc(start_or_end),
                                        &quote!{
                                            #[eo_loc]
                                            #ErSc: #ts,
                                        }
                                    );
                                    (gen_vrt_ts(&StartOrEnd::Start), gen_vrt_ts(&StartOrEnd::End))
                                };
                                quote! {
                                    #start_vrt_ts,
                                    #end_vrt_ts,
                                }
                            };
                            let time_var_ts = gen_loc_var_ts(
                                &TimeUcc,
                                &quote!{
                                    #[eo_loc]
                                    #ErSc: #sqlx_types_chrono_naive_time_as_nn_time_orgn_try_new_er_ucc,
                                }
                            );
                            let ts: &dyn ToTokens = match &pg_type_init_try_new {
                                PgTypeInitTryNew::StringAsText => &string_as_text_try_new_er_vrts_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime | PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_vrt_try_new_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => &{
                                    let date_var_ts = gen_loc_var_ts(
                                        &DateUcc,
                                        &quote!{
                                            #[eo_loc]
                                            #ErSc: #sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc,
                                        }
                                    );
                                    quote! {
                                        #date_var_ts,
                                        #time_var_ts,
                                    }
                                },
                                PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &{
                                    let date_naive_var_ts = gen_loc_var_ts(
                                        &DateNaiveUcc,
                                        &quote!{
                                            #[eo_loc]
                                            #ErSc: #sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc,
                                        }
                                    );
                                    quote! {
                                        #date_naive_var_ts,
                                        #time_var_ts,
                                    }
                                },
                                PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_ts(
                                    &sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc
                                ),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_ts(
                                    &sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_try_new_er_ucc
                                ),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_ts(
                                    &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_try_new_er_ucc
                                ),
                            };
                            quote!{{#ts}}
                        }
                    );
                quote!{
                    #AllowClippyArbitrarySrcItemOrdering
                    #ts_d57d5de2
                }
            } else {
                Ts2::new()
            };
            let mb_pub_enum_ident_stdrt_nn_orgn_try_new_for_de_er_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                //todo this is bad design. refactor later
                let gen_er_ts = |pg_type_impl_try_new_for_deserialize: &PgTypeImplTryNewForDe|{
                    let ts_026f2a24 = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_thiserror_error()
                    .d_loc_lib_location()
                    .build_enum(
                        &Ts2::new(),
                        &ident_stdrt_nn_orgn_try_new_for_de_er_ucc,
                        &Ts2::new(),
                        &{
                            let ts: &dyn ToTokens = match &pg_type_impl_try_new_for_deserialize {
                                PgTypeImplTryNewForDe::StringAsText => &string_as_text_try_new_er_vrts_ts,
                                PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime => &{
                                    let invalid_hour_or_minute_or_second_or_microsecond_var_ts = gen_loc_var_ts(
                                        &InvalidHourOrMinuteOrSecondOrMicrosecondUcc,
                                        &quote!{
                                            #[eo_to_err_string_serde]
                                            #HourSc: #U32,
                                            #[eo_to_err_string_serde]
                                            #MinSc: #U32,
                                            #[eo_to_err_string_serde]
                                            #SecSc: #U32,
                                            #[eo_to_err_string_serde]
                                            #MicroSc: #U32,
                                        }
                                    );
                                    quote! {
                                        #invalid_hour_or_minute_or_second_or_microsecond_var_ts,
                                        #nanosecond_precision_is_not_supported_vrt_try_new_ts
                                    }
                                },
                                PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime => &{
                                    let invalid_hour_or_minute_or_second_or_microsecond_var_ts = gen_loc_var_ts(
                                        &InvalidHourOrMinuteOrSecondOrMicrosecondUcc,
                                        &quote!{
                                            #[eo_to_err_string_serde]
                                            #ErSc: #StringTs,
                                            #[eo_to_err_string_serde]
                                            #MicrosecondSc: #U32,
                                            #[eo_to_err_string_serde]
                                            #HourSc: #U8,
                                            #[eo_to_err_string_serde]
                                            #MinuteSc: #U8,
                                            #[eo_to_err_string_serde]
                                            #SecondSc: #U8,
                                        }
                                    );
                                    quote! {
                                        #invalid_hour_or_minute_or_second_or_microsecond_var_ts,
                                        #nanosecond_precision_is_not_supported_vrt_try_new_ts
                                    }
                                },
                                PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts,
                                PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidV4InitByPg |
                                PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidInitByClient => &uuid_as_uuid_v4_as_string_try_new_er_vrts_ts,
                            };
                            quote!{{#ts}}
                        }
                    );
                    quote!{
                        #AllowClippyArbitrarySrcItemOrdering
                        #ts_026f2a24
                    }
                };
                match &de_derive_or_impl {
                    DeriveOrImpl::Derive => if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                        match &pg_type {
                            PgType::I16AsInt2 |
                            PgType::I32AsInt4 |
                            PgType::I64AsInt8 |
                            PgType::F32AsFloat4 |
                            PgType::F64AsFloat8 |
                            PgType::I16AsSmallSerialInitByPg |
                            PgType::I32AsSerialInitByPg |
                            PgType::I64AsBigSerialInitByPg |
                            PgType::BoolAsBool |
                            PgType::StdVecVecU8AsBytea |
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                            PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange |
                            PgType::SqlxPgTypesPgMoneyAsMoney |
                            PgType::SqlxPgTypesPgIntervalAsInterval => Ts2::new(),
                            PgType::StringAsText => gen_er_ts(&PgTypeImplTryNewForDe::StringAsText),
                            PgType::SqlxTypesChronoNaiveTimeAsTime => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime),
                            PgType::SqlxTypesTimeTimeAsTime => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime),
                            PgType::SqlxTypesChronoNaiveDateAsDate => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate),
                            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidV4InitByPg),
                            PgType::SqlxTypesUuidUuidAsUuidInitByClient => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidInitByClient),
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_er_ts(&PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range),
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_er_ts(&PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range),
                        }
                    }
                    else {
                        Ts2::new()
                    },
                    DeriveOrImpl::Impl(_) => match &pg_type_deserialize {
                        PgTypeDeserialize::Derive => Ts2::new(),
                        PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDe(pg_type_impl_new_for_de_or_try_new_for_deserialize) => match &pg_type_impl_new_for_de_or_try_new_for_deserialize {
                            PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize => Ts2::new(),
                            PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(pg_type_impl_try_new_for_deserialize) => gen_er_ts(pg_type_impl_try_new_for_deserialize)
                        },
                    }
                }
            } else {
                Ts2::new()
            };
            let impl_ident_orgn_ts = {
                let fn_new_or_try_new_ts = pg_type_init_try_new_try_from_pg_type.as_ref().map_or_else(
                |()| {
                    let ts = {
                        let ts = {
                            let gen_match_opt_ts = |ts: &dyn ToTokens| {
                                quote! {#VSc.map(#ts::#NewSc)}
                            };
                            let gen_arr_dims_init_ts = |ts: &dyn ToTokens| match &is_nl {
                                IsNl::False => quote! {#VSc.into_iter().map(#ts::#NewSc).collect()},
                                IsNl::True => gen_match_opt_ts(&ts),
                            };
                            match &pg_type_pattern {
                                PgTypePattern::Stdrt => match &is_nl {
                                    IsNl::False => {
                                        range_try_from_pg_type.as_ref().map_or_else(
                                            |()| quote! {#VSc},
                                            |v_6ed98462| gen_pg_range_conversion_ts(
                                                &VSc,
                                                &{
                                                    let range_pg_type_ident_orgn = SelfOrgnUcc::from_display(&gen_ident_str(&PgType::from(v_6ed98462), is_nl, pg_type_pattern));
                                                    quote! {#range_pg_type_ident_orgn::#NewSc(v_af65ccce)}
                                                }
                                            )
                                        )
                                    }
                                    IsNl::True => gen_match_opt_ts(&ident_stdrt_nn_orgn_ucc),
                                },
                                PgTypePattern::ArrDim1 { dim1_is_nl } => gen_arr_dims_init_ts(&dim1_ident_orgn_ts(dim1_is_nl)),
                            }
                        };
                        quote! {Self(#ts)}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            IsNl::False => gen_const_new_ts(
                                &MustUse,
                                &v_ident_inn_type_ts,
                                &ts
                            ),
                            IsNl::True => gen_new_ts(
                                &MustUse,
                                &v_ident_inn_type_ts,
                                &ts
                            ),
                        },
                        PgTypePattern::ArrDim1 { .. } => gen_new_ts(
                            &MustUse,
                            &v_ident_inn_type_ts,
                            &ts
                        ),
                    }
                },
                |pg_type_init_try_new| {
                    let ts = {
                        let gen_match_opt_ts = |ts: &dyn ToTokens| {
                            quote! {Ok(Self(match #VSc {
                                Some(v_989d943e) => Some(match #ts::#TryNewSc(v_989d943e) {
                                    Ok(v_ea2a4a8c) => v_ea2a4a8c,
                                    Err(er) => {
                                        return Err(er);
                                    },
                                }),
                                None => None
                            }))}
                        };
                        let gen_arr_dims_init_ts = |ts: &dyn ToTokens| match &is_nl {
                            IsNl::False => quote! {
                                Ok(Self({
                                    let mut acc_4ce2782a = Vec::new();
                                    for el_de177578 in #VSc {
                                        match #ts::#TryNewSc(el_de177578) {
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
                            IsNl::True => gen_match_opt_ts(&ts),
                        };
                        match &pg_type_pattern {
                            PgTypePattern::Stdrt => match &is_nl {
                                IsNl::False => {
                                    let gen_int_range_check_ts = |int_range_type: &IntRangeType| {
                                        let max_v_ts = {
                                            let type_ts = int_range_type_to_range_inn_type_ts(int_range_type);
                                            quote! {#type_ts::MAX}
                                        };
                                        quote! {
                                            let max = #max_v_ts;
                                            let (#StartSc, #EndSc) = match (#VSc.#StartSc, #VSc.#EndSc) {
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Included(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#IncludedStartGreaterThanIncludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                    if #EndSc == max {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#StartSc), std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Excluded(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#IncludedStartGreaterThanExcludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#StartSc), std::ops::Bound::Excluded(#EndSc))
                                                }
                                                (std::ops::Bound::Included(#StartSc), std::ops::Bound::Unbounded) => (std::ops::Bound::Included(#StartSc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Included(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#ExcludedStartGreaterThanIncludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                    if #EndSc == max {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Included(#EndSc))
                                                }
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Excluded(#EndSc)) => {
                                                    if #StartSc > #EndSc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#ExcludedStartGreaterThanExcludedEndUcc {
                                                            #StartSc,
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Excluded(#EndSc))
                                                }
                                                (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Unbounded) => (std::ops::Bound::Excluded(#StartSc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Included(#EndSc)) => {
                                                    if #EndSc == max {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#IncludedEndCannotBeMaxUcc {
                                                            #EndSc,
                                                            loc: loc_lib::loc!(),
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
                                    let gen_ok_self_sqlx_pg_types_pg_range_ts = |ts: &dyn ToTokens| {
                                        let gen_bound_arms_ts = |variant_ts: &dyn ToTokens| quote! {
                                            std::ops::Bound::Included(v_bound_incl) => match #ts::#TryNewSc(v_bound_incl) {
                                                Ok(v_bound_ok) => std::ops::Bound::Included(v_bound_ok.0),
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#variant_ts {
                                                        #ErSc,
                                                        loc: loc_lib::loc!(),
                                                    });
                                                }
                                            },
                                            std::ops::Bound::Excluded(v_bound_excl) => match #ts::#TryNewSc(v_bound_excl) {
                                                Ok(v_bound_ok) => std::ops::Bound::Excluded(v_bound_ok.0),
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#variant_ts {
                                                        #ErSc,
                                                        loc: loc_lib::loc!(),
                                                    });
                                                }
                                            },
                                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                        };
                                        let start_arms_ts = gen_bound_arms_ts(&quote! {#StartUcc});
                                        let end_arms_ts = gen_bound_arms_ts(&quote! {#EndUcc});
                                        quote! {
                                            Ok(Self(sqlx::postgres::types::PgRange {
                                                #StartSc: match #VSc.#StartSc { #start_arms_ts },
                                                #EndSc: match #VSc.#EndSc { #end_arms_ts },
                                            }))
                                        }
                                    };
                                    match &pg_type_init_try_new {
                                        PgTypeInitTryNew::StringAsText => quote! {
                                            if #VSc.find('\0').is_some() {
                                                Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#ContainsNullByteUcc {
                                                    #VSc,
                                                    loc: loc_lib::loc!(),
                                                })
                                            } else {
                                                Ok(Self(#VSc))
                                            }
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime => quote! {
                                            if <#inn_type_stdrt_nn_ts as chrono::Timelike>::nanosecond(&#VSc).checked_rem(1000).expect("7c8b4e12") != 0 {
                                                return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                    #VSc: #VSc.to_string(),
                                                    loc: loc_lib::loc!(),
                                                });
                                            }
                                            Ok(Self(#VSc))
                                        },
                                        PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => quote! {
                                            if #VSc.nanosecond().checked_rem(1000).expect("ce47524f") != 0 {
                                                return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                                    #VSc: #VSc.to_string(),
                                                    loc: loc_lib::loc!(),
                                                });
                                            }
                                            Ok(Self(#VSc))
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => quote! {
                                            let #EarliestSupportedDateSc = #inn_type_stdrt_nn_ts::from_ymd_opt(-4713, 12, 31).expect("9f6241e5");
                                            if #VSc >= #EarliestSupportedDateSc {
                                                Ok(Self(#VSc))
                                            }
                                            else {
                                                Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#EarlierDateNotSupportedUcc {
                                                    value: #VSc.to_string(),
                                                    #EarliestSupportedDateSc: #EarliestSupportedDateSc.to_string(),
                                                    loc: loc_lib::loc!(),
                                                })
                                            }
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote! {
                                            let #DateSc = match #sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc::#TryNewSc(
                                                #VSc.#DateSc()
                                            ) {
                                                Ok(v_9be8eddb) => v_9be8eddb,
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#DateUcc {
                                                        #ErSc,
                                                        loc: loc_lib::loc!(),
                                                    });
                                                }
                                            };
                                            let #TimeSc = match #sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc::#TryNewSc(
                                                #VSc.#TimeSc()
                                            ) {
                                                Ok(v_993484ce) => v_993484ce,
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#TimeUcc {
                                                        #ErSc,
                                                        loc: loc_lib::loc!(),
                                                    });
                                                }
                                            };
                                            Ok(Self(#inn_type_stdrt_nn_ts::#NewSc(#DateSc.0, #TimeSc.0)))
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                                #DateNaiveSc.0,
                                                #TimeSc.0
                                            }));
                                            quote! {
                                                let #DateNaiveSc = match #sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc::#TryNewSc(#VSc.date_naive()) {
                                                    Ok(v_158945ad) => v_158945ad,
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#DateNaiveUcc {
                                                            #ErSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                };
                                                let #TimeSc = match #sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc::#TryNewSc(#VSc.time()) {
                                                    Ok(v_c5af739c) => v_c5af739c,
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#TimeUcc {
                                                            #ErSc,
                                                            loc: loc_lib::loc!(),
                                                        });
                                                    }
                                                };
                                                Ok(Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts))
                                            }
                                        }
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => gen_int_range_check_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => gen_int_range_check_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc),
                                    }
                                }
                                IsNl::True => gen_match_opt_ts(&ident_stdrt_nn_orgn_ucc),
                            },
                            PgTypePattern::ArrDim1 { dim1_is_nl } => gen_arr_dims_init_ts(&dim1_ident_orgn_ts(dim1_is_nl)),
                        }
                    };
                    quote! {
                        pub fn #TryNewSc(#v_ident_inn_type_ts) -> Result<Self, #ident_stdrt_nn_orgn_try_new_er_ucc> {
                            #ts
                        }
                    }
                });
                let mb_fn_new_or_try_new_for_de_token = {
                    let gen_v_pg_range_int_type_ts = |int_range_type: &IntRangeType| {
                        let type_ts = {
                            let ts = int_range_type_to_range_inn_type_ts(int_range_type);
                            quote! {std::ops::Bound<#ts>}
                        };
                        quote! {
                            start_9a8ef454: #type_ts,
                            end_a14eb2b9: #type_ts
                        }
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            IsNl::False => match &pg_type_deserialize {
                                PgTypeDeserialize::Derive => if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                                    match &pg_type {
                                        PgType::I16AsInt2 |
                                        PgType::I32AsInt4 |
                                        PgType::I64AsInt8 |
                                        PgType::F32AsFloat4 |
                                        PgType::F64AsFloat8 |
                                        PgType::I16AsSmallSerialInitByPg |
                                        PgType::I32AsSerialInitByPg |
                                        PgType::I64AsBigSerialInitByPg |
                                        PgType::BoolAsBool |
                                        PgType::StdVecVecU8AsBytea |
                                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange |
                                        PgType::SqlxPgTypesPgMoneyAsMoney |
                                        PgType::SqlxTypesChronoNaiveTimeAsTime |
                                        PgType::SqlxTypesTimeTimeAsTime |
                                        PgType::SqlxPgTypesPgIntervalAsInterval |
                                        PgType::SqlxTypesChronoNaiveDateAsDate |
                                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                        PgType::StringAsText |
                                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => Ts2::new(),
                                        PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_v_pg_range_int_type_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_v_pg_range_int_type_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                    }
                                }
                                else {
                                    Ts2::new()
                                },
                                PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDe(_) => Ts2::new()
                            },
                            IsNl::True => Ts2::new(),
                        },
                        PgTypePattern::ArrDim1 { .. } => Ts2::new(),
                    }
                };
                quote! {
                    #AllowClippyArbitrarySrcItemOrdering
                    impl #ident_orgn_ucc {
                        #fn_new_or_try_new_ts
                        #mb_fn_new_or_try_new_for_de_token
                    }
                }
            };
            let impl_from_ident_orgn_for_ident_inn_type_ts = gen_impl_from_ts(
                &ident_orgn_ucc,
                &ident_inn_type_ts,
                &{
                    let v_dot_zero = quote! {#VSc.0};
                    let gen_match_ts = |
                        match_ts: &dyn ToTokens,
                        some_ts: &dyn ToTokens,
                        some_v_ts: &dyn ToTokens,
                    | quote! {
                        #match_ts.map(|#some_v_ts|#some_v_ts.0#some_ts)
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            IsNl::False => v_dot_zero,
                            IsNl::True => gen_match_ts(
                                &v_dot_zero,
                                &Ts2::new(),
                                &quote!{v_6bfd70fa}
                            ),
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nl } => {
                            let el_dot_zero_ts = quote! {el_6910aab7.0};
                            let dim1_ts = match &dim1_is_nl {
                                IsNl::False => el_dot_zero_ts,
                                IsNl::True => gen_match_ts(
                                    &el_dot_zero_ts,
                                    &Ts2::new(),
                                    &quote!{v_1b8cbd77}
                                ),
                            };
                            let into_iter_dim1_ts = quote! {.into_iter().map(|el_6910aab7|#dim1_ts).collect()};
                            match &is_nl {
                                IsNl::False => quote! {
                                    #v_dot_zero #into_iter_dim1_ts
                                },
                                IsNl::True => gen_match_ts(
                                    &v_dot_zero,
                                    &into_iter_dim1_ts,
                                    &quote!{v_38cfcd24}
                                ),
                            }
                        }
                    }
                }
            );
            let mb_impl_is_string_empty_for_ident_orgn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                match &is_nl {
                    IsNl::False => match &pg_type {
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
                            &ident_orgn_ucc,
                            &quote! {self.0.clone().is_empty()},
                        ),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_impl_crate_is_string_empty_for_ident_ts(
                            &ident_orgn_ucc,
                            &quote! {self.0.to_string().is_empty()},
                        ),
                    },
                    IsNl::True => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            let mb_impl_ser_for_ident_stdrt_nn_orgn_ts = match &ser_derive_or_impl {
                DeriveOrImpl::Derive => &Ts2::new(),
                DeriveOrImpl::Impl(v) => v,
            };
            let mb_impl_de_for_ident_stdrt_nn_orgn_ts = match &de_derive_or_impl {
                DeriveOrImpl::Derive => &Ts2::new(),
                DeriveOrImpl::Impl(v) => v,
            };
            let md_de_from_for_ident_stndrt_nn_orgn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                let self_sqlx_pg_types_pg_range_ts = {
                    let (start_ts, end_ts) = {
                        let gen_ts = |start_or_end: StartOrEnd|{
                            let name_ts = match start_or_end {
                                StartOrEnd::End => quote!{end},
                                StartOrEnd::Start => quote!{start},
                            };
                            let ts0 = match start_or_end {
                                StartOrEnd::End => quote!{v.1},
                                StartOrEnd::Start => quote!{v.0},
                            };
                            quote!{
                                #name_ts: match #ts0 {
                                    std::ops::Bound::Included(v0) => std::ops::Bound::Included(v0.0),
                                    std::ops::Bound::Excluded(v0) => std::ops::Bound::Excluded(v0.0),
                                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                },
                            }
                        };
                        (gen_ts(StartOrEnd::Start), gen_ts(StartOrEnd::End))
                    };
                    quote!{Self(sqlx::postgres::types::PgRange {
                        #start_ts
                        #end_ts
                    })}
                };
                let gen_impl_try_from_ts_e9596027 = |
                    from_type_ts: &dyn ToTokens,
                    ts: &dyn ToTokens,
                |gen_impl_from_ts(
                    from_type_ts,
                    &ident_orgn_ucc,
                    ts,
                );
                match &pg_type {
                    PgType::I16AsInt2 |
                    PgType::I32AsInt4 |
                    PgType::I64AsInt8 |
                    PgType::F32AsFloat4 |
                    PgType::F64AsFloat8 |
                    PgType::I16AsSmallSerialInitByPg |
                    PgType::I32AsSerialInitByPg |
                    PgType::I64AsBigSerialInitByPg |
                    PgType::BoolAsBool |
                    PgType::StringAsText |
                    PgType::StdVecVecU8AsBytea |
                    PgType::SqlxTypesChronoNaiveTimeAsTime |
                    PgType::SqlxTypesTimeTimeAsTime |
                    PgType::SqlxTypesChronoNaiveDateAsDate |
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                    PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ts2::new(),
                    PgType::SqlxPgTypesPgMoneyAsMoney => gen_impl_try_from_ts_e9596027(
                        &quote!{i64},
                        &quote!{Self::new(#inn_type_stdrt_nn_ts(v))}
                    ),
                    PgType::SqlxPgTypesPgIntervalAsInterval => gen_impl_try_from_ts_e9596027(
                        &quote!{(i32,i32,i64)},
                        &quote!{
                            Self(sqlx::postgres::types::PgInterval {
                                #MonthsSc: v.0,
                                #DaysSc: v.1,
                                #MicrosecondsSc: v.2,
                            })
                        }
                    ),
                    //todo reuse naming
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => gen_impl_try_from_ts_e9596027(
                        &quote!{(#sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts,SqlxTypesChronoNaiveTimeAsNnTimeOrgn)},
                        &quote!{Self(#inn_type_stdrt_nn_ts::#NewSc(v.0.0, v.1.0))}
                    ),
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => gen_impl_try_from_ts_e9596027(
                        &quote!{(#sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts,SqlxTypesChronoNaiveTimeAsNnTimeOrgn)},
                        &{
                            let ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                v.0.0,
                                v.1.0
                            }));
                            quote!{Self(#ts)}
                        }
                    ),
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_impl_try_from_ts_e9596027(
                        &quote!{[u8; 6]},
                        &quote!{Self(#inn_type_stdrt_nn_ts::new(v))}
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_impl_try_from_ts_e9596027(
                        &{
                            let bound_ts = quote!{std::ops::Bound<#sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts>};
                            quote!{(#bound_ts,#bound_ts)}
                        },
                        &self_sqlx_pg_types_pg_range_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_impl_try_from_ts_e9596027(
                        &{
                            let bound_ts = quote!{std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNnTimestampOrgn>};
                            quote!{(#bound_ts,#bound_ts)}
                        },
                        &self_sqlx_pg_types_pg_range_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_impl_try_from_ts_e9596027(
                        &{
                            let bound_ts = quote!{std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzOrgn>};
                            quote!{(#bound_ts,#bound_ts)}
                        },
                        &self_sqlx_pg_types_pg_range_ts
                    ),
                }
            }
            else {
                Ts2::new()
            };
            let md_de_try_from_for_ident_stndrt_nn_orgn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                let gen_self_match_try_new_ts = |prms_ts_04a82119: &dyn ToTokens, match_er_vrts_ts: &dyn ToTokens| {
                    quote! {
                        match Self::#TryNewSc(#prms_ts_04a82119) {
                            Ok(v_b318fc86) => Ok(v_b318fc86),
                            Err(er) => match er {
                                #match_er_vrts_ts
                            }
                        }
                    }
                };
                let gen_impl_try_from_ts_9de9a96b = |
                    from_type_ts: &dyn ToTokens,
                    er_type_ts: &dyn ToTokens,
                    ts: &dyn ToTokens
                |gen_impl_try_from_ts(
                    from_type_ts,
                    &ident_orgn_ucc,
                    er_type_ts,
                    ts
                );
                let gen_impl_try_from_ts_756a7745 = |
                    from_type_ts: &dyn ToTokens,
                    ts: &dyn ToTokens
                |gen_impl_try_from_ts_9de9a96b(
                    from_type_ts,
                    &ident_stdrt_nn_orgn_try_new_for_de_er_ucc,
                    ts
                );
                let gen_impl_try_from_ts_ab5ed01e = |
                    int_range_type: IntRangeType,
                |gen_impl_try_from_ts_756a7745(
                    &{
                        let ts0 = match int_range_type {
                            IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range => &quote!{i32},
                            IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range => &quote!{i64},
                        };
                        quote!{(std::ops::Bound<#ts0>,std::ops::Bound<#ts0>)}
                    },
                    &gen_self_match_try_new_ts(
                        &quote! {sqlx::postgres::types::PgRange { #StartSc: v.0, #EndSc: v.1 }},
                        &{
                            let gen_match_ts = |name_ts: &dyn ToTokens, ts: &dyn ToTokens|quote! {
                                #ident_stdrt_nn_orgn_try_new_er_ucc::#name_ts {
                                    loc,
                                    #ts
                                } => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#name_ts {
                                    loc,
                                    #ts
                                }),
                            };
                            let (
                                included_start_greater_than_included_end_ts,
                                included_start_greater_than_excluded_end_ts,
                                excluded_start_greater_than_included_end_ts,
                                excluded_start_greater_than_excluded_end_ts,
                            ) = {
                                let gen_ts = |ts: &dyn ToTokens|gen_match_ts(
                                    &ts,
                                    &quote!{
                                        #StartSc,
                                        #EndSc,
                                    }
                                );
                                (
                                    gen_ts(&IncludedStartGreaterThanIncludedEndUcc),
                                    gen_ts(&IncludedStartGreaterThanExcludedEndUcc),
                                    gen_ts(&ExcludedStartGreaterThanIncludedEndUcc),
                                    gen_ts(&ExcludedStartGreaterThanExcludedEndUcc),
                                )
                            };
                            let included_end_cannot_be_max_ts = gen_match_ts(
                                &IncludedEndCannotBeMaxUcc,
                                &quote!{#EndSc,}
                            );
                            quote! {
                                #included_start_greater_than_included_end_ts
                                #included_start_greater_than_excluded_end_ts
                                #excluded_start_greater_than_included_end_ts
                                #excluded_start_greater_than_excluded_end_ts
                                #included_end_cannot_be_max_ts
                            }
                        },
                    )
                );
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
                    PgType::StdVecVecU8AsBytea |
                    PgType::SqlxPgTypesPgIntervalAsInterval |
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ts2::new(),
                    PgType::StringAsText => gen_impl_try_from_ts_9de9a96b(
                        &inn_type_stdrt_nn_ts,
                        &ident_stdrt_nn_orgn_try_new_er_ucc,
                        &quote!{Self::try_new(v)}//todo use try_from instead of try_new ?
                    ),
                    PgType::SqlxTypesChronoNaiveTimeAsTime => gen_impl_try_from_ts_756a7745(
                        &quote!{(u32,u32,u32,u32)},
                        &quote!{
                            match #inn_type_stdrt_nn_ts::from_hms_micro_opt(
                                v.0,
                                v.1,
                                v.2,
                                v.3,
                            ) {
                                Some(v_b143b9e1) => {
                                    if <#inn_type_stdrt_nn_ts as chrono::Timelike>::nanosecond(&v_b143b9e1).checked_rem(1000).expect("c0514180") != 0 {
                                        return Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                            #VSc: v_b143b9e1.to_string(),
                                            loc: loc_lib::loc!(),
                                        });
                                    }
                                    Ok(Self(v_b143b9e1))
                                },
                                None => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                    #HourSc: v.0,
                                    #MinSc: v.1,
                                    #SecSc: v.2,
                                    #MicroSc: v.3,
                                    loc: loc_lib::loc!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxTypesTimeTimeAsTime => gen_impl_try_from_ts_756a7745(
                        &quote!{(u8,u8,u8,u32)},
                        &quote!{
                            match #inn_type_stdrt_nn_ts::from_hms_micro(
                                v.0,
                                v.1,
                                v.2,
                                v.3,
                            ) {
                                Ok(v_9932d535) => {
                                    if v_9932d535.nanosecond().checked_rem(1000).expect("0def33ce") != 0 {
                                        return Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#NanosecondPrecisionIsNotSupportedUcc {
                                            #VSc: v_9932d535.to_string(),
                                            loc: loc_lib::loc!(),
                                        });
                                    }
                                    Ok(Self(v_9932d535))
                                },
                                Err(er) => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#InvalidHourOrMinuteOrSecondOrMicrosecondUcc {
                                    #HourSc: v.0,
                                    #MinuteSc: v.1,
                                    #SecondSc: v.2,
                                    #MicrosecondSc: v.3,
                                    #ErSc: er.to_string(),
                                    loc: loc_lib::loc!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxTypesChronoNaiveDateAsDate => gen_impl_try_from_ts_756a7745(
                        &quote!{sqlx::types::chrono::NaiveDate},
                        &gen_self_match_try_new_ts(
                            &VSc,
                            &quote! {
                                #ident_stdrt_nn_orgn_try_new_er_ucc::#EarlierDateNotSupportedUcc {
                                    value,
                                    #EarliestSupportedDateSc,
                                    loc,
                                } => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#EarlierDateNotSupportedUcc {
                                    value,
                                    #EarliestSupportedDateSc,
                                    loc,
                                }),
                            }
                        )
                    ),
                    PgType::SqlxTypesUuidUuidAsUuidInitByClient | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => gen_impl_try_from_ts_756a7745(
                        &quote!{String},
                        &quote!{
                            match uuid::Uuid::try_parse(&v) {
                                Ok(v0) => Ok(Self(v0)),
                                Err(er) => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#NotUuidUcc {
                                    #VSc: er.to_string(),
                                    loc: loc_lib::loc!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_impl_try_from_ts_ab5ed01e(
                        IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range
                    ),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_impl_try_from_ts_ab5ed01e(
                        IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range
                    ),
                }
            }
            else {
                Ts2::new()
            };
            let impl_display_for_ident_orgn_ts = gen_impl_display_ts(&Ts2::new(), &ident_orgn_ucc, &Ts2::new(), &quote! {write!(f, "{self:?}")});
            let impl_loc_lib_to_err_string_for_ident_orgn_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_orgn_ucc, &Ts2::new(), &quote! {self.to_string()});
            let impl_dflt_some_one_el_for_ident_orgn_ts = gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_orgn_ucc, &{
                let ts = match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        IsNl::False => {
                            let pg_range_int_dflt_init_ts = quote! {
                                sqlx::postgres::types::PgRange {
                                    start: std::ops::Bound::Included(#CoreDefault),
                                    end: std::ops::Bound::Excluded(#CoreDefault),
                                }
                            };
                            let gen_as_dflt_some_one_el_call_ts = |ts: &dyn ToTokens| {
                                quote! {<#ts as #import::DfltSomeOneEl>::dflt_some_one_el()}
                            };
                            let gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts = |ts: &dyn ToTokens| {
                                let ts0 = gen_as_dflt_some_one_el_call_ts(&ts);
                                quote! {sqlx::postgres::types::PgRange {
                                    #StartSc: std::ops::Bound::Included(#ts0.0),
                                    #EndSc: std::ops::Bound::Excluded(#ts0.0),
                                }}
                            };
                            let sqlx_types_chrono_naive_date_as_nn_date_orgn_as_dflt_some_one_el_call_ts = gen_as_dflt_some_one_el_call_ts(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc);
                            let sqlx_types_chrono_naive_time_as_nn_time_orgn_as_dflt_some_one_el_call_ts = gen_as_dflt_some_one_el_call_ts(&sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc);
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
                                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => &quote! {#ft_h::default()},
                                PgType::SqlxTypesUuidUuidAsUuidInitByClient => &quote! {#ident_inn_type_ts::default()},
                                PgType::SqlxPgTypesPgMoneyAsMoney => &quote! {#inn_type_stdrt_nn_ts(#CoreDefault)},
                                PgType::StdVecVecU8AsBytea => &quote! {vec![#CoreDefault]},
                                PgType::SqlxTypesTimeTimeAsTime => &gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote! {0,0,0,0}),
                                PgType::SqlxPgTypesPgIntervalAsInterval => &quote! {#inn_type_stdrt_nn_ts {
                                    #MonthsSc: #CoreDefault,
                                    #DaysSc: #CoreDefault,
                                    #MicrosecondsSc: #CoreDefault
                                }},
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                    #sqlx_types_chrono_naive_date_as_nn_date_orgn_as_dflt_some_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_nn_time_orgn_as_dflt_some_one_el_call_ts.0,
                                }),
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! {
                                    #sqlx_types_chrono_naive_date_as_nn_date_orgn_as_dflt_some_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_nn_time_orgn_as_dflt_some_one_el_call_ts.0,
                                })),
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet => &quote! {
                                    sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#NewSc(core::net::Ipv4Addr::UNSPECIFIED, #CoreDefault).expect("9e9c9b57"))
                                },
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => &pg_range_int_dflt_init_ts,
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts(&sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc),
                            };
                            quote! {#init_ts}
                        }
                        IsNl::True => quote! {Some(#PgCrudCmnDfltSomeOneElCall)},
                    },
                    PgTypePattern::ArrDim1 { .. } => match &is_nl {
                        IsNl::False => quote! {vec![#PgCrudCmnDfltSomeOneElCall]},
                        IsNl::True => quote! {Some(#PgCrudCmnDfltSomeOneElCall)},
                    },
                };
                quote! {Self(#ts)}
            });
            let impl_sqlx_type_for_ident_orgn_ts = gen_impl_sqlx_type_for_ident_ts(&ident_orgn_ucc, &ft_h);
            let impl_sqlx_encode_sqlx_pg_for_ident_orgn_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_orgn_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_pg_for_ident_orgn_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(&ident_orgn_ucc, &ft_h, &{
                let scopes_v_ts = quote! {(v)};
                let ok_self_scopes_v_ts = quote! {Ok(Self #scopes_v_ts)};
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        IsNl::False => match &pg_type {
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
                        IsNl::True => ok_self_scopes_v_ts,
                    },
                    PgTypePattern::ArrDim1 { .. } => ok_self_scopes_v_ts,
                }
            });
            let impl_sqlx_pg_pg_has_arr_type_for_ident_orgn_ts = {
                quote! {
                    impl sqlx::postgres::PgHasArrayType for #ident_orgn_ucc {
                        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                            <#inn_type_stdrt_nn_ts as sqlx::postgres::PgHasArrayType>::array_type_info()
                        }
                    }
                }
            };
            let mb_impl_from_ident_rd_for_ident_orgn_ts = match &is_nn_stdrt_can_be_pk {
                IsNnStdrtCanBePk::False => Ts2::new(),
                IsNnStdrtCanBePk::True => gen_impl_from_ts(&ident_stdrt_nn_rd_ucc, &ident_orgn_ucc, &{
                    let ident_stdrt_nn_as_crate_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_nn_ucc);
                    quote! {Self::#NewSc(#ident_stdrt_nn_as_crate_pg_type_ts::into_inn(#VSc))}
                }),
            };
            quote! {
                #ident_orgn_ts
                #mb_pub_enum_ident_stdrt_nn_orgn_try_new_er_ts
                #mb_pub_enum_ident_stdrt_nn_orgn_try_new_for_de_er_ts
                #impl_ident_orgn_ts
                #impl_from_ident_orgn_for_ident_inn_type_ts
                #mb_impl_is_string_empty_for_ident_orgn_ts
                #mb_impl_ser_for_ident_stdrt_nn_orgn_ts
                #mb_impl_de_for_ident_stdrt_nn_orgn_ts
                #md_de_from_for_ident_stndrt_nn_orgn_ts
                #md_de_try_from_for_ident_stndrt_nn_orgn_ts
                #impl_display_for_ident_orgn_ts
                #impl_loc_lib_to_err_string_for_ident_orgn_ts
                #impl_dflt_some_one_el_for_ident_orgn_ts
                #impl_sqlx_type_for_ident_orgn_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_orgn_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_orgn_ts
                #impl_sqlx_pg_pg_has_arr_type_for_ident_orgn_ts
                #mb_impl_from_ident_rd_for_ident_orgn_ts
            }
        };
        let gen_pub_struct_tokens_ts = |ident_ts_46b769df: &dyn ToTokens, ts: &dyn ToTokens, derive_dflt: DDefault| {
            DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_default_if(derive_dflt)
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_ts_46b769df,
                    &Ts2::new(),
                    &ts
                )
        };
        let ident_orgn_struct_ts = quote!{(#ident_orgn_ucc);};
        let ident_tt_ucc = SelfTtUcc::from_tokens(&ident);
        let ident_tt_ts = {
            let ident_tt_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_partial_ord_if(match &is_stdrt_nn {
                    IsStdrtNn::False => DPartialOrd::False,
                    IsStdrtNn::True => match &pg_type {
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
                        | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => DPartialOrd::True,
                        PgType::SqlxPgTypesPgMoneyAsMoney
                        | PgType::SqlxPgTypesPgIntervalAsInterval
                        | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                        | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                        | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => DPartialOrd::False,
                    },
                })
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_tt_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_ts
                );
            let impl_ident_tt_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_tt_ucc);
            let impl_dflt_some_one_el_for_ident_tt_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_tt_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            let impl_sqlx_type_for_ident_tt_ts = gen_impl_sqlx_type_for_ident_ts(&ident_tt_ucc, &ident_orgn_ucc);
            let impl_sqlx_encode_sqlx_pg_for_ident_tt_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_tt_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_pg_for_ident_tt_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(&ident_tt_ucc, &ident_orgn_ucc, &quote! {Ok(Self(v))});
            //todo rewrite as dependency of PgType trait?
            let impl_pg_type_eq_oprtr_for_ident_tt_ts = impl_pg_type_eq_oprtr_for_ident_ts(
                &import,
                &ident_tt_ucc,
                //todo
                &{
                    let eq_ts = EqOprtrH::Eq.to_tokens_path(&import);
                    let is_null_ts = EqOprtrH::IsNull.to_tokens_path(&import);
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            IsNl::False => eq_ts,
                            IsNl::True => quote! {
                                if self.0.0.is_some() {
                                    #eq_ts
                                }
                                else {
                                    #is_null_ts
                                }
                            },
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nl } => match &is_nl {
                            IsNl::False => match &dim1_is_nl {
                                IsNl::False => eq_ts,
                                IsNl::True => {
                                    //todo thats not actually usefull coz nl arr comparison has different logic. need to refactor EqOprtrH enum
                                    eq_ts
                                }
                            },
                            IsNl::True => quote! {
                                if self.0.0.is_some() {
                                    #eq_ts
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
                #ident_tt_ts
                #impl_ident_tt_ts
                #impl_dflt_some_one_el_for_ident_tt_ts
                #impl_sqlx_type_for_ident_tt_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_tt_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_tt_ts
                #impl_pg_type_eq_oprtr_for_ident_tt_ts
            }
        };
        let ident_stdrt_nn_tt_ucc = SelfTtUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_cr_ucc = SelfCrUcc::from_tokens(&ident);
        let ident_cr_ts = {
            let ident_cr_ts = match &can_be_pk {
                CanBePk::False => DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_copy_if(derive_copy)
                    .d_partial_eq()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .build_struct(
                        &Ts2::new(),
                        &ident_cr_ucc,
                        &Ts2::new(),
                        &ident_orgn_struct_ts
                    ),
                CanBePk::True => gen_pub_struct_tokens_ts(&ident_cr_ucc, &quote! {(());}, DDefault::False),
            };
            let mb_impl_ident_cr_ts = match &can_be_pk {
                CanBePk::False => gen_pub_const_new_or_pub_try_new_ts(&ident_cr_ucc),
                CanBePk::True => Ts2::new(),
            };
            let impl_dflt_some_one_el_for_ident_cr_ts = gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_cr_ucc, &{
                let ts: &dyn ToTokens = match &can_be_pk {
                    CanBePk::False => &PgCrudCmnDfltSomeOneElCall,
                    CanBePk::True => &quote! {()},
                };
                quote! {Self(#ts)}
            });
            let mb_impl_sqlx_encode_sqlx_pg_for_ident_cr_ts = match &can_be_pk {
                CanBePk::False => gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_cr_ucc, &quote! {#SelfSc.0}),
                CanBePk::True => Ts2::new(),
            };
            let mb_impl_sqlx_type_for_ident_cr_ts = match &can_be_pk {
                CanBePk::False => gen_impl_sqlx_type_for_ident_ts(&ident_cr_ucc, &ident_orgn_ucc),
                CanBePk::True => Ts2::new(),
            };
            quote! {
                #ident_cr_ts
                #mb_impl_ident_cr_ts
                #impl_dflt_some_one_el_for_ident_cr_ts
                #mb_impl_sqlx_encode_sqlx_pg_for_ident_cr_ts
                #mb_impl_sqlx_type_for_ident_cr_ts
            }
        };
        let ident_sel_ucc = SelfSelUcc::from_tokens(&ident);
        let ident_sel_ts = {
            let pub_struct_ident_sel_ts = gen_pub_struct_tokens_ts(
                &ident_sel_ucc,
                &match &pg_type_pattern {
                    PgTypePattern::Stdrt => quote! {;},
                    PgTypePattern::ArrDim1 { .. } => {
                        let mut args_ts = Vec::new();
                        for el0 in 1..=arr_dims_nbr {
                            let dim_nbr_pgn_ts = format!("dim{el0}_pgn").parse::<Ts2>().expect("af86f2d1");
                            args_ts.push(quote! {
                                #dim_nbr_pgn_ts: pg_types_cmn::PgnStartsWithOne
                            });
                        }
                        quote! {{#(#args_ts),*}}
                    }
                },
                DDefault::True,
            );
            let (impl_dflt_some_one_el_for_ident_sel_ts, impl_dflt_some_one_el_max_page_size_for_ident_sel_ts) = {
                let gen_ts = |dflt_some_one_or_dflt_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| match &pg_type_pattern {
                    PgTypePattern::Stdrt => quote! {Self},
                    PgTypePattern::ArrDim1 { .. } => {
                        let ts: &dyn ToTokens = match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudCmnDfltSomeOneElCall,
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudCmnDfltSomeOneElMaxPageSizeCall,
                        };
                        let mut args_ts = Vec::new();
                        for el0 in 1..=arr_dims_nbr {
                            let dim_nbr_pgn_ts = format!("dim{el0}_pgn").parse::<Ts2>().expect("e5250a98");
                            args_ts.push(quote! {
                                #dim_nbr_pgn_ts: #ts
                            });
                        }
                        quote! {Self {#(#args_ts),*}}
                    }
                };
                (
                    gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_sel_ucc, &gen_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)),
                    gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts(&ident_sel_ucc, &gen_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)),
                )
            };
            quote! {
                #pub_struct_ident_sel_ts
                #impl_dflt_some_one_el_for_ident_sel_ts
                #impl_dflt_some_one_el_max_page_size_for_ident_sel_ts
            }
        };
        let ident_rd_ucc = SelfRdUcc::from_tokens(&ident);
        let ident_wh_ucc = SelfWhUcc::from_tokens(&ident);
        let ident_wh_ts = gen_pg_type_wh_ts(
            &AllowClippyArbitrarySrcItemOrdering,
            &{
                let cmn_pg_type_flts = vec![PgTypeFlt::Eq { ident: quote! {#ident_tt_ucc} }];
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => {
                        let greater_than = PgTypeFlt::GreaterThan {
                            ident: quote! {#ident_stdrt_nn_tt_ucc},
                        };
                        let btwn = PgTypeFlt::Btwn {
                            ident: quote! {#ident_stdrt_nn_tt_ucc},
                        };
                        let in_h = PgTypeFlt::In { ident: quote! {#ident_tt_ucc} };
                        let rgx = PgTypeFlt::Rgx;
                        let eq_to_encoded_string_representation = PgTypeFlt::EqToEncodedStringRepresentation;
                        let date_9c6d41ca = PgTypeFlt::CrntDate;
                        let greater_than_crnt_date = PgTypeFlt::GreaterThanCrntDate;
                        let time_49c41c1c = PgTypeFlt::CrntTime;
                        let greater_than_crnt_time = PgTypeFlt::GreaterThanCrntTime;
                        let timestamp_ad2e556b = PgTypeFlt::CrntTimestamp;
                        let greater_than_crnt_timestamp = PgTypeFlt::GreaterThanCrntTimestamp;
                        let before = PgTypeFlt::Before {
                            ident: quote! {#ident_stdrt_nn_tt_ucc},
                        };
                        // let bit_vec_position_eq = PgTypeFlt::BitVecPositionEq;
                        let cmn_stdrt_pg_type_flts = { cmn_pg_type_flts };
                        let cmn_stdrt_pg_type_nbr_flts = {
                            let mut vec = cmn_stdrt_pg_type_flts.clone();
                            vec.push(greater_than.clone());
                            vec.push(btwn.clone());
                            vec.push(in_h.clone());
                            vec
                        };
                        let ranges_cmn_flt_vec = {
                            let mut vec = cmn_stdrt_pg_type_flts.clone();
                            vec.push(PgTypeFlt::FindRangesWithinGivenRange {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::FindRangesThatFullyContainTheGivenRange {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::StrictlyToLeftOfRange {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::StrictlyToRightOfRange {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::IncludedLowerBound {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::ExcludedUpperBound {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::GreaterThanIncludedLowerBound {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::GreaterThanExcludedUpperBound {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::OverlapWithRange {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::AdjacentWithRange {
                                ident: quote! {#ident_stdrt_nn_tt_ucc},
                            });
                            vec.push(PgTypeFlt::RangeLen);
                            vec
                        };
                        match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::F64AsFloat8
                            | PgType::I16AsSmallSerialInitByPg
                            | PgType::I32AsSerialInitByPg
                            | PgType::I64AsBigSerialInitByPg => cmn_stdrt_pg_type_nbr_flts,
                            PgType::SqlxPgTypesPgMoneyAsMoney => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(in_h);
                                vec
                            }
                            PgType::StdVecVecU8AsBytea => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(eq_to_encoded_string_representation);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(greater_than);
                                vec.push(btwn);
                                vec.push(time_49c41c1c);
                                vec.push(greater_than_crnt_time);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(greater_than);
                                vec.push(btwn);
                                vec.push(date_9c6d41ca);
                                vec.push(greater_than_crnt_date);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(greater_than);
                                vec.push(btwn);
                                vec.push(timestamp_ad2e556b);
                                vec.push(greater_than_crnt_timestamp);
                                vec
                            }
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(before);
                                vec.push(btwn);
                                vec
                            }
                            PgType::StringAsText | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(rgx);
                                vec
                            }
                            PgType::BoolAsBool | PgType::SqlxPgTypesPgIntervalAsInterval | PgType::SqlxTypesIpnetworkIpNetworkAsInet => cmn_stdrt_pg_type_flts,
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = cmn_stdrt_pg_type_flts;
                                vec.push(greater_than);
                                vec.push(rgx);
                                vec
                            },
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ranges_cmn_flt_vec,
                        }
                    }
                    PgTypePattern::ArrDim1 { dim1_is_nl } => {
                        let dim_one_greater_than = PgTypeFlt::DimOneGreaterThan {
                            ident: quote! {#ident_stdrt_nn_tt_ucc},
                        };
                        let dim_one_btwn = PgTypeFlt::DimOneBtwn {
                            ident: quote! {#ident_stdrt_nn_tt_ucc},
                        };
                        let dim_one_in_h = PgTypeFlt::DimOneIn {
                            ident: {
                                let ts = SelfTtUcc::from_tokens(&match &pg_type.can_be_nl() {
                                    CanBeNl::False => quote! {#ident_stdrt_nn_ucc},
                                    CanBeNl::True => gen_ident_ts(pg_type, is_nl, &PgTypePattern::Stdrt)
                                });
                                quote!{#ts}
                            },
                        };
                        let dim_one_rgx = PgTypeFlt::DimOneRgx;
                        let dim_one_crnt_date = PgTypeFlt::DimOneCrntDate;
                        let dim_one_greater_than_crnt_date = PgTypeFlt::DimOneGreaterThanCrntDate;
                        let dim_one_crnt_time = PgTypeFlt::DimOneCrntTime;
                        let dim_one_greater_than_crnt_time = PgTypeFlt::DimOneGreaterThanCrntTime;
                        let dim_one_crnt_timestamp = PgTypeFlt::DimOneCrntTimestamp;
                        let dim_one_greater_than_crnt_timestamp = PgTypeFlt::DimOneGreaterThanCrntTimestamp;
                        let dim_one_before = PgTypeFlt::DimOneBefore {
                            ident: quote! {#ident_stdrt_nn_tt_ucc},
                        };
                        let cmn_arr_dim1_pg_type_flts = {
                            let mut vec = cmn_pg_type_flts;
                            vec.push(PgTypeFlt::DimOneEq {
                                ident: {
                                    let ts = SelfTtUcc::from_tokens(&match &dim1_is_nl {
                                        IsNl::False => &ident_stdrt_nn_ucc,
                                        IsNl::True => &ident_stdrt_nl_ucc,
                                    });
                                    quote! {#ts}
                                },
                            });
                            vec.push(PgTypeFlt::DimOneLenEq);
                            vec.push(PgTypeFlt::DimOneLenGreaterThan);
                            vec
                        };
                        let cmn_arr_dim1_pg_type_nbr_flts = {
                            let mut vec = cmn_arr_dim1_pg_type_flts.clone();
                            vec.push(dim_one_greater_than.clone());
                            vec.push(dim_one_btwn.clone());
                            vec.push(dim_one_in_h.clone());
                            vec
                        };
                        let (
                            wh_sqlx_pg_types_pg_range_i32_ts,
                            wh_sqlx_pg_types_pg_range_i64_ts,
                            wh_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            wh_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            wh_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        ) = {
                            let gen_ts = |range: Range| {
                                let pg_type_from_range = PgType::from(&range);
                                let range_el_ident_stdrt_nn_ts = gen_ident_stdrt_nn_ts(&pg_type_from_range);
                                let mut vec = cmn_arr_dim1_pg_type_flts.clone();
                                let range_el_ident_stdrt_nn_as_crate_pg_type_rd_ts = {
                                    let range_el_ident_stdrt_nn_as_crate_pg_type_ts = gen_as_pg_type_ts(&range_el_ident_stdrt_nn_ts);
                                    quote! {#range_el_ident_stdrt_nn_as_crate_pg_type_ts::Rd}
                                };
                                vec.push(PgTypeFlt::DimOneFindRangesWithinGivenRange {
                                    ident: quote! {#ident_stdrt_nn_tt_ucc},
                                });
                                vec.push(PgTypeFlt::DimOneFindRangesThatFullyContainTheGivenRange {
                                    ident: quote! {#ident_stdrt_nn_tt_ucc},
                                });
                                vec.push(PgTypeFlt::DimOneStrictlyToLeftOfRange {
                                    ident: quote! {#ident_stdrt_nn_tt_ucc},
                                });
                                vec.push(PgTypeFlt::DimOneStrictlyToRightOfRange {
                                    ident: quote! {#ident_stdrt_nn_tt_ucc},
                                });
                                vec.push(PgTypeFlt::DimOneIncludedLowerBound {
                                    ident: range_el_ident_stdrt_nn_as_crate_pg_type_rd_ts.clone(),
                                });
                                vec.push(PgTypeFlt::DimOneExcludedUpperBound {
                                    ident: range_el_ident_stdrt_nn_as_crate_pg_type_rd_ts.clone(),
                                });
                                vec.push(PgTypeFlt::DimOneGreaterThanIncludedLowerBound {
                                    ident: range_el_ident_stdrt_nn_as_crate_pg_type_rd_ts.clone(),
                                });
                                vec.push(PgTypeFlt::DimOneGreaterThanExcludedUpperBound {
                                    ident: range_el_ident_stdrt_nn_as_crate_pg_type_rd_ts,
                                });
                                vec.push(PgTypeFlt::DimOneOverlapWithRange {
                                    ident: quote! {#ident_stdrt_nn_tt_ucc},
                                });
                                vec.push(PgTypeFlt::DimOneAdjacentWithRange {
                                    ident: quote! {#ident_stdrt_nn_tt_ucc},
                                });
                                vec.push(PgTypeFlt::DimOneRangeLen);
                                vec
                            };
                            (
                                gen_ts(Range::I32AsInt4),
                                gen_ts(Range::I64AsInt8),
                                gen_ts(Range::SqlxTypesChronoNaiveDateAsDate),
                                gen_ts(Range::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                                gen_ts(Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
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
                            | PgType::I64AsBigSerialInitByPg => cmn_arr_dim1_pg_type_nbr_flts,
                            PgType::SqlxPgTypesPgMoneyAsMoney => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_in_h);
                                vec
                            }
                            PgType::StdVecVecU8AsBytea => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(PgTypeFlt::DimOneEqToEncodedStringRepresentation);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_btwn);
                                vec.push(dim_one_crnt_time);
                                vec.push(dim_one_greater_than_crnt_time);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateAsDate => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_btwn);
                                vec.push(dim_one_crnt_date);
                                vec.push(dim_one_greater_than_crnt_date);
                                vec
                            }
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_btwn);
                                vec.push(dim_one_crnt_timestamp);
                                vec.push(dim_one_greater_than_crnt_timestamp);
                                vec
                            }
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_before);
                                vec.push(dim_one_btwn);
                                vec
                            }
                            PgType::StringAsText | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_rgx);
                                vec
                            }
                            PgType::BoolAsBool | PgType::SqlxPgTypesPgIntervalAsInterval | PgType::SqlxTypesIpnetworkIpNetworkAsInet => cmn_arr_dim1_pg_type_flts,
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => {
                                let mut vec = cmn_arr_dim1_pg_type_flts;
                                vec.push(dim_one_greater_than);
                                vec.push(dim_one_rgx);
                                vec
                            }
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => wh_sqlx_pg_types_pg_range_i32_ts,
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => wh_sqlx_pg_types_pg_range_i64_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => wh_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => wh_sqlx_pg_types_pg_range_sqlx_types_chrono_naive_date_time_ts,
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => wh_sqlx_pg_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_ts,
                        }
                    }
                }
            }
            .iter()
            .map(|el0| {
                let el1: &dyn PgFlt = el0;
                el1
            })
            .collect(),
            &ident,
            &ShouldDeriveUtoipaToSchema::False,
            &ShouldDSchemarsJsonSchema::False,
            &IsQbMut::False,
        );
        let ident_rd_ts = {
            let ident_rd_ts = {
                let (
                    derive_eq,
                    derive_partial_ord,
                    derive_ord
                ) = match &is_nn_stdrt_can_be_pk {
                    IsNnStdrtCanBePk::False => (
                        DEq::False,
                        DPartialOrd::False,
                        DOrd::False
                    ),
                    IsNnStdrtCanBePk::True => (
                        DEq::True,
                        DPartialOrd::True,
                        DOrd::True
                    ),
                };
                DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_copy_if(derive_copy)
                    .d_partial_eq()
                    .d_eq_if(derive_eq)
                    .d_partial_ord_if(derive_partial_ord)
                    .d_ord_if(derive_ord)
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .build_struct(
                        &Ts2::new(),
                        &ident_rd_ucc,
                        &Ts2::new(),
                        &ident_orgn_struct_ts
                    )
            };
            let impl_ident_rd_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_rd_ucc);
            let impl_loc_lib_to_err_string_for_ident_rd_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_rd_ucc, &Ts2::new(), &quote! {self.0.to_string()});
            let impl_crate_dflt_some_one_el_for_ident_rd_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_rd_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            let impl_sqlx_encode_sqlx_pg_for_ident_orgn_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_rd_ucc, &quote! {#SelfSc.0});
            let impl_sqlx_decode_sqlx_pg_for_ident_rd_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_rd_ucc,
                &ident_orgn_ucc,
                &quote! {Ok(Self(v))}
            );
            let impl_sqlx_type_for_ident_rd_ts = gen_impl_sqlx_type_for_ident_ts(&ident_rd_ucc, &ident_orgn_ucc);
            let mb_impl_pg_type_wh_flt_for_ident_rd_if_can_be_pk_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                impl_pg_type_wh_flt_for_ident_ts(
                    &quote! {<'lt>},
                    &ident_stdrt_nn_rd_ucc,
                    &Ts2::new(),
                    &IncrPrmUndrscr::False,
                    &ColPrmUndrscr::False,
                    &AddOprtrUndrscr::True,
                    &quote! {
                        match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                            Ok(v_8da76391) => Ok(format!("({col} = ${v_8da76391})")),
                            Err(er) => Err(er)
                        }
                    },
                    &IsQbMut::True,
                    &gen_typical_qb_ts(&SelfSc),
                    &import,
                )
            } else {
                Ts2::new()
            };
            quote! {
                #ident_rd_ts
                #impl_ident_rd_ts
                #impl_loc_lib_to_err_string_for_ident_rd_ts
                #impl_crate_dflt_some_one_el_for_ident_rd_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_orgn_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_rd_ts
                #impl_sqlx_type_for_ident_rd_ts
                #mb_impl_pg_type_wh_flt_for_ident_rd_if_can_be_pk_ts
            }
        };
        let ident_rd_ids_ucc = SelfRdIdsUcc::from_tokens(&ident);
        let ident_rd_ids_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
            let ident_rd_ids_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_rd_ids_ucc,
                    &Ts2::new(),
                    &quote!{(#ident_rd_ucc);},
                );
            let impl_sqlx_decode_sqlx_pg_for_ident_rd_ids_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_rd_ids_ucc,
                &ident_rd_ucc,
                &quote! {Ok(Self(v))}
            );
            let impl_sqlx_type_for_ident_rd_ids_ts = gen_impl_sqlx_type_for_ident_ts(&ident_rd_ids_ucc, &ident_rd_ucc);
            quote! {
                #ident_rd_ids_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_rd_ids_ts
                #impl_sqlx_type_for_ident_rd_ids_ts
            }
        } else {
            Ts2::new()
        };
        let ident_rd_inn_ucc = SelfRdInnUcc::from_tokens(&ident);
        let ident_rd_inn_ts = quote! {
            pub type #ident_rd_inn_ucc = #ident_inn_type_ts;
        };
        let ident_upd_ts = {
            let ident_upd_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_upd_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_ts
                );
            let impl_ident_upd_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_upd_ucc);
            let impl_dflt_some_one_el_for_ident_upd_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_upd_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            let impl_loc_lib_to_err_string_for_ident_upd_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_upd_ucc, &Ts2::new(), &quote! {self.0.#ToErrStringSc()});
            quote! {
                #ident_upd_ts
                #impl_ident_upd_ts
                #impl_dflt_some_one_el_for_ident_upd_ts
                #impl_loc_lib_to_err_string_for_ident_upd_ts
            }
        };
        let ident_upd_for_query_ucc = SelfUpdForQueryUcc::from_tokens(&ident);
        let ident_upd_for_query_ts = {
            let ident_upd_for_query_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_upd_for_query_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_ts
                );
            let impl_sqlx_type_for_ident_upd_for_query_ts = gen_impl_sqlx_type_for_ident_ts(&ident_upd_for_query_ucc, &ident_orgn_ucc);
            let impl_sqlx_encode_sqlx_pg_for_ident_upd_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_upd_for_query_ucc, &quote! {#SelfSc.0});
            let impl_from_ident_upd_for_ident_upd_for_query_ts = gen_impl_from_ts(&ident_upd_ucc, &ident_upd_for_query_ucc, &quote! {Self(#VSc.0)});
            quote! {
                #ident_upd_for_query_ts
                #impl_sqlx_type_for_ident_upd_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_upd_for_query_ts
                #impl_from_ident_upd_for_ident_upd_for_query_ts
            }
        };
        let impl_pg_type_for_ident_ts = {
            let gen_ok_string_from_tokens_ts = |ts: &dyn ToTokens| {
                quote! {Ok(#StringTs::from(#ts))}
            };
            let ok_string_from_dflt_ts = gen_ok_string_from_tokens_ts(&quote! {"dflt"});
            let ok_string_from_uuid_generate_v4_ts = gen_ok_string_from_tokens_ts(&quote! {"uuid_generate_v4()"});
            let typical_qp_ts = {
                let if_write_is_err_ts = gen_if_write_is_err_ts(
                    &quote! {acc_c7df00f5, "${v_ba581e0f}"},
                    &gen_return_err_qp_er_write_into_buffer_ts(import)
                );
                quote! {
                    let mut acc_c7df00f5 = String::default();
                    match #import::incr_checked_add_one_returning_incr(#IncrSc) {
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
            let (qp_cr_ts, bind_v_to_query_cr_ts): H<'_> = {
                let typical: H<'_> = { (&typical_qp_ts, &typical_qb_ts) };
                let dflt_init_by_pg: H<'_> = (&ok_string_from_dflt_ts, &ok_query_ts);
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
                    PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg => dflt_init_by_pg,
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => (&ok_string_from_uuid_generate_v4_ts, &ok_query_ts),
                }
            };
            let sel_only_ids_and_sel_only_updd_ids_query_cmn_ts = {
                let format_ts = dq_ts(&{
                    let col_comma = "{col},";
                    if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) { col_comma.to_owned() } else { format!("'{{{{\\\"v\\\": null}}}}'::jsonb as {col_comma}") }
                });
                quote! {Ok(format!(#format_ts))}
            };
            gen_impl_pg_type_ts(
                &import,
                &ident,
                &ident_tt_ucc,
                &match &can_be_pk {
                    CanBePk::False => IsPkUndrscr::True,
                    CanBePk::True => IsPkUndrscr::False,
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
                    let mb_arr_part = match &pg_type_pattern {
                        PgTypePattern::Stdrt => String::new(),
                        PgTypePattern::ArrDim1 { .. } => repeat_n("[]", arr_dims_nbr).collect::<String>(),
                    };
                    let mb_constraint_part = match &pg_type_pattern {
                        PgTypePattern::Stdrt => String::new(),
                        PgTypePattern::ArrDim1 { dim1_is_nl } => match &dim1_is_nl {
                            IsNl::False => ",check (array_position({col},null) is null)".to_owned(),
                            IsNl::True => String::new(),
                        },
                    };
                    let mb_pk_is_pk_ts = quote! {pg_types_cmn::mb_pk(is_pk)};
                    let col_pg_query_type = format!("{{col}} {pg_query_type}{mb_arr_part}{mb_constraint_part}");
                    let col_pg_query_type_nn = format!("{{col}} {pg_query_type}{mb_arr_part} not null{mb_constraint_part}");
                    let space_extra_prm = " {}";
                    match (&is_nl, &can_be_pk) {
                        (IsNl::False, CanBePk::False) => {
                            let format_ts = dq_ts(&col_pg_query_type_nn);
                            quote! {
                                format!(#format_ts)
                            }
                        }
                        (IsNl::False, CanBePk::True) => {
                            let format_ts = dq_ts(&format!("{col_pg_query_type_nn}{space_extra_prm}"));
                            quote! {
                                format!(#format_ts, #mb_pk_is_pk_ts)
                            }
                        }
                        (IsNl::True, CanBePk::False) => {
                            let format_ts = dq_ts(&col_pg_query_type);
                            quote! {
                                format!(#format_ts)
                            }
                        }
                        (IsNl::True, CanBePk::True) => {
                            let format_ts = dq_ts(&format!("{col_pg_query_type}{space_extra_prm}"));
                            quote! {
                                format!(#format_ts, #mb_pk_is_pk_ts)
                            }
                        }
                    }
                },
                &ident_cr_ucc,
                &CrQpValueUndrscr::True,
                &match &can_be_pk {
                    CanBePk::False => CrQpIncrUndrscr::False,
                    CanBePk::True => CrQpIncrUndrscr::True,
                },
                &qp_cr_ts,
                &match &can_be_pk {
                    CanBePk::False => CrQbValueUndrscr::False,
                    CanBePk::True => CrQbValueUndrscr::True,
                },
                &match &can_be_pk {
                    CanBePk::False => IsCrQbMut::True,
                    CanBePk::True => IsCrQbMut::False,
                },
                &bind_v_to_query_cr_ts,
                &ident_sel_ucc,
                &match &el.pg_type_pattern {
                    PgTypePattern::Stdrt => SelQpValueUndrscr::True,
                    PgTypePattern::ArrDim1 { .. } => SelQpValueUndrscr::False,
                },
                &{
                    let ts = match &pg_type_pattern {
                        PgTypePattern::Stdrt => quote! {#ColSc.to_owned()},
                        PgTypePattern::ArrDim1 { .. } => {
                            let format_ts = dq_ts(&{
                                let acc = repeat_n("[{}:{}]", arr_dims_nbr).collect::<String>();
                                format!("{{col}}{acc}")
                            });
                            let args_ts = (1..=arr_dims_nbr)
                            .map(|el0| {
                                let dim_nbr_pgn_ts = format!("dim{el0}_pgn")
                                .parse::<Ts2>()
                                .expect("6f2305ee");
                                quote! {
                                    #VSc.#dim_nbr_pgn_ts.start(),
                                    #VSc.#dim_nbr_pgn_ts.end(),
                                }
                            });
                            quote! {format!(
                                #format_ts,
                                #(#args_ts)*
                            )}
                        }
                    };
                    quote! {Ok(#ts)}
                },
                &ident_wh_ucc,
                &ident_rd_ucc,
                &{
                    let gen_ident_rd_ident_orgn_ts = |ts: &dyn ToTokens| {
                        quote! {#ident_rd_ucc(#ident_orgn_ucc(#ts))}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            IsNl::False => {
                                Range::try_from(pg_type).as_ref().map_or_else(
                                    |()| quote! {#VSc},
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
                                                #ident_stdrt_nn_rd_ucc(#ident_stdrt_nn_orgn_ucc(match (
                                                    #VSc.0.0.#StartSc,
                                                    #VSc.0.0.#EndSc
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
                                        let gen_if_start_end_eq_ts = |true_ts: &dyn ToTokens, false_ts: &dyn ToTokens| {
                                            quote! {
                                                if #StartSc == #EndSc {
                                                    #true_ts
                                                } else {
                                                    #false_ts
                                                }
                                            }
                                        };
                                        let if_eq_unbounded_unbounded_or_included_excluded_ts = gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_included_excluded_ts);
                                        let int_range_normalize_ts = {
                                            let (
                                                included_start_checked_add_ts,
                                                excluded_end_checked_add_ts
                                            ) = {
                                                let gen_ts = |first_ts: &dyn ToTokens, second_ts: &dyn ToTokens| {
                                                    quote! {#first_ts(#second_ts.checked_add(1).expect("0ec0992f"))}
                                                };
                                                (
                                                    gen_ts(&IncludedUcc, &StartSc),
                                                    gen_ts(&ExcludedUcc, &EndSc)
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
                                                &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_included_excluded_ts),
                                                &included_unbounded_ts,
                                                &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_checked_add_ts),
                                                &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_ts),
                                                &included_checked_add_unbounded_ts,
                                                &unbounded_excluded_checked_add_ts,
                                                &unbounded_excluded_ts,
                                            )
                                        };
                                        let range_match_timestamp_and_timestamp_tz_ts = gen_range_match_ts(
                                            &gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &included_end_ts),
                                            &if_eq_unbounded_unbounded_or_included_excluded_ts,
                                            &sqlx_pg_types_pg_range_included_unbounded_ts,
                                            &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_excluded_included_ts),
                                            &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_excluded_excluded_ts),
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
                                                    &if_eq_unbounded_unbounded_or_included_excluded_ts,
                                                    &sqlx_pg_types_pg_range_included_unbounded_ts,
                                                    &gen_if_start_end_eq_ts(
                                                        &sqlx_pg_types_pg_range_unbounded_unbounded_ts,
                                                        &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"98a0357b-d21a-4949-a101-c641528d2376"), &gen_excluded_end_succ_opt_ts(&"fe53a6b9-2d7e-4605-9f5a-7f5c21cc01e6")),
                                                    ),
                                                    &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"d8a26635-c478-4a2a-acf4-bf1765702889"), &excluded_end_ts)),
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
                            IsNl::True => gen_ident_rd_ident_orgn_ts(&quote! {
                                #VSc.0.0.map(
                                    |v_4561270e|
                                    <
                                        #ident_stdrt_nn_ucc
                                        as
                                        #import::PgType
                                    >::normalize(
                                        #ident_stdrt_nn_rd_ucc(v_4561270e)
                                    ).0
                                )
                            }),
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nl } => match (&is_nl, &dim1_is_nl) {
                            (IsNl::False, IsNl::False) => gen_ident_rd_ident_orgn_ts(&quote! {
                                #VSc.0.0.into_iter().map(|el|{
                                    #ident_stdrt_nn_as_pg_type_ts::normalize(
                                        #ident_stdrt_nn_rd_ucc(el)
                                    ).0
                                }).collect()
                            }),
                            (IsNl::False, IsNl::True) => gen_ident_rd_ident_orgn_ts(&{
                                let ident_ts_e4c5a2a3 = gen_ident_ts(pg_type, &IsNl::True, &PgTypePattern::Stdrt);
                                let ident_arr_stdrt_nl_rd_ucc = SelfRdUcc::from_tokens(&ident_ts_e4c5a2a3);
                                quote! {
                                    #VSc.0.0.into_iter().map(|el|{
                                        #ident_stdrt_nl_as_pg_type_ts::normalize(
                                            #ident_arr_stdrt_nl_rd_ucc(el)
                                        ).0
                                    }).collect()
                                }
                            }),
                            (IsNl::True, IsNl::False) => gen_ident_rd_ident_orgn_ts(&{
                                let ident_arr_dim1_nn_nn_ucc = gen_ident_ts(
                                    pg_type,
                                    &IsNl::False,
                                    &PgTypePattern::ArrDim1 {
                                        dim1_is_nl: IsNl::False,
                                    },
                                );
                                let ident_arr_dim1_nn_nn_rd_ucc = SelfRdUcc::from_tokens(&ident_arr_dim1_nn_nn_ucc);
                                quote! {
                                    #VSc.0.0.map(|v_b4d912fb|
                                        <
                                            #ident_arr_dim1_nn_nn_ucc
                                            as
                                            #import::PgType
                                        >::normalize(
                                            #ident_arr_dim1_nn_nn_rd_ucc(v_b4d912fb),
                                        ).0
                                    )
                                }
                            }),
                            (IsNl::True, IsNl::True) => gen_ident_rd_ident_orgn_ts(&{
                                let ident_arr_dim1_nn_nl_ucc = gen_ident_ts(
                                    pg_type,
                                    &IsNl::False,
                                    &PgTypePattern::ArrDim1 {
                                        dim1_is_nl: IsNl::True,
                                    },
                                );
                                let ident_arr_dim1_nn_nl_rd_ucc = SelfRdUcc::from_tokens(&ident_arr_dim1_nn_nl_ucc);
                                quote! {
                                    #VSc.0.0.map(
                                        |v_dd042db2|
                                        <
                                            #ident_arr_dim1_nn_nl_ucc
                                            as
                                            #import::PgType
                                        >::normalize(
                                            #ident_arr_dim1_nn_nl_rd_ucc(v_dd042db2),
                                        ).0
                                    )
                                }
                            }),
                        },
                    }
                },
                &if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                    quote! {#ident_rd_ids_ucc}
                } else {
                    quote! {#import_non_pk_pg_type_rd_ids_ts}
                },
                &sel_only_ids_and_sel_only_updd_ids_query_cmn_ts,
                &ident_rd_inn_ucc,
                &{
                    let gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts = |ts: &dyn ToTokens| {
                        quote! {
                            #ident_stdrt_nn_as_pg_type_ts::into_inn(
                                #ident_stdrt_nn_rd_ucc(#ts)
                            )
                        }
                    };
                    let v_dot_zero_ts = quote! {#VSc.0};
                    let v_dot_zero_dot_zero_ts = quote! {#v_dot_zero_ts.0};
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            IsNl::False => {
                                if range_try_from_pg_type_is_ok {
                                    gen_pg_range_conversion_ts(&v_dot_zero_dot_zero_ts, &quote!{v_af65ccce})
                                } else {
                                    v_dot_zero_dot_zero_ts
                                }
                            }
                            IsNl::True => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts(&quote!{v_bd169d3b})
                                } else {
                                    quote!{v_bd169d3b.0}
                                };
                                quote! {#v_dot_zero_dot_zero_ts.map(|v_bd169d3b| #ts)}
                            }
                        },
                        PgTypePattern::ArrDim1 { dim1_is_nl } => match (&is_nl, &dim1_is_nl) {
                            (IsNl::False, IsNl::False) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts(&quote!{el_f5e94f0c})
                                } else {
                                    quote! {el_f5e94f0c.0}
                                };
                                quote! {
                                    #v_dot_zero_dot_zero_ts.into_iter().map(|el_f5e94f0c|#ts).collect()
                                }
                            }
                            (IsNl::False, IsNl::True) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts(&quote!{v_e9a6bd41})
                                } else {
                                    quote!{v_e9a6bd41.0}
                                };
                                quote! {
                                    #v_dot_zero_dot_zero_ts.into_iter().map(|el_236259fc|
                                        el_236259fc.0.map(|v_e9a6bd41| #ts)
                                    ).collect()
                                }
                            }
                            (IsNl::True, IsNl::False) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts(&quote!{el_b37be63e})
                                } else {
                                    quote! {el_b37be63e.0}
                                };
                                quote! {
                                    #v_dot_zero_dot_zero_ts.map(|v_47fb2e43|
                                        v_47fb2e43.0.into_iter().map(|el_b37be63e|#ts).collect()
                                    )
                                }
                            }
                            (IsNl::True, IsNl::True) => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts(&quote!{v_e5c5f65c})
                                } else {
                                    quote!{v_e5c5f65c.0}
                                };
                                quote! {
                                    #v_dot_zero_dot_zero_ts.map(|v_b1a259c4| v_b1a259c4.0.into_iter().map(|el_19a7e6d0|
                                        el_19a7e6d0.0.map(|v_e5c5f65c| #ts)
                                    ).collect())
                                }
                            }
                        },
                    }
                },
                &ident_upd_ucc,
                &ident_upd_for_query_ucc,
                &UpdQpValueUndrscr::True,
                &UpdQpJsonbSetAccumulatorUndrscr::True,
                &UpdQpJsonbSetTargetUndrscr::True,
                &UpdQpJsonbSetPathUndrscr::True,
                &typical_qp_ts,
                &IsUpdQbMut::True,
                &typical_qb_ts,
                &sel_only_ids_and_sel_only_updd_ids_query_cmn_ts,
                &IsSelOnlyUpddIdsQbMut::False,
                &quote! {Ok(#QuerySc)},
            )
        };
        let impl_pg_type_test_cases_for_ident_ts = {
            enum IsNeedToUseInto {
                False,
                True,
            }
            let gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts = |rd_or_upd: &RdOrUpd| {
                let rd_or_upd_ucc = rd_or_upd.ucc();
                let ts = if pg_type_init_try_new_try_from_pg_type.is_ok() {
                    quote! {#TryNewSc(#VSc).expect("69477d2f")}
                } else {
                    quote! {#NewSc(#VSc)}
                };
                quote! {<#SelfUcc::#PgTypeUcc
                    as
                #import::#PgTypeUcc>::#rd_or_upd_ucc:: #ts}
            };
            let gen_stdrt_nn_test_case_h_ts = |is_need_to_use_into: &IsNeedToUseInto| {
                let gen_range_rd_ids_to_2_dims_vec_rd_inn_ts =
                    |min_ts: &dyn ToTokens, negative_less_typical_ts: &dyn ToTokens, negative_more_typical_ts: &dyn ToTokens, near_zero_ts: &dyn ToTokens, positive_less_typical_ts: &dyn ToTokens, positive_more_typical_ts: &dyn ToTokens, max_ts: &dyn ToTokens| {
                        enum Bnd<'lt> {
                            Excl(&'lt dyn ToTokens),
                            Incl(&'lt dyn ToTokens),
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
                let gen_int_pgrange_rd_ids_to_2_dims_vec_rd_inn_ts = |int_range_type: &IntRangeType| {
                    let range_inn_type_ts = int_range_type_to_range_inn_type_ts(int_range_type);
                    gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(&quote! {#range_inn_type_ts::MIN}, &quote! {-20}, &quote! {-10}, &quote! {0}, &quote! {10}, &quote! {20}, &quote! {#range_inn_type_ts::MAX - 1})
                };
                let empty_vec_ts = quote! {Vec::new()};
                let gen_ident_stdrt_nn_fn_ts = |
                    ident_8b874ea5: &dyn ToTokens,
                    ts: &dyn ToTokens
                |quote!{#ident_8b874ea5::#ts()};
                let (
                    ident_sqlx_types_chrono_naive_time_min_ts,
                    ident_sqlx_types_chrono_naive_time_ten_ts,
                    ident_sqlx_types_chrono_naive_time_twenty_ts,
                    ident_sqlx_types_chrono_naive_time_max_ts
                ) = {
                    let gen_ts = |
                        ts_fd88ca39: &dyn ToTokens
                    |gen_ident_stdrt_nn_fn_ts(
                        &gen_ident_stdrt_nn_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime),
                        &ts_fd88ca39
                    );
                    (
                        gen_ts(
                            &sqlx_types_chrono_naive_time_min_fn_ts
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_time_ten_fn_ts
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_time_twenty_fn_ts
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_time_max_fn_ts
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
                    let gen_ts = |
                        ts_7c66f815: &dyn ToTokens
                    |gen_ident_stdrt_nn_fn_ts(
                        &gen_ident_stdrt_nn_ts(&PgType::SqlxTypesChronoNaiveDateAsDate),
                        &ts_7c66f815
                    );
                    (
                        gen_ts(
                            &sqlx_types_chrono_naive_date_min_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_negative_less_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_negative_more_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_near_zero_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_positive_less_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_positive_more_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_max_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_ts,
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
                    let gen_ts = |date: &dyn ToTokens, time: &dyn ToTokens| {
                        gen_sqlx_types_chrono_naive_date_time_new_ts(&quote! { #date, #time })
                    };
                    (
                        gen_ts(&ident_sqlx_types_chrono_naive_date_min_ts, &ident_sqlx_types_chrono_naive_time_min_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_negative_less_typical_ts, &ident_sqlx_types_chrono_naive_time_twenty_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_negative_more_typical_ts, &ident_sqlx_types_chrono_naive_time_ten_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_near_zero_ts, &ident_sqlx_types_chrono_naive_time_min_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_positive_less_typical_ts, &ident_sqlx_types_chrono_naive_time_ten_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_positive_more_typical_ts, &ident_sqlx_types_chrono_naive_time_twenty_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_max_ts, &ident_sqlx_types_chrono_naive_time_max_ts),
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
                    let gen_ts = |ts: &dyn ToTokens| gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(ts);
                    (
                        gen_ts(&sqlx_types_chrono_naive_date_time_min_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_negative_less_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_negative_more_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_near_zero_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_positive_less_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_positive_more_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_max_ts),
                    )
                };
                let gen_typical_test_cases_vec_ts = |ts: &dyn ToTokens| {
                    let ts0 = match &is_need_to_use_into {
                        IsNeedToUseInto::True => quote! {.into()},
                        IsNeedToUseInto::False => Ts2::new(),
                    };
                    quote! {#import::#ts()#ts0}
                };
                let gen_ts = |ts: &dyn ToTokens| gen_ident_stdrt_nn_fn_ts(&SelfUcc, &ts);
                match &pg_type {
                    PgType::I16AsInt2 => gen_typical_test_cases_vec_ts(&quote! {i16_test_cases_vec}),
                    PgType::I32AsInt4 => gen_typical_test_cases_vec_ts(&quote! {i32_test_cases_vec}),
                    PgType::I64AsInt8 => gen_typical_test_cases_vec_ts(&quote! {i64_test_cases_vec}),
                    PgType::F32AsFloat4 => gen_typical_test_cases_vec_ts(&quote! {f32_test_cases_vec}),
                    PgType::F64AsFloat8 => gen_typical_test_cases_vec_ts(&quote! {f64_test_cases_vec}),
                    PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg => empty_vec_ts,
                    PgType::SqlxPgTypesPgMoneyAsMoney => quote! {
                        #import::i64_test_cases_vec().into_iter().map(
                            #inn_type_stdrt_nn_ts
                        ).collect::<Vec<#inn_type_stdrt_nn_ts>>()
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
                        let self_sqlx_types_chrono_naive_time_min_ts = gen_ts(&sqlx_types_chrono_naive_time_min_fn_ts);
                        let self_sqlx_types_chrono_naive_time_ten_ts = gen_ts(&sqlx_types_chrono_naive_time_ten_fn_ts);
                        let self_sqlx_types_chrono_naive_time_twenty_ts = gen_ts(&sqlx_types_chrono_naive_time_twenty_fn_ts);
                        let self_sqlx_types_chrono_naive_time_max_ts = gen_ts(&sqlx_types_chrono_naive_time_max_fn_ts);
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
                        let sqlx_types_chrono_naive_date_min_ts = gen_ts(&sqlx_types_chrono_naive_date_min_fn_ts);
                        let sqlx_types_chrono_naive_date_negative_less_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_negative_less_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_negative_more_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_negative_more_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_near_zero_ts = gen_ts(&sqlx_types_chrono_naive_date_near_zero_fn_ts);
                        let sqlx_types_chrono_naive_date_positive_less_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_positive_less_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_positive_more_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_positive_more_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_max_ts = gen_ts(&sqlx_types_chrono_naive_date_max_fn_ts);
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
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_int_pgrange_rd_ids_to_2_dims_vec_rd_inn_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_int_pgrange_rd_ids_to_2_dims_vec_rd_inn_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(
                        &ident_sqlx_types_chrono_naive_date_min_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_near_zero_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_max_pred_opt_expect_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(
                        &sqlx_types_chrono_naive_date_time_min_ts,
                        &sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_near_zero_ts,
                        &sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_max_ts,
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(
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
            let opt_vec_cr_ts = {
                let gen_some_acc_ts = |
                    is_nl_27c3e340: &IsNl,
                    ident_ts_3fadfa9d: &dyn ToTokens,
                    additonal_ts: &dyn ToTokens
                | {
                    let (new_or_try_new_ts, mb_acc_push_none_ts) = match (&is_nl_27c3e340, pg_type_init_try_new_try_from_pg_type.is_ok()) {
                        (IsNl::False, true) => (quote! {try_new(vec![el_0fd5865b.0.into()]).expect("adbae6b3")}, Ts2::new()),
                        (IsNl::False, false) => (quote! {new(vec![el_0fd5865b.0.into()])}, Ts2::new()),
                        (IsNl::True, true) => (
                            quote! {try_new(Some(el_0fd5865b.0.into())).expect("b244d498")},
                            quote! {acc_0b59a062.push(#self_as_pg_type_ts::Cr::try_new(None).expect("31878971"));},
                        ),
                        (IsNl::True, false) => (quote! {new(Some(el_0fd5865b.0.into()))}, quote! {acc_0b59a062.push(#self_as_pg_type_ts::Cr::new(None));}),
                    };
                    let ident_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_ts_3fadfa9d);
                    quote! {Some({
                        let mut acc_0b59a062 = Vec::new();
                        for el_0fd5865b in #ident_as_pg_type_test_cases_ts::#OptVecCrSc().unwrap_or(Vec::new()) {
                            acc_0b59a062.push(#self_as_pg_type_ts::Cr::#new_or_try_new_ts);
                        }
                        #mb_acc_push_none_ts
                        #additonal_ts
                        acc_0b59a062
                    })}
                };
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        IsNl::False => match &can_be_pk {
                            CanBePk::False => {
                                let ts = gen_stdrt_nn_test_case_h_ts(&IsNeedToUseInto::False);
                                let new_or_try_new_ts = {
                                    let self_as_pg_type_cr_ts = quote!{#self_as_pg_type_ts::Cr};
                                    if pg_type_init_try_new_try_from_pg_type.is_ok() {
                                        quote! {
                                            |el_043a7d30|#self_as_pg_type_cr_ts::try_new(
                                                el_043a7d30
                                            ).expect("941bd15c")
                                        }
                                    } else {
                                        quote! {#self_as_pg_type_cr_ts::#NewSc}
                                    }
                                };
                                quote! {Some(
                                    #ts.into_iter().map(
                                        #new_or_try_new_ts
                                    ).collect()
                                )}
                            }
                            CanBePk::True => none_ts.clone(),
                        },
                        IsNl::True => gen_some_acc_ts(is_nl, &gen_ident_ts(pg_type, &IsNl::False, &PgTypePattern::Stdrt), &Ts2::new()),
                    },
                    PgTypePattern::ArrDim1 { dim1_is_nl } => gen_some_acc_ts(
                        is_nl,
                        &gen_ident_ts(
                            pg_type,
                            &match &is_nl {
                                IsNl::False => *dim1_is_nl,
                                IsNl::True => IsNl::False,
                            },
                            &match &is_nl {
                                IsNl::False => PgTypePattern::Stdrt,
                                IsNl::True => PgTypePattern::ArrDim1 { dim1_is_nl: *dim1_is_nl },
                            },
                        ),
                        &match &is_nl {
                            IsNl::False => {
                                let ts: &dyn ToTokens = match &dim1_is_nl {
                                    IsNl::False => &ident_stdrt_nn_as_pg_type_test_cases_ts,
                                    IsNl::True => &ident_stdrt_nl_as_pg_type_test_cases_ts,
                                };
                                let (first_ts, second_ts, third_ts) = {
                                    let gen_new_or_try_new_ts = |ts0: &dyn ToTokens| {
                                        if pg_type_init_try_new_try_from_pg_type.is_ok() {
                                            quote! {try_new(#ts0).expect("75ad9383")}
                                        } else {
                                            quote! {new(#ts0)}
                                        }
                                    };
                                    let gen_ts = |v: usize| {
                                        let nbr_ts = v.to_string().parse::<Ts2>().expect("50c87202");
                                        //todo mb correlate with .d_copy_if()
                                        let mb_dot_clone_ts_060db18f: &dyn ToTokens = match &pg_type {
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
                                        quote! {vec![v_6465e8ae #mb_dot_clone_ts_060db18f.0.into(); #nbr_ts]}
                                    };
                                    (
                                        gen_new_or_try_new_ts(&quote! {
                                            #ts::#OptVecCrSc().unwrap_or(Vec::new())
                                            .into_iter()
                                            .map(|el_ffb375dd|el_ffb375dd.0.into())
                                            .collect()
                                        }),
                                        gen_new_or_try_new_ts(&gen_ts(2)),
                                        gen_new_or_try_new_ts(&gen_ts(1000)),
                                    )
                                };
                                quote! {
                                    acc_0b59a062.push(#self_as_pg_type_ts::Cr::#first_ts);
                                    if let Some(v_6465e8ae) = #ts::#OptVecCrSc().unwrap_or(Vec::new()).first() {
                                        acc_0b59a062.push(#self_as_pg_type_ts::Cr::#second_ts);
                                        acc_0b59a062.push(#self_as_pg_type_ts::Cr::#third_ts);
                                    }
                                }
                            }
                            IsNl::True => Ts2::new(),
                        },
                    ),
                }
            };
            let rd_ids_to_2_dims_vec_rd_inn_ts = {
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
                    PgTypePattern::Stdrt => match &is_nl {
                        IsNl::False => {
                            let ts = gen_stdrt_nn_test_case_h_ts(&IsNeedToUseInto::True);
                            quote! {vec![{#ts}]}
                        }
                        IsNl::True => quote! {
                            #ident_stdrt_nn_as_pg_type_test_cases_ts::#RdIdsTo2DimsVecRdInnSc(#RdIdsSc)
                            .into_iter()
                            .flat_map(|el0| el0.into_iter().map(|el1| vec![Some(el1)]))
                            .chain(std::iter::once(vec![None]))
                            .collect()
                        },
                    },
                    PgTypePattern::ArrDim1 { dim1_is_nl } => match &is_nl {
                        IsNl::False => match &dim1_is_nl {
                            IsNl::False => {
                                let el_d27d1981_ts = gen_star_or_dot_clone_ts(&quote!{el_d27d1981});
                                quote! {
                                    let mut acc_abf96c9f = Vec::new();
                                    let rd_ids_to_2_dims_vec_rd_inn = #ident_stdrt_nn_as_pg_type_test_cases_ts::#RdIdsTo2DimsVecRdInnSc(#RdIdsSc);
                                    let opt_extra = {
                                        let mut opt_extra = None;
                                        for el_cb3f4b45 in &rd_ids_to_2_dims_vec_rd_inn {
                                            if opt_extra.is_some() {
                                                break;
                                            }
                                            for el_d27d1981 in el_cb3f4b45 {
                                                if opt_extra.is_none() {
                                                    opt_extra = Some((vec![
                                                        vec![#el_d27d1981_ts]],
                                                        vec![vec![#el_d27d1981_ts, #el_d27d1981_ts]
                                                    ]));
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                        }
                                        opt_extra
                                    };
                                    let has_len_greater_than_one = {
                                        let mut has_len_greater_than_one = false;
                                        for el_89e74982 in &rd_ids_to_2_dims_vec_rd_inn {
                                            if el_89e74982.len() > 1 {
                                                has_len_greater_than_one = true;
                                                break;
                                            }
                                        }
                                        has_len_greater_than_one
                                    };
                                    for el_cb836246 in rd_ids_to_2_dims_vec_rd_inn {
                                        acc_abf96c9f.push(vec![el_cb836246]);
                                    }
                                    if let Some(v_e22f9ad2) = opt_extra {
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
                            IsNl::True => {
                                let el_6b831e7c_ts = gen_star_or_dot_clone_ts(&quote!{el_6b831e7c});
                                quote! {
                                    let mut acc_68eba82f = Vec::new();
                                    let rd_ids_to_2_dims_vec_rd_inn = #ident_stdrt_nl_as_pg_type_test_cases_ts::#RdIdsTo2DimsVecRdInnSc(#RdIdsSc);
                                    let opt_extra = {
                                        let mut opt_extra = None;
                                        for el_b04183c6 in &rd_ids_to_2_dims_vec_rd_inn {
                                            if opt_extra.is_some() {
                                                break;
                                            }
                                            for el_6b831e7c in el_b04183c6 {
                                                if opt_extra.is_none() {
                                                    opt_extra = Some((
                                                        vec![vec![#el_6b831e7c_ts]],
                                                        vec![vec![#el_6b831e7c_ts, #el_6b831e7c_ts]]
                                                    ));
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                        }
                                        opt_extra
                                    };
                                    let has_len_greater_than_one = rd_ids_to_2_dims_vec_rd_inn.len() > 1;
                                    acc_68eba82f.push(vec![
                                        rd_ids_to_2_dims_vec_rd_inn
                                        .into_iter()
                                        .flat_map(IntoIterator::into_iter)
                                        .collect()
                                    ]);
                                    if let Some(v_a0f0f172) = opt_extra {
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
                        IsNl::True => {
                            let ts = match &dim1_is_nl {
                                IsNl::False => &ident_arr_nn_as_pg_type_test_cases_ts,
                                IsNl::True => &ident_arr_nl_as_pg_type_test_cases_ts,
                            };
                            let el_31abc64a_ts = gen_star_or_dot_clone_ts(&quote!{el_31abc64a});
                            quote! {
                                let mut acc_5f7f59ac = Vec::new();
                                let rd_ids_to_2_dims_vec_rd_inn = #ts::#RdIdsTo2DimsVecRdInnSc(#RdIdsSc);
                                let opt_extra = {
                                    let mut opt_extra = None;
                                    for el_12a259ab in &rd_ids_to_2_dims_vec_rd_inn {
                                        if opt_extra.is_some() {
                                            break;
                                        }
                                        for el_16a61773 in el_12a259ab {
                                            if opt_extra.is_some() {
                                                break;
                                            }
                                            for el_31abc64a in el_16a61773 {
                                                if opt_extra.is_none() {
                                                    opt_extra = Some((
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
                                    opt_extra
                                };
                                let has_len_greater_than_one = {
                                    let mut has_len_greater_than_one = false;
                                    for el_a177c6a3 in &rd_ids_to_2_dims_vec_rd_inn {
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
                                    rd_ids_to_2_dims_vec_rd_inn
                                    .into_iter()
                                    .flatten()
                                    .flatten()
                                    .collect()
                                )]);
                                acc_5f7f59ac.push(vec![None]);
                                if let Some(v_3530786a) = opt_extra {
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
            let rd_inn_into_rd_with_new_or_try_new_unwraped_ts = gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(&RdOrUpd::Rd);
            let rd_inn_into_upd_with_new_or_try_new_unwraped_ts = gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(&RdOrUpd::Upd);
            let upd_to_rd_ids_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                quote! {
                    #ident_rd_ids_ucc(#ident_rd_ucc(#VSc.0 #mb_dot_clone_ts))//todo its not correct. must be only for pk but it for all types what van be pk
                }
            } else {
                let ts = gen_v_init_ts0(&none_ts);
                quote! {
                    #import_non_pk_pg_type_rd_ids_ts(#ts)
                }
            };
            let rd_ids_to_opt_v_rd_dflt_some_one_el_ts = {
                //todo that is not correct for arr of generated by pg pks but mb just need to remove this vrts and thats it?
                let ts = gen_v_init_ts0(&{
                    let ts: &dyn ToTokens = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                        &quote! {#VSc.0 #mb_dot_clone_ts}
                    } else {
                        &PgCrudCmnDfltSomeOneElCall
                    };
                    quote! {#self_pg_type_as_pg_type_ts::normalize(#ts)}
                });
                quote! {Some(#ts)}
            };
            let previous_rd_and_opt_upd_into_rd_ts = quote! {
                #OptUpdSc.map_or(#RdSc, |#VSc| #ident_rd_ucc(#VSc.0))
            };
            let rd_ids_and_cr_into_rd_ts = {
                let ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                    quote! {#RdIdsSc.0}
                } else {
                    quote! {#ident_rd_ucc(#CrSc.0)}
                };
                quote! {
                    #self_pg_type_as_pg_type_ts::normalize(#ts)
                }
            };
            let rd_ids_and_cr_into_opt_v_rd_ts = {
                let ts = gen_v_init_ts0(&quote! {
                    <Self as #import::PgTypeTestCases>::#RdIdsAndCrIntoRdSc(
                        #RdIdsSc,
                        #CrSc
                    )
                });
                quote! {Some(#ts)}
            };
            let rd_ids_and_cr_into_tt_ts = {
                let ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                    quote! {#RdIdsSc.0.0}
                } else {
                    quote! {#CrSc.0}
                };
                quote! {#ident_tt_ucc(#ts)}
            };
            //todo mb it into fn (not in proc macro)
            let rd_ids_and_cr_into_wh_eq_ts = {
                let ts = if matches!(&pg_type_pattern, PgTypePattern::Stdrt)
                    && matches!(&is_nl, IsNl::False)
                    && matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True)
                {
                    quote! {#RdIdsSc.0.0}
                } else {
                    quote! {#CrSc.0}
                };
                quote! {
                    #ident_wh_ucc::#EqUcc(wh_flts::PgTypeWhEq {
                        oprtr: #import::Oprtr::Or,
                        #VSc: #ident_tt_ucc(#ts),
                    })
                }
            };
            let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts = quote! {
                #import::NotEmptyUnqVec::try_new(vec![
                    #rd_ids_and_cr_into_wh_eq_ts
                ]).expect("4c08b551")
            };
            let rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts = none_ts.clone();
            let cr_into_pg_type_opt_vec_wh_dim_one_eq_ts = match &pg_type_pattern {
                PgTypePattern::Stdrt => none_ts.clone(),
                PgTypePattern::ArrDim1 { dim1_is_nl } => {
                    let ident_stdrt_is_nl_tt_ucc: &dyn ToTokens = match &dim1_is_nl {
                        IsNl::False => &ident_stdrt_nn_tt_ucc,
                        IsNl::True => &ident_stdrt_nl_tt_ucc,
                    };
                    let some_ts = {
                        let ts: &dyn ToTokens = match &is_nl {
                            IsNl::False => &quote! {#CrSc.0.0},
                            IsNl::True => &quote! {v_09152b2e.0},
                        };
                        quote! {
                            match #import::NotEmptyUnqVec::try_new({
                                let mut acc_74c71d5d = Vec::new();
                                for (i_7702518c, el_081d735b) in #ts.into_iter().enumerate() {
                                    acc_74c71d5d.push(
                                        #ident_wh_ucc::DimOneEq(
                                            wh_flts::PgTypeWhDimOneEq {
                                                oprtr: #import::Oprtr::Or,
                                                dims: wh_flts::BoundedVec::try_from(
                                                    vec![
                                                        pg_crud_cmn::NotZeroUnsignedPartOfI32::try_from(
                                                            i32::try_from(i_7702518c.checked_add(1)?).expect("5954966c")
                                                        ).expect("8d269b8f")
                                                    ]
                                                ).expect("fe1e037f"),
                                                #VSc: #ident_stdrt_is_nl_tt_ucc(el_081d735b),
                                            }
                                        )
                                    );
                                }
                                acc_74c71d5d
                            }) {
                                Ok(v_2218be19) => Some(v_2218be19),
                                Err(er) => match er {
                                    #import::NotEmptyUnqVecTryNewEr::IsEmpty {..} => None,
                                    #import::NotEmptyUnqVecTryNewEr::NotUnq {..} => panic!("45c8de3c")
                                }
                            }
                        }
                    };
                    match &is_nl {
                        IsNl::False => some_ts,
                        IsNl::True => quote! {
                            match #CrSc.0.0 {
                                Some(v_09152b2e) => #some_ts,
                                None => None
                            }
                        },
                    }
                }
            };
            let pg_type_opt_vec_wh_greater_than_test_ts = {
                let greater_than = PgTypeGreaterThanVrt::GreaterThan;
                let not_greater_than = PgTypeGreaterThanVrt::NotGreaterThan;
                let eq_not_greater_than = PgTypeGreaterThanVrt::EqNotGreaterThan;
                let gen_greater_than_test_ts = |greater_than_vrt_ts: &PgTypeGreaterThanVrt, cr_ts: &dyn ToTokens, tt_ts: &dyn ToTokens| {
                    quote! {
                        #import::PgTypeGreaterThanTest {
                            vrt: #import::PgTypeGreaterThanVrt::#greater_than_vrt_ts,
                            cr: #self_as_pg_type_ts::Cr::#cr_ts,
                            greater_than: #self_as_pg_type_ts::Tt::#tt_ts,
                        }
                    }
                };
                let gen_greater_than_test_new_new_ts =
                    |greater_than_vrt_ts: &PgTypeGreaterThanVrt, cr_ts: &dyn ToTokens, greater_than_ts: &dyn ToTokens| gen_greater_than_test_ts(greater_than_vrt_ts, &quote! {new(#cr_ts)}, &quote! {new(#greater_than_ts)});
                let gen_greater_than_test_try_new_try_new_ts = |greater_than_vrt_ts: &PgTypeGreaterThanVrt, cr_ts: &dyn ToTokens, greater_than_ts: &dyn ToTokens| {
                    gen_greater_than_test_ts(
                        greater_than_vrt_ts,
                        &quote! {try_new(#cr_ts).expect("8327c651")},
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
                    let eq_not_greater_than_less_ts = gen_greater_than_test_new_new_ts(&eq_not_greater_than, &less_ts, &less_ts);
                    let eq_not_greater_than_zero_ts = gen_greater_than_test_new_new_ts(&eq_not_greater_than, &zero_ts, &zero_ts);
                    let eq_not_greater_than_more_ts = gen_greater_than_test_new_new_ts(&eq_not_greater_than, &more_ts, &more_ts);
                    quote! {
                        #greater_than_less_ts,
                        #greater_than_zero_ts,
                        #greater_than_more_ts,
                        #not_greater_than_less_ts,
                        #not_greater_than_zero_ts,
                        #not_greater_than_more_ts,
                        #eq_not_greater_than_less_ts,
                        #eq_not_greater_than_zero_ts,
                        #eq_not_greater_than_more_ts
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
                    let eq_not_greater_than_less_ts = gen_greater_than_test_try_new_try_new_ts(&eq_not_greater_than, &less_ts, &less_ts);
                    let eq_not_greater_than_zero_ts = gen_greater_than_test_try_new_try_new_ts(&eq_not_greater_than, &zero_ts, &zero_ts);
                    let eq_not_greater_than_more_ts = gen_greater_than_test_try_new_try_new_ts(&eq_not_greater_than, &more_ts, &more_ts);
                    quote! {
                        #greater_than_less_ts,
                        #greater_than_zero_ts,
                        #greater_than_more_ts,
                        #not_greater_than_less_ts,
                        #not_greater_than_zero_ts,
                        #not_greater_than_more_ts,
                        #eq_not_greater_than_less_ts,
                        #eq_not_greater_than_zero_ts,
                        #eq_not_greater_than_more_ts
                    }
                };
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        IsNl::False => {
                            let wrap_into_not_empty_unq_vec_ts = |ts: &dyn ToTokens| quote! {Some(
                                #import::NotEmptyUnqVec::try_new(vec![#ts]).expect("3ad4b6bf")
                            )};
                            let sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts = &gen_ident_ts(
                                &PgType::SqlxTypesChronoNaiveTimeAsTime,
                                &IsNl::False,
                                &PgTypePattern::Stdrt
                            );
                            let sqlx_types_chrono_naive_date_as_date_stdrt_nn_ts = &gen_ident_ts(
                                &PgType::SqlxTypesChronoNaiveDateAsDate,
                                &IsNl::False,
                                &PgTypePattern::Stdrt
                            );
                            match &pg_type {
                                PgType::I16AsInt2 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#I16::MIN},
                                    &quote!{#I16::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#I16::MAX},
                                    &quote!{#I16::MAX - 1}
                                )),
                                PgType::I32AsInt4 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#I32::MIN},
                                    &quote!{#I32::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#I32::MAX},
                                    &quote!{#I32::MAX - 1}
                                )),
                                PgType::I64AsInt8 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#I64::MIN},
                                    &quote!{#I64::MIN + 1},
                                    &quote!{0},
                                    &quote!{1},
                                    &quote!{#I64::MAX},
                                    &quote!{#I64::MAX - 1}
                                )),
                                PgType::F32AsFloat4 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote!{#F32::MIN},
                                    &quote!{#F32::MIN.next_up()},
                                    &quote!{0.0},
                                    &quote!{1.0},
                                    &quote!{#F32::MAX},
                                    &quote!{#F32::MAX.next_down()}
                                )),
                                PgType::F64AsFloat8 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                //todo rust f64 != pg float8
                                    &quote!{-2.0},
                                    &quote!{-2.0 + 1.0},
                                    &quote!{0.0},
                                    &quote!{1.0},
                                    &quote!{2.0},
                                    &quote!{2.0 - 1.0}
                                )),
                                PgType::SqlxTypesChronoNaiveTimeAsTime => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{Self::min_inn_type()},
                                    &quote!{Self::slightly_more_than_min_inn_type()},
                                    &quote!{Self::middle_inn_type()},
                                    &quote!{Self::slightly_more_than_middle_inn_type()},
                                    &quote!{Self::max_inn_type()},
                                    &quote!{Self::slightly_less_than_max_inn_type()},
                                )),
                                PgType::SqlxTypesTimeTimeAsTime => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{Self::min_inn_type()},
                                    &quote!{Self::slightly_more_than_min_inn_type()},
                                    &quote!{Self::middle_inn_type()},
                                    &quote!{Self::slightly_more_than_middle_inn_type()},
                                    &quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_999).expect("f3d895bb")},
                                    &quote!{sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_998).expect("1e71f8c6")},
                                )),
                                PgType::SqlxTypesChronoNaiveDateAsDate => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 30)?},//todo not sure about this values. mb reuse
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 31)?},
                                    &quote!{Self::middle_inn_type()},
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 2)?},
                                    &quote!{Self::max_inn_type()},
                                    &quote!{sqlx::types::chrono::NaiveDate::from_ymd_opt(262_142, 12, 30)?},
                                )),
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::min_inn_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::slightly_more_than_min_inn_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_stdrt_nn_ts::middle_inn_type(),
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::min_inn_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_stdrt_nn_ts::middle_inn_type(),
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::slightly_more_than_min_inn_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::max_inn_type()
                                    )},
                                    &quote!{sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::slightly_less_than_max_inn_type()
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
                        IsNl::True => quote! {
                            <#ident_stdrt_nn_ucc as #import::PgTypeTestCases>::pg_type_opt_vec_wh_greater_than_test().map(
                                |el_e4af7fd9|
                                #import::NotEmptyUnqVec::try_new(
                                    el_e4af7fd9
                                    .into_vec()
                                    .into_iter()
                                    .map(|el_504739e6| #import::PgTypeGreaterThanTest {
                                        vrt: el_504739e6.vrt,
                                        cr: #ident_cr_ucc(#ident_orgn_ucc(Some(el_504739e6.cr.0))),
                                        greater_than: #ident_tt_ucc(#ident_orgn_ucc(Some(el_504739e6.greater_than.0))),
                                    })
                                    .collect()
                                ).expect("63ce5df3")
                            )
                        },
                    },
                    PgTypePattern::ArrDim1 { .. } => none_ts.clone(),
                }
            };
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts = match &pg_type_pattern {
                PgTypePattern::Stdrt => {
                    enum IsNeedToImplPgTypeGreaterThanTest {
                        False,
                        TrueFromCr,
                        TrueFromRdIds,
                    }
                    enum CrRdIds {
                        Cr,
                        RdIds,
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
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => IsNeedToImplPgTypeGreaterThanTest::TrueFromCr,
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg => IsNeedToImplPgTypeGreaterThanTest::TrueFromRdIds,
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
                    let gen_some_ts = |v_476d047b: &CrRdIds| match &is_nl {
                        IsNl::False => {
                            let ts = match &v_476d047b {
                                CrRdIds::RdIds => quote! {#ident_stdrt_nn_tt_ucc(#RdIdsSc.0.0)},
                                CrRdIds::Cr => quote! {tt},
                            };
                            quote! {Some(#ident_wh_ucc::GreaterThan(
                                wh_flts::PgTypeWhGreaterThan {
                                    oprtr: greater_than_vrt.oprtr(),
                                    #VSc: #ts,
                                }
                            ))}
                        }
                        IsNl::True => {
                            let ts = match &v_476d047b {
                                CrRdIds::RdIds => quote! {#RdIdsSc.0},
                                CrRdIds::Cr => quote! {#TtSc.0.0},
                            };
                            quote! {
                                #ts.map(|el_886032ca| #ident_wh_ucc::GreaterThan(wh_flts::PgTypeWhGreaterThan {
                                    oprtr: greater_than_vrt.oprtr(),
                                    #VSc: #ident_stdrt_nn_tt_ucc(el_886032ca),
                                }))
                            }
                        }
                    };
                    match &is_need_to_impl_greater_than_test {
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromRdIds => gen_some_ts(&CrRdIds::RdIds),
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromCr => gen_some_ts(&CrRdIds::Cr),
                        IsNeedToImplPgTypeGreaterThanTest::False => none_ts.clone(),
                    }
                }
                PgTypePattern::ArrDim1 { .. } => none_ts.clone(),
            };
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts = none_ts.clone();
            let cr_into_pg_json_opt_vec_wh_len_eq_ts = none_ts.clone();
            let cr_into_pg_json_opt_vec_wh_len_greater_than_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts = none_ts.clone();
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts = none_ts;
            gen_impl_pg_type_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &import,
                &ident_inn_type_ts,
                &ident,
                &opt_vec_cr_ts,
                &rd_ids_to_2_dims_vec_rd_inn_ts,
                &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
                &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
                &upd_to_rd_ids_ts,
                &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
                &previous_rd_and_opt_upd_into_rd_ts,
                &rd_ids_and_cr_into_rd_ts,
                &rd_ids_and_cr_into_opt_v_rd_ts,
                &rd_ids_and_cr_into_tt_ts,
                &rd_ids_and_cr_into_wh_eq_ts,
                &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
                &rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts,
                &cr_into_pg_type_opt_vec_wh_dim_one_eq_ts,
                &pg_type_opt_vec_wh_greater_than_test_ts,
                &rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts,
                &cr_into_pg_json_opt_vec_wh_len_eq_ts,
                &cr_into_pg_json_opt_vec_wh_len_greater_than_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts,
            )
        };
        let mb_impl_pg_type_pk_for_ident_stdrt_nn_if_can_be_pk_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
            let v_as_rd_ids_ts = quote! {#VSc: #self_as_pg_type_ts::#RdIdsUcc};
            quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #import::#PgTypePkUcc for #ident_stdrt_nn_ucc {
                    type #PgTypeUcc = Self;
                    type #TtUcc = #ident_stdrt_nn_tt_ucc;
                    fn #RdIdsIntoTtSc(#v_as_rd_ids_ts) -> #self_as_pg_type_ts::#TtUcc {
                        #ident_tt_ucc(#VSc.0.0)
                    }
                    fn #RdIdsIntoRdSc(#v_as_rd_ids_ts) -> #self_as_pg_type_ts::#RdUcc {
                        #VSc.0
                    }
                    fn #RdIdsIntoUpdSc(#v_as_rd_ids_ts) -> #self_as_pg_type_ts::#UpdUcc {
                        #ident_upd_ucc(#VSc.0.0)
                    }
                    fn #RdIntoTtSc(
                        #VSc: #self_as_pg_type_ts::#RdUcc
                    ) -> #self_as_pg_type_ts::#TtUcc {
                        #ident_tt_ucc(#VSc.0)
                    }
                }
            }
        } else {
            Ts2::new()
        };
        let mb_impl_pg_type_not_pk_for_ident_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
            Ts2::new()
        } else {
            gen_impl_pg_type_not_pk_for_ident_ts(&import, &ident)
        };
        let generated = quote! {
            #ident_ts
            #ident_orgn_ts
            #ident_tt_ts
            #ident_cr_ts
            #ident_sel_ts
            #ident_wh_ts
            #ident_rd_ts
            #ident_rd_ids_ts
            #ident_rd_inn_ts
            #ident_upd_ts
            #ident_upd_for_query_ts
            #impl_pg_type_for_ident_ts
            #impl_pg_type_test_cases_for_ident_ts
            #mb_impl_pg_type_pk_for_ident_stdrt_nn_if_can_be_pk_ts
            #mb_impl_pg_type_not_pk_for_ident_ts
        };
        (
            {
                let fi = format!("col_{i}").parse::<Ts2>().expect("2e15af68");
                quote! {
                    pub #fi: pg_crud::pg_type:: #ident,
                }
                .to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    mb_write_ts_into_file(
        gen_pg_json_config.pg_tbl_cols_write_into_file,
        "pg_tbl_cols_using_pg_types",
        &{
            let ts = cols_ts
                .into_iter()
                .map(|el_2e3fc869| el_2e3fc869.parse::<Ts2>().expect("79ee6381"))
                .collect::<Vec<Ts2>>();
            quote! {
                struct PgTblColsUsingPgTypes {
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
    mb_write_ts_into_file(
        gen_pg_json_config.whole_write_into_file,
        "gen_pg_types",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
