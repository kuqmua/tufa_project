use gen_quotes::dq_ts;
use macros_helpers::{
    DCopy, DSchemarsJsonSchema, DTsBuilder, FormatWithCargofmt, ShouldWriteTsIntoFile,
    gen_impl_from_ts, gen_pub_const_new_ts, gen_pub_new_ts, mb_write_ts_into_file,
};
use naming::{
    ArrOfUcc, AsUcc, BooleanUcc, ColFieldSc, CrForQueryUcc, CrSc, EqUcc, ErSc, GenPgJsonModSc,
    IncrSc, NbrUcc, NewSc, OptUpdSc, OptVecCrSc, PgJsonUcc, QuerySc, RdIdsAndCrIntoRdSc,
    RdIdsAndCrIntoVecWhEqUsingFieldsSc, RdIdsAndCrIntoWhEqSc, RdIdsSc, RdIdsTo2DimsVecRdInnSc,
    RdInnUcc, RdSc, SelfSc, SelfUcc, StringUcc, UpdForQueryUcc, UpdUcc, VSc, VecOfUcc,
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
    IsSelQpSelfSelUsed, IsStdrtNn, IsUpdQbMut, PgFlt, PgJsonFlt, RdOrUpd,
    ShouldDSchemarsJsonSchema, ShouldDeriveUtoipaToSchema, gen_dim_nbr_pgn_ts,
    gen_impl_crate_is_string_empty_for_ident_ts, gen_impl_display_and_to_err_string_debug_ts,
    gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts,
    gen_impl_pg_crud_cmn_dflt_some_one_el_ts, gen_impl_pg_json_test_cases_for_ident_ts,
    gen_impl_pg_json_ts, gen_impl_sqlx_type_and_encode_for_ident_ts,
    gen_impl_to_err_string_no_generics_ts, gen_opt_type_dcl_ts, gen_pg_type_wh_ts,
    gen_sqlx_types_json_type_dcl_ts, gen_v_dcl_ts, gen_v_init_ts, gen_vec_tokens_dcl_ts,
};
use pg_crud_macros_cmn::{gen_case_jsonb_typeof_null, gen_jsonb_build_obj, gen_jsonb_build_obj_v};
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
use strum::IntoEnumIterator as _;
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
        Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter, Optml,
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
        Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter, Optml,
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
        const fn dim1_is_nl(&self) -> Option<IsNl> {
            match self {
                Self::Stdrt => None,
                Self::ArrDim1 { dim1_is_nl, .. }
                | Self::ArrDim2 { dim1_is_nl, .. }
                | Self::ArrDim3 { dim1_is_nl, .. }
                | Self::ArrDim4 { dim1_is_nl, .. } => Some(*dim1_is_nl),
            }
        }
        const fn dim_count(&self) -> usize {
            match self {
                Self::Stdrt => 0,
                Self::ArrDim1 { .. } => 1,
                Self::ArrDim2 { .. } => 2,
                Self::ArrDim3 { .. } => 3,
                Self::ArrDim4 { .. } => 4,
            }
        }
        fn dims(&self) -> Vec<IsNl> {
            match self {
                Self::Stdrt => vec![],
                Self::ArrDim1 { dim1_is_nl } => vec![*dim1_is_nl],
                Self::ArrDim2 {
                    dim1_is_nl,
                    dim2_is_nl,
                } => vec![*dim1_is_nl, *dim2_is_nl],
                Self::ArrDim3 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                } => vec![*dim1_is_nl, *dim2_is_nl, *dim3_is_nl],
                Self::ArrDim4 {
                    dim1_is_nl,
                    dim2_is_nl,
                    dim3_is_nl,
                    dim4_is_nl,
                } => vec![*dim1_is_nl, *dim2_is_nl, *dim3_is_nl, *dim4_is_nl],
            }
        }
        fn down_by(&self, n: usize) -> Option<Self> {
            let dims = self.dims();
            dims.get(n..).map(Self::from_dims)
        }
        fn from_dims(dims: &[IsNl]) -> Self {
            match dims {
                [] => Self::Stdrt,
                [d1] => Self::ArrDim1 { dim1_is_nl: *d1 },
                [d1, d2] => Self::ArrDim2 {
                    dim1_is_nl: *d1,
                    dim2_is_nl: *d2,
                },
                [d1, d2, d3] => Self::ArrDim3 {
                    dim1_is_nl: *d1,
                    dim2_is_nl: *d2,
                    dim3_is_nl: *d3,
                },
                [d1, d2, d3, d4] => Self::ArrDim4 {
                    dim1_is_nl: *d1,
                    dim2_is_nl: *d2,
                    dim3_is_nl: *d3,
                    dim4_is_nl: *d4,
                },
                _ => panic!("f7a3c1e2 unsupported dims length"),
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
            let dims = v.dims();
            match dims.as_slice() {
                [_] => Ok(Self::ArrDim1),
                [d1, _] => Ok(Self::ArrDim2 { dim1_is_nl: *d1 }),
                [d1, d2, _] => Ok(Self::ArrDim3 {
                    dim1_is_nl: *d1,
                    dim2_is_nl: *d2,
                }),
                [d1, d2, d3, _] => Ok(Self::ArrDim4 {
                    dim1_is_nl: *d1,
                    dim2_is_nl: *d2,
                    dim3_is_nl: *d3,
                }),
                [] | [_, _, _, _, _, ..] => Err(()),
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
        Concrete(Vec<Record>),
        OnlyDimFour,
        OnlyDimOne,
        OnlyDimThree,
        OnlyDimTwo,
        OnlyStdrt,
        Subset(Vec<PgJson>),
        WithDimFour,
        WithDimOne,
        WithDimThree,
        WithDimTwo,
        WithoutDims,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize, Optml)]
    struct GenPgJsonsConfig {
        vrt: ConfigVrt,
        #[serde(default)]
        mod_name: Option<String>,
        #[serde(default)]
        max_dim: Option<i32>,
        pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json: ShouldWriteTsIntoFile,
        whole_cnt_write_into_gen_pg_json: ShouldWriteTsIntoFile,
    }
    panic_loc();
    let config = match from_str::<GenPgJsonsConfig>(&input_ts.to_string()) {
        Ok(v) => v,
        Err(er) => {
            let msg = format!("failed to parse GenPgJsonsConfig: {er}");
            return quote! { compile_error!(#msg); };
        }
    };
    let (fields_ts, pg_json_arr) = {
        let acc = {
            fn all_is_nl_combinations(depth: usize) -> Vec<Vec<IsNl>> {
                if depth == 0 {
                    return vec![vec![]];
                }
                let sub = all_is_nl_combinations(depth.checked_sub(1).expect("a3b7c9d1"));
                sub.into_iter().flat_map(|combo| {
                    IsNl::iter().map(move |nl| {
                        let mut c_drvd = combo.clone();
                        c_drvd.push(nl);
                        c_drvd
                    })
                }).collect()
            }
            let dim_of_pattern = |pattern: &Pattern| -> i32 {
                i32::try_from(pattern.dim_count()).expect("c5d9e3f7")
            };
            let gen_vrts = |max_dim: Option<i32>, exact_dim: Option<i32>, filter: Option<&[PgJson]>|{
                PgJson::iter().filter(|pg_json| filter.is_none_or(|f| f.contains(pg_json))).fold(Vec::new(), |mut acc, pg_json| {
                    for pattern in Pattern::iter() {
                        let pattern_dim = dim_of_pattern(&pattern);
                        let include = match (exact_dim, max_dim) {
                            (Some(ed), _) => pattern_dim == ed,
                            (None, None) => true,
                            (None, Some(md)) => pattern_dim <= md,
                        };
                        if include {
                            let dim_count = pattern.dim_count();
                            for combo in all_is_nl_combinations(dim_count) {
                                for is_nl in IsNl::iter() {
                                    acc.push(Record {
                                        pg_json: pg_json.clone(),
                                        is_nl,
                                        pattern: Pattern::from_dims(&combo),
                                    });
                                }
                            }
                        }
                    }
                    acc
                })
            };
            match config.vrt {
                ConfigVrt::All => gen_vrts(None, None, None),
                ConfigVrt::Concrete(v) => v,
                ConfigVrt::OnlyDimFour => gen_vrts(None, Some(4i32), None),
                ConfigVrt::OnlyDimOne => gen_vrts(None, Some(1i32), None),
                ConfigVrt::OnlyDimThree => gen_vrts(None, Some(3i32), None),
                ConfigVrt::OnlyDimTwo => gen_vrts(None, Some(2i32), None),
                ConfigVrt::OnlyStdrt => gen_vrts(None, Some(0i32), None),
                ConfigVrt::Subset(types) => gen_vrts(config.max_dim, None, Some(&types)),
                ConfigVrt::WithDimFour => gen_vrts(Some(4i32), None, None),
                ConfigVrt::WithDimOne => gen_vrts(Some(1i32), None, None),
                ConfigVrt::WithDimThree => gen_vrts(Some(3i32), None, None),
                ConfigVrt::WithDimTwo => gen_vrts(Some(2i32), None, None),
                ConfigVrt::WithoutDims => gen_vrts(Some(0i32), None, None),
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
                let gen_vec = |record_h_inner: RecordH| gen_record_h_vec(
                    record_h_inner
                ).into_iter().chain(once(record_h.clone())).collect();
                match (&record_h.is_nl, &record_h.pattern) {
                    (IsNl::False, Pattern::Stdrt) => vec![record_h],
                    (IsNl::True, Pattern::Stdrt) => gen_vec(RecordH {
                        is_nl: IsNl::False,
                        pattern: Pattern::Stdrt,
                    }),
                    (IsNl::False, _) => {
                        let dim1 = record_h.pattern.dim1_is_nl().expect("b4e2f8a1");
                        gen_vec(RecordH {
                            is_nl: dim1,
                            pattern: record_h.pattern.down_by(1).expect("0e970a4f"),
                        })
                    }
                    (IsNl::True, _) => gen_vec(RecordH {
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
        let gen_ident_ts = |is_nl_prm: &IsNl, pattern_prm: &Pattern| {
            use std::fmt::Write as _;
            let dims = pattern_prm.dims();
            let mut rust_part = String::new();
            let mut pg_part = String::new();
            for dim_is_nl in &dims {
                write!(rust_part, "{VecOfUcc}{}", dim_is_nl.rust()).expect("e4f5a6b7");
                write!(pg_part, "{ArrOfUcc}{}", dim_is_nl.nn_or_nl_str()).expect("c8d9e0f1");
            }
            write!(rust_part, "{rust_type_name}").expect("a2b3c4d5");
            write!(pg_part, "{pg_json_name}").expect("f6e7d8c9");
            let nn_or_nl_str = is_nl_prm.nn_or_nl_str();
            format!("{}{rust_part}{AsUcc}{nn_or_nl_str}{pg_part}", is_nl_prm.rust()).parse::<Ts2>().expect("998d1471")
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
        let mb_const_fn = if matches!((&pattern, &is_nl), (Pattern::Stdrt, IsNl::False)) {
            ConstFn::True
        } else {
            ConstFn::False
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
        let mb_derive_copy = if matches!(&pattern, Pattern::Stdrt) && !matches!(&pg_json, PgJson::StringAsJsonbString) {
            DCopy::True
        } else {
            DCopy::False
        };
        let ident_rd_inn_sqlx_types_json_type_dcl_ts = gen_sqlx_types_json_type_dcl_ts(&ident_rd_inn_ucc);
        let sqlx_json_ref_self_zero_encode_ts = quote! {sqlx::types::Json(&#SelfSc.0)};
        let ident_orgn_ts = {
            let gen_ident_orgn_non_wrapping = |
                is_nl_prm: &IsNl,
                pattern_prm: &Pattern
            | SelfOrgnUcc::from_tokens(&gen_ident_ts(is_nl_prm, pattern_prm));
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
                    if matches!((&is_stdrt_nn, &pg_json), (IsStdrtNn::True, PgJson::UuidUuidAsJsonbString)) {
                        DSchemarsJsonSchema::False
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
                            let gen_ident_orgn_wrapping = |is_nl_prm: &IsNl, pattern_prm: &Pattern| {
                                let v = gen_ident_orgn_non_wrapping(is_nl_prm, pattern_prm);
                                match &is_nl {
                                    IsNl::False => gen_vec_tokens_dcl_ts(&v),
                                    IsNl::True => gen_opt_type_dcl_ts(&v),
                                }
                            };
                            let gen_dims_ts = |dim1_is_nl_prm: &IsNl|{
                                let (is_nl_drvd, pattern_drvd): (&IsNl, &Pattern) = match &is_nl {
                                    IsNl::False => (dim1_is_nl_prm, &pattern.down_by(1).expect("e994797d")),
                                    IsNl::True => (&IsNl::False, pattern),
                                };
                                gen_ident_orgn_wrapping(is_nl_drvd, pattern_drvd)
                            };
                            match pattern.dim1_is_nl() {
                                None => match &is_nl {
                                    IsNl::False => &ident_rd_inn_stdrt_nn_al_ts,
                                    IsNl::True => &gen_opt_type_dcl_ts(&ident_stdrt_nn_orgn_ucc),
                                },
                                Some(dim1) => &gen_dims_ts(&dim1),
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
                pattern.dim1_is_nl().map_or_else(
                    || match &is_nl {
                        IsNl::False => quote! {#VSc},
                        IsNl::True => gen_v_map_type_new_ts(&ident_stdrt_nn_orgn_ucc),
                    },
                    |dim1| gen_arr_dims_init_ts(&{
                        let (pattern_drvd, is_nl_drvd): (&Pattern, &IsNl) = match &is_nl {
                            IsNl::False => (&pattern.down_by(1).expect("1160d3df"), &dim1),
                            IsNl::True => (pattern, &IsNl::False),
                        };
                        gen_ident_orgn_non_wrapping(is_nl_drvd, pattern_drvd)
                    }),
                )
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
                    PgJson::I8AsJsonbNbr | PgJson::I16AsJsonbNbr | PgJson::I32AsJsonbNbr | PgJson::I64AsJsonbNbr | PgJson::U8AsJsonbNbr | PgJson::U16AsJsonbNbr | PgJson::U32AsJsonbNbr | PgJson::U64AsJsonbNbr | PgJson::F32AsJsonbNbr | PgJson::F64AsJsonbNbr | PgJson::BoolAsJsonbBoolean | PgJson::StringAsJsonbString => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            let mb_impl_is_string_empty_for_ident_orgn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True) {
                match &pg_json {
                    PgJson::StringAsJsonbString => gen_impl_crate_is_string_empty_for_ident_ts(
                        &ident_orgn_ucc,
                        &quote!{self.0.clone().is_empty()}
                    ),
                    PgJson::UuidUuidAsJsonbString => gen_impl_crate_is_string_empty_for_ident_ts(
                        &ident_orgn_ucc,
                        &quote!{self.0.to_string().is_empty()}
                    ),
                    PgJson::I8AsJsonbNbr | PgJson::I16AsJsonbNbr | PgJson::I32AsJsonbNbr | PgJson::I64AsJsonbNbr | PgJson::U8AsJsonbNbr | PgJson::U16AsJsonbNbr | PgJson::U32AsJsonbNbr | PgJson::U64AsJsonbNbr | PgJson::F32AsJsonbNbr | PgJson::F64AsJsonbNbr | PgJson::BoolAsJsonbBoolean => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            let impl_display_and_to_err_string_for_ident_orgn_ts = gen_impl_display_and_to_err_string_debug_ts(&ident_orgn_ucc);
            let some_dflt_some_one_el_call_ts = quote! {Some(#PgCrudCmnDfltSomeOneElCall)};
            let impl_dflt_some_one_el_for_ident_orgn_ts = gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_orgn_ucc, &{
                let content_ts = match &pattern {
                    Pattern::Stdrt => match &is_nl {
                        IsNl::False => match &pg_json {
                            PgJson::StringAsJsonbString => quote! {String::default()},
                            PgJson::UuidUuidAsJsonbString => quote! {uuid::Uuid::new_v4()},
                            PgJson::I8AsJsonbNbr | PgJson::I16AsJsonbNbr | PgJson::I32AsJsonbNbr | PgJson::I64AsJsonbNbr | PgJson::U8AsJsonbNbr | PgJson::U16AsJsonbNbr | PgJson::U32AsJsonbNbr | PgJson::U64AsJsonbNbr | PgJson::F32AsJsonbNbr | PgJson::F64AsJsonbNbr | PgJson::BoolAsJsonbBoolean => quote! {Default::default()},
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
            let impl_sqlx_type_and_encode_for_ident_orgn_ts = gen_impl_sqlx_type_and_encode_for_ident_ts(&ident_orgn_ucc, &ident_rd_inn_sqlx_types_json_type_dcl_ts, &sqlx_json_ref_self_zero_encode_ts);
            quote! {
                #ident_orgn_ts
                #impl_ident_orgn_ts
                #impl_from_ident_cr_for_ident_orgn_ts
                #impl_from_ident_upd_for_ident_orgn_ts
                #mb_impl_schemars_json_schema_for_ident_orgn_ts
                #mb_impl_is_string_empty_for_ident_orgn_ts
                #impl_display_and_to_err_string_for_ident_orgn_ts
                #impl_dflt_some_one_el_for_ident_orgn_ts
                #impl_sqlx_type_and_encode_for_ident_orgn_ts
            }
        };
        let ident_orgn_struct_cnt_ts = quote!{(#ident_orgn_ucc);};
        let self_dflt_some_one_el_call_ts = quote! {Self(#PgCrudCmnDfltSomeOneElCall)};
        let self_v_zero_ts = quote! {Self(#VSc.0)};
        let cmn_d_ts_builder = pg_crud_macros_cmn::cmn_d_ts_builder()
            .d_copy_if(mb_derive_copy)
            .d_utoipa_to_schema()
            .d_schemars_json_schema();
        let gen_for_query_ts = |for_query_ucc: &dyn ToTokens| {
            let struct_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(mb_derive_copy)
                .d_partial_eq()
                .d_serde_serialize()
                .build_struct(
                    &Ts2::new(),
                    &for_query_ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_new_ts = gen_impl_new_for_ucc_ts(&for_query_ucc);
            let impl_sqlx_type_and_encode_ts = gen_impl_sqlx_type_and_encode_for_ident_ts(&for_query_ucc, &ident_orgn_ucc, &sqlx_json_ref_self_zero_encode_ts);
            quote! {
                #struct_ts
                #impl_new_ts
                #impl_sqlx_type_and_encode_ts
            }
        };
        let gen_cr_or_upd_cmn_ts = |ucc: &dyn ToTokens| {
            let struct_ts = cmn_d_ts_builder.build_struct(
                &Ts2::new(),
                &ucc,
                &Ts2::new(),
                &ident_orgn_struct_cnt_ts
            );
            let impl_new_ts = gen_impl_new_for_ucc_ts(&ucc);
            let impl_dflt_some_one_el_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ucc, &self_dflt_some_one_el_call_ts);
            quote! {
                #struct_ts
                #impl_new_ts
                #impl_dflt_some_one_el_ts
            }
        };
        let gen_tt_or_rd_ts = |ucc: &dyn ToTokens| {
            let struct_ts = cmn_d_ts_builder
                .d_partial_ord()
                .build_struct(
                    &Ts2::new(),
                    &ucc,
                    &Ts2::new(),
                    &ident_orgn_struct_cnt_ts
                );
            let impl_new_ts = gen_impl_new_for_ucc_ts(&ucc);
            let impl_dflt_some_one_el_ts =
                gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ucc, &self_dflt_some_one_el_call_ts);
            let impl_sqlx_type_and_encode_ts = gen_impl_sqlx_type_and_encode_for_ident_ts(&ucc, &ident_rd_inn_sqlx_types_json_type_dcl_ts, &quote! {&#SelfSc.0});
            quote! {
                #struct_ts
                #impl_new_ts
                #impl_dflt_some_one_el_ts
                #impl_sqlx_type_and_encode_ts
            }
        };
        let ident_tt_ts = gen_tt_or_rd_ts(&ident_tt_ucc);
        let ident_cr_ts = gen_cr_or_upd_cmn_ts(&ident_cr_ucc);
        let ident_cr_for_query_ts = {
            let cmn_ts = gen_for_query_ts(&ident_cr_for_query_ucc);
            let impl_from_ident_cr_for_ident_cr_for_query_ts = gen_impl_from_ts(&ident_cr_ucc, &ident_cr_for_query_ucc, &self_v_zero_ts);
            let mb_impl_from_ident_upd_for_ident_cr_for_query_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                gen_impl_from_ts(&ident_upd_ucc, &ident_cr_for_query_ucc, &self_v_zero_ts)
            } else {
                Ts2::new()
            };
            quote! {
                #cmn_ts
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
                            let mut args_ts = Vec::with_capacity(arr_dim.to_usize());
                            for el0 in 1..=arr_dim.to_usize() {
                                let dim_nbr_pgn_ts = gen_dim_nbr_pgn_ts(el0);
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
                        let mut args_ts = Vec::with_capacity(arr_dim.to_usize());
                        for el0 in 1..=arr_dim.to_usize() {
                            let dim_nbr_pgn_ts = gen_dim_nbr_pgn_ts(el0);
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
                    let gen_flts_with = |base: Vec<PgJsonFlt>, extra: &[PgJsonFlt]| {
                        let mut vec = base;
                        vec.extend_from_slice(extra);
                        vec
                    };
                    match &pattern {
                        Pattern::Stdrt => {
                            let cmn_stdrt_pg_json_flts = gen_flts_with(cmn_pg_json_flts, &[PgJsonFlt::In {
                                ident: ident_tt_ucc_ts.clone(),
                            }]);
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => gen_flts_with(cmn_stdrt_pg_json_flts, &[
                                    PgJsonFlt::GreaterThan { ident: ident_tt_ucc_ts.clone() },
                                    PgJsonFlt::Btwn { ident: ident_tt_ucc_ts },
                                ]),
                                PgJsonSpecific::Bool => cmn_stdrt_pg_json_flts,
                                PgJsonSpecific::String => gen_flts_with(cmn_stdrt_pg_json_flts, &[PgJsonFlt::Rgx]),
                            }
                        }
                        Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => {
                            let dims_vec = pattern.dims();
                            let n_dims = dims_vec.len();
                            let dim_idents: Vec<Ts2> = dims_vec.iter().enumerate().map(|(dim_idx, dim_is_nl)| {
                                gen_dim_tt_ucc_ts(dim_is_nl, &pattern.down_by(dim_idx.checked_add(1).expect("b1c2d3e4")).expect("f5a6b7c8"))
                            }).collect();
                            let last_ident = dim_idents.last().expect("d9e0f1a2").clone();
                            let mut flts = cmn_pg_json_flts;
                            for (k, ident_k) in dim_idents.iter().enumerate() {
                                flts.push(PgJsonFlt::dim_eq(k.checked_add(1).expect("3be06657"), ident_k.clone()));
                            }
                            flts.push(PgJsonFlt::LenEq);
                            for k in 1..=n_dims {
                                flts.push(PgJsonFlt::dim_len_eq(k));
                            }
                            flts.push(PgJsonFlt::LenGreaterThan);
                            for k in 1..=n_dims {
                                flts.push(PgJsonFlt::dim_len_greater_than(k));
                            }
                            flts.push(PgJsonFlt::dim_contains_all_els_of_arr(n_dims, last_ident.clone()));
                            flts.push(PgJsonFlt::dim_overlaps_with_arr(n_dims, last_ident.clone()));
                            for (k, ident_k) in dim_idents.iter().enumerate() {
                                flts.push(PgJsonFlt::dim_all_els_eq(k, ident_k.clone()));
                            }
                            for (k, ident_k) in dim_idents.iter().enumerate() {
                                flts.push(PgJsonFlt::dim_in(k.checked_add(1).expect("16f28538"), ident_k.clone()));
                            }
                            match &pg_json_specific {
                                PgJsonSpecific::Nbr => {
                                    flts.push(PgJsonFlt::dim_greater_than(n_dims, last_ident.clone()));
                                    flts.push(PgJsonFlt::dim_btwn(n_dims, last_ident.clone()));
                                    flts.push(PgJsonFlt::dim_contains_el_greater_than(n_dims.checked_sub(1).expect("5bcc63f3"), last_ident.clone()));
                                    flts.push(PgJsonFlt::dim_all_els_greater_than(n_dims.checked_sub(1).expect("a5b6c7d8"), last_ident));
                                }
                                PgJsonSpecific::Bool => {}
                                PgJsonSpecific::String => {
                                    flts.push(PgJsonFlt::dim_rgx(n_dims));
                                    flts.push(PgJsonFlt::dim_contains_el_rgx(n_dims.checked_sub(1).expect("e9f0a1b2")));
                                    flts.push(PgJsonFlt::dim_all_els_rgx(n_dims.checked_sub(1).expect("c3d4e5f6")));
                                }
                            }
                            flts
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
        let ident_rd_ts = gen_tt_or_rd_ts(&ident_rd_ucc);
        let ident_rd_ids_stdrt_nn_ucc = SelfRdIdsUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_rd_ids_ts = pg_crud_macros_cmn::cmn_d_ts_builder()
            .build_struct(
                &Ts2::new(),
                &ident_rd_ids_ucc,
                &Ts2::new(),
                &{
                    let opt_unit_ts = gen_opt_type_dcl_ts(&quote! {()});
                    let vec_ts = |ts: &dyn ToTokens| gen_vec_tokens_dcl_ts(&ts);
                    let content_ts = gen_v_dcl_ts(&import, &if matches!(&pg_json, PgJson::UuidUuidAsJsonbString) {
                        let base_ts = quote! {#ident_rd_inn_stdrt_nn_al_ts};
                        {
                            let dims_vec = pattern.dims();
                            let folded = dims_vec.iter().rev().fold(base_ts, |acc, dim_nl| {
                                vec_ts(&dim_nl.mb_opt_wrap(acc))
                            });
                            is_nl.mb_opt_wrap(folded)
                        }
                    } else {
                        opt_unit_ts
                    });
                    quote!{(pub #content_ts);}
                }
            );
        let ident_rd_inn_ts = {
            let type_ts_owned = {
                let dims_vec = pattern.dims();
                let base = quote! {#ident_rd_inn_stdrt_nn_al_ts};
                if dims_vec.is_empty() {
                    match &is_nl {
                        IsNl::False => base,
                        IsNl::True => gen_opt_type_dcl_ts(&base),
                    }
                } else {
                    let folded = dims_vec.iter().rev().fold(base, |acc, dim_nl| {
                        gen_vec_tokens_dcl_ts(&dim_nl.mb_opt_wrap(acc))
                    });
                    is_nl.mb_opt_wrap(folded)
                }
            };
            let type_ts = &type_ts_owned;
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
                        Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => match &is_nl {
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
            let cmn_ts = gen_cr_or_upd_cmn_ts(&ident_upd_ucc);
            let impl_loc_lib_to_err_string_for_ident_upd_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                gen_impl_to_err_string_no_generics_ts(&ident_upd_ucc, &quote! {format!("{self:?}")})
            } else {
                Ts2::new()
            };
            quote! {
                #cmn_ts
                #impl_loc_lib_to_err_string_for_ident_upd_ts
            }
        };
        let ident_upd_for_query_ts = {
            let cmn_ts = gen_for_query_ts(&ident_upd_for_query_ucc);
            let impl_from_ident_upd_for_ident_upd_for_query_ts = gen_impl_from_ts(&ident_upd_ucc, &ident_upd_for_query_ucc, &self_v_zero_ts);
            quote! {
                #cmn_ts
                #impl_from_ident_upd_for_ident_upd_for_query_ts
            }
        };
        let pg_crud_macros_cmn_import_pg_crud_cmn = Import::PgCrudCmn;
        let impl_pg_json_for_ident_ts = {
            let gen_dim_nbr_str = |dims_nbr: usize| format!("dim{dims_nbr}");
            let gen_dim_nbr_start_str = |dims_nbr: usize| format!("{}_start", gen_dim_nbr_str(dims_nbr));
            let gen_dim_nbr_end_str = |dims_nbr: usize| format!("{}_end", gen_dim_nbr_str(dims_nbr));
            let is_uuid = matches!(&pg_json, PgJson::UuidUuidAsJsonbString);
            let opt_sel_only_ids_qp_ts: Option<Ts2> = is_uuid.then(|| {
                let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"{col_field}"));
                quote! {Ok(format!(#dq_ts0))}
            });
            let opt_sel_only_crd_or_updd_ids_qp_ts = is_uuid.then(|| {
                let dq_ts0 = dq_ts(&format!("'{{fi}}',{},", gen_jsonb_build_obj_v(&"${v_f06128be}")));
                quote! {
                    match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                        Ok(v_f06128be) => Ok(format!(#dq_ts0)),
                        Err(#ErSc) => Err(#ErSc),
                    }
                }
            });
            let opt_sel_only_crd_or_updd_ids_qb_ts = is_uuid.then(|| {
                quote! {
                    if let Err(#ErSc) = #QuerySc.try_bind(#VSc) {
                        return Err(#ErSc.to_string());
                    }
                    Ok(#QuerySc)
                }
            });
            gen_impl_pg_json_ts(
                &pg_crud_macros_cmn_import_pg_crud_cmn,
                &ident,
                &ident_tt_ucc,
                &ident_cr_ucc,
                &ident_cr_for_query_ucc,
                &ident_sel_ucc,
                &if matches!(&pattern, Pattern::Stdrt) { IsSelQpSelfSelUsed::False } else { IsSelQpSelfSelUsed::True },
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
                                                let mut dim_depth = match &arr_dim_sel_pattern {
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
                                                .fold(gen_dot_v(&gen_d_nbr_elem(dim_depth)), |mut acc, is_nl_el| {
                                                    let dim_depth_minus_one = dim_depth.checked_sub(one).expect("a35e873e");
                                                    let d_usize_minus_one_elem_v = gen_dot_v(&gen_d_nbr_elem(dim_depth_minus_one));
                                                    let v = gen_jsonb_agg(
                                                        &acc,
                                                        &d_usize_minus_one_elem_v,
                                                        &gen_as_v_wh(&gen_d_nbr_elem(dim_depth), &gen_d_nbr_ord(dim_depth)),
                                                        dim_depth,
                                                    );
                                                    acc = match &is_nl_el {
                                                        IsNl::False => v,
                                                        IsNl::True => {
                                                            format!("case when jsonb_typeof({d_usize_minus_one_elem_v})='array' then ({v}) else null end")
                                                        }
                                                    };
                                                    dim_depth = dim_depth_minus_one;
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
                            IsNl::True => gen_case_jsonb_typeof_null(&col_field_fi, &format_h),
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
                opt_sel_only_ids_qp_ts.as_ref().map(|v| {
                    let r: &dyn ToTokens = v;
                    r
                }),
                &ident_rd_inn_ucc,
                &{
                    let inner_content_ts = quote! {#VSc.0.0};
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
                        is_nl_prm: &IsNl
                    | {
                        gen_into_iter_map_el_collect_ts(
                            &el_ts,
                            &match &is_nl_prm {
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
                        is_nl_prm: &IsNl,
                        content_ts_prm: &dyn ToTokens
                    | {
                        match &is_nl_prm {
                            IsNl::False => gen_into_iter_map_el_collect_ts(
                                &el_ts,
                                &quote! {#el_ts.0 #content_ts_prm}
                            ),
                            IsNl::True => {
                                let match_el_zero_ts = gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &v_ts,
                                    &content_ts_prm
                                );
                                quote! {.into_iter().map(|#el_ts|#match_el_zero_ts).collect()}
                            }
                        }
                    };
                    let into_inn_cnt_ts = {
                        let dims_vec = pattern.dims();
                        let el_names: [Ts2; 4] = [
                            quote!{el_0fdb74a5}, quote!{el_cf5646e9},
                            quote!{el_bf67606b}, quote!{el_f64124e2},
                        ];
                        let v_names: [Ts2; 4] = [
                            quote!{v_f8b0b01d}, quote!{v_1c90c80c},
                            quote!{v_721e4164}, quote!{v_7fc1b146},
                        ];
                        let n_dims = dims_vec.len();
                        if n_dims == 0 {
                            Ts2::new()
                        } else {
                            let last_idx = n_dims.checked_sub(1).expect("163218c1");
                            let mut acc_ts = gen_into_iter_map_el_collect_is_nl_ts(
                                el_names.get(last_idx).expect("a3528607"),
                                dims_vec.get(last_idx).expect("c94eb65d"),
                            );
                            for rev_idx in (0..last_idx).rev() {
                                acc_ts = gen_into_iter_map_el_collect_is_nl_cnt_ts(
                                    el_names.get(rev_idx).expect("9a38f4fc"),
                                    v_names.get(rev_idx).expect("ac3bdb9e"),
                                    dims_vec.get(rev_idx).expect("e14ddc0c"),
                                    &acc_ts,
                                );
                            }
                            acc_ts
                        }
                    };
                    match &is_nl {
                        IsNl::False => quote! {#inner_content_ts #into_inn_cnt_ts},
                        IsNl::True => gen_match_el_zero_ts(
                            &inner_content_ts,
                            &quote!{v_3432e728},
                            &into_inn_cnt_ts
                        ),
                    }
                },
                &ident_upd_ucc,
                &ident_upd_for_query_ucc,
                None,
                &IsUpdQbMut::True,
                &quote! {
                    if let Err(er) = query.try_bind(#VSc) {
                        return Err(er.to_string());
                    }
                    Ok(query)
                },
                opt_sel_only_crd_or_updd_ids_qp_ts.as_ref().map(|qp_ts| {
                    let qb_ts = opt_sel_only_crd_or_updd_ids_qb_ts.as_ref().expect("5b3a21f8");
                    let qp_r: &dyn ToTokens = qp_ts;
                    let qb_r: &dyn ToTokens = qb_ts;
                    (qp_r, &IsSelOnlyUpddIdsQbMut::True, qb_r)
                }),
                opt_sel_only_crd_or_updd_ids_qp_ts.as_ref().map(|qp_ts| {
                    let qb_ts = opt_sel_only_crd_or_updd_ids_qb_ts.as_ref().expect("7c4e92d1");
                    let qp_r: &dyn ToTokens = qp_ts;
                    let qb_r: &dyn ToTokens = qb_ts;
                    (qp_r, &IsSelOnlyCrdIdsQbMut::True, qb_r)
                }),
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
                let gen_some_acc_cnt_ts = |is_nl_prm: &IsNl, ident_ts_prm: &dyn ToTokens| {
                    let (new_cnt_ts, mb_acc_push_none_ts) = match &is_nl_prm {
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
                        if let Some(v_8de026a4) = <#ident_ts_prm as #import::PgJsonTestCases>::#OptVecCrSc() {
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
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => gen_some_acc_cnt_ts(
                        is_nl,
                        &match &is_nl {
                            IsNl::False => gen_ident_ts(&pattern.dim1_is_nl().expect("dec468c0"), &pattern.down_by(1).expect("dec468c1")),
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
                let gen_acc_cnt_h_ts = |ident_ts_prm: &dyn ToTokens, has_len_greater_than_one_cnt_ts: &dyn ToTokens| {
                    let ident_rd_ids_ucc_drvd = SelfRdIdsUcc::from_tokens(&ident_ts_prm);
                    let opt_extra_cnt_ts = {
                        let el_clone_ts = quote! {el_82c7dc0a.clone()};
                        let first = quote! {vec![#el_clone_ts]};
                        let second = quote! {vec![#el_clone_ts, #el_clone_ts]};
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
                            #ident_ts_prm
                            as
                            #import::PgJsonTestCases
                        >::#RdIdsTo2DimsVecRdInnSc(
                            &#ident_rd_ids_ucc_drvd(rd_ids.0.clone())
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
                        &gen_ident_ts(dim1_is_nl, &pattern.down_by(1).expect("d6f89137")),
                        &match &dim1_is_nl {
                            IsNl::False => &has_len_greater_than_one_for_for_ts,
                            IsNl::True => &has_len_greater_than_one_ts,
                        },
                    ),
                    Pattern::ArrDim2 { dim1_is_nl, .. } |
                    Pattern::ArrDim3 { dim1_is_nl, .. } |
                    Pattern::ArrDim4 { dim1_is_nl, .. } => gen_acc_cnt_h_ts(&gen_ident_ts(dim1_is_nl, &pattern.down_by(1).expect("38774398")), &has_len_greater_than_one_ts),
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
                        is_nl_prm: &IsNl,
                        ident_ts_prm: &dyn ToTokens,
                        upd_is_nl_prm: &IsNl
                    | {
                        let v_zero_zero_ts = quote! {#VSc.0.0};
                        let content_ts = {
                            let ident_upd_ts_drvd = SelfUpdUcc::from_tokens(&ident_ts_prm);
                            let content_ts = {
                                let content_ts = match &upd_is_nl_prm {
                                    IsNl::False => quote! {el_aa999306.clone()},
                                    IsNl::True => quote! {v_92de91cc.clone()},
                                };
                                quote! {#ident_upd_ts_drvd(#content_ts)}
                            };
                            quote! {
                                <
                                    #ident_ts_prm
                                    as
                                    #import::PgJsonTestCases
                                >::upd_to_rd_ids(&#content_ts).0.#VSc
                            }
                        };
                        match &is_nl_prm {
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
                        Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => gen_iter_or_match_ts(
                            is_nl,
                            &gen_ident_ts(
                                &match &is_nl {
                                    IsNl::False => pattern.dim1_is_nl().expect("e84064c3"),
                                    IsNl::True => IsNl::False,
                                },
                                &match &is_nl {
                                    IsNl::False => pattern.down_by(1).expect("e84064c4"),
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
            let rd_ids_and_cr_into_orgn_ts = if matches!(&is_stdrt_nn_uuid, IsStdrtNnUuid::True) {
                quote! {#ident_orgn_ucc::new(#RdIdsSc.0.#VSc)}
            } else {
                quote! {#CrSc.into()}
            };
            let rd_ids_and_cr_into_rd_ts = quote! {#ident_rd_ucc(#rd_ids_and_cr_into_orgn_ts)};
            let rd_ids_and_cr_into_opt_v_rd_ts = {
                let ts = gen_v_init_ts0(&quote! {
                    <Self as #import::PgJsonTestCases>::#RdIdsAndCrIntoRdSc(
                        #RdIdsSc,
                        #CrSc
                    )
                });
                quote! {Some(#ts)}
            };
            let rd_ids_and_cr_into_tt_ts = quote! {#ident_tt_ucc(#rd_ids_and_cr_into_orgn_ts)};
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
                        let ident_wh_ucc_drvd = SelfWhUcc::from_tokens(&ident_nn_ts);
                        let ident_tt_ucc_drvd = SelfTtUcc::from_tokens(&ident_nn_ts);
                        let eq_ts = gen_eq_ts(&quote! {#ident_tt_ucc_drvd::new(v_18544acf.into())});
                        quote! {
                            #import::NlJsonObjPgTypeWhFlt(
                                #CrSc.0.0.map(|v_18544acf| pg_crud_cmn::NotEmptyUnqVec::try_new(
                                    vec![#ident_wh_ucc_drvd::#EqUcc(#eq_ts)]
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
                        let content_ts_outer = {
                            let gen_i_nbr_ts = |nbr: usize|format!("i_{nbr}").parse::<Ts2>().expect("afbe7252");
                            let gen_v_nbr_ts = |nbr: usize|format!("v{nbr}").parse::<Ts2>().expect("568d8eb6");
                            let gen_for_in_ts = |
                                i_ts: &dyn ToTokens,
                                v_ts: &dyn ToTokens,
                                enumerate_ts: &dyn ToTokens,
                                content_ts_for_body: &dyn ToTokens,
                            |quote!{
                                for (#i_ts, #v_ts) in #enumerate_ts.0.into_iter().enumerate() {
                                    #content_ts_for_body
                                }
                            };
                            let gen_for_v_i_dot_zero_into_iter_enumerate_ts = |
                                i_nbr: usize,
                                v_from: usize,
                                v_to: usize,
                                content_ts_enumerate_body: &dyn ToTokens,
                            |gen_for_in_ts(
                                &gen_i_nbr_ts(i_nbr),
                                &gen_v_nbr_ts(v_from),
                                &gen_v_nbr_ts(v_to),
                                &content_ts_enumerate_body
                            );
                            let gen_if_let_some_ts = |
                                some_ts: &dyn ToTokens,
                                eq_ts: &dyn ToTokens,
                                content_ts_if_let_body: &dyn ToTokens,
                            |quote!{
                                if let Some(#some_ts) = #eq_ts.0 {
                                    #content_ts_if_let_body
                                }
                                else {
                                    return None;//todo to fix it - should rewrite logic
                                }
                            };
                            let gen_if_let_some_eqs_v_i_dot_zero_ts = |
                                v_from: usize,
                                v_to: usize,
                                content_ts_if_let_eqs_body: &dyn ToTokens,
                            |gen_if_let_some_ts(
                                &gen_v_nbr_ts(v_from),
                                &gen_v_nbr_ts(v_to),
                                &content_ts_if_let_eqs_body
                            );
                            let gen_i = |start_i: usize, is_nl_vec_prm: &[&IsNl]| -> usize {
                                start_i.checked_add(
                                    is_nl_vec_prm
                                    .iter()
                                    .filter(|el0| matches!(el0, IsNl::True))
                                    .count()
                                ).expect("de4c4116")
                            };
                            let mut content_ts_acc = {
                                let content_ts_inner = {
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
                                    let wh_ident_wh_ucc_drvd = SelfWhUcc::from_tokens(&gen_ident_ts(&IsNl::False, pattern));
                                    let ident_tt_ucc_drvd = SelfTtUcc::from_tokens(&gen_ident_ts(
                                        is_nl_vec.last().expect("1221f6ec"),
                                        &match dim_i_nbr_max {
                                            DimIndexNbr::Zero => pattern.down_by(1).expect("1a47af86"),
                                            DimIndexNbr::One => pattern.down_by(2).expect("d8260225"),
                                            DimIndexNbr::Two => pattern.down_by(3).expect("473ac422"),
                                            DimIndexNbr::Three => pattern.down_by(4).expect("6a143218"),
                                        }
                                    ));
                                    let vec_cnt_ts = {
                                        let content_ts_dim_indices = (
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
                                        quote! {#(#content_ts_dim_indices),*}
                                    };
                                    quote! {
                                        #wh_ident_wh_ucc_drvd::#dim_nbr_starting_with_one_eq_ts(
                                            wh_flts::#pg_json_wh_dim_nbr_starting_with_one_eq_ts {
                                                oprtr: #import::Oprtr::And,
                                                dims: wh_flts::BoundedVec::try_from(
                                                    vec![#vec_cnt_ts]
                                                ).expect("82cc0a3c"),
                                                #VSc: #ident_tt_ucc_drvd::new(#ts.into()),
                                            }
                                        )
                                    }
                                };
                                match is_nl {
                                    IsNl::False => quote! {acc_049ff0b3.push(#content_ts_inner);},
                                    IsNl::True => gen_not_empty_unq_vec_try_new_match_ts(
                                        &quote!{vec![#content_ts_inner]},
                                        &quote!{v_9328b66f},
                                        &quote!{{
                                            acc_049ff0b3.push(#import::NlJsonObjPgTypeWhFlt(Some(v_9328b66f)));
                                        }},
                                        &quote!{()},
                                        &quote!{panic!("2f5f648a")},
                                    )
                                }
                            };
                            for (idx, _) in is_nl_vec.iter().take(is_nl_vec.len().saturating_sub(1)).enumerate() {
                                let is_nl_vec_drvd = is_nl_vec
                                .iter()
                                .take(
                                    is_nl_vec
                                        .len()
                                        .saturating_sub(idx.checked_add(1).expect("75d5ed28")),
                                )
                                .copied()
                                .collect::<Vec<&IsNl>>();
                                let is_nl_vec_drvd_len = is_nl_vec_drvd.len();
                                let is_nl_vec_drvd_len_saturating_sub_one = is_nl_vec_drvd_len.saturating_sub(1);
                                content_ts_acc = {
                                    let start_idx = gen_i(
                                        is_nl_vec_drvd_len_saturating_sub_one,
                                        &once(is_nl)
                                        .chain(
                                            is_nl_vec_drvd
                                                .iter()
                                                .take(is_nl_vec_drvd_len_saturating_sub_one)
                                                .copied(),
                                        ).collect::<Vec<&IsNl>>()
                                    );
                                    let start_idx_incr_by_1 = start_idx.checked_add(1).expect("96e90e72");
                                    match &is_nl_vec_drvd.last().expect("88548240") {
                                        IsNl::False => gen_for_v_i_dot_zero_into_iter_enumerate_ts(
                                            is_nl_vec_drvd_len,
                                            start_idx_incr_by_1,
                                            start_idx,
                                            &content_ts_acc,
                                        ),
                                        IsNl::True => gen_if_let_some_eqs_v_i_dot_zero_ts(
                                            start_idx_incr_by_1,
                                            start_idx,
                                            &gen_for_v_i_dot_zero_into_iter_enumerate_ts(
                                                is_nl_vec_drvd_len,
                                                start_idx.checked_add(2).expect("00da046c"),
                                                start_idx_incr_by_1,
                                                &content_ts_acc,
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
                                    &content_ts_acc
                                ),
                                IsNl::True => gen_if_let_some_ts(
                                    &gen_v_nbr_ts(0),
                                    &cr_dot_zero_ts,
                                    &gen_for_v_i_dot_zero_into_iter_enumerate_ts(
                                        0,
                                        1,
                                        0,
                                        &content_ts_acc
                                    )
                                )
                            }
                        };
                        quote! {
                            Some(#import::NotEmptyUnqVec::try_new({
                                let mut acc_049ff0b3 = Vec::new();
                                #content_ts_outer
                                acc_049ff0b3
                            }).expect("e99ecd08"))
                        }
                    };
                    {
                        let dims_vec = pattern.dims();
                        let dim_i_nbr_max_usize = match dim_i_nbr_max {
                            DimIndexNbr::Zero => 0usize,
                            DimIndexNbr::One => 1usize,
                            DimIndexNbr::Two => 2usize,
                            DimIndexNbr::Three => 3usize,
                        };
                        let take_count = dim_i_nbr_max_usize.checked_add(1).expect("a4b5c6d7");
                        if dims_vec.is_empty() || take_count > dims_vec.len() {
                            quote!{#NoneTs}
                        } else {
                            let dims_refs: Vec<&IsNl> = dims_vec.iter().take(take_count).collect();
                            gen_dim_i_nbr_ts(&dims_refs)
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
            let wh_len_ref_ts: &dyn ToTokens = match &is_nl {
                IsNl::False => &cr_dot_zero_dot_zero,
                IsNl::True => &quote! {v_wh_nl_some.0},
            };
            let cr_into_pg_json_opt_vec_wh_len_eq_ts = match &pattern {
                Pattern::Stdrt => quote!{#NoneTs},
                Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => gen_not_empty_unq_vec_try_new_match_ts(
                    &{
                        let len_eq_ts = quote! {
                            ::LenEq(
                                wh_flts::PgJsonWhLenEq {
                                    oprtr: #import::Oprtr::Or,
                                    #VSc: pg_crud_cmn::UnsignedPartOfI32::try_from(
                                        i32::try_from(#wh_len_ref_ts.len()).expect("64d3424f")
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
                Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => gen_not_empty_unq_vec_try_new_match_ts(
                    &{
                        let len_gt_ts = quote! {
                            ::LenGreaterThan(
                                wh_flts::PgJsonWhLenGreaterThan {
                                    oprtr: #import::Oprtr::Or,
                                    #VSc: if let Ok(v_762dae1f) = pg_crud_cmn::UnsignedPartOfI32::try_from(
                                        if let Ok(v_9dca0200) = i32::try_from(
                                            //todo temp code. make it better checking all cases
                                            match #wh_len_ref_ts.len().checked_sub(1) {
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
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True)
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
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True)
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
            let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts = if matches!(&is_stdrt_nn, IsStdrtNn::True)
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
            let opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts: Option<Ts2> = matches!((&is_stdrt_nn, &pg_json), (IsStdrtNn::True, PgJson::StringAsJsonbString)).then(||
                gen_not_empty_unq_vec_try_new_match_ts(
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
            );
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
            let opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts: Option<Ts2> = match &pattern {
                Pattern::ArrDim1 { dim1_is_nl } => {
                    matches!((&is_nl, dim1_is_nl, &pg_json), (IsNl::False, IsNl::False, PgJson::StringAsJsonbString)).then(||
                        gen_not_empty_unq_vec_try_new_match_ts(
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
                    )
                },
                Pattern::Stdrt |
                Pattern::ArrDim2 {..} |
                Pattern::ArrDim3 {..} |
                Pattern::ArrDim4 {..} => None
            };
            gen_impl_pg_json_test_cases_for_ident_ts(
                &quote! {
                    #[cfg(feature = "test-utils")]
                    #[allow(clippy::question_mark)] // generated test-case expansions prefer explicit branches over `?`
                },
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
                matches!(&pattern, Pattern::ArrDim4{..}).then(|| {
                    let r: &dyn ToTokens = &rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts;
                    r
                }),
                &cr_into_pg_json_opt_vec_wh_len_eq_ts,
                &cr_into_pg_json_opt_vec_wh_len_greater_than_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts,
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts,
                opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts.as_ref().map(|v| {let r: &dyn ToTokens = v; r}),
                &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts,
                opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts.as_ref().map(|v| {let r: &dyn ToTokens = v; r}),
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
    let parse_strs_to_ts2_vec = pg_crud_macros_cmn::parse_strs_to_ts2_vec;
    mb_write_ts_into_file(
        config.pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json,
        "pg_tbl_cols_using_pg_json",
        &{
            let fields_cnt_ts = parse_strs_to_ts2_vec(fields_ts, "1d8cd8e4");
            quote! {
                pub struct PgTblColsUsingPgJsons {
                    #(#fields_cnt_ts)*
                }
            }
        },
        &FormatWithCargofmt::True,
    );
    let generated = {
        let content_ts = parse_strs_to_ts2_vec(pg_json_arr, "84e21b40");
        let mod_name_ts = config.mod_name.as_ref().map_or_else(
            || quote! { #GenPgJsonModSc },
            |name| name.parse::<Ts2>().expect("c7a3f1b2"),
        );
        pg_crud_macros_cmn::gen_mod_with_pub_use_ts(&mod_name_ts, &content_ts)
    };
    mb_write_ts_into_file(
        config.whole_cnt_write_into_gen_pg_json,
        "gen_pg_json",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
