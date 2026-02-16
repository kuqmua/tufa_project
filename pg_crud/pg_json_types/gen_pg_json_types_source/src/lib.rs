use enum_extension_lib::EnumExtension;
use gen_quotes::double_quotes_ts;
use macros_helpers::{
    DeriveCopy, DeriveSchemarsJsonSchema, FormatWithCargofmt, ShouldWriteTokenStreamIntoFile,
    StructOrEnumDeriveTokenStreamBuilder, gen_impl_display_ts, gen_impl_from_ts,
    gen_impl_to_err_string_ts, gen_pub_const_new_ts, gen_pub_new_ts, maybe_write_ts_into_file,
};
use naming::{
    ArrayOfUcc, AsUcc, BooleanUcc, ColumnNameAndMaybeFieldGetterSc, CreateForQueryUcc, CreateSc,
    EqualUcc, ErrorSc, GenPgJsonTypesModSc, IncrementSc, JsonbSetAccumulatorSc, NewSc, NumberUcc,
    OptionUpdateSc, OptionVecCreateSc, PgJsonTypeUcc, QuerySc, ReadInnerUcc,
    ReadOnlyIdsMergedWithCreateIntoReadSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc, ReadOnlyIdsSc,
    ReadOnlyIdsToTwoDimalVecReadInnerSc, ReadSc, SelfSc, SelfUcc, StringUcc, UpdateForQueryUcc,
    UpdateUcc, ValueSc, VecOfUcc,
    parameter::{
        JsonbSelfUcc, SelfCreateForQueryUcc, SelfCreateUcc, SelfOriginUcc, SelfReadInnerUcc,
        SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc, SelfTableTypeDeclarationUcc,
        SelfUpdateForQueryUcc, SelfUpdateUcc, SelfWhereUcc,
    },
};
use panic_location::panic_location;
use pg_crud_macros_common::{
    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, Dim, DimIndexNumber, ImportPath, IsNullable,
    IsQueryBindMutable, IsSelectOnlyCreatedIdsQueryBindMutable,
    IsSelectOnlyUpdatedIdsQueryBindMutable,
    IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed,
    IsSelectQueryPartIsPgTypeUsed, IsSelectQueryPartSelfSelectUsed, IsStandartNotNull,
    IsUpdateQueryBindMutable, IsUpdateQueryPartJsonbSetTargetUsed, IsUpdateQueryPartSelfUpdateUsed,
    PgFilter, PgJsonTypeFilter, ReadOrUpdate, ShouldDeriveSchemarsJsonSchema,
    ShouldDeriveUtoipaToSchema, gen_impl_crate_is_string_empty_for_ident_content_ts,
    gen_impl_pg_crud_common_default_option_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_common_default_option_some_vec_one_el_ts,
    gen_impl_pg_json_type_test_cases_for_ident_ts, gen_impl_pg_json_type_ts,
    gen_impl_sqlx_encode_sqlx_pg_for_ident_ts, gen_impl_sqlx_type_sqlx_pg_for_ident_ts,
    gen_pg_type_where_ts, gen_sqlx_types_json_type_declaration_ts,
    gen_std_option_option_tokens_declaration_ts, gen_std_vec_vec_tokens_declaration_ts,
    gen_value_initialization_ts,
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
    PgCrudCommonDefaultOptionSomeVecOneEl, PgCrudCommonDefaultOptionSomeVecOneElCall,
    PgCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall, StdStringString, U8, U16, U32, U64,
    UuidUuid,
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
        StdStringString,
        UuidUuid,
    }
    impl From<&PgJsonType> for RustTypeName {
        fn from(value: &PgJsonType) -> Self {
            match &value {
                PgJsonType::I8AsJsonbNumber => Self::I8,
                PgJsonType::I16AsJsonbNumber => Self::I16,
                PgJsonType::I32AsJsonbNumber => Self::I32,
                PgJsonType::I64AsJsonbNumber => Self::I64,
                PgJsonType::U8AsJsonbNumber => Self::U8,
                PgJsonType::U16AsJsonbNumber => Self::U16,
                PgJsonType::U32AsJsonbNumber => Self::U32,
                PgJsonType::U64AsJsonbNumber => Self::U64,
                PgJsonType::F32AsJsonbNumber => Self::F32,
                PgJsonType::F64AsJsonbNumber => Self::F64,
                PgJsonType::BoolAsJsonbBoolean => Self::Bool,
                PgJsonType::StdStringStringAsJsonbString => Self::StdStringString,
                PgJsonType::UuidUuidAsJsonbString => Self::UuidUuid,
            }
        }
    }
    #[derive(Debug)]
    enum PgJsonTypeName {
        Boolean,
        Number,
        String,
    }
    impl Display for PgJsonTypeName {
        fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
            write!(
                f,
                "{}",
                JsonbSelfUcc::from_display(match &self {
                    Self::Number => &NumberUcc,
                    Self::Boolean => &BooleanUcc,
                    Self::String => &StringUcc,
                })
            )
        }
    }
    impl From<&PgJsonType> for PgJsonTypeName {
        fn from(value: &PgJsonType) -> Self {
            match &value {
                PgJsonType::I8AsJsonbNumber
                | PgJsonType::I16AsJsonbNumber
                | PgJsonType::I32AsJsonbNumber
                | PgJsonType::I64AsJsonbNumber
                | PgJsonType::U8AsJsonbNumber
                | PgJsonType::U16AsJsonbNumber
                | PgJsonType::U32AsJsonbNumber
                | PgJsonType::U64AsJsonbNumber
                | PgJsonType::F32AsJsonbNumber
                | PgJsonType::F64AsJsonbNumber => Self::Number,
                PgJsonType::BoolAsJsonbBoolean => Self::Boolean,
                PgJsonType::StdStringStringAsJsonbString | PgJsonType::UuidUuidAsJsonbString => {
                    Self::String
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter, EnumExtension,
    )]
    enum PgJsonType {
        I8AsJsonbNumber,
        I16AsJsonbNumber,
        I32AsJsonbNumber,
        I64AsJsonbNumber,
        U8AsJsonbNumber,
        U16AsJsonbNumber,
        U32AsJsonbNumber,
        U64AsJsonbNumber,
        F32AsJsonbNumber,
        F64AsJsonbNumber,
        BoolAsJsonbBoolean,
        StdStringStringAsJsonbString,
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
    enum PgJsonTypePattern {
        Standart,
        ArrayDim1 {
            dim1_is_nullable: IsNullable,
        },
        ArrayDim2 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
        },
        ArrayDim3 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
            dim3_is_nullable: IsNullable,
        },
        ArrayDim4 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
            dim3_is_nullable: IsNullable,
            dim4_is_nullable: IsNullable,
        },
    }
    impl PgJsonTypePattern {
        const fn down_by_1(&self) -> Option<Self> {
            match &self {
                Self::Standart => None,
                Self::ArrayDim1 { .. } => Some(Self::Standart),
                Self::ArrayDim2 {
                    dim2_is_nullable, ..
                } => Some(Self::ArrayDim1 {
                    dim1_is_nullable: *dim2_is_nullable,
                }),
                Self::ArrayDim3 {
                    dim2_is_nullable,
                    dim3_is_nullable,
                    ..
                } => Some(Self::ArrayDim2 {
                    dim1_is_nullable: *dim2_is_nullable,
                    dim2_is_nullable: *dim3_is_nullable,
                }),
                Self::ArrayDim4 {
                    dim2_is_nullable,
                    dim3_is_nullable,
                    dim4_is_nullable,
                    ..
                } => Some(Self::ArrayDim3 {
                    dim1_is_nullable: *dim2_is_nullable,
                    dim2_is_nullable: *dim3_is_nullable,
                    dim3_is_nullable: *dim4_is_nullable,
                }),
            }
        }
        const fn down_by_2(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDim1 { .. } => None,
                Self::ArrayDim2 { .. } => Some(Self::Standart),
                Self::ArrayDim3 {
                    dim3_is_nullable, ..
                } => Some(Self::ArrayDim1 {
                    dim1_is_nullable: *dim3_is_nullable,
                }),
                Self::ArrayDim4 {
                    dim3_is_nullable,
                    dim4_is_nullable,
                    ..
                } => Some(Self::ArrayDim2 {
                    dim1_is_nullable: *dim3_is_nullable,
                    dim2_is_nullable: *dim4_is_nullable,
                }),
            }
        }
        const fn down_by_3(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDim1 { .. } | Self::ArrayDim2 { .. } => None,
                Self::ArrayDim3 { .. } => Some(Self::Standart),
                Self::ArrayDim4 {
                    dim4_is_nullable, ..
                } => Some(Self::ArrayDim1 {
                    dim1_is_nullable: *dim4_is_nullable,
                }),
            }
        }
        const fn down_by_4(&self) -> Option<Self> {
            match &self {
                Self::Standart
                | Self::ArrayDim1 { .. }
                | Self::ArrayDim2 { .. }
                | Self::ArrayDim3 { .. } => None,
                Self::ArrayDim4 { .. } => Some(Self::Standart),
            }
        }
    }
    enum ArrayDim {
        ArrayDim1,
        ArrayDim2 {
            dim1_is_nullable: IsNullable,
        },
        ArrayDim3 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
        },
        ArrayDim4 {
            dim1_is_nullable: IsNullable,
            dim2_is_nullable: IsNullable,
            dim3_is_nullable: IsNullable,
        },
    }
    impl ArrayDim {
        const fn to_usize(&self) -> usize {
            match &self {
                Self::ArrayDim1 { .. } => 1,
                Self::ArrayDim2 { .. } => 2,
                Self::ArrayDim3 { .. } => 3,
                Self::ArrayDim4 { .. } => 4,
            }
        }
    }
    impl TryFrom<&PgJsonTypePattern> for ArrayDim {
        type Error = ();
        fn try_from(value: &PgJsonTypePattern) -> Result<Self, Self::Error> {
            match &value {
                PgJsonTypePattern::Standart => Err(()),
                PgJsonTypePattern::ArrayDim1 { .. } => Ok(Self::ArrayDim1),
                PgJsonTypePattern::ArrayDim2 {
                    dim1_is_nullable, ..
                } => Ok(Self::ArrayDim2 {
                    dim1_is_nullable: *dim1_is_nullable,
                }),
                PgJsonTypePattern::ArrayDim3 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    ..
                } => Ok(Self::ArrayDim3 {
                    dim1_is_nullable: *dim1_is_nullable,
                    dim2_is_nullable: *dim2_is_nullable,
                }),
                PgJsonTypePattern::ArrayDim4 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                    ..
                } => Ok(Self::ArrayDim4 {
                    dim1_is_nullable: *dim1_is_nullable,
                    dim2_is_nullable: *dim2_is_nullable,
                    dim3_is_nullable: *dim3_is_nullable,
                }),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct PgJsonTypeRecord {
        pg_json_type: PgJsonType,
        is_nullable: IsNullable,
        pg_json_type_pattern: PgJsonTypePattern,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize)]
    enum GenPgJsonTypesConfigVariant {
        All,
        WithoutDims,
        WithDimOne,
        WithDimTwo,
        WithDimThree,
        WithDimFour,
        Concrete(Vec<PgJsonTypeRecord>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize)]
    struct GenPgJsonTypesConfig {
        pg_table_columns_content_write_into_pg_table_columns_using_pg_json_types:
            ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_pg_json_types: ShouldWriteTokenStreamIntoFile,
        variant: GenPgJsonTypesConfigVariant,
    }
    panic_location();
    let gen_pg_json_types_config =
        from_str::<GenPgJsonTypesConfig>(&input_ts.to_string()).expect("1123f78f");
    let (fields_ts, pg_json_type_array) = {
        {
            let gen_standart = |acc_29796d99: &mut Vec<PgJsonTypeRecord>, pg_json_type: PgJsonType|{
                for el_2f39e657 in IsNullable::into_array() {
                    acc_29796d99.push(PgJsonTypeRecord {
                        pg_json_type: pg_json_type.clone(),
                        is_nullable: el_2f39e657,
                        pg_json_type_pattern: PgJsonTypePattern::Standart,
                    });
                }
            };
            let gen_array_dim1 = |acc_5b22a398: &mut Vec<PgJsonTypeRecord>, pg_json_type: PgJsonType|{
                for el_29854486 in IsNullable::into_array() {
                    for el_6440cc9c in IsNullable::into_array() {
                        acc_5b22a398.push(PgJsonTypeRecord {
                            pg_json_type: pg_json_type.clone(),
                            is_nullable: el_29854486,
                            pg_json_type_pattern: PgJsonTypePattern::ArrayDim1 { dim1_is_nullable: el_6440cc9c },
                        });
                    }
                }
            };
            let gen_array_dim2 = |acc_e59f7158: &mut Vec<PgJsonTypeRecord>, pg_json_type: PgJsonType|{
                for el_a6ba4c3e in IsNullable::into_array() {
                    for el_4b5a815d in IsNullable::into_array() {
                        for el_2e4896dd in IsNullable::into_array() {
                            acc_e59f7158.push(PgJsonTypeRecord {
                                pg_json_type: pg_json_type.clone(),
                                is_nullable: el_a6ba4c3e,
                                pg_json_type_pattern: PgJsonTypePattern::ArrayDim2 {
                                    dim1_is_nullable: el_4b5a815d,
                                    dim2_is_nullable: el_2e4896dd
                                },
                            });
                        }
                    }
                }
            };
            let gen_array_dim3 = |acc_77498dc3: &mut Vec<PgJsonTypeRecord>, pg_json_type: PgJsonType|{
                for el_8f03b1c2 in IsNullable::into_array() {
                    for el_a27b642f in IsNullable::into_array() {
                        for el_dc57e9b7 in IsNullable::into_array() {
                            for el_4361fee5 in IsNullable::into_array() {
                                acc_77498dc3.push(PgJsonTypeRecord {
                                    pg_json_type: pg_json_type.clone(),
                                    is_nullable: el_8f03b1c2,
                                    pg_json_type_pattern: PgJsonTypePattern::ArrayDim3 {
                                        dim1_is_nullable: el_a27b642f,
                                        dim2_is_nullable: el_dc57e9b7,
                                        dim3_is_nullable: el_4361fee5,
                                    },
                                });
                            }
                        }
                    }
                }
            };
            let gen_array_dim4 = |acc_7c8a3329: &mut Vec<PgJsonTypeRecord>, pg_json_type: PgJsonType|{
                for el_daf10957 in IsNullable::into_array() {
                    for el_fc5a53dd in IsNullable::into_array() {
                        for el_69b59c5c in IsNullable::into_array() {
                            for el_d7efbd09 in IsNullable::into_array() {
                                for el_c16cb65b in IsNullable::into_array() {
                                    acc_7c8a3329.push(PgJsonTypeRecord {
                                        pg_json_type: pg_json_type.clone(),
                                        is_nullable: el_daf10957,
                                        pg_json_type_pattern: PgJsonTypePattern::ArrayDim4 {
                                            dim1_is_nullable: el_fc5a53dd,
                                            dim2_is_nullable: el_69b59c5c,
                                            dim3_is_nullable: el_d7efbd09,
                                            dim4_is_nullable: el_c16cb65b,
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
            };
            let acc_d97120ed = match gen_pg_json_types_config.variant {
                GenPgJsonTypesConfigVariant::All => PgJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_e2f65a79, pg_json_type| {
                    for el_644643cd in PgJsonTypePattern::into_array() {
                        match &el_644643cd {
                            PgJsonTypePattern::Standart => gen_standart(&mut acc_e2f65a79, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim1 { .. } => gen_array_dim1(&mut acc_e2f65a79, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim2 { .. } => gen_array_dim2(&mut acc_e2f65a79, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim3 { .. } => gen_array_dim3(&mut acc_e2f65a79, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim4 { .. } => gen_array_dim4(&mut acc_e2f65a79, pg_json_type.clone()),
                        }
                    }
                    acc_e2f65a79
                }),
                GenPgJsonTypesConfigVariant::WithoutDims => PgJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_3d95645c, pg_json_type| {
                    for el_fccf1979 in PgJsonTypePattern::into_array() {
                        match &el_fccf1979 {
                            PgJsonTypePattern::Standart => gen_standart(&mut acc_3d95645c, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim1 { .. } |
                            PgJsonTypePattern::ArrayDim2 { .. } |
                            PgJsonTypePattern::ArrayDim3 { .. } |
                            PgJsonTypePattern::ArrayDim4 { .. } => (),
                        }
                    }
                    acc_3d95645c
                }),
                GenPgJsonTypesConfigVariant::WithDimOne => PgJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_66a17cae, pg_json_type| {
                    for el_e69bd1fc in PgJsonTypePattern::into_array() {
                        match &el_e69bd1fc {
                            PgJsonTypePattern::Standart => gen_standart(&mut acc_66a17cae, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim1 { .. } => gen_array_dim1(&mut acc_66a17cae, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim2 { .. } |
                            PgJsonTypePattern::ArrayDim3 { .. } |
                            PgJsonTypePattern::ArrayDim4 { .. } => (),
                        }
                    }
                    acc_66a17cae
                }),
                GenPgJsonTypesConfigVariant::WithDimTwo => PgJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_c5ffb796, pg_json_type| {
                    for el_345fd6bd in PgJsonTypePattern::into_array() {
                        match &el_345fd6bd {
                            PgJsonTypePattern::Standart => gen_standart(&mut acc_c5ffb796, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim1 { .. } => gen_array_dim1(&mut acc_c5ffb796, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim2 { .. } => gen_array_dim2(&mut acc_c5ffb796, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim3 { .. } |
                            PgJsonTypePattern::ArrayDim4 { .. } => (),
                        }
                    }
                    acc_c5ffb796
                }),
                GenPgJsonTypesConfigVariant::WithDimThree => PgJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_78b27c00, pg_json_type| {
                    for el_88e3b8fe in PgJsonTypePattern::into_array() {
                        match &el_88e3b8fe {
                            PgJsonTypePattern::Standart => gen_standart(&mut acc_78b27c00, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim1 { .. } => gen_array_dim1(&mut acc_78b27c00, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim2 { .. } => gen_array_dim2(&mut acc_78b27c00, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim3 { .. } => gen_array_dim3(&mut acc_78b27c00, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim4 { .. } => (),
                        }
                    }
                    acc_78b27c00
                }),
                GenPgJsonTypesConfigVariant::WithDimFour => PgJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_172c62ad, pg_json_type| {
                    for el_80434642 in PgJsonTypePattern::into_array() {
                        match &el_80434642 {
                            PgJsonTypePattern::Standart => gen_standart(&mut acc_172c62ad, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim1 { .. } => gen_array_dim1(&mut acc_172c62ad, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim2 { .. } => gen_array_dim2(&mut acc_172c62ad, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim3 { .. } => gen_array_dim3(&mut acc_172c62ad, pg_json_type.clone()),
                            PgJsonTypePattern::ArrayDim4 { .. } => gen_array_dim4(&mut acc_172c62ad, pg_json_type.clone()),
                        }
                    }
                    acc_172c62ad
                }),
                GenPgJsonTypesConfigVariant::Concrete(value) => value,
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
                struct PgJsonTypeRecordHandle {
                    is_nullable: IsNullable,
                    pg_json_type_pattern: PgJsonTypePattern,
                }
                fn gen_pg_json_type_record_handle_vec(pg_json_type_record_handle: PgJsonTypeRecordHandle) -> Vec<PgJsonTypeRecordHandle> {
                    let gen_vec = |pg_json_type_record_handle_1e4b61e4: PgJsonTypeRecordHandle| gen_pg_json_type_record_handle_vec(
                        pg_json_type_record_handle_1e4b61e4
                    ).into_iter().chain(once(pg_json_type_record_handle.clone())).collect();
                    match (&pg_json_type_record_handle.is_nullable, &pg_json_type_record_handle.pg_json_type_pattern) {
                        (IsNullable::False, PgJsonTypePattern::Standart) => vec![pg_json_type_record_handle],
                        (IsNullable::True, PgJsonTypePattern::Standart) => gen_vec(PgJsonTypeRecordHandle {
                            is_nullable: IsNullable::False,
                            pg_json_type_pattern: PgJsonTypePattern::Standart,
                        }),
                        (IsNullable::False, PgJsonTypePattern::ArrayDim1 { dim1_is_nullable }) => gen_vec(PgJsonTypeRecordHandle {
                            is_nullable: *dim1_is_nullable,
                            pg_json_type_pattern: pg_json_type_record_handle.pg_json_type_pattern.down_by_1().expect("0e970a4f"),
                        }),
                        (IsNullable::False, PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, .. }) => gen_vec(PgJsonTypeRecordHandle {
                            is_nullable: *dim1_is_nullable,
                            pg_json_type_pattern: pg_json_type_record_handle.pg_json_type_pattern.down_by_1().expect("85f8ed83"),
                        }),
                        (
                            IsNullable::False,
                            PgJsonTypePattern::ArrayDim3 {
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                            },
                        ) => gen_vec(PgJsonTypeRecordHandle {
                            is_nullable: *dim1_is_nullable,
                            pg_json_type_pattern: PgJsonTypePattern::ArrayDim2 {
                                dim1_is_nullable: *dim2_is_nullable,
                                dim2_is_nullable: *dim3_is_nullable,
                            },
                        }),
                        (
                            IsNullable::False,
                            PgJsonTypePattern::ArrayDim4 {
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                                dim4_is_nullable,
                            },
                        ) => gen_vec(PgJsonTypeRecordHandle {
                            is_nullable: *dim1_is_nullable,
                            pg_json_type_pattern: PgJsonTypePattern::ArrayDim3 {
                                dim1_is_nullable: *dim2_is_nullable,
                                dim2_is_nullable: *dim3_is_nullable,
                                dim3_is_nullable: *dim4_is_nullable,
                            },
                        }),
                        (IsNullable::True, PgJsonTypePattern::ArrayDim1 { .. } | PgJsonTypePattern::ArrayDim2 { .. } | PgJsonTypePattern::ArrayDim3 { .. } | PgJsonTypePattern::ArrayDim4 { .. }) => gen_vec(PgJsonTypeRecordHandle {
                            is_nullable: IsNullable::False,
                            pg_json_type_pattern: pg_json_type_record_handle.pg_json_type_pattern.clone(),
                        }),
                    }
                }
                gen_pg_json_type_record_handle_vec(PgJsonTypeRecordHandle {
                    is_nullable: el_c4f9bf8f.is_nullable,
                    pg_json_type_pattern: el_c4f9bf8f.pg_json_type_pattern,
                })
            } {
                let pg_json_type_record = PgJsonTypeRecord {
                    pg_json_type: el_c4f9bf8f.pg_json_type.clone(),
                    is_nullable: el_7ae8d2ae.is_nullable,
                    pg_json_type_pattern: el_7ae8d2ae.pg_json_type_pattern,
                };
                if !acc_5e43269c.contains(&pg_json_type_record) {
                    acc_5e43269c.push(pg_json_type_record);
                }
            }
            acc_5e43269c
        })
    }
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PgJsonTypeRecord)>>()
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
        // struct SchemaObjectTokenStream<'lifetime> {
        //     metadata: &'lifetime dyn ToTokens,
        //     instance_type: &'lifetime dyn ToTokens,
        //     format: &'lifetime dyn ToTokens,
        //     enum_values: &'lifetime dyn ToTokens,
        //     const_value: &'lifetime dyn ToTokens,
        //     subschemas: &'lifetime dyn ToTokens,
        //     number: &'lifetime dyn ToTokens,
        //     string: &'lifetime dyn ToTokens,
        //     array: &'lifetime dyn ToTokens,
        //     object: &'lifetime dyn ToTokens,
        //     reference: &'lifetime dyn ToTokens,
        //     extensions: &'lifetime dyn ToTokens,
        // }
        // enum SchemarsJsonSchema<'schema_object_ts_tifetime> {
        //     Derive,
        //     Impl(SchemaObjectTokenStream<'schema_object_ts_tifetime>),
        // }
        let pg_json_type = &el_1d376874.pg_json_type;
        let is_nullable = &el_1d376874.is_nullable;
        let pg_json_type_pattern = &el_1d376874.pg_json_type_pattern;
        let rust_type_name = RustTypeName::from(pg_json_type);
        let pg_json_type_name = PgJsonTypeName::from(pg_json_type);
        let is_standart_not_null = if matches!((&pg_json_type_pattern, &is_nullable), (PgJsonTypePattern::Standart, IsNullable::False)) {
            IsStandartNotNull::True
        } else {
            IsStandartNotNull::False
        };
        let is_standart_not_null_uuid = if matches!((&is_nullable, &pg_json_type_pattern, &pg_json_type), (IsNullable::False, PgJsonTypePattern::Standart, PgJsonType::UuidUuidAsJsonbString)) {
            IsStandartNotNullUuid::True
        } else {
            IsStandartNotNullUuid::False
        };
        let import_path = ImportPath::PgCrudCommon;
        let none_ts = quote! {None};
        let u64_ts = U64;
        let std_string_string_ts = StdStringString;
        let gen_import_path_value_initialization_ts = |content_ts: &dyn ToTokens| gen_value_initialization_ts(&import_path, &content_ts);
        let gen_ident_ts = |is_nullable_ddf79d44: &IsNullable, pg_json_type_pattern_2c09ee59: &PgJsonTypePattern| {
            let vec_of_ucc = VecOfUcc;
            let array_of_ucc = ArrayOfUcc;
            let is_nullable_rust = is_nullable_ddf79d44.rust();
            let (rust_part, pg_part) = match &pg_json_type_pattern_2c09ee59 {
                PgJsonTypePattern::Standart => (rust_type_name.to_string(), pg_json_type_name.to_string()),
                PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    (format!("{vec_of_ucc}{d1_rust}{rust_type_name}"), format!("{array_of_ucc}{d1}{pg_json_type_name}"))
                }
                PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, dim2_is_nullable } => {
                    let d1 = dim1_is_nullable.not_null_or_nullable_str();
                    let d1_rust = dim1_is_nullable.rust();
                    let d2 = dim2_is_nullable.not_null_or_nullable_str();
                    let d2_rust = dim2_is_nullable.rust();
                    (format!("{vec_of_ucc}{d1_rust}{vec_of_ucc}{d2_rust}{rust_type_name}"), format!("{array_of_ucc}{d1}{array_of_ucc}{d2}{pg_json_type_name}"))
                }
                PgJsonTypePattern::ArrayDim3 {
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
                        format!("{vec_of_ucc}{d1_rust}{vec_of_ucc}{d2_rust}{vec_of_ucc}{d3_rust}{rust_type_name}"),
                        format!("{array_of_ucc}{d1}{array_of_ucc}{d2}{array_of_ucc}{d3}{pg_json_type_name}"),
                    )
                }
                PgJsonTypePattern::ArrayDim4 {
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
                        format!("{vec_of_ucc}{d1_rust}{vec_of_ucc}{d2_rust}{vec_of_ucc}{d3_rust}{vec_of_ucc}{d4_rust}{rust_type_name}"),
                        format!("{array_of_ucc}{d1}{array_of_ucc}{d2}{array_of_ucc}{d3}{array_of_ucc}{d4}{pg_json_type_name}"),
                    )
                }
            };
            let not_null_or_nullable_str = is_nullable_ddf79d44.not_null_or_nullable_str();
            format!("{is_nullable_rust}{rust_part}{AsUcc}{not_null_or_nullable_str}{pg_part}").parse::<Ts2>().expect("998d1471")
        };
        let ident = &gen_ident_ts(is_nullable, pg_json_type_pattern);
        let ident_standart_not_null_ucc = &gen_ident_ts(&IsNullable::False, &PgJsonTypePattern::Standart);
        let ident_standart_not_null_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident);
        let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
        let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
        let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
        let ident_not_null_ts = gen_ident_ts(&IsNullable::False, pg_json_type_pattern);
        let ident_ts = {
            let ident_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
        let ident_read_inner_standart_not_null_alias_ts = {
            let content_ts: &dyn ToTokens = match &pg_json_type {
                PgJsonType::I8AsJsonbNumber => &I8,
                PgJsonType::I16AsJsonbNumber => &I16,
                PgJsonType::I32AsJsonbNumber => &I32,
                PgJsonType::I64AsJsonbNumber => &I64,
                PgJsonType::U8AsJsonbNumber => &U8,
                PgJsonType::U16AsJsonbNumber => &U16,
                PgJsonType::U32AsJsonbNumber => &U32,
                PgJsonType::U64AsJsonbNumber => &u64_ts,
                PgJsonType::F32AsJsonbNumber => &F32,
                PgJsonType::F64AsJsonbNumber => &F64,
                PgJsonType::BoolAsJsonbBoolean => &Bool,
                PgJsonType::StdStringStringAsJsonbString => &std_string_string_ts,
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
        let maybe_const_fn = match &pg_json_type_pattern {
            PgJsonTypePattern::Standart => match &is_nullable {
                IsNullable::False => ConstFn::True,
                IsNullable::True => ConstFn::False,
            },
            PgJsonTypePattern::ArrayDim1 { .. } |
            PgJsonTypePattern::ArrayDim2 { .. } |
            PgJsonTypePattern::ArrayDim3 { .. } |
            PgJsonTypePattern::ArrayDim4 { .. } => ConstFn::False,
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
        let maybe_derive_copy = match &pg_json_type_pattern {
            PgJsonTypePattern::Standart => match &pg_json_type {
                PgJsonType::I8AsJsonbNumber |
                PgJsonType::I16AsJsonbNumber |
                PgJsonType::I32AsJsonbNumber |
                PgJsonType::I64AsJsonbNumber |
                PgJsonType::U8AsJsonbNumber |
                PgJsonType::U16AsJsonbNumber |
                PgJsonType::U32AsJsonbNumber |
                PgJsonType::U64AsJsonbNumber |
                PgJsonType::F32AsJsonbNumber |
                PgJsonType::F64AsJsonbNumber |
                PgJsonType::BoolAsJsonbBoolean |
                PgJsonType::UuidUuidAsJsonbString => DeriveCopy::True,
                PgJsonType::StdStringStringAsJsonbString => DeriveCopy::False,
            },
            PgJsonTypePattern::ArrayDim1 {..} |
            PgJsonTypePattern::ArrayDim2 {..} |
            PgJsonTypePattern::ArrayDim3 {..} |
            PgJsonTypePattern::ArrayDim4 {..} => DeriveCopy::False,
        };
        let ident_origin_ts = {
            let gen_ident_origin_non_wrapping_6c0934a6 = |
                is_nullable_e7d1d83c: &IsNullable,
                pg_json_type_pattern_1ca83c6c: &PgJsonTypePattern
            | SelfOriginUcc::from_tokens(&gen_ident_ts(is_nullable_e7d1d83c, pg_json_type_pattern_1ca83c6c));
            // let schema_name_format_handle_ts = double_quotes_ts(&ident_origin_ucc);
            //todo json schema logic
            // let metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_ts = quote! {
            //     Some(Box::new(schemars::schema::Metadata {
            //         id: None,
            //         title: Some(#schema_name_format_handle_ts.to_owned()),
            //         description: None,
            //         default: None,
            //         deprecated: false,
            //         read_only: false,
            //         write_only: false,
            //         examples: Vec::default(),
            //     }))
            // };
            // let extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_ts = quote! {schemars::Map::default()};
            // let (instance_type_number_ts, instance_type_string_ts) = {
            //     let gen_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type = |instance_type: &schemars::schema::InstanceType| {
            //         let instance_type_ts: &dyn ToTokens = match &instance_type {
            //             schemars::schema::InstanceType::Null => &NullUcc,
            //             schemars::schema::InstanceType::Boolean => &BooleanUcc,
            //             schemars::schema::InstanceType::Object => &ObjectUcc,
            //             schemars::schema::InstanceType::Array => &ArrayUcc,
            //             schemars::schema::InstanceType::Number => &NumberUcc,
            //             schemars::schema::InstanceType::String => &StringUcc,
            //             schemars::schema::InstanceType::Integer => &IntegerUcc,
            //         };
            //         quote! {Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::#instance_type_ts)))}
            //     };
            //     (
            //         gen_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::Number),
            //         gen_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::String),
            //     )
            // };
            // let schemars_json_schema = if let IsStandartNotNull::True = &is_standart_not_null {
            //     match &pg_json_type {
            //         PgJsonType::I8AsJsonbNumber
            //         | PgJsonType::I16AsJsonbNumber
            //         | PgJsonType::I32AsJsonbNumber
            //         | PgJsonType::I64AsJsonbNumber
            //         | PgJsonType::U8AsJsonbNumber
            //         | PgJsonType::U16AsJsonbNumber
            //         | PgJsonType::U32AsJsonbNumber
            //         | PgJsonType::U64AsJsonbNumber => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
            //             metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_ts,
            //             instance_type: &instance_type_number_ts,
            //             format: &none_ucc,
            //             enum_values: &none_ucc,
            //             const_value: &none_ucc,
            //             subschemas: &none_ucc,
            //             number: &quote! {Some(Box::new(schemars::schema::NumberValidation {
            //                 multiple_of: None,
            //                 maximum: Some(#ident_read_inner_standart_not_null_alias_ts ::MAX as #f64_ts),
            //                 exclusive_maximum: None,
            //                 minimum: Some(#ident_read_inner_standart_not_null_alias_ts ::MIN as #f64_ts),
            //                 exclusive_minimum: None,
            //             }))},
            //             string: &none_ucc,
            //             array: &none_ucc,
            //             object: &none_ucc,
            //             reference: &none_ucc,
            //             extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_ts,
            //         }),
            //         PgJsonType::F32AsJsonbNumber | PgJsonType::F64AsJsonbNumber | PgJsonType::BoolAsJsonbBoolean | PgJsonType::StdStringStringAsJsonbString => SchemarsJsonSchema::Derive,
            //         PgJsonType::UuidUuidAsJsonbString => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
            //             metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_ts,
            //             instance_type: &instance_type_string_ts,
            //             format: &none_ucc,
            //             enum_values: &none_ucc,
            //             const_value: &none_ucc,
            //             subschemas: &none_ucc,
            //             number: &none_ucc,
            //             string: &quote! {Some(Box::new(schemars::schema::StringValidation {
            //                 max_length: Some(36),
            //                 min_length: Some(36),
            //                 pattern: None,
            //             }))},
            //             array: &none_ucc,
            //             object: &none_ucc,
            //             reference: &none_ucc,
            //             extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_ts,
            //         }),
            //     }
            // } else {
            //     SchemarsJsonSchema::Derive
            // };
            let ident_origin_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
                    //todo
                    // match &schemars_json_schema {
                    //     SchemarsJsonSchema::Derive => DeriveSchemarsJsonSchema::True,
                    //     SchemarsJsonSchema::Impl(_) => DeriveSchemarsJsonSchema::False,
                    // }
                    if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                        match &pg_json_type {
                            PgJsonType::UuidUuidAsJsonbString => DeriveSchemarsJsonSchema::False,
                            PgJsonType::I8AsJsonbNumber
                            | PgJsonType::I16AsJsonbNumber
                            | PgJsonType::I32AsJsonbNumber
                            | PgJsonType::I64AsJsonbNumber
                            | PgJsonType::U8AsJsonbNumber
                            | PgJsonType::U16AsJsonbNumber
                            | PgJsonType::U32AsJsonbNumber
                            | PgJsonType::U64AsJsonbNumber
                            | PgJsonType::F32AsJsonbNumber
                            | PgJsonType::F64AsJsonbNumber
                            | PgJsonType::BoolAsJsonbBoolean
                            | PgJsonType::StdStringStringAsJsonbString => DeriveSchemarsJsonSchema::True,
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
                            let gen_ident_origin_6f054930 = |is_nullable_70fb22e6: &IsNullable, pg_json_type_pattern_042c1c1d: &PgJsonTypePattern| {
                                let value = gen_ident_origin_non_wrapping_6c0934a6(is_nullable_70fb22e6, pg_json_type_pattern_042c1c1d);
                                match &is_nullable {
                                    IsNullable::False => gen_std_vec_vec_tokens_declaration_ts(&value),
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&value),
                                }
                            };
                            match &pg_json_type_pattern {
                                PgJsonTypePattern::Standart => match &is_nullable {
                                    IsNullable::False => &ident_read_inner_standart_not_null_alias_ts,
                                    IsNullable::True => &gen_std_option_option_tokens_declaration_ts(&ident_standart_not_null_origin_ucc),
                                },
                                PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => &{
                                    let (is_nullable_572191e3, pg_json_type_pattern_0d46e7d9): (&IsNullable, &PgJsonTypePattern) = match &is_nullable {
                                        IsNullable::False => (dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("e994797d")),
                                        IsNullable::True => (&IsNullable::False, pg_json_type_pattern),
                                    };
                                    gen_ident_origin_6f054930(is_nullable_572191e3, pg_json_type_pattern_0d46e7d9)
                                },
                                PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, .. } => &{
                                    let (is_nullable_800854cc, pg_json_type_pattern_9f256bad): (&IsNullable, &PgJsonTypePattern) = match &is_nullable {
                                        IsNullable::False => (dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("76eb44e3")),
                                        IsNullable::True => (&IsNullable::False, pg_json_type_pattern),
                                    };
                                    gen_ident_origin_6f054930(is_nullable_800854cc, pg_json_type_pattern_9f256bad)
                                },
                                PgJsonTypePattern::ArrayDim3 { dim1_is_nullable, .. } => &{
                                    let (is_nullable_1941ef3f, pg_json_type_pattern_afc056a8): (&IsNullable, &PgJsonTypePattern) = match &is_nullable {
                                        IsNullable::False => (dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("1b996c86")),
                                        IsNullable::True => (&IsNullable::False, pg_json_type_pattern),
                                    };
                                    gen_ident_origin_6f054930(is_nullable_1941ef3f, pg_json_type_pattern_afc056a8)
                                },
                                PgJsonTypePattern::ArrayDim4 { dim1_is_nullable, .. } => &{
                                    let (is_nullable_90884b9d, pg_json_type_pattern_ae87dbd1): (&IsNullable, &PgJsonTypePattern) = match &is_nullable {
                                        IsNullable::False => (dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("d24b7481")),
                                        IsNullable::True => (&IsNullable::False, pg_json_type_pattern),
                                    };
                                    gen_ident_origin_6f054930(is_nullable_90884b9d, pg_json_type_pattern_ae87dbd1)
                                },
                            }
                        };
                        quote!{(#content_ts);}
                    }
                );
            let ident_origin_impl_new_self_content_ts = {
                let gen_value_map_type_new_ts = |type_ts: &dyn ToTokens| quote! {#ValueSc.map(#type_ts::#NewSc)};
                let gen_array_dims_initialization_ts = |type_ts: &dyn ToTokens| match &is_nullable {
                    IsNullable::False => quote! {#ValueSc.into_iter().map(#type_ts::#NewSc).collect()},
                    IsNullable::True => gen_value_map_type_new_ts(&type_ts),
                };
                match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => match &is_nullable {
                        IsNullable::False => quote! {value},
                        IsNullable::True => gen_value_map_type_new_ts(&ident_standart_not_null_origin_ucc),
                    },
                    PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => gen_array_dims_initialization_ts(&{
                        let (pg_json_type_pattern_38178717, is_nullable_b0d116f8): (&PgJsonTypePattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pg_json_type_pattern.down_by_1().expect("1160d3df"), dim1_is_nullable),
                            IsNullable::True => (pg_json_type_pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_b0d116f8, pg_json_type_pattern_38178717)
                    }),
                    PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, .. } => gen_array_dims_initialization_ts(&{
                        let (pg_json_type_pattern_8e2a682a, is_nullable_c378003c): (&PgJsonTypePattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pg_json_type_pattern.down_by_1().expect("8ab62f4e"), dim1_is_nullable),
                            IsNullable::True => (pg_json_type_pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_c378003c, pg_json_type_pattern_8e2a682a)
                    }),
                    PgJsonTypePattern::ArrayDim3 { dim1_is_nullable, .. } => gen_array_dims_initialization_ts(&{
                        let (pg_json_type_pattern_305989a9, is_nullable_4a8825a3): (&PgJsonTypePattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pg_json_type_pattern.down_by_1().expect("ed64919d"), dim1_is_nullable),
                            IsNullable::True => (pg_json_type_pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_4a8825a3, pg_json_type_pattern_305989a9)
                    }),
                    PgJsonTypePattern::ArrayDim4 { dim1_is_nullable, .. } => gen_array_dims_initialization_ts(&{
                        let (pg_json_type_pattern_ea606504, is_nullable_63d0fe05): (&PgJsonTypePattern, &IsNullable) = match &is_nullable {
                            IsNullable::False => (&pg_json_type_pattern.down_by_1().expect("25646d29"), dim1_is_nullable),
                            IsNullable::True => (pg_json_type_pattern, &IsNullable::False),
                        };
                        gen_ident_origin_non_wrapping_6c0934a6(is_nullable_63d0fe05, pg_json_type_pattern_ea606504)
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
                        let ident_standart_not_null_origin_double_quotes_ts = double_quotes_ts(
                            &ident_standart_not_null_origin_ucc
                        );
                        let text_ident_standart_not_null_origin_double_quotes_ts = double_quotes_ts(
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
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#ident_standart_not_null_origin_double_quotes_ts)
                                    }
                                    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#text_ident_standart_not_null_origin_double_quotes_ts)
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
                    PgJsonType::I8AsJsonbNumber
                    | PgJsonType::I16AsJsonbNumber
                    | PgJsonType::I32AsJsonbNumber
                    | PgJsonType::I64AsJsonbNumber
                    | PgJsonType::U8AsJsonbNumber
                    | PgJsonType::U16AsJsonbNumber
                    | PgJsonType::U32AsJsonbNumber
                    | PgJsonType::U64AsJsonbNumber
                    | PgJsonType::F32AsJsonbNumber
                    | PgJsonType::F64AsJsonbNumber
                    | PgJsonType::BoolAsJsonbBoolean
                    | PgJsonType::StdStringStringAsJsonbString => Ts2::new(),
                }
            } else {
                Ts2::new()
            };
            // match &schemars_json_schema {
            //     SchemarsJsonSchema::Derive => &Ts2::new(),
            //     SchemarsJsonSchema::Impl(schema_object_ts) => &{
            //         let schema_id_format_handle_ts = double_quotes_ts(&format!("pg_crud::postgersql_json_type::{ident_origin_ucc}"));
            //         let metadata_ts = &schema_object_ts.metadata;
            //         let instance_type_ts = &schema_object_ts.instance_type;
            //         let format_ts = &schema_object_ts.format;
            //         let enum_values_ts = &schema_object_ts.enum_values;
            //         let const_value_ts = &schema_object_ts.const_value;
            //         let subschemas_ts = &schema_object_ts.subschemas;
            //         let number_ts = &schema_object_ts.number;
            //         let string_ts = &schema_object_ts.string;
            //         let array_ts = &schema_object_ts.array;
            //         let object_ts = &schema_object_ts.object;
            //         let reference_ts = &schema_object_ts.reference;
            //         let extensions_ts = &schema_object_ts.extensions;
            //         //todo maybe reuse
            //         quote! {
            //             impl #schemars_json_schema_ts for #ident_origin_ucc {
            //                 fn schema_name() -> String {
            //                     #schema_name_format_handle_ts.to_owned()
            //                 }
            //                 fn schema_id() -> std::borrow::Cow<'static, str> {
            //                     std::borrow::Cow::Borrowed(#schema_id_format_handle_ts)
            //                 }
            //                 fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
            //                     schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            //                         metadata: #metadata_ts,
            //                         instance_type: #instance_type_ts,
            //                         format: #format_ts,
            //                         enum_values: #enum_values_ts,
            //                         const_value: #const_value_ts,
            //                         subschemas: #subschemas_ts,
            //                         number: #number_ts,
            //                         string: #string_ts,
            //                         array: #array_ts,
            //                         object: #object_ts,
            //                         reference: #reference_ts,
            //                         extensions: #extensions_ts,
            //                     })
            //                 }
            //             }
            //         }
            //     },
            // };
            let maybe_impl_is_string_empty_for_ident_origin_ts = if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNumber
                    | PgJsonType::I16AsJsonbNumber
                    | PgJsonType::I32AsJsonbNumber
                    | PgJsonType::I64AsJsonbNumber
                    | PgJsonType::U8AsJsonbNumber
                    | PgJsonType::U16AsJsonbNumber
                    | PgJsonType::U32AsJsonbNumber
                    | PgJsonType::U64AsJsonbNumber
                    | PgJsonType::F32AsJsonbNumber
                    | PgJsonType::F64AsJsonbNumber
                    | PgJsonType::BoolAsJsonbBoolean => Ts2::new(),
                    PgJsonType::StdStringStringAsJsonbString => gen_impl_crate_is_string_empty_for_ident_content_ts(
                        &ident_origin_ucc,
                        &quote!{self.0.clone().is_empty()}
                    ),
                    PgJsonType::UuidUuidAsJsonbString => gen_impl_crate_is_string_empty_for_ident_content_ts(
                        &ident_origin_ucc,
                        &quote!{self.0.to_string().is_empty()}
                    ),
                }
            } else {
                Ts2::new()
            };
            let impl_display_for_ident_origin_ts = gen_impl_display_ts(&Ts2::new(), &ident_origin_ucc, &Ts2::new(), &quote! {write!(f, "{self:?}")});
            let impl_error_occurence_lib_to_err_string_for_ident_origin_ts = gen_impl_to_err_string_ts(&Ts2::new(), &ident_origin_ucc, &Ts2::new(), &quote! {format!("{self:#?}")});
            let impl_default_option_some_vec_one_el_for_ident_origin_ts = gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(&ident_origin_ucc, &{
                let content_ts = match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => match &is_nullable {
                        IsNullable::False => match &pg_json_type {
                            PgJsonType::I8AsJsonbNumber
                            | PgJsonType::I16AsJsonbNumber
                            | PgJsonType::I32AsJsonbNumber
                            | PgJsonType::I64AsJsonbNumber
                            | PgJsonType::U8AsJsonbNumber
                            | PgJsonType::U16AsJsonbNumber
                            | PgJsonType::U32AsJsonbNumber
                            | PgJsonType::U64AsJsonbNumber
                            | PgJsonType::F32AsJsonbNumber
                            | PgJsonType::F64AsJsonbNumber
                            | PgJsonType::BoolAsJsonbBoolean => quote! {Default::default()},
                            PgJsonType::StdStringStringAsJsonbString => quote! {String::default()},
                            PgJsonType::UuidUuidAsJsonbString => quote! {uuid::Uuid::new_v4()},
                        },
                        IsNullable::True => quote! {Some(#PgCrudCommonDefaultOptionSomeVecOneElCall)},
                    },
                    PgJsonTypePattern::ArrayDim1 { .. } | PgJsonTypePattern::ArrayDim2 { .. } | PgJsonTypePattern::ArrayDim3 { .. } | PgJsonTypePattern::ArrayDim4 { .. } => match &is_nullable {
                        IsNullable::False => quote! {vec![#PgCrudCommonDefaultOptionSomeVecOneElCall]},
                        IsNullable::True => quote! {Some(#PgCrudCommonDefaultOptionSomeVecOneElCall)},
                    },
                };
                quote! {Self(#content_ts)}
            });
            let impl_sqlx_type_sqlx_pg_for_ident_origin_ts = gen_impl_sqlx_type_sqlx_pg_for_ident_ts(&ident_origin_ucc, &gen_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            let impl_sqlx_encode_sqlx_pg_for_ident_origin_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_origin_ucc, &quote! {sqlx::types::Json(&#SelfSc.0)});
            quote! {
                #ident_origin_ts
                #impl_ident_origin_ts
                #impl_from_ident_create_for_ident_origin_ts
                #impl_from_ident_update_for_ident_origin_ts
                #maybe_impl_schemars_json_schema_for_ident_origin_ts
                #maybe_impl_is_string_empty_for_ident_origin_ts
                #impl_display_for_ident_origin_ts
                #impl_error_occurence_lib_to_err_string_for_ident_origin_ts
                #impl_default_option_some_vec_one_el_for_ident_origin_ts
                #impl_sqlx_type_sqlx_pg_for_ident_origin_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_origin_ts
            }
        };
        let ident_origin_struct_content_ts = quote!{(#ident_origin_ucc);};
        let ident_table_type_declaration_ts = {
            let ident_table_type_declaration_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
                    &ident_table_type_declaration_ucc,
                    &Ts2::new(),
                    &ident_origin_struct_content_ts
                );
            let impl_ident_table_type_declaration_ts = {
                quote!{
                    impl #ident_table_type_declaration_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_default_option_some_vec_one_el_for_ident_table_type_declaration_ts =
                gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(&ident_table_type_declaration_ucc, &quote! {Self(#PgCrudCommonDefaultOptionSomeVecOneElCall)});
            //todo maybe add to trait?
            let impl_sqlx_encode_sqlx_pg_for_ident_table_type_declaration_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_table_type_declaration_ucc, &quote! {&#SelfSc.0});
            //todo maybe add to trait?
            let impl_sqlx_type_sqlx_pg_for_ident_table_type_declaration_ts = gen_impl_sqlx_type_sqlx_pg_for_ident_ts(&ident_table_type_declaration_ucc, &gen_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            quote! {
                #ident_table_type_declaration_ts
                #impl_ident_table_type_declaration_ts
                #impl_default_option_some_vec_one_el_for_ident_table_type_declaration_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_table_type_declaration_ts
                #impl_sqlx_type_sqlx_pg_for_ident_table_type_declaration_ts
            }
        };
        let ident_create_ts = {
            let ident_create_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
            let impl_default_option_some_vec_one_el_for_ident_create_ts =
                gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(&ident_create_ucc, &quote! {Self(#PgCrudCommonDefaultOptionSomeVecOneElCall)});
            quote! {
                #ident_create_ts
                #impl_ident_create_ts
                #impl_default_option_some_vec_one_el_for_ident_create_ts
            }
        };
        let ident_create_for_query_ts = {
            let ident_create_for_query_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
            let impl_sqlx_type_sqlx_pg_for_ident_create_for_query_ts = gen_impl_sqlx_type_sqlx_pg_for_ident_ts(&ident_create_for_query_ucc, &ident_origin_ucc);
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
                #impl_sqlx_type_sqlx_pg_for_ident_create_for_query_ts
                #impl_from_ident_create_for_ident_create_for_query_ts
                #maybe_impl_from_ident_update_for_ident_create_for_query_ts
            }
        };
        let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
        let ident_select_ts = {
            let ident_select_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
                    &ArrayDim::try_from(pg_json_type_pattern).map_or_else(
                        |()| quote! {;},
                        |array_dim| {
                            let mut arguments_ts = Vec::new();
                            for el_419a74e5 in 1..=array_dim.to_usize() {
                                let dim_number_pagination_ts = format!("dim{el_419a74e5}_pagination").parse::<Ts2>()
                                .expect("2ad1faf7");
                                arguments_ts.push(quote! {
                                    #dim_number_pagination_ts: #import_path::PaginationStartsWithZero
                                });
                            }
                            quote! {{#(#arguments_ts),*}}
                        }
                    )
                );
            let gen_default_some_one_content_ts = |default_some_one_or_default_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| {
                let content_ts = ArrayDim::try_from(pg_json_type_pattern).map_or_else(
                    |()| Ts2::new(),
                    |array_dim| {
                        let content_ts: &dyn ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudCommonDefaultOptionSomeVecOneElCall,
                            DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall,
                        };
                        let mut arguments_ts = Vec::new();
                        for el_d56aec99 in 1..=array_dim.to_usize() {
                            let dim_number_pagination_ts = format!("dim{el_d56aec99}_pagination").parse::<Ts2>().expect("26ca29fb");
                            arguments_ts.push(quote! {
                                #dim_number_pagination_ts: #content_ts
                            });
                        }
                        quote! {#(#arguments_ts),*}
                    }
                );
                quote! {Self{#content_ts}}
            };
            let impl_default_option_some_vec_one_el_for_pg_json_type_ident_select_ts =
                gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(&ident_select_ucc, &gen_default_some_one_content_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne));
            let impl_default_option_some_vec_one_el_max_page_size_for_pg_json_type_ident_select_ts =
                gen_impl_pg_crud_common_default_option_some_vec_one_el_max_page_size_ts(&ident_select_ucc, &gen_default_some_one_content_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize));
            quote! {
                #ident_select_ts
                #impl_default_option_some_vec_one_el_for_pg_json_type_ident_select_ts
                #impl_default_option_some_vec_one_el_max_page_size_for_pg_json_type_ident_select_ts
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
                        Number,
                        String,
                    }
                    impl From<&PgJsonType> for PgJsonTypeSpecific {
                        fn from(value: &PgJsonType) -> Self {
                            match value {
                                PgJsonType::I8AsJsonbNumber
                                | PgJsonType::I16AsJsonbNumber
                                | PgJsonType::I32AsJsonbNumber
                                | PgJsonType::I64AsJsonbNumber
                                | PgJsonType::U8AsJsonbNumber
                                | PgJsonType::U16AsJsonbNumber
                                | PgJsonType::U32AsJsonbNumber
                                | PgJsonType::U64AsJsonbNumber
                                | PgJsonType::F32AsJsonbNumber
                                | PgJsonType::F64AsJsonbNumber => Self::Number,
                                PgJsonType::BoolAsJsonbBoolean => Self::Bool,
                                PgJsonType::StdStringStringAsJsonbString | PgJsonType::UuidUuidAsJsonbString => Self::String,
                            }
                        }
                    }
                    let pg_json_type_specific = PgJsonTypeSpecific::from(pg_json_type);
                    let common_pg_json_type_filters = vec![PgJsonTypeFilter::Equal { ident: quote! {#ident_table_type_declaration_ucc} }];
                    let ident_table_type_declaration_ucc_ts = quote! {#ident_table_type_declaration_ucc};
                    match &pg_json_type_pattern {
                        PgJsonTypePattern::Standart => {
                            let common_standart_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::In {
                                    ident: ident_table_type_declaration_ucc_ts.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Number => {
                                    let mut vec = common_standart_pg_json_type_filters;
                                    vec.push(PgJsonTypeFilter::GreaterThan {
                                        ident: ident_table_type_declaration_ucc_ts.clone(),
                                    });
                                    vec.push(PgJsonTypeFilter::Between { ident: ident_table_type_declaration_ucc_ts });
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
                        PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => {
                            let array_dim1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("21eaebaf")));
                                quote! {#value}
                            };
                            let common_array_dim1_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneContainsAllElementsOfArray {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneOverlapsWithArray {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dim1_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimOneGreaterThan {
                                        ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimOneBetween {
                                        ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::ContainsElGreaterThan {
                                        ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::AllElementsGreaterThan {
                                        ident: array_dim1_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_array_dim1_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_array_dim1_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimOneRegularExpression);
                                    filters.push(PgJsonTypeFilter::ContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::AllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, dim2_is_nullable } => {
                            let array_dim1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("0c4491c4")));
                                quote! {#value}
                            };
                            let array_dim2_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim2_is_nullable, &pg_json_type_pattern.down_by_2().expect("2d4ee5d4")));
                                quote! {#value}
                            };
                            let common_array_dim2_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoEqual {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::DimTwoLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoContainsAllElementsOfArray {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoOverlapsWithArray {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneAllElementsEqual {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimTwoIn {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dim2_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimTwoGreaterThan {
                                        ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimTwoBetween {
                                        ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimOneContainsElGreaterThan {
                                        ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimOneAllElementsGreaterThan {
                                        ident: array_dim2_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_array_dim2_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_array_dim2_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimTwoRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimOneContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimOneAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PgJsonTypePattern::ArrayDim3 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                        } => {
                            let array_dim1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("3450bef4")));
                                quote! {#value}
                            };
                            let array_dim2_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim2_is_nullable, &pg_json_type_pattern.down_by_2().expect("3c0d10f4")));
                                quote! {#value}
                            };
                            let array_dim3_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim3_is_nullable, &pg_json_type_pattern.down_by_3().expect("9aaf9e82")));
                                quote! {#value}
                            };
                            let common_array_dim3_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoEqual {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeEqual {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::LengthEqual);
                                vec.push(PgJsonTypeFilter::DimOneLengthEqual);
                                vec.push(PgJsonTypeFilter::DimTwoLengthEqual);
                                vec.push(PgJsonTypeFilter::DimThreeLengthEqual);
                                vec.push(PgJsonTypeFilter::LengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimOneLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimTwoLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimThreeLengthGreaterThan);
                                vec.push(PgJsonTypeFilter::DimThreeContainsAllElementsOfArray {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeOverlapsWithArray {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneAllElementsEqual {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoAllElementsEqual {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimTwoIn {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimThreeIn {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dim3_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimThreeGreaterThan {
                                        ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimThreeBetween {
                                        ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimTwoContainsElGreaterThan {
                                        ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimTwoAllElementsGreaterThan {
                                        ident: array_dim3_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_array_dim3_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_array_dim3_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimThreeRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimTwoContainsElRegularExpression);
                                    filters.push(PgJsonTypeFilter::DimTwoAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PgJsonTypePattern::ArrayDim4 {
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable,
                            dim4_is_nullable,
                        } => {
                            let array_dim1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("550d313b")));
                                quote! {#value}
                            };
                            let array_dim2_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim2_is_nullable, &pg_json_type_pattern.down_by_2().expect("7bda1424")));
                                quote! {#value}
                            };
                            let array_dim3_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim3_is_nullable, &pg_json_type_pattern.down_by_3().expect("b43aa5bd")));
                                quote! {#value}
                            };
                            let array_dim4_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(dim4_is_nullable, &pg_json_type_pattern.down_by_4().expect("a246885a")));
                                quote! {#value}
                            };
                            let common_array_dim4_pg_json_type_filters = {
                                let mut vec = common_pg_json_type_filters;
                                vec.push(PgJsonTypeFilter::DimOneEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoEqual {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeEqual {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimFourEqual {
                                    ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
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
                                vec.push(PgJsonTypeFilter::DimFourContainsAllElementsOfArray {
                                    ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimFourOverlapsWithArray {
                                    ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::AllElementsEqual {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneAllElementsEqual {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimTwoAllElementsEqual {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimThreeAllElementsEqual {
                                    ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(PgJsonTypeFilter::DimOneIn {
                                    ident: array_dim1_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimTwoIn {
                                    ident: array_dim2_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimThreeIn {
                                    ident: array_dim3_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(PgJsonTypeFilter::DimFourIn {
                                    ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &pg_json_type_specific {
                                PgJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dim4_pg_json_type_filters;
                                    filters.push(PgJsonTypeFilter::DimFourGreaterThan {
                                        ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimFourBetween {
                                        ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimThreeContainsElGreaterThan {
                                        ident: array_dim4_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(PgJsonTypeFilter::DimThreeAllElementsGreaterThan {
                                        ident: array_dim4_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PgJsonTypeSpecific::Bool => common_array_dim4_pg_json_type_filters,
                                PgJsonTypeSpecific::String => {
                                    let mut filters = common_array_dim4_pg_json_type_filters;
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
            let ident_read_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
            let impl_default_option_some_vec_one_el_for_ident_read_ts =
                gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(&ident_read_ucc, &quote! {Self(#PgCrudCommonDefaultOptionSomeVecOneElCall)});
            let impl_sqlx_encode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(&ident_read_ucc, &quote! {&#SelfSc.0});
            let impl_sqlx_type_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_type_sqlx_pg_for_ident_ts(&ident_read_ucc, &gen_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            quote! {
                #ident_read_ts
                #impl_ident_read_ts
                #impl_default_option_some_vec_one_el_for_ident_read_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_read_ts
                #impl_sqlx_type_sqlx_pg_for_ident_read_ts
            }
        };
        let ident_read_only_ids_standart_not_null_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_read_only_ids_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
                    let std_option_option_unit_ts = gen_std_option_option_tokens_declaration_ts(&quote! {()});
                    let vec_ts = |value: &dyn ToTokens| gen_std_vec_vec_tokens_declaration_ts(&value);
                    let content_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                        match &pg_json_type_pattern {
                            PgJsonTypePattern::Standart => {
                                let ts1 = match &is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                };
                                quote! {#ts1}
                            }
                            PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => {
                                let ts1 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = match &is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts1),
                                };
                                quote! {#ts2}
                            }
                            PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, dim2_is_nullable } => {
                                let ts1 = vec_ts(&match &dim2_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts1),
                                });
                                let ts3 = match &is_nullable {
                                    IsNullable::False => ts2,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts2),
                                };
                                quote! {#ts3}
                            }
                            PgJsonTypePattern::ArrayDim3 {
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                            } => {
                                let ts1 = vec_ts(&match &dim3_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = vec_ts(&match &dim2_is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts1),
                                });
                                let ts3 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => ts2,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts2),
                                });
                                let ts4 = match &is_nullable {
                                    IsNullable::False => ts3,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts3),
                                };
                                quote! {#ts4}
                            }
                            PgJsonTypePattern::ArrayDim4 {
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                                dim4_is_nullable,
                            } => {
                                let ts1 = vec_ts(&match &dim4_is_nullable {
                                    IsNullable::False => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = vec_ts(&match &dim3_is_nullable {
                                    IsNullable::False => ts1,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts1),
                                });
                                let ts3 = vec_ts(&match &dim2_is_nullable {
                                    IsNullable::False => ts2,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts2),
                                });
                                let ts4 = vec_ts(&match &dim1_is_nullable {
                                    IsNullable::False => ts3,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts3),
                                });
                                let ts5 = match &is_nullable {
                                    IsNullable::False => ts4,
                                    IsNullable::True => gen_std_option_option_tokens_declaration_ts(&ts4),
                                };
                                quote! {#ts5}
                            }
                        }
                    } else {
                        std_option_option_unit_ts
                    };
                    quote!{(pub #import_path::Value<#content_ts>);}
                }
            );
        let ident_read_inner_ts = {
            let type_ts = match &pg_json_type_pattern {
                PgJsonTypePattern::Standart => match &is_nullable {
                    IsNullable::False => &ident_read_inner_standart_not_null_alias_ts,
                    IsNullable::True => &gen_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                },
                PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => &{
                    let dim1_type = dim1_is_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim1_type))
                },
                PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, dim2_is_nullable } => &{
                    let dim2_type = dim2_is_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    let dim1_type = dim1_is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim2_type));
                    is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim1_type))
                },
                PgJsonTypePattern::ArrayDim3 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                } => &{
                    let dim3_type = dim3_is_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    let dim2_type = dim2_is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim3_type));
                    let dim1_type = dim1_is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim2_type));
                    is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim1_type))
                },
                PgJsonTypePattern::ArrayDim4 {
                    dim1_is_nullable,
                    dim2_is_nullable,
                    dim3_is_nullable,
                    dim4_is_nullable,
                } => &{
                    let dim4_type = dim4_is_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    let dim3_type = dim3_is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim4_type));
                    let dim2_type = dim2_is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim3_type));
                    let dim1_type = dim1_is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim2_type));
                    is_nullable.maybe_option_wrap(gen_std_vec_vec_tokens_declaration_ts(&dim1_type))
                },
            };
            let impl_from_ident_origin_for_ident_read_inner_ts = {
                let value_dot_zero_ts = quote!{#ValueSc.0};
                let nullable_ts = quote!{#value_dot_zero_ts.map(Into::into)};
                let into_inner_content_ts = match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => match &is_nullable {
                        IsNullable::False => value_dot_zero_ts,
                        IsNullable::True => nullable_ts,
                    },
                    PgJsonTypePattern::ArrayDim1 {..} |
                    PgJsonTypePattern::ArrayDim2 {..} |
                    PgJsonTypePattern::ArrayDim3 {..} |
                    PgJsonTypePattern::ArrayDim4 {..} => match &is_nullable {
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
            let ident_update_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
            let impl_error_occurence_lib_to_err_string_for_ident_update_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                gen_impl_to_err_string_ts(&Ts2::new(), &ident_update_ucc, &Ts2::new(), &quote! {format!("{self:?}")})
            } else {
                Ts2::new()
            };
            let impl_default_option_some_vec_one_el_for_ident_update_ts =
                gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(&ident_update_ucc, &quote! {Self(#PgCrudCommonDefaultOptionSomeVecOneElCall)});
            quote! {
                #ident_update_ts
                #impl_ident_update_ts
                #impl_error_occurence_lib_to_err_string_for_ident_update_ts
                #impl_default_option_some_vec_one_el_for_ident_update_ts
            }
        };
        let ident_update_for_query_ts = {
            let ident_update_for_query_ts = StructOrEnumDeriveTokenStreamBuilder::new()
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
            let impl_sqlx_type_sqlx_pg_for_ident_update_for_query_ts = gen_impl_sqlx_type_sqlx_pg_for_ident_ts(&ident_update_for_query_ucc, &ident_origin_ucc);
            quote! {
                #ident_update_for_query_ts
                #impl_ident_update_for_query_ts
                #impl_from_ident_update_for_ident_update_for_query_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_update_for_query_ts
                #impl_sqlx_type_sqlx_pg_for_ident_update_for_query_ts
            }
        };
        let pg_crud_macros_common_import_path_pg_crud_common = ImportPath::PgCrudCommon;
        let impl_pg_json_type_for_ident_ts = {
            let gen_dim_number_str = |dims_number: usize| format!("dim{dims_number}");
            let gen_dim_number_start_str = |dims_number: usize| format!("{}_start", gen_dim_number_str(dims_number));
            let gen_dim_number_end_str = |dims_number: usize| format!("{}_end", gen_dim_number_str(dims_number));
            let select_only_created_or_updated_ids_query_part_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                quote! {
                    match #import_path::increment_checked_add_one_returning_increment(#IncrementSc) {
                        Ok(value_f06128be) => Ok(format!("'{field_ident}',jsonb_build_object('value',${value_f06128be}),")),
                        Err(#ErrorSc) => Err(#ErrorSc),
                    }
                }
            } else {
                quote! {Ok(gen_pg_json_types_common::field_ident_jsonb_build_object_value(field_ident))}
            };
            let select_only_created_or_updated_ids_query_bind_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                quote! {
                    if let Err(#ErrorSc) = #QuerySc.try_bind(#ValueSc) {
                        return Err(#ErrorSc.to_string());
                    }
                    Ok(#QuerySc)
                }
            } else {
                quote! {Ok(#QuerySc)}
            };
            gen_impl_pg_json_type_ts(
                &pg_crud_macros_common_import_path_pg_crud_common,
                &ident,
                &ident_table_type_declaration_ucc,
                &ident_create_ucc,
                &ident_create_for_query_ucc,
                &ident_select_ucc,
                &match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => IsSelectQueryPartSelfSelectUsed::False,
                    PgJsonTypePattern::ArrayDim1 { .. } | PgJsonTypePattern::ArrayDim2 { .. } | PgJsonTypePattern::ArrayDim3 { .. } | PgJsonTypePattern::ArrayDim4 { .. } => IsSelectQueryPartSelfSelectUsed::True,
                },
                &IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::False,
                &IsSelectQueryPartIsPgTypeUsed::False,
                &{
                    let format_handle = {
                        //last child dim value does not matter - null or type - works both good
                        let column_name_and_maybe_field_getter_field_ident = format!("{{{ColumnNameAndMaybeFieldGetterSc}}}->'{{field_ident}}'");
                        let format_handle = ArrayDim::try_from(pg_json_type_pattern).map_or_else(
                            |()| column_name_and_maybe_field_getter_field_ident.clone(),
                            |array_dim| {
                                enum ArrayDimSelectPattern {
                                    ArrayDim2 {
                                        dim1_is_nullable: IsNullable,
                                    },
                                    ArrayDim3 {
                                        dim1_is_nullable: IsNullable,
                                        dim2_is_nullable: IsNullable,
                                    },
                                    ArrayDim4 {
                                        dim1_is_nullable: IsNullable,
                                        dim2_is_nullable: IsNullable,
                                        dim3_is_nullable: IsNullable,
                                    },
                                }
                                impl TryFrom<&ArrayDim> for ArrayDimSelectPattern {
                                    type Error = ();
                                    fn try_from(value: &ArrayDim) -> Result<Self, Self::Error> {
                                        match &value {
                                            ArrayDim::ArrayDim1 => Err(()),
                                            ArrayDim::ArrayDim2 {
                                                dim1_is_nullable,
                                            } => Ok(Self::ArrayDim2 {
                                                dim1_is_nullable: *dim1_is_nullable,
                                            }),
                                            ArrayDim::ArrayDim3 {
                                                dim1_is_nullable,
                                                dim2_is_nullable,
                                            } => Ok(Self::ArrayDim3 {
                                                dim1_is_nullable: *dim1_is_nullable,
                                                dim2_is_nullable: *dim2_is_nullable,
                                            }),
                                            ArrayDim::ArrayDim4 {
                                                dim1_is_nullable,
                                                dim2_is_nullable,
                                                dim3_is_nullable,
                                            } => Ok(Self::ArrayDim4 {
                                                dim1_is_nullable: *dim1_is_nullable,
                                                dim2_is_nullable: *dim2_is_nullable,
                                                dim3_is_nullable: *dim3_is_nullable,
                                            }),
                                        }
                                    }
                                }
                                let gen_jsonb_agg = |jsonb_agg_content: &str, jsonb_array_elements_content: &str, ordinality_content: &str, dims_number: usize| {
                                    let dim_number_start = gen_dim_number_start_str(dims_number);
                                    let dim_number_end = gen_dim_number_end_str(dims_number);
                                    format!(
                                        "select jsonb_agg(({jsonb_agg_content})) from jsonb_array_elements(({jsonb_array_elements_content})) with ordinality {ordinality_content} between {{{dim_number_start}}} and {{{dim_number_end}}}"
                                    )
                                };
                                ArrayDimSelectPattern::try_from(&array_dim).map_or_else(
                                    |()| gen_jsonb_agg(
                                        "value",
                                        &format!("select {column_name_and_maybe_field_getter_field_ident}"),
                                        "where ordinality",
                                        1,
                                    ),
                                    |array_dim_select_pattern| {
                                        // Dim1 does not fit into pattern. its only for 2+ dims
                                        let gen_d_number_elem = |content: usize| format!("d{content}_elem");
                                        let gen_d_number_ord = |content: usize| format!("d{content}_elem");
                                        let gen_dot_value = |content: &str| format!("{content}.value");
                                        let gen_as_value_where = |
                                            first_content: &str,
                                            second_content: &str
                                        | format!("as {first_content}(value, {second_content}) where {second_content}");
                                        let one = 1;
                                        gen_jsonb_agg(
                                            &{
                                                let mut current_usize_value = match &array_dim_select_pattern {
                                                    ArrayDimSelectPattern::ArrayDim2 { .. } => 2,
                                                    ArrayDimSelectPattern::ArrayDim3 { .. } => 3,
                                                    ArrayDimSelectPattern::ArrayDim4 { .. } => 4,
                                                };
                                                match &array_dim_select_pattern {
                                                    ArrayDimSelectPattern::ArrayDim2 {
                                                        dim1_is_nullable,
                                                    } => vec![dim1_is_nullable],
                                                    ArrayDimSelectPattern::ArrayDim3 {
                                                        dim1_is_nullable,
                                                        dim2_is_nullable,
                                                    } => vec![
                                                        dim2_is_nullable,
                                                        dim1_is_nullable,
                                                    ],
                                                    ArrayDimSelectPattern::ArrayDim4 {
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
                                                .fold(gen_dot_value(&gen_d_number_elem(current_usize_value)), |mut acc_64e08e3a, current_is_nullable| {
                                                    let current_usize_value_minus_one = current_usize_value.checked_sub(one).expect("a35e873e");
                                                    let d_usize_minus_one_elem_value = gen_dot_value(&gen_d_number_elem(current_usize_value_minus_one));
                                                    let value = gen_jsonb_agg(
                                                        &acc_64e08e3a,
                                                        &d_usize_minus_one_elem_value,
                                                        &gen_as_value_where(&gen_d_number_elem(current_usize_value), &gen_d_number_ord(current_usize_value)),
                                                        current_usize_value,
                                                    );
                                                    acc_64e08e3a = match &current_is_nullable {
                                                        IsNullable::False => value,
                                                        IsNullable::True => {
                                                            format!("case when jsonb_typeof({d_usize_minus_one_elem_value})='array' then ({value}) else null end")
                                                        }
                                                    };
                                                    current_usize_value = current_usize_value_minus_one;
                                                    acc_64e08e3a
                                                })
                                            },
                                            &column_name_and_maybe_field_getter_field_ident,
                                            &gen_as_value_where(&gen_d_number_elem(one), &gen_d_number_ord(one)),
                                            one,
                                        )
                                    },
                                )
                            },
                        );
                        match &is_nullable {
                            IsNullable::False => format_handle,
                            IsNullable::True => format!("case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident})='null' then 'null'::jsonb else ({format_handle}) end"),
                        }
                    };
                    let maybe_dims_start_end_initialization = ArrayDim::try_from(pg_json_type_pattern).ok().into_iter().flat_map(|array_dim| {
                        (1..=array_dim.to_usize()).map(|el_8d56d66d| {
                            let dim_number_start_ts =
                                gen_dim_number_start_str(el_8d56d66d)
                                    .parse::<Ts2>()
                                    .expect("77ec13b9");
                            let dim_number_end_ts =
                                gen_dim_number_end_str(el_8d56d66d)
                                    .parse::<Ts2>()
                                    .expect("24acbb5e");
                            let dim_number_pagination_ts =
                                format!(
                                    "{}_pagination",
                                    gen_dim_number_str(el_8d56d66d)
                                )
                                .parse::<Ts2>()
                                .expect("745c99b3");
                            quote! {
                                let #dim_number_start_ts = #ValueSc.#dim_number_pagination_ts.start();
                                let #dim_number_end_ts = #ValueSc.#dim_number_pagination_ts.end();
                            }
                        })
                    });
                    let format_handle_ts = double_quotes_ts(&format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',({format_handle})))"));
                    quote! {
                        #(#maybe_dims_start_end_initialization)*
                        Ok(format!(#format_handle_ts))
                    }
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &ident_read_only_ids_ucc,
                &{
                    let content_ts = if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                        quote! {format!("jsonb_build_object('value',{column_name_and_maybe_field_getter})")}
                    } else {
                        quote! {"jsonb_build_object('value','null'::jsonb)".to_owned()}
                    };
                    quote! {Ok(#content_ts)}
                },
                &ident_read_inner_ucc,
                &{
                    let content_ts = quote! {#ValueSc.0.0};
                    let gen_match_el_zero_ts = |
                        match_ts: &dyn ToTokens,
                        value_ts: &dyn ToTokens,
                        current_content_ts: &dyn ToTokens
                    | {
                        quote! {#match_ts.map(|#value_ts| #value_ts.0 #current_content_ts)}
                    };
                    let gen_into_iter_map_el_collect_ts = |
                        el_ts: &dyn ToTokens,
                        current_content_ts: &dyn ToTokens
                    | {
                        quote! {.into_iter().map(|#el_ts|#current_content_ts).collect()}
                    };
                    let gen_into_iter_map_el_collect_is_nullable_ts = |
                        el_ts: &dyn ToTokens,
                        current_is_nullable: &IsNullable
                    | {
                        gen_into_iter_map_el_collect_ts(
                            &el_ts,
                            &match &current_is_nullable {
                                IsNullable::False => quote! {#el_ts.0},
                                IsNullable::True => gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &quote! {value_f8b0b01d},
                                    &Ts2::new()
                                )
                            }
                        )
                    };
                    let gen_into_iter_map_el_collect_is_nullable_content_ts = |
                        el_ts: &dyn ToTokens,
                        value_ts: &dyn ToTokens,
                        current_is_nullable: &IsNullable,
                        current_content_ts: &dyn ToTokens
                    | {
                        match &current_is_nullable {
                            IsNullable::False => gen_into_iter_map_el_collect_ts(
                                &el_ts,
                                &quote! {#el_ts.0 #current_content_ts}
                            ),
                            IsNullable::True => {
                                let match_el_zero_ts = gen_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &value_ts,
                                    &current_content_ts
                                );
                                quote! {.into_iter().map(|#el_ts|#match_el_zero_ts).collect()}
                            }
                        }
                    };
                    let into_inner_content_ts = match &pg_json_type_pattern {
                        PgJsonTypePattern::Standart => Ts2::new(),
                        PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => gen_into_iter_map_el_collect_is_nullable_ts(
                            &quote!{el_0fdb74a5},
                            dim1_is_nullable,
                        ),
                        PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, dim2_is_nullable } => {
                            let dim2_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_ts(
                                &quote!{el_dac5ba56},
                                dim2_is_nullable
                            );
                            gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_cf5646e9},
                                &quote!{value_1c90c80c},
                                dim1_is_nullable,
                                &dim2_is_nullable_content_ts
                            )
                        }
                        PgJsonTypePattern::ArrayDim3 {
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
                                &quote!{value_3d1307e8},
                                dim2_is_nullable,
                                &dim3_is_nullable_content_ts
                            );
                            gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_bf67606b},
                                &quote!{value_721e4164},
                                dim1_is_nullable,
                                &dim2_is_nullable_content_ts
                            )
                        }
                        PgJsonTypePattern::ArrayDim4 {
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
                                &quote!{value_995a5fbe},
                                dim3_is_nullable,
                                &dim4_is_nullable_content_ts
                            );
                            let dim2_is_nullable_content_ts = gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_34e95172},
                                &quote!{value_75b18ade},
                                dim2_is_nullable,
                                &dim3_is_nullable_content_ts
                            );
                            gen_into_iter_map_el_collect_is_nullable_content_ts(
                                &quote!{el_f64124e2},
                                &quote!{value_7fc1b146},
                                dim1_is_nullable,
                                &dim2_is_nullable_content_ts
                            )
                        }
                    };
                    match &is_nullable {
                        IsNullable::False => quote! {#content_ts #into_inner_content_ts},
                        IsNullable::True => gen_match_el_zero_ts(
                            &content_ts,
                            &quote!{value_3432e728},
                            &into_inner_content_ts
                        ),
                    }
                },
                &ident_update_ucc,
                &ident_update_for_query_ucc,
                &{
                    let jsonb_set_accumulator_sc = JsonbSetAccumulatorSc;
                    let format_handle_ts = double_quotes_ts(&format!("jsonb_set({{{jsonb_set_accumulator_sc}}},'{{{{{{jsonb_set_path}}}}}}',${{value_26526e0f}})"));
                    quote! {
                        match #import_path::increment_checked_add_one_returning_increment(#IncrementSc) {
                            Ok(value_26526e0f) => Ok(format!(#format_handle_ts)),
                            Err(#ErrorSc) => Err(#ErrorSc),
                        }
                    }
                },
                &IsUpdateQueryPartSelfUpdateUsed::False,
                &IsUpdateQueryPartJsonbSetTargetUsed::False,
                &IsUpdateQueryBindMutable::True,
                &quote! {
                    if let Err(error) = query.try_bind(#ValueSc) {
                        return Err(error.to_string());
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
                            #std_string_string_ts
                        > {
                            if let Err(#ErrorSc) = #QuerySc.try_bind(#ValueSc.0.0.to_string()) {
                                return Err(#ErrorSc.to_string())
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
                    fn increment_checked_add_one(#IncrementSc: &mut #u64_ts) -> Result<#u64_ts, #import_path::QueryPartError> {
                        #import_path::increment_checked_add_one_returning_increment(#IncrementSc)
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
                PgJsonType::I8AsJsonbNumber => quote! {i8_test_cases_vec},
                PgJsonType::I16AsJsonbNumber => quote! {i16_test_cases_vec},
                PgJsonType::I32AsJsonbNumber => quote! {i32_test_cases_vec},
                PgJsonType::I64AsJsonbNumber => quote! {i64_test_cases_vec},
                PgJsonType::U8AsJsonbNumber => quote! {u8_test_cases_vec},
                PgJsonType::U16AsJsonbNumber => quote! {u16_test_cases_vec},
                PgJsonType::U32AsJsonbNumber => quote! {u32_test_cases_vec},
                PgJsonType::U64AsJsonbNumber => quote! {u64_test_cases_vec},
                PgJsonType::F32AsJsonbNumber => quote! {f32_test_cases_vec},
                PgJsonType::F64AsJsonbNumber => quote! {f64_test_cases_vec},
                PgJsonType::BoolAsJsonbBoolean => quote! {bool_test_cases_vec},
                PgJsonType::StdStringStringAsJsonbString => quote! {std_string_string_test_cases_vec},
                PgJsonType::UuidUuidAsJsonbString => quote! {uuid_uuid_test_cases_vec},
            };
            let gen_array_dim_equal_ts = |dim: &Dim| {
                let dim_index_number_max = DimIndexNumber::from(dim);
                let gen_dim_index_number_ts = |is_nullable_vec: &[&IsNullable]|{
                    assert!(!is_nullable_vec.is_empty(), "c1a5939d");
                    let content_ts_c85923bd = {
                        let gen_index_number_ts = |index_c1128a3e: usize|format!("index_{index_c1128a3e}").parse::<Ts2>().expect("afbe7252");
                        let gen_value_number_ts = |index_0abe6039: usize|format!("value{index_0abe6039}").parse::<Ts2>().expect("568d8eb6");
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
                            &gen_index_number_ts(index_0082bcdf),
                            &gen_value_number_ts(index_e81c6d28),
                            &gen_value_number_ts(index_b7b230b2),
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
                            &gen_value_number_ts(index_c4552aef),
                            &gen_value_number_ts(index_9f1fbc9f),
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
                                let value_index_ts = gen_value_number_ts(
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
                                let to_number_starting_with_one_word_str = |dim_index_number: &DimIndexNumber| match dim_index_number {
                                    DimIndexNumber::Zero => "One",
                                    DimIndexNumber::One => "Two",
                                    DimIndexNumber::Two => "Three",
                                    DimIndexNumber::Three => "Four",
                                };
                                let dim_number_starting_with_one_equal_ts = format!("Dim{}Equal", to_number_starting_with_one_word_str(&dim_index_number_max)).parse::<Ts2>().expect("52fa34ac");
                                let pg_json_type_where_dim_number_starting_with_one_equal_ts = format!("PgJsonTypeWhereDim{}Equal", to_number_starting_with_one_word_str(&dim_index_number_max)).parse::<Ts2>().expect("15d769b0");
                                let current_where_ident_where_ucc = SelfWhereUcc::from_tokens(&gen_ident_ts(&IsNullable::False, pg_json_type_pattern));
                                let current_value_ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&gen_ident_ts(
                                    is_nullable_vec.last().expect("1221f6ec"),
                                    &match dim_index_number_max {
                                        DimIndexNumber::Zero => pg_json_type_pattern.down_by_1().expect("1a47af86"),
                                        DimIndexNumber::One => pg_json_type_pattern.down_by_2().expect("d8260225"),
                                        DimIndexNumber::Two => pg_json_type_pattern.down_by_3().expect("473ac422"),
                                        DimIndexNumber::Three => pg_json_type_pattern.down_by_4().expect("6a143218"),
                                    }
                                ));
                                let vec_content_ts = {
                                    let content_ts_0dc5a500 = (
                                        0i32..=match dim_index_number_max {
                                            DimIndexNumber::Zero => 0i32,
                                            DimIndexNumber::One => 1i32,
                                            DimIndexNumber::Two => 2i32,
                                            DimIndexNumber::Three => 3i32,
                                        }
                                    )
                                    .map(|el_db559599| {
                                        let index_number_ts = format!("index_{el_db559599}")
                                            .parse::<Ts2>()
                                            .expect("f0ce7e73");
                                        quote! {
                                            pg_crud_common::UnsignedPartOfI32::try_from(
                                                i32::try_from(#index_number_ts)
                                                    .expect("5a1818e7")
                                            ).expect("ad1ab73f")
                                        }
                                    }).collect::<Vec<Ts2>>();
                                    quote! {#(#content_ts_0dc5a500),*}
                                };
                                quote! {
                                    #current_where_ident_where_ucc::#dim_number_starting_with_one_equal_ts(
                                        where_filters::#pg_json_type_where_dim_number_starting_with_one_equal_ts {
                                            logical_operator: #import_path::LogicalOperator::And,
                                            dims: where_filters::BoundedStdVecVec::try_from(
                                                vec![#vec_content_ts]
                                            ).expect("82cc0a3c"),
                                            #ValueSc: #current_value_ident_table_type_declaration_ucc::new(#value_index_ts.into()),
                                        }
                                    )
                                }
                            };
                            match is_nullable {
                                IsNullable::False => quote! {acc_049ff0b3.push(#content_ts_f1ffd3b2);},
                                IsNullable::True => quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new(vec![#content_ts_f1ffd3b2]) {
                                        Ok(value_9328b66f) => {
                                            acc_049ff0b3.push(#import_path::NullableJsonObjectPgTypeWhereFilter(Some(value_9328b66f)));
                                        },
                                        Err(error) => match error {
                                            #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => (),
                                            #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("2f5f648a")
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
                                let index_74ae6d77_increment_by_1 = index_74ae6d77.checked_add(1).expect("96e90e72");
                                match &is_nullable_vec_e7e7f6f8.last().expect("88548240") {
                                    IsNullable::False => gen_for_value_index_dot_zero_into_iter_enumerate_ts(
                                        is_nullable_vec_e7e7f6f8_len,
                                        index_74ae6d77_increment_by_1,
                                        index_74ae6d77,
                                        &content_ts_4c106eea,
                                    ),
                                    IsNullable::True => gen_if_let_some_equals_value_index_dot_zero_ts(
                                        index_74ae6d77_increment_by_1,
                                        index_74ae6d77,
                                        &gen_for_value_index_dot_zero_into_iter_enumerate_ts(
                                            is_nullable_vec_e7e7f6f8_len,
                                            index_74ae6d77.checked_add(2).expect("00da046c"),
                                            index_74ae6d77_increment_by_1,
                                            &content_ts_4c106eea,
                                        )
                                    )
                                }
                            };
                        }
                        let create_dot_zero_ts = quote!{create.0};
                        match &is_nullable {
                            IsNullable::False => gen_for_in_ts(
                                &gen_index_number_ts(0),
                                &gen_value_number_ts(0),
                                &create_dot_zero_ts,
                                &content_ts_4c106eea
                            ),
                            IsNullable::True => gen_if_let_some_ts(
                                &gen_value_number_ts(0),
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
                match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => none_ts.clone(),
                    PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => match dim_index_number_max {
                        DimIndexNumber::Zero => gen_dim_index_number_ts(&[
                            dim1_is_nullable,
                        ]),
                        DimIndexNumber::One | DimIndexNumber::Two | DimIndexNumber::Three => none_ts.clone(),
                    },
                    PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, dim2_is_nullable } => match dim_index_number_max {
                        DimIndexNumber::Zero => gen_dim_index_number_ts(&[
                            dim1_is_nullable,
                        ]),
                        DimIndexNumber::One => gen_dim_index_number_ts(&[
                            dim1_is_nullable,
                            dim2_is_nullable
                        ]),
                        DimIndexNumber::Two | DimIndexNumber::Three => none_ts.clone(),
                    },
                    PgJsonTypePattern::ArrayDim3 {
                        dim1_is_nullable,
                        dim2_is_nullable,
                        dim3_is_nullable,
                    } => match dim_index_number_max {
                        DimIndexNumber::Zero => gen_dim_index_number_ts(&[
                            dim1_is_nullable,
                        ]),
                        DimIndexNumber::One => gen_dim_index_number_ts(&[
                            dim1_is_nullable,
                            dim2_is_nullable,
                        ]),
                        DimIndexNumber::Two => gen_dim_index_number_ts(&[
                            dim1_is_nullable,
                            dim2_is_nullable,
                            dim3_is_nullable
                        ]),
                        DimIndexNumber::Three => none_ts.clone(),
                    },
                    PgJsonTypePattern::ArrayDim4 {
                        dim1_is_nullable,
                        dim2_is_nullable,
                        dim3_is_nullable,
                        dim4_is_nullable,
                    } => {
                        match dim_index_number_max {
                            DimIndexNumber::Zero => gen_dim_index_number_ts(&[
                                dim1_is_nullable
                            ]),
                            DimIndexNumber::One => gen_dim_index_number_ts(&[
                                dim1_is_nullable,
                                dim2_is_nullable,
                            ]),
                            DimIndexNumber::Two => gen_dim_index_number_ts(&[
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                            ]),
                            DimIndexNumber::Three => gen_dim_index_number_ts(&[
                                dim1_is_nullable,
                                dim2_is_nullable,
                                dim3_is_nullable,
                                dim4_is_nullable
                            ])
                        }
                    }
                }
            };
            let option_vec_create_ts = {
                let gen_some_acc_content_ts = |current_is_nullable: &IsNullable, current_ident_ts: &dyn ToTokens| {
                    let (new_content_ts, maybe_acc_push_none_ts) = match &current_is_nullable {
                        IsNullable::False => (quote! {vec![el_88131059.0.into()]}, Ts2::new()),
                        IsNullable::True => (quote! {Some(el_88131059.0.into())}, quote! {acc_50e99088.push(<Self as #import_path::PgJsonType>::Create::new(None));}),
                    };
                    //todo check - maybe need to add something here
                    let maybe_acc_push_long_vec_ts = match &is_nullable {
                        IsNullable::False => quote! {
                            let mut acc_27624e5e = Vec::new();
                            for el_0dcb405a in value_8de026a4 {
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
                    // let maybe_dot_clone_ts = match &pg_json_type_pattern {
                    //     PgJsonTypePattern::Standart => match &is_nullable {
                    //         IsNullable::False => match &pg_json_type {
                    //             | PgJsonType::F32AsJsonbNumber
                    //             | PgJsonType::F64AsJsonbNumber => Ts2::new(),
                    //             PgJsonType::I8AsJsonbNumber
                    //             | PgJsonType::I16AsJsonbNumber
                    //             | PgJsonType::I32AsJsonbNumber
                    //             | PgJsonType::I64AsJsonbNumber
                    //             | PgJsonType::U8AsJsonbNumber
                    //             | PgJsonType::U16AsJsonbNumber
                    //             | PgJsonType::U32AsJsonbNumber
                    //             | PgJsonType::U64AsJsonbNumber
                    //             | PgJsonType::BoolAsJsonbBoolean
                    //             | PgJsonType::StdStringStringAsJsonbString
                    //             | PgJsonType::UuidUuidAsJsonbString => quote!{.clone()},
                    //         }
                    //         IsNullable::True => Ts2::new(),
                    //     },
                    //     PgJsonTypePattern::ArrayDim1 { .. } |
                    //     PgJsonTypePattern::ArrayDim2 { .. } |
                    //     PgJsonTypePattern::ArrayDim3 { .. } |
                    //     PgJsonTypePattern::ArrayDim4 { .. } => quote!{.clone()},
                    // };
                    quote! {Some({
                        let mut acc_50e99088 = Vec::new();
                        if let Some(value_8de026a4) = <#current_ident_ts as #import_path::PgJsonTypeTestCases>::#OptionVecCreateSc() {
                            for el_88131059 in value_8de026a4 #maybe_dot_clone_ts {
                                acc_50e99088.push(<Self as #import_path::PgJsonType>::Create::new(#new_content_ts));
                            }
                            #maybe_acc_push_long_vec_ts
                        }
                        #maybe_acc_push_none_ts
                        acc_50e99088
                    })}
                };
                let content_ts = match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => match &is_nullable {
                        IsNullable::False => quote! {
                            Some(
                                #import_path::#standart_not_null_test_cases_vec_name_ts().into_iter().map(<Self as #import_path::PgJsonType>::Create::new).collect()
                            )
                        },
                        IsNullable::True => gen_some_acc_content_ts(is_nullable, &gen_ident_ts(&IsNullable::False, &PgJsonTypePattern::Standart)),
                    },
                    PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("dec468c0")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pg_json_type_pattern),
                        },
                    ),
                    PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, .. } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("4010ebf7")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pg_json_type_pattern),
                        },
                    ),
                    PgJsonTypePattern::ArrayDim3 { dim1_is_nullable, .. } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("acdbb564")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pg_json_type_pattern),
                        },
                    ),
                    PgJsonTypePattern::ArrayDim4 { dim1_is_nullable, .. } => gen_some_acc_content_ts(
                        is_nullable,
                        &match &is_nullable {
                            IsNullable::False => gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("5abf9504")),
                            IsNullable::True => gen_ident_ts(&IsNullable::False, pg_json_type_pattern),
                        },
                    ),
                };
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNumber
                    | PgJsonType::I16AsJsonbNumber
                    | PgJsonType::I32AsJsonbNumber
                    | PgJsonType::I64AsJsonbNumber
                    | PgJsonType::U8AsJsonbNumber
                    | PgJsonType::U16AsJsonbNumber
                    | PgJsonType::U32AsJsonbNumber
                    | PgJsonType::U64AsJsonbNumber
                    | PgJsonType::F32AsJsonbNumber
                    | PgJsonType::F64AsJsonbNumber
                    | PgJsonType::BoolAsJsonbBoolean
                    | PgJsonType::StdStringStringAsJsonbString => content_ts,
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
                let gen_acc_content_handle_ts = |current_ident_ts: &dyn ToTokens, has_len_greater_than_one_content_ts: &dyn ToTokens| {
                    let current_ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&current_ident_ts);
                    let option_additional_content_ts = {
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
                            let option_additional = {
                                let mut option_additional = None;
                                for el_c4f9bf8f in &read_only_ids_to_two_dimal_vec_read_inner {
                                    if option_additional.is_some() {
                                        break;
                                    }
                                    for el_82c7dc0a in el_c4f9bf8f {
                                        if option_additional.is_none() {
                                            option_additional = Some((vec![#first_content_ts], vec![#second_content_ts]));
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                                option_additional
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
                            #current_ident_ts
                            as
                            #import_path::PgJsonTypeTestCases
                        >::#ReadOnlyIdsToTwoDimalVecReadInnerSc(
                            &#current_ident_read_only_ids_ucc(read_only_ids.0.clone())
                        );
                        #option_additional_content_ts
                        #has_len_greater_than_one_content_ts
                        #acc_push_vec_content_ts
                        #maybe_acc_push_vec_none_ts
                        if let Some(value_3de7fba4) = option_additional {
                            if has_len_greater_than_one {
                                acc_0a07db18.push(value_3de7fba4.0);
                            }
                            else {
                                acc_0a07db18.push(value_3de7fba4.1);
                            }
                        }
                        acc_0a07db18
                    }
                };
                let content_ts = match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => match &is_nullable {
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
                    PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => gen_acc_content_handle_ts(
                        &gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("d6f89137")),
                        &match &dim1_is_nullable {
                            IsNullable::False => &has_len_greater_than_one_for_for_ts,
                            IsNullable::True => &has_len_greater_than_one_ts,
                        },
                    ),
                    PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, .. } => gen_acc_content_handle_ts(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("38774398")), &has_len_greater_than_one_ts),
                    PgJsonTypePattern::ArrayDim3 { dim1_is_nullable, .. } => gen_acc_content_handle_ts(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("053f4bab")), &has_len_greater_than_one_ts),
                    PgJsonTypePattern::ArrayDim4 { dim1_is_nullable, .. } => gen_acc_content_handle_ts(&gen_ident_ts(dim1_is_nullable, &pg_json_type_pattern.down_by_1().expect("860f8f15")), &has_len_greater_than_one_ts),
                };
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNumber
                    | PgJsonType::I16AsJsonbNumber
                    | PgJsonType::I32AsJsonbNumber
                    | PgJsonType::I64AsJsonbNumber
                    | PgJsonType::U8AsJsonbNumber
                    | PgJsonType::U16AsJsonbNumber
                    | PgJsonType::U32AsJsonbNumber
                    | PgJsonType::U64AsJsonbNumber
                    | PgJsonType::F32AsJsonbNumber
                    | PgJsonType::F64AsJsonbNumber
                    | PgJsonType::BoolAsJsonbBoolean
                    | PgJsonType::StdStringStringAsJsonbString => content_ts,
                    PgJsonType::UuidUuidAsJsonbString => quote! {Vec::new()},
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_ts = gen_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&ReadOrUpdate::Update);
            let read_only_ids_into_option_value_read_inner_ts = {
                let content_ts = gen_import_path_value_initialization_ts(&if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ValueSc.0.#ValueSc}
                } else {
                    quote! {
                        <Self as #import_path::PgJsonType>::into_inner(
                            <
                                <Self as #import_path::PgJsonType>::Read
                                as
                                #PgCrudCommonDefaultOptionSomeVecOneEl
                            >::default_option_some_vec_one_el()
                        )
                    }
                });
                quote! {Some(#content_ts)}
            };
            let update_to_read_only_ids_ts = {
                let value_initialization_ts = gen_import_path_value_initialization_ts(&if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                    let gen_iter_or_match_ts = |
                        current_is_nullable: &IsNullable,
                        current_ident_ts: &dyn ToTokens,
                        update_current_is_nullable: &IsNullable
                    | {
                        let value_zero_zero_ts = quote! {#ValueSc.0.0};
                        let content_ts = {
                            let current_ident_update_ts = SelfUpdateUcc::from_tokens(&current_ident_ts);
                            let content_ts = {
                                let content_ts = match &update_current_is_nullable {
                                    IsNullable::False => quote! {el_aa999306.clone()},
                                    IsNullable::True => quote! {value_92de91cc.clone()},
                                };
                                quote! {#current_ident_update_ts(#content_ts)}
                            };
                            quote! {
                                <
                                    #current_ident_ts
                                    as
                                    #import_path::PgJsonTypeTestCases
                                >::update_to_read_only_ids(&#content_ts).0.#ValueSc
                            }
                        };
                        match &current_is_nullable {
                            IsNullable::False => quote! {
                                #value_zero_zero_ts.iter().map(|el_aa999306|#content_ts).collect()
                            },
                            IsNullable::True => quote! {
                                #value_zero_zero_ts.as_ref().map(|value_92de91cc| #content_ts)
                            },
                        }
                    };
                    match &pg_json_type_pattern {
                        PgJsonTypePattern::Standart => match &is_nullable {
                            IsNullable::False => quote! {#ValueSc.0.clone().into()},
                            IsNullable::True => gen_iter_or_match_ts(
                                is_nullable,
                                &ident_not_null_ts,
                                is_nullable
                            ),
                        },
                        PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pg_json_type_pattern.down_by_1().expect("e84064c3"),
                                    IsNullable::True => pg_json_type_pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                        PgJsonTypePattern::ArrayDim2 { dim1_is_nullable, .. } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pg_json_type_pattern.down_by_1().expect("226c6318"),
                                    IsNullable::True => pg_json_type_pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                        PgJsonTypePattern::ArrayDim3 { dim1_is_nullable, .. } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pg_json_type_pattern.down_by_1().expect("3ae1e9f8"),
                                    IsNullable::True => pg_json_type_pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                        PgJsonTypePattern::ArrayDim4 { dim1_is_nullable, .. } => gen_iter_or_match_ts(
                            is_nullable,
                            &gen_ident_ts(
                                &match &is_nullable {
                                    IsNullable::False => *dim1_is_nullable,
                                    IsNullable::True => IsNullable::False,
                                },
                                &match &is_nullable {
                                    IsNullable::False => pg_json_type_pattern.down_by_1().expect("44d51dc5"),
                                    IsNullable::True => pg_json_type_pattern.clone(),
                                },
                            ),
                            is_nullable,
                        ),
                    }
                } else {
                    none_ts.clone()
                });
                quote! {#ident_read_only_ids_ucc(#value_initialization_ts)}
            };
            let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = {
                let value_initialization_ts = gen_import_path_value_initialization_ts(&if matches!(&pg_json_type, PgJsonType::UuidUuidAsJsonbString) {
                    quote! {#ident_read_ucc::new(#ValueSc.0.#ValueSc.clone())}
                } else {
                    quote! {#PgCrudCommonDefaultOptionSomeVecOneElCall}
                });
                quote! {Some(#value_initialization_ts)}
            };
            let previous_read_merged_with_option_update_into_read_ts = quote! {
                #OptionUpdateSc.map_or(#ReadSc, |value_f6e37412| #ident_read_ucc(value_f6e37412.into()))
            };
            let read_only_ids_merged_with_create_into_read_ts = {
                let content_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ident_origin_ucc::new(#ReadOnlyIdsSc.0.#ValueSc)}
                } else {
                    quote! {#CreateSc.into()}
                };
                quote! {#ident_read_ucc(#content_ts)}
            };
            let read_only_ids_merged_with_create_into_option_value_read_ts = {
                let value_initialization_ts = gen_import_path_value_initialization_ts(&quote! {
                    <Self as #import_path::PgJsonTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoReadSc(
                        #ReadOnlyIdsSc,
                        #CreateSc
                    )
                });
                quote! {Some(#value_initialization_ts)}
            };
            let read_only_ids_merged_with_create_into_table_type_declaration_ts = {
                let content_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ident_origin_ucc::new(#ReadOnlyIdsSc.0.#ValueSc)}
                } else {
                    quote! {#CreateSc.into()}
                };
                quote! {#ident_table_type_declaration_ucc(#content_ts)}
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
                        let equal_ts = gen_equal_ts(&quote! {#ident_table_type_declaration_ucc::new(#CreateSc.0.into())});
                        quote! {#ident_where_ucc::#EqualUcc(#equal_ts)}
                    }
                    IsNullable::True => {
                        let current_ident_where_ucc = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                        let current_ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_not_null_ts);
                        let equal_ts = gen_equal_ts(&quote! {#current_ident_table_type_declaration_ucc::new(value_18544acf.into())});
                        quote! {
                            #import_path::NullableJsonObjectPgTypeWhereFilter(
                                #CreateSc.0.0.map(|value_18544acf| pg_crud_common::NotEmptyUniqueVec::try_new(
                                    vec![#current_ident_where_ucc::#EqualUcc(#equal_ts)]
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
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts = gen_array_dim_equal_ts(&Dim::One);
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts = gen_array_dim_equal_ts(&Dim::Two);
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts = gen_array_dim_equal_ts(&Dim::Three);
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts = gen_array_dim_equal_ts(&Dim::Four);
            //todo maybe reuse LengthEqual and LengthGreaterThan
            let create_into_pg_json_type_option_vec_where_length_equal_ts = {
                let gen_ts = || {
                    let content_ts = {
                        let create_dot_zero_dot_zero = quote! {#CreateSc.0.0};
                        let content_ts = {
                            let content_ts: &dyn ToTokens = match &is_nullable {
                                IsNullable::False => &create_dot_zero_dot_zero,
                                IsNullable::True => &quote! {value_1bbf74bc.0},
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
                                let current_ident_where_ucc = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                                quote! {
                                    #import_path::NullableJsonObjectPgTypeWhereFilter(
                                        match #create_dot_zero_dot_zero {
                                            Some(value_1bbf74bc) => match #import_path::NotEmptyUniqueVec::try_new(
                                                vec![#current_ident_where_ucc #content_ts]
                                            ) {
                                                Ok(value_d82bbdbe) => Some(value_d82bbdbe),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => {
                                                        return None;
                                                    },
                                                    #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("3d7ce854")
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
                            Ok(value_e196e86d) => Some(value_e196e86d),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("e9f9b021")
                            }
                        }
                    }
                };
                match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => none_ts.clone(),
                    PgJsonTypePattern::ArrayDim1 { .. } | PgJsonTypePattern::ArrayDim2 { .. } | PgJsonTypePattern::ArrayDim3 { .. } | PgJsonTypePattern::ArrayDim4 { .. } => gen_ts(),
                }
            };
            let create_into_pg_json_type_option_vec_where_length_greater_than_ts = {
                let gen_ts = || {
                    let content_ts = {
                        let create_dot_zero_dot_zero = quote! {#CreateSc.0.0};
                        let content_ts = {
                            let content_ts: &dyn ToTokens = match &is_nullable {
                                IsNullable::False => &create_dot_zero_dot_zero,
                                IsNullable::True => &quote! {value_68880991.0},
                            };
                            quote! {
                                ::LengthGreaterThan(
                                    where_filters::PgJsonTypeWhereLengthGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #ValueSc: if let Ok(value_762dae1f) = pg_crud_common::UnsignedPartOfI32::try_from(
                                            if let Ok(value_9dca0200) = i32::try_from(
                                                //todo temp code. make it better checking all cases
                                                match #content_ts.len().checked_sub(1) {
                                                    Some(value_92860143) => value_92860143,
                                                    None => {
                                                        return None;
                                                    }
                                                }
                                            ) {
                                                value_9dca0200
                                            }
                                            else {
                                                return None;
                                            }
                                        ) {
                                            value_762dae1f
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
                                let current_ident_where_ucc = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                                quote! {
                                    #import_path::NullableJsonObjectPgTypeWhereFilter(match #create_dot_zero_dot_zero {
                                        Some(value_68880991) => match #import_path::NotEmptyUniqueVec::try_new(
                                            vec![#current_ident_where_ucc #content_ts]
                                        ) {
                                            Ok(value_cdc120a8) => Some(value_cdc120a8),
                                            Err(error) => match error {
                                                #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => {
                                                    return None;
                                                },
                                                #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("584f801e")
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
                            Ok(value_cee8d0ab) => Some(value_cee8d0ab),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("497359a5")
                            },
                        }
                    }
                };
                match &pg_json_type_pattern {
                    PgJsonTypePattern::Standart => none_ts.clone(),
                    PgJsonTypePattern::ArrayDim1 { .. } | PgJsonTypePattern::ArrayDim2 { .. } | PgJsonTypePattern::ArrayDim3 { .. } | PgJsonTypePattern::ArrayDim4 { .. } => gen_ts(),
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
                    //The correct way to compare floating point numbers is to define an allowed error margin
                    if (#content_ts - value).abs() < #more_ts {
                        None
                    }
                    else {
                        value.is_finite().then_some(value)
                    }
                }}
            };
            //todo additonal logic for Option<value> and element of array? optional element of array?
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts = if matches!(&pg_json_type_pattern, PgJsonTypePattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                let (
                    int_greater_than_one_less_ts,
                    float_32_greater_than_one_less_ts,
                    float_64_greater_than_one_less_ts,
                ) = {
                    let gen_greater_than_one_less_ts = |content_ts: &dyn ToTokens|quote!{
                        let value_7aa498e8 = #content_ts?;
                        match #import_path::NotEmptyUniqueVec::try_new(
                            vec![
                                #import_path::SingleOrMultiple::Single(#ident_where_ucc::GreaterThan(
                                    where_filters::PgJsonTypeWhereGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        value: #ident_table_type_declaration_ucc(
                                            #ident_origin_ucc(value_7aa498e8)
                                        ),
                                    }
                                ))
                            ]
                        ) {
                            Ok(value_6f3e23b5) => Some(value_6f3e23b5),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewError::IsEmpty { .. } => None,
                                #import_path::NotEmptyUniqueVecTryNewError::NotUnique { .. } => panic!("11287f54"),
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
                    PgJsonType::I8AsJsonbNumber |
                    PgJsonType::I16AsJsonbNumber |
                    PgJsonType::I32AsJsonbNumber |
                    PgJsonType::I64AsJsonbNumber |
                    PgJsonType::U8AsJsonbNumber |
                    PgJsonType::U16AsJsonbNumber |
                    PgJsonType::U32AsJsonbNumber |
                    PgJsonType::U64AsJsonbNumber => int_greater_than_one_less_ts,
                    PgJsonType::F32AsJsonbNumber => float_32_greater_than_one_less_ts,
                    PgJsonType::F64AsJsonbNumber => float_64_greater_than_one_less_ts,
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::StdStringStringAsJsonbString |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts = if matches!(&pg_json_type_pattern, PgJsonTypePattern::Standart) &&
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
                                #ident_table_type_declaration_ucc::new(start),
                                #ident_table_type_declaration_ucc::new(end)
                            ) {
                                Ok(value_cdde02cc) => match pg_crud_common::NotEmptyUniqueVec::try_new(vec![
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_ucc::Between(
                                            where_filters::PgJsonTypeWhereBetween {
                                                logical_operator: pg_crud_common::LogicalOperator::Or,
                                                value: value_cdde02cc,
                                            }
                                        )
                                    )
                                ]) {
                                    Ok(value_41af48fb) => Some(value_41af48fb),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("5edabfcc")
                                    }
                                },
                                Err(error) => None
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
                    PgJsonType::I8AsJsonbNumber |
                    PgJsonType::I16AsJsonbNumber |
                    PgJsonType::I32AsJsonbNumber |
                    PgJsonType::I64AsJsonbNumber |
                    PgJsonType::U8AsJsonbNumber |
                    PgJsonType::U16AsJsonbNumber |
                    PgJsonType::U32AsJsonbNumber |
                    PgJsonType::U64AsJsonbNumber => between_one_less_and_one_more_int_ts,
                    PgJsonType::F32AsJsonbNumber |
                    PgJsonType::F64AsJsonbNumber => between_one_less_and_one_more_float_ts,
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::StdStringStringAsJsonbString |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone()
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts = if matches!(&pg_json_type_pattern, PgJsonTypePattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNumber |
                    PgJsonType::I16AsJsonbNumber |
                    PgJsonType::I32AsJsonbNumber |
                    PgJsonType::I64AsJsonbNumber |
                    PgJsonType::U8AsJsonbNumber |
                    PgJsonType::U16AsJsonbNumber |
                    PgJsonType::U32AsJsonbNumber |
                    PgJsonType::U64AsJsonbNumber |
                    PgJsonType::F32AsJsonbNumber |
                    PgJsonType::F64AsJsonbNumber => {
                        //todo additional variants to test
                        quote!{
                            match #import_path::NotEmptyUniqueVec::try_new(vec![
                                #import_path::SingleOrMultiple::Single(
                                    #ident_where_ucc::In(
                                        where_filters::PgJsonTypeWhereIn {
                                            logical_operator: #import_path::LogicalOperator::Or,
                                            value: where_filters::PgJsonTypeNotEmptyUniqueVec::try_new(
                                                vec![#ident_table_type_declaration_ucc::new(#CreateSc.0.0)]
                                            ).expect("2737c0ed")
                                        }
                                    ),
                                )
                            ]) {
                                Ok(value_1c4f89a4) => Some(value_1c4f89a4),
                                Err(error) => match error {
                                    #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                    #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("16ae359d")
                                }
                            }
                        }
                    },
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::StdStringStringAsJsonbString |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone()
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts = if matches!(&pg_json_type_pattern, PgJsonTypePattern::Standart) &&
                matches!(&is_nullable, IsNullable::False)
            {
                match &pg_json_type {
                    PgJsonType::I8AsJsonbNumber |
                    PgJsonType::I16AsJsonbNumber |
                    PgJsonType::I32AsJsonbNumber |
                    PgJsonType::I64AsJsonbNumber |
                    PgJsonType::U8AsJsonbNumber |
                    PgJsonType::U16AsJsonbNumber |
                    PgJsonType::U32AsJsonbNumber |
                    PgJsonType::U64AsJsonbNumber |
                    PgJsonType::F32AsJsonbNumber |
                    PgJsonType::F64AsJsonbNumber |
                    PgJsonType::BoolAsJsonbBoolean |
                    PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                    PgJsonType::StdStringStringAsJsonbString => quote!{
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
                            Ok(value_75ae8964) => Some(value_75ae8964),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("b9713787")
                            }
                        }
                    },
                }
            }
            else {
                none_ts.clone()
            };
            //todo add contains_el_greater_than for dim 2,3,4
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts = match &pg_json_type_pattern {
                PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => {
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
                                        let value_027d0d3a = #content_ts;
                                        match value_027d0d3a {
                                            Some(value_0cd93c25) => {
                                                acc_f95ec4f2.push(
                                                    #import_path::SingleOrMultiple::Single(
                                                        #ident_where_ucc::ContainsElGreaterThan(
                                                            where_filters::PgJsonTypeWhereContainsElGreaterThan {
                                                                logical_operator: #import_path::LogicalOperator::Or,
                                                                value: #ident_standart_not_null_table_type_declaration_ucc(
                                                                    #ident_standart_not_null_origin_ucc(value_0cd93c25)
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
                                    Ok(value_69c93ec5) => Some(value_69c93ec5),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("47e44ecd")
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
                            PgJsonType::I8AsJsonbNumber |
                            PgJsonType::I16AsJsonbNumber |
                            PgJsonType::I32AsJsonbNumber |
                            PgJsonType::I64AsJsonbNumber |
                            PgJsonType::U8AsJsonbNumber |
                            PgJsonType::U16AsJsonbNumber |
                            PgJsonType::U32AsJsonbNumber |
                            PgJsonType::U64AsJsonbNumber => int_greater_than_one_less_ts,
                            PgJsonType::F32AsJsonbNumber => float_32_greater_than_one_less_ts,
                            PgJsonType::F64AsJsonbNumber => float_64_greater_than_one_less_ts,
                            PgJsonType::BoolAsJsonbBoolean |
                            PgJsonType::StdStringStringAsJsonbString |
                            PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                        }
                    }
                    else {
                        none_ts.clone()
                    }
                },
                PgJsonTypePattern::Standart |
                PgJsonTypePattern::ArrayDim2 {..} |
                PgJsonTypePattern::ArrayDim3 {..} |
                PgJsonTypePattern::ArrayDim4 {..} => none_ts.clone()
            };
            //todo add contains_el_regular_expression for dim 2,3,4
            let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts = match &pg_json_type_pattern {
                PgJsonTypePattern::ArrayDim1 { dim1_is_nullable } => {
                    if matches!((&is_nullable, &dim1_is_nullable), (
                        IsNullable::False,
                        IsNullable::False
                    )) {
                        match &pg_json_type {
                            PgJsonType::I8AsJsonbNumber |
                            PgJsonType::I16AsJsonbNumber |
                            PgJsonType::I32AsJsonbNumber |
                            PgJsonType::I64AsJsonbNumber |
                            PgJsonType::U8AsJsonbNumber |
                            PgJsonType::U16AsJsonbNumber |
                            PgJsonType::U32AsJsonbNumber |
                            PgJsonType::U64AsJsonbNumber |
                            PgJsonType::F32AsJsonbNumber |
                            PgJsonType::F64AsJsonbNumber |
                            PgJsonType::BoolAsJsonbBoolean |
                            PgJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                            PgJsonType::StdStringStringAsJsonbString => quote!{
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
                                    Ok(value_0363f494) => Some(value_0363f494),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!("415a73d9")
                                    }
                                }
                            }
                        }
                    }
                    else {
                        none_ts.clone()
                    }
                },
                PgJsonTypePattern::Standart |
                PgJsonTypePattern::ArrayDim2 {..} |
                PgJsonTypePattern::ArrayDim3 {..} |
                PgJsonTypePattern::ArrayDim4 {..} => none_ts.clone()
            };
            gen_impl_pg_json_type_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &pg_crud_macros_common_import_path_pg_crud_common,
                &ident_read_inner_ucc,
                &ident,
                &option_vec_create_ts,
                &read_only_ids_to_two_dimal_vec_read_inner_ts,
                &read_inner_into_read_with_new_or_try_new_unwraped_ts,
                &read_inner_into_update_with_new_or_try_new_unwraped_ts,
                &read_only_ids_into_option_value_read_inner_ts,
                &update_to_read_only_ids_ts,
                &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
                &previous_read_merged_with_option_update_into_read_ts,
                &read_only_ids_merged_with_create_into_read_ts,
                &read_only_ids_merged_with_create_into_option_value_read_ts,
                &read_only_ids_merged_with_create_into_table_type_declaration_ts,
                &read_only_ids_merged_with_create_into_where_equal_ts,
                &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
                &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts,
                &create_into_pg_json_type_option_vec_where_length_equal_ts,
                &create_into_pg_json_type_option_vec_where_length_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
                &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts,
            )
        };
        let generated = quote! {
            #ident_ts
            #ident_origin_ts
            #ident_table_type_declaration_ts
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
        gen_pg_json_types_config
            .pg_table_columns_content_write_into_pg_table_columns_using_pg_json_types,
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
        let gen_pg_json_types_mod_sc = GenPgJsonTypesModSc;
        let content_ts = pg_json_type_array
            .into_iter()
            .map(|el_af9caefa| el_af9caefa.parse::<Ts2>().expect("84e21b40"))
            .collect::<Vec<Ts2>>();
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #gen_pg_json_types_mod_sc {
                #(#content_ts)*
            }
            pub use #gen_pg_json_types_mod_sc::*;
        }
    };
    maybe_write_ts_into_file(
        gen_pg_json_types_config.whole_content_write_into_gen_pg_json_types,
        "gen_pg_json_types",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
