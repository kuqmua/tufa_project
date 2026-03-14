use enum_extension_lib::EnumExtension;
use gen_quotes::dq_ts;
use macros_helpers::{
    DCopy, DSchemarsJsonSchema, DTsBuilder, FormatWithCargofmt, ShouldWriteTsIntoFile,
    gen_impl_display_ts, gen_impl_from_ts, gen_impl_to_err_string_ts, gen_pub_const_new_ts,
    gen_pub_new_ts, mb_write_ts_into_file,
};
use naming::{
    ArrOfUcc, AsUcc, BooleanUcc, ColFieldSc, CrForQueryUcc, CrSc, EqUcc, ErSc, GenPgJsonModSc,
    IncrSc, JsonbSetAccumulatorSc, NbrUcc, NewSc, OptUpdSc, OptVecCrSc, PgJsonUcc, QuerySc,
    RdIdsAndCrIntoRdSc, RdIdsAndCrIntoVecWhEqUsingFieldsSc, RdIdsAndCrIntoWhEqSc, RdIdsSc,
    RdIdsTo2DimsVecRdInnSc, RdInnUcc, RdSc, SelfSc, SelfUcc, StringUcc, UpdForQueryUcc, UpdUcc,
    VSc, VecOfUcc,
    prm::{
        JsonbSelfUcc, SelfCrForQueryUcc, SelfCrUcc, SelfOrgnUcc, SelfRdIdsUcc, SelfRdInnUcc,
        SelfRdUcc, SelfSelUcc, SelfTtUcc, SelfUpdForQueryUcc, SelfUpdUcc, SelfWhUcc,
    },
};
use optml::Optml;
use panic_loc::panic_loc;
use pg_crud_macros_cmn::{
    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, Dim, DimIndexNbr, Import, IsNl, IsQbMut,
    IsSelOnlyCrdIdsQbMut, IsSelOnlyUpddIdsQbMut, IsSelQpColFieldForErMsgUsed, IsSelQpIsPgTypeUsed,
    IsSelQpSelfSelUsed, IsStdrtNn, IsUpdQbMut, IsUpdQpJsonbSetTargetUsed, IsUpdQpSelfUpdUsed,
    PgFlt, PgJsonFlt, RdOrUpd, ShouldDSchemarsJsonSchema, ShouldDeriveUtoipaToSchema,
    gen_impl_crate_is_string_empty_for_ident_ts,
    gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts,
    gen_impl_pg_crud_cmn_dflt_some_one_el_ts, gen_impl_pg_json_test_cases_for_ident_ts,
    gen_impl_pg_json_ts, gen_impl_sqlx_encode_sqlx_pg_for_ident_ts,
    gen_impl_sqlx_type_for_ident_ts, gen_opt_type_dcl_ts, gen_pg_type_wh_ts,
    gen_sqlx_types_json_type_dcl_ts, gen_v_dcl_ts, gen_v_init_ts, gen_vec_tokens_dcl_ts,
};
use pg_crud_macros_cmn::{gen_jsonb_build_obj, gen_jsonb_build_obj_v};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as StdFmtResult},
    iter::once,
};
use strum_macros::{Display, EnumIter};
use token_patterns::{
    AllowClippyArbitrarySrcItemOrdering, Bool, F32, F64, I8, I16, I32, I64, MustUse, NoneTs,
    PgCrudCmnDfltSomeOneEl, PgCrudCmnDfltSomeOneElCall, PgCrudCmnDfltSomeOneElMaxPageSizeCall,
    StringTs, U8, U16, U32, U64, UuidUuid,
};
#[must_use]
pub fn gen_pg_json(input_ts: &Ts2) -> Ts2 {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Display, Optml)]
    enum RustTypeName {
        I8,
        I16,
        I32,
        I64,
        U8,
        U16,
        U32,
        U64,
        F32,
        F64,
        Bool,
        String,
        UuidUuid,
    }
    impl From<&PgJson> for RustTypeName {
        fn from(v: &PgJson) -> Self {
            match &v {
                PgJson::I8AsJsonbNbr => Self::I8,
                PgJson::I16AsJsonbNbr => Self::I16,
                PgJson::I32AsJsonbNbr => Self::I32,
                PgJson::I64AsJsonbNbr => Self::I64,
                PgJson::U8AsJsonbNbr => Self::U8,
                PgJson::U16AsJsonbNbr => Self::U16,
                PgJson::U32AsJsonbNbr => Self::U32,
                PgJson::U64AsJsonbNbr => Self::U64,
                PgJson::F32AsJsonbNbr => Self::F32,
                PgJson::F64AsJsonbNbr => Self::F64,
                PgJson::BoolAsJsonbBoolean => Self::Bool,
                PgJson::StringAsJsonbString => Self::String,
                PgJson::UuidUuidAsJsonbString => Self::UuidUuid,
            }
        }
    }
    #[derive(Debug, Optml)]
    enum PgJsonName {
        Boolean,
        Nbr,
        String,
    }
    impl Display for PgJsonName {
        fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
            write!(
                f,
                "{}",
                JsonbSelfUcc::from_display(match &self {
                    Self::Nbr => &NbrUcc,
                    Self::Boolean => &BooleanUcc,
                    Self::String => &StringUcc,
                })
            )
        }
    }
    impl From<&PgJson> for PgJsonName {
        fn from(v: &PgJson) -> Self {
            match &v {
                PgJson::I8AsJsonbNbr
                | PgJson::I16AsJsonbNbr
                | PgJson::I32AsJsonbNbr
                | PgJson::I64AsJsonbNbr
                | PgJson::U8AsJsonbNbr
                | PgJson::U16AsJsonbNbr
                | PgJson::U32AsJsonbNbr
                | PgJson::U64AsJsonbNbr
                | PgJson::F32AsJsonbNbr
                | PgJson::F64AsJsonbNbr => Self::Nbr,
                PgJson::BoolAsJsonbBoolean => Self::Boolean,
                PgJson::StringAsJsonbString | PgJson::UuidUuidAsJsonbString => Self::String,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        Display,
        EnumIter,
        EnumExtension,
        Optml,
    )]
    enum PgJson {
        I8AsJsonbNbr,
        I16AsJsonbNbr,
        I32AsJsonbNbr,
        I64AsJsonbNbr,
        U8AsJsonbNbr,
        U16AsJsonbNbr,
        U32AsJsonbNbr,
        U64AsJsonbNbr,
        F32AsJsonbNbr,
        F64AsJsonbNbr,
        BoolAsJsonbBoolean,
        StringAsJsonbString,
        UuidUuidAsJsonbString,
    }
    impl ToTokens for PgJson {
        fn to_tokens(&self, tokens: &mut Ts2) {
            self.to_string()
                .parse::<Ts2>()
                .expect("eb6cafe0")
                .to_tokens(tokens);
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        Display,
        EnumIter,
        EnumExtension,
        Optml,
    )]
    enum Pattern {
        Stdrt,
        ArrDim1 {
            dim1_is_nl: IsNl,
        },
        ArrDim2 {
            dim1_is_nl: IsNl,
            dim2_is_nl: IsNl,
        },
        ArrDim3 {
            dim1_is_nl: IsNl,
            dim2_is_nl: IsNl,
            dim3_is_nl: IsNl,
        },
        ArrDim4 {
            dim1_is_nl: IsNl,
            dim2_is_nl: IsNl,
            dim3_is_nl: IsNl,
            dim4_is_nl: IsNl,
        },
    }
    impl Pattern {
        const fn down_by_1(&self) -> Option<Self> {
            match &self {
                Self::Stdrt => None,
                Self::ArrDim1 { .. } => Some(Self::Stdrt),
                Self::ArrDim2 { dim2_is_nl, .. } => Some(Self::ArrDim1 {
                    dim1_is_nl: *dim2_is_nl,
                }),
                Self::ArrDim3 {
                    dim2_is_nl,
                    dim3_is_nl,
                    ..
                } => Some(Self::ArrDim2 {
                    dim1_is_nl: *dim2_is_nl,
                    dim2_is_nl: *dim3_is_nl,
                }),
                Self::ArrDim4 {
                    dim2_is_nl,
                    dim3_is_nl,
                    dim4_is_nl,
                    ..
                } => Some(Self::ArrDim3 {
                    dim1_is_nl: *dim2_is_nl,
                    dim2_is_nl: *dim3_is_nl,
                    dim3_is_nl: *dim4_is_nl,
                }),
            }
        }
        const fn down_by_2(&self) -> Option<Self> {
            match &self {
                Self::Stdrt | Self::ArrDim1 { .. } => None,
                Self::ArrDim2 { .. } => Some(Self::Stdrt),
                Self::ArrDim3 { dim3_is_nl, .. } => Some(Self::ArrDim1 {
                    dim1_is_nl: *dim3_is_nl,
                }),
                Self::ArrDim4 {
                    dim3_is_nl,
                    dim4_is_nl,
                    ..
                } => Some(Self::ArrDim2 {
                    dim1_is_nl: *dim3_is_nl,
                    dim2_is_nl: *dim4_is_nl,
                }),
            }
        }
        const fn down_by_3(&self) -> Option<Self> {
            match &self {
                Self::Stdrt | Self::ArrDim1 { .. } | Self::ArrDim2 { .. } => None,
                Self::ArrDim3 { .. } => Some(Self::Stdrt),
                Self::ArrDim4 { dim4_is_nl, .. } => Some(Self::ArrDim1 {
                    dim1_is_nl: *dim4_is_nl,
                }),
            }
        }
        const fn down_by_4(&self) -> Option<Self> {
            match &self {
                Self::Stdrt
                | Self::ArrDim1 { .. }
                | Self::ArrDim2 { .. }
                | Self::ArrDim3 { .. } => None,
                Self::ArrDim4 { .. } => Some(Self::Stdrt),
            }
        }
    }
    enum ArrDim {
        ArrDim1,
        ArrDim2 {
            dim1_is_nl: IsNl,
        },
        ArrDim3 {
            dim1_is_nl: IsNl,
            dim2_is_nl: IsNl,
        },
        ArrDim4 {
            dim1_is_nl: IsNl,
            dim2_is_nl: IsNl,
            dim3_is_nl: IsNl,
        },
    }
    impl ArrDim {
        const fn to_usize(&self) -> usize {
            match &self {
                Self::ArrDim1 { .. } => 1,
                Self::ArrDim2 { .. } => 2,
                Self::ArrDim3 { .. } => 3,
                Self::ArrDim4 { .. } => 4,
            }
        }
    }
    impl TryFrom<&Pattern> for ArrDim {
        type Error = ();
        fn try_from(v: &Pattern) -> Result<Self, Self::Error> {
            match &v {
                Pattern::Stdrt => Err(()),
                Pattern::ArrDim1 { .. } => Ok(Self::ArrDim1),
                Pattern::ArrDim2 { dim1_is_nl, .. } => Ok(Self::ArrDim2 {
                    dim1_is_nl: *dim1_is_nl,
                }),
                Pattern::ArrDim3 {
                    dim1_is_nl,
                    dim2_is_nl,
                    ..
                } => Ok(Self::ArrDim3 {
                    dim1_is_nl: *dim1_is_nl,
                    dim2_is_nl: *dim2_is_nl,
                }),
                Pattern::ArrDim4 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                    ..
                } => Ok(Self::ArrDim4 {
                    dim1_is_nl: *dim1_is_nl,
                    dim2_is_nl: *dim2_is_nl,
                    dim3_is_nl: *dim3_is_nl,
                }),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Optml)]
    struct Record {
        pg_json: PgJson,
        is_nl: IsNl,
        pattern: Pattern,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize, Optml)]
    enum ConfigVrt {
        All,
        WithoutDims,
        WithDimOne,
        WithDimTwo,
        WithDimThree,
        WithDimFour,
        Concrete(Vec<Record>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize, Optml)]
    struct GenPgJsonsConfig {
        vrt: ConfigVrt,
        pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json: ShouldWriteTsIntoFile,
        whole_cnt_write_into_gen_pg_json: ShouldWriteTsIntoFile,
    }
    panic_loc();
    let config = from_str::<GenPgJsonsConfig>(&input_ts.to_string()).expect("1123f78f");
    let (fields_ts, pg_json_arr) = {
        let acc = {
            let gen_vrts = |max_dim: Option<i32>|{
                PgJson::into_arr().into_iter().fold(Vec::new(), |mut acc, pg_json| {
                    for pattern in Pattern::into_arr() {
                        if match max_dim {
                            None => true,
                            Some(0i32) => matches!(pattern, Pattern::Stdrt),
                            Some(v_b6fece15) => {
                                let dim = match pattern {
                                    Pattern::Stdrt => 0i32,
                                    Pattern::ArrDim1 { .. } => 1i32,
                                    Pattern::ArrDim2 { .. } => 2i32,
                                    Pattern::ArrDim3 { .. } => 3i32,
                                    Pattern::ArrDim4 { .. } => 4i32,
                                };
                                dim <= v_b6fece15
                            }
                        } {
                            match pattern {
                                Pattern::Stdrt => {
                                    for is_nl in IsNl::into_arr() {
                                        acc.push(Record {
                                            pg_json: pg_json.clone(),
                                            is_nl,
                                            pattern: Pattern::Stdrt,
                                        });
                                    }
                                }
                                Pattern::ArrDim1 { .. } => {
                                    for is_nl in IsNl::into_arr() {
                                        for dim1_is_nl in IsNl::into_arr() {
                                            acc.push(Record {
                                                pg_json: pg_json.clone(),
                                                is_nl,
                                                pattern: Pattern::ArrDim1 {
                                                    dim1_is_nl,
                                                },
                                            });
                                        }
                                    }
                                }
                                Pattern::ArrDim2 { .. } => {
                                    for is_nl in IsNl::into_arr() {
                                        for dim1_is_nl in IsNl::into_arr() {
                                            for dim2_is_nl in IsNl::into_arr() {
                                                acc.push(Record {
                                                    pg_json: pg_json.clone(),
                                                    is_nl,
                                                    pattern: Pattern::ArrDim2 {
                                                        dim1_is_nl,
                                                        dim2_is_nl,
                                                    },
                                                });
                                            }
                                        }
                                    }
                                }
                                Pattern::ArrDim3 { .. } => {
                                    for is_nl in IsNl::into_arr() {
                                        for dim1_is_nl in IsNl::into_arr() {
                                            for dim2_is_nl in IsNl::into_arr() {
                                                for dim3_is_nl in IsNl::into_arr() {
                                                    acc.push(Record {
                                                        pg_json: pg_json.clone(),
                                                        is_nl,
                                                        pattern: Pattern::ArrDim3 {
                                                            dim1_is_nl,
                                                            dim2_is_nl,
                                                            dim3_is_nl,
                                                        },
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                Pattern::ArrDim4 { .. } => {
                                    for is_nl in IsNl::into_arr() {
                                        for dim1_is_nl in IsNl::into_arr() {
                                            for dim2_is_nl in IsNl::into_arr() {
                                                for dim3_is_nl in IsNl::into_arr() {
                                                    for dim4_is_nl in IsNl::into_arr() {
                                                        acc.push(Record {
                                                            pg_json: pg_json.clone(),
                                                            is_nl,
                                                            pattern: Pattern::ArrDim4 {
                                                                dim1_is_nl,
                                                                dim2_is_nl,
                                                                dim3_is_nl,
                                                                dim4_is_nl,
                                                            },
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    acc
                })
            };
            match config.vrt {
                ConfigVrt::All => gen_vrts(None),
                ConfigVrt::WithoutDims => gen_vrts(Some(0i32)),
                ConfigVrt::WithDimOne => gen_vrts(Some(1i32)),
                ConfigVrt::WithDimTwo => gen_vrts(Some(2i32)),
                ConfigVrt::WithDimThree => gen_vrts(Some(3i32)),
                ConfigVrt::WithDimFour => gen_vrts(Some(4i32)),
                ConfigVrt::Concrete(v) => v,
            }
        };
        let mut seen = HashSet::new();
        assert!(
            acc
                .iter()
                .all(|el| seen.insert(el)),
            "c2d37017"
        );
        acc
    }.into_iter().fold(Vec::new(), |mut acc, el| {
        for el0 in {
            #[derive(Clone, Optml)]
            struct RecordH {
                is_nl: IsNl,
                pattern: Pattern,
            }
            fn gen_record_h_vec(record_h: RecordH) -> Vec<RecordH> {
                let gen_vec = |record_h_1e4b61e4: RecordH| gen_record_h_vec(
                    record_h_1e4b61e4
                ).into_iter().chain(once(record_h.clone())).collect();
                match (&record_h.is_nl, &record_h.pattern) {
                    (IsNl::False, Pattern::Stdrt) => vec![record_h],
                    (IsNl::True, Pattern::Stdrt) => gen_vec(RecordH {
                        is_nl: IsNl::False,
                        pattern: Pattern::Stdrt,
                    }),
                    (IsNl::False, Pattern::ArrDim1 { dim1_is_nl }) => gen_vec(RecordH {
                        is_nl: *dim1_is_nl,
                        pattern: record_h.pattern.down_by_1().expect("0e970a4f"),
                    }),
                    (IsNl::False, Pattern::ArrDim2 { dim1_is_nl, .. }) => gen_vec(RecordH {
                        is_nl: *dim1_is_nl,
                        pattern: record_h.pattern.down_by_1().expect("85f8ed83"),
                    }),
                    (
                        IsNl::False,
                        Pattern::ArrDim3 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                        },
                    ) => gen_vec(RecordH {
                        is_nl: *dim1_is_nl,
                        pattern: Pattern::ArrDim2 {
                            dim1_is_nl: *dim2_is_nl,
                            dim2_is_nl: *dim3_is_nl,
                        },
                    }),
                    (
                        IsNl::False,
                        Pattern::ArrDim4 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                            dim4_is_nl,
                        },
                    ) => gen_vec(RecordH {
                        is_nl: *dim1_is_nl,
                        pattern: Pattern::ArrDim3 {
                            dim1_is_nl: *dim2_is_nl,
                            dim2_is_nl: *dim3_is_nl,
                            dim3_is_nl: *dim4_is_nl,
                        },
                    }),
                    (IsNl::True, Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. }) => gen_vec(RecordH {
                        is_nl: IsNl::False,
                        pattern: record_h.pattern.clone(),
                    }),
                }
            }
            gen_record_h_vec(RecordH {
                is_nl: el.is_nl,
                pattern: el.pattern,
            })
        } {
            let record = Record {
                pg_json: el.pg_json.clone(),
                is_nl: el0.is_nl,
                pattern: el0.pattern,
            };
            if !acc.contains(&record) {
                acc.push(record);
            }
        }
        acc
    })
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, Record)>>()
    .par_iter()
    // .into_iter() //just for console prints ordering
    .map(|(i, el)| {
        enum IsStdrtNnUuid {
            False,
            True,
        }
        enum ConstFn {
            False,
            True,
        }
        let pg_json = &el.pg_json;
        let is_nl = &el.is_nl;
        let pattern = &el.pattern;
        let rust_type_name = RustTypeName::from(pg_json);
        let pg_json_name = PgJsonName::from(pg_json);
        let is_stdrt_nn = if matches!((&pattern, &is_nl), (Pattern::Stdrt, IsNl::False)) {
            IsStdrtNn::True
        } else {
            IsStdrtNn::False
        };
        let is_stdrt_nn_uuid = if matches!((&is_nl, &pattern, &pg_json), (IsNl::False, Pattern::Stdrt, PgJson::UuidUuidAsJsonbString)) {
            IsStdrtNnUuid::True
        } else {
            IsStdrtNnUuid::False
        };
        let import = Import::PgCrudCmn;
        let gen_v_init_ts0 = |ts: &dyn ToTokens| gen_v_init_ts(&import, &ts);
        let gen_ident_ts = |is_nl_ddf79d44: &IsNl, pattern_2c09ee59: &Pattern| {
            let (rust_part, pg_part) = match &pattern_2c09ee59 {
                Pattern::Stdrt => (rust_type_name.to_string(), pg_json_name.to_string()),
                Pattern::ArrDim1 { dim1_is_nl } => {
                    let d1 = dim1_is_nl.nn_or_nl_str();
                    let d1_rust = dim1_is_nl.rust();
                    (format!("{VecOfUcc}{d1_rust}{rust_type_name}"), format!("{ArrOfUcc}{d1}{pg_json_name}"))
                }
                Pattern::ArrDim2 { dim1_is_nl, dim2_is_nl } => {
                    let d1 = dim1_is_nl.nn_or_nl_str();
                    let d1_rust = dim1_is_nl.rust();
                    let d2 = dim2_is_nl.nn_or_nl_str();
                    let d2_rust = dim2_is_nl.rust();
                    (format!("{VecOfUcc}{d1_rust}{VecOfUcc}{d2_rust}{rust_type_name}"), format!("{ArrOfUcc}{d1}{ArrOfUcc}{d2}{pg_json_name}"))
                }
                Pattern::ArrDim3 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                } => {
                    let d1 = dim1_is_nl.nn_or_nl_str();
                    let d1_rust = dim1_is_nl.rust();
                    let d2 = dim2_is_nl.nn_or_nl_str();
                    let d2_rust = dim2_is_nl.rust();
                    let d3 = dim3_is_nl.nn_or_nl_str();
                    let d3_rust = dim3_is_nl.rust();
                    (
                        format!("{VecOfUcc}{d1_rust}{VecOfUcc}{d2_rust}{VecOfUcc}{d3_rust}{rust_type_name}"),
                        format!("{ArrOfUcc}{d1}{ArrOfUcc}{d2}{ArrOfUcc}{d3}{pg_json_name}"),
                    )
                }
                Pattern::ArrDim4 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                    dim4_is_nl,
                } => {
                    let d1 = dim1_is_nl.nn_or_nl_str();
                    let d1_rust = dim1_is_nl.rust();
                    let d2 = dim2_is_nl.nn_or_nl_str();
                    let d2_rust = dim2_is_nl.rust();
                    let d3 = dim3_is_nl.nn_or_nl_str();
                    let d3_rust = dim3_is_nl.rust();
                    (
                        format!("{VecOfUcc}{d1_rust}{VecOfUcc}{d2_rust}{VecOfUcc}{d3_rust}{VecOfUcc}{}{rust_type_name}", dim4_is_nl.rust()),
                        format!("{ArrOfUcc}{d1}{ArrOfUcc}{d2}{ArrOfUcc}{d3}{ArrOfUcc}{}{pg_json_name}", dim4_is_nl.nn_or_nl_str()),
                    )
                }
            };
            let nn_or_nl_str = is_nl_ddf79d44.nn_or_nl_str();
            format!("{}{rust_part}{AsUcc}{nn_or_nl_str}{pg_part}", is_nl_ddf79d44.rust()).parse::<Ts2>().expect("998d1471")
        };
        let ident = &gen_ident_ts(is_nl, pattern);
        let ident_stdrt_nn_ucc = &gen_ident_ts(&IsNl::False, &Pattern::Stdrt);
        let ident_stdrt_nn_tt_ucc = SelfTtUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_tt_ucc = SelfTtUcc::from_tokens(&ident);
        let ident_cr_ucc = SelfCrUcc::from_tokens(&ident);
        let ident_wh_ucc = SelfWhUcc::from_tokens(&ident);
        let ident_rd_ids_ucc = SelfRdIdsUcc::from_tokens(&ident);
        let ident_nn_ts = gen_ident_ts(&IsNl::False, pattern);
        let ident_ts = {
            let ident_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy()
                .build_struct(
                    &Ts2::new(),
                    &ident,
                    &Ts2::new(),
                    &quote!{;}
                );
            quote! {
                #ident_ts
            }
        };
        let ident_stdrt_nn_orgn_ucc = SelfOrgnUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_orgn_ucc = SelfOrgnUcc::from_tokens(&ident);
        let ident_rd_inn_stdrt_nn_al_ts = {
            let content_ts: &dyn ToTokens = match &pg_json {
                PgJson::I8AsJsonbNbr => &I8,
                PgJson::I16AsJsonbNbr => &I16,
                PgJson::I32AsJsonbNbr => &I32,
                PgJson::I64AsJsonbNbr => &I64,
                PgJson::U8AsJsonbNbr => &U8,
                PgJson::U16AsJsonbNbr => &U16,
                PgJson::U32AsJsonbNbr => &U32,
                PgJson::U64AsJsonbNbr => &U64,
                PgJson::F32AsJsonbNbr => &F32,
                PgJson::F64AsJsonbNbr => &F64,
                PgJson::BoolAsJsonbBoolean => &Bool,
                PgJson::StringAsJsonbString => &StringTs,
                PgJson::UuidUuidAsJsonbString => &UuidUuid,
            };
            quote! {#content_ts}
        };
        let ident_rd_inn_ucc = SelfRdInnUcc::from_tokens(&ident);
        let v_ident_rd_inn_ts = quote! {#VSc: #ident_rd_inn_ucc};
        let gen_pub_fn_new_v_ident_rd_inn_cnt_ts = |ts: &dyn ToTokens| gen_pub_new_ts(
            &MustUse,
            &v_ident_rd_inn_ts,
            &ts
        );
        let gen_pub_const_fn_new_v_ident_rd_inn_cnt_ts = |ts: &dyn ToTokens| gen_pub_const_new_ts(
            &MustUse,
            &v_ident_rd_inn_ts,
            &ts
        );
        let self_ident_orgn_new_v_ts = quote! {Self(#ident_orgn_ucc::new(#VSc))};
        let mb_const_fn = match &pattern {
            Pattern::Stdrt => match &is_nl {
                IsNl::False => ConstFn::True,
                IsNl::True => ConstFn::False,
            },
            Pattern::ArrDim1 { .. } |
            Pattern::ArrDim2 { .. } |
            Pattern::ArrDim3 { .. } |
            Pattern::ArrDim4 { .. } => ConstFn::False,
        };
        let gen_pub_new_or_fn_new_ts = |const_new_ts: &dyn ToTokens, new_ts: &dyn ToTokens|match mb_const_fn {
            ConstFn::False => gen_pub_fn_new_v_ident_rd_inn_cnt_ts(
                &new_ts
            ),
            ConstFn::True => gen_pub_const_fn_new_v_ident_rd_inn_cnt_ts(
                &const_new_ts
            ),
        };
        let pub_new_or_const_new_self_ident_orgn_new_v_ts = gen_pub_new_or_fn_new_ts(
            &self_ident_orgn_new_v_ts,
            &self_ident_orgn_new_v_ts
        );
        let gen_impl_new_for_ucc_ts = |ucc: &dyn ToTokens| quote! {
            impl #ucc {
                #pub_new_or_const_new_self_ident_orgn_new_v_ts
            }
        };
        let gen_not_empty_unq_vec_try_new_match_ts = |
            prm_ts: &dyn ToTokens,
            ok_v_ts: &dyn ToTokens,
            ok_ts: &dyn ToTokens,
            is_empty_ts: &dyn ToTokens,
            not_unq_ts: &dyn ToTokens,
        |quote! {
            match #import::NotEmptyUnqVec::try_new(#prm_ts) {
                Ok(#ok_v_ts) => #ok_ts,
                Err(er) => match er {
                    #import::NotEmptyUnqVecTryNewEr::IsEmpty {..} => #is_empty_ts,
                    #import::NotEmptyUnqVecTryNewEr::NotUnq {..} => #not_unq_ts
                }
            }
        };
        let ident_cr_for_query_ucc = SelfCrForQueryUcc::from_tokens(&ident);
        let ident_upd_ucc = SelfUpdUcc::from_tokens(&ident);
        let ident_upd_for_query_ucc = SelfUpdForQueryUcc::from_tokens(&ident);
        let mb_derive_copy = match &pattern {
            Pattern::Stdrt => match &pg_json {
                PgJson::I8AsJsonbNbr |
                PgJson::I16AsJsonbNbr |
                PgJson::I32AsJsonbNbr |
                PgJson::I64AsJsonbNbr |
                PgJson::U8AsJsonbNbr |
                PgJson::U16AsJsonbNbr |
                PgJson::U32AsJsonbNbr |
                PgJson::U64AsJsonbNbr |
                PgJson::F32AsJsonbNbr |
                PgJson::F64AsJsonbNbr |
                PgJson::BoolAsJsonbBoolean |
                PgJson::UuidUuidAsJsonbString => DCopy::True,
                PgJson::StringAsJsonbString => DCopy::False,
            },
            Pattern::ArrDim1 {..} |
            Pattern::ArrDim2 {..} |
            Pattern::ArrDim3 {..} |
            Pattern::ArrDim4 {..} => DCopy::False,
        };
        let ident_orgn_ts = {
            let gen_ident_orgn_non_wrapping_6c0934a6 = |
                is_nl_e7d1d83c: &IsNl,
                pattern_1ca83c6c: &Pattern
            | SelfOrgnUcc::from_tokens(&gen_ident_ts(is_nl_e7d1d83c, pattern_1ca83c6c));
            let ident_orgn_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_partial_ord()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema_if(
                    if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                        match &pg_json {
                            PgJson::UuidUuidAsJsonbString => DSchemarsJsonSchema::False,
                            PgJson::I8AsJsonbNbr
                            | PgJson::I16AsJsonbNbr
                            | PgJson::I32AsJsonbNbr
                            | PgJson::I64AsJsonbNbr
                            | PgJson::U8AsJsonbNbr
                            | PgJson::U16AsJsonbNbr
                            | PgJson::U32AsJsonbNbr
                            | PgJson::U64AsJsonbNbr
                            | PgJson::F32AsJsonbNbr
                            | PgJson::F64AsJsonbNbr
                            | PgJson::BoolAsJsonbBoolean
                            | PgJson::StringAsJsonbString => DSchemarsJsonSchema::True,
                        }
                    } else {
                        DSchemarsJsonSchema::True
                    }
                )
                .build_struct(
                    &Ts2::new(),
                    &ident_orgn_ucc,
                    &Ts2::new(),
                    &{
                        let content_ts: &dyn ToTokens = {
                            let gen_ident_orgn_6f054930 = |is_nl_70fb22e6: &IsNl, pattern_042c1c1d: &Pattern| {
                                let v = gen_ident_orgn_non_wrapping_6c0934a6(is_nl_70fb22e6, pattern_042c1c1d);
                                match &is_nl {
                                    IsNl::False => gen_vec_tokens_dcl_ts(&v),
                                    IsNl::True => gen_opt_type_dcl_ts(&v),
                                }
                            };
                            let gen_dims_ts = |dim1_is_nl_a5c667cd: &IsNl|{
                                let (is_nl_79be16e9, pattern_3437adad): (&IsNl, &Pattern) = match &is_nl {
                                    IsNl::False => (dim1_is_nl_a5c667cd, &pattern.down_by_1().expect("e994797d")),
                                    IsNl::True => (&IsNl::False, pattern),
                                };
                                gen_ident_orgn_6f054930(is_nl_79be16e9, pattern_3437adad)
                            };
                            match &pattern {
                                Pattern::Stdrt => match &is_nl {
                                    IsNl::False => &ident_rd_inn_stdrt_nn_al_ts,
                                    IsNl::True => &gen_opt_type_dcl_ts(&ident_stdrt_nn_orgn_ucc),
                                },
                                Pattern::ArrDim1 { dim1_is_nl } |
                                Pattern::ArrDim2 { dim1_is_nl, .. } |
                                Pattern::ArrDim3 { dim1_is_nl, .. } |
                                Pattern::ArrDim4 { dim1_is_nl, .. } => &gen_dims_ts(dim1_is_nl),
                            }
                        };
                        quote!{(#content_ts);}
                    }
                );
            let ident_orgn_impl_new_self_cnt_ts = {
                let gen_v_map_type_new_ts = |ts: &dyn ToTokens| quote! {#VSc.map(#ts::#NewSc)};
                let gen_arr_dims_init_ts = |ts: &dyn ToTokens| match &is_nl {
                    IsNl::False => quote! {#VSc.into_iter().map(#ts::#NewSc).collect()},
                    IsNl::True => gen_v_map_type_new_ts(&ts),
                };
                match &pattern {
                    Pattern::Stdrt => match &is_nl {
                        IsNl::False => quote! {#VSc},
                        IsNl::True => gen_v_map_type_new_ts(&ident_stdrt_nn_orgn_ucc),
                    },
                    Pattern::ArrDim1 { dim1_is_nl, .. } |
                    Pattern::ArrDim2 { dim1_is_nl, .. } |
                    Pattern::ArrDim3 { dim1_is_nl, .. } |
                    Pattern::ArrDim4 { dim1_is_nl, .. } => gen_arr_dims_init_ts(&{
                        let (pattern_38178717, is_nl_b0d116f8): (&Pattern, &IsNl) = match &is_nl {
                            IsNl::False => (&pattern.down_by_1().expect("1160d3df"), dim1_is_nl),
                            IsNl::True => (pattern, &IsNl::False),
                        };
                        gen_ident_orgn_non_wrapping_6c0934a6(is_nl_b0d116f8, pattern_38178717)
                    }),
                }
            };
            let impl_ident_orgn_ts = {
                let pub_fn_new_ts = {
                    let self_ident_orgn_impl_new_self_cnt_ts = quote!{
                        Self(#ident_orgn_impl_new_self_cnt_ts)
                    };
                    gen_pub_new_or_fn_new_ts(
                        &self_ident_orgn_impl_new_self_cnt_ts,
                        &self_ident_orgn_impl_new_self_cnt_ts
                    )
                };
                quote! {
                    impl #ident_orgn_ucc {
                        #pub_fn_new_ts
                    }
                }
            };
            let impl_from_ident_cr_for_ident_orgn_ts = gen_impl_from_ts(&ident_cr_ucc, &ident_orgn_ucc, &quote! {#VSc.0});
            let impl_from_ident_upd_for_ident_orgn_ts = gen_impl_from_ts(&ident_upd_ucc, &ident_orgn_ucc, &quote! {#VSc.0});
            //todo
            let mb_impl_schemars_json_schema_for_ident_orgn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                match &pg_json {
                    PgJson::UuidUuidAsJsonbString => {
                        let ident_stdrt_nn_orgn_dq_ts = dq_ts(
                            &ident_stdrt_nn_orgn_ucc
                        );
                        let text_ident_stdrt_nn_orgn_dq_ts = dq_ts(
                            &format!("tests::{ident_stdrt_nn_orgn_ucc}")
                        );
                        quote!{
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #AllowClippyArbitrarySrcItemOrdering
                            const _: () = {
                                #[automatically_derived]
                                #[allow(unused_braces)]
                                impl schemars::JsonSchema for #ident_stdrt_nn_orgn_ucc {
                                    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#ident_stdrt_nn_orgn_dq_ts)
                                    }
                                    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#text_ident_stdrt_nn_orgn_dq_ts)
                                    }
                                    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                                        { generator.subschema_for::<String>() }
                                    }
                                    fn inline_schema() -> bool {
                                        false
                                    }
                                }
                            };
                        }
                    },
                    PgJson::I8AsJsonbNbr
                    | PgJson::I16AsJsonbNbr
                    | PgJson::I32AsJsonbNbr
                    | PgJson::I64AsJsonbNbr
                    | PgJson::U8AsJsonbNbr
                    | PgJson::U16AsJsonbNbr
                    | PgJson::U32AsJsonbNbr
                    | PgJson::U64AsJsonbNbr
                    | PgJson::F32AsJsonbNbr
                    | PgJson::F64AsJsonbNbr
                    | PgJson::BoolAsJsonbBoolean
                    | PgJson::StringAsJsonbString => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            let mb_impl_is_string_empty_for_ident_orgn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                match &pg_json {
                    PgJson::I8AsJsonbNbr
                    | PgJson::I16AsJsonbNbr
                    | PgJson::I32AsJsonbNbr
                    | PgJson::I64AsJsonbNbr
                    | PgJson::U8AsJsonbNbr
                    | PgJson::U16AsJsonbNbr
                    | PgJson::U32AsJsonbNbr
                    | PgJson::U64AsJsonbNbr
                    | PgJson::F32AsJsonbNbr
                    | PgJson::F64AsJsonbNbr
                    | PgJson::BoolAsJsonbBoolean => Ts2::new(),
                    PgJson::StringAsJsonbString => gen_impl_crate_is_string_empty_for_ident_ts(
                        &ident_orgn_ucc,
                        &quote!{self.0.clone().is_empty()}
                    ),
                    PgJson::UuidUuidAsJsonbString => gen_impl_crate_is_string_empty_for_ident_ts(
                        &ident_orgn_ucc,
                        &quote!{self.0.to_string().is_empty()}
                    ),
                }
            } else {
                Ts2::new()
            };
            let impl_display_for_ident_orgn_ts = gen_impl_display_ts(&Ts2::new(), &ident_orgn_ucc, &Ts2::new(), &quote! {write!(f, "{self:?}")});
            let impl_loc_lib_to_err_string_for_ident_orgn_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_orgn_ucc, &Ts2::new(), &quote! {format!("{self:#?}")});
            let some_dflt_some_one_el_call_ts = quote! {Some(#PgCrudCmnDfltSomeOneElCall)};
            let impl_dflt_some_one_el_for_ident_orgn_ts = gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_orgn_ucc, &{
                let content_ts = match &pattern {
                    Pattern::Stdrt => match &is_nl {
                        IsNl::False => match &pg_json {
                            PgJson::I8AsJsonbNbr
                            | PgJson::I16AsJsonbNbr
                            | PgJson::I32AsJsonbNbr
                            | PgJson::I64AsJsonbNbr
                            | PgJson::U8AsJsonbNbr
                            | PgJson::U16AsJsonbNbr
                            | PgJson::U32AsJsonbNbr
                            | PgJson::U64AsJsonbNbr
                            | PgJson::F32AsJsonbNbr
                            | PgJson::F64AsJsonbNbr
                            | PgJson::BoolAsJsonbBoolean => quote! {Default::default()},
                            PgJson::StringAsJsonbString => quote! {String::default()},
                            PgJson::UuidUuidAsJsonbString => quote! {uuid::Uuid::new_v4()},
                        },
                        IsNl::True => some_dflt_some_one_el_call_ts,
                    },
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => match &is_nl {
                        IsNl::False => quote! {vec![#PgCrudCmnDfltSomeOneElCall]},
                        IsNl::True => some_dflt_some_one_el_call_ts,
                    },
                };
                quote! {Self(#content_ts)}
            });
            let impl_sqlx_type_for_ident_orgn_ts = gen_impl_sqlx_type_for_ident_ts(&ident_orgn_ucc, &gen_sqlx_types_json_type_dcl_ts(&ident_rd_inn_ucc));
            let impl_sqlx_encode_sqlx_pg_for_ident_orgn_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_orgn_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            quote! {
                #ident_orgn_ts
                #impl_ident_orgn_ts
                #impl_from_ident_cr_for_ident_orgn_ts
                #impl_from_ident_upd_for_ident_orgn_ts
                #mb_impl_schemars_json_schema_for_ident_orgn_ts
                #mb_impl_is_string_empty_for_ident_orgn_ts
                #impl_display_for_ident_orgn_ts
                #impl_loc_lib_to_err_string_for_ident_orgn_ts
                #impl_dflt_some_one_el_for_ident_orgn_ts
                #impl_sqlx_type_for_ident_orgn_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_orgn_ts
            }
        };
        let ident_orgn_struct_cnt_ts = quote!{(#ident_orgn_ucc);};
        let ident_tt_ts = {
            let ident_tt_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_partial_ord()//mb add it to the trait?
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &Ts2::new(),
                    &ident_tt_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_ident_tt_ts = gen_impl_new_for_ucc_ts(&ident_tt_ucc);
            let impl_dflt_some_one_el_for_ident_tt_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_tt_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            //todo mb add to trait?
            let impl_sqlx_encode_sqlx_pg_for_ident_tt_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_tt_ucc, &quote! {&#SelfSc.0});
            //todo mb add to trait?
            let impl_sqlx_type_for_ident_tt_ts = gen_impl_sqlx_type_for_ident_ts(&ident_tt_ucc, &gen_sqlx_types_json_type_dcl_ts(&ident_rd_inn_ucc));
            quote! {
                #ident_tt_ts
                #impl_ident_tt_ts
                #impl_dflt_some_one_el_for_ident_tt_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_tt_ts
                #impl_sqlx_type_for_ident_tt_ts
            }
        };
        let ident_cr_ts = {
            let ident_cr_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &Ts2::new(),
                    &ident_cr_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_ident_cr_ts = gen_impl_new_for_ucc_ts(&ident_cr_ucc);
            let impl_dflt_some_one_el_for_ident_cr_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_cr_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            quote! {
                #ident_cr_ts
                #impl_ident_cr_ts
                #impl_dflt_some_one_el_for_ident_cr_ts
            }
        };
        let ident_cr_for_query_ts = {
            let ident_cr_for_query_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_cr_for_query_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_ident_cr_for_query_ts = gen_impl_new_for_ucc_ts(&ident_cr_for_query_ucc);
            let impl_sqlx_encode_sqlx_pg_for_ident_cr_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_cr_for_query_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            let impl_sqlx_type_for_ident_cr_for_query_ts = gen_impl_sqlx_type_for_ident_ts(&ident_cr_for_query_ucc, &ident_orgn_ucc);
            let impl_from_ident_cr_for_ident_cr_for_query_ts = gen_impl_from_ts(&ident_cr_ucc, &ident_cr_for_query_ucc, &quote! {Self(#VSc.0)});
            let mb_impl_from_ident_upd_for_ident_cr_for_query_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                gen_impl_from_ts(&ident_upd_ucc, &ident_cr_for_query_ucc, &quote! {Self(#VSc.0)})
            } else {
                Ts2::new()
            };
            quote! {
                #ident_cr_for_query_ts
                #impl_ident_cr_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_cr_for_query_ts
                #impl_sqlx_type_for_ident_cr_for_query_ts
                #impl_from_ident_cr_for_ident_cr_for_query_ts
                #mb_impl_from_ident_upd_for_ident_cr_for_query_ts
            }
        };
        let ident_sel_ucc = SelfSelUcc::from_tokens(&ident);
        let ident_sel_ts = {
            let ident_sel_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &Ts2::new(),
                    &ident_sel_ucc,
                    &Ts2::new(),
                    &ArrDim::try_from(pattern).map_or_else(
                        |()| quote! {;},
                        |arr_dim| {
                            let mut args_ts = Vec::new();
                            for el0 in 1..=arr_dim.to_usize() {
                                let dim_nbr_pgn_ts = format!("dim{el0}_pgn").parse::<Ts2>()
                                .expect("2ad1faf7");
                                args_ts.push(quote! {
                                    #dim_nbr_pgn_ts: #import::PgnStartsWithZero
                                });
                            }
                            quote! {{#(#args_ts),*}}
                        }
                    )
                );
            let gen_dflt_some_one_cnt_ts = |dflt_some_one_or_dflt_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| {
                let content_ts = ArrDim::try_from(pattern).map_or_else(
                    |()| Ts2::new(),
                    |arr_dim| {
                        let content_ts: &dyn ToTokens = match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudCmnDfltSomeOneElCall,
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudCmnDfltSomeOneElMaxPageSizeCall,
                        };
                        let mut args_ts = Vec::new();
                        for el0 in 1..=arr_dim.to_usize() {
                            let dim_nbr_pgn_ts = format!("dim{el0}_pgn").parse::<Ts2>().expect("26ca29fb");
                            args_ts.push(quote! {
                                #dim_nbr_pgn_ts: #content_ts
                            });
                        }
                        quote! {#(#args_ts),*}
                    }
                );
                quote! {Self{#content_ts}}
            };
            let impl_dflt_some_one_el_for_pg_json_ident_sel_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_sel_ucc, &gen_dflt_some_one_cnt_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne));
            let impl_dflt_some_one_el_max_page_size_for_pg_json_ident_sel_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts(&ident_sel_ucc, &gen_dflt_some_one_cnt_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize));
            quote! {
                #ident_sel_ts
                #impl_dflt_some_one_el_for_pg_json_ident_sel_ts
                #impl_dflt_some_one_el_max_page_size_for_pg_json_ident_sel_ts
            }
        };
        let ident_rd_ucc = SelfRdUcc::from_tokens(&ident);
        let gen_dim_tt_ucc_ts = |dim_is_nl: &IsNl, down_pattern: &Pattern| {
            let v = SelfTtUcc::from_tokens(&gen_ident_ts(dim_is_nl, down_pattern));
            quote! {#v}
        };
        let ident_wh_ts = match &is_nl {
            IsNl::False => gen_pg_type_wh_ts(
                &AllowClippyArbitrarySrcItemOrdering,
                &{
                    #[derive(Debug, Clone, Optml)]
                    enum PgJsonSpecific {
                        Bool,
                        Nbr,
                        String,
                    }
                    impl From<&PgJson> for PgJsonSpecific {
                        fn from(v: &PgJson) -> Self {
                            match v {
                                PgJson::I8AsJsonbNbr
                                | PgJson::I16AsJsonbNbr
                                | PgJson::I32AsJsonbNbr
                                | PgJson::I64AsJsonbNbr
                                | PgJson::U8AsJsonbNbr
                                | PgJson::U16AsJsonbNbr
                                | PgJson::U32AsJsonbNbr
                                | PgJson::U64AsJsonbNbr
                                | PgJson::F32AsJsonbNbr
                                | PgJson::F64AsJsonbNbr => Self::Nbr,
                                PgJson::BoolAsJsonbBoolean => Self::Bool,
                                PgJson::StringAsJsonbString | PgJson::UuidUuidAsJsonbString => Self::String,
                            }
                        }
                    }
                    let pg_json_specific = PgJsonSpecific::from(pg_json);
                    let cmn_pg_json_flts = vec![PgJsonFlt::Eq { ident: quote! {#ident_tt_ucc} }];
                    let ident_tt_ucc_ts = quote! {#ident_tt_ucc};
                    match &pattern {
                        Pattern::Stdrt => {
                            let cmn_stdrt_pg_json_flts = {
                                let mut vec = cmn_pg_json_flts;
                                vec.push(PgJsonFlt::In {
                                    ident: ident_tt_ucc_ts.clone(),
                                });
                                vec
                            };
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => {
                                    let mut vec = cmn_stdrt_pg_json_flts;
                                    vec.push(PgJsonFlt::GreaterThan {
                                        ident: ident_tt_ucc_ts.clone(),
                                    });
                                    vec.push(PgJsonFlt::Btwn { ident: ident_tt_ucc_ts });
                                    vec
                                }
                                PgJsonSpecific::Bool => cmn_stdrt_pg_json_flts,
                                PgJsonSpecific::String => {
                                    let mut vec = cmn_stdrt_pg_json_flts;
                                    vec.push(PgJsonFlt::Rgx);
                                    vec
                                }
                            }
                        }
                        Pattern::ArrDim1 { dim1_is_nl } => {
                            let arr_dim1_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim1_is_nl, &pattern.down_by_1().expect("21eaebaf"));
                            let cmn_arr_dim1_pg_json_flts = {
                                let mut vec = cmn_pg_json_flts;
                                vec.push(PgJsonFlt::DimOneEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::LenEq);
                                vec.push(PgJsonFlt::DimOneLenEq);
                                vec.push(PgJsonFlt::LenGreaterThan);
                                vec.push(PgJsonFlt::DimOneLenGreaterThan);
                                vec.push(PgJsonFlt::DimOneContainsAllElsOfArr {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneOverlapsWithArr {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::AllElsEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneIn {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => {
                                    let mut flts = cmn_arr_dim1_pg_json_flts;
                                    flts.push(PgJsonFlt::DimOneGreaterThan {
                                        ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimOneBtwn {
                                        ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::ContainsElGreaterThan {
                                        ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::AllElsGreaterThan {
                                        ident: arr_dim1_inn_el_ident_tt_ucc,
                                    });
                                    flts
                                }
                                PgJsonSpecific::Bool => cmn_arr_dim1_pg_json_flts,
                                PgJsonSpecific::String => {
                                    let mut flts = cmn_arr_dim1_pg_json_flts;
                                    flts.push(PgJsonFlt::DimOneRgx);
                                    flts.push(PgJsonFlt::ContainsElRgx);
                                    flts.push(PgJsonFlt::AllElsRgx);
                                    flts
                                }
                            }
                        }
                        Pattern::ArrDim2 { dim1_is_nl, dim2_is_nl } => {
                            let arr_dim1_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim1_is_nl, &pattern.down_by_1().expect("0c4491c4"));
                            let arr_dim2_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim2_is_nl, &pattern.down_by_2().expect("2d4ee5d4"));
                            let cmn_arr_dim2_pg_json_flts = {
                                let mut vec = cmn_pg_json_flts;
                                vec.push(PgJsonFlt::DimOneEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimTwoEq {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::LenEq);
                                vec.push(PgJsonFlt::DimOneLenEq);
                                vec.push(PgJsonFlt::DimTwoLenEq);
                                vec.push(PgJsonFlt::LenGreaterThan);
                                vec.push(PgJsonFlt::DimOneLenGreaterThan);
                                vec.push(PgJsonFlt::DimTwoLenGreaterThan);
                                vec.push(PgJsonFlt::DimTwoContainsAllElsOfArr {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimTwoOverlapsWithArr {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::AllElsEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneAllElsEq {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneIn {
                                    ident: arr_dim1_inn_el_ident_tt_ucc,
                                });
                                vec.push(PgJsonFlt::DimTwoIn {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => {
                                    let mut flts = cmn_arr_dim2_pg_json_flts;
                                    flts.push(PgJsonFlt::DimTwoGreaterThan {
                                        ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimTwoBtwn {
                                        ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimOneContainsElGreaterThan {
                                        ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimOneAllElsGreaterThan {
                                        ident: arr_dim2_inn_el_ident_tt_ucc,
                                    });
                                    flts
                                }
                                PgJsonSpecific::Bool => cmn_arr_dim2_pg_json_flts,
                                PgJsonSpecific::String => {
                                    let mut flts = cmn_arr_dim2_pg_json_flts;
                                    flts.push(PgJsonFlt::DimTwoRgx);
                                    flts.push(PgJsonFlt::DimOneContainsElRgx);
                                    flts.push(PgJsonFlt::DimOneAllElsRgx);
                                    flts
                                }
                            }
                        }
                        Pattern::ArrDim3 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                        } => {
                            let arr_dim1_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim1_is_nl, &pattern.down_by_1().expect("3450bef4"));
                            let arr_dim2_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim2_is_nl, &pattern.down_by_2().expect("3c0d10f4"));
                            let arr_dim3_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim3_is_nl, &pattern.down_by_3().expect("9aaf9e82"));
                            let cmn_arr_dim3_pg_json_flts = {
                                let mut vec = cmn_pg_json_flts;
                                vec.push(PgJsonFlt::DimOneEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimTwoEq {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimThreeEq {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::LenEq);
                                vec.push(PgJsonFlt::DimOneLenEq);
                                vec.push(PgJsonFlt::DimTwoLenEq);
                                vec.push(PgJsonFlt::DimThreeLenEq);
                                vec.push(PgJsonFlt::LenGreaterThan);
                                vec.push(PgJsonFlt::DimOneLenGreaterThan);
                                vec.push(PgJsonFlt::DimTwoLenGreaterThan);
                                vec.push(PgJsonFlt::DimThreeLenGreaterThan);
                                vec.push(PgJsonFlt::DimThreeContainsAllElsOfArr {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimThreeOverlapsWithArr {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::AllElsEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneAllElsEq {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimTwoAllElsEq {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneIn {
                                    ident: arr_dim1_inn_el_ident_tt_ucc,
                                });
                                vec.push(PgJsonFlt::DimTwoIn {
                                    ident: arr_dim2_inn_el_ident_tt_ucc,
                                });
                                vec.push(PgJsonFlt::DimThreeIn {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => {
                                    let mut flts = cmn_arr_dim3_pg_json_flts;
                                    flts.push(PgJsonFlt::DimThreeGreaterThan {
                                        ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimThreeBtwn {
                                        ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimTwoContainsElGreaterThan {
                                        ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimTwoAllElsGreaterThan {
                                        ident: arr_dim3_inn_el_ident_tt_ucc,
                                    });
                                    flts
                                }
                                PgJsonSpecific::Bool => cmn_arr_dim3_pg_json_flts,
                                PgJsonSpecific::String => {
                                    let mut flts = cmn_arr_dim3_pg_json_flts;
                                    flts.push(PgJsonFlt::DimThreeRgx);
                                    flts.push(PgJsonFlt::DimTwoContainsElRgx);
                                    flts.push(PgJsonFlt::DimTwoAllElsRgx);
                                    flts
                                }
                            }
                        }
                        Pattern::ArrDim4 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                            dim4_is_nl,
                        } => {
                            let arr_dim1_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim1_is_nl, &pattern.down_by_1().expect("550d313b"));
                            let arr_dim2_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim2_is_nl, &pattern.down_by_2().expect("7bda1424"));
                            let arr_dim3_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim3_is_nl, &pattern.down_by_3().expect("b43aa5bd"));
                            let arr_dim4_inn_el_ident_tt_ucc = gen_dim_tt_ucc_ts(dim4_is_nl, &pattern.down_by_4().expect("a246885a"));
                            let cmn_arr_dim4_pg_json_flts = {
                                let mut vec = cmn_pg_json_flts;
                                vec.push(PgJsonFlt::DimOneEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimTwoEq {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimThreeEq {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimFourEq {
                                    ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::LenEq);
                                vec.push(PgJsonFlt::DimOneLenEq);
                                vec.push(PgJsonFlt::DimTwoLenEq);
                                vec.push(PgJsonFlt::DimThreeLenEq);
                                vec.push(PgJsonFlt::DimFourLenEq);
                                vec.push(PgJsonFlt::LenGreaterThan);
                                vec.push(PgJsonFlt::DimOneLenGreaterThan);
                                vec.push(PgJsonFlt::DimTwoLenGreaterThan);
                                vec.push(PgJsonFlt::DimThreeLenGreaterThan);
                                vec.push(PgJsonFlt::DimFourLenGreaterThan);
                                vec.push(PgJsonFlt::DimFourContainsAllElsOfArr {
                                    ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimFourOverlapsWithArr {
                                    ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::AllElsEq {
                                    ident: arr_dim1_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneAllElsEq {
                                    ident: arr_dim2_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimTwoAllElsEq {
                                    ident: arr_dim3_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimThreeAllElsEq {
                                    ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                });
                                vec.push(PgJsonFlt::DimOneIn {
                                    ident: arr_dim1_inn_el_ident_tt_ucc,
                                });
                                vec.push(PgJsonFlt::DimTwoIn {
                                    ident: arr_dim2_inn_el_ident_tt_ucc,
                                });
                                vec.push(PgJsonFlt::DimThreeIn {
                                    ident: arr_dim3_inn_el_ident_tt_ucc,
                                });
                                vec.push(PgJsonFlt::DimFourIn {
                                    ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => {
                                    let mut flts = cmn_arr_dim4_pg_json_flts;
                                    flts.push(PgJsonFlt::DimFourGreaterThan {
                                        ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimFourBtwn {
                                        ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimThreeContainsElGreaterThan {
                                        ident: arr_dim4_inn_el_ident_tt_ucc.clone(),
                                    });
                                    flts.push(PgJsonFlt::DimThreeAllElsGreaterThan {
                                        ident: arr_dim4_inn_el_ident_tt_ucc,
                                    });
                                    flts
                                }
                                PgJsonSpecific::Bool => cmn_arr_dim4_pg_json_flts,
                                PgJsonSpecific::String => {
                                    let mut flts = cmn_arr_dim4_pg_json_flts;
                                    flts.push(PgJsonFlt::DimFourRgx);
                                    flts.push(PgJsonFlt::DimThreeContainsElRgx);
                                    flts.push(PgJsonFlt::DimThreeAllElsRgx);
                                    flts
                                }
                            }
                        }
                    }
                }
                .iter()
                .map(|el0| {
                    let handle: &dyn PgFlt = el0;
                    handle
                })
                .collect(),
                &ident,
                &ShouldDeriveUtoipaToSchema::True,
                &ShouldDSchemarsJsonSchema::True,
                &IsQbMut::False,
            ),
            IsNl::True => quote! {
                pub type #ident_wh_ucc = #import::NlJsonObjPgTypeWhFlt<
                    <#ident_nn_ts as #import::PgJson>::Wh
                >;
            }
        };
        //exists because need to implement .into_inn() for fields (only for rd subtype)
        let ident_rd_ts = {
            //todo mb add some derive\impl to trait
            let ident_rd_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_partial_ord()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &Ts2::new(),
                    &ident_rd_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_ident_rd_ts = gen_impl_new_for_ucc_ts(&ident_rd_ucc);
            let impl_dflt_some_one_el_for_ident_rd_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_rd_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            let impl_sqlx_encode_sqlx_pg_for_ident_rd_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_rd_ucc, &quote! {&#SelfSc.0});
            let impl_sqlx_type_for_ident_rd_ts = gen_impl_sqlx_type_for_ident_ts(&ident_rd_ucc, &gen_sqlx_types_json_type_dcl_ts(&ident_rd_inn_ucc));
            quote! {
                #ident_rd_ts
                #impl_ident_rd_ts
                #impl_dflt_some_one_el_for_ident_rd_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_rd_ts
                #impl_sqlx_type_for_ident_rd_ts
            }
        };
        let ident_rd_ids_stdrt_nn_ucc = SelfRdIdsUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_rd_ids_ts = DTsBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_partial_eq()
            .d_serde_serialize()
            .d_serde_deserialize()
            .build_struct(
                &Ts2::new(),
                &ident_rd_ids_ucc,
                &Ts2::new(),
                &{
                    let opt_unit_ts = gen_opt_type_dcl_ts(&quote! {()});
                    let vec_ts = |ts: &dyn ToTokens| gen_vec_tokens_dcl_ts(&ts);
                    let content_ts = gen_v_dcl_ts(&import, &if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                        let base_ts = quote! {#ident_rd_inn_stdrt_nn_al_ts};
                        match &pattern {
                            Pattern::Stdrt => is_nl.mb_opt_wrap(base_ts),
                            Pattern::ArrDim1 { dim1_is_nl } => {
                                is_nl.mb_opt_wrap(vec_ts(&dim1_is_nl.mb_opt_wrap(base_ts)))
                            }
                            Pattern::ArrDim2 { dim1_is_nl, dim2_is_nl } => {
                                let ts1 = vec_ts(&dim2_is_nl.mb_opt_wrap(base_ts));
                                is_nl.mb_opt_wrap(vec_ts(&dim1_is_nl.mb_opt_wrap(ts1)))
                            }
                            Pattern::ArrDim3 {
                                dim1_is_nl,
                                dim2_is_nl,
                                dim3_is_nl,
                            } => {
                                let ts1 = vec_ts(&dim3_is_nl.mb_opt_wrap(base_ts));
                                let ts2 = vec_ts(&dim2_is_nl.mb_opt_wrap(ts1));
                                is_nl.mb_opt_wrap(vec_ts(&dim1_is_nl.mb_opt_wrap(ts2)))
                            }
                            Pattern::ArrDim4 {
                                dim1_is_nl,
                                dim2_is_nl,
                                dim3_is_nl,
                                dim4_is_nl,
                            } => {
                                let ts1 = vec_ts(&dim4_is_nl.mb_opt_wrap(base_ts));
                                let ts2 = vec_ts(&dim3_is_nl.mb_opt_wrap(ts1));
                                let ts3 = vec_ts(&dim2_is_nl.mb_opt_wrap(ts2));
                                is_nl.mb_opt_wrap(vec_ts(&dim1_is_nl.mb_opt_wrap(ts3)))
                            }
                        }
                    } else {
                        opt_unit_ts
                    });
                    quote!{(pub #content_ts);}
                }
            );
        let ident_rd_inn_ts = {
            let type_ts = match &pattern {
                Pattern::Stdrt => match &is_nl {
                    IsNl::False => &ident_rd_inn_stdrt_nn_al_ts,
                    IsNl::True => &gen_opt_type_dcl_ts(&ident_rd_inn_stdrt_nn_al_ts),
                },
                Pattern::ArrDim1 { dim1_is_nl } => &{
                    let dim1_type = dim1_is_nl.mb_opt_wrap(quote! {#ident_rd_inn_stdrt_nn_al_ts});
                    is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim1_type))
                },
                Pattern::ArrDim2 { dim1_is_nl, dim2_is_nl } => &{
                    let dim2_type = dim2_is_nl.mb_opt_wrap(quote! {#ident_rd_inn_stdrt_nn_al_ts});
                    let dim1_type = dim1_is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim2_type));
                    is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim1_type))
                },
                Pattern::ArrDim3 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                } => &{
                    let dim3_type = dim3_is_nl.mb_opt_wrap(quote! {#ident_rd_inn_stdrt_nn_al_ts});
                    let dim2_type = dim2_is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim3_type));
                    let dim1_type = dim1_is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim2_type));
                    is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim1_type))
                },
                Pattern::ArrDim4 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                    dim4_is_nl,
                } => &{
                    let dim4_type = dim4_is_nl.mb_opt_wrap(quote! {#ident_rd_inn_stdrt_nn_al_ts});
                    let dim3_type = dim3_is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim4_type));
                    let dim2_type = dim2_is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim3_type));
                    let dim1_type = dim1_is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim2_type));
                    is_nl.mb_opt_wrap(gen_vec_tokens_dcl_ts(&dim1_type))
                },
            };
            let impl_from_ident_orgn_for_ident_rd_inn_ts = gen_impl_from_ts(
                &ident_orgn_ucc,
                &ident_rd_inn_ucc,
                &{
                    let v_dot_zero_ts = quote!{#VSc.0};
                    let nl_ts = quote!{#v_dot_zero_ts.map(Into::into)};
                    match &pattern {
                        Pattern::Stdrt => match &is_nl {
                            IsNl::False => v_dot_zero_ts,
                            IsNl::True => nl_ts,
                        },
                        Pattern::ArrDim1 {..} |
                        Pattern::ArrDim2 {..} |
                        Pattern::ArrDim3 {..} |
                        Pattern::ArrDim4 {..} => match &is_nl {
                            IsNl::False => quote!{#v_dot_zero_ts.into_iter().map(Into::into).collect()},
                            IsNl::True => nl_ts
                        },
                    }
                },
            );
            quote! {
                pub type #ident_rd_inn_ucc = #type_ts;
                #impl_from_ident_orgn_for_ident_rd_inn_ts
            }
        };
        let ident_upd_ts = {
            let ident_upd_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &Ts2::new(),
                    &ident_upd_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_ident_upd_ts = gen_impl_new_for_ucc_ts(&ident_upd_ucc);
            let impl_loc_lib_to_err_string_for_ident_upd_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                gen_impl_to_err_string_ts(&Ts2::new(), &ident_upd_ucc, &Ts2::new(), &quote! {format!("{self:?}")})
            } else {
                Ts2::new()
            };
            let impl_dflt_some_one_el_for_ident_upd_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_upd_ucc, &quote! {Self(#PgCrudCmnDfltSomeOneElCall)});
            quote! {
                #ident_upd_ts
                #impl_ident_upd_ts
                #impl_loc_lib_to_err_string_for_ident_upd_ts
                #impl_dflt_some_one_el_for_ident_upd_ts
            }
        };
        let ident_upd_for_query_ts = {
            let ident_upd_for_query_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .build_struct(
                    &Ts2::new(),
                    &ident_upd_for_query_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_ident_upd_for_query_ts = gen_impl_new_for_ucc_ts(&ident_upd_for_query_ucc);
            let impl_from_ident_upd_for_ident_upd_for_query_ts = gen_impl_from_ts(&ident_upd_ucc, &ident_upd_for_query_ucc, &quote! {Self(#VSc.0)});
            //its only for primitive json types
            let impl_sqlx_encode_sqlx_pg_for_ident_upd_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_upd_for_query_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            let impl_sqlx_type_for_ident_upd_for_query_ts = gen_impl_sqlx_type_for_ident_ts(&ident_upd_for_query_ucc, &ident_orgn_ucc);
            quote! {
                #ident_upd_for_query_ts
                #impl_ident_upd_for_query_ts
                #impl_from_ident_upd_for_ident_upd_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_upd_for_query_ts
                #impl_sqlx_type_for_ident_upd_for_query_ts
            }
        };
        let pg_crud_macros_cmn_import_pg_crud_cmn = Import::PgCrudCmn;
        let impl_pg_json_for_ident_ts = {
            let gen_dim_nbr_str = |dims_nbr: usize| format!("dim{dims_nbr}");
            let gen_dim_nbr_start_str = |dims_nbr: usize| format!("{}_start", gen_dim_nbr_str(dims_nbr));
            let gen_dim_nbr_end_str = |dims_nbr: usize| format!("{}_end", gen_dim_nbr_str(dims_nbr));
            let sel_only_crd_or_updd_ids_qp_ts = if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                let dq_ts0 = dq_ts(&format!("'{{fi}}',{},", gen_jsonb_build_obj_v(&"${v_f06128be}")));
                quote! {
                    match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                        Ok(v_f06128be) => Ok(format!(#dq_ts0)),
                        Err(#ErSc) => Err(#ErSc),
                    }
                }
            } else {
                quote! {Ok(gen_pg_json_cmn::fi_jsonb_build_obj_v(fi))}
            };
            let sel_only_crd_or_updd_ids_qb_ts = if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                quote! {
                    if let Err(#ErSc) = #QuerySc.try_bind(#VSc) {
                        return Err(#ErSc.to_string());
                    }
                    Ok(#QuerySc)
                }
            } else {
                quote! {Ok(#QuerySc)}
            };
            gen_impl_pg_json_ts(
                &pg_crud_macros_cmn_import_pg_crud_cmn,
                &ident,
                &ident_tt_ucc,
                &ident_cr_ucc,
                &ident_cr_for_query_ucc,
                &ident_sel_ucc,
                &match &pattern {
                    Pattern::Stdrt => IsSelQpSelfSelUsed::False,
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => IsSelQpSelfSelUsed::True,
                },
                &IsSelQpColFieldForErMsgUsed::False,
                &IsSelQpIsPgTypeUsed::False,
                &{
                    let format_h = {
                        //last child dim v does not matter - null or type - works both good
                        let col_field_fi = format!("{{{ColFieldSc}}}->'{{fi}}'");
                        let format_h = ArrDim::try_from(pattern).map_or_else(
                            |()| col_field_fi.clone(),
                            |arr_dim| {
                                enum ArrDimSelPattern {
                                    ArrDim2 {
                                        dim1_is_nl: IsNl,
                                    },
                                    ArrDim3 {
                                        dim1_is_nl: IsNl,
                                        dim2_is_nl: IsNl,
                                    },
                                    ArrDim4 {
                                        dim1_is_nl: IsNl,
                                        dim2_is_nl: IsNl,
                                        dim3_is_nl: IsNl,
                                    },
                                }
                                impl TryFrom<&ArrDim> for ArrDimSelPattern {
                                    type Error = ();
                                    fn try_from(v: &ArrDim) -> Result<Self, Self::Error> {
                                        match &v {
                                            ArrDim::ArrDim1 => Err(()),
                                            ArrDim::ArrDim2 {
                                                dim1_is_nl,
                                            } => Ok(Self::ArrDim2 {
                                                dim1_is_nl: *dim1_is_nl,
                                            }),
                                            ArrDim::ArrDim3 {
                                                dim1_is_nl,
                                                dim2_is_nl,
                                            } => Ok(Self::ArrDim3 {
                                                dim1_is_nl: *dim1_is_nl,
                                                dim2_is_nl: *dim2_is_nl,
                                            }),
                                            ArrDim::ArrDim4 {
                                                dim1_is_nl,
                                                dim2_is_nl,
                                                dim3_is_nl,
                                            } => Ok(Self::ArrDim4 {
                                                dim1_is_nl: *dim1_is_nl,
                                                dim2_is_nl: *dim2_is_nl,
                                                dim3_is_nl: *dim3_is_nl,
                                            }),
                                        }
                                    }
                                }
                                let gen_jsonb_agg = |jsonb_agg_cnt: &str, jsonb_arr_els_cnt: &str, ordinality_cnt: &str, dims_nbr: usize| {
                                    format!(
                                        "select jsonb_agg(({jsonb_agg_cnt})) from jsonb_array_elements(({jsonb_arr_els_cnt})) with ordinality {ordinality_cnt} between {{{}}} and {{{}}}",
                                        gen_dim_nbr_start_str(dims_nbr),
                                        gen_dim_nbr_end_str(dims_nbr),
                                    )
                                };
                                ArrDimSelPattern::try_from(&arr_dim).map_or_else(
                                    |()| gen_jsonb_agg(
                                        "value",
                                        &format!("select {col_field_fi}"),
                                        "where ordinality",
                                        1,
                                    ),
                                    |arr_dim_sel_pattern| {
                                        // Dim1 does not fit into pattern. its only for 2+ dims
                                        let gen_d_nbr_elem = |content: usize| format!("d{content}_elem");
                                        let gen_d_nbr_ord = |content: usize| format!("d{content}_elem");
                                        let gen_dot_v = |content: &str| format!("{content}.value");
                                        let gen_as_v_wh = |
                                            first_cnt: &str,
                                            second_cnt: &str
                                        | format!("as {first_cnt}(value, {second_cnt}) where {second_cnt}");
                                        let one = 1;
                                        gen_jsonb_agg(
                                            &{
                                                let mut usize_v_0ff8cf42 = match &arr_dim_sel_pattern {
                                                    ArrDimSelPattern::ArrDim2 { .. } => 2,
                                                    ArrDimSelPattern::ArrDim3 { .. } => 3,
                                                    ArrDimSelPattern::ArrDim4 { .. } => 4,
                                                };
                                                match &arr_dim_sel_pattern {
                                                    ArrDimSelPattern::ArrDim2 {
                                                        dim1_is_nl,
                                                    } => vec![dim1_is_nl],
                                                    ArrDimSelPattern::ArrDim3 {
                                                        dim1_is_nl,
                                                        dim2_is_nl,
                                                    } => vec![
                                                        dim2_is_nl,
                                                        dim1_is_nl,
                                                    ],
                                                    ArrDimSelPattern::ArrDim4 {
                                                        dim1_is_nl,
                                                        dim2_is_nl,
                                                        dim3_is_nl,
                                                    } => vec![
                                                        dim3_is_nl,
                                                        dim2_is_nl,
                                                        dim1_is_nl,
                                                    ],
                                                }
                                                .into_iter()
                                                .fold(gen_dot_v(&gen_d_nbr_elem(usize_v_0ff8cf42)), |mut acc, is_nl_0ff8cf42| {
                                                    let usize_v_minus_one_0ff8cf42 = usize_v_0ff8cf42.checked_sub(one).expect("a35e873e");
                                                    let d_usize_minus_one_elem_v = gen_dot_v(&gen_d_nbr_elem(usize_v_minus_one_0ff8cf42));
                                                    let v = gen_jsonb_agg(
                                                        &acc,
                                                        &d_usize_minus_one_elem_v,
                                                        &gen_as_v_wh(&gen_d_nbr_elem(usize_v_0ff8cf42), &gen_d_nbr_ord(usize_v_0ff8cf42)),
                                                        usize_v_0ff8cf42,
                                                    );
                                                    acc = match &is_nl_0ff8cf42 {
                                                        IsNl::False => v,
                                                        IsNl::True => {
                                                            format!("case when jsonb_typeof({d_usize_minus_one_elem_v})='arr' then ({v}) else null end")
                                                        }
                                                    };
                                                    usize_v_0ff8cf42 = usize_v_minus_one_0ff8cf42;
                                                    acc
                                                })
                                            },
                                            &col_field_fi,
                                            &gen_as_v_wh(&gen_d_nbr_elem(one), &gen_d_nbr_ord(one)),
                                            one,
                                        )
                                    },
                                )
                            },
                        );
                        match &is_nl {
                            IsNl::False => format_h,
                            IsNl::True => format!("case when jsonb_typeof({col_field_fi})='null' then 'null'::jsonb else ({format_h}) end"),
                        }
                    };
                    let mb_dims_start_end_init = ArrDim::try_from(pattern).ok().into_iter().flat_map(|arr_dim| {
                        (1..=arr_dim.to_usize()).map(|el0| {
                            let dim_nbr_start_ts =
                                gen_dim_nbr_start_str(el0)
                                    .parse::<Ts2>()
                                    .expect("77ec13b9");
                            let dim_nbr_end_ts =
                                gen_dim_nbr_end_str(el0)
                                    .parse::<Ts2>()
                                    .expect("24acbb5e");
                            let dim_nbr_pgn_ts =
                                format!(
                                    "{}_pgn",
                                    gen_dim_nbr_str(el0)
                                )
                                .parse::<Ts2>()
                                .expect("745c99b3");
                            quote! {
                                let #dim_nbr_start_ts = #VSc.#dim_nbr_pgn_ts.start();
                                let #dim_nbr_end_ts = #VSc.#dim_nbr_pgn_ts.end();
                            }
                        })
                    });
                    let dq_ts0 = dq_ts(
                        &gen_jsonb_build_obj(&format!("'{{fi}}',{}", gen_jsonb_build_obj_v(&format!("({format_h})"))))
                    );
                    quote! {
                        #(#mb_dims_start_end_init)*
                        Ok(format!(#dq_ts0))
                    }
                },
                &ident_wh_ucc,
                &ident_rd_ucc,
                &ident_rd_ids_ucc,
                &{
                    let content_ts = if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                        let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"{col_field}"));
                        quote! {format!(#dq_ts0)}
                    } else {
                        let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"'null'::jsonb"));
                        quote! {#dq_ts0.to_owned()}
                    };
                    quote! {Ok(#content_ts)}
                },
                &ident_rd_inn_ucc,
                &{
                    let content_ts_0ff8cf42 = quote! {#VSc.0.0};
                    let gen_match_el_zero_ts = |
                        match_ts: &dyn ToTokens,
                        v_ts: &dyn ToTokens,
                        content_ts: &dyn ToTokens
                    | {
                        quote! {#match_ts.map(|#v_ts| #v_ts.0 #content_ts)}
                    };
                    let gen_into_iter_map_el_collect_ts = |
                        el_ts: &dyn ToTokens,
                        content_ts: &dyn ToTokens
                    | {
                        quote! {.into_iter().map(|#el_ts|#content_ts).collect()}
                    };
                    let gen_into_iter_map_el_collect_is_nl_ts = |
                        el_ts: &dyn ToTokens,
                        is_nl_d9400a66: &IsNl
                    | {
                        gen_into_iter_map_el_collect_ts(
                            &el_ts,
                            &match &is_nl_d9400a66 {
                                IsNl::False => quote! {#el_ts.0},
                                IsNl::True => gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &quote! {v_f8b0b01d},
                                    &Ts2::new()
                                )
                            }
                        )
                    };
                    let gen_into_iter_map_el_collect_is_nl_cnt_ts = |
                        el_ts: &dyn ToTokens,
                        v_ts: &dyn ToTokens,
                        is_nl_d9400a66: &IsNl,
                        content_ts_d9400a66: &dyn ToTokens
                    | {
                        match &is_nl_d9400a66 {
                            IsNl::False => gen_into_iter_map_el_collect_ts(
                                &el_ts,
                                &quote! {#el_ts.0 #content_ts_d9400a66}
                            ),
                            IsNl::True => {
                                let match_el_zero_ts = gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &v_ts,
                                    &content_ts_d9400a66
                                );
                                quote! {.into_iter().map(|#el_ts|#match_el_zero_ts).collect()}
                            }
                        }
                    };
                    let into_inn_cnt_ts = match &pattern {
                        Pattern::Stdrt => Ts2::new(),
                        Pattern::ArrDim1 { dim1_is_nl } => gen_into_iter_map_el_collect_is_nl_ts(
                            &quote!{el_0fdb74a5},
                            dim1_is_nl,
                        ),
                        Pattern::ArrDim2 { dim1_is_nl, dim2_is_nl } => {
                            let dim2_is_nl_cnt_ts = gen_into_iter_map_el_collect_is_nl_ts(
                                &quote!{el_dac5ba56},
                                dim2_is_nl
                            );
                            gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                &quote!{el_cf5646e9},
                                &quote!{v_1c90c80c},
                                dim1_is_nl,
                                &dim2_is_nl_cnt_ts
                            )
                        }
                        Pattern::ArrDim3 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                        } => {
                            let dim3_is_nl_cnt_ts = gen_into_iter_map_el_collect_is_nl_ts(
                                &quote!{el_c935a865},
                                dim3_is_nl
                            );
                            let dim2_is_nl_cnt_ts = gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                &quote!{el_dc9e788b},
                                &quote!{v_3d1307e8},
                                dim2_is_nl,
                                &dim3_is_nl_cnt_ts
                            );
                            gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                &quote!{el_bf67606b},
                                &quote!{v_721e4164},
                                dim1_is_nl,
                                &dim2_is_nl_cnt_ts
                            )
                        }
                        Pattern::ArrDim4 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                            dim4_is_nl,
                        } => {
                            let dim4_is_nl_cnt_ts = gen_into_iter_map_el_collect_is_nl_ts(
                                &quote!{el_144c60e6},
                                dim4_is_nl
                            );
                            let dim3_is_nl_cnt_ts = gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                &quote!{el_98961cb7},
                                &quote!{v_995a5fbe},
                                dim3_is_nl,
                                &dim4_is_nl_cnt_ts
                            );
                            let dim2_is_nl_cnt_ts = gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                &quote!{el_34e95172},
                                &quote!{v_75b18ade},
                                dim2_is_nl,
                                &dim3_is_nl_cnt_ts
                            );
                            gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                &quote!{el_f64124e2},
                                &quote!{v_7fc1b146},
                                dim1_is_nl,
                                &dim2_is_nl_cnt_ts
                            )
                        }
                    };
                    match &is_nl {
                        IsNl::False => quote! {#content_ts_0ff8cf42 #into_inn_cnt_ts},
                        IsNl::True => gen_match_el_zero_ts(
                            &content_ts_0ff8cf42,
                            &quote!{v_3432e728},
                            &into_inn_cnt_ts
                        ),
                    }
                },
                &ident_upd_ucc,
                &ident_upd_for_query_ucc,
                &{
                    let format_ts = dq_ts(&format!("jsonb_set({{{JsonbSetAccumulatorSc}}},'{{{{{{jsonb_set_path}}}}}}',${{v_26526e0f}})"));
                    quote! {
                        match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                            Ok(v_26526e0f) => Ok(format!(#format_ts)),
                            Err(#ErSc) => Err(#ErSc),
                        }
                    }
                },
                &IsUpdQpSelfUpdUsed::False,
                &IsUpdQpJsonbSetTargetUsed::False,
                &IsUpdQbMut::True,
                &quote! {
                    if let Err(er) = query.try_bind(#VSc) {
                        return Err(er.to_string());
                    }
                    Ok(query)
                },
                &sel_only_crd_or_updd_ids_qp_ts,
                &if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                    IsSelOnlyUpddIdsQbMut::True
                } else {
                    IsSelOnlyUpddIdsQbMut::False
                },
                &sel_only_crd_or_updd_ids_qb_ts,
                &sel_only_crd_or_updd_ids_qp_ts,
                &if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                    IsSelOnlyCrdIdsQbMut::True
                } else {
                    IsSelOnlyCrdIdsQbMut::False
                },
                &sel_only_crd_or_updd_ids_qb_ts,
            )
        };
        let mb_impl_pg_json_obj_vec_el_id_for_ident_orgn_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
            let (qb_string_as_pg_text_cr_for_query_ts, qb_string_as_pg_text_upd_for_query_ts) = {
                enum CrOrUpdForQuery {
                    CrForQuery,
                    UpdForQuery,
                }
                let gen_ts = |cr_or_upd_for_query: &CrOrUpdForQuery| {
                    let name_ts = format!(
                        "qb_string_as_pg_text_{}_for_query",
                        match &cr_or_upd_for_query {
                            CrOrUpdForQuery::CrForQuery => "cr",
                            CrOrUpdForQuery::UpdForQuery => "upd",
                        }
                    )
                    .parse::<Ts2>()
                    .expect("f1bcde08");
                    let type_ts: &dyn ToTokens = match &cr_or_upd_for_query {
                        CrOrUpdForQuery::CrForQuery => &CrForQueryUcc,
                        CrOrUpdForQuery::UpdForQuery => &UpdForQueryUcc,
                    };
                    quote! {
                        fn #name_ts(
                            #VSc: <Self::PgJson as #import::PgJson>::#type_ts,
                            mut #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                        ) -> Result<
                            sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                            #StringTs
                        > {
                            if let Err(#ErSc) = #QuerySc.try_bind(#VSc.0.0.to_string()) {
                                return Err(#ErSc.to_string())
                            }
                            Ok(#QuerySc)
                        }
                    }
                };
                (gen_ts(&CrOrUpdForQuery::CrForQuery), gen_ts(&CrOrUpdForQuery::UpdForQuery))
            };
            quote! {
                #AllowClippyArbitrarySrcItemOrdering
                impl #import::PgJsonObjVecElId for #ident {
                    type PgJson = Self;
                    type #CrForQueryUcc = #ident_cr_for_query_ucc;
                    type #UpdUcc = #ident_upd_ucc;
                    type #RdInnUcc = #ident_rd_inn_ucc;
                    #qb_string_as_pg_text_cr_for_query_ts
                    #qb_string_as_pg_text_upd_for_query_ts
                    fn get_inn(#VSc: &<Self::PgJson as #import::PgJson>::#CrForQueryUcc) -> &Self::#RdInnUcc {
                        &#VSc.0.0
                    }
                    fn incr_checked_add_one(#IncrSc: &mut #U64) -> Result<#U64, #import::QpEr> {
                        #import::incr_checked_add_one_returning_incr(#IncrSc)
                    }
                }
            }
        } else {
            Ts2::new()
        };
        let impl_pg_json_test_cases_for_ident_ts = {
            enum F32OrF64 {
                F32,
                F64
            }
            let gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts = |rd_or_upd: &RdOrUpd| {
                let rd_or_upd_ucc = rd_or_upd.ucc();
                quote! {<#SelfUcc::#PgJsonUcc
                    as
                    #pg_crud_macros_cmn_import_pg_crud_cmn::#PgJsonUcc
                >::#rd_or_upd_ucc::#NewSc(#VSc)}
            };
            let stdrt_nn_test_cases_vec_name_ts = match &pg_json {
                PgJson::I8AsJsonbNbr => quote! {i8_test_cases_vec},
                PgJson::I16AsJsonbNbr => quote! {i16_test_cases_vec},
                PgJson::I32AsJsonbNbr => quote! {i32_test_cases_vec},
                PgJson::I64AsJsonbNbr => quote! {i64_test_cases_vec},
                PgJson::U8AsJsonbNbr => quote! {u8_test_cases_vec},
                PgJson::U16AsJsonbNbr => quote! {u16_test_cases_vec},
                PgJson::U32AsJsonbNbr => quote! {u32_test_cases_vec},
                PgJson::U64AsJsonbNbr => quote! {u64_test_cases_vec},
                PgJson::F32AsJsonbNbr => quote! {f32_test_cases_vec},
                PgJson::F64AsJsonbNbr => quote! {f64_test_cases_vec},
                PgJson::BoolAsJsonbBoolean => quote! {bool_test_cases_vec},
                PgJson::StringAsJsonbString => quote! {string_test_cases_vec},
                PgJson::UuidUuidAsJsonbString => quote! {uuid_uuid_test_cases_vec},
            };
            let opt_vec_cr_ts = {
                let gen_some_acc_cnt_ts = |is_nl_c964bb93: &IsNl, ident_ts_dc0d5797: &dyn ToTokens| {
                    let (new_cnt_ts, mb_acc_push_none_ts) = match &is_nl_c964bb93 {
                        IsNl::False => (quote! {vec![el_88131059.0.into()]}, Ts2::new()),
                        IsNl::True => (quote! {Some(el_88131059.0.into())}, quote! {acc_50e99088.push(<Self as #import::PgJson>::Cr::new(#NoneTs));}),
                    };
                    //todo check - mb need to add something here
                    let mb_acc_push_long_vec_ts = match &is_nl {
                        IsNl::False => quote! {
                            let mut acc_27624e5e = Vec::new();
                            for el_0dcb405a in v_8de026a4 {
                                acc_27624e5e.push(el_0dcb405a.0.into());
                            }
                            if !acc_27624e5e.is_empty() {
                                acc_50e99088.push(<Self as #import::PgJson>::Cr::new(acc_27624e5e));
                            }
                        },
                        IsNl::True => Ts2::new(),
                    };
                    let mb_dot_clone_ts = match &is_nl {
                        IsNl::False => quote!{.clone()},
                        IsNl::True => Ts2::new(),
                    };
                    quote! {Some({
                        let mut acc_50e99088 = Vec::new();
                        if let Some(v_8de026a4) = <#ident_ts_dc0d5797 as #import::PgJsonTestCases>::#OptVecCrSc() {
                            for el_88131059 in v_8de026a4 #mb_dot_clone_ts {
                                acc_50e99088.push(<Self as #import::PgJson>::Cr::new(#new_cnt_ts));
                            }
                            #mb_acc_push_long_vec_ts
                        }
                        #mb_acc_push_none_ts
                        acc_50e99088
                    })}
                };
                let content_ts = match &pattern {
                    Pattern::Stdrt => match &is_nl {
                        IsNl::False => quote! {
                            Some(
                                #import::#stdrt_nn_test_cases_vec_name_ts().into_iter().map(<Self as #import::PgJson>::Cr::new).collect()
                            )
                        },
                        IsNl::True => gen_some_acc_cnt_ts(is_nl, &gen_ident_ts(&IsNl::False, &Pattern::Stdrt)),
                    },
                    Pattern::ArrDim1 { dim1_is_nl, .. } |
                    Pattern::ArrDim2 { dim1_is_nl, .. } |
                    Pattern::ArrDim3 { dim1_is_nl, .. } |
                    Pattern::ArrDim4 { dim1_is_nl, .. } => gen_some_acc_cnt_ts(
                        is_nl,
                        &match &is_nl {
                            IsNl::False => gen_ident_ts(dim1_is_nl, &pattern.down_by_1().expect("dec468c0")),
                            IsNl::True => gen_ident_ts(&IsNl::False, pattern),
                        },
                    ),
                };
                match &pg_json {
                    PgJson::I8AsJsonbNbr
                    | PgJson::I16AsJsonbNbr
                    | PgJson::I32AsJsonbNbr
                    | PgJson::I64AsJsonbNbr
                    | PgJson::U8AsJsonbNbr
                    | PgJson::U16AsJsonbNbr
                    | PgJson::U32AsJsonbNbr
                    | PgJson::U64AsJsonbNbr
                    | PgJson::F32AsJsonbNbr
                    | PgJson::F64AsJsonbNbr
                    | PgJson::BoolAsJsonbBoolean
                    | PgJson::StringAsJsonbString => content_ts,
                    PgJson::UuidUuidAsJsonbString => quote! {None},
                }
            };
            let rd_ids_to_2_dims_vec_rd_inn_ts = {
                let (has_len_greater_than_one_ts, has_len_greater_than_one_for_for_ts) = {
                    let gen_ts = |ts: &dyn ToTokens| {
                        quote! {let has_len_greater_than_one = #ts;}
                    };
                    (
                        gen_ts(&quote! {rd_ids_to_2_dims_vec_rd_inn.len() > 1}),
                        gen_ts(&quote! {{
                            let mut has_len_greater_than_one = false;
                            for el_4a00ab02 in &rd_ids_to_2_dims_vec_rd_inn {
                                if el_4a00ab02.len() > 1 {
                                    has_len_greater_than_one = true;
                                    break;
                                }
                            }
                            has_len_greater_than_one
                        }}),
                    )
                };
                let gen_acc_cnt_h_ts = |ident_ts_416231d8: &dyn ToTokens, has_len_greater_than_one_cnt_ts: &dyn ToTokens| {
                    let ident_rd_ids_ucc_1d31038d = SelfRdIdsUcc::from_tokens(&ident_ts_416231d8);
                    let opt_extra_cnt_ts = {
                        let el_82c7dc0a_clone_ts = quote! {el_82c7dc0a.clone()};
                        let first = quote! {vec![#el_82c7dc0a_clone_ts]};
                        let second = quote! {vec![#el_82c7dc0a_clone_ts, #el_82c7dc0a_clone_ts]};
                        let (first_cnt_ts, second_cnt_ts) = match &is_nl {
                            IsNl::False => (first, second),
                            IsNl::True => {
                                let gen_ts = |ts: &dyn ToTokens| quote! {Some(#ts)};
                                (gen_ts(&first), gen_ts(&second))
                            }
                        };
                        quote! {
                            let opt_extra = {
                                let mut opt_extra = None;
                                for el_c4f9bf8f in &rd_ids_to_2_dims_vec_rd_inn {
                                    if opt_extra.is_some() {
                                        break;
                                    }
                                    for el_82c7dc0a in el_c4f9bf8f {
                                        if opt_extra.is_none() {
                                            opt_extra = Some((vec![#first_cnt_ts], vec![#second_cnt_ts]));
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                                opt_extra
                            };
                        }
                    };
                    let acc_push_vec_cnt_ts = {
                        let content_ts = {
                            let inn_cnt_ts = quote! {{
                                let mut acc_6cd5b60a = Vec::new();
                                for el_640f58e8 in rd_ids_to_2_dims_vec_rd_inn {
                                    for el_d251d1f6 in el_640f58e8 {
                                        acc_6cd5b60a.push(el_d251d1f6);
                                    }
                                }
                                acc_6cd5b60a
                            }};
                            match &is_nl {
                                IsNl::False => inn_cnt_ts,
                                IsNl::True => quote! {Some(#inn_cnt_ts)},
                            }
                        };
                        quote! {acc_0a07db18.push(vec![#content_ts]);}
                    };
                    let mb_acc_push_vec_none_ts = match &is_nl {
                        IsNl::False => Ts2::new(),
                        IsNl::True => quote! {acc_0a07db18.push(vec![#NoneTs]);},
                    };
                    quote! {
                        let mut acc_0a07db18 = Vec::new();
                        let rd_ids_to_2_dims_vec_rd_inn = <
                            #ident_ts_416231d8
                            as
                            #import::PgJsonTestCases
                        >::#RdIdsTo2DimsVecRdInnSc(
                            &#ident_rd_ids_ucc_1d31038d(rd_ids.0.clone())
                        );
                        #opt_extra_cnt_ts
                        #has_len_greater_than_one_cnt_ts
                        #acc_push_vec_cnt_ts
                        #mb_acc_push_vec_none_ts
                        if let Some(v_3de7fba4) = opt_extra {
                            if has_len_greater_than_one {
                                acc_0a07db18.push(v_3de7fba4.0);
                            }
                            else {
                                acc_0a07db18.push(v_3de7fba4.1);
                            }
                        }
                        acc_0a07db18
                    }
                };
                let content_ts = match &pattern {
                    Pattern::Stdrt => match &is_nl {
                        IsNl::False => quote! {vec![#import::#stdrt_nn_test_cases_vec_name_ts().into()]},
                        IsNl::True => quote! {
                            let mut acc_97242d4d = Vec::new();
                            for el_8f3646f9 in <#ident_stdrt_nn_ucc as #import::PgJsonTestCases>::#RdIdsTo2DimsVecRdInnSc(&#ident_rd_ids_stdrt_nn_ucc(rd_ids.0.clone())) {
                                for el_35a4dba9 in el_8f3646f9 {
                                    acc_97242d4d.push(vec![Some(el_35a4dba9)]);
                                }
                            }
                            acc_97242d4d.push(vec![None]);
                            acc_97242d4d
                        },
                    },
                    Pattern::ArrDim1 { dim1_is_nl } => gen_acc_cnt_h_ts(
                        &gen_ident_ts(dim1_is_nl, &pattern.down_by_1().expect("d6f89137")),
                        &match &dim1_is_nl {
                            IsNl::False => &has_len_greater_than_one_for_for_ts,
                            IsNl::True => &has_len_greater_than_one_ts,
                        },
                    ),
                    Pattern::ArrDim2 { dim1_is_nl, .. } |
                    Pattern::ArrDim3 { dim1_is_nl, .. } |
                    Pattern::ArrDim4 { dim1_is_nl, .. } => gen_acc_cnt_h_ts(&gen_ident_ts(dim1_is_nl, &pattern.down_by_1().expect("38774398")), &has_len_greater_than_one_ts),
                };
                match &pg_json {
                    PgJson::I8AsJsonbNbr
                    | PgJson::I16AsJsonbNbr
                    | PgJson::I32AsJsonbNbr
                    | PgJson::I64AsJsonbNbr
                    | PgJson::U8AsJsonbNbr
                    | PgJson::U16AsJsonbNbr
                    | PgJson::U32AsJsonbNbr
                    | PgJson::U64AsJsonbNbr
                    | PgJson::F32AsJsonbNbr
                    | PgJson::F64AsJsonbNbr
                    | PgJson::BoolAsJsonbBoolean
                    | PgJson::StringAsJsonbString => content_ts,
                    PgJson::UuidUuidAsJsonbString => quote! {Vec::new()},
                }
            };
            let rd_inn_into_rd_with_new_or_try_new_unwraped_ts = gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(&RdOrUpd::Rd);
            let rd_inn_into_upd_with_new_or_try_new_unwraped_ts = gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(&RdOrUpd::Upd);
            let rd_ids_into_opt_v_rd_inn_ts = {
                let content_ts = gen_v_init_ts0(&if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                    quote! {#VSc.0.#VSc}
                } else {
                    quote! {
                        <Self as #import::PgJson>::into_inn(
                            <
                                <Self as #import::PgJson>::Rd
                                as
                                #PgCrudCmnDfltSomeOneEl
                            >::dflt_some_one_el()
                        )
                    }
                });
                quote! {Some(#content_ts)}
            };
            let upd_to_rd_ids_ts = {
                let ts = gen_v_init_ts0(&if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                    let gen_iter_or_match_ts = |
                        is_nl_1d9cc9dd: &IsNl,
                        ident_ts_36d8e080: &dyn ToTokens,
                        upd_is_nl_69216aba: &IsNl
                    | {
                        let v_zero_zero_ts = quote! {#VSc.0.0};
                        let content_ts = {
                            let ident_upd_ts_7c40250a = SelfUpdUcc::from_tokens(&ident_ts_36d8e080);
                            let content_ts = {
                                let content_ts = match &upd_is_nl_69216aba {
                                    IsNl::False => quote! {el_aa999306.clone()},
                                    IsNl::True => quote! {v_92de91cc.clone()},
                                };
                                quote! {#ident_upd_ts_7c40250a(#content_ts)}
                            };
                            quote! {
                                <
                                    #ident_ts_36d8e080
                                    as
                                    #import::PgJsonTestCases
                                >::upd_to_rd_ids(&#content_ts).0.#VSc
                            }
                        };
                        match &is_nl_1d9cc9dd {
                            IsNl::False => quote! {
                                #v_zero_zero_ts.iter().map(|el_aa999306|#content_ts).collect()
                            },
                            IsNl::True => quote! {
                                #v_zero_zero_ts.as_ref().map(|v_92de91cc| #content_ts)
                            },
                        }
                    };
                    match &pattern {
                        Pattern::Stdrt => match &is_nl {
                            IsNl::False => quote! {#VSc.0.clone().into()},
                            IsNl::True => gen_iter_or_match_ts(
                                is_nl,
                                &ident_nn_ts,
                                is_nl
                            ),
                        },
                        Pattern::ArrDim1 { dim1_is_nl, .. } |
                        Pattern::ArrDim2 { dim1_is_nl, .. } |
                        Pattern::ArrDim3 { dim1_is_nl, .. } |
                        Pattern::ArrDim4 { dim1_is_nl, .. } => gen_iter_or_match_ts(
                            is_nl,
                            &gen_ident_ts(
                                &match &is_nl {
                                    IsNl::False => *dim1_is_nl,
                                    IsNl::True => IsNl::False,
                                },
                                &match &is_nl {
                                    IsNl::False => pattern.down_by_1().expect("e84064c3"),
                                    IsNl::True => pattern.clone(),
                                },
                            ),
                            is_nl,
                        ),
                    }
                } else {
                    quote!{#NoneTs}
                });
                quote! {#ident_rd_ids_ucc(#ts)}
            };
            let rd_ids_to_opt_v_rd_dflt_some_one_el_ts = {
                let ts = gen_v_init_ts0(&if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                    quote! {#ident_rd_ucc::new(#VSc.0.#VSc.clone())}
                } else {
                    quote! {#PgCrudCmnDfltSomeOneElCall}
                });
                quote! {Some(#ts)}
            };
            let previous_rd_and_opt_upd_into_rd_ts = quote! {
                #OptUpdSc.map_or(#RdSc, |v_f6e37412| #ident_rd_ucc(v_f6e37412.into()))
            };
            let rd_ids_and_cr_into_rd_ts = {
                let content_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                    quote! {#ident_orgn_ucc::new(#RdIdsSc.0.#VSc)}
                } else {
                    quote! {#CrSc.into()}
                };
                quote! {#ident_rd_ucc(#content_ts)}
            };
            let rd_ids_and_cr_into_opt_v_rd_ts = {
                let ts = gen_v_init_ts0(&quote! {
                    <Self as #import::PgJsonTestCases>::#RdIdsAndCrIntoRdSc(
                        #RdIdsSc,
                        #CrSc
                    )
                });
                quote! {Some(#ts)}
            };
            let rd_ids_and_cr_into_tt_ts = {
                let ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                    quote! {#ident_orgn_ucc::new(#RdIdsSc.0.#VSc)}
                } else {
                    quote! {#CrSc.into()}
                };
                quote! {#ident_tt_ucc(#ts)}
            };
            let rd_ids_and_cr_into_wh_eq_ts = {
                let gen_eq_ts = |ts: &dyn ToTokens| {
                    quote! {
                        wh_flts::PgJsonWhEq {
                            oprtr: #import::Oprtr::Or,
                            #VSc: #ts
                        }
                    }
                };
                match &is_nl {
                    IsNl::False => {
                        let eq_ts = gen_eq_ts(&quote! {#ident_tt_ucc::new(#CrSc.0.into())});
                        quote! {#ident_wh_ucc::#EqUcc(#eq_ts)}
                    }
                    IsNl::True => {
                        let ident_wh_ucc_029b3848 = SelfWhUcc::from_tokens(&ident_nn_ts);
                        let ident_tt_ucc_db49334a = SelfTtUcc::from_tokens(&ident_nn_ts);
                        let eq_ts = gen_eq_ts(&quote! {#ident_tt_ucc_db49334a::new(v_18544acf.into())});
                        quote! {
                            #import::NlJsonObjPgTypeWhFlt(
                                #CrSc.0.0.map(|v_18544acf| pg_crud_cmn::NotEmptyUnqVec::try_new(
                                    vec![#ident_wh_ucc_029b3848::#EqUcc(#eq_ts)]
                                ).expect("88bfa095"))
                            )
                        }
                    }
                }
            };
            let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts = quote! {
                #import::NotEmptyUnqVec::try_new(vec![
                    <Self as #import::PgJsonTestCases>::#RdIdsAndCrIntoWhEqSc(
                        #RdIdsSc,
                        #CrSc
                    )
                ]).expect("56eb9ad4")
            };
            let rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts = quote! {<Self as #import::PgJsonTestCases>::#RdIdsAndCrIntoVecWhEqUsingFieldsSc(
                #RdIdsSc,
                #CrSc
            )};
            let (
                rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts,
                rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts,
                rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts,
                rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts
            ) = {
                let gen_ts = |dim: &Dim| {
                    let dim_i_nbr_max = DimIndexNbr::from(dim);
                    let gen_dim_i_nbr_ts = |is_nl_vec: &[&IsNl]|{
                        assert!(!is_nl_vec.is_empty(), "c1a5939d");
                        let content_ts_c85923bd = {
                            let gen_i_nbr_ts = |i_c1128a3e: usize|format!("i_{i_c1128a3e}").parse::<Ts2>().expect("afbe7252");
                            let gen_v_nbr_ts = |i_0abe6039: usize|format!("v{i_0abe6039}").parse::<Ts2>().expect("568d8eb6");
                            let gen_for_in_ts = |
                                i_ts: &dyn ToTokens,
                                v_ts: &dyn ToTokens,
                                enumerate_ts: &dyn ToTokens,
                                content_ts_aaf03124: &dyn ToTokens,
                            |quote!{
                                for (#i_ts, #v_ts) in #enumerate_ts.0.into_iter().enumerate() {
                                    #content_ts_aaf03124
                                }
                            };
                            let gen_for_v_i_dot_zero_into_iter_enumerate_ts = |
                                i_0082bcdf: usize,
                                i_e81c6d28: usize,
                                i_b7b230b2: usize,
                                content_ts_d575a40c: &dyn ToTokens,
                            |gen_for_in_ts(
                                &gen_i_nbr_ts(i_0082bcdf),
                                &gen_v_nbr_ts(i_e81c6d28),
                                &gen_v_nbr_ts(i_b7b230b2),
                                &content_ts_d575a40c
                            );
                            let gen_if_let_some_ts = |
                                some_ts: &dyn ToTokens,
                                eq_ts: &dyn ToTokens,
                                content_ts_9292e3cf: &dyn ToTokens,
                            |quote!{
                                if let Some(#some_ts) = #eq_ts.0 {
                                    #content_ts_9292e3cf
                                }
                                else {
                                    return None;//todo to fix it - should rewrite logic
                                }
                            };
                            let gen_if_let_some_eqs_v_i_dot_zero_ts = |
                                i_c4552aef: usize,
                                i_9f1fbc9f: usize,
                                content_ts_832b20d5: &dyn ToTokens,
                            |gen_if_let_some_ts(
                                &gen_v_nbr_ts(i_c4552aef),
                                &gen_v_nbr_ts(i_9f1fbc9f),
                                &content_ts_832b20d5
                            );
                            let gen_i = |start_i: usize, is_nl_vec_41b82a0c: &[&IsNl]| -> usize {
                                start_i.checked_add(
                                    is_nl_vec_41b82a0c
                                    .iter()
                                    .filter(|el0| matches!(el0, IsNl::True))
                                    .count()
                                ).expect("de4c4116")
                            };
                            let mut content_ts_4c106eea = {
                                let content_ts_f1ffd3b2 = {
                                    let ts = gen_v_nbr_ts(
                                        gen_i(
                                            is_nl_vec.len().saturating_sub(1),
                                            &once(is_nl)
                                            .chain(
                                                is_nl_vec
                                                    .iter()
                                                    .take(is_nl_vec.len().saturating_sub(1))
                                                    .copied(),
                                            ).collect::<Vec<&IsNl>>()
                                        )
                                    );
                                    let to_nbr_starting_with_one_word_str = |dim_i_nbr: &DimIndexNbr| match dim_i_nbr {
                                        DimIndexNbr::Zero => "One",
                                        DimIndexNbr::One => "Two",
                                        DimIndexNbr::Two => "Three",
                                        DimIndexNbr::Three => "Four",
                                    };
                                    let dim_nbr_starting_with_one_eq_ts = format!("Dim{}Eq", to_nbr_starting_with_one_word_str(&dim_i_nbr_max)).parse::<Ts2>().expect("52fa34ac");
                                    let pg_json_wh_dim_nbr_starting_with_one_eq_ts = format!("PgJsonWhDim{}Eq", to_nbr_starting_with_one_word_str(&dim_i_nbr_max)).parse::<Ts2>().expect("15d769b0");
                                    let wh_ident_wh_ucc_c994819b = SelfWhUcc::from_tokens(&gen_ident_ts(&IsNl::False, pattern));
                                    let ident_tt_ucc_0d9dce86 = SelfTtUcc::from_tokens(&gen_ident_ts(
                                        is_nl_vec.last().expect("1221f6ec"),
                                        &match dim_i_nbr_max {
                                            DimIndexNbr::Zero => pattern.down_by_1().expect("1a47af86"),
                                            DimIndexNbr::One => pattern.down_by_2().expect("d8260225"),
                                            DimIndexNbr::Two => pattern.down_by_3().expect("473ac422"),
                                            DimIndexNbr::Three => pattern.down_by_4().expect("6a143218"),
                                        }
                                    ));
                                    let vec_cnt_ts = {
                                        let content_ts_0dc5a500 = (
                                            0i32..=match dim_i_nbr_max {
                                                DimIndexNbr::Zero => 0i32,
                                                DimIndexNbr::One => 1i32,
                                                DimIndexNbr::Two => 2i32,
                                                DimIndexNbr::Three => 3i32,
                                            }
                                        )
                                        .map(|el0| {
                                            let i_nbr_ts = format!("i_{el0}")
                                                .parse::<Ts2>()
                                                .expect("f0ce7e73");
                                            quote! {
                                                pg_crud_cmn::UnsignedPartOfI32::try_from(
                                                    i32::try_from(#i_nbr_ts)
                                                        .expect("5a1818e7")
                                                ).expect("ad1ab73f")
                                            }
                                        }).collect::<Vec<Ts2>>();
                                        quote! {#(#content_ts_0dc5a500),*}
                                    };
                                    quote! {
                                        #wh_ident_wh_ucc_c994819b::#dim_nbr_starting_with_one_eq_ts(
                                            wh_flts::#pg_json_wh_dim_nbr_starting_with_one_eq_ts {
                                                oprtr: #import::Oprtr::And,
                                                dims: wh_flts::BoundedVec::try_from(
                                                    vec![#vec_cnt_ts]
                                                ).expect("82cc0a3c"),
                                                #VSc: #ident_tt_ucc_0d9dce86::new(#ts.into()),
                                            }
                                        )
                                    }
                                };
                                match is_nl {
                                    IsNl::False => quote! {acc_049ff0b3.push(#content_ts_f1ffd3b2);},
                                    IsNl::True => gen_not_empty_unq_vec_try_new_match_ts(
                                        &quote!{vec![#content_ts_f1ffd3b2]},
                                        &quote!{v_9328b66f},
                                        &quote!{{
                                            acc_049ff0b3.push(#import::NlJsonObjPgTypeWhFlt(Some(v_9328b66f)));
                                        }},
                                        &quote!{()},
                                        &quote!{panic!("2f5f648a")},
                                    )
                                }
                            };
                            for (i_ef936914, _) in is_nl_vec.iter().take(is_nl_vec.len().saturating_sub(1)).enumerate() {
                                let is_nl_vec_e7e7f6f8 = is_nl_vec
                                .iter()
                                .take(
                                    is_nl_vec
                                        .len()
                                        .saturating_sub(i_ef936914.checked_add(1).expect("75d5ed28")),
                                )
                                .copied()
                                .collect::<Vec<&IsNl>>();
                                let is_nl_vec_e7e7f6f8_len = is_nl_vec_e7e7f6f8.len();
                                let is_nl_vec_e7e7f6f8_len_saturating_sub_one = is_nl_vec_e7e7f6f8_len.saturating_sub(1);
                                content_ts_4c106eea = {
                                    let i_74ae6d77 = gen_i(
                                        is_nl_vec_e7e7f6f8_len_saturating_sub_one,
                                        &once(is_nl)
                                        .chain(
                                            is_nl_vec_e7e7f6f8
                                                .iter()
                                                .take(is_nl_vec_e7e7f6f8_len_saturating_sub_one)
                                                .copied(),
                                        ).collect::<Vec<&IsNl>>()
                                    );
                                    let i_74ae6d77_incr_by_1 = i_74ae6d77.checked_add(1).expect("96e90e72");
                                    match &is_nl_vec_e7e7f6f8.last().expect("88548240") {
                                        IsNl::False => gen_for_v_i_dot_zero_into_iter_enumerate_ts(
                                            is_nl_vec_e7e7f6f8_len,
                                            i_74ae6d77_incr_by_1,
                                            i_74ae6d77,
                                            &content_ts_4c106eea,
                                        ),
                                        IsNl::True => gen_if_let_some_eqs_v_i_dot_zero_ts(
                                            i_74ae6d77_incr_by_1,
                                            i_74ae6d77,
                                            &gen_for_v_i_dot_zero_into_iter_enumerate_ts(
                                                is_nl_vec_e7e7f6f8_len,
                                                i_74ae6d77.checked_add(2).expect("00da046c"),
                                                i_74ae6d77_incr_by_1,
                                                &content_ts_4c106eea,
                                            )
                                        )
                                    }
                                };
                            }
                            let cr_dot_zero_ts = quote!{cr.0};
                            match &is_nl {
                                IsNl::False => gen_for_in_ts(
                                    &gen_i_nbr_ts(0),
                                    &gen_v_nbr_ts(0),
                                    &cr_dot_zero_ts,
                                    &content_ts_4c106eea
                                ),
                                IsNl::True => gen_if_let_some_ts(
                                    &gen_v_nbr_ts(0),
                                    &cr_dot_zero_ts,
                                    &gen_for_v_i_dot_zero_into_iter_enumerate_ts(
                                        0,
                                        1,
                                        0,
                                        &content_ts_4c106eea
                                    )
                                )
                            }
                        };
                        quote! {
                            Some(#import::NotEmptyUnqVec::try_new({
                                let mut acc_049ff0b3 = Vec::new();
                                #content_ts_c85923bd
                                acc_049ff0b3
                            }).expect("e99ecd08"))
                        }
                    };
                    match &pattern {
                        Pattern::Stdrt => quote!{#NoneTs},
                        Pattern::ArrDim1 { dim1_is_nl } => match dim_i_nbr_max {
                            DimIndexNbr::Zero => gen_dim_i_nbr_ts(&[
                                dim1_is_nl,
                            ]),
                            DimIndexNbr::One | DimIndexNbr::Two | DimIndexNbr::Three => quote!{#NoneTs},
                        },
                        Pattern::ArrDim2 { dim1_is_nl, dim2_is_nl } => match dim_i_nbr_max {
                            DimIndexNbr::Zero => gen_dim_i_nbr_ts(&[
                                dim1_is_nl,
                            ]),
                            DimIndexNbr::One => gen_dim_i_nbr_ts(&[
                                dim1_is_nl,
                                dim2_is_nl
                            ]),
                            DimIndexNbr::Two | DimIndexNbr::Three => quote!{#NoneTs},
                        },
                        Pattern::ArrDim3 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                        } => match dim_i_nbr_max {
                            DimIndexNbr::Zero => gen_dim_i_nbr_ts(&[
                                dim1_is_nl,
                            ]),
                            DimIndexNbr::One => gen_dim_i_nbr_ts(&[
                                dim1_is_nl,
                                dim2_is_nl,
                            ]),
                            DimIndexNbr::Two => gen_dim_i_nbr_ts(&[
                                dim1_is_nl,
                                dim2_is_nl,
                                dim3_is_nl
                            ]),
                            DimIndexNbr::Three => quote!{#NoneTs},
                        },
                        Pattern::ArrDim4 {
                            dim1_is_nl,
                            dim2_is_nl,
                            dim3_is_nl,
                            dim4_is_nl,
                        } => {
                            match dim_i_nbr_max {
                                DimIndexNbr::Zero => gen_dim_i_nbr_ts(&[
                                    dim1_is_nl
                                ]),
                                DimIndexNbr::One => gen_dim_i_nbr_ts(&[
                                    dim1_is_nl,
                                    dim2_is_nl,
                                ]),
                                DimIndexNbr::Two => gen_dim_i_nbr_ts(&[
                                    dim1_is_nl,
                                    dim2_is_nl,
                                    dim3_is_nl,
                                ]),
                                DimIndexNbr::Three => gen_dim_i_nbr_ts(&[
                                    dim1_is_nl,
                                    dim2_is_nl,
                                    dim3_is_nl,
                                    dim4_is_nl
                                ])
                            }
                        }
                    }
                };
                (
                    gen_ts(&Dim::One),
                    gen_ts(&Dim::Two),
                    gen_ts(&Dim::Three),
                    gen_ts(&Dim::Four)
                )
            };
            let cr_dot_zero_dot_zero = quote! {#CrSc.0.0};
            let gen_wh_nl_cnt_ts = |content_ts: &dyn ToTokens, panic_uuid_ts: &Ts2| match &is_nl {
                IsNl::False => quote! {#ident_wh_ucc #content_ts},
                IsNl::True => {
                    let ident_wh_ucc_nn = SelfWhUcc::from_tokens(&ident_nn_ts);
                    let not_empty_unq_vec_try_new_match_ts = gen_not_empty_unq_vec_try_new_match_ts(
                        &quote!{vec![#ident_wh_ucc_nn #content_ts]},
                        &quote!{v_wh_nl_ok},
                        &quote!{Some(v_wh_nl_ok)},
                        &quote!{{return None;}},
                        &quote!{panic!(#panic_uuid_ts)},
                    );
                    quote! {
                        #import::NlJsonObjPgTypeWhFlt(match #cr_dot_zero_dot_zero {
                            Some(v_wh_nl_some) => #not_empty_unq_vec_try_new_match_ts,
                            None => None,
                        })
                    }
                }
            };
            let cr_into_pg_json_opt_vec_wh_len_eq_ts = match &pattern {
                Pattern::Stdrt => quote!{#NoneTs},
                Pattern::ArrDim1 { .. } |
                Pattern::ArrDim2 { .. } |
                Pattern::ArrDim3 { .. } |
                Pattern::ArrDim4 { .. } => gen_not_empty_unq_vec_try_new_match_ts(
                    &{
                        let ref_ts: &dyn ToTokens = match &is_nl {
                            IsNl::False => &cr_dot_zero_dot_zero,
                            IsNl::True => &quote! {v_wh_nl_some.0},
                        };
                        let len_eq_ts = quote! {
                            ::LenEq(
                                wh_flts::PgJsonWhLenEq {
                                    oprtr: #import::Oprtr::Or,
                                    #VSc: pg_crud_cmn::UnsignedPartOfI32::try_from(
                                        i32::try_from(#ref_ts.len()).expect("64d3424f")
                                    ).expect("081f4463"),
                                }
                            )
                        };
                        let content_ts = gen_wh_nl_cnt_ts(&len_eq_ts, &quote!{"3d7ce854"});
                        quote!{vec![#content_ts]}
                    },
                    &quote!{v_e196e86d},
                    &quote!{Some(v_e196e86d)},
                    &quote!{None},
                    &quote!{panic!("e9f9b021")},
                ),
            };
            let cr_into_pg_json_opt_vec_wh_len_greater_than_ts = match &pattern {
                Pattern::Stdrt => quote!{#NoneTs},
                Pattern::ArrDim1 { .. } |
                Pattern::ArrDim2 { .. } |
                Pattern::ArrDim3 { .. } |
                Pattern::ArrDim4 { .. } => gen_not_empty_unq_vec_try_new_match_ts(
                    &{
                        let ref_ts: &dyn ToTokens = match &is_nl {
                            IsNl::False => &cr_dot_zero_dot_zero,
                            IsNl::True => &quote! {v_wh_nl_some.0},
                        };
                        let len_gt_ts = quote! {
                            ::LenGreaterThan(
                                wh_flts::PgJsonWhLenGreaterThan {
                                    oprtr: #import::Oprtr::Or,
                                    #VSc: if let Ok(v_762dae1f) = pg_crud_cmn::UnsignedPartOfI32::try_from(
                                        if let Ok(v_9dca0200) = i32::try_from(
                                            //todo temp code. make it better checking all cases
                                            match #ref_ts.len().checked_sub(1) {
                                                Some(v_92860143) => v_92860143,
                                                None => {
                                                    return None;
                                                }
                                            }
                                        ) {
                                            v_9dca0200
                                        }
                                        else {
                                            return None;
                                        }
                                    ) {
                                        v_762dae1f
                                    }
                                    else {
                                        return None;
                                    }
                                }
                            )
                        };
                        let content_ts = gen_wh_nl_cnt_ts(&len_gt_ts, &quote!{"584f801e"});
                        quote!{vec![#content_ts]}
                    },
                    &quote!{v_cee8d0ab},
                    &quote!{Some(v_cee8d0ab)},
                    &quote!{None},
                    &quote!{panic!("497359a5")},
                ),
            };
            let gen_dot_checked_sub_one_ts = |ts: &dyn ToTokens|quote!{#ts.checked_sub(1)};
            let gen_minus_one_is_finite_then_some_ts = |
                f32_or_f64: F32OrF64,
                content_ts: &dyn ToTokens
            |{
                let minus_ts = match &f32_or_f64 {
                    F32OrF64::F32 => quote!{1.0f32},
                    F32OrF64::F64 => quote!{1.0f64}
                };
                let more_ts = match &f32_or_f64 {
                    F32OrF64::F32 => quote!{0.1f32},
                    F32OrF64::F64 => quote!{0.1f64}
                };
                quote!{{
                    let #VSc = #content_ts - #minus_ts;
                    //The correct way to compare floating point nbrs is to define an allowed er margin
                    if (#content_ts - #VSc).abs() < #more_ts {
                        None
                    }
                    else {
                        #VSc.is_finite().then_some(#VSc)
                    }
                }}
            };
            let select_nbr_pg_json_ts = |int_ts: Ts2, float32_ts: Ts2, float64_ts: Ts2| match &pg_json {
                PgJson::I8AsJsonbNbr
                | PgJson::I16AsJsonbNbr
                | PgJson::I32AsJsonbNbr
                | PgJson::I64AsJsonbNbr
                | PgJson::U8AsJsonbNbr
                | PgJson::U16AsJsonbNbr
                | PgJson::U32AsJsonbNbr
                | PgJson::U64AsJsonbNbr => int_ts,
                PgJson::F32AsJsonbNbr => float32_ts,
                PgJson::F64AsJsonbNbr => float64_ts,
                PgJson::BoolAsJsonbBoolean
                | PgJson::StringAsJsonbString
                | PgJson::UuidUuidAsJsonbString => quote! {#NoneTs},
            };
            //todo additonal logic for Option<v> and el of arr? optal el of arr?
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts = if matches!(&pattern, Pattern::Stdrt) &&
                matches!(&is_nl, IsNl::False)
            {
                let (
                    int_greater_than_one_less_ts,
                    float_32_greater_than_one_less_ts,
                    float_64_greater_than_one_less_ts,
                ) = {
                    let gen_ts = |ts: &dyn ToTokens|{
                        let not_empty_unq_vec_try_new_match_ts = gen_not_empty_unq_vec_try_new_match_ts(
                            &quote!{vec![
                                #import::SingleOrMultiple::Single(#ident_wh_ucc::GreaterThan(
                                    wh_flts::PgJsonWhGreaterThan {
                                        oprtr: #import::Oprtr::Or,
                                        #VSc: #ident_tt_ucc(
                                            #ident_orgn_ucc(v_7aa498e8)
                                        ),
                                    }
                                ))
                            ]},
                            &quote!{v_6f3e23b5},
                            &quote!{Some(v_6f3e23b5)},
                            &quote!{None},
                            &quote!{panic!("11287f54")},
                        );
                        quote!{
                            let v_7aa498e8 = #ts?;
                            #not_empty_unq_vec_try_new_match_ts
                        }
                    };
                    let cr_dot_zero_dot_zero_ts = quote!{cr.0.0};
                    (
                        gen_ts(&gen_dot_checked_sub_one_ts(
                            &cr_dot_zero_dot_zero_ts
                        )),
                        gen_ts(&gen_minus_one_is_finite_then_some_ts(
                            F32OrF64::F32,
                            &cr_dot_zero_dot_zero_ts
                        )),
                        gen_ts(&gen_minus_one_is_finite_then_some_ts(
                            F32OrF64::F64,
                            &cr_dot_zero_dot_zero_ts
                        )),
                    )
                };
                select_nbr_pg_json_ts(int_greater_than_one_less_ts, float_32_greater_than_one_less_ts, float_64_greater_than_one_less_ts)
            }
            else {
                quote!{#NoneTs}
            };
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts = if matches!(&pattern, Pattern::Stdrt) &&
                matches!(&is_nl, IsNl::False)
            {
                let (
                    btwn_one_less_and_one_more_int_ts,
                    btwn_one_less_and_one_more_float_ts
                ) = {
                    let gen_ts = |
                        less_ts: &dyn ToTokens,
                        more_ts: &dyn ToTokens
                    |quote!{
                        if let (Some(start), Some(end)) = (#less_ts, #more_ts) {
                            match wh_flts::Btwn::try_new(
                                #ident_tt_ucc::new(start),
                                #ident_tt_ucc::new(end)
                            ) {
                                Ok(v_cdde02cc) => match pg_crud_cmn::NotEmptyUnqVec::try_new(vec![
                                    #import::SingleOrMultiple::Single(
                                        #ident_wh_ucc::Btwn(
                                            wh_flts::PgJsonWhBtwn {
                                                oprtr: pg_crud_cmn::Oprtr::Or,
                                                #VSc: v_cdde02cc,
                                            }
                                        )
                                    )
                                ]) {
                                    Ok(v_41af48fb) => Some(v_41af48fb),
                                    Err(er) => match er {
                                        #import::NotEmptyUnqVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUnqVecTryNewEr::NotUnq {..} => panic!("5edabfcc")
                                    }
                                },
                                Err(er) => None
                            }
                        }
                        else {
                            None
                        }
                    };
                    (
                        {
                            let gen_ts0 = |ts: &dyn ToTokens|quote!{cr.0.0.#ts(1)};
                            gen_ts(
                                &gen_ts0(&quote!{checked_sub}),
                                &gen_ts0(&quote!{checked_add})
                            )
                        },
                        {
                            let gen_ts0 = |ts: &dyn ToTokens|quote!{{
                                let #VSc = cr.0.0 #ts 1.0;
                                #VSc.is_finite().then_some(#VSc)
                            }};
                            gen_ts(
                                &gen_ts0(&quote!{-}),
                                &gen_ts0(&quote!{+})
                            )
                        }
                    )
                };
                match &pg_json {
                    PgJson::I8AsJsonbNbr |
                    PgJson::I16AsJsonbNbr |
                    PgJson::I32AsJsonbNbr |
                    PgJson::I64AsJsonbNbr |
                    PgJson::U8AsJsonbNbr |
                    PgJson::U16AsJsonbNbr |
                    PgJson::U32AsJsonbNbr |
                    PgJson::U64AsJsonbNbr => btwn_one_less_and_one_more_int_ts,
                    PgJson::F32AsJsonbNbr |
                    PgJson::F64AsJsonbNbr => btwn_one_less_and_one_more_float_ts,
                    PgJson::BoolAsJsonbBoolean |
                    PgJson::StringAsJsonbString |
                    PgJson::UuidUuidAsJsonbString => quote!{#NoneTs}
                }
            }
            else {
                quote!{#NoneTs}
            };
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts = if matches!(&pattern, Pattern::Stdrt) &&
                matches!(&is_nl, IsNl::False)
            {
                match &pg_json {
                    PgJson::I8AsJsonbNbr |
                    PgJson::I16AsJsonbNbr |
                    PgJson::I32AsJsonbNbr |
                    PgJson::I64AsJsonbNbr |
                    PgJson::U8AsJsonbNbr |
                    PgJson::U16AsJsonbNbr |
                    PgJson::U32AsJsonbNbr |
                    PgJson::U64AsJsonbNbr |
                    PgJson::F32AsJsonbNbr |
                    PgJson::F64AsJsonbNbr => gen_not_empty_unq_vec_try_new_match_ts(
                        &quote!{vec![
                            #import::SingleOrMultiple::Single(
                                #ident_wh_ucc::In(
                                    wh_flts::PgJsonWhIn {
                                        oprtr: #import::Oprtr::Or,
                                        #VSc: wh_flts::PgJsonNotEmptyUnqVec::try_from(
                                            vec![#ident_tt_ucc::new(#CrSc.0.0)]
                                        ).expect("2737c0ed")
                                    }
                                ),
                            )
                        ]},
                        &quote!{v_1c4f89a4},
                        &quote!{Some(v_1c4f89a4)},
                        &quote!{None},
                        &quote!{panic!("16ae359d")},
                    ),
                    PgJson::BoolAsJsonbBoolean |
                    PgJson::StringAsJsonbString |
                    PgJson::UuidUuidAsJsonbString => quote!{#NoneTs}
                }
            }
            else {
                quote!{#NoneTs}
            };
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts = if matches!(&pattern, Pattern::Stdrt) &&
                matches!(&is_nl, IsNl::False)
            {
                match &pg_json {
                    PgJson::I8AsJsonbNbr |
                    PgJson::I16AsJsonbNbr |
                    PgJson::I32AsJsonbNbr |
                    PgJson::I64AsJsonbNbr |
                    PgJson::U8AsJsonbNbr |
                    PgJson::U16AsJsonbNbr |
                    PgJson::U32AsJsonbNbr |
                    PgJson::U64AsJsonbNbr |
                    PgJson::F32AsJsonbNbr |
                    PgJson::F64AsJsonbNbr |
                    PgJson::BoolAsJsonbBoolean |
                    PgJson::UuidUuidAsJsonbString => quote!{#NoneTs},
                    PgJson::StringAsJsonbString => gen_not_empty_unq_vec_try_new_match_ts(
                        &quote!{vec![
                            #import::SingleOrMultiple::Single(
                                #ident_wh_ucc::Rgx(
                                    wh_flts::PgJsonWhRgx {
                                        oprtr: #import::Oprtr::Or,
                                        rgx_case: wh_flts::RgxCase::Sensitive,
                                        #VSc: wh_flts::RgxRgx(regex::Regex::new(&format!("^{}$", regex::escape(&#CrSc.0.0))).expect("3814ff38")),
                                    }
                                ),
                            )
                        ]},
                        &quote!{v_75ae8964},
                        &quote!{Some(v_75ae8964)},
                        &quote!{None},
                        &quote!{panic!("b9713787")},
                    )
                }
            }
            else {
                quote!{#NoneTs}
            };
            //todo add contains_el_greater_than for dim 2,3,4
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts = match &pattern {
                Pattern::ArrDim1 { dim1_is_nl } => {
                    if matches!((&is_nl, &dim1_is_nl), (
                        IsNl::False,
                        IsNl::False
                    )) {
                        let (
                            int_greater_than_one_less_ts,
                            float_32_greater_than_one_less_ts,
                            float_64_greater_than_one_less_ts,
                        ) = {
                            let gen_ts = |ts: &dyn ToTokens|gen_not_empty_unq_vec_try_new_match_ts(
                                &quote!{{
                                    let mut acc_f95ec4f2 = vec![];
                                    for el_ba78af60 in cr.0.0 {
                                        let v_027d0d3a = #ts;
                                        match v_027d0d3a {
                                            Some(v_0cd93c25) => {
                                                acc_f95ec4f2.push(
                                                    #import::SingleOrMultiple::Single(
                                                        #ident_wh_ucc::ContainsElGreaterThan(
                                                            wh_flts::PgJsonWhContainsElGreaterThan {
                                                                oprtr: #import::Oprtr::Or,
                                                                #VSc: #ident_stdrt_nn_tt_ucc(
                                                                    #ident_stdrt_nn_orgn_ucc(v_0cd93c25)
                                                                )
                                                            }
                                                        )
                                                    )
                                                );
                                            },
                                            None => {
                                                return None;
                                            },
                                        }
                                    }
                                    acc_f95ec4f2
                                }},
                                &quote!{v_69c93ec5},
                                &quote!{Some(v_69c93ec5)},
                                &quote!{None},
                                &quote!{panic!("47e44ecd")},
                            );
                            let el_dot_zero_ts = quote!{el_ba78af60.0};
                            (
                                gen_ts(&gen_dot_checked_sub_one_ts(
                                    &el_dot_zero_ts
                                )),
                                gen_ts(&gen_minus_one_is_finite_then_some_ts(
                                    F32OrF64::F32,
                                    &el_dot_zero_ts
                                )),
                                gen_ts(&gen_minus_one_is_finite_then_some_ts(
                                    F32OrF64::F64,
                                    &el_dot_zero_ts
                                )),
                            )
                        };
                        select_nbr_pg_json_ts(int_greater_than_one_less_ts, float_32_greater_than_one_less_ts, float_64_greater_than_one_less_ts)
                    }
                    else {
                        quote!{#NoneTs}
                    }
                },
                Pattern::Stdrt |
                Pattern::ArrDim2 {..} |
                Pattern::ArrDim3 {..} |
                Pattern::ArrDim4 {..} => quote!{#NoneTs}
            };
            //todo add contains_el_regex for dim 2,3,4
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts = match &pattern {
                Pattern::ArrDim1 { dim1_is_nl } => {
                    if matches!((&is_nl, &dim1_is_nl), (
                        IsNl::False,
                        IsNl::False
                    )) {
                        match &pg_json {
                            PgJson::I8AsJsonbNbr |
                            PgJson::I16AsJsonbNbr |
                            PgJson::I32AsJsonbNbr |
                            PgJson::I64AsJsonbNbr |
                            PgJson::U8AsJsonbNbr |
                            PgJson::U16AsJsonbNbr |
                            PgJson::U32AsJsonbNbr |
                            PgJson::U64AsJsonbNbr |
                            PgJson::F32AsJsonbNbr |
                            PgJson::F64AsJsonbNbr |
                            PgJson::BoolAsJsonbBoolean |
                            PgJson::UuidUuidAsJsonbString => quote!{#NoneTs},
                            PgJson::StringAsJsonbString => gen_not_empty_unq_vec_try_new_match_ts(
                                &quote!{cr.0.0.into_iter().map(|el_590fca71| {
                                    #import::SingleOrMultiple::Single(
                                        #ident_wh_ucc::ContainsElRgx(
                                            wh_flts::PgJsonWhContainsElRgx {
                                                oprtr: #import::Oprtr::Or,
                                                rgx_case:
                                                    wh_flts::RgxCase::Sensitive,
                                                #VSc: wh_flts::RgxRgx(
                                                    regex::Regex::new(&format!(
                                                        "^{}$",
                                                        regex::escape(&el_590fca71.0)
                                                    ))
                                                    .expect("7d01653a"),
                                                ),
                                            },
                                        ),
                                    )
                                })
                                .collect()},
                                &quote!{v_0363f494},
                                &quote!{Some(v_0363f494)},
                                &quote!{None},
                                &quote!{panic!("415a73d9")},
                            )
                        }
                    }
                    else {
                        quote!{#NoneTs}
                    }
                },
                Pattern::Stdrt |
                Pattern::ArrDim2 {..} |
                Pattern::ArrDim3 {..} |
                Pattern::ArrDim4 {..} => quote!{#NoneTs}
            };
            gen_impl_pg_json_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &pg_crud_macros_cmn_import_pg_crud_cmn,
                &ident_rd_inn_ucc,
                &ident,
                &opt_vec_cr_ts,
                &rd_ids_to_2_dims_vec_rd_inn_ts,
                &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
                &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
                &rd_ids_into_opt_v_rd_inn_ts,
                &upd_to_rd_ids_ts,
                &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
                &previous_rd_and_opt_upd_into_rd_ts,
                &rd_ids_and_cr_into_rd_ts,
                &rd_ids_and_cr_into_opt_v_rd_ts,
                &rd_ids_and_cr_into_tt_ts,
                &rd_ids_and_cr_into_wh_eq_ts,
                &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
                &rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts,
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
        let generated = quote! {
            #ident_ts
            #ident_orgn_ts
            #ident_tt_ts
            #ident_cr_ts
            #ident_cr_for_query_ts
            #ident_sel_ts
            #ident_wh_ts
            #ident_rd_ts
            #ident_rd_ids_ts
            #ident_rd_inn_ts
            #ident_upd_ts
            #ident_upd_for_query_ts
            #impl_pg_json_for_ident_ts
            #mb_impl_pg_json_obj_vec_el_id_for_ident_orgn_ts
            #impl_pg_json_test_cases_for_ident_ts
        };
        (
            {
                let fi = format!("field_{i}").parse::<Ts2>().expect("f992f797");
                quote! {pub #fi: #ident,}.to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    mb_write_ts_into_file(
        config.pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json,
        "pg_tbl_cols_using_pg_json",
        &{
            let fields_cnt_ts = fields_ts
                .into_iter()
                .map(|el| el.parse::<Ts2>().expect("1d8cd8e4"))
                .collect::<Vec<Ts2>>();
            quote! {
                pub struct PgTblColsUsingPgJsons {
                    #(#fields_cnt_ts)*
                }
            }
        },
        &FormatWithCargofmt::True,
    );
    let generated = {
        let content_ts = pg_json_arr
            .into_iter()
            .map(|el| el.parse::<Ts2>().expect("84e21b40"))
            .collect::<Vec<Ts2>>();
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #GenPgJsonModSc {
                #(#content_ts)*
            }
            pub use #GenPgJsonModSc::*;
        }
    };
    mb_write_ts_into_file(
        config.whole_cnt_write_into_gen_pg_json,
        "gen_pg_json",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
