use enum_extension_lib::EnumExtension;
use gen_quotes::dq_ts;
use macros_helpers::{
    DeriveCopy, DeriveSchemarsJsonSchema, FormatWithCargofmt, ShouldWriteTokenStreamIntoFile,
    StructOrEnumDeriveTsStreamBuilder, gen_impl_display_ts, gen_impl_from_ts,
    gen_impl_to_err_string_ts, gen_pub_const_new_ts, gen_pub_new_ts, maybe_write_ts_into_file,
};
use naming::{
    ArrOfUcc, AsUcc, BooleanUcc, ColumnFieldSc, CreateForQueryUcc, CreateSc, EqualUcc, ErSc,
    GenPgJsonTypesModSc, IncrSc, JsonbSetAccumulatorSc, NbrUcc, NewSc, OptUpdateSc, OptVecCreateSc,
    PgJsonTypeUcc, QuerySc, ReadInnerUcc, ReadOnlyIdsMergedWithCreateIntoReadSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc, ReadOnlyIdsSc,
    ReadOnlyIdsToTwoDimalVecReadInnerSc, ReadSc, SelfSc, SelfUcc, StringUcc, UpdateForQueryUcc,
    UpdateUcc, ValueSc, VecOfUcc,
    parameter::{
        JsonbSelfUcc, SelfCreateForQueryUcc, SelfCreateUcc, SelfOriginUcc, SelfReadInnerUcc,
        SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc, SelfTableTypeUcc, SelfUpdateForQueryUcc,
        SelfUpdateUcc, SelfWhereUcc,
    },
};
use panic_location::panic_location;
use pg_crud_macros_common::{
    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, Dim, DimIndexNbr, ImportPath, IsNullable,
    IsQueryBindMutable, IsSelectOnlyCreatedIdsQueryBindMutable,
    IsSelectOnlyUpdatedIdsQueryBindMutable, IsSelectQueryPartColumnFieldForErMessageUsed,
    IsSelectQueryPartIsPgTypeUsed, IsSelectQueryPartSelfSelectUsed, IsStandartNotNull,
    IsUpdateQueryBindMutable, IsUpdateQueryPartJsonbSetTargetUsed, IsUpdateQueryPartSelfUpdateUsed,
    PgFilter, PgJsonTypeFilter, ReadOrUpdate, ShouldDeriveSchemarsJsonSchema,
    ShouldDeriveUtoipaToSchema, gen_impl_crate_is_string_empty_for_ident_ts,
    gen_impl_pg_crud_common_default_opt_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts,
    gen_impl_pg_json_type_test_cases_for_ident_ts, gen_impl_pg_json_type_ts,
    gen_impl_sqlx_encode_sqlx_pg_for_ident_ts, gen_impl_sqlx_type_for_ident_ts,
    gen_opt_type_decl_ts, gen_pg_type_where_ts, gen_sqlx_types_json_type_declaration_ts,
    gen_value_init_ts, gen_vec_tokens_declaration_ts,
};
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
    AllowClippyArbitrarySourceItemOrdering, Bool, F32, F64, I8, I16, I32, I64, MustUse,
    PgCrudCommonDefaultOptSomeVecOneEl, PgCrudCommonDefaultOptSomeVecOneElCall,
    PgCrudCommonDefaultOptSomeVecOneElMaxPageSizeCall, StringTs, U8, U16, U32, U64, UuidUuid,
};
#[must_use]
pub fn gen_pg_json_types(input_ts: &Ts2) -> Ts2 {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Display)]
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
    impl From<&PgJsonType> for RustTypeName {
        fn from(v: &PgJsonType) -> Self {
            match &v {
                PgJsonType::I8AsJsonbNbr => Self::I8,
                PgJsonType::I16AsJsonbNbr => Self::I16,
                PgJsonType::I32AsJsonbNbr => Self::I32,
                PgJsonType::I64AsJsonbNbr => Self::I64,
                PgJsonType::U8AsJsonbNbr => Self::U8,
                PgJsonType::U16AsJsonbNbr => Self::U16,
                PgJsonType::U32AsJsonbNbr => Self::U32,
                PgJsonType::U64AsJsonbNbr => Self::U64,
                PgJsonType::F32AsJsonbNbr => Self::F32,
                PgJsonType::F64AsJsonbNbr => Self::F64,
                PgJsonType::BoolAsJsonbBoolean => Self::Bool,
                PgJsonType::StringAsJsonbString => Self::String,
                PgJsonType::UuidUuidAsJsonbString => Self::UuidUuid,
            }
        }
    }
    #[derive(Debug)]
    enum PgJsonTypeName {
        Boolean,
        Nbr,
        String,
    }
    impl Display for PgJsonTypeName {
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
    impl From<&PgJsonType> for PgJsonTypeName {
        fn from(v: &PgJsonType) -> Self {
            match &v {
                PgJsonType::I8AsJsonbNbr
                | PgJsonType::I16AsJsonbNbr
                | PgJsonType::I32AsJsonbNbr
                | PgJsonType::I64AsJsonbNbr
                | PgJsonType::U8AsJsonbNbr
                | PgJsonType::U16AsJsonbNbr
                | PgJsonType::U32AsJsonbNbr
                | PgJsonType::U64AsJsonbNbr
                | PgJsonType::F32AsJsonbNbr
                | PgJsonType::F64AsJsonbNbr => Self::Nbr,
                PgJsonType::BoolAsJsonbBoolean => Self::Boolean,
                PgJsonType::StringAsJsonbString | PgJsonType::UuidUuidAsJsonbString => Self::String,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter, EnumExtension,
    )]
    enum PgJsonType {
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
    impl ToTokens for PgJsonType {
        fn to_tokens(&self, tokens: &mut Ts2) {
            self.to_string()
                .parse::<Ts2>()
                .expect("eb6cafe0")
                .to_tokens(tokens);
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter, EnumExtension,
    )]
    enum Pattern {
        Standart,
        ArrDim1 {
            dim1_is_nullable: IsNullable,
        },
        ArrDim2 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
        },
        ArrDim3 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
            dim3_is_nullable: IsNullable,
        },
        ArrDim4 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
            dim3_is_nullable: IsNullable,
            dim4_is_nullable: IsNullable,
        },
    }
    impl Pattern {
        const fn down_by_1(&self) -> Option<Self> {
            match &self {
                Self::Standart => None,
                Self::ArrDim1 { .. } => Some(Self::Standart),
                Self::ArrDim2 {
                    dim2_is_nullable, ..
                } => Some(Self::ArrDim1 {
                    dim1_is_nullable: *dim2_is_nullable,
                }),
                Self::ArrDim3 {
                    dim2_is_nullable,
                    dim3_is_nullable,
                    ..
                } => Some(Self::ArrDim2 {
                    dim1_is_nullable: *dim2_is_nullable,
                    dim2_is_nullable: *dim3_is_nullable,
                }),
                Self::ArrDim4 {
                    dim2_is_nullable,
                    dim3_is_nullable,
                    dim4_is_nullable,
                    ..
                } => Some(Self::ArrDim3 {
                    dim1_is_nullable: *dim2_is_nullable,
                    dim2_is_nullable: *dim3_is_nullable,
                    dim3_is_nullable: *dim4_is_nullable,
                }),
            }
        }
        const fn down_by_2(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrDim1 { .. } => None,
                Self::ArrDim2 { .. } => Some(Self::Standart),
                Self::ArrDim3 {
                    dim3_is_nullable, ..
                } => Some(Self::ArrDim1 {
                    dim1_is_nullable: *dim3_is_nullable,
                }),
                Self::ArrDim4 {
                    dim3_is_nullable,
                    dim4_is_nullable,
                    ..
                } => Some(Self::ArrDim2 {
                    dim1_is_nullable: *dim3_is_nullable,
                    dim2_is_nullable: *dim4_is_nullable,
                }),
            }
        }
        const fn down_by_3(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrDim1 { .. } | Self::ArrDim2 { .. } => None,
                Self::ArrDim3 { .. } => Some(Self::Standart),
                Self::ArrDim4 {
                    dim4_is_nullable, ..
                } => Some(Self::ArrDim1 {
                    dim1_is_nullable: *dim4_is_nullable,
                }),
            }
        }
        const fn down_by_4(&self) -> Option<Self> {
            match &self {
                Self::Standart
                | Self::ArrDim1 { .. }
                | Self::ArrDim2 { .. }
                | Self::ArrDim3 { .. } => None,
                Self::ArrDim4 { .. } => Some(Self::Standart),
            }
        }
    }
    enum ArrDim {
        ArrDim1,
        ArrDim2 {
            dim1_is_nullable: IsNullable,
        },
        ArrDim3 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
        },
        ArrDim4 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
            dim3_is_nullable: IsNullable,
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
                Pattern::Standart => Err(()),
                Pattern::ArrDim1 { .. } => Ok(Self::ArrDim1),
                Pattern::ArrDim2 {
                    dim1_is_nullable, ..
                } => Ok(Self::ArrDim2 {
                    dim1_is_nullable: *dim1_is_nullable,
                }),
                Pattern::ArrDim3 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    ..
                } => Ok(Self::ArrDim3 {
                    dim1_is_nullable: *dim1_is_nullable,
                    dim2_is_nullable: *dim2_is_nullable,
                }),
                Pattern::ArrDim4 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                    ..
                } => Ok(Self::ArrDim4 {
                    dim1_is_nullable: *dim1_is_nullable,
                    dim2_is_nullable: *dim2_is_nullable,
                    dim3_is_nullable: *dim3_is_nullable,
                }),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct Record {
        pg_json_type: PgJsonType,
        is_nullable: IsNullable,
        pattern: Pattern,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize)]
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
    #[derive(Debug, Deserialize)]
    struct GenPgJsonTypesConfig {
        pg_table_columns_content_write_into_pg_table_columns_using_pg_json_types:
            ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_pg_json_types: ShouldWriteTokenStreamIntoFile,
        vrt: ConfigVrt,
    }
    panic_location();
    let config = from_str::<GenPgJsonTypesConfig>(&input_ts.to_string()).expect("1123f78f");
    let (fields_ts, pg_json_type_arr) = {
        let acc_d97120ed = {
            let gen_vrts = |max_dim: Option<i32>|{
                PgJsonType::into_arr().into_iter().fold(Vec::new(), |mut acc, pg_json_type| {
                    for pattern in Pattern::into_arr() {
                        let include = match max_dim {
                            None => true,
                            Some(0i32) => matches!(pattern, Pattern::Standart),
                            Some(v_b6fece15) => {
                                let dim = match pattern {
                                    Pattern::Standart => 0i32,
                                    Pattern::ArrDim1 { .. } => 1i32,
                                    Pattern::ArrDim2 { .. } => 2i32,
                                    Pattern::ArrDim3 { .. } => 3i32,
                                    Pattern::ArrDim4 { .. } => 4i32,
                                };
                                dim <= v_b6fece15
                            }
                        };
                        if include {
                            match pattern {
                                Pattern::Standart => {
                                    for is_nullable in IsNullable::into_arr() {
                                        acc.push(Record {
                                            pg_json_type: pg_json_type.clone(),
                                            is_nullable,
                                            pattern: Pattern::Standart,
                                        });
                                    }
                                }
                                Pattern::ArrDim1 { .. } => {
                                    for is_nullable in IsNullable::into_arr() {
                                        for dim1_is_nullable in IsNullable::into_arr() {
                                            acc.push(Record {
                                                pg_json_type: pg_json_type.clone(),
                                                is_nullable,
                                                pattern: Pattern::ArrDim1 {
                                                    dim1_is_nullable,
                                                },
                                            });
                                        }
                                    }
                                }
                                Pattern::ArrDim2 { .. } => {
                                    for is_nullable in IsNullable::into_arr() {
                                        for dim1_is_nullable in IsNullable::into_arr() {
                                            for dim2_is_nullable in IsNullable::into_arr() {
                                                acc.push(Record {
                                                    pg_json_type: pg_json_type.clone(),
                                                    is_nullable,
                                                    pattern: Pattern::ArrDim2 {
                                                        dim1_is_nullable,
                                                        dim2_is_nullable,
                                                    },
                                                });
                                            }
                                        }
                                    }
                                }
                                Pattern::ArrDim3 { .. } => {
                                    for is_nullable in IsNullable::into_arr() {
                                        for dim1_is_nullable in IsNullable::into_arr() {
                                            for dim2_is_nullable in IsNullable::into_arr() {
                                                for dim3_is_nullable in IsNullable::into_arr() {
                                                    acc.push(Record {
                                                        pg_json_type: pg_json_type.clone(),
                                                        is_nullable,
                                                        pattern: Pattern::ArrDim3 {
                                                            dim1_is_nullable,
                                                            dim2_is_nullable,
                                                            dim3_is_nullable,
                                                        },
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                Pattern::ArrDim4 { .. } => {
                                    for is_nullable in IsNullable::into_arr() {
                                        for dim1_is_nullable in IsNullable::into_arr() {
                                            for dim2_is_nullable in IsNullable::into_arr() {
                                                for dim3_is_nullable in IsNullable::into_arr() {
                                                    for dim4_is_nullable in IsNullable::into_arr() {
                                                        acc.push(Record {
                                                            pg_json_type: pg_json_type.clone(),
                                                            is_nullable,
                                                            pattern: Pattern::ArrDim4 {
                                                                dim1_is_nullable,
                                                                dim2_is_nullable,
                                                                dim3_is_nullable,
                                                                dim4_is_nullable,
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
            acc_d97120ed
                .iter()
                .all(|el_8ef63a77| seen.insert(el_8ef63a77)),
            "c2d37017"
        );
        acc_d97120ed
    }.into_iter().fold(Vec::new(), |mut acc_5e43269c, el_c4f9bf8f| {
        for el_7ae8d2ae in {
            #[derive(Clone)]
            struct RecordHandle {
                is_nullable: IsNullable,
                pattern: Pattern,
            }
            fn gen_record_handle_vec(record_handle: RecordHandle) -> Vec<RecordHandle> {
                let gen_vec = |record_handle_1e4b61e4: RecordHandle| gen_record_handle_vec(
                    record_handle_1e4b61e4
                ).into_iter().chain(once(record_handle.clone())).collect();
                match (&record_handle.is_nullable, &record_handle.pattern) {
                    (IsNullable::False, Pattern::Standart) => vec![record_handle],
                    (IsNullable::True, Pattern::Standart) => gen_vec(RecordHandle {
                        is_nullable: IsNullable::False,
                        pattern: Pattern::Standart,
                    }),
                    (IsNullable::False, Pattern::ArrDim1 { dim1_is_nullable }) => gen_vec(RecordHandle {
                        is_nullable: *dim1_is_nullable,
                        pattern: record_handle.pattern.down_by_1().expect("0e970a4f"),
                    }),
                    (IsNullable::False, Pattern::ArrDim2 { dim1_is_nullable, .. }) => gen_vec(RecordHandle {
                        is_nullable: *dim1_is_nullable,
                        pattern: record_handle.pattern.down_by_1().expect("85f8ed83"),
                    }),
                    (
                        IsNullable::False,
                        Pattern::ArrDim3 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                        },
                    ) => gen_vec(RecordHandle {
                        is_nullable: *dim1_is_nullable,
                        pattern: Pattern::ArrDim2 {
                            dim1_is_nullable: *dim2_is_nullable,
                            dim2_is_nullable: *dim3_is_nullable,
                        },
                    }),
                    (
                        IsNullable::False,
                        Pattern::ArrDim4 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                            dim4_is_nullable,
                        },
                    ) => gen_vec(RecordHandle {
                        is_nullable: *dim1_is_nullable,
                        pattern: Pattern::ArrDim3 {
                            dim1_is_nullable: *dim2_is_nullable,
                            dim2_is_nullable: *dim3_is_nullable,
                            dim3_is_nullable: *dim4_is_nullable,
                        },
                    }),
                    (IsNullable::True, Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. }) => gen_vec(RecordHandle {
                        is_nullable: IsNullable::False,
                        pattern: record_handle.pattern.clone(),
                    }),
                }
            }
            gen_record_handle_vec(RecordHandle {
                is_nullable: el_c4f9bf8f.is_nullable,
                pattern: el_c4f9bf8f.pattern,
            })
        } {
            let record = Record {
                pg_json_type: el_c4f9bf8f.pg_json_type.clone(),
                is_nullable: el_7ae8d2ae.is_nullable,
                pattern: el_7ae8d2ae.pattern,
            };
            if !acc_5e43269c.contains(&record) {
                acc_5e43269c.push(record);
            }
        }
        acc_5e43269c
    })
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, Record)>>()
    .par_iter()
    // .into_iter() //just for console prints ordering
    .map(|(index, el_1d376874)| {
        enum IsStandartNotNullUuid {
            False,
            True,
        }
        enum ConstFn {
            False,
            True,
        }
        let pg_json_type = &el_1d376874.pg_json_type;
        let is_nullable = &el_1d376874.is_nullable;
        let pattern = &el_1d376874.pattern;
        let rust_type_name = RustTypeName::from(pg_json_type);
        let pg_json_type_name = PgJsonTypeName::from(pg_json_type);
        let is_standart_not_null = if matches!((&pattern, &is_nullable), (Pattern::Standart, IsNullable::False)) {
            IsStandartNotNull::True
        } else {
            IsStandartNotNull::False
        };
        let is_standart_not_null_uuid = if matches!((&is_nullable, &pattern, &pg_json_type), (IsNullable::False, Pattern::Standart, PgJsonType::UuidUuidAsJsonbString)) {
            IsStandartNotNullUuid::True
        } else {
            IsStandartNotNullUuid::False
        };
        let import_path = ImportPath::PgCrudCommon;
        let none_ts = quote! {None};
        let gen_import_path_value_init_ts = |content_ts: &dyn ToTokens| gen_value_init_ts(&import_path, &content_ts);
        let gen_ident_ts = |is_nullable_ddf79d44: &IsNullable, pattern_2c09ee59: &Pattern| {
            let is_nullable_rust = is_nullable_ddf79d44.rust();
            let (rust_part, pg_part) = match &pattern_2c09ee59 {
                Pattern::Standart => (rust_type_name.to_string(), pg_json_type_name.to_string()),
                Pattern::ArrDim1 { dim1_is_nullable } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    (format!("{VecOfUcc}{d1_rust}{rust_type_name}"), format!("{ArrOfUcc}{d1}{pg_json_type_name}"))
                }
                Pattern::ArrDim2 { dim1_is_nullable, dim2_is_nullable } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    let d2 = dim2_is_nullable.not_null_or_nullable_str();
                    let d2_rust = dim2_is_nullable.rust();
                    (format!("{VecOfUcc}{d1_rust}{VecOfUcc}{d2_rust}{rust_type_name}"), format!("{ArrOfUcc}{d1}{ArrOfUcc}{d2}{pg_json_type_name}"))
                }
                Pattern::ArrDim3 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    let d2 = dim2_is_nullable.not_null_or_nullable_str();
                    let d2_rust = dim2_is_nullable.rust();
                    let d3 = dim3_is_nullable.not_null_or_nullable_str();
                    let d3_rust = dim3_is_nullable.rust();
                    (
                        format!("{VecOfUcc}{d1_rust}{VecOfUcc}{d2_rust}{VecOfUcc}{d3_rust}{rust_type_name}"),
                        format!("{ArrOfUcc}{d1}{ArrOfUcc}{d2}{ArrOfUcc}{d3}{pg_json_type_name}"),
                    )
                }
                Pattern::ArrDim4 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                    dim4_is_nullable,
                } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    let d2 = dim2_is_nullable.not_null_or_nullable_str();
                    let d2_rust = dim2_is_nullable.rust();
                    let d3 = dim3_is_nullable.not_null_or_nullable_str();
                    let d3_rust = dim3_is_nullable.rust();
                    let d4 = dim4_is_nullable.not_null_or_nullable_str();
                    let d4_rust = dim4_is_nullable.rust();
                    (
                        format!("{VecOfUcc}{d1_rust}{VecOfUcc}{d2_rust}{VecOfUcc}{d3_rust}{VecOfUcc}{d4_rust}{rust_type_name}"),
                        format!("{ArrOfUcc}{d1}{ArrOfUcc}{d2}{ArrOfUcc}{d3}{ArrOfUcc}{d4}{pg_json_type_name}"),
                    )
                }
            };
            let not_null_or_nullable_str = is_nullable_ddf79d44.not_null_or_nullable_str();
            format!("{is_nullable_rust}{rust_part}{AsUcc}{not_null_or_nullable_str}{pg_part}").parse::<Ts2>().expect("998d1471")
        };
        let ident = &gen_ident_ts(is_nullable, pattern);
        let ident_standart_not_null_ucc = &gen_ident_ts(&IsNullable::False, &Pattern::Standart);
        let ident_standart_not_null_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident);
        let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
        let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
        let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
        let ident_not_null_ts = gen_ident_ts(&IsNullable::False, pattern);
        let ident_ts = {
            let ident_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .build_struct(
                    &ident,
                    &Ts2::new(),
                    &quote!{;}
                );
            quote! {
                #ident_ts
            }
        };
        let ident_standart_not_null_origin_ucc = SelfOriginUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_origin_ucc = SelfOriginUcc::from_tokens(&ident);
        let ident_read_inner_standart_not_null_al_ts = {
            let content_ts: &dyn ToTokens = match &pg_json_type {
                PgJsonType::I8AsJsonbNbr => &I8,
                PgJsonType::I16AsJsonbNbr => &I16,
                PgJsonType::I32AsJsonbNbr => &I32,
                PgJsonType::I64AsJsonbNbr => &I64,
                PgJsonType::U8AsJsonbNbr => &U8,
                PgJsonType::U16AsJsonbNbr => &U16,
                PgJsonType::U32AsJsonbNbr => &U32,
                PgJsonType::U64AsJsonbNbr => &U64,
                PgJsonType::F32AsJsonbNbr => &F32,
                PgJsonType::F64AsJsonbNbr => &F64,
                PgJsonType::BoolAsJsonbBoolean => &Bool,
                PgJsonType::StringAsJsonbString => &StringTs,
                PgJsonType::UuidUuidAsJsonbString => &UuidUuid,
            };
            quote! {#content_ts}
        };
        let ident_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident);
        let value_ident_read_inner_ts = quote! {#ValueSc: #ident_read_inner_ucc};
        let gen_pub_fn_new_value_ident_read_inner_content_ts = |content_ts: &dyn ToTokens| gen_pub_new_ts(
            &MustUse,
            &value_ident_read_inner_ts,
            &content_ts
        );
        let gen_pub_const_fn_new_value_ident_read_inner_content_ts = |content_ts: &dyn ToTokens| gen_pub_const_new_ts(
            &MustUse,
            &value_ident_read_inner_ts,
            &content_ts
        );
        let self_ident_origin_new_value_ts = quote! {Self(#ident_origin_ucc::new(#ValueSc))};
        let maybe_const_fn = match &pattern {
            Pattern::Standart => match &is_nullable {
                IsNullable::False => ConstFn::True,
                IsNullable::True => ConstFn::False,
            },
            Pattern::ArrDim1 { .. } |
            Pattern::ArrDim2 { .. } |
            Pattern::ArrDim3 { .. } |
            Pattern::ArrDim4 { .. } => ConstFn::False,
        };
        let gen_pub_new_or_fn_new_ts = |const_new_ts: &dyn ToTokens, new_ts: &dyn ToTokens|match maybe_const_fn {
            ConstFn::False => gen_pub_fn_new_value_ident_read_inner_content_ts(
                &new_ts
            ),
            ConstFn::True => gen_pub_const_fn_new_value_ident_read_inner_content_ts(
                &const_new_ts
            ),
        };
        let pub_new_or_const_new_self_ident_origin_new_value_ts = gen_pub_new_or_fn_new_ts(
            &self_ident_origin_new_value_ts,
            &self_ident_origin_new_value_ts
        );
        let ident_create_for_query_ucc = SelfCreateForQueryUcc::from_tokens(&ident);
        let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
        let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
        let maybe_derive_copy = match &pattern {
            Pattern::Standart => match &pg_json_type {
                PgJsonType::I8AsJsonbNbr |
                PgJsonType::I16AsJsonbNbr |
                PgJsonType::I32AsJsonbNbr |
                PgJsonType::I64AsJsonbNbr |
                PgJsonType::U8AsJsonbNbr |
                PgJsonType::U16AsJsonbNbr |
                PgJsonType::U32AsJsonbNbr |
                PgJsonType::U64AsJsonbNbr |
                PgJsonType::F32AsJsonbNbr |
                PgJsonType::F64AsJsonbNbr |
                PgJsonType::BoolAsJsonbBoolean |
                PgJsonType::UuidUuidAsJsonbString => DeriveCopy::True,
                PgJsonType::StringAsJsonbString => DeriveCopy::False,
            },
            Pattern::ArrDim1 {..} |
            Pattern::ArrDim2 {..} |
            Pattern::ArrDim3 {..} |
            Pattern::ArrDim4 {..} => DeriveCopy::False,
        };
        let ident_origin_ts = {
            let gen_ident_origin_non_wrapping_6c0934a6 = |
                is_nullable_e7d1d83c: &IsNullable,
                pattern_1ca83c6c: &Pattern
            | SelfOriginUcc::from_tokens(&gen_ident_ts(is_nullable_e7d1d83c, pattern_1ca83c6c));
            let ident_origin_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_partial_ord()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema_if(
                    if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                        match &pg_json_type {
                            PgJsonType::UuidUuidAsJsonbString => DeriveSchemarsJsonSchema::False,
                            PgJsonType::I8AsJsonbNbr
                            | PgJsonType::I16AsJsonbNbr
                            | PgJsonType::I32AsJsonbNbr
                            | PgJsonType::I64AsJsonbNbr
                            | PgJsonType::U8AsJsonbNbr
                            | PgJsonType::U16AsJsonbNbr
                            | PgJsonType::U32AsJsonbNbr
                            | PgJsonType::U64AsJsonbNbr
                            | PgJsonType::F32AsJsonbNbr
                            | PgJsonType::F64AsJsonbNbr
                            | PgJsonType::BoolAsJsonbBoolean
                            | PgJsonType::StringAsJsonbString => DeriveSchemarsJsonSchema::True,
                        }
                    } else {
                        DeriveSchemarsJsonSchema::True
                    }
                )
                .build_struct(
                    &ident_origin_ucc,
                    &Ts2::new(),
                    &{
                        let content_ts: &dyn ToTokens = {
                            let gen_ident_origin_6f054930 = |is_nullable_70fb22e6: &IsNullable, pattern_042c1c1d: &Pattern| {
                                let value = gen_ident_origin_non_wrapping_6c0934a6(is_nullable_70fb22e6, pattern_042c1c1d);
                                match &is_nullable {
                                    IsNullable::False => gen_vec_tokens_declaration_ts(&value),
                                    IsNullable::True => gen_opt_type_decl_ts(&value),
                                }
                            };
                            let gen_dims_ts = |dim1_is_nullable_a5c667cd: &IsNullable|{
                                let (is_nullable_79be16e9, pattern_3437adad): (&IsNullable, &Pattern) = match &is_nullable {
                                    IsNullable::False => (dim1_is_nullable_a5c667cd, &pattern.down_by_1().expect("e994797d")),
                                    IsNullable::True => (&IsNullable::False, pattern),
                                };
                                gen_ident_origin_6f054930(is_nullable_79be16e9, pattern_3437adad)
                            };
                            match &pattern {
                                Pattern::Standart => match &is_nullable {
                                    IsNullable::False => &ident_read_inner_standart_not_null_al_ts,
                                    IsNullable::True => &gen_opt_type_decl_ts(&ident_standart_not_null_origin_ucc),
                                },
                                Pattern::ArrDim1 { dim1_is_nullable } |
                                Pattern::ArrDim2 { dim1_is_nullable, .. } |
                                Pattern::ArrDim3 { dim1_is_nullable, .. } |
                                Pattern::ArrDim4 { dim1_is_nullable, .. } => &gen_dims_ts(dim1_is_nullable),
                            }
                        };
                        quote!{(#content_ts);}
                    }
                );
            let ident_origin_impl_new_self_content_ts = {
                let gen_value_map_type_new_ts = |type_ts: &dyn ToTokens| quote! {#ValueSc.map(#type_ts::#NewSc)};
                let gen_arr_dims_init_ts = |type_ts: &dyn ToTokens| match &is_nullable {
                    IsNullable::False => quote! {#ValueSc.into_iter().map(#type_ts::#NewSc).collect()},
                    IsNullable::True => gen_value_map_type_new_ts(&type_ts),
                };
                match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => quote! {value},
                        IsNullable::True => gen_value_map_type_new_ts(&ident_standart_not_null_origin_ucc),
                    },
                    Pattern::ArrDim1 { dim1_is_nullable } => gen_arr_dims_init_ts(&{
                        let (pattern_38178717, is_nullable_b0d116f8): (&Pattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pattern.down_by_1().expect("1160d3df"), dim1_is_nullable),
                            IsNullable::True => (pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_b0d116f8, pattern_38178717)
                    }),
                    Pattern::ArrDim2 { dim1_is_nullable, .. } => gen_arr_dims_init_ts(&{
                        let (pattern_8e2a682a, is_nullable_c378003c): (&Pattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pattern.down_by_1().expect("8ab62f4e"), dim1_is_nullable),
                            IsNullable::True => (pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_c378003c, pattern_8e2a682a)
                    }),
                    Pattern::ArrDim3 { dim1_is_nullable, .. } => gen_arr_dims_init_ts(&{
                        let (pattern_305989a9, is_nullable_4a8825a3): (&Pattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pattern.down_by_1().expect("ed64919d"), dim1_is_nullable),
                            IsNullable::True => (pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_4a8825a3, pattern_305989a9)
                    }),
                    Pattern::ArrDim4 { dim1_is_nullable, .. } => gen_arr_dims_init_ts(&{
                        let (pattern_ea606504, is_nullable_63d0fe05): (&Pattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pattern.down_by_1().expect("25646d29"), dim1_is_nullable),
                            IsNullable::True => (pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_63d0fe05, pattern_ea606504)
                    }),
                }
            };
            let impl_ident_origin_ts = {
                let pub_fn_new_ts = {
                    let self_ident_origin_impl_new_self_content_ts = quote!{
                        Self(#ident_origin_impl_new_self_content_ts)
                    };
                    gen_pub_new_or_fn_new_ts(
                        &self_ident_origin_impl_new_self_content_ts,
                        &self_ident_origin_impl_new_self_content_ts
                    )
                };
                quote! {
                    impl #ident_origin_ucc {
                        #pub_fn_new_ts
                    }
                }
            };
            let impl_from_ident_create_for_ident_origin_ts = gen_impl_from_ts(&ident_create_ucc, &ident_origin_ucc, &quote! {#ValueSc.0});
            let impl_from_ident_update_for_ident_origin_ts = gen_impl_from_ts(&ident_update_ucc, &ident_origin_ucc, &quote! {#ValueSc.0});
            //todo
            let maybe_impl_schemars_json_schema_for_ident_origin_ts = if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                match &pg_json_type {
                    PgJsonType::UuidUuidAsJsonbString => {
                        let ident_standart_not_null_origin_dq_ts = dq_ts(
                            &ident_standart_not_null_origin_ucc
                        );
                        let text_ident_standart_not_null_origin_dq_ts = dq_ts(
                            &format!("tests::{ident_standart_not_null_origin_ucc}")
                        );
                        quote!{
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #AllowClippyArbitrarySourceItemOrdering
                            const _: () = {
                                #[automatically_derived]
                                #[allow(unused_braces)]
                                impl schemars::JsonSchema for #ident_standart_not_null_origin_ucc {
                                    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#ident_standart_not_null_origin_dq_ts)
                                    }
                                    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#text_ident_standart_not_null_origin_dq_ts)
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
                    PgJsonType::I8AsJsonbNbr
                    | PgJsonType::I16AsJsonbNbr
                    | PgJsonType::I32AsJsonbNbr
                    | PgJsonType::I64AsJsonbNbr
                    | PgJsonType::U8AsJsonbNbr
                    | PgJsonType::U16AsJsonbNbr
                    | PgJsonType::U32AsJsonbNbr
                    | PgJsonType::U64AsJsonbNbr
                    | PgJsonType::F32AsJsonbNbr
                    | PgJsonType::F64AsJsonbNbr
                    | PgJsonType::BoolAsJsonbBoolean
                    | PgJsonType::StringAsJsonbString => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            let maybe_impl_is_string_empty_for_ident_origin_ts = if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr
                    | PgJsonType::I16AsJsonbNbr
                    | PgJsonType::I32AsJsonbNbr
                    | PgJsonType::I64AsJsonbNbr
                    | PgJsonType::U8AsJsonbNbr
                    | PgJsonType::U16AsJsonbNbr
                    | PgJsonType::U32AsJsonbNbr
                    | PgJsonType::U64AsJsonbNbr
                    | PgJsonType::F32AsJsonbNbr
                    | PgJsonType::F64AsJsonbNbr
                    | PgJsonType::BoolAsJsonbBoolean => Ts2::new(),
                    PgJsonType::StringAsJsonbString => gen_impl_crate_is_string_empty_for_ident_ts(
                        &ident_origin_ucc,
                        &quote!{self.0.clone().is_empty()}
                    ),
                    PgJsonType::UuidUuidAsJsonbString => gen_impl_crate_is_string_empty_for_ident_ts(
                        &ident_origin_ucc,
                        &quote!{self.0.to_string().is_empty()}
                    ),
                }
            } else {
                Ts2::new()
            };
            let impl_display_for_ident_origin_ts = gen_impl_display_ts(&Ts2::new(), &ident_origin_ucc, &Ts2::new(), &quote! {write!(f, "{self:?}")});
            let impl_location_lib_to_err_string_for_ident_origin_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_origin_ucc, &Ts2::new(), &quote! {format!("{self:#?}")});
            let impl_default_opt_some_vec_one_el_for_ident_origin_ts = gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_origin_ucc, &{
                let content_ts = match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => match &pg_json_type {
                            PgJsonType::I8AsJsonbNbr
                            | PgJsonType::I16AsJsonbNbr
                            | PgJsonType::I32AsJsonbNbr
                            | PgJsonType::I64AsJsonbNbr
                            | PgJsonType::U8AsJsonbNbr
                            | PgJsonType::U16AsJsonbNbr
                            | PgJsonType::U32AsJsonbNbr
                            | PgJsonType::U64AsJsonbNbr
                            | PgJsonType::F32AsJsonbNbr
                            | PgJsonType::F64AsJsonbNbr
                            | PgJsonType::BoolAsJsonbBoolean => quote! {Default::default()},
                            PgJsonType::StringAsJsonbString => quote! {String::default()},
                            PgJsonType::UuidUuidAsJsonbString => quote! {uuid::Uuid::new_v4()},
                        },
                        IsNullable::True => quote! {Some(#PgCrudCommonDefaultOptSomeVecOneElCall)},
                    },
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => match &is_nullable {
                        IsNullable::False => quote! {vec![#PgCrudCommonDefaultOptSomeVecOneElCall]},
                        IsNullable::True => quote! {Some(#PgCrudCommonDefaultOptSomeVecOneElCall)},
                    },
                };
                quote! {Self(#content_ts)}
            });
            let impl_sqlx_type_for_ident_origin_ts = gen_impl_sqlx_type_for_ident_ts(&ident_origin_ucc, &gen_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            let impl_sqlx_encode_sqlx_pg_for_ident_origin_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_origin_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            quote! {
                #ident_origin_ts
                #impl_ident_origin_ts
                #impl_from_ident_create_for_ident_origin_ts
                #impl_from_ident_update_for_ident_origin_ts
                #maybe_impl_schemars_json_schema_for_ident_origin_ts
                #maybe_impl_is_string_empty_for_ident_origin_ts
                #impl_display_for_ident_origin_ts
                #impl_location_lib_to_err_string_for_ident_origin_ts
                #impl_default_opt_some_vec_one_el_for_ident_origin_ts
                #impl_sqlx_type_for_ident_origin_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_origin_ts
            }
        };
        let ident_origin_struct_content_ts = quote!{(#ident_origin_ucc);};
        let ident_table_type_ts = {
            let ident_table_type_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_partial_ord()//maybe add it to the trait?
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_table_type_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_table_type_ts = {
                quote!{
                    impl #ident_table_type_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_default_opt_some_vec_one_el_for_ident_table_type_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_table_type_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            //todo maybe add to trait?
            let impl_sqlx_encode_sqlx_pg_for_ident_table_type_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_table_type_ucc, &quote! {&#SelfSc.0});
            //todo maybe add to trait?
            let impl_sqlx_type_for_ident_table_type_ts = gen_impl_sqlx_type_for_ident_ts(&ident_table_type_ucc, &gen_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            quote! {
                #ident_table_type_ts
                #impl_ident_table_type_ts
                #impl_default_opt_some_vec_one_el_for_ident_table_type_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_table_type_ts
                #impl_sqlx_type_for_ident_table_type_ts
            }
        };
        let ident_create_ts = {
            let ident_create_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_create_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_create_ts = {
                quote!{
                    impl #ident_create_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_default_opt_some_vec_one_el_for_ident_create_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_create_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            quote! {
                #ident_create_ts
                #impl_ident_create_ts
                #impl_default_opt_some_vec_one_el_for_ident_create_ts
            }
        };
        let ident_create_for_query_ts = {
            let ident_create_for_query_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &ident_create_for_query_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_create_for_query_ts = {
                quote! {
                    impl #ident_create_for_query_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_sqlx_encode_sqlx_pg_for_ident_create_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_create_for_query_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            let impl_sqlx_type_for_ident_create_for_query_ts = gen_impl_sqlx_type_for_ident_ts(&ident_create_for_query_ucc, &ident_origin_ucc);
            let impl_from_ident_create_for_ident_create_for_query_ts = gen_impl_from_ts(&ident_create_ucc, &ident_create_for_query_ucc, &quote! {Self(#ValueSc.0)});
            let maybe_impl_from_ident_update_for_ident_create_for_query_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                gen_impl_from_ts(&ident_update_ucc, &ident_create_for_query_ucc, &quote! {Self(#ValueSc.0)})
            } else {
                Ts2::new()
            };
            quote! {
                #ident_create_for_query_ts
                #impl_ident_create_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_create_for_query_ts
                #impl_sqlx_type_for_ident_create_for_query_ts
                #impl_from_ident_create_for_ident_create_for_query_ts
                #maybe_impl_from_ident_update_for_ident_create_for_query_ts
            }
        };
        let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
        let ident_select_ts = {
            let ident_select_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_select_ucc,
                    &Ts2::new(),
                    &ArrDim::try_from(pattern).map_or_else(
                        |()| quote! {;},
                        |arr_dim| {
                            let mut arguments_ts = Vec::new();
                            for el_419a74e5 in 1..=arr_dim.to_usize() {
                                let dim_nbr_pagination_ts = format!("dim{el_419a74e5}_pagination").parse::<Ts2>()
                                .expect("2ad1faf7");
                                arguments_ts.push(quote! {
                                    #dim_nbr_pagination_ts: #import_path::PaginationStartsWithZero
                                });
                            }
                            quote! {{#(#arguments_ts),*}}
                        }
                    )
                );
            let gen_default_some_one_content_ts = |default_some_one_or_default_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| {
                let content_ts = ArrDim::try_from(pattern).map_or_else(
                    |()| Ts2::new(),
                    |arr_dim| {
                        let content_ts: &dyn ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudCommonDefaultOptSomeVecOneElCall,
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudCommonDefaultOptSomeVecOneElMaxPageSizeCall,
                        };
                        let mut arguments_ts = Vec::new();
                        for el_d56aec99 in 1..=arr_dim.to_usize() {
                            let dim_nbr_pagination_ts = format!("dim{el_d56aec99}_pagination").parse::<Ts2>().expect("26ca29fb");
                            arguments_ts.push(quote! {
                                #dim_nbr_pagination_ts: #content_ts
                            });
                        }
                        quote! {#(#arguments_ts),*}
                    }
                );
                quote! {Self{#content_ts}}
            };
            let impl_default_opt_some_vec_one_el_for_pg_json_type_ident_select_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_select_ucc, &gen_default_some_one_content_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne));
            let impl_default_opt_some_vec_one_el_max_page_size_for_pg_json_type_ident_select_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_max_page_size_ts(&ident_select_ucc, &gen_default_some_one_content_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize));
            quote! {
                #ident_select_ts
                #impl_default_opt_some_vec_one_el_for_pg_json_type_ident_select_ts
                #impl_default_opt_some_vec_one_el_max_page_size_for_pg_json_type_ident_select_ts
            }
        };
        let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
        let ident_where_ts = match &is_nullable {
            IsNullable::False => gen_pg_type_where_ts(
                &AllowClippyArbitrarySourceItemOrdering,
                &{
                    #[derive(Debug, Clone)]
                    enum PgJsonTypeSpecific {
                        Bool,
                        Nbr,
                        String,
                    }
                    impl From<&PgJsonType> for PgJsonTypeSpecific {
                        fn from(v: &PgJsonType) -> Self {
                            match v {
                                PgJsonType::I8AsJsonbNbr
                                | PgJsonType::I16AsJsonbNbr
                                | PgJsonType::I32AsJsonbNbr
                                | PgJsonType::I64AsJsonbNbr
                                | PgJsonType::U8AsJsonbNbr
                                | PgJsonType::U16AsJsonbNbr
                                | PgJsonType::U32AsJsonbNbr
                                | PgJsonType::U64AsJsonbNbr
                                | PgJsonType::F32AsJsonbNbr
                                | PgJsonType::F64AsJsonbNbr => Self::Nbr,
                                PgJsonType::BoolAsJsonbBoolean => Self::Bool,
                                PgJsonType::StringAsJsonbString | PgJsonType::UuidUuidAsJsonbString => Self::String,
                            }
                        }
                    }
                    let pg_json_type_specific = PgJsonTypeSpecific::from(pg_json_type);
                    let common_pg_json_type_filters = vec![PgJsonTypeFilter::Equal { ident: quote! {#ident_table_type_ucc} }];
                    let ident_table_type_ucc_ts = quote! {#ident_table_type_ucc};
                    match &pattern {
                        Pattern::Standart => {
                            let common_standart_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::In {
                                    ident: ident_table_type_ucc_ts.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Nbr => {
                                    let mut vec = common_standart_pg_json_type_filters;
                                    vec.push(PgJsonTypeFilter::GreaterThan {
                                        ident: ident_table_type_ucc_ts.clone(),
                                    });
                                    vec.push(PgJsonTypeFilter::Between { ident: ident_table_type_ucc_ts });
                                    vec
                                }
                                PgJsonTypeSpecific::Bool => common_standart_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut vec = common_standart_pg_json_type_filters;
                                    vec.push(PgJsonTypeFilter::RegularExpression);
                                    vec
                                }
                            }
                        }
                        Pattern::ArrDim1 { dim1_is_nullable } => {
                            let arr_dim1_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("21eaebaf")));
                                quote! {#value}
                            };
                            let common_arr_dim1_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneContainsAllElementsOfArr {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneOverlapsWithArr {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Nbr => {
                                    let mut filters = common_arr_dim1_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimOneGreaterThan {
                                        ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimOneBetween {
                                        ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::ContainsElGreaterThan {
                                        ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::AllElementsGreaterThan {
                                        ident: arr_dim1_inner_el_ident_table_type_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_arr_dim1_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_arr_dim1_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimOneRegularExpression);
                                    filters.push(PgJsonTypeFilter::ContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::AllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        Pattern::ArrDim2 { dim1_is_nullable, dim2_is_nullable } => {
                            let arr_dim1_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("0c4491c4")));
                                quote! {#value}
                            };
                            let arr_dim2_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim2_is_nullable, &pattern.down_by_2().expect("2d4ee5d4")));
                                quote! {#value}
                            };
                            let common_arr_dim2_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoEqual {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::DimTwoLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoContainsAllElementsOfArr {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoOverlapsWithArr {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneAllElementsEqual {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimTwoIn {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Nbr => {
                                    let mut filters = common_arr_dim2_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimTwoGreaterThan {
                                        ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimTwoBetween {
                                        ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimOneContainsElGreaterThan {
                                        ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimOneAllElementsGreaterThan {
                                        ident: arr_dim2_inner_el_ident_table_type_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_arr_dim2_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_arr_dim2_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimTwoRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimOneContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimOneAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        Pattern::ArrDim3 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                        } => {
                            let arr_dim1_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("3450bef4")));
                                quote! {#value}
                            };
                            let arr_dim2_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim2_is_nullable, &pattern.down_by_2().expect("3c0d10f4")));
                                quote! {#value}
                            };
                            let arr_dim3_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim3_is_nullable, &pattern.down_by_3().expect("9aaf9e82")));
                                quote! {#value}
                            };
                            let common_arr_dim3_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoEqual {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeEqual {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::DimTwoLengthEqual);
                                vec.push(PgJsonTypeFilter::DimThreeLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimThreeLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimThreeContainsAllElementsOfArr {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeOverlapsWithArr {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneAllElementsEqual {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoAllElementsEqual {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimTwoIn {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimThreeIn {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Nbr => {
                                    let mut filters = common_arr_dim3_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimThreeGreaterThan {
                                        ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimThreeBetween {
                                        ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimTwoContainsElGreaterThan {
                                        ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimTwoAllElementsGreaterThan {
                                        ident: arr_dim3_inner_el_ident_table_type_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_arr_dim3_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_arr_dim3_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimThreeRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimTwoContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimTwoAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        Pattern::ArrDim4 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                            dim4_is_nullable,
                        } => {
                            let arr_dim1_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("550d313b")));
                                quote! {#value}
                            };
                            let arr_dim2_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim2_is_nullable, &pattern.down_by_2().expect("7bda1424")));
                                quote! {#value}
                            };
                            let arr_dim3_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim3_is_nullable, &pattern.down_by_3().expect("b43aa5bd")));
                                quote! {#value}
                            };
                            let arr_dim4_inner_el_ident_table_type_ucc = {
                                let value = SelfTableTypeUcc::from_tokens(&gen_ident_ts(dim4_is_nullable, &pattern.down_by_4().expect("a246885a")));
                                quote! {#value}
                            };
                            let common_arr_dim4_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoEqual {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeEqual {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimFourEqual {
                                    ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::DimTwoLengthEqual);
                                vec.push(PgJsonTypeFilter::DimThreeLengthEqual);
                                vec.push(PgJsonTypeFilter::DimFourLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimThreeLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimFourLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimFourContainsAllElementsOfArr {
                                    ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimFourOverlapsWithArr {
                                    ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneAllElementsEqual {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoAllElementsEqual {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeAllElementsEqual {
                                    ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: arr_dim1_inner_el_ident_table_type_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimTwoIn {
                                    ident: arr_dim2_inner_el_ident_table_type_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimThreeIn {
                                    ident: arr_dim3_inner_el_ident_table_type_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimFourIn {
                                    ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Nbr => {
                                    let mut filters = common_arr_dim4_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimFourGreaterThan {
                                        ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimFourBetween {
                                        ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimThreeContainsElGreaterThan {
                                        ident: arr_dim4_inner_el_ident_table_type_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimThreeAllElementsGreaterThan {
                                        ident: arr_dim4_inner_el_ident_table_type_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_arr_dim4_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_arr_dim4_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimFourRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimThreeContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimThreeAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                    }
                }
                .iter()
                .map(|el_f992f593| {
                    let handle: &dyn PgFilter = el_f992f593;
                    handle
                })
                .collect(),
                &ident,
                &ShouldDeriveUtoipaToSchema::True,
                &ShouldDeriveSchemarsJsonSchema::True,
                &IsQueryBindMutable::False,
            ),
            IsNullable::True => quote! {
                pub type #ident_where_ucc = #import_path::NullableJsonObjectPgTypeWhereFilter<
                    <#ident_not_null_ts as #import_path::PgJsonType>::Where
                >;
            }
        };
        //exists because need to implement .into_inner() for fields (only for read subtype)
        let ident_read_ts = {
            //todo maybe add some derive\impl to trait
            let ident_read_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_partial_ord()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_read_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_read_ts = {
                quote!{
                    impl #ident_read_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_default_opt_some_vec_one_el_for_ident_read_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_read_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            let impl_sqlx_encode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_read_ucc, &quote! {&#SelfSc.0});
            let impl_sqlx_type_for_ident_read_ts = gen_impl_sqlx_type_for_ident_ts(&ident_read_ucc, &gen_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            quote! {
                #ident_read_ts
                #impl_ident_read_ts
                #impl_default_opt_some_vec_one_el_for_ident_read_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_read_ts
                #impl_sqlx_type_for_ident_read_ts
            }
        };
        let ident_read_only_ids_standart_not_null_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_read_only_ids_ts = StructOrEnumDeriveTsStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_struct(
                &ident_read_only_ids_ucc,
                &Ts2::new(),
                &{
                    let opt_unit_ts = gen_opt_type_decl_ts(&quote! {()});
                    let vec_ts = |v: &dyn ToTokens| gen_vec_tokens_declaration_ts(&v);
                    let content_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                        match &pattern {
                            Pattern::Standart => {
                                let ts1 = match &is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_al_ts},
                                    IsNullable::True => gen_opt_type_decl_ts(&ident_read_inner_standart_not_null_al_ts),
                                };
                                quote! {#ts1}
                            }
                            Pattern::ArrDim1 { dim1_is_nullable } => {
                                let ts1 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_al_ts},
                                    IsNullable::True => gen_opt_type_decl_ts(&ident_read_inner_standart_not_null_al_ts),
                                });
                                let ts2 = match &is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts1),
                                };
                                quote! {#ts2}
                            }
                            Pattern::ArrDim2 { dim1_is_nullable, dim2_is_nullable } => {
                                let ts1 = vec_ts(&match &dim2_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_al_ts},
                                    IsNullable::True => gen_opt_type_decl_ts(&ident_read_inner_standart_not_null_al_ts),
                                });
                                let ts2 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts1),
                                });
                                let ts3 = match &is_nullable {
                                    IsNullable::False => ts2,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts2),
                                };
                                quote! {#ts3}
                            }
                            Pattern::ArrDim3 {
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                            } => {
                                let ts1 = vec_ts(&match &dim3_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_al_ts},
                                    IsNullable::True => gen_opt_type_decl_ts(&ident_read_inner_standart_not_null_al_ts),
                                });
                                let ts2 = vec_ts(&match &dim2_is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts1),
                                });
                                let ts3 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => ts2,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts2),
                                });
                                let ts4 = match &is_nullable {
                                    IsNullable::False => ts3,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts3),
                                };
                                quote! {#ts4}
                            }
                            Pattern::ArrDim4 {
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                                dim4_is_nullable,
                            } => {
                                let ts1 = vec_ts(&match &dim4_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_al_ts},
                                    IsNullable::True => gen_opt_type_decl_ts(&ident_read_inner_standart_not_null_al_ts),
                                });
                                let ts2 = vec_ts(&match &dim3_is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts1),
                                });
                                let ts3 = vec_ts(&match &dim2_is_nullable {
                                    IsNullable::False => ts2,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts2),
                                });
                                let ts4 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => ts3,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts3),
                                });
                                let ts5 = match &is_nullable {
                                    IsNullable::False => ts4,
                                    IsNullable::True => gen_opt_type_decl_ts(&ts4),
                                };
                                quote! {#ts5}
                            }
                        }
                    } else {
                        opt_unit_ts
                    };
                    quote!{(pub #import_path::Value<#content_ts>);}
                }
            );
        let ident_read_inner_ts = {
            let type_ts = match &pattern {
                Pattern::Standart => match &is_nullable {
                    IsNullable::False => &ident_read_inner_standart_not_null_al_ts,
                    IsNullable::True => &gen_opt_type_decl_ts(&ident_read_inner_standart_not_null_al_ts),
                },
                Pattern::ArrDim1 { dim1_is_nullable } => &{
                    let dim1_type = dim1_is_nullable.maybe_opt_wrap(quote! {#ident_read_inner_standart_not_null_al_ts});
                    is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim1_type))
                },
                Pattern::ArrDim2 { dim1_is_nullable, dim2_is_nullable } => &{
                    let dim2_type = dim2_is_nullable.maybe_opt_wrap(quote! {#ident_read_inner_standart_not_null_al_ts});
                    let dim1_type = dim1_is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim2_type));
                    is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim1_type))
                },
                Pattern::ArrDim3 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                } => &{
                    let dim3_type = dim3_is_nullable.maybe_opt_wrap(quote! {#ident_read_inner_standart_not_null_al_ts});
                    let dim2_type = dim2_is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim3_type));
                    let dim1_type = dim1_is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim2_type));
                    is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim1_type))
                },
                Pattern::ArrDim4 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                    dim4_is_nullable,
                } => &{
                    let dim4_type = dim4_is_nullable.maybe_opt_wrap(quote! {#ident_read_inner_standart_not_null_al_ts});
                    let dim3_type = dim3_is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim4_type));
                    let dim2_type = dim2_is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim3_type));
                    let dim1_type = dim1_is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim2_type));
                    is_nullable.maybe_opt_wrap(gen_vec_tokens_declaration_ts(&dim1_type))
                },
            };
            let impl_from_ident_origin_for_ident_read_inner_ts = {
                let value_dot_zero_ts = quote!{#ValueSc.0};
                let nullable_ts = quote!{#value_dot_zero_ts.map(Into::into)};
                let into_inner_content_ts = match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => value_dot_zero_ts,
                        IsNullable::True => nullable_ts,
                    },
                    Pattern::ArrDim1 {..} |
                    Pattern::ArrDim2 {..} |
                    Pattern::ArrDim3 {..} |
                    Pattern::ArrDim4 {..} => match &is_nullable {
                        IsNullable::False => quote!{#value_dot_zero_ts.into_iter().map(Into::into).collect()},
                        IsNullable::True => nullable_ts
                    },
                };
                quote! {
                    impl From<#ident_origin_ucc> for #ident_read_inner_ucc {
                        fn from(#ValueSc: #ident_origin_ucc) -> Self {
                            #into_inner_content_ts
                        }
                    }
                }
            };
            quote! {
                pub type #ident_read_inner_ucc = #type_ts;
                #impl_from_ident_origin_for_ident_read_inner_ts
            }
        };
        let ident_update_ts = {
            let ident_update_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_update_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_update_ts = {
                quote!{
                    impl #ident_update_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_location_lib_to_err_string_for_ident_update_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                gen_impl_to_err_string_ts(&Ts2::new(), &ident_update_ucc, &Ts2::new(), &quote! {format!("{self:?}")})
            } else {
                Ts2::new()
            };
            let impl_default_opt_some_vec_one_el_for_ident_update_ts =
                gen_impl_pg_crud_common_default_opt_some_vec_one_el_ts(&ident_update_ucc, &quote! {Self(#PgCrudCommonDefaultOptSomeVecOneElCall)});
            quote! {
                #ident_update_ts
                #impl_ident_update_ts
                #impl_location_lib_to_err_string_for_ident_update_ts
                #impl_default_opt_some_vec_one_el_for_ident_update_ts
            }
        };
        let ident_update_for_query_ts = {
            let ident_update_for_query_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &ident_update_for_query_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_update_for_query_ts = {
                quote! {
                    impl #ident_update_for_query_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_from_ident_update_for_ident_update_for_query_ts = gen_impl_from_ts(&ident_update_ucc, &ident_update_for_query_ucc, &quote! {Self(#ValueSc.0)});
            //its only for primitive json types
            let impl_sqlx_encode_sqlx_pg_for_ident_update_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_update_for_query_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            let impl_sqlx_type_for_ident_update_for_query_ts = gen_impl_sqlx_type_for_ident_ts(&ident_update_for_query_ucc, &ident_origin_ucc);
            quote! {
                #ident_update_for_query_ts
                #impl_ident_update_for_query_ts
                #impl_from_ident_update_for_ident_update_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_update_for_query_ts
                #impl_sqlx_type_for_ident_update_for_query_ts
            }
        };
        let pg_crud_macros_common_import_path_pg_crud_common = ImportPath::PgCrudCommon;
        let impl_pg_json_type_for_ident_ts = {
            let gen_dim_nbr_str = |dims_nbr: usize| format!("dim{dims_nbr}");
            let gen_dim_nbr_start_str = |dims_nbr: usize| format!("{}_start", gen_dim_nbr_str(dims_nbr));
            let gen_dim_nbr_end_str = |dims_nbr: usize| format!("{}_end", gen_dim_nbr_str(dims_nbr));
            let select_only_created_or_updated_ids_query_part_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                quote! {
                    match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                        Ok(v_f06128be) => Ok(format!("'{field_ident}',jsonb_build_object('value',${v_f06128be}),")),
                        Err(#ErSc) => Err(#ErSc),
                    }
                }
            } else {
                quote! {Ok(gen_pg_json_types_common::field_ident_jsonb_build_object_value(field_ident))}
            };
            let select_only_created_or_updated_ids_query_bind_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                quote! {
                    if let Err(#ErSc) = #QuerySc.try_bind(#ValueSc) {
                        return Err(#ErSc.to_string());
                    }
                    Ok(#QuerySc)
                }
            } else {
                quote! {Ok(#QuerySc)}
            };
            gen_impl_pg_json_type_ts(
                &pg_crud_macros_common_import_path_pg_crud_common,
                &ident,
                &ident_table_type_ucc,
                &ident_create_ucc,
                &ident_create_for_query_ucc,
                &ident_select_ucc,
                &match &pattern {
                    Pattern::Standart => IsSelectQueryPartSelfSelectUsed::False,
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => IsSelectQueryPartSelfSelectUsed::True,
                },
                &IsSelectQueryPartColumnFieldForErMessageUsed::False,
                &IsSelectQueryPartIsPgTypeUsed::False,
                &{
                    let format_handle = {
                        //last child dim value does not matter - null or type - works both good
                        let column_field_field_ident = format!("{{{ColumnFieldSc}}}->'{{field_ident}}'");
                        let format_handle = ArrDim::try_from(pattern).map_or_else(
                            |()| column_field_field_ident.clone(),
                            |arr_dim| {
                                enum ArrDimSelectPattern {
                                    ArrDim2 {
                                        dim1_is_nullable: IsNullable,
                                    },
                                    ArrDim3 {
                                        dim1_is_nullable: IsNullable,
                                        dim2_is_nullable: IsNullable,
                                    },
                                    ArrDim4 {
                                        dim1_is_nullable: IsNullable,
                                        dim2_is_nullable: IsNullable,
                                        dim3_is_nullable: IsNullable,
                                    },
                                }
                                impl TryFrom<&ArrDim> for ArrDimSelectPattern {
                                    type Error = ();
                                    fn try_from(v: &ArrDim) -> Result<Self, Self::Error> {
                                        match &v {
                                            ArrDim::ArrDim1 => Err(()),
                                            ArrDim::ArrDim2 {
                                                dim1_is_nullable,
                                            } => Ok(Self::ArrDim2 {
                                                dim1_is_nullable: *dim1_is_nullable,
                                            }),
                                            ArrDim::ArrDim3 {
                                                dim1_is_nullable,
                                                dim2_is_nullable,
                                            } => Ok(Self::ArrDim3 {
                                                dim1_is_nullable: *dim1_is_nullable,
                                                dim2_is_nullable: *dim2_is_nullable,
                                            }),
                                            ArrDim::ArrDim4 {
                                                dim1_is_nullable,
                                                dim2_is_nullable,
                                                dim3_is_nullable,
                                            } => Ok(Self::ArrDim4 {
                                                dim1_is_nullable: *dim1_is_nullable,
                                                dim2_is_nullable: *dim2_is_nullable,
                                                dim3_is_nullable: *dim3_is_nullable,
                                            }),
                                        }
                                    }
                                }
                                let gen_jsonb_agg = |jsonb_agg_content: &str, jsonb_arr_elements_content: &str, ordinality_content: &str, dims_nbr: usize| {
                                    let dim_nbr_start = gen_dim_nbr_start_str(dims_nbr);
                                    let dim_nbr_end = gen_dim_nbr_end_str(dims_nbr);
                                    format!(
                                        "select jsonb_agg(({jsonb_agg_content})) from jsonb_arr_elements(({jsonb_arr_elements_content})) with ordinality {ordinality_content} between {{{dim_nbr_start}}} and {{{dim_nbr_end}}}"
                                    )
                                };
                                ArrDimSelectPattern::try_from(&arr_dim).map_or_else(
                                    |()| gen_jsonb_agg(
                                        "value",
                                        &format!("select {column_field_field_ident}"),
                                        "where ordinality",
                                        1,
                                    ),
                                    |arr_dim_select_pattern| {
                                        // Dim1 does not fit into pattern. its only for 2+ dims
                                        let gen_d_nbr_elem = |content: usize| format!("d{content}_elem");
                                        let gen_d_nbr_ord = |content: usize| format!("d{content}_elem");
                                        let gen_dot_value = |content: &str| format!("{content}.value");
                                        let gen_as_value_where = |
                                            first_content: &str,
                                            second_content: &str
                                        | format!("as {first_content}(value, {second_content}) where {second_content}");
                                        let one = 1;
                                        gen_jsonb_agg(
                                            &{
                                                let mut usize_v_0ff8cf42 = match &arr_dim_select_pattern {
                                                    ArrDimSelectPattern::ArrDim2 { .. } => 2,
                                                    ArrDimSelectPattern::ArrDim3 { .. } => 3,
                                                    ArrDimSelectPattern::ArrDim4 { .. } => 4,
                                                };
                                                match &arr_dim_select_pattern {
                                                    ArrDimSelectPattern::ArrDim2 {
                                                        dim1_is_nullable,
                                                    } => vec![dim1_is_nullable],
                                                    ArrDimSelectPattern::ArrDim3 {
                                                        dim1_is_nullable,
                                                        dim2_is_nullable,
                                                    } => vec![
                                                        dim2_is_nullable,
                                                        dim1_is_nullable,
                                                    ],
                                                    ArrDimSelectPattern::ArrDim4 {
                                                        dim1_is_nullable,
                                                        dim2_is_nullable,
                                                        dim3_is_nullable,
                                                    } => vec![
                                                        dim3_is_nullable,
                                                        dim2_is_nullable,
                                                        dim1_is_nullable,
                                                    ],
                                                }
                                                .into_iter()
                                                .fold(gen_dot_value(&gen_d_nbr_elem(usize_v_0ff8cf42)), |mut acc_64e08e3a, is_nullable_0ff8cf42| {
                                                    let usize_value_minus_one_0ff8cf42 = usize_v_0ff8cf42.checked_sub(one).expect("a35e873e");
                                                    let d_usize_minus_one_elem_value = gen_dot_value(&gen_d_nbr_elem(usize_value_minus_one_0ff8cf42));
                                                    let value = gen_jsonb_agg(
                                                        &acc_64e08e3a,
                                                        &d_usize_minus_one_elem_value,
                                                        &gen_as_value_where(&gen_d_nbr_elem(usize_v_0ff8cf42), &gen_d_nbr_ord(usize_v_0ff8cf42)),
                                                        usize_v_0ff8cf42,
                                                    );
                                                    acc_64e08e3a = match &is_nullable_0ff8cf42 {
                                                        IsNullable::False => value,
                                                        IsNullable::True => {
                                                            format!("case when jsonb_typeof({d_usize_minus_one_elem_value})='arr' then ({value}) else null end")
                                                        }
                                                    };
                                                    usize_v_0ff8cf42 = usize_value_minus_one_0ff8cf42;
                                                    acc_64e08e3a
                                                })
                                            },
                                            &column_field_field_ident,
                                            &gen_as_value_where(&gen_d_nbr_elem(one), &gen_d_nbr_ord(one)),
                                            one,
                                        )
                                    },
                                )
                            },
                        );
                        match &is_nullable {
                            IsNullable::False => format_handle,
                            IsNullable::True => format!("case when jsonb_typeof({column_field_field_ident})='null' then 'null'::jsonb else ({format_handle}) end"),
                        }
                    };
                    let maybe_dims_start_end_init = ArrDim::try_from(pattern).ok().into_iter().flat_map(|arr_dim| {
                        (1..=arr_dim.to_usize()).map(|el_8d56d66d| {
                            let dim_nbr_start_ts =
                                gen_dim_nbr_start_str(el_8d56d66d)
                                    .parse::<Ts2>()
                                    .expect("77ec13b9");
                            let dim_nbr_end_ts =
                                gen_dim_nbr_end_str(el_8d56d66d)
                                    .parse::<Ts2>()
                                    .expect("24acbb5e");
                            let dim_nbr_pagination_ts =
                                format!(
                                    "{}_pagination",
                                    gen_dim_nbr_str(el_8d56d66d)
                                )
                                .parse::<Ts2>()
                                .expect("745c99b3");
                            quote! {
                                let #dim_nbr_start_ts = #ValueSc.#dim_nbr_pagination_ts.start();
                                let #dim_nbr_end_ts = #ValueSc.#dim_nbr_pagination_ts.end();
                            }
                        })
                    });
                    let format_ts = dq_ts(&format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',({format_handle})))"));
                    quote! {
                        #(#maybe_dims_start_end_init)*
                        Ok(format!(#format_ts))
                    }
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &ident_read_only_ids_ucc,
                &{
                    let content_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                        quote! {format!("jsonb_build_object('value',{column_field})")}
                    } else {
                        quote! {"jsonb_build_object('value','null'::jsonb)".to_owned()}
                    };
                    quote! {Ok(#content_ts)}
                },
                &ident_read_inner_ucc,
                &{
                    let content_ts_0ff8cf42 = quote! {#ValueSc.0.0};
                    let gen_match_el_zero_ts = |
                        match_ts: &dyn ToTokens,
                        value_ts: &dyn ToTokens,
                        content_ts: &dyn ToTokens
                    | {
                        quote! {#match_ts.map(|#value_ts| #value_ts.0 #content_ts)}
                    };
                    let gen_into_iter_map_el_collect_ts = |
                        el_ts: &dyn ToTokens,
                        content_ts: &dyn ToTokens
                    | {
                        quote! {.into_iter().map(|#el_ts|#content_ts).collect()}
                    };
                    let gen_into_iter_map_el_collect_is_nullable_ts = |
                        el_ts: &dyn ToTokens,
                        is_nullable_d9400a66: &IsNullable
                    | {
                        gen_into_iter_map_el_collect_ts(
                            &el_ts,
                            &match &is_nullable_d9400a66 {
                                IsNullable::False => quote! {#el_ts.0},
                                IsNullable::True => gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &quote! {v_f8b0b01d},
                                    &Ts2::new()
                                )
                            }
                        )
                    };
                    let gen_into_iter_map_el_collect_is_nullable_content_ts = |
                        el_ts: &dyn ToTokens,
                        value_ts: &dyn ToTokens,
                        is_nullable_d9400a66: &IsNullable,
                        content_ts_d9400a66: &dyn ToTokens
                    | {
                        match &is_nullable_d9400a66 {
                            IsNullable::False => gen_into_iter_map_el_collect_ts(
                                &el_ts,
                                &quote! {#el_ts.0 #content_ts_d9400a66}
                            ),
                            IsNullable::True => {
                                let match_el_zero_ts = gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &value_ts,
                                    &content_ts_d9400a66
                                );
                                quote! {.into_iter().map(|#el_ts|#match_el_zero_ts).collect()}
                            }
                        }
                    };
                    let into_inner_content_ts = match &pattern {
                        Pattern::Standart => Ts2::new(),
                        Pattern::ArrDim1 { dim1_is_nullable } => gen_into_iter_map_el_collect_is_nullable_ts(
                            &quote!{el_0fdb74a5},
                            dim1_is_nullable,
                        ),
                        Pattern::ArrDim2 { dim1_is_nullable, dim2_is_nullable } => {
                            let dim2_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_ts(
                                &quote!{el_dac5ba56},
                                dim2_is_nullable
                            );
                            gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_cf5646e9},
                                &quote!{v_1c90c80c},
                                dim1_is_nullable,
                                &dim2_is_nullable_content_ts
                            )
                        }
                        Pattern::ArrDim3 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                        } => {
                            let dim3_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_ts(
                                &quote!{el_c935a865},
                                dim3_is_nullable
                            );
                            let dim2_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_dc9e788b},
                                &quote!{v_3d1307e8},
                                dim2_is_nullable,
                                &dim3_is_nullable_content_ts
                            );
                            gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_bf67606b},
                                &quote!{v_721e4164},
                                dim1_is_nullable,
                                &dim2_is_nullable_content_ts
                            )
                        }
                        Pattern::ArrDim4 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                            dim4_is_nullable,
                        } => {
                            let dim4_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_ts(
                                &quote!{el_144c60e6},
                                dim4_is_nullable
                            );
                            let dim3_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_98961cb7},
                                &quote!{v_995a5fbe},
                                dim3_is_nullable,
                                &dim4_is_nullable_content_ts
                            );
                            let dim2_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_34e95172},
                                &quote!{v_75b18ade},
                                dim2_is_nullable,
                                &dim3_is_nullable_content_ts
                            );
                            gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_f64124e2},
                                &quote!{v_7fc1b146},
                                dim1_is_nullable,
                                &dim2_is_nullable_content_ts
                            )
                        }
                    };
                    match &is_nullable {
                        IsNullable::False => quote! {#content_ts_0ff8cf42 #into_inner_content_ts},
                        IsNullable::True => gen_match_el_zero_ts(
                            &content_ts_0ff8cf42,
                            &quote!{v_3432e728},
                            &into_inner_content_ts
                        ),
                    }
                },
                &ident_update_ucc,
                &ident_update_for_query_ucc,
                &{
                    let format_ts = dq_ts(&format!("jsonb_set({{{JsonbSetAccumulatorSc}}},'{{{{{{jsonb_set_path}}}}}}',${{v_26526e0f}})"));
                    quote! {
                        match #import_path::incr_checked_add_one_returning_incr(#IncrSc) {
                            Ok(v_26526e0f) => Ok(format!(#format_ts)),
                            Err(#ErSc) => Err(#ErSc),
                        }
                    }
                },
                &IsUpdateQueryPartSelfUpdateUsed::False,
                &IsUpdateQueryPartJsonbSetTargetUsed::False,
                &IsUpdateQueryBindMutable::True,
                &quote! {
                    if let Err(er) = query.try_bind(#ValueSc) {
                        return Err(er.to_string());
                    }
                    Ok(query)
                },
                &select_only_created_or_updated_ids_query_part_ts,
                &if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                    IsSelectOnlyUpdatedIdsQueryBindMutable::True
                } else {
                    IsSelectOnlyUpdatedIdsQueryBindMutable::False
                },
                &select_only_created_or_updated_ids_query_bind_ts,
                &select_only_created_or_updated_ids_query_part_ts,
                &if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                    IsSelectOnlyCreatedIdsQueryBindMutable::True
                } else {
                    IsSelectOnlyCreatedIdsQueryBindMutable::False
                },
                &select_only_created_or_updated_ids_query_bind_ts,
            )
        };
        let maybe_impl_pg_json_type_object_vec_el_id_for_ident_origin_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
            let (query_bind_string_as_pg_text_create_for_query_ts, query_bind_string_as_pg_text_update_for_query_ts) = {
                enum CreateForQueryOrUpdateForQuery {
                    CreateForQuery,
                    UpdateForQuery,
                }
                let gen_query_bind_string_as_pg_text_ts = |create_for_query_or_update_for_query: &CreateForQueryOrUpdateForQuery| {
                    let name_ts = format!(
                        "query_bind_string_as_pg_text_{}_for_query",
                        match &create_for_query_or_update_for_query {
                            CreateForQueryOrUpdateForQuery::CreateForQuery => "create",
                            CreateForQueryOrUpdateForQuery::UpdateForQuery => "update",
                        }
                    )
                    .parse::<Ts2>()
                    .expect("f1bcde08");
                    let type_ts: &dyn ToTokens = match &create_for_query_or_update_for_query {
                        CreateForQueryOrUpdateForQuery::CreateForQuery => &CreateForQueryUcc,
                        CreateForQueryOrUpdateForQuery::UpdateForQuery => &UpdateForQueryUcc,
                    };
                    quote! {
                        fn #name_ts(
                            #ValueSc: <Self::PgJsonType as #import_path::PgJsonType>::#type_ts,
                            mut #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                        ) -> Result<
                            sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                            #StringTs
                        > {
                            if let Err(#ErSc) = #QuerySc.try_bind(#ValueSc.0.0.to_string()) {
                                return Err(#ErSc.to_string())
                            }
                            Ok(#QuerySc)
                        }
                    }
                };
                (gen_query_bind_string_as_pg_text_ts(&CreateForQueryOrUpdateForQuery::CreateForQuery), gen_query_bind_string_as_pg_text_ts(&CreateForQueryOrUpdateForQuery::UpdateForQuery))
            };
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                impl #import_path::PgJsonTypeObjectVecElementId for #ident {
                    type PgJsonType = Self;
                    type #CreateForQueryUcc = #ident_create_for_query_ucc;
                    type #UpdateUcc = #ident_update_ucc;
                    type #ReadInnerUcc = #ident_read_inner_ucc;
                    #query_bind_string_as_pg_text_create_for_query_ts
                    #query_bind_string_as_pg_text_update_for_query_ts
                    fn get_inner(#ValueSc: &<Self::PgJsonType as #import_path::PgJsonType>::#CreateForQueryUcc) -> &Self::#ReadInnerUcc {
                        &#ValueSc.0.0
                    }
                    fn incr_checked_add_one(#IncrSc: &mut #U64) -> Result<#U64, #import_path::QueryPartEr> {
                        #import_path::incr_checked_add_one_returning_incr(#IncrSc)
                    }
                }
            }
        } else {
            Ts2::new()
        };
        let impl_pg_json_type_test_cases_for_ident_ts = {
            enum F32OrF64 {
                F32,
                F64
            }
            let gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts = |read_or_update: &ReadOrUpdate| {
                let read_or_update_ucc = read_or_update.ucc();
                quote! {<#SelfUcc::#PgJsonTypeUcc
                    as
                    #pg_crud_macros_common_import_path_pg_crud_common::#PgJsonTypeUcc
                >::#read_or_update_ucc::#NewSc(#ValueSc)}
            };
            let standart_not_null_test_cases_vec_name_ts = match &pg_json_type {
                PgJsonType::I8AsJsonbNbr => quote! {i8_test_cases_vec},
                PgJsonType::I16AsJsonbNbr => quote! {i16_test_cases_vec},
                PgJsonType::I32AsJsonbNbr => quote! {i32_test_cases_vec},
                PgJsonType::I64AsJsonbNbr => quote! {i64_test_cases_vec},
                PgJsonType::U8AsJsonbNbr => quote! {u8_test_cases_vec},
                PgJsonType::U16AsJsonbNbr => quote! {u16_test_cases_vec},
                PgJsonType::U32AsJsonbNbr => quote! {u32_test_cases_vec},
                PgJsonType::U64AsJsonbNbr => quote! {u64_test_cases_vec},
                PgJsonType::F32AsJsonbNbr => quote! {f32_test_cases_vec},
                PgJsonType::F64AsJsonbNbr => quote! {f64_test_cases_vec},
                PgJsonType::BoolAsJsonbBoolean => quote! {bool_test_cases_vec},
                PgJsonType::StringAsJsonbString => quote! {string_test_cases_vec},
                PgJsonType::UuidUuidAsJsonbString => quote! {uuid_uuid_test_cases_vec},
            };
            let gen_arr_dim_equal_ts = |dim: &Dim| {
                let dim_index_nbr_max = DimIndexNbr::from(dim);
                let gen_dim_index_nbr_ts = |is_nullable_vec: &[&IsNullable]|{
                    assert!(!is_nullable_vec.is_empty(), "c1a5939d");
                    let content_ts_c85923bd = {
                        let gen_index_nbr_ts = |index_c1128a3e: usize|format!("index_{index_c1128a3e}").parse::<Ts2>().expect("afbe7252");
                        let gen_value_nbr_ts = |index_0abe6039: usize|format!("value{index_0abe6039}").parse::<Ts2>().expect("568d8eb6");
                        let gen_for_in_ts = |
                            index_ts: &dyn ToTokens,
                            value_ts: &dyn ToTokens,
                            enumerate_ts: &dyn ToTokens,
                            content_ts_aaf03124: &dyn ToTokens,
                        |quote!{
                            for (#index_ts, #value_ts) in #enumerate_ts.0.into_iter().enumerate() {
                                #content_ts_aaf03124
                            }
                        };
                        let gen_for_value_index_dot_zero_into_iter_enumerate_ts = |
                            index_0082bcdf: usize,
                            index_e81c6d28: usize,
                            index_b7b230b2: usize,
                            content_ts_d575a40c: &dyn ToTokens,
                        |gen_for_in_ts(
                            &gen_index_nbr_ts(index_0082bcdf),
                            &gen_value_nbr_ts(index_e81c6d28),
                            &gen_value_nbr_ts(index_b7b230b2),
                            &content_ts_d575a40c
                        );
                        let gen_if_let_some_ts = |
                            some_ts: &dyn ToTokens,
                            equal_ts: &dyn ToTokens,
                            content_ts_9292e3cf: &dyn ToTokens,
                        |quote!{
                            if let Some(#some_ts) = #equal_ts.0 {
                                #content_ts_9292e3cf
                            }
                        };
                        let gen_if_let_some_equals_value_index_dot_zero_ts = |
                            index_c4552aef: usize,
                            index_9f1fbc9f: usize,
                            content_ts_832b20d5: &dyn ToTokens,
                        |gen_if_let_some_ts(
                            &gen_value_nbr_ts(index_c4552aef),
                            &gen_value_nbr_ts(index_9f1fbc9f),
                            &content_ts_832b20d5
                        );
                        let gen_index = |start_index: usize, is_nullable_vec_41b82a0c: &[&IsNullable]| -> usize {
                            start_index.checked_add(
                                is_nullable_vec_41b82a0c
                                .iter()
                                .filter(|el_bf28b242| matches!(el_bf28b242, IsNullable::True))
                                .count()
                            ).expect("de4c4116")
                        };
                        let mut content_ts_4c106eea = {
                            let content_ts_f1ffd3b2 = {
                                let value_index_ts = gen_value_nbr_ts(
                                    gen_index(
                                        is_nullable_vec.len().saturating_sub(1),
                                        &once(is_nullable)
                                        .chain(
                                            is_nullable_vec
                                                .iter()
                                                .take(is_nullable_vec.len().saturating_sub(1))
                                                .copied(),
                                        ).collect::<Vec<&IsNullable>>()
                                    )
                                );
                                let to_nbr_starting_with_one_word_str = |dim_index_nbr: &DimIndexNbr| match dim_index_nbr {
                                    DimIndexNbr::Zero => "One",
                                    DimIndexNbr::One => "Two",
                                    DimIndexNbr::Two => "Three",
                                    DimIndexNbr::Three => "Four",
                                };
                                let dim_nbr_starting_with_one_equal_ts = format!("Dim{}Equal", to_nbr_starting_with_one_word_str(&dim_index_nbr_max)).parse::<Ts2>().expect("52fa34ac");
                                let pg_json_type_where_dim_nbr_starting_with_one_equal_ts = format!("PgJsonTypeWhereDim{}Equal", to_nbr_starting_with_one_word_str(&dim_index_nbr_max)).parse::<Ts2>().expect("15d769b0");
                                let where_ident_where_ucc_c994819b = SelfWhereUcc::from_tokens(&gen_ident_ts(&IsNullable::False, pattern));
                                let value_ident_table_type_ucc_0d9dce86 = SelfTableTypeUcc::from_tokens(&gen_ident_ts(
                                    is_nullable_vec.last().expect("1221f6ec"),
                                    &match dim_index_nbr_max {
                                        DimIndexNbr::Zero => pattern.down_by_1().expect("1a47af86"),
                                        DimIndexNbr::One => pattern.down_by_2().expect("d8260225"),
                                        DimIndexNbr::Two => pattern.down_by_3().expect("473ac422"),
                                        DimIndexNbr::Three => pattern.down_by_4().expect("6a143218"),
                                    }
                                ));
                                let vec_content_ts = {
                                    let content_ts_0dc5a500 = (
                                        0i32..=match dim_index_nbr_max {
                                            DimIndexNbr::Zero => 0i32,
                                            DimIndexNbr::One => 1i32,
                                            DimIndexNbr::Two => 2i32,
                                            DimIndexNbr::Three => 3i32,
                                        }
                                    )
                                    .map(|el_db559599| {
                                        let index_nbr_ts = format!("index_{el_db559599}")
                                            .parse::<Ts2>()
                                            .expect("f0ce7e73");
                                        quote! {
                                            pg_crud_common::UnsignedPartOfI32::try_from(
                                                i32::try_from(#index_nbr_ts)
                                                    .expect("5a1818e7")
                                            ).expect("ad1ab73f")
                                        }
                                    }).collect::<Vec<Ts2>>();
                                    quote! {#(#content_ts_0dc5a500),*}
                                };
                                quote! {
                                    #where_ident_where_ucc_c994819b::#dim_nbr_starting_with_one_equal_ts(
                                        where_filters::#pg_json_type_where_dim_nbr_starting_with_one_equal_ts {
                                            logical_operator: #import_path::LogicalOperator::And,
                                            dims: where_filters::BoundedStdVecVec::try_from(
                                                vec![#vec_content_ts]
                                            ).expect("82cc0a3c"),
                                            #ValueSc: #value_ident_table_type_ucc_0d9dce86::new(#value_index_ts.into()),
                                        }
                                    )
                                }
                            };
                            match is_nullable {
                                IsNullable::False => quote! {acc_049ff0b3.push(#content_ts_f1ffd3b2);},
                                IsNullable::True => quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new(vec![#content_ts_f1ffd3b2]) {
                                        Ok(v_9328b66f) => {
                                            acc_049ff0b3.push(#import_path::NullableJsonObjectPgTypeWhereFilter(Some(v_9328b66f)));
                                        },
                                        Err(er) => match er {
                                            #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                            #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("2f5f648a")
                                        }
                                    }
                                },
                            }
                        };
                        for (index_ef936914, _) in is_nullable_vec.iter().take(is_nullable_vec.len().saturating_sub(1)).enumerate() {
                            let is_nullable_vec_e7e7f6f8 = is_nullable_vec
                            .iter()
                            .take(
                                is_nullable_vec
                                    .len()
                                    .saturating_sub(index_ef936914.checked_add(1).expect("75d5ed28")),
                            )
                            .copied()
                            .collect::<Vec<&IsNullable>>();
                            let is_nullable_vec_e7e7f6f8_len = is_nullable_vec_e7e7f6f8.len();
                            let is_nullable_vec_e7e7f6f8_len_saturating_sub_one = is_nullable_vec_e7e7f6f8_len.saturating_sub(1);
                            content_ts_4c106eea = {
                                let index_74ae6d77 = gen_index(
                                    is_nullable_vec_e7e7f6f8_len_saturating_sub_one,
                                    &once(is_nullable)
                                    .chain(
                                        is_nullable_vec_e7e7f6f8
                                            .iter()
                                            .take(is_nullable_vec_e7e7f6f8_len_saturating_sub_one)
                                            .copied(),
                                    ).collect::<Vec<&IsNullable>>()
                                );
                                let index_74ae6d77_incr_by_1 = index_74ae6d77.checked_add(1).expect("96e90e72");
                                match &is_nullable_vec_e7e7f6f8.last().expect("88548240") {
                                    IsNullable::False => gen_for_value_index_dot_zero_into_iter_enumerate_ts(
                                        is_nullable_vec_e7e7f6f8_len,
                                        index_74ae6d77_incr_by_1,
                                        index_74ae6d77,
                                        &content_ts_4c106eea,
                                    ),
                                    IsNullable::True => gen_if_let_some_equals_value_index_dot_zero_ts(
                                        index_74ae6d77_incr_by_1,
                                        index_74ae6d77,
                                        &gen_for_value_index_dot_zero_into_iter_enumerate_ts(
                                            is_nullable_vec_e7e7f6f8_len,
                                            index_74ae6d77.checked_add(2).expect("00da046c"),
                                            index_74ae6d77_incr_by_1,
                                            &content_ts_4c106eea,
                                        )
                                    )
                                }
                            };
                        }
                        let create_dot_zero_ts = quote!{create.0};
                        match &is_nullable {
                            IsNullable::False => gen_for_in_ts(
                                &gen_index_nbr_ts(0),
                                &gen_value_nbr_ts(0),
                                &create_dot_zero_ts,
                                &content_ts_4c106eea
                            ),
                            IsNullable::True => gen_if_let_some_ts(
                                &gen_value_nbr_ts(0),
                                &create_dot_zero_ts,
                                &gen_for_value_index_dot_zero_into_iter_enumerate_ts(
                                    0,
                                    1,
                                    0,
                                    &content_ts_4c106eea
                                )
                            )
                        }
                    };
                    quote! {
                        Some(#import_path::NotEmptyUniqueVec::try_new({
                            let mut acc_049ff0b3 = Vec::new();
                            #content_ts_c85923bd
                            acc_049ff0b3
                        }).expect("e99ecd08"))
                    }
                };
                match &pattern {
                    Pattern::Standart => none_ts.clone(),
                    Pattern::ArrDim1 { dim1_is_nullable } => match dim_index_nbr_max {
                        DimIndexNbr::Zero => gen_dim_index_nbr_ts(&[
                            dim1_is_nullable,
                        ]),
                        DimIndexNbr::One | DimIndexNbr::Two | DimIndexNbr::Three => none_ts.clone(),
                    },
                    Pattern::ArrDim2 { dim1_is_nullable, dim2_is_nullable } => match dim_index_nbr_max {
                        DimIndexNbr::Zero => gen_dim_index_nbr_ts(&[
                            dim1_is_nullable,
                        ]),
                        DimIndexNbr::One => gen_dim_index_nbr_ts(&[
                            dim1_is_nullable,
                            dim2_is_nullable
                        ]),
                        DimIndexNbr::Two | DimIndexNbr::Three => none_ts.clone(),
                    },
                    Pattern::ArrDim3 {
                        dim1_is_nullable,
                        dim2_is_nullable,
                        dim3_is_nullable,
                    } => match dim_index_nbr_max {
                        DimIndexNbr::Zero => gen_dim_index_nbr_ts(&[
                            dim1_is_nullable,
                        ]),
                        DimIndexNbr::One => gen_dim_index_nbr_ts(&[
                            dim1_is_nullable,
                            dim2_is_nullable,
                        ]),
                        DimIndexNbr::Two => gen_dim_index_nbr_ts(&[
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable
                        ]),
                        DimIndexNbr::Three => none_ts.clone(),
                    },
                    Pattern::ArrDim4 {
                        dim1_is_nullable,
                        dim2_is_nullable,
                        dim3_is_nullable,
                        dim4_is_nullable,
                    } => {
                        match dim_index_nbr_max {
                            DimIndexNbr::Zero => gen_dim_index_nbr_ts(&[
                                dim1_is_nullable
                            ]),
                            DimIndexNbr::One => gen_dim_index_nbr_ts(&[
                                dim1_is_nullable,
                                dim2_is_nullable,
                            ]),
                            DimIndexNbr::Two => gen_dim_index_nbr_ts(&[
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                            ]),
                            DimIndexNbr::Three => gen_dim_index_nbr_ts(&[
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                                dim4_is_nullable
                            ])
                        }
                    }
                }
            };
            let opt_vec_create_ts = {
                let gen_some_acc_content_ts = |is_nullable_c964bb93: &IsNullable, ident_ts_dc0d5797: &dyn ToTokens| {
                    let (new_content_ts, maybe_acc_push_none_ts) = match &is_nullable_c964bb93 {
                        IsNullable::False => (quote! {vec![el_88131059.0.into()]}, Ts2::new()),
                        IsNullable::True => (quote! {Some(el_88131059.0.into())}, quote! {acc_50e99088.push(<Self as #import_path::PgJsonType>::Create::new(None));}),
                    };
                    //todo check - maybe need to add something here
                    let maybe_acc_push_long_vec_ts = match &is_nullable {
                        IsNullable::False => quote! {
                            let mut acc_27624e5e = Vec::new();
                            for el_0dcb405a in v_8de026a4 {
                                acc_27624e5e.push(el_0dcb405a.0.into());
                            }
                            if !acc_27624e5e.is_empty() {
                                acc_50e99088.push(<Self as #import_path::PgJsonType>::Create::new(acc_27624e5e));
                            }
                        },
                        IsNullable::True => Ts2::new(),
                    };
                    let maybe_dot_clone_ts = match &is_nullable {
                        IsNullable::False => quote!{.clone()},
                        IsNullable::True => Ts2::new(),
                    };
                    quote! {Some({
                        let mut acc_50e99088 = Vec::new();
                        if let Some(v_8de026a4) = <#ident_ts_dc0d5797 as #import_path::PgJsonTypeTestCases>::#OptVecCreateSc() {
                            for el_88131059 in v_8de026a4 #maybe_dot_clone_ts {
                                acc_50e99088.push(<Self as #import_path::PgJsonType>::Create::new(#new_content_ts));
                            }
                            #maybe_acc_push_long_vec_ts
                        }
                        #maybe_acc_push_none_ts
                        acc_50e99088
                    })}
                };
                let content_ts = match &pattern {
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => quote! {
                            Some(
                                #import_path::#standart_not_null_test_cases_vec_name_ts().into_iter().map(<Self as #import_path::PgJsonType>::Create::new).collect()
                            )
                        },
                        IsNullable::True => gen_some_acc_content_ts(is_nullable, &gen_ident_ts(&IsNullable::False, &Pattern::Standart)),
                    },
                    Pattern::ArrDim1 { dim1_is_nullable } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("dec468c0")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pattern),
                        },
                    ),
                    Pattern::ArrDim2 { dim1_is_nullable, .. } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("4010ebf7")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pattern),
                        },
                    ),
                    Pattern::ArrDim3 { dim1_is_nullable, .. } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("acdbb564")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pattern),
                        },
                    ),
                    Pattern::ArrDim4 { dim1_is_nullable, .. } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("5abf9504")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pattern),
                        },
                    ),
                };
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr
                    | PgJsonType::I16AsJsonbNbr
                    | PgJsonType::I32AsJsonbNbr
                    | PgJsonType::I64AsJsonbNbr
                    | PgJsonType::U8AsJsonbNbr
                    | PgJsonType::U16AsJsonbNbr
                    | PgJsonType::U32AsJsonbNbr
                    | PgJsonType::U64AsJsonbNbr
                    | PgJsonType::F32AsJsonbNbr
                    | PgJsonType::F64AsJsonbNbr
                    | PgJsonType::BoolAsJsonbBoolean
                    | PgJsonType::StringAsJsonbString => content_ts,
                    PgJsonType::UuidUuidAsJsonbString => quote! {None},
                }
            };
            let read_only_ids_to_two_dimal_vec_read_inner_ts = {
                let (has_len_greater_than_one_ts, has_len_greater_than_one_for_for_ts) = {
                    let gen_ts = |content_ts: &dyn ToTokens| {
                        quote! {let has_len_greater_than_one = #content_ts;}
                    };
                    (
                        gen_ts(&quote! {read_only_ids_to_two_dimal_vec_read_inner.len() > 1}),
                        gen_ts(&quote! {{
                            let mut has_len_greater_than_one = false;
                            for el_4a00ab02 in &read_only_ids_to_two_dimal_vec_read_inner {
                                if el_4a00ab02.len() > 1 {
                                    has_len_greater_than_one = true;
                                    break;
                                }
                            }
                            has_len_greater_than_one
                        }}),
                    )
                };
                let gen_acc_content_handle_ts = |ident_ts_416231d8: &dyn ToTokens, has_len_greater_than_one_content_ts: &dyn ToTokens| {
                    let ident_read_only_ids_ucc_1d31038d = SelfReadOnlyIdsUcc::from_tokens(&ident_ts_416231d8);
                    let opt_additional_content_ts = {
                        let el_82c7dc0a_clone_ts = quote! {el_82c7dc0a.clone()};
                        let first = quote! {vec![#el_82c7dc0a_clone_ts]};
                        let second = quote! {vec![#el_82c7dc0a_clone_ts, #el_82c7dc0a_clone_ts]};
                        let (first_content_ts, second_content_ts) = match &is_nullable {
                            IsNullable::False => (first, second),
                            IsNullable::True => {
                                let gen_ts = |content_ts: &dyn ToTokens| quote! {Some(#content_ts)};
                                (gen_ts(&first), gen_ts(&second))
                            }
                        };
                        quote! {
                            let opt_additional = {
                                let mut opt_additional = None;
                                for el_c4f9bf8f in &read_only_ids_to_two_dimal_vec_read_inner {
                                    if opt_additional.is_some() {
                                        break;
                                    }
                                    for el_82c7dc0a in el_c4f9bf8f {
                                        if opt_additional.is_none() {
                                            opt_additional = Some((vec![#first_content_ts], vec![#second_content_ts]));
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                                opt_additional
                            };
                        }
                    };
                    let acc_push_vec_content_ts = {
                        let content_ts = {
                            let inner_content_ts = quote! {{
                                let mut acc_6cd5b60a = Vec::new();
                                for el_640f58e8 in read_only_ids_to_two_dimal_vec_read_inner {
                                    for el_d251d1f6 in el_640f58e8 {
                                        acc_6cd5b60a.push(el_d251d1f6);
                                    }
                                }
                                acc_6cd5b60a
                            }};
                            match &is_nullable {
                                IsNullable::False => inner_content_ts,
                                IsNullable::True => quote! {Some(#inner_content_ts)},
                            }
                        };
                        quote! {acc_0a07db18.push(vec![#content_ts]);}
                    };
                    let maybe_acc_push_vec_none_ts = match &is_nullable {
                        IsNullable::False => Ts2::new(),
                        IsNullable::True => quote! {acc_0a07db18.push(vec![None]);},
                    };
                    quote! {
                        let mut acc_0a07db18 = Vec::new();
                        let read_only_ids_to_two_dimal_vec_read_inner = <
                            #ident_ts_416231d8
                            as
                            #import_path::PgJsonTypeTestCases
                        >::#ReadOnlyIdsToTwoDimalVecReadInnerSc(
                            &#ident_read_only_ids_ucc_1d31038d(read_only_ids.0.clone())
                        );
                        #opt_additional_content_ts
                        #has_len_greater_than_one_content_ts
                        #acc_push_vec_content_ts
                        #maybe_acc_push_vec_none_ts
                        if let Some(v_3de7fba4) = opt_additional {
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
                    Pattern::Standart => match &is_nullable {
                        IsNullable::False => quote! {vec![#import_path::#standart_not_null_test_cases_vec_name_ts().into()]},
                        IsNullable::True => quote! {
                            let mut acc_97242d4d = Vec::new();
                            for el_8f3646f9 in <#ident_standart_not_null_ucc as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsToTwoDimalVecReadInnerSc(&#ident_read_only_ids_standart_not_null_ucc(read_only_ids.0.clone())) {
                                for el_35a4dba9 in el_8f3646f9 {
                                    acc_97242d4d.push(vec![Some(el_35a4dba9)]);
                                }
                            }
                            acc_97242d4d.push(vec![None]);
                            acc_97242d4d
                        },
                    },
                    Pattern::ArrDim1 { dim1_is_nullable } => gen_acc_content_handle_ts(
                        &gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("d6f89137")),
                        &match &dim1_is_nullable {
                            IsNullable::False => &has_len_greater_than_one_for_for_ts,
                            IsNullable::True => &has_len_greater_than_one_ts,
                        },
                    ),
                    Pattern::ArrDim2 { dim1_is_nullable, .. } => gen_acc_content_handle_ts(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("38774398")), &has_len_greater_than_one_ts),
                    Pattern::ArrDim3 { dim1_is_nullable, .. } => gen_acc_content_handle_ts(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("053f4bab")), &has_len_greater_than_one_ts),
                    Pattern::ArrDim4 { dim1_is_nullable, .. } => gen_acc_content_handle_ts(&gen_ident_ts(dim1_is_nullable, &pattern.down_by_1().expect("860f8f15")), &has_len_greater_than_one_ts),
                };
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr
                    | PgJsonType::I16AsJsonbNbr
                    | PgJsonType::I32AsJsonbNbr
                    | PgJsonType::I64AsJsonbNbr
                    | PgJsonType::U8AsJsonbNbr
                    | PgJsonType::U16AsJsonbNbr
                    | PgJsonType::U32AsJsonbNbr
                    | PgJsonType::U64AsJsonbNbr
                    | PgJsonType::F32AsJsonbNbr
                    | PgJsonType::F64AsJsonbNbr
                    | PgJsonType::BoolAsJsonbBoolean
                    | PgJsonType::StringAsJsonbString => content_ts,
                    PgJsonType::UuidUuidAsJsonbString => quote! {Vec::new()},
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&ReadOrUpdate::Update);
            let read_only_ids_into_opt_value_read_inner_ts = {
                let content_ts = gen_import_path_value_init_ts(&if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ValueSc.0.#ValueSc}
                } else {
                    quote! {
                        <Self as #import_path::PgJsonType>::into_inner(
                            <
                                <Self as #import_path::PgJsonType>::Read
                                as
                                #PgCrudCommonDefaultOptSomeVecOneEl
                            >::default_opt_some_vec_one_el()
                        )
                    }
                });
                quote! {Some(#content_ts)}
            };
            let update_to_read_only_ids_ts = {
                let value_init_ts = gen_import_path_value_init_ts(&if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                    let gen_iter_or_match_ts = |
                        is_nullable_1d9cc9dd: &IsNullable,
                        ident_ts_36d8e080: &dyn ToTokens,
                        update_is_nullable_69216aba: &IsNullable
                    | {
                        let value_zero_zero_ts = quote! {#ValueSc.0.0};
                        let content_ts = {
                            let ident_update_ts_7c40250a = SelfUpdateUcc::from_tokens(&ident_ts_36d8e080);
                            let content_ts = {
                                let content_ts = match &update_is_nullable_69216aba {
                                    IsNullable::False => quote! {el_aa999306.clone()},
                                    IsNullable::True => quote! {v_92de91cc.clone()},
                                };
                                quote! {#ident_update_ts_7c40250a(#content_ts)}
                            };
                            quote! {
                                <
                                    #ident_ts_36d8e080
                                    as
                                    #import_path::PgJsonTypeTestCases
                                >::update_to_read_only_ids(&#content_ts).0.#ValueSc
                            }
                        };
                        match &is_nullable_1d9cc9dd {
                            IsNullable::False => quote! {
                                #value_zero_zero_ts.iter().map(|el_aa999306|#content_ts).collect()
                            },
                            IsNullable::True => quote! {
                                #value_zero_zero_ts.as_ref().map(|v_92de91cc| #content_ts)
                            },
                        }
                    };
                    match &pattern {
                        Pattern::Standart => match &is_nullable {
                            IsNullable::False => quote! {#ValueSc.0.clone().into()},
                            IsNullable::True => gen_iter_or_match_ts(
                                is_nullable,
                                &ident_not_null_ts,
                                is_nullable
                            ),
                        },
                        Pattern::ArrDim1 { dim1_is_nullable } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pattern.down_by_1().expect("e84064c3"),
                                    IsNullable::True => pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                        Pattern::ArrDim2 { dim1_is_nullable, .. } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pattern.down_by_1().expect("226c6318"),
                                    IsNullable::True => pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                        Pattern::ArrDim3 { dim1_is_nullable, .. } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pattern.down_by_1().expect("3ae1e9f8"),
                                    IsNullable::True => pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                        Pattern::ArrDim4 { dim1_is_nullable, .. } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pattern.down_by_1().expect("44d51dc5"),
                                    IsNullable::True => pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                    }
                } else {
                    none_ts.clone()
                });
                quote! {#ident_read_only_ids_ucc(#value_init_ts)}
            };
            let read_only_ids_to_opt_value_read_default_opt_some_vec_one_el_ts = {
                let value_init_ts = gen_import_path_value_init_ts(&if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                    quote! {#ident_read_ucc::new(#ValueSc.0.#ValueSc.clone())}
                } else {
                    quote! {#PgCrudCommonDefaultOptSomeVecOneElCall}
                });
                quote! {Some(#value_init_ts)}
            };
            let previous_read_merged_with_opt_update_into_read_ts = quote! {
                #OptUpdateSc.map_or(#ReadSc, |v_f6e37412| #ident_read_ucc(v_f6e37412.into()))
            };
            let read_only_ids_merged_with_create_into_read_ts = {
                let content_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ident_origin_ucc::new(#ReadOnlyIdsSc.0.#ValueSc)}
                } else {
                    quote! {#CreateSc.into()}
                };
                quote! {#ident_read_ucc(#content_ts)}
            };
            let read_only_ids_merged_with_create_into_opt_value_read_ts = {
                let value_init_ts = gen_import_path_value_init_ts(&quote! {
                    <Self as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoReadSc(
                        #ReadOnlyIdsSc,
                        #CreateSc
                    )
                });
                quote! {Some(#value_init_ts)}
            };
            let read_only_ids_merged_with_create_into_table_type_ts = {
                let content_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ident_origin_ucc::new(#ReadOnlyIdsSc.0.#ValueSc)}
                } else {
                    quote! {#CreateSc.into()}
                };
                quote! {#ident_table_type_ucc(#content_ts)}
            };
            let read_only_ids_merged_with_create_into_where_equal_ts = {
                let gen_equal_ts = |content_ts: &dyn ToTokens| {
                    quote! {
                        where_filters::PgJsonTypeWhereEqual {
                            logical_operator: #import_path::LogicalOperator::Or,
                            #ValueSc: #content_ts
                        }
                    }
                };
                match &is_nullable {
                    IsNullable::False => {
                        let equal_ts = gen_equal_ts(&quote! {#ident_table_type_ucc::new(#CreateSc.0.into())});
                        quote! {#ident_where_ucc::#EqualUcc(#equal_ts)}
                    }
                    IsNullable::True => {
                        let ident_where_ucc_029b3848 = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                        let ident_table_type_ucc_db49334a = SelfTableTypeUcc::from_tokens(&ident_not_null_ts);
                        let equal_ts = gen_equal_ts(&quote! {#ident_table_type_ucc_db49334a::new(v_18544acf.into())});
                        quote! {
                            #import_path::NullableJsonObjectPgTypeWhereFilter(
                                #CreateSc.0.0.map(|v_18544acf| pg_crud_common::NotEmptyUniqueVec::try_new(
                                    vec![#ident_where_ucc_029b3848::#EqualUcc(#equal_ts)]
                                ).expect("88bfa095"))
                            )
                        }
                    }
                }
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = quote! {
                #import_path::NotEmptyUniqueVec::try_new(vec![
                    <Self as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoWhereEqualSc(
                        #ReadOnlyIdsSc,
                        #CreateSc
                    )
                ]).expect("56eb9ad4")
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts = quote! {<Self as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
                #ReadOnlyIdsSc,
                #CreateSc
            )};
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts = gen_arr_dim_equal_ts(&Dim::One);
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts = gen_arr_dim_equal_ts(&Dim::Two);
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts = gen_arr_dim_equal_ts(&Dim::Three);
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts = gen_arr_dim_equal_ts(&Dim::Four);
            //todo maybe reuse LengthEqual and LengthGreaterThan
            let create_into_pg_json_type_opt_vec_where_length_equal_ts = {
                let gen_ts = || {
                    let content_ts = {
                        let create_dot_zero_dot_zero = quote! {#CreateSc.0.0};
                        let content_ts = {
                            let content_ts: &dyn ToTokens = match &is_nullable {
                                IsNullable::False => &create_dot_zero_dot_zero,
                                IsNullable::True => &quote! {v_1bbf74bc.0},
                            };
                            quote! {
                                ::LengthEqual(
                                    where_filters::PgJsonTypeWhereLengthEqual {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #ValueSc: pg_crud_common::UnsignedPartOfI32::try_from(
                                            i32::try_from(#content_ts.len()).expect("64d3424f")
                                        ).expect("081f4463"),
                                    }
                                )
                            }
                        };
                        match &is_nullable {
                            IsNullable::False => quote! {#ident_where_ucc #content_ts},
                            IsNullable::True => {
                                let ident_where_ucc_db49334a = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                                quote! {
                                    #import_path::NullableJsonObjectPgTypeWhereFilter(
                                        match #create_dot_zero_dot_zero {
                                            Some(v_1bbf74bc) => match #import_path::NotEmptyUniqueVec::try_new(
                                                vec![#ident_where_ucc_db49334a #content_ts]
                                            ) {
                                                Ok(v_d82bbdbe) => Some(v_d82bbdbe),
                                                Err(er) => match er {
                                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => {
                                                        return None;
                                                    },
                                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("3d7ce854")
                                                }
                                            },
                                            None => None,
                                        }
                                    )
                                }
                            }
                        }
                    };
                    quote! {
                        match #import_path::NotEmptyUniqueVec::try_new(vec![
                            #content_ts
                        ]) {
                            Ok(v_e196e86d) => Some(v_e196e86d),
                            Err(er) => match er {
                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("e9f9b021")
                            }
                        }
                    }
                };
                match &pattern {
                    Pattern::Standart => none_ts.clone(),
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => gen_ts(),
                }
            };
            let create_into_pg_json_type_opt_vec_where_length_greater_than_ts = {
                let gen_ts = || {
                    let content_ts = {
                        let create_dot_zero_dot_zero = quote! {#CreateSc.0.0};
                        let content_ts = {
                            let content_ts: &dyn ToTokens = match &is_nullable {
                                IsNullable::False => &create_dot_zero_dot_zero,
                                IsNullable::True => &quote! {v_68880991.0},
                            };
                            quote! {
                                ::LengthGreaterThan(
                                    where_filters::PgJsonTypeWhereLengthGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #ValueSc: if let Ok(v_762dae1f) = pg_crud_common::UnsignedPartOfI32::try_from(
                                            if let Ok(v_9dca0200) = i32::try_from(
                                                //todo temp code. make it better checking all cases
                                                match #content_ts.len().checked_sub(1) {
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
                            }
                        };
                        match &is_nullable {
                            IsNullable::False => quote! {#ident_where_ucc #content_ts},
                            IsNullable::True => {
                                let ident_where_ucc_8a412c1a = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                                quote! {
                                    #import_path::NullableJsonObjectPgTypeWhereFilter(match #create_dot_zero_dot_zero {
                                        Some(v_68880991) => match #import_path::NotEmptyUniqueVec::try_new(
                                            vec![#ident_where_ucc_8a412c1a #content_ts]
                                        ) {
                                            Ok(v_cdc120a8) => Some(v_cdc120a8),
                                            Err(er) => match er {
                                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => {
                                                    return None;
                                                },
                                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("584f801e")
                                            }
                                        },
                                        None => None,
                                    })
                                }
                            }
                        }
                    };
                    quote! {
                        match #import_path::NotEmptyUniqueVec::try_new(vec![#content_ts]) {
                            Ok(v_cee8d0ab) => Some(v_cee8d0ab),
                            Err(er) => match er {
                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("497359a5")
                            },
                        }
                    }
                };
                match &pattern {
                    Pattern::Standart => none_ts.clone(),
                    Pattern::ArrDim1 { .. } | Pattern::ArrDim2 { .. } | Pattern::ArrDim3 { .. } | Pattern::ArrDim4 { .. } => gen_ts(),
                }
            };
            let gen_dot_checked_sub_one_ts = |content_ts: &dyn ToTokens|quote!{#content_ts.checked_sub(1)};
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
                    let value = #content_ts - #minus_ts;
                    //The correct way to compare floating point nbrs is to define an allowed er margin
                    if (#content_ts - value).abs() < #more_ts {
                        None
                    }
                    else {
                        value.is_finite().then_some(value)
                    }
                }}
            };
            //todo additonal logic for Option<value> and element of arr? optal element of arr?
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts = if matches!(&pattern, Pattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                let (
                    int_greater_than_one_less_ts,
                    float_32_greater_than_one_less_ts,
                    float_64_greater_than_one_less_ts,
                ) = {
                    let gen_greater_than_one_less_ts = |content_ts: &dyn ToTokens|quote!{
                        let v_7aa498e8 = #content_ts?;
                        match #import_path::NotEmptyUniqueVec::try_new(
                            vec![
                                #import_path::SingleOrMultiple::Single(#ident_where_ucc::GreaterThan(
                                    where_filters::PgJsonTypeWhereGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        value: #ident_table_type_ucc(
                                            #ident_origin_ucc(v_7aa498e8)
                                        ),
                                    }
                                ))
                            ]
                        ) {
                            Ok(v_6f3e23b5) => Some(v_6f3e23b5),
                            Err(er) => match er {
                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => None,
                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("11287f54"),
                            },
                        }
                    };
                    let create_dot_zero_dot_zero_ts = quote!{create.0.0};
                    (
                        gen_greater_than_one_less_ts(&gen_dot_checked_sub_one_ts(
                            &create_dot_zero_dot_zero_ts
                        )),
                        gen_greater_than_one_less_ts(&gen_minus_one_is_finite_then_some_ts(
                            F32OrF64::F32,
                            &create_dot_zero_dot_zero_ts
                        )),
                        gen_greater_than_one_less_ts(&gen_minus_one_is_finite_then_some_ts(
                            F32OrF64::F64,
                            &create_dot_zero_dot_zero_ts
                        )),
                    )
                };
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr |
                    PgJsonType::I16AsJsonbNbr |
                    PgJsonType::I32AsJsonbNbr |
                    PgJsonType::I64AsJsonbNbr |
                    PgJsonType::U8AsJsonbNbr |
                    PgJsonType::U16AsJsonbNbr |
                    PgJsonType::U32AsJsonbNbr |
                    PgJsonType::U64AsJsonbNbr => int_greater_than_one_less_ts,
                    PgJsonType::F32AsJsonbNbr => float_32_greater_than_one_less_ts,
                    PgJsonType::F64AsJsonbNbr => float_64_greater_than_one_less_ts,
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::StringAsJsonbString |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts = if matches!(&pattern, Pattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                let (
                    between_one_less_and_one_more_int_ts,
                    between_one_less_and_one_more_float_ts
                ) = {
                    let gen_between_one_less_and_one_more_ts = |
                        less_ts: &dyn ToTokens,
                        more_ts: &dyn ToTokens
                    |quote!{
                        if let (Some(start), Some(end)) = (#less_ts, #more_ts) {
                            match where_filters::Between::try_new(
                                #ident_table_type_ucc::new(start),
                                #ident_table_type_ucc::new(end)
                            ) {
                                Ok(v_cdde02cc) => match pg_crud_common::NotEmptyUniqueVec::try_new(vec![
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_ucc::Between(
                                            where_filters::PgJsonTypeWhereBetween {
                                                logical_operator: pg_crud_common::LogicalOperator::Or,
                                                value: v_cdde02cc,
                                            }
                                        )
                                    )
                                ]) {
                                    Ok(v_41af48fb) => Some(v_41af48fb),
                                    Err(er) => match er {
                                        #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("5edabfcc")
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
                            let gen_content_ts = |content_ts: &dyn ToTokens|quote!{create.0.0.#content_ts(1)};
                            gen_between_one_less_and_one_more_ts(
                                &gen_content_ts(&quote!{checked_sub}),
                                &gen_content_ts(&quote!{checked_add})
                            )
                        },
                        {
                            let gen_content_ts = |content_ts: &dyn ToTokens|quote!{{
                                let value = create.0.0 #content_ts 1.0;
                                value.is_finite().then_some(value)
                            }};
                            gen_between_one_less_and_one_more_ts(
                                &gen_content_ts(&quote!{-}),
                                &gen_content_ts(&quote!{+})
                            )
                        }
                    )
                };
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr |
                    PgJsonType::I16AsJsonbNbr |
                    PgJsonType::I32AsJsonbNbr |
                    PgJsonType::I64AsJsonbNbr |
                    PgJsonType::U8AsJsonbNbr |
                    PgJsonType::U16AsJsonbNbr |
                    PgJsonType::U32AsJsonbNbr |
                    PgJsonType::U64AsJsonbNbr => between_one_less_and_one_more_int_ts,
                    PgJsonType::F32AsJsonbNbr |
                    PgJsonType::F64AsJsonbNbr => between_one_less_and_one_more_float_ts,
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::StringAsJsonbString |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone()
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts = if matches!(&pattern, Pattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr |
                    PgJsonType::I16AsJsonbNbr |
                    PgJsonType::I32AsJsonbNbr |
                    PgJsonType::I64AsJsonbNbr |
                    PgJsonType::U8AsJsonbNbr |
                    PgJsonType::U16AsJsonbNbr |
                    PgJsonType::U32AsJsonbNbr |
                    PgJsonType::U64AsJsonbNbr |
                    PgJsonType::F32AsJsonbNbr |
                    PgJsonType::F64AsJsonbNbr => {
                        //todo additional vrts to test
                        quote!{
                            match #import_path::NotEmptyUniqueVec::try_new(vec![
                                #import_path::SingleOrMultiple::Single(
                                    #ident_where_ucc::In(
                                        where_filters::PgJsonTypeWhereIn {
                                            logical_operator: #import_path::LogicalOperator::Or,
                                            value: where_filters::PgJsonTypeNotEmptyUniqueVec::try_new(
                                                vec![#ident_table_type_ucc::new(#CreateSc.0.0)]
                                            ).expect("2737c0ed")
                                        }
                                    ),
                                )
                            ]) {
                                Ok(v_1c4f89a4) => Some(v_1c4f89a4),
                                Err(er) => match er {
                                    #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                    #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("16ae359d")
                                }
                            }
                        }
                    },
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::StringAsJsonbString |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone()
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regular_expression_ts = if matches!(&pattern, Pattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNbr |
                    PgJsonType::I16AsJsonbNbr |
                    PgJsonType::I32AsJsonbNbr |
                    PgJsonType::I64AsJsonbNbr |
                    PgJsonType::U8AsJsonbNbr |
                    PgJsonType::U16AsJsonbNbr |
                    PgJsonType::U32AsJsonbNbr |
                    PgJsonType::U64AsJsonbNbr |
                    PgJsonType::F32AsJsonbNbr |
                    PgJsonType::F64AsJsonbNbr |
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                    PgJsonType::StringAsJsonbString => quote!{
                        match #import_path::NotEmptyUniqueVec::try_new(vec![
                            #import_path::SingleOrMultiple::Single(
                                #ident_where_ucc::RegularExpression(
                                    where_filters::PgJsonTypeWhereRegularExpression {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        regular_expression_case: where_filters::RegularExpressionCase::Sensitive,
                                        value: where_filters::RegexRegex(regex::Regex::new(&format!("^{}$", regex::escape(&#CreateSc.0.0))).expect("3814ff38")),
                                    }
                                ),
                            )
                        ]) {
                            Ok(v_75ae8964) => Some(v_75ae8964),
                            Err(er) => match er {
                                #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("b9713787")
                            }
                        }
                    },
                }
            }
            else {
                none_ts.clone()
            };
            //todo add contains_el_greater_than for dim 2,3,4
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts = match &pattern {
                Pattern::ArrDim1 { dim1_is_nullable } => {
                    if matches!((&is_nullable, &dim1_is_nullable), (
                        IsNullable::False,
                        IsNullable::False
                    )) {
                        let (
                            int_greater_than_one_less_ts,
                            float_32_greater_than_one_less_ts,
                            float_64_greater_than_one_less_ts,
                        ) = {
                            let gen_greater_than_one_less_ts = |content_ts: &dyn ToTokens|quote!{
                                match #import_path::NotEmptyUniqueVec::try_new({
                                    let mut acc_f95ec4f2 = vec![];
                                    for el_ba78af60 in create.0.0 {
                                        let v_027d0d3a = #content_ts;
                                        match v_027d0d3a {
                                            Some(v_0cd93c25) => {
                                                acc_f95ec4f2.push(
                                                    #import_path::SingleOrMultiple::Single(
                                                        #ident_where_ucc::ContainsElGreaterThan(
                                                            where_filters::PgJsonTypeWhereContainsElGreaterThan {
                                                                logical_operator: #import_path::LogicalOperator::Or,
                                                                value: #ident_standart_not_null_table_type_ucc(
                                                                    #ident_standart_not_null_origin_ucc(v_0cd93c25)
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
                                }) {
                                    Ok(v_69c93ec5) => Some(v_69c93ec5),
                                    Err(er) => match er {
                                        #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("47e44ecd")
                                    }
                                }
                            };
                            let el_dot_zero_ts = quote!{el_ba78af60.0};
                            (
                                gen_greater_than_one_less_ts(&gen_dot_checked_sub_one_ts(
                                    &el_dot_zero_ts
                                )),
                                gen_greater_than_one_less_ts(&gen_minus_one_is_finite_then_some_ts(
                                    F32OrF64::F32,
                                    &el_dot_zero_ts
                                )),
                                gen_greater_than_one_less_ts(&gen_minus_one_is_finite_then_some_ts(
                                    F32OrF64::F64,
                                    &el_dot_zero_ts
                                )),
                            )
                        };
                        match &pg_json_type {
                            PgJsonType::I8AsJsonbNbr |
                            PgJsonType::I16AsJsonbNbr |
                            PgJsonType::I32AsJsonbNbr |
                            PgJsonType::I64AsJsonbNbr |
                            PgJsonType::U8AsJsonbNbr |
                            PgJsonType::U16AsJsonbNbr |
                            PgJsonType::U32AsJsonbNbr |
                            PgJsonType::U64AsJsonbNbr => int_greater_than_one_less_ts,
                            PgJsonType::F32AsJsonbNbr => float_32_greater_than_one_less_ts,
                            PgJsonType::F64AsJsonbNbr => float_64_greater_than_one_less_ts,
                            PgJsonType::BoolAsJsonbBoolean |
                            PgJsonType::StringAsJsonbString |
                            PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                        }
                    }
                    else {
                        none_ts.clone()
                    }
                },
                Pattern::Standart |
                Pattern::ArrDim2 {..} |
                Pattern::ArrDim3 {..} |
                Pattern::ArrDim4 {..} => none_ts.clone()
            };
            //todo add contains_el_regular_expression for dim 2,3,4
            let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regular_expression_ts = match &pattern {
                Pattern::ArrDim1 { dim1_is_nullable } => {
                    if matches!((&is_nullable, &dim1_is_nullable), (
                        IsNullable::False,
                        IsNullable::False
                    )) {
                        match &pg_json_type {
                            PgJsonType::I8AsJsonbNbr |
                            PgJsonType::I16AsJsonbNbr |
                            PgJsonType::I32AsJsonbNbr |
                            PgJsonType::I64AsJsonbNbr |
                            PgJsonType::U8AsJsonbNbr |
                            PgJsonType::U16AsJsonbNbr |
                            PgJsonType::U32AsJsonbNbr |
                            PgJsonType::U64AsJsonbNbr |
                            PgJsonType::F32AsJsonbNbr |
                            PgJsonType::F64AsJsonbNbr |
                            PgJsonType::BoolAsJsonbBoolean |
                            PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                            PgJsonType::StringAsJsonbString => quote!{
                                match #import_path::NotEmptyUniqueVec::try_new(create.0.0.into_iter().map(|el_590fca71| {
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_ucc::ContainsElRegularExpression(
                                            where_filters::PgJsonTypeWhereContainsElRegularExpression {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                regular_expression_case:
                                                    where_filters::RegularExpressionCase::Sensitive,
                                                value: where_filters::RegexRegex(
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
                                .collect()) {
                                    Ok(v_0363f494) => Some(v_0363f494),
                                    Err(er) => match er {
                                        #import_path::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("415a73d9")
                                    }
                                }
                            }
                        }
                    }
                    else {
                        none_ts.clone()
                    }
                },
                Pattern::Standart |
                Pattern::ArrDim2 {..} |
                Pattern::ArrDim3 {..} |
                Pattern::ArrDim4 {..} => none_ts.clone()
            };
            gen_impl_pg_json_type_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &pg_crud_macros_common_import_path_pg_crud_common,
                &ident_read_inner_ucc,
                &ident,
                &opt_vec_create_ts,
                &read_only_ids_to_two_dimal_vec_read_inner_ts,
                &read_inner_into_read_with_new_or_try_new_unwraped_ts,
                &read_inner_into_update_with_new_or_try_new_unwraped_ts,
                &read_only_ids_into_opt_value_read_inner_ts,
                &update_to_read_only_ids_ts,
                &read_only_ids_to_opt_value_read_default_opt_some_vec_one_el_ts,
                &previous_read_merged_with_opt_update_into_read_ts,
                &read_only_ids_merged_with_create_into_read_ts,
                &read_only_ids_merged_with_create_into_opt_value_read_ts,
                &read_only_ids_merged_with_create_into_table_type_ts,
                &read_only_ids_merged_with_create_into_where_equal_ts,
                &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
                &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts,
                &create_into_pg_json_type_opt_vec_where_length_equal_ts,
                &create_into_pg_json_type_opt_vec_where_length_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regular_expression_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regular_expression_ts,
            )
        };
        let generated = quote! {
            #ident_ts
            #ident_origin_ts
            #ident_table_type_ts
            #ident_create_ts
            #ident_create_for_query_ts
            #ident_select_ts
            #ident_where_ts
            #ident_read_ts
            #ident_read_only_ids_ts
            #ident_read_inner_ts
            #ident_update_ts
            #ident_update_for_query_ts
            #impl_pg_json_type_for_ident_ts
            #maybe_impl_pg_json_type_object_vec_el_id_for_ident_origin_ts
            #impl_pg_json_type_test_cases_for_ident_ts
        };
        (
            {
                let field_ident = format!("field_{index}").parse::<Ts2>().expect("f992f797");
                quote! {pub #field_ident: #ident,}.to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    maybe_write_ts_into_file(
        config.pg_table_columns_content_write_into_pg_table_columns_using_pg_json_types,
        "pg_table_columns_using_pg_json_types",
        &{
            let fields_content_ts = fields_ts
                .into_iter()
                .map(|el_7ec253fa| el_7ec253fa.parse::<Ts2>().expect("1d8cd8e4"))
                .collect::<Vec<Ts2>>();
            quote! {
                pub struct PgTableColumnsUsingPgJsonTypes {
                    #(#fields_content_ts)*
                }
            }
        },
        &FormatWithCargofmt::True,
    );
    let generated = {
        let content_ts = pg_json_type_arr
            .into_iter()
            .map(|el_af9caefa| el_af9caefa.parse::<Ts2>().expect("84e21b40"))
            .collect::<Vec<Ts2>>();
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #GenPgJsonTypesModSc {
                #(#content_ts)*
            }
            pub use #GenPgJsonTypesModSc::*;
        }
    };
    maybe_write_ts_into_file(
        config.whole_content_write_into_gen_pg_json_types,
        "gen_pg_json_types",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
