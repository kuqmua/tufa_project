use naming::{
    ArrayOfUcc, AsUcc, BooleanUcc, ColumnNameAndMaybeFieldGetterSc, CreateForQueryUcc, CreateSc,
    EqualUcc, ErrorSc, GeneratePostgresqlJsonTypesModSc, IncrementSc, JsonbSetAccumulatorSc, NewSc,
    NumberUcc, OptionUpdateSc, OptionVecCreateSc, PostgresqlJsonTypeUcc, QuerySc, ReadInnerUcc,
    ReadOnlyIdsMergedWithCreateIntoReadSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc, ReadOnlyIdsSc,
    ReadOnlyIdsToTwoDimensionalVecReadInnerSc, ReadSc, SelfSc, SelfUcc, StringUcc,
    UpdateForQueryUcc, UpdateUcc, ValueSc, VecOfUcc,
    parameter::{
        JsonbSelfUcc, SelfCreateForQueryUcc, SelfCreateUcc, SelfOriginUcc, SelfReadInnerUcc,
        SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc, SelfTableTypeDeclarationUcc,
        SelfUpdateForQueryUcc, SelfUpdateUcc, SelfWhereUcc,
    },
};
use postgresql_crud_macros_common::{ImportPath, IsStandartNotNull, NotNullOrNullable};
use quote::quote;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as StdFmtResult},
    iter::once,
};

#[must_use]
pub fn generate_postgresql_json_types(
    input_ts: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, strum_macros::Display)]
    enum RustTypeName {
        StdPrimitiveI8,
        StdPrimitiveI16,
        StdPrimitiveI32,
        StdPrimitiveI64,
        StdPrimitiveU8,
        StdPrimitiveU16,
        StdPrimitiveU32,
        StdPrimitiveU64,
        StdPrimitiveF32,
        StdPrimitiveF64,
        StdPrimitiveBool,
        StdStringString,
        UuidUuid,
    }
    impl From<&PostgresqlJsonType> for RustTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => Self::StdPrimitiveI8,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => Self::StdPrimitiveI16,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => Self::StdPrimitiveI32,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => Self::StdPrimitiveI64,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => Self::StdPrimitiveU8,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => Self::StdPrimitiveU16,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => Self::StdPrimitiveU32,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => Self::StdPrimitiveU64,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => Self::StdPrimitiveF32,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::StdPrimitiveF64,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::StdPrimitiveBool,
                PostgresqlJsonType::StdStringStringAsJsonbString => Self::StdStringString,
                PostgresqlJsonType::UuidUuidAsJsonbString => Self::UuidUuid,
            }
        }
    }
    #[derive(Debug)]
    enum PostgresqlJsonTypeName {
        Boolean,
        Number,
        String,
    }
    impl Display for PostgresqlJsonTypeName {
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
    impl From<&PostgresqlJsonType> for PostgresqlJsonTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::Number,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Boolean,
                PostgresqlJsonType::StdStringStringAsJsonbString
                | PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
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
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
    enum PostgresqlJsonType {
        StdPrimitiveI8AsJsonbNumber,
        StdPrimitiveI16AsJsonbNumber,
        StdPrimitiveI32AsJsonbNumber,
        StdPrimitiveI64AsJsonbNumber,
        StdPrimitiveU8AsJsonbNumber,
        StdPrimitiveU16AsJsonbNumber,
        StdPrimitiveU32AsJsonbNumber,
        StdPrimitiveU64AsJsonbNumber,
        StdPrimitiveF32AsJsonbNumber,
        StdPrimitiveF64AsJsonbNumber,
        StdPrimitiveBoolAsJsonbBoolean,
        StdStringStringAsJsonbString,
        UuidUuidAsJsonbString,
    }
    impl quote::ToTokens for PostgresqlJsonType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("eb6cafe0-ad0d-4108-8b0e-c062b155efbb")
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
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
    enum PostgresqlJsonTypePattern {
        Standart,
        ArrayDimension1 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
        },
        ArrayDimension2 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
            dimension2_not_null_or_nullable: NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
            dimension2_not_null_or_nullable: NotNullOrNullable,
            dimension3_not_null_or_nullable: NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
            dimension2_not_null_or_nullable: NotNullOrNullable,
            dimension3_not_null_or_nullable: NotNullOrNullable,
            dimension4_not_null_or_nullable: NotNullOrNullable,
        },
    }
    impl PostgresqlJsonTypePattern {
        const fn down_by_1(&self) -> Option<Self> {
            match &self {
                Self::Standart => None,
                Self::ArrayDimension1 { .. } => Some(Self::Standart),
                Self::ArrayDimension2 {
                    dimension2_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension1 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                Self::ArrayDimension3 {
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
                Self::ArrayDimension4 {
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_2(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } => None,
                Self::ArrayDimension2 { .. } => Some(Self::Standart),
                Self::ArrayDimension3 {
                    dimension3_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension1 {
                    dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
                Self::ArrayDimension4 {
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_3(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } | Self::ArrayDimension2 { .. } => {
                    None
                }
                Self::ArrayDimension3 { .. } => Some(Self::Standart),
                Self::ArrayDimension4 {
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension1 {
                    dimension1_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_4(&self) -> Option<Self> {
            match &self {
                Self::Standart
                | Self::ArrayDimension1 { .. }
                | Self::ArrayDimension2 { .. }
                | Self::ArrayDimension3 { .. } => None,
                Self::ArrayDimension4 { .. } => Some(Self::Standart),
            }
        }
    }
    enum ArrayDimension {
        ArrayDimension1,
        ArrayDimension2 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
            dimension2_not_null_or_nullable: NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: NotNullOrNullable,
            dimension2_not_null_or_nullable: NotNullOrNullable,
            dimension3_not_null_or_nullable: NotNullOrNullable,
        },
    }
    impl ArrayDimension {
        const fn to_usize(&self) -> usize {
            match &self {
                Self::ArrayDimension1 { .. } => 1,
                Self::ArrayDimension2 { .. } => 2,
                Self::ArrayDimension3 { .. } => 3,
                Self::ArrayDimension4 { .. } => 4,
            }
        }
    }
    impl TryFrom<&PostgresqlJsonTypePattern> for ArrayDimension {
        type Error = ();
        fn try_from(value: &PostgresqlJsonTypePattern) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlJsonTypePattern::Standart => Err(()),
                PostgresqlJsonTypePattern::ArrayDimension1 { .. } => Ok(Self::ArrayDimension1),
                PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable,
                    ..
                } => Ok(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                }),
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    ..
                } => Ok(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    ..
                } => Ok(Self::ArrayDimension4 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    struct PostgresqlJsonTypeRecord {
        postgresql_json_type: PostgresqlJsonType,
        not_null_or_nullable: NotNullOrNullable,
        postgresql_json_type_pattern: PostgresqlJsonTypePattern,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlJsonTypesConfigVariant {
        All,
        WithoutDimensions,
        WithDimensionOne,
        WithDimensionTwo,
        WithDimensionThree,
        WithDimensionFour,
        Concrete(Vec<PostgresqlJsonTypeRecord>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize)]
    struct GeneratePostgresqlJsonTypesConfig {
        postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_generate_postgresql_json_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        variant: GeneratePostgresqlJsonTypesConfigVariant,
    }
    panic_location::panic_location();
    let generate_postgresql_json_types_config =
        serde_json::from_str::<GeneratePostgresqlJsonTypesConfig>(&input_ts.to_string())
            .expect("1123f78f-9c84-4001-b619-b534dd55a835");
    let (fields_ts, postgresql_json_type_array) = {
        {
            let generate_standart = |acc_29796d99: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for el_2f39e657 in NotNullOrNullable::into_array() {
                    acc_29796d99.push(PostgresqlJsonTypeRecord {
                        postgresql_json_type: postgresql_json_type.clone(),
                        not_null_or_nullable: el_2f39e657,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                    });
                }
            };
            let generate_array_dimension1 = |acc_5b22a398: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for el_29854486 in NotNullOrNullable::into_array() {
                    for el_6440cc9c in NotNullOrNullable::into_array() {
                        acc_5b22a398.push(PostgresqlJsonTypeRecord {
                            postgresql_json_type: postgresql_json_type.clone(),
                            not_null_or_nullable: el_29854486,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: el_6440cc9c },
                        });
                    }
                }
            };
            let generate_array_dimension2 = |acc_e59f7158: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for el_a6ba4c3e in NotNullOrNullable::into_array() {
                    for el_4b5a815d in NotNullOrNullable::into_array() {
                        for el_2e4896dd in NotNullOrNullable::into_array() {
                            acc_e59f7158.push(PostgresqlJsonTypeRecord {
                                postgresql_json_type: postgresql_json_type.clone(),
                                not_null_or_nullable: el_a6ba4c3e,
                                postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                                    dimension1_not_null_or_nullable: el_4b5a815d,
                                    dimension2_not_null_or_nullable: el_2e4896dd
                                },
                            });
                        }
                    }
                }
            };
            let generate_array_dimension3 = |acc_77498dc3: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for el_8f03b1c2 in NotNullOrNullable::into_array() {
                    for el_a27b642f in NotNullOrNullable::into_array() {
                        for el_dc57e9b7 in NotNullOrNullable::into_array() {
                            for el_4361fee5 in NotNullOrNullable::into_array() {
                                acc_77498dc3.push(PostgresqlJsonTypeRecord {
                                    postgresql_json_type: postgresql_json_type.clone(),
                                    not_null_or_nullable: el_8f03b1c2,
                                    postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                        dimension1_not_null_or_nullable: el_a27b642f,
                                        dimension2_not_null_or_nullable: el_dc57e9b7,
                                        dimension3_not_null_or_nullable: el_4361fee5,
                                    },
                                });
                            }
                        }
                    }
                }
            };
            let generate_array_dimension4 = |acc_7c8a3329: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for el_daf10957 in NotNullOrNullable::into_array() {
                    for el_fc5a53dd in NotNullOrNullable::into_array() {
                        for el_69b59c5c in NotNullOrNullable::into_array() {
                            for el_d7efbd09 in NotNullOrNullable::into_array() {
                                for el_c16cb65b in NotNullOrNullable::into_array() {
                                    acc_7c8a3329.push(PostgresqlJsonTypeRecord {
                                        postgresql_json_type: postgresql_json_type.clone(),
                                        not_null_or_nullable: el_daf10957,
                                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension4 {
                                            dimension1_not_null_or_nullable: el_fc5a53dd,
                                            dimension2_not_null_or_nullable: el_69b59c5c,
                                            dimension3_not_null_or_nullable: el_d7efbd09,
                                            dimension4_not_null_or_nullable: el_c16cb65b,
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
            };
            let acc_d97120ed = match generate_postgresql_json_types_config.variant {
                GeneratePostgresqlJsonTypesConfigVariant::All => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_e2f65a79, postgresql_json_type| {
                    for el_644643cd in PostgresqlJsonTypePattern::into_array() {
                        match &el_644643cd {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } => generate_array_dimension3(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_array_dimension4(&mut acc_e2f65a79, postgresql_json_type.clone()),
                        }
                    }
                    acc_e2f65a79
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithoutDimensions => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_3d95645c, postgresql_json_type| {
                    for el_fccf1979 in PostgresqlJsonTypePattern::into_array() {
                        match &el_fccf1979 {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_3d95645c, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_3d95645c
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionOne => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_66a17cae, postgresql_json_type| {
                    for el_e69bd1fc in PostgresqlJsonTypePattern::into_array() {
                        match &el_e69bd1fc {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_66a17cae, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_66a17cae, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_66a17cae
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionTwo => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_c5ffb796, postgresql_json_type| {
                    for el_345fd6bd in PostgresqlJsonTypePattern::into_array() {
                        match &el_345fd6bd {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_c5ffb796, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_c5ffb796, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_c5ffb796, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_c5ffb796
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionThree => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_78b27c00, postgresql_json_type| {
                    for el_88e3b8fe in PostgresqlJsonTypePattern::into_array() {
                        match &el_88e3b8fe {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } => generate_array_dimension3(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_78b27c00
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionFour => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_172c62ad, postgresql_json_type| {
                    for el_80434642 in PostgresqlJsonTypePattern::into_array() {
                        match &el_80434642 {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } => generate_array_dimension3(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_array_dimension4(&mut acc_172c62ad, postgresql_json_type.clone()),
                        }
                    }
                    acc_172c62ad
                }),
                GeneratePostgresqlJsonTypesConfigVariant::Concrete(value) => value,
            };
            let mut seen = HashSet::new();
            assert!(
                acc_d97120ed
                    .iter()
                    .all(|el_8ef63a77| seen.insert(el_8ef63a77)),
                "c2d37017-229c-4259-bcee-c434852dca1b"
            );
            acc_d97120ed
        }.into_iter().fold(Vec::new(), |mut acc_5e43269c, el_c4f9bf8f| {
            for el_7ae8d2ae in {
                #[derive(Clone)]
                struct PostgresqlJsonTypeRecordHandle {
                    not_null_or_nullable: NotNullOrNullable,
                    postgresql_json_type_pattern: PostgresqlJsonTypePattern,
                }
                fn generate_postgresql_json_type_record_handle_vec(postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle) -> Vec<PostgresqlJsonTypeRecordHandle> {
                    let generate_vec = |current_postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle| generate_postgresql_json_type_record_handle_vec(
                        current_postgresql_json_type_record_handle
                    ).into_iter().chain(once(postgresql_json_type_record_handle.clone())).collect();
                    match (&postgresql_json_type_record_handle.not_null_or_nullable, &postgresql_json_type_record_handle.postgresql_json_type_pattern) {
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => vec![postgresql_json_type_record_handle],
                        (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                        }),
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.down_by_1().expect("0e970a4f-eeec-421e-aa30-f90fc536d388"),
                        }),
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.down_by_1().expect("85f8ed83-aa1f-4f52-8f8f-80aeb86f8083"),
                        }),
                        (
                            NotNullOrNullable::NotNull,
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                            },
                        }),
                        (
                            NotNullOrNullable::NotNull,
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                            },
                        }),
                        (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                        }),
                    }
                }
                generate_postgresql_json_type_record_handle_vec(PostgresqlJsonTypeRecordHandle {
                    not_null_or_nullable: el_c4f9bf8f.not_null_or_nullable,
                    postgresql_json_type_pattern: el_c4f9bf8f.postgresql_json_type_pattern,
                })
            } {
                let postgresql_json_type_record = PostgresqlJsonTypeRecord {
                    postgresql_json_type: el_c4f9bf8f.postgresql_json_type.clone(),
                    not_null_or_nullable: el_7ae8d2ae.not_null_or_nullable,
                    postgresql_json_type_pattern: el_7ae8d2ae.postgresql_json_type_pattern,
                };
                if !acc_5e43269c.contains(&postgresql_json_type_record) {
                    acc_5e43269c.push(postgresql_json_type_record);
                }
            }
            acc_5e43269c
        })
    }
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PostgresqlJsonTypeRecord)>>()
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
        //     metadata: &'lifetime dyn quote::ToTokens,
        //     instance_type: &'lifetime dyn quote::ToTokens,
        //     format: &'lifetime dyn quote::ToTokens,
        //     enum_values: &'lifetime dyn quote::ToTokens,
        //     const_value: &'lifetime dyn quote::ToTokens,
        //     subschemas: &'lifetime dyn quote::ToTokens,
        //     number: &'lifetime dyn quote::ToTokens,
        //     string: &'lifetime dyn quote::ToTokens,
        //     array: &'lifetime dyn quote::ToTokens,
        //     object: &'lifetime dyn quote::ToTokens,
        //     reference: &'lifetime dyn quote::ToTokens,
        //     extensions: &'lifetime dyn quote::ToTokens,
        // }
        // enum SchemarsJsonSchema<'schema_object_ts_tifetime> {
        //     Derive,
        //     Impl(SchemaObjectTokenStream<'schema_object_ts_tifetime>),
        // }
        let postgresql_json_type = &el_1d376874.postgresql_json_type;
        let not_null_or_nullable = &el_1d376874.not_null_or_nullable;
        let postgresql_json_type_pattern = &el_1d376874.postgresql_json_type_pattern;
        let rust_type_name = RustTypeName::from(postgresql_json_type);
        let postgresql_json_type_name = PostgresqlJsonTypeName::from(postgresql_json_type);
        let is_standart_not_null = if matches!((&postgresql_json_type_pattern, &not_null_or_nullable), (PostgresqlJsonTypePattern::Standart, NotNullOrNullable::NotNull)) {
            IsStandartNotNull::True
        } else {
            IsStandartNotNull::False
        };
        let is_standart_not_null_uuid = if matches!((&not_null_or_nullable, &postgresql_json_type_pattern, &postgresql_json_type), (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart, PostgresqlJsonType::UuidUuidAsJsonbString)) {
            IsStandartNotNullUuid::True
        } else {
            IsStandartNotNullUuid::False
        };
        let value_sc = ValueSc;
        let as_ucc = AsUcc;
        let new_sc = NewSc;
        let self_ucc = SelfUcc;
        let increment_sc = IncrementSc;
        let query_sc = QuerySc;
        let read_sc = ReadSc;
        let error_sc = ErrorSc;
        let option_vec_create_sc = OptionVecCreateSc;
        let option_update_sc = OptionUpdateSc;
        let read_only_ids_to_two_dimensional_vec_read_inner_sc = ReadOnlyIdsToTwoDimensionalVecReadInnerSc;
        let create_sc = CreateSc;
        let read_only_ids_sc = ReadOnlyIdsSc;
        let postgresql_json_type_ucc = PostgresqlJsonTypeUcc;
        let import_path = ImportPath::PostgresqlCrudCommon;
        let create_for_query_ucc = CreateForQueryUcc;
        let update_for_query_ucc = UpdateForQueryUcc;
        let update_ucc = UpdateUcc;
        let self_sc = SelfSc;
        let equal_ucc = EqualUcc;
        let read_inner_ucc = ReadInnerUcc;
        let read_only_ids_merged_with_create_into_read_sc = ReadOnlyIdsMergedWithCreateIntoReadSc;
        let read_only_ids_merged_with_create_into_where_equal_sc = ReadOnlyIdsMergedWithCreateIntoWhereEqualSc;
        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc = ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc;

        let std_primitive_i8_ts = token_patterns::StdPrimitiveI8;
        let std_primitive_i16_ts = token_patterns::StdPrimitiveI16;
        let std_primitive_i32_ts = token_patterns::StdPrimitiveI32;
        let std_primitive_i64_ts = token_patterns::StdPrimitiveI64;
        let std_primitive_u8_ts = token_patterns::StdPrimitiveU8;
        let std_primitive_u16_ts = token_patterns::StdPrimitiveU16;
        let std_primitive_u32_ts = token_patterns::StdPrimitiveU32;
        let std_primitive_u64_ts = token_patterns::StdPrimitiveU64;
        let std_primitive_f32_ts = token_patterns::StdPrimitiveF32;
        let std_primitive_f64_ts = token_patterns::StdPrimitiveF64;
        let std_primitive_bool_ts = token_patterns::StdPrimitiveBool;
        let std_string_string_ts = token_patterns::StdStringString;
        let uuid_uuid_ts = token_patterns::UuidUuid;
        // let schemars_json_schema_ts = token_patterns::SchemarsJsonSchema;

        let none_ts = quote! {None};
        let must_use_ts = token_patterns::MustUse;
        let allow_clippy_arbitrary_source_item_ordering_ts = token_patterns::AllowClippyArbitrarySourceItemOrdering;

        let postgresql_crud_common_default_option_some_vec_one_el_ts = token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneEl;
        let postgresql_crud_common_default_option_some_vec_one_el_call_ts = token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneElCall;
        let postgresql_crud_common_default_option_some_vec_one_el_max_page_size_call_ts = token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall;

        let generate_import_path_value_initialization_ts = |content_ts: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_value_initialization_ts(&import_path, &content_ts);
        let generate_ident_ts = |current_not_null_or_nullable: &NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
            let vec_of_ucc = VecOfUcc;
            let array_of_ucc = ArrayOfUcc;
            let not_null_or_nullable_rust = current_not_null_or_nullable.rust();
            let (rust_part, postgresql_part) = match &current_postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => (rust_type_name.to_string(), postgresql_json_type_name.to_string()),
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    (format!("{vec_of_ucc}{d1_rust}{rust_type_name}"), format!("{array_of_ucc}{d1}{postgresql_json_type_name}"))
                }
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    (format!("{vec_of_ucc}{d1_rust}{vec_of_ucc}{d2_rust}{rust_type_name}"), format!("{array_of_ucc}{d1}{array_of_ucc}{d2}{postgresql_json_type_name}"))
                }
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    let d3 = dimension3_not_null_or_nullable;
                    let d3_rust = dimension3_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_ucc}{d1_rust}{vec_of_ucc}{d2_rust}{vec_of_ucc}{d3_rust}{rust_type_name}"),
                        format!("{array_of_ucc}{d1}{array_of_ucc}{d2}{array_of_ucc}{d3}{postgresql_json_type_name}"),
                    )
                }
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    let d3 = dimension3_not_null_or_nullable;
                    let d3_rust = dimension3_not_null_or_nullable.rust();
                    let d4 = dimension4_not_null_or_nullable;
                    let d4_rust = dimension4_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_ucc}{d1_rust}{vec_of_ucc}{d2_rust}{vec_of_ucc}{d3_rust}{vec_of_ucc}{d4_rust}{rust_type_name}"),
                        format!("{array_of_ucc}{d1}{array_of_ucc}{d2}{array_of_ucc}{d3}{array_of_ucc}{d4}{postgresql_json_type_name}"),
                    )
                }
            };
            format!("{not_null_or_nullable_rust}{rust_part}{as_ucc}{current_not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().expect("998d1471-be98-4669-8bd3-ca6c4a1a5853")
        };
        let ident = &generate_ident_ts(not_null_or_nullable, postgresql_json_type_pattern);
        let ident_standart_not_null_ucc = &generate_ident_ts(&NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart);
        let ident_standart_not_null_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident);
        let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
        let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
        let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
        let ident_not_null_ts = generate_ident_ts(&NotNullOrNullable::NotNull, postgresql_json_type_pattern);
        let ident_ts = {
            let ident_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .build_struct(
                    &ident,
                    &quote!{;}
                );
            quote! {
                #ident_ts
            }
        };
        let ident_standart_not_null_origin_ucc = SelfOriginUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_origin_ucc = SelfOriginUcc::from_tokens(&ident);
        let ident_read_inner_standart_not_null_alias_ts = {
            let content_ts: &dyn quote::ToTokens = match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => &std_primitive_i8_ts,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => &std_primitive_i16_ts,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => &std_primitive_i32_ts,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => &std_primitive_i64_ts,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => &std_primitive_u8_ts,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => &std_primitive_u16_ts,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => &std_primitive_u32_ts,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => &std_primitive_u64_ts,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => &std_primitive_f32_ts,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => &std_primitive_f64_ts,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => &std_primitive_bool_ts,
                PostgresqlJsonType::StdStringStringAsJsonbString => &std_string_string_ts,
                PostgresqlJsonType::UuidUuidAsJsonbString => &uuid_uuid_ts,
            };
            quote! {#content_ts}
        };
        let ident_read_inner_ucc = SelfReadInnerUcc::from_tokens(&ident);
        let value_ident_read_inner_ts = quote! {#value_sc: #ident_read_inner_ucc};
        let generate_pub_fn_new_value_ident_read_inner_content_ts = |content_ts: &dyn quote::ToTokens| macros_helpers::generate_pub_new_ts(
            &must_use_ts,
            &value_ident_read_inner_ts,
            &content_ts
        );
        let generate_pub_const_fn_new_value_ident_read_inner_content_ts = |content_ts: &dyn quote::ToTokens| macros_helpers::generate_pub_const_new_ts(
            &must_use_ts,
            &value_ident_read_inner_ts,
            &content_ts
        );
        let self_ident_origin_new_value_ts = quote! {Self(#ident_origin_ucc::new(#value_sc))};
        let maybe_const_fn = match &postgresql_json_type_pattern {
            PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                NotNullOrNullable::NotNull => ConstFn::True,
                NotNullOrNullable::Nullable => ConstFn::False,
            },
            PostgresqlJsonTypePattern::ArrayDimension1 { .. } |
            PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => ConstFn::False,
        };
        let generate_pub_new_or_fn_new_ts = |const_new_ts: &dyn quote::ToTokens, new_ts: &dyn quote::ToTokens|match maybe_const_fn {
            ConstFn::False => generate_pub_fn_new_value_ident_read_inner_content_ts(
                &new_ts
            ),
            ConstFn::True => generate_pub_const_fn_new_value_ident_read_inner_content_ts(
                &const_new_ts
            ),
        };
        let pub_new_or_const_new_self_ident_origin_new_value_ts = generate_pub_new_or_fn_new_ts(
            &self_ident_origin_new_value_ts,
            &self_ident_origin_new_value_ts
        );
        let ident_create_for_query_ucc = SelfCreateForQueryUcc::from_tokens(&ident);
        let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
        let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
        let maybe_derive_copy = match &postgresql_json_type_pattern {
            PostgresqlJsonTypePattern::Standart => match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                PostgresqlJsonType::UuidUuidAsJsonbString => macros_helpers::DeriveCopy::True,
                PostgresqlJsonType::StdStringStringAsJsonbString => macros_helpers::DeriveCopy::False,
            },
            PostgresqlJsonTypePattern::ArrayDimension1 {..} |
            PostgresqlJsonTypePattern::ArrayDimension2 {..} |
            PostgresqlJsonTypePattern::ArrayDimension3 {..} |
            PostgresqlJsonTypePattern::ArrayDimension4 {..} => macros_helpers::DeriveCopy::False,
        };
        let ident_origin_ts = {
            let generate_current_ident_origin_non_wrapping = |
                current_not_null_or_nullable: &NotNullOrNullable,
                current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern
            | SelfOriginUcc::from_tokens(&generate_ident_ts(current_not_null_or_nullable, current_postgresql_json_type_pattern));
            // let schema_name_format_handle_ts = generate_quotes::double_quotes_ts(&ident_origin_ucc);
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
            //     let generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type = |instance_type: &schemars::schema::InstanceType| {
            //         let instance_type_ts: &dyn quote::ToTokens = match &instance_type {
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
            //         generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::Number),
            //         generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::String),
            //     )
            // };
            // let schemars_json_schema = if let IsStandartNotNull::True = &is_standart_not_null {
            //     match &postgresql_json_type {
            //         PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
            //             metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_ts,
            //             instance_type: &instance_type_number_ts,
            //             format: &none_ucc,
            //             enum_values: &none_ucc,
            //             const_value: &none_ucc,
            //             subschemas: &none_ucc,
            //             number: &quote! {Some(Box::new(schemars::schema::NumberValidation {
            //                 multiple_of: None,
            //                 maximum: Some(#ident_read_inner_standart_not_null_alias_ts ::MAX as #std_primitive_f64_ts),
            //                 exclusive_maximum: None,
            //                 minimum: Some(#ident_read_inner_standart_not_null_alias_ts ::MIN as #std_primitive_f64_ts),
            //                 exclusive_minimum: None,
            //             }))},
            //             string: &none_ucc,
            //             array: &none_ucc,
            //             object: &none_ucc,
            //             reference: &none_ucc,
            //             extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_ts,
            //         }),
            //         PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean | PostgresqlJsonType::StdStringStringAsJsonbString => SchemarsJsonSchema::Derive,
            //         PostgresqlJsonType::UuidUuidAsJsonbString => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
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
            let ident_origin_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                    //     SchemarsJsonSchema::Derive => macros_helpers::DeriveSchemarsJsonSchema::True,
                    //     SchemarsJsonSchema::Impl(_) => macros_helpers::DeriveSchemarsJsonSchema::False,
                    // }
                    if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                        match &postgresql_json_type {
                            PostgresqlJsonType::UuidUuidAsJsonbString => macros_helpers::DeriveSchemarsJsonSchema::False,
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                            | PostgresqlJsonType::StdStringStringAsJsonbString => macros_helpers::DeriveSchemarsJsonSchema::True,
                        }
                    } else {
                        macros_helpers::DeriveSchemarsJsonSchema::True
                    }
                )
                .build_struct(
                    &ident_origin_ucc,
                    &{
                        let content_ts: &dyn quote::ToTokens = {
                            let generate_current_ident_origin = |current_not_null_or_nullable: &NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                                let value = generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern);
                                match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&value),
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&value),
                                }
                            };
                            match &postgresql_json_type_pattern {
                                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => &ident_read_inner_standart_not_null_alias_ts,
                                    NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_standart_not_null_origin_ucc),
                                },
                                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("e994797d-7334-4e30-b180-af24c16b68b1")),
                                        NotNullOrNullable::Nullable => (&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("76eb44e3-3099-4c9a-a935-da3e6f6e4210")),
                                        NotNullOrNullable::Nullable => (&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                                PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("1b996c86-0b08-476a-b963-373dd6838496")),
                                        NotNullOrNullable::Nullable => (&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                                PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("d24b7481-27d9-40c0-8476-344a16d08f27")),
                                        NotNullOrNullable::Nullable => (&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                            }
                        };
                        quote!{(#content_ts);}
                    }
                );
            let ident_origin_impl_new_self_content_ts = {
                let generate_value_map_type_new_ts = |type_ts: &dyn quote::ToTokens| quote! {#value_sc.map(#type_ts::#new_sc)};
                let generate_array_dimensions_initialization_ts = |type_ts: &dyn quote::ToTokens| match &not_null_or_nullable {
                    NotNullOrNullable::NotNull => quote! {#value_sc.into_iter().map(#type_ts::#new_sc).collect()},
                    NotNullOrNullable::Nullable => generate_value_map_type_new_ts(&type_ts),
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {value},
                        NotNullOrNullable::Nullable => generate_value_map_type_new_ts(&ident_standart_not_null_origin_ucc),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_array_dimensions_initialization_ts(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("1160d3df-2e2b-4a33-a297-6b48546b9ca8"), dimension1_not_null_or_nullable),
                            NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_ts(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("8ab62f4e-611b-4228-a295-3e731e934b9c"), dimension1_not_null_or_nullable),
                            NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                    PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_ts(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("ed64919d-4679-45da-9d14-22ddee84716b"), dimension1_not_null_or_nullable),
                            NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                    PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_ts(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &NotNullOrNullable) = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("25646d29-5a30-49fb-b91a-7b49ed8c5b0a"), dimension1_not_null_or_nullable),
                            NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                }
            };
            let impl_ident_origin_ts = {
                let pub_fn_new_ts = {
                    let self_ident_origin_impl_new_self_content_ts = quote!{
                        Self(#ident_origin_impl_new_self_content_ts)
                    };
                    generate_pub_new_or_fn_new_ts(
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
            let impl_std_convert_from_ident_create_for_ident_origin_ts = macros_helpers::generate_impl_std_convert_from_ts(&ident_create_ucc, &ident_origin_ucc, &quote! {#value_sc.0});
            let impl_std_convert_from_ident_update_for_ident_origin_ts = macros_helpers::generate_impl_std_convert_from_ts(&ident_update_ucc, &ident_origin_ucc, &quote! {#value_sc.0});
            //todo
            let maybe_impl_schemars_json_schema_for_ident_origin_ts = if matches!(&is_standart_not_null, IsStandartNotNull::True) {
                match &postgresql_json_type {
                    PostgresqlJsonType::UuidUuidAsJsonbString => {
                        let ident_standart_not_null_origin_double_quotes_ts = generate_quotes::double_quotes_ts(
                            &ident_standart_not_null_origin_ucc
                        );
                        let text_ident_standart_not_null_origin_double_quotes_ts = generate_quotes::double_quotes_ts(
                            &format!("tests::{ident_standart_not_null_origin_ucc}")
                        );
                        quote!{
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #allow_clippy_arbitrary_source_item_ordering_ts
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
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                    | PostgresqlJsonType::StdStringStringAsJsonbString => proc_macro2::TokenStream::new(),
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            // match &schemars_json_schema {
            //     SchemarsJsonSchema::Derive => &proc_macro2::TokenStream::new(),
            //     SchemarsJsonSchema::Impl(schema_object_ts) => &{
            //         let schema_id_format_handle_ts = generate_quotes::double_quotes_ts(&format!("postgresql_crud::postgersql_json_type::{ident_origin_ucc}"));
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
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => proc_macro2::TokenStream::new(),
                    PostgresqlJsonType::StdStringStringAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_content_ts(
                        &ident_origin_ucc,
                        &quote!{self.0.clone().is_empty()}
                    ),
                    PostgresqlJsonType::UuidUuidAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_content_ts(
                        &ident_origin_ucc,
                        &quote!{self.0.to_string().is_empty()}
                    ),
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_std_fmt_display_for_ident_origin_ts = macros_helpers::generate_impl_std_fmt_display_ts(&proc_macro2::TokenStream::new(), &ident_origin_ucc, &proc_macro2::TokenStream::new(), &quote! {write!(f, "{self:?}")});
            let impl_error_occurence_lib_to_std_string_string_for_ident_origin_ts = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_ts(&proc_macro2::TokenStream::new(), &ident_origin_ucc, &proc_macro2::TokenStream::new(), &quote! {format!("{self:#?}")});
            let impl_default_option_some_vec_one_el_for_ident_origin_ts = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(&ident_origin_ucc, &{
                let content_ts = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                            | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote! {Default::default()},
                            PostgresqlJsonType::StdStringStringAsJsonbString => quote! {String::default()},
                            PostgresqlJsonType::UuidUuidAsJsonbString => quote! {uuid::Uuid::new_v4()},
                        },
                        NotNullOrNullable::Nullable => quote! {Some(#postgresql_crud_common_default_option_some_vec_one_el_call_ts)},
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {vec![#postgresql_crud_common_default_option_some_vec_one_el_call_ts]},
                        NotNullOrNullable::Nullable => quote! {Some(#postgresql_crud_common_default_option_some_vec_one_el_call_ts)},
                    },
                };
                quote! {Self(#content_ts)}
            });
            let impl_sqlx_type_sqlx_postgres_for_ident_origin_ts = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_origin_ucc, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_ts = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_origin_ucc, &quote! {sqlx::types::Json(&#self_sc.0)});
            quote! {
                #ident_origin_ts
                #impl_ident_origin_ts
                #impl_std_convert_from_ident_create_for_ident_origin_ts
                #impl_std_convert_from_ident_update_for_ident_origin_ts
                #maybe_impl_schemars_json_schema_for_ident_origin_ts
                #maybe_impl_is_string_empty_for_ident_origin_ts
                #impl_std_fmt_display_for_ident_origin_ts
                #impl_error_occurence_lib_to_std_string_string_for_ident_origin_ts
                #impl_default_option_some_vec_one_el_for_ident_origin_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_origin_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_origin_ts
            }
        };
        let ident_origin_struct_content_ts = quote!{(#ident_origin_ucc);};
        let ident_table_type_declaration_ts = {
            let ident_table_type_declaration_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(&ident_table_type_declaration_ucc, &quote! {Self(#postgresql_crud_common_default_option_some_vec_one_el_call_ts)});
            //todo maybe add to trait?
            let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_ts = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_table_type_declaration_ucc, &quote! {&#self_sc.0});
            //todo maybe add to trait?
            let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_ts = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_table_type_declaration_ucc, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            quote! {
                #ident_table_type_declaration_ts
                #impl_ident_table_type_declaration_ts
                #impl_default_option_some_vec_one_el_for_ident_table_type_declaration_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_ts
            }
        };
        let ident_create_ts = {
            let ident_create_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(&ident_create_ucc, &quote! {Self(#postgresql_crud_common_default_option_some_vec_one_el_call_ts)});
            quote! {
                #ident_create_ts
                #impl_ident_create_ts
                #impl_default_option_some_vec_one_el_for_ident_create_ts
            }
        };
        let ident_create_for_query_ts = {
            let ident_create_for_query_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &ident_create_for_query_ucc,
                    &ident_origin_struct_content_ts
                );
            let impl_ident_create_for_query_ts = {
                quote! {
                    impl #ident_create_for_query_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_ts = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_create_for_query_ucc, &quote! {sqlx::types::Json(&#self_sc.0)});
            let impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_ts = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_create_for_query_ucc, &ident_origin_ucc);
            let impl_std_convert_from_ident_create_for_ident_create_for_query_ts = macros_helpers::generate_impl_std_convert_from_ts(&ident_create_ucc, &ident_create_for_query_ucc, &quote! {Self(#value_sc.0)});
            let maybe_impl_std_convert_from_ident_update_for_ident_create_for_query_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                macros_helpers::generate_impl_std_convert_from_ts(&ident_update_ucc, &ident_create_for_query_ucc, &quote! {Self(#value_sc.0)})
            } else {
                proc_macro2::TokenStream::new()
            };
            quote! {
                #ident_create_for_query_ts
                #impl_ident_create_for_query_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_ts
                #impl_std_convert_from_ident_create_for_ident_create_for_query_ts
                #maybe_impl_std_convert_from_ident_update_for_ident_create_for_query_ts
            }
        };
        let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
        let ident_select_ts = {
            let ident_select_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                    &ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                        |()| quote! {;},
                        |array_dimension| {
                            let mut arguments_ts = Vec::new();
                            for el_419a74e5 in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_ts = format!("dimension{el_419a74e5}_pagination").parse::<proc_macro2::TokenStream>()
                                .expect("2ad1faf7-57a8-4cfb-8b71-0082b06436ea");
                                arguments_ts.push(quote! {
                                    #dimension_number_pagination_ts: #import_path::PaginationStartsWithZero
                                });
                            }
                            quote! {{#(#arguments_ts),*}}
                        }
                    )
                );
            let generate_default_some_one_content_ts = |default_some_one_or_default_some_one_with_max_page_size: &postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| {
                let content_ts = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                    |()| proc_macro2::TokenStream::new(),
                    |array_dimension| {
                        let content_ts: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgresql_crud_common_default_option_some_vec_one_el_call_ts,
                            postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgresql_crud_common_default_option_some_vec_one_el_max_page_size_call_ts,
                        };
                        let mut arguments_ts = Vec::new();
                        for el_d56aec99 in 1..=array_dimension.to_usize() {
                            let dimension_number_pagination_ts = format!("dimension{el_d56aec99}_pagination").parse::<proc_macro2::TokenStream>().expect("26ca29fb-fd98-466a-a380-974a6c5d4166");
                            arguments_ts.push(quote! {
                                #dimension_number_pagination_ts: #content_ts
                            });
                        }
                        quote! {#(#arguments_ts),*}
                    }
                );
                quote! {Self{#content_ts}}
            };
            let impl_default_option_some_vec_one_el_for_postgresql_json_type_ident_select_ts =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(&ident_select_ucc, &generate_default_some_one_content_ts(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne));
            let impl_default_option_some_vec_one_el_max_page_size_for_postgresql_json_type_ident_select_ts =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_max_page_size_ts(&ident_select_ucc, &generate_default_some_one_content_ts(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize));
            quote! {
                #ident_select_ts
                #impl_default_option_some_vec_one_el_for_postgresql_json_type_ident_select_ts
                #impl_default_option_some_vec_one_el_max_page_size_for_postgresql_json_type_ident_select_ts
            }
        };
        let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
        let ident_where_ts = match &not_null_or_nullable {
            NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_postgresql_type_where_ts(
                &allow_clippy_arbitrary_source_item_ordering_ts,
                &{
                    #[derive(Debug, Clone)]
                    enum PostgresqlJsonTypeSpecific {
                        Bool,
                        Number,
                        String,
                    }
                    impl From<&PostgresqlJsonType> for PostgresqlJsonTypeSpecific {
                        fn from(value: &PostgresqlJsonType) -> Self {
                            match value {
                                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                                | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::Number,
                                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Bool,
                                PostgresqlJsonType::StdStringStringAsJsonbString | PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
                            }
                        }
                    }
                    let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(postgresql_json_type);
                    let common_postgresql_json_type_filters = vec![postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { ident: quote! {#ident_table_type_declaration_ucc} }];
                    let ident_table_type_declaration_ucc_ts = quote! {#ident_table_type_declaration_ucc};
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => {
                            let common_standart_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                                    ident: ident_table_type_declaration_ucc_ts.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut vec = common_standart_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                                        ident: ident_table_type_declaration_ucc_ts.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { ident: ident_table_type_declaration_ucc_ts });
                                    vec
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_standart_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut vec = common_standart_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression);
                                    vec
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                            let array_dimension1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("21eaebaf-cd7a-4625-9232-0e23788a5432")));
                                quote! {#value}
                            };
                            let common_array_dimension1_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension1_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                                        ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween {
                                        ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElGreaterThan {
                                        ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                                        ident: array_dimension1_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension1_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension1_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                            let array_dimension1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("0c4491c4-8364-4c27-9478-227aefadb086")));
                                quote! {#value}
                            };
                            let array_dimension2_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("2d4ee5d4-490e-4503-91a7-ed29f73e6219")));
                                quote! {#value}
                            };
                            let common_array_dimension2_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension2_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                                        ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween {
                                        ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElGreaterThan {
                                        ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                                        ident: array_dimension2_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension2_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension2_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                        } => {
                            let array_dimension1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("3450bef4-e5f3-4bcd-b2de-4e4c67143336")));
                                quote! {#value}
                            };
                            let array_dimension2_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("3c0d10f4-6d7d-45d0-b929-5e307c7d79b1")));
                                quote! {#value}
                            };
                            let array_dimension3_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension3_not_null_or_nullable, &postgresql_json_type_pattern.down_by_3().expect("9aaf9e82-0a92-4848-bfd4-de49013972a5")));
                                quote! {#value}
                            };
                            let common_array_dimension3_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension3_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                                        ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween {
                                        ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElGreaterThan {
                                        ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                                        ident: array_dimension3_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension3_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension3_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                            dimension4_not_null_or_nullable,
                        } => {
                            let array_dimension1_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("550d313b-e925-496d-8a57-87931c573155")));
                                quote! {#value}
                            };
                            let array_dimension2_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("7bda1424-64c0-402e-9bf8-44d5fb3b9903")));
                                quote! {#value}
                            };
                            let array_dimension3_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension3_not_null_or_nullable, &postgresql_json_type_pattern.down_by_3().expect("b43aa5bd-9bba-4f3e-b93b-a41f108262ff")));
                                quote! {#value}
                            };
                            let array_dimension4_inner_el_ident_table_type_declaration_ucc = {
                                let value = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(dimension4_not_null_or_nullable, &postgresql_json_type_pattern.down_by_4().expect("a246885a-ca72-4e37-a396-b7220e237c7e")));
                                quote! {#value}
                            };
                            let common_array_dimension4_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {
                                    ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                                    ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                                    ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                                    ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                    ident: array_dimension2_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                    ident: array_dimension3_inner_el_ident_table_type_declaration_ucc,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn {
                                    ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension4_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                                        ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween {
                                        ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElGreaterThan {
                                        ident: array_dimension4_inner_el_ident_table_type_declaration_ucc.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                                        ident: array_dimension4_inner_el_ident_table_type_declaration_ucc,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension4_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension4_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                    }
                }
                .iter()
                .map(|el_f992f593| {
                    let handle: &dyn postgresql_crud_macros_common::PostgresqlFilter = el_f992f593;
                    handle
                })
                .collect(),
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveUtoipaToSchema::True,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            ),
            NotNullOrNullable::Nullable => quote! {
                pub type #ident_where_ucc = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<
                    <#ident_not_null_ts as #import_path::PostgresqlJsonType>::Where
                >;
            }
        };
        //exists because need to implement .into_inner() for fields (only for read subtype)
        let ident_read_ts = {
            //todo maybe add some derive\impl to trait
            let ident_read_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(&ident_read_ucc, &quote! {Self(#postgresql_crud_common_default_option_some_vec_one_el_call_ts)});
            let impl_sqlx_encode_sqlx_postgres_for_ident_read_ts = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_read_ucc, &quote! {&#self_sc.0});
            let impl_sqlx_type_sqlx_postgres_for_ident_read_ts = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_read_ucc, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_ts(&ident_read_inner_ucc));
            quote! {
                #ident_read_ts
                #impl_ident_read_ts
                #impl_default_option_some_vec_one_el_for_ident_read_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_read_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_read_ts
            }
        };
        let ident_read_only_ids_standart_not_null_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident_standart_not_null_ucc);
        let ident_read_only_ids_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_struct(
                &ident_read_only_ids_ucc,
                &{
                    let std_option_option_unit_ts = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&quote! {()});
                    let vec_ts = |value: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&value);
                    let content_ts = if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                        match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => {
                                let ts1 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                };
                                quote! {#ts1}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let ts1 = vec_ts(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts1),
                                };
                                quote! {#ts2}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                let ts1 = vec_ts(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = vec_ts(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts1),
                                });
                                let ts3 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts2),
                                };
                                quote! {#ts3}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => {
                                let ts1 = vec_ts(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = vec_ts(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts1),
                                });
                                let ts3 = vec_ts(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts2),
                                });
                                let ts4 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts3,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts3),
                                };
                                quote! {#ts4}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            } => {
                                let ts1 = vec_ts(&match &dimension4_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote! {#ident_read_inner_standart_not_null_alias_ts},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                                });
                                let ts2 = vec_ts(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts1),
                                });
                                let ts3 = vec_ts(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts2),
                                });
                                let ts4 = vec_ts(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts3,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts3),
                                });
                                let ts5 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => ts4,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ts4),
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
            let type_ts = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                    NotNullOrNullable::NotNull => &ident_read_inner_standart_not_null_alias_ts,
                    NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&ident_read_inner_standart_not_null_alias_ts),
                },
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => &{
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => &{
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension3_type));
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => &{
                    let dimension4_type = dimension4_not_null_or_nullable.maybe_option_wrap(quote! {#ident_read_inner_standart_not_null_alias_ts});
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension4_type));
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension3_type));
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(&dimension1_type))
                },
            };
            let impl_std_convert_from_ident_origin_for_ident_read_inner_ts = {
                let value_dot_zero_ts = quote!{#value_sc.0};
                let nullable_ts = quote!{#value_dot_zero_ts.map(Into::into)};
                let into_inner_content_ts = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => value_dot_zero_ts,
                        NotNullOrNullable::Nullable => nullable_ts,
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 {..} |
                    PostgresqlJsonTypePattern::ArrayDimension2 {..} |
                    PostgresqlJsonTypePattern::ArrayDimension3 {..} |
                    PostgresqlJsonTypePattern::ArrayDimension4 {..} => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote!{#value_dot_zero_ts.into_iter().map(Into::into).collect()},
                        NotNullOrNullable::Nullable => nullable_ts
                    },
                };
                quote! {
                    impl From<#ident_origin_ucc> for #ident_read_inner_ucc {
                        fn from(#value_sc: #ident_origin_ucc) -> Self {
                            #into_inner_content_ts
                        }
                    }
                }
            };
            quote! {
                pub type #ident_read_inner_ucc = #type_ts;
                #impl_std_convert_from_ident_origin_for_ident_read_inner_ts
            }
        };
        let ident_update_ts = {
            let ident_update_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
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
                    &ident_origin_struct_content_ts
                );
            let impl_ident_update_ts = {
                quote!{
                    impl #ident_update_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_error_occurence_lib_to_std_string_string_for_ident_update_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_ts(&proc_macro2::TokenStream::new(), &ident_update_ucc, &proc_macro2::TokenStream::new(), &quote! {format!("{self:?}")})
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_default_option_some_vec_one_el_for_ident_update_ts =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(&ident_update_ucc, &quote! {Self(#postgresql_crud_common_default_option_some_vec_one_el_call_ts)});
            quote! {
                #ident_update_ts
                #impl_ident_update_ts
                #impl_error_occurence_lib_to_std_string_string_for_ident_update_ts
                #impl_default_option_some_vec_one_el_for_ident_update_ts
            }
        };
        let ident_update_for_query_ts = {
            let ident_update_for_query_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &ident_update_for_query_ucc,
                    &ident_origin_struct_content_ts
                );
            let impl_ident_update_for_query_ts = {
                quote! {
                    impl #ident_update_for_query_ucc {
                        #pub_new_or_const_new_self_ident_origin_new_value_ts
                    }
                }
            };
            let impl_std_convert_from_ident_update_for_ident_update_for_query_ts = macros_helpers::generate_impl_std_convert_from_ts(&ident_update_ucc, &ident_update_for_query_ucc, &quote! {Self(#value_sc.0)});
            //its only for primitive json types
            let impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_ts = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_ts(&ident_update_for_query_ucc, &quote! {sqlx::types::Json(&#self_sc.0)});
            let impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_ts = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_ts(&ident_update_for_query_ucc, &ident_origin_ucc);
            quote! {
                #ident_update_for_query_ts
                #impl_ident_update_for_query_ts
                #impl_std_convert_from_ident_update_for_ident_update_for_query_ts
                #impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_ts
                #impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_ts
            }
        };
        let postgresql_crud_macros_common_import_path_postgresql_crud_common = ImportPath::PostgresqlCrudCommon;
        let impl_postgresql_json_type_for_ident_ts = {
            let generate_dimension_number_stringified = |dimensions_number: usize| format!("dimension{dimensions_number}");
            let generate_dimension_number_start_stringified = |dimensions_number: usize| format!("{}_start", generate_dimension_number_stringified(dimensions_number));
            let generate_dimension_number_end_stringified = |dimensions_number: usize| format!("{}_end", generate_dimension_number_stringified(dimensions_number));
            let select_only_created_or_updated_ids_query_part_ts = if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                quote! {
                    match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                        Ok(value_f06128be) => Ok(format!("'{field_ident}',jsonb_build_object('value',${value_f06128be}),")),
                        Err(#error_sc) => Err(#error_sc),
                    }
                }
            } else {
                quote! {Ok(generate_postgresql_json_types_common::field_ident_jsonb_build_object_value(field_ident))}
            };
            let select_only_created_or_updated_ids_query_bind_ts = if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                quote! {
                    if let Err(#error_sc) = #query_sc.try_bind(#value_sc) {
                        return Err(#error_sc.to_string());
                    }
                    Ok(#query_sc)
                }
            } else {
                quote! {Ok(#query_sc)}
            };
            postgresql_crud_macros_common::generate_impl_postgresql_json_type_ts(
                &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                &ident,
                &ident_table_type_declaration_ucc,
                &ident_create_ucc,
                &ident_create_for_query_ucc,
                &ident_select_ucc,
                &match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::False,
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                },
                &postgresql_crud_macros_common::IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::False,
                &postgresql_crud_macros_common::IsSelectQueryPartIsPostgresqlTypeUsed::False,
                &{
                    let format_handle = {
                        //last child dimension value does not matter - null or type - works both good
                        let column_name_and_maybe_field_getter_field_ident = format!("{{{ColumnNameAndMaybeFieldGetterSc}}}->'{{field_ident}}'");
                        let format_handle = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                            |()| column_name_and_maybe_field_getter_field_ident.clone(),
                            |array_dimension| {
                                enum ArrayDimensionSelectPattern {
                                    ArrayDimension2 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable,
                                    },
                                    ArrayDimension3 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable,
                                        dimension2_not_null_or_nullable: NotNullOrNullable,
                                    },
                                    ArrayDimension4 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable,
                                        dimension2_not_null_or_nullable: NotNullOrNullable,
                                        dimension3_not_null_or_nullable: NotNullOrNullable,
                                    },
                                }
                                impl TryFrom<&ArrayDimension> for ArrayDimensionSelectPattern {
                                    type Error = ();
                                    fn try_from(value: &ArrayDimension) -> Result<Self, Self::Error> {
                                        match &value {
                                            ArrayDimension::ArrayDimension1 => Err(()),
                                            ArrayDimension::ArrayDimension2 {
                                                dimension1_not_null_or_nullable,
                                            } => Ok(Self::ArrayDimension2 {
                                                dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                            }),
                                            ArrayDimension::ArrayDimension3 {
                                                dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable,
                                            } => Ok(Self::ArrayDimension3 {
                                                dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                            }),
                                            ArrayDimension::ArrayDimension4 {
                                                dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable,
                                                dimension3_not_null_or_nullable,
                                            } => Ok(Self::ArrayDimension4 {
                                                dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                            }),
                                        }
                                    }
                                }
                                let generate_jsonb_agg = |jsonb_agg_content: &str, jsonb_array_elements_content: &str, ordinality_content: &str, dimensions_number: usize| {
                                    let dimension_number_start = generate_dimension_number_start_stringified(dimensions_number);
                                    let dimension_number_end = generate_dimension_number_end_stringified(dimensions_number);
                                    format!(
                                        "select jsonb_agg(({jsonb_agg_content})) from jsonb_array_elements(({jsonb_array_elements_content})) with ordinality {ordinality_content} between {{{dimension_number_start}}} and {{{dimension_number_end}}}"
                                    )
                                };
                                ArrayDimensionSelectPattern::try_from(&array_dimension).map_or_else(
                                    |()| generate_jsonb_agg(
                                        "value",
                                        &format!("select {column_name_and_maybe_field_getter_field_ident}"),
                                        "where ordinality",
                                        1,
                                    ),
                                    |array_dimension_select_pattern| {
                                        // Dimension1 does not fit into pattern. its only for 2+ dimensions
                                        let generate_d_number_elem = |content: usize| format!("d{content}_elem");
                                        let generate_d_number_ord = |content: usize| format!("d{content}_elem");
                                        let generate_dot_value = |content: &str| format!("{content}.value");
                                        let generate_as_value_where = |
                                            first_content: &str,
                                            second_content: &str
                                        | format!("as {first_content}(value, {second_content}) where {second_content}");
                                        let one = 1;
                                        generate_jsonb_agg(
                                            &{
                                                let mut current_usize_value = match &array_dimension_select_pattern {
                                                    ArrayDimensionSelectPattern::ArrayDimension2 { .. } => 2,
                                                    ArrayDimensionSelectPattern::ArrayDimension3 { .. } => 3,
                                                    ArrayDimensionSelectPattern::ArrayDimension4 { .. } => 4,
                                                };
                                                match &array_dimension_select_pattern {
                                                    ArrayDimensionSelectPattern::ArrayDimension2 {
                                                        dimension1_not_null_or_nullable,
                                                    } => vec![dimension1_not_null_or_nullable],
                                                    ArrayDimensionSelectPattern::ArrayDimension3 {
                                                        dimension1_not_null_or_nullable,
                                                        dimension2_not_null_or_nullable,
                                                    } => vec![
                                                        dimension2_not_null_or_nullable,
                                                        dimension1_not_null_or_nullable,
                                                    ],
                                                    ArrayDimensionSelectPattern::ArrayDimension4 {
                                                        dimension1_not_null_or_nullable,
                                                        dimension2_not_null_or_nullable,
                                                        dimension3_not_null_or_nullable,
                                                    } => vec![
                                                        dimension3_not_null_or_nullable,
                                                        dimension2_not_null_or_nullable,
                                                        dimension1_not_null_or_nullable,
                                                    ],
                                                }
                                                .into_iter()
                                                .fold(generate_dot_value(&generate_d_number_elem(current_usize_value)), |mut acc_64e08e3a, current_not_null_or_nullable| {
                                                    let current_usize_value_minus_one = current_usize_value.checked_sub(one).expect("a35e873e-a2a1-4a25-8de1-c35dbb0b65af");
                                                    let d_usize_minus_one_elem_value = generate_dot_value(&generate_d_number_elem(current_usize_value_minus_one));
                                                    let value = generate_jsonb_agg(
                                                        &acc_64e08e3a,
                                                        &d_usize_minus_one_elem_value,
                                                        &generate_as_value_where(&generate_d_number_elem(current_usize_value), &generate_d_number_ord(current_usize_value)),
                                                        current_usize_value,
                                                    );
                                                    acc_64e08e3a = match &current_not_null_or_nullable {
                                                        NotNullOrNullable::NotNull => value,
                                                        NotNullOrNullable::Nullable => {
                                                            format!("case when jsonb_typeof({d_usize_minus_one_elem_value})='array' then ({value}) else null end")
                                                        }
                                                    };
                                                    current_usize_value = current_usize_value_minus_one;
                                                    acc_64e08e3a
                                                })
                                            },
                                            &column_name_and_maybe_field_getter_field_ident,
                                            &generate_as_value_where(&generate_d_number_elem(one), &generate_d_number_ord(one)),
                                            one,
                                        )
                                    },
                                )
                            },
                        );
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => format_handle,
                            NotNullOrNullable::Nullable => format!("case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident})='null' then 'null'::jsonb else ({format_handle}) end"),
                        }
                    };
                    let maybe_dimensions_start_end_initialization = ArrayDimension::try_from(postgresql_json_type_pattern).ok().into_iter().flat_map(|array_dimension| {
                        (1..=array_dimension.to_usize()).map(|el_8d56d66d| {
                            let dimension_number_start_ts =
                                generate_dimension_number_start_stringified(el_8d56d66d)
                                    .parse::<proc_macro2::TokenStream>()
                                    .expect("77ec13b9-710a-460f-9239-ac9c3680244d");
                            let dimension_number_end_ts =
                                generate_dimension_number_end_stringified(el_8d56d66d)
                                    .parse::<proc_macro2::TokenStream>()
                                    .expect("24acbb5e-c0fe-4257-b299-8746887ce198");
                            let dimension_number_pagination_ts =
                                format!(
                                    "{}_pagination",
                                    generate_dimension_number_stringified(el_8d56d66d)
                                )
                                .parse::<proc_macro2::TokenStream>()
                                .expect("745c99b3-4e24-46c2-a671-9c7b4dce76f4");
                            quote! {
                                let #dimension_number_start_ts = #value_sc.#dimension_number_pagination_ts.start();
                                let #dimension_number_end_ts = #value_sc.#dimension_number_pagination_ts.end();
                            }
                        })
                    });
                    let format_handle_ts = generate_quotes::double_quotes_ts(&format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',({format_handle})))"));
                    quote! {
                        #(#maybe_dimensions_start_end_initialization)*
                        Ok(format!(#format_handle_ts))
                    }
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &ident_read_only_ids_ucc,
                &{
                    let content_ts = if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                        quote! {format!("jsonb_build_object('value',{column_name_and_maybe_field_getter})")}
                    } else {
                        quote! {"jsonb_build_object('value','null'::jsonb)".to_owned()}
                    };
                    quote! {Ok(#content_ts)}
                },
                &ident_read_inner_ucc,
                &{
                    let content_ts = quote! {#value_sc.0.0};
                    let generate_match_el_zero_ts = |
                        match_ts: &dyn quote::ToTokens,
                        value_ts: &dyn quote::ToTokens,
                        current_content_ts: &dyn quote::ToTokens
                    | {
                        quote! {#match_ts.map(|#value_ts| #value_ts.0 #current_content_ts)}
                    };
                    let generate_into_iter_map_el_collect_ts = |
                        el_ts: &dyn quote::ToTokens,
                        current_content_ts: &dyn quote::ToTokens
                    | {
                        quote! {.into_iter().map(|#el_ts|#current_content_ts).collect()}
                    };
                    let generate_into_iter_map_el_collect_not_null_or_nullable_ts = |
                        el_ts: &dyn quote::ToTokens,
                        current_not_null_or_nullable: &NotNullOrNullable
                    | {
                        generate_into_iter_map_el_collect_ts(
                            &el_ts,
                            &match &current_not_null_or_nullable {
                                NotNullOrNullable::NotNull => quote! {#el_ts.0},
                                NotNullOrNullable::Nullable => generate_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &quote! {value_f8b0b01d},
                                    &proc_macro2::TokenStream::new()
                                )
                            }
                        )
                    };
                    let generate_into_iter_map_el_collect_not_null_or_nullable_content_ts = |
                        el_ts: &dyn quote::ToTokens,
                        value_ts: &dyn quote::ToTokens,
                        current_not_null_or_nullable: &NotNullOrNullable,
                        current_content_ts: &dyn quote::ToTokens
                    | {
                        match &current_not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_into_iter_map_el_collect_ts(
                                &el_ts,
                                &quote! {#el_ts.0 #current_content_ts}
                            ),
                            NotNullOrNullable::Nullable => {
                                let match_el_zero_ts = generate_match_el_zero_ts(
                                    &quote! {#el_ts.0},
                                    &value_ts,
                                    &current_content_ts
                                );
                                quote! {.into_iter().map(|#el_ts|#match_el_zero_ts).collect()}
                            }
                        }
                    };
                    let into_inner_content_ts = match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => proc_macro2::TokenStream::new(),
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_into_iter_map_el_collect_not_null_or_nullable_ts(
                            &quote!{el_0fdb74a5},
                            dimension1_not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                            let dimension2_not_null_or_nullable_content_ts = generate_into_iter_map_el_collect_not_null_or_nullable_ts(
                                &quote!{el_dac5ba56},
                                dimension2_not_null_or_nullable
                            );
                            generate_into_iter_map_el_collect_not_null_or_nullable_content_ts(
                                &quote!{el_cf5646e9},
                                &quote!{value_1c90c80c},
                                dimension1_not_null_or_nullable,
                                &dimension2_not_null_or_nullable_content_ts
                            )
                        }
                        PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                        } => {
                            let dimension3_not_null_or_nullable_content_ts = generate_into_iter_map_el_collect_not_null_or_nullable_ts(
                                &quote!{el_c935a865},
                                dimension3_not_null_or_nullable
                            );
                            let dimension2_not_null_or_nullable_content_ts = generate_into_iter_map_el_collect_not_null_or_nullable_content_ts(
                                &quote!{el_dc9e788b},
                                &quote!{value_3d1307e8},
                                dimension2_not_null_or_nullable,
                                &dimension3_not_null_or_nullable_content_ts
                            );
                            generate_into_iter_map_el_collect_not_null_or_nullable_content_ts(
                                &quote!{el_bf67606b},
                                &quote!{value_721e4164},
                                dimension1_not_null_or_nullable,
                                &dimension2_not_null_or_nullable_content_ts
                            )
                        }
                        PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                            dimension4_not_null_or_nullable,
                        } => {
                            let dimension4_not_null_or_nullable_content_ts = generate_into_iter_map_el_collect_not_null_or_nullable_ts(
                                &quote!{el_144c60e6},
                                dimension4_not_null_or_nullable
                            );
                            let dimension3_not_null_or_nullable_content_ts = generate_into_iter_map_el_collect_not_null_or_nullable_content_ts(
                                &quote!{el_98961cb7},
                                &quote!{value_995a5fbe},
                                dimension3_not_null_or_nullable,
                                &dimension4_not_null_or_nullable_content_ts
                            );
                            let dimension2_not_null_or_nullable_content_ts = generate_into_iter_map_el_collect_not_null_or_nullable_content_ts(
                                &quote!{el_34e95172},
                                &quote!{value_75b18ade},
                                dimension2_not_null_or_nullable,
                                &dimension3_not_null_or_nullable_content_ts
                            );
                            generate_into_iter_map_el_collect_not_null_or_nullable_content_ts(
                                &quote!{el_f64124e2},
                                &quote!{value_7fc1b146},
                                dimension1_not_null_or_nullable,
                                &dimension2_not_null_or_nullable_content_ts
                            )
                        }
                    };
                    match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {#content_ts #into_inner_content_ts},
                        NotNullOrNullable::Nullable => generate_match_el_zero_ts(
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
                    let format_handle_ts = generate_quotes::double_quotes_ts(&format!("jsonb_set({{{jsonb_set_accumulator_sc}}},'{{{{{{jsonb_set_path}}}}}}',${{value_26526e0f}})"));
                    quote! {
                        match #import_path::increment_checked_add_one_returning_increment(#increment_sc) {
                            Ok(value_26526e0f) => Ok(format!(#format_handle_ts)),
                            Err(#error_sc) => Err(#error_sc),
                        }
                    }
                },
                &postgresql_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::False,
                &postgresql_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::False,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                &quote! {
                    if let Err(error) = query.try_bind(#value_sc) {
                        return Err(error.to_string());
                    }
                    Ok(query)
                },
                &select_only_created_or_updated_ids_query_part_ts,
                &if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                    postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True
                } else {
                    postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False
                },
                &select_only_created_or_updated_ids_query_bind_ts,
                &select_only_created_or_updated_ids_query_part_ts,
                &if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                    postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True
                } else {
                    postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::False
                },
                &select_only_created_or_updated_ids_query_bind_ts,
            )
        };
        let maybe_impl_postgresql_json_type_object_vec_el_id_for_ident_origin_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
            let (query_bind_string_as_postgresql_text_create_for_query_ts, query_bind_string_as_postgresql_text_update_for_query_ts) = {
                enum CreateForQueryOrUpdateForQuery {
                    CreateForQuery,
                    UpdateForQuery,
                }
                let generate_query_bind_string_as_postgresql_text_ts = |create_for_query_or_update_for_query: &CreateForQueryOrUpdateForQuery| {
                    let name_ts = format!(
                        "query_bind_string_as_postgresql_text_{}_for_query",
                        match &create_for_query_or_update_for_query {
                            CreateForQueryOrUpdateForQuery::CreateForQuery => "create",
                            CreateForQueryOrUpdateForQuery::UpdateForQuery => "update",
                        }
                    )
                    .parse::<proc_macro2::TokenStream>()
                    .expect("f1bcde08-085f-4c98-9a1e-1fb583c9fb6e");
                    let type_ts: &dyn quote::ToTokens = match &create_for_query_or_update_for_query {
                        CreateForQueryOrUpdateForQuery::CreateForQuery => &create_for_query_ucc,
                        CreateForQueryOrUpdateForQuery::UpdateForQuery => &update_for_query_ucc,
                    };
                    quote! {
                        fn #name_ts(
                            #value_sc: <Self::PostgresqlJsonType as #import_path::PostgresqlJsonType>::#type_ts,
                            mut #query_sc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                        ) -> Result<
                            sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                            #std_string_string_ts
                        > {
                            if let Err(#error_sc) = #query_sc.try_bind(#value_sc.0.0.to_string()) {
                                return Err(#error_sc.to_string())
                            }
                            Ok(#query_sc)
                        }
                    }
                };
                (generate_query_bind_string_as_postgresql_text_ts(&CreateForQueryOrUpdateForQuery::CreateForQuery), generate_query_bind_string_as_postgresql_text_ts(&CreateForQueryOrUpdateForQuery::UpdateForQuery))
            };
            quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                impl #import_path::PostgresqlJsonTypeObjectVecElementId for #ident {
                    type PostgresqlJsonType = Self;
                    type #create_for_query_ucc = #ident_create_for_query_ucc;
                    type #update_ucc = #ident_update_ucc;
                    type #read_inner_ucc = #ident_read_inner_ucc;
                    #query_bind_string_as_postgresql_text_create_for_query_ts
                    #query_bind_string_as_postgresql_text_update_for_query_ts
                    fn get_inner(#value_sc: &<Self::PostgresqlJsonType as #import_path::PostgresqlJsonType>::#create_for_query_ucc) -> &Self::#read_inner_ucc {
                        &#value_sc.0.0
                    }
                    fn increment_checked_add_one(#increment_sc: &mut #std_primitive_u64_ts) -> Result<#std_primitive_u64_ts, #import_path::QueryPartErrorNamed> {
                        #import_path::increment_checked_add_one_returning_increment(#increment_sc)
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let impl_postgresql_json_type_test_cases_for_ident_ts = {
            enum F32OrF64 {
                F32,
                F64
            }
            let generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts = |read_or_update: &postgresql_crud_macros_common::ReadOrUpdate| {
                let read_or_update_ucc = read_or_update.ucc();
                quote! {<#self_ucc::#postgresql_json_type_ucc
                    as
                    #postgresql_crud_macros_common_import_path_postgresql_crud_common::#postgresql_json_type_ucc
                >::#read_or_update_ucc::#new_sc(#value_sc)}
            };
            let standart_not_null_test_cases_vec_name_ts = match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => quote! {std_primitive_i8_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => quote! {std_primitive_i16_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => quote! {std_primitive_i32_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => quote! {std_primitive_i64_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => quote! {std_primitive_u8_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => quote! {std_primitive_u16_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => quote! {std_primitive_u32_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => quote! {std_primitive_u64_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => quote! {std_primitive_f32_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => quote! {std_primitive_f64_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote! {std_primitive_bool_test_cases_vec},
                PostgresqlJsonType::StdStringStringAsJsonbString => quote! {std_string_string_test_cases_vec},
                PostgresqlJsonType::UuidUuidAsJsonbString => quote! {uuid_uuid_test_cases_vec},
            };
            let generate_array_dimension_equal_ts = |dimension: &postgresql_crud_macros_common::Dimension| {
                let dimension_index_number_max = postgresql_crud_macros_common::DimensionIndexNumber::from(dimension);
                let generate_dimension_index_number_ts = |not_null_or_nullable_vec: &[&NotNullOrNullable]|{
                    assert!(!not_null_or_nullable_vec.is_empty(), "c1a5939d-3235-4bcd-88fc-bfdf2101dffd");
                    let content_ts_c85923bd = {
                        let generate_index_number_ts = |index_c1128a3e: usize|format!("index_{index_c1128a3e}").parse::<proc_macro2::TokenStream>().expect("afbe7252-745f-40ad-9bf4-1bb20377b5a5");
                        let generate_value_number_ts = |index_0abe6039: usize|format!("value{index_0abe6039}").parse::<proc_macro2::TokenStream>().expect("568d8eb6-df23-4f57-afdd-ef392e3b7f72");
                        let generate_for_in_ts = |
                            index_ts: &dyn quote::ToTokens,
                            value_ts: &dyn quote::ToTokens,
                            enumerate_ts: &dyn quote::ToTokens,
                            content_ts_aaf03124: &dyn quote::ToTokens,
                        |quote!{
                            for (#index_ts, #value_ts) in #enumerate_ts.0.into_iter().enumerate() {
                                #content_ts_aaf03124
                            }
                        };
                        let generate_for_value_index_dot_zero_into_iter_enumerate_ts = |
                            index_0082bcdf: usize,
                            index_e81c6d28: usize,
                            index_b7b230b2: usize,
                            content_ts_d575a40c: &dyn quote::ToTokens,
                        |generate_for_in_ts(
                            &generate_index_number_ts(index_0082bcdf),
                            &generate_value_number_ts(index_e81c6d28),
                            &generate_value_number_ts(index_b7b230b2),
                            &content_ts_d575a40c
                        );
                        let generate_if_let_some_ts = |
                            some_ts: &dyn quote::ToTokens,
                            equal_ts: &dyn quote::ToTokens,
                            content_ts_9292e3cf: &dyn quote::ToTokens,
                        |quote!{
                            if let Some(#some_ts) = #equal_ts.0 {
                                #content_ts_9292e3cf
                            }
                        };
                        let generate_if_let_some_equals_value_index_dot_zero_ts = |
                            index_c4552aef: usize,
                            index_9f1fbc9f: usize,
                            content_ts_832b20d5: &dyn quote::ToTokens,
                        |generate_if_let_some_ts(
                            &generate_value_number_ts(index_c4552aef),
                            &generate_value_number_ts(index_9f1fbc9f),
                            &content_ts_832b20d5
                        );
                        let generate_index = |start_index: usize, not_null_or_nullable_vec_41b82a0c: &[&NotNullOrNullable]| -> usize {
                            start_index.checked_add(
                                not_null_or_nullable_vec_41b82a0c
                                .iter()
                                .filter(|el_bf28b242| matches!(el_bf28b242, NotNullOrNullable::Nullable))
                                .count()
                            ).expect("de4c4116-b645-4a8f-b097-1d7772aecc19")
                        };
                        let mut content_ts_4c106eea = {
                            let content_ts_f1ffd3b2 = {
                                let value_index_ts = generate_value_number_ts(
                                    generate_index(
                                        not_null_or_nullable_vec.len().saturating_sub(1),
                                        &once(not_null_or_nullable)
                                        .chain(
                                            not_null_or_nullable_vec
                                                .iter()
                                                .take(not_null_or_nullable_vec.len().saturating_sub(1))
                                                .copied(),
                                        ).collect::<Vec<&NotNullOrNullable>>()
                                    )
                                );
                                let to_number_starting_with_one_word_str = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber| match dimension_index_number {
                                    postgresql_crud_macros_common::DimensionIndexNumber::Zero => "One",
                                    postgresql_crud_macros_common::DimensionIndexNumber::One => "Two",
                                    postgresql_crud_macros_common::DimensionIndexNumber::Two => "Three",
                                    postgresql_crud_macros_common::DimensionIndexNumber::Three => "Four",
                                };
                                let dimension_number_starting_with_one_equal_ts = format!("Dimension{}Equal", to_number_starting_with_one_word_str(&dimension_index_number_max)).parse::<proc_macro2::TokenStream>().expect("52fa34ac-5cd1-4ae9-8a1d-832e73a505d7");
                                let postgresql_json_type_where_dimension_number_starting_with_one_equal_ts = format!("PostgresqlJsonTypeWhereDimension{}Equal", to_number_starting_with_one_word_str(&dimension_index_number_max)).parse::<proc_macro2::TokenStream>().expect("15d769b0-0767-473c-a2c5-3d0f6e221ced");
                                let current_where_ident_where_ucc = SelfWhereUcc::from_tokens(&generate_ident_ts(&NotNullOrNullable::NotNull, postgresql_json_type_pattern));
                                let current_value_ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&generate_ident_ts(
                                    not_null_or_nullable_vec.last().expect("1221f6ec-8865-4456-bd18-ebeff15439f6"),
                                    &match dimension_index_number_max {
                                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => postgresql_json_type_pattern.down_by_1().expect("1a47af86-470b-41dd-aee1-01dfccef56a1"),
                                        postgresql_crud_macros_common::DimensionIndexNumber::One => postgresql_json_type_pattern.down_by_2().expect("d8260225-71af-4ea5-a354-075432088e96"),
                                        postgresql_crud_macros_common::DimensionIndexNumber::Two => postgresql_json_type_pattern.down_by_3().expect("473ac422-6c8c-417f-a115-8a7c0743ca08"),
                                        postgresql_crud_macros_common::DimensionIndexNumber::Three => postgresql_json_type_pattern.down_by_4().expect("6a143218-a98e-4893-ad2b-ed028a20ef39"),
                                    }
                                ));
                                let vec_content_ts = {
                                    let content_ts_0dc5a500 = (
                                        0i32..=match dimension_index_number_max {
                                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => 0i32,
                                            postgresql_crud_macros_common::DimensionIndexNumber::One => 1i32,
                                            postgresql_crud_macros_common::DimensionIndexNumber::Two => 2i32,
                                            postgresql_crud_macros_common::DimensionIndexNumber::Three => 3i32,
                                        }
                                    )
                                    .map(|el_db559599| {
                                        let index_number_ts = format!("index_{el_db559599}")
                                            .parse::<proc_macro2::TokenStream>()
                                            .expect("f0ce7e73-6d15-4de8-8f15-ce00334ed410");
                                        quote! {
                                            postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                                i32::try_from(#index_number_ts)
                                                    .expect("5a1818e7-3865-4222-bf6b-31486bd721d2")
                                            ).expect("ad1ab73f-fd3b-4162-adb0-bb09a19d31a0")
                                        }
                                    }).collect::<Vec<proc_macro2::TokenStream>>();
                                    quote! {#(#content_ts_0dc5a500),*}
                                };
                                quote! {
                                    #current_where_ident_where_ucc::#dimension_number_starting_with_one_equal_ts(
                                        where_filters::#postgresql_json_type_where_dimension_number_starting_with_one_equal_ts {
                                            logical_operator: #import_path::LogicalOperator::And,
                                            dimensions: where_filters::BoundedStdVecVec::try_from(
                                                vec![#vec_content_ts]
                                            ).expect("82cc0a3c-3e8d-47c4-b317-2795362a9b37"),
                                            #value_sc: #current_value_ident_table_type_declaration_ucc::new(#value_index_ts.into()),
                                        }
                                    )
                                }
                            };
                            match not_null_or_nullable {
                                NotNullOrNullable::NotNull => quote! {acc_049ff0b3.push(#content_ts_f1ffd3b2);},
                                NotNullOrNullable::Nullable => quote! {
                                    match #import_path::NotEmptyUniqueVec::try_new(vec![#content_ts_f1ffd3b2]) {
                                        Ok(value_9328b66f) => {
                                            acc_049ff0b3.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_9328b66f)));
                                        },
                                        Err(error) => match error {
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                            #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("2f5f648a-4dc6-4699-8656-33870b2c629f")
                                        }
                                    }
                                },
                            }
                        };
                        for (index_ef936914, _) in not_null_or_nullable_vec.iter().take(not_null_or_nullable_vec.len().saturating_sub(1)).enumerate() {
                            let not_null_or_nullable_vec_e7e7f6f8 = not_null_or_nullable_vec
                            .iter()
                            .take(
                                not_null_or_nullable_vec
                                    .len()
                                    .saturating_sub(index_ef936914.checked_add(1).expect("75d5ed28-131b-4387-a064-8c77841894fd")),
                            )
                            .copied()
                            .collect::<Vec<&NotNullOrNullable>>();
                            let not_null_or_nullable_vec_e7e7f6f8_len = not_null_or_nullable_vec_e7e7f6f8.len();
                            let not_null_or_nullable_vec_e7e7f6f8_len_saturating_sub_one = not_null_or_nullable_vec_e7e7f6f8_len.saturating_sub(1);
                            content_ts_4c106eea = {
                                let index_74ae6d77 = generate_index(
                                    not_null_or_nullable_vec_e7e7f6f8_len_saturating_sub_one,
                                    &once(not_null_or_nullable)
                                    .chain(
                                        not_null_or_nullable_vec_e7e7f6f8
                                            .iter()
                                            .take(not_null_or_nullable_vec_e7e7f6f8_len_saturating_sub_one)
                                            .copied(),
                                    ).collect::<Vec<&NotNullOrNullable>>()
                                );
                                let index_74ae6d77_increment_by_1 = index_74ae6d77.checked_add(1).expect("96e90e72-bf43-4c6c-8ab6-b496953e88ec");
                                match &not_null_or_nullable_vec_e7e7f6f8.last().expect("88548240-8588-4ee8-b166-5dacfd997088") {
                                    NotNullOrNullable::NotNull => generate_for_value_index_dot_zero_into_iter_enumerate_ts(
                                        not_null_or_nullable_vec_e7e7f6f8_len,
                                        index_74ae6d77_increment_by_1,
                                        index_74ae6d77,
                                        &content_ts_4c106eea,
                                    ),
                                    NotNullOrNullable::Nullable => generate_if_let_some_equals_value_index_dot_zero_ts(
                                        index_74ae6d77_increment_by_1,
                                        index_74ae6d77,
                                        &generate_for_value_index_dot_zero_into_iter_enumerate_ts(
                                            not_null_or_nullable_vec_e7e7f6f8_len,
                                            index_74ae6d77.checked_add(2).expect("00da046c-1486-4de1-990b-258b2cd90e2c"),
                                            index_74ae6d77_increment_by_1,
                                            &content_ts_4c106eea,
                                        )
                                    )
                                }
                            };
                        }
                        let create_dot_zero_ts = quote!{create.0};
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_for_in_ts(
                                &generate_index_number_ts(0),
                                &generate_value_number_ts(0),
                                &create_dot_zero_ts,
                                &content_ts_4c106eea
                            ),
                            NotNullOrNullable::Nullable => generate_if_let_some_ts(
                                &generate_value_number_ts(0),
                                &create_dot_zero_ts,
                                &generate_for_value_index_dot_zero_into_iter_enumerate_ts(
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
                        }).expect("e99ecd08-0aec-4a25-931d-163319bb8179"))
                    }
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => none_ts.clone(),
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match dimension_index_number_max {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_ts(&[
                            dimension1_not_null_or_nullable,
                        ]),
                        postgresql_crud_macros_common::DimensionIndexNumber::One | postgresql_crud_macros_common::DimensionIndexNumber::Two | postgresql_crud_macros_common::DimensionIndexNumber::Three => none_ts.clone(),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => match dimension_index_number_max {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_ts(&[
                            dimension1_not_null_or_nullable,
                        ]),
                        postgresql_crud_macros_common::DimensionIndexNumber::One => generate_dimension_index_number_ts(&[
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable
                        ]),
                        postgresql_crud_macros_common::DimensionIndexNumber::Two | postgresql_crud_macros_common::DimensionIndexNumber::Three => none_ts.clone(),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                    } => match dimension_index_number_max {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_ts(&[
                            dimension1_not_null_or_nullable,
                        ]),
                        postgresql_crud_macros_common::DimensionIndexNumber::One => generate_dimension_index_number_ts(&[
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                        ]),
                        postgresql_crud_macros_common::DimensionIndexNumber::Two => generate_dimension_index_number_ts(&[
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable
                        ]),
                        postgresql_crud_macros_common::DimensionIndexNumber::Three => none_ts.clone(),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                        dimension4_not_null_or_nullable,
                    } => {
                        match dimension_index_number_max {
                            postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_ts(&[
                                dimension1_not_null_or_nullable
                            ]),
                            postgresql_crud_macros_common::DimensionIndexNumber::One => generate_dimension_index_number_ts(&[
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                            ]),
                            postgresql_crud_macros_common::DimensionIndexNumber::Two => generate_dimension_index_number_ts(&[
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            ]),
                            postgresql_crud_macros_common::DimensionIndexNumber::Three => generate_dimension_index_number_ts(&[
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable
                            ])
                        }
                    }
                }
            };
            let option_vec_create_ts = {
                let generate_some_acc_content_ts = |current_not_null_or_nullable: &NotNullOrNullable, current_ident_ts: &dyn quote::ToTokens| {
                    let (new_content_ts, maybe_acc_push_none_ts) = match &current_not_null_or_nullable {
                        NotNullOrNullable::NotNull => (quote! {vec![el_88131059.0.into()]}, proc_macro2::TokenStream::new()),
                        NotNullOrNullable::Nullable => (quote! {Some(el_88131059.0.into())}, quote! {acc_50e99088.push(<Self as #import_path::PostgresqlJsonType>::Create::new(None));}),
                    };
                    //todo check - maybe need to add something here
                    let maybe_acc_push_long_vec_ts = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {
                            let mut acc_27624e5e = Vec::new();
                            for el_0dcb405a in value_8de026a4 {
                                acc_27624e5e.push(el_0dcb405a.0.into());
                            }
                            if !acc_27624e5e.is_empty() {
                                acc_50e99088.push(<Self as #import_path::PostgresqlJsonType>::Create::new(acc_27624e5e));
                            }
                        },
                        NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    };
                    let maybe_dot_clone_ts = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote!{.clone()},
                        NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    };
                    // let maybe_dot_clone_ts = match &postgresql_json_type_pattern {
                    //     PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                    //         NotNullOrNullable::NotNull => match &postgresql_json_type {
                    //             | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => proc_macro2::TokenStream::new(),
                    //             PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    //             | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                    //             | PostgresqlJsonType::StdStringStringAsJsonbString
                    //             | PostgresqlJsonType::UuidUuidAsJsonbString => quote!{.clone()},
                    //         }
                    //         NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    //     },
                    //     PostgresqlJsonTypePattern::ArrayDimension1 { .. } |
                    //     PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
                    //     PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                    //     PostgresqlJsonTypePattern::ArrayDimension4 { .. } => quote!{.clone()},
                    // };
                    quote! {Some({
                        let mut acc_50e99088 = Vec::new();
                        if let Some(value_8de026a4) = <#current_ident_ts as #import_path::PostgresqlJsonTypeTestCases>::#option_vec_create_sc() {
                            for el_88131059 in value_8de026a4 #maybe_dot_clone_ts {
                                acc_50e99088.push(<Self as #import_path::PostgresqlJsonType>::Create::new(#new_content_ts));
                            }
                            #maybe_acc_push_long_vec_ts
                        }
                        #maybe_acc_push_none_ts
                        acc_50e99088
                    })}
                };
                let content_ts = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {
                            Some(
                                #import_path::#standart_not_null_test_cases_vec_name_ts().into_iter().map(<Self as #import_path::PostgresqlJsonType>::Create::new).collect()
                            )
                        },
                        NotNullOrNullable::Nullable => generate_some_acc_content_ts(not_null_or_nullable, &generate_ident_ts(&NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart)),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_some_acc_content_ts(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("dec468c0-09fd-4db3-98e7-7fa9cd565909")),
                            NotNullOrNullable::Nullable => generate_ident_ts(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_ts(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("4010ebf7-d6e2-4d6e-a562-a299201c92ec")),
                            NotNullOrNullable::Nullable => generate_ident_ts(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_ts(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("acdbb564-b169-40db-9815-2653c0150a4c")),
                            NotNullOrNullable::Nullable => generate_ident_ts(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_ts(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("5abf9504-cde0-4c6c-adb9-145b385918a5")),
                            NotNullOrNullable::Nullable => generate_ident_ts(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                };
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                    | PostgresqlJsonType::StdStringStringAsJsonbString => content_ts,
                    PostgresqlJsonType::UuidUuidAsJsonbString => quote! {None},
                }
            };
            let read_only_ids_to_two_dimensional_vec_read_inner_ts = {
                let (has_len_greater_than_one_ts, has_len_greater_than_one_for_for_ts) = {
                    let generate_ts = |content_ts: &dyn quote::ToTokens| {
                        quote! {let has_len_greater_than_one = #content_ts;}
                    };
                    (
                        generate_ts(&quote! {read_only_ids_to_two_dimensional_vec_read_inner.len() > 1}),
                        generate_ts(&quote! {{
                            let mut has_len_greater_than_one = false;
                            for el_4a00ab02 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                if el_4a00ab02.len() > 1 {
                                    has_len_greater_than_one = true;
                                    break;
                                }
                            }
                            has_len_greater_than_one
                        }}),
                    )
                };
                let generate_acc_content_handle_ts = |current_ident_ts: &dyn quote::ToTokens, has_len_greater_than_one_content_ts: &dyn quote::ToTokens| {
                    let current_ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&current_ident_ts);
                    let option_additional_content_ts = {
                        let el_82c7dc0a_clone_ts = quote! {el_82c7dc0a.clone()};
                        let first = quote! {vec![#el_82c7dc0a_clone_ts]};
                        let second = quote! {vec![#el_82c7dc0a_clone_ts, #el_82c7dc0a_clone_ts]};
                        let (first_content_ts, second_content_ts) = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => (first, second),
                            NotNullOrNullable::Nullable => {
                                let generate_ts = |content_ts: &dyn quote::ToTokens| quote! {Some(#content_ts)};
                                (generate_ts(&first), generate_ts(&second))
                            }
                        };
                        quote! {
                            let option_additional = {
                                let mut option_additional = None;
                                for el_c4f9bf8f in &read_only_ids_to_two_dimensional_vec_read_inner {
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
                                for el_640f58e8 in read_only_ids_to_two_dimensional_vec_read_inner {
                                    for el_d251d1f6 in el_640f58e8 {
                                        acc_6cd5b60a.push(el_d251d1f6);
                                    }
                                }
                                acc_6cd5b60a
                            }};
                            match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => inner_content_ts,
                                NotNullOrNullable::Nullable => quote! {Some(#inner_content_ts)},
                            }
                        };
                        quote! {acc_0a07db18.push(vec![#content_ts]);}
                    };
                    let maybe_acc_push_vec_none_ts = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => proc_macro2::TokenStream::new(),
                        NotNullOrNullable::Nullable => quote! {acc_0a07db18.push(vec![None]);},
                    };
                    quote! {
                        let mut acc_0a07db18 = Vec::new();
                        let read_only_ids_to_two_dimensional_vec_read_inner = <
                            #current_ident_ts
                            as
                            #import_path::PostgresqlJsonTypeTestCases
                        >::#read_only_ids_to_two_dimensional_vec_read_inner_sc(
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
                let content_ts = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote! {vec![#import_path::#standart_not_null_test_cases_vec_name_ts().into()]},
                        NotNullOrNullable::Nullable => quote! {
                            let mut acc_97242d4d = Vec::new();
                            for el_8f3646f9 in <#ident_standart_not_null_ucc as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_to_two_dimensional_vec_read_inner_sc(&#ident_read_only_ids_standart_not_null_ucc(read_only_ids.0.clone())) {
                                for el_35a4dba9 in el_8f3646f9 {
                                    acc_97242d4d.push(vec![Some(el_35a4dba9)]);
                                }
                            }
                            acc_97242d4d.push(vec![None]);
                            acc_97242d4d
                        },
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_acc_content_handle_ts(
                        &generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("d6f89137-9a47-4f74-afce-0e1959d3dc59")),
                        &match &dimension1_not_null_or_nullable {
                            NotNullOrNullable::NotNull => &has_len_greater_than_one_for_for_ts,
                            NotNullOrNullable::Nullable => &has_len_greater_than_one_ts,
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_ts(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("38774398-d485-4c14-84e8-92e06c36c23b")), &has_len_greater_than_one_ts),
                    PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_ts(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("053f4bab-0a8e-457f-9176-50e519b312bb")), &has_len_greater_than_one_ts),
                    PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_ts(&generate_ident_ts(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("860f8f15-72ac-4557-a2c6-87b1aa958eb4")), &has_len_greater_than_one_ts),
                };
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                    | PostgresqlJsonType::StdStringStringAsJsonbString => content_ts,
                    PostgresqlJsonType::UuidUuidAsJsonbString => quote! {Vec::new()},
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_ts = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&postgresql_crud_macros_common::ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_ts = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_ts(&postgresql_crud_macros_common::ReadOrUpdate::Update);
            let read_only_ids_into_option_value_read_inner_ts = {
                let content_ts = generate_import_path_value_initialization_ts(&if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#value_sc.0.#value_sc}
                } else {
                    quote! {
                        <Self as #import_path::PostgresqlJsonType>::into_inner(
                            <
                                <Self as #import_path::PostgresqlJsonType>::Read
                                as
                                #postgresql_crud_common_default_option_some_vec_one_el_ts
                            >::default_option_some_vec_one_el()
                        )
                    }
                });
                quote! {Some(#content_ts)}
            };
            let update_to_read_only_ids_ts = {
                let value_initialization_ts = generate_import_path_value_initialization_ts(&if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                    let generate_iter_or_match_ts = |
                        current_not_null_or_nullable: &NotNullOrNullable,
                        current_ident_ts: &dyn quote::ToTokens,
                        update_current_not_null_or_nullable: &NotNullOrNullable
                    | {
                        let value_zero_zero_ts = quote! {#value_sc.0.0};
                        let content_ts = {
                            let current_ident_update_ts = SelfUpdateUcc::from_tokens(&current_ident_ts);
                            let content_ts = {
                                let content_ts = match &update_current_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote! {el_aa999306.clone()},
                                    NotNullOrNullable::Nullable => quote! {value_92de91cc.clone()},
                                };
                                quote! {#current_ident_update_ts(#content_ts)}
                            };
                            quote! {
                                <
                                    #current_ident_ts
                                    as
                                    #import_path::PostgresqlJsonTypeTestCases
                                >::update_to_read_only_ids(&#content_ts).0.#value_sc
                            }
                        };
                        match &current_not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote! {
                                #value_zero_zero_ts.iter().map(|el_aa999306|#content_ts).collect()
                            },
                            NotNullOrNullable::Nullable => quote! {
                                #value_zero_zero_ts.as_ref().map(|value_92de91cc| #content_ts)
                            },
                        }
                    };
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote! {#value_sc.0.clone().into()},
                            NotNullOrNullable::Nullable => generate_iter_or_match_ts(
                                not_null_or_nullable,
                                &ident_not_null_ts,
                                not_null_or_nullable
                            ),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_iter_or_match_ts(
                            not_null_or_nullable,
                            &generate_ident_ts(
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("e84064c3-5c31-4fa6-8dbc-ba454128c9da"),
                                    NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_ts(
                            not_null_or_nullable,
                            &generate_ident_ts(
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("226c6318-6be3-4b85-b2cd-c0b53a2d6bf9"),
                                    NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_ts(
                            not_null_or_nullable,
                            &generate_ident_ts(
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("3ae1e9f8-84ec-4369-a633-eca188d9b10a"),
                                    NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_ts(
                            not_null_or_nullable,
                            &generate_ident_ts(
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    NotNullOrNullable::Nullable => NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("44d51dc5-1b15-4807-ad63-c4fcfb01251c"),
                                    NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                    }
                } else {
                    none_ts.clone()
                });
                quote! {#ident_read_only_ids_ucc(#value_initialization_ts)}
            };
            let read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts = {
                let value_initialization_ts = generate_import_path_value_initialization_ts(&if matches!(&postgresql_json_type, PostgresqlJsonType::UuidUuidAsJsonbString) {
                    quote! {#ident_read_ucc::new(#value_sc.0.#value_sc.clone())}
                } else {
                    quote! {#postgresql_crud_common_default_option_some_vec_one_el_call_ts}
                });
                quote! {Some(#value_initialization_ts)}
            };
            let previous_read_merged_with_option_update_into_read_ts = quote! {
                #option_update_sc.map_or(#read_sc, |value_f6e37412| #ident_read_ucc(value_f6e37412.into()))
            };
            let read_only_ids_merged_with_create_into_read_ts = {
                let content_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ident_origin_ucc::new(#read_only_ids_sc.0.#value_sc)}
                } else {
                    quote! {#create_sc.into()}
                };
                quote! {#ident_read_ucc(#content_ts)}
            };
            let read_only_ids_merged_with_create_into_option_value_read_ts = {
                let value_initialization_ts = generate_import_path_value_initialization_ts(&quote! {
                    <Self as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_read_sc(
                        #read_only_ids_sc,
                        #create_sc
                    )
                });
                quote! {Some(#value_initialization_ts)}
            };
            let read_only_ids_merged_with_create_into_table_type_declaration_ts = {
                let content_ts = if matches!(&is_standart_not_null_uuid, IsStandartNotNullUuid::True) {
                    quote! {#ident_origin_ucc::new(#read_only_ids_sc.0.#value_sc)}
                } else {
                    quote! {#create_sc.into()}
                };
                quote! {#ident_table_type_declaration_ucc(#content_ts)}
            };
            let read_only_ids_merged_with_create_into_where_equal_ts = {
                let generate_equal_ts = |content_ts: &dyn quote::ToTokens| {
                    quote! {
                        where_filters::PostgresqlJsonTypeWhereEqual {
                            logical_operator: #import_path::LogicalOperator::Or,
                            #value_sc: #content_ts
                        }
                    }
                };
                match &not_null_or_nullable {
                    NotNullOrNullable::NotNull => {
                        let equal_ts = generate_equal_ts(&quote! {#ident_table_type_declaration_ucc::new(#create_sc.0.into())});
                        quote! {#ident_where_ucc::#equal_ucc(#equal_ts)}
                    }
                    NotNullOrNullable::Nullable => {
                        let current_ident_where_ucc = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                        let current_ident_table_type_declaration_ucc = SelfTableTypeDeclarationUcc::from_tokens(&ident_not_null_ts);
                        let equal_ts = generate_equal_ts(&quote! {#current_ident_table_type_declaration_ucc::new(value_18544acf.into())});
                        quote! {
                            #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                #create_sc.0.0.map(|value_18544acf| postgresql_crud_common::NotEmptyUniqueVec::try_new(
                                    vec![#current_ident_where_ucc::#equal_ucc(#equal_ts)]
                                ).expect("88bfa095-a3ab-4d0c-be71-af63c3acd50f"))
                            )
                        }
                    }
                }
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts = quote! {
                #import_path::NotEmptyUniqueVec::try_new(vec![
                    <Self as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_where_equal_sc(
                        #read_only_ids_sc,
                        #create_sc
                    )
                ]).expect("56eb9ad4-8f4f-4833-9618-7327f42b0014")
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts = quote! {<Self as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
                #read_only_ids_sc,
                #create_sc
            )};
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts = generate_array_dimension_equal_ts(&postgresql_crud_macros_common::Dimension::One);
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts = generate_array_dimension_equal_ts(&postgresql_crud_macros_common::Dimension::Two);
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts = generate_array_dimension_equal_ts(&postgresql_crud_macros_common::Dimension::Three);
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts = generate_array_dimension_equal_ts(&postgresql_crud_macros_common::Dimension::Four);
            //todo maybe reuse LengthEqual and LengthGreaterThan
            let create_into_postgresql_json_type_option_vec_where_length_equal_ts = {
                let generate_ts = || {
                    let content_ts = {
                        let create_dot_zero_dot_zero = quote! {#create_sc.0.0};
                        let content_ts = {
                            let content_ts: &dyn quote::ToTokens = match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => &create_dot_zero_dot_zero,
                                NotNullOrNullable::Nullable => &quote! {value_1bbf74bc.0},
                            };
                            quote! {
                                ::LengthEqual(
                                    where_filters::PostgresqlJsonTypeWhereLengthEqual {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #value_sc: postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                            i32::try_from(#content_ts.len()).expect("64d3424f-86fb-4b44-a437-75aea9997f47")
                                        ).expect("081f4463-0430-4901-8a76-83afcfb3f856"),
                                    }
                                )
                            }
                        };
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote! {#ident_where_ucc #content_ts},
                            NotNullOrNullable::Nullable => {
                                let current_ident_where_ucc = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                                quote! {
                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                        match #create_dot_zero_dot_zero {
                                            Some(value_1bbf74bc) => match #import_path::NotEmptyUniqueVec::try_new(
                                                vec![#current_ident_where_ucc #content_ts]
                                            ) {
                                                Ok(value_d82bbdbe) => Some(value_d82bbdbe),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => {
                                                        return None;
                                                    },
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("3d7ce854-db39-493c-bde3-a3dec5a8a9c3")
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
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("e9f9b021-e884-412e-bc02-7c1dafd35fdc")
                            }
                        }
                    }
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => none_ts.clone(),
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_ts(),
                }
            };
            let create_into_postgresql_json_type_option_vec_where_length_greater_than_ts = {
                let generate_ts = || {
                    let content_ts = {
                        let create_dot_zero_dot_zero = quote! {#create_sc.0.0};
                        let content_ts = {
                            let content_ts: &dyn quote::ToTokens = match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => &create_dot_zero_dot_zero,
                                NotNullOrNullable::Nullable => &quote! {value_68880991.0},
                            };
                            quote! {
                                ::LengthGreaterThan(
                                    where_filters::PostgresqlJsonTypeWhereLengthGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #value_sc: if let Ok(value_762dae1f) = postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
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
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => quote! {#ident_where_ucc #content_ts},
                            NotNullOrNullable::Nullable => {
                                let current_ident_where_ucc = SelfWhereUcc::from_tokens(&ident_not_null_ts);
                                quote! {
                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(match #create_dot_zero_dot_zero {
                                        Some(value_68880991) => match #import_path::NotEmptyUniqueVec::try_new(
                                            vec![#current_ident_where_ucc #content_ts]
                                        ) {
                                            Ok(value_cdc120a8) => Some(value_cdc120a8),
                                            Err(error) => match error {
                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => {
                                                    return None;
                                                },
                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("584f801e-e26d-486b-8814-758cd421bee4")
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
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("497359a5-49fb-4152-a9f4-5d1bbda2f926")
                            },
                        }
                    }
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => none_ts.clone(),
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_ts(),
                }
            };
            let generate_dot_checked_sub_one_ts = |content_ts: &dyn quote::ToTokens|quote!{#content_ts.checked_sub(1)};
            let generate_minus_one_is_finite_then_some_ts = |
                f32_or_f64: F32OrF64,
                content_ts: &dyn quote::ToTokens
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
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts = if matches!(&postgresql_json_type_pattern, PostgresqlJsonTypePattern::Standart) &&
                matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
            {
                let (
                    int_greater_than_one_less_ts,
                    float_32_greater_than_one_less_ts,
                    float_64_greater_than_one_less_ts,
                ) = {
                    let generate_greater_than_one_less_ts = |content_ts: &dyn quote::ToTokens|quote!{
                        let value_7aa498e8 = #content_ts?;
                        match #import_path::NotEmptyUniqueVec::try_new(
                            vec![
                                #import_path::SingleOrMultiple::Single(#ident_where_ucc::GreaterThan(
                                    where_filters::PostgresqlJsonTypeWhereGreaterThan {
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
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("11287f54-f840-4076-a16b-1a59a74e6dee"),
                            },
                        }
                    };
                    let create_dot_zero_dot_zero_ts = quote!{create.0.0};
                    (
                        generate_greater_than_one_less_ts(&generate_dot_checked_sub_one_ts(
                            &create_dot_zero_dot_zero_ts
                        )),
                        generate_greater_than_one_less_ts(&generate_minus_one_is_finite_then_some_ts(
                            F32OrF64::F32,
                            &create_dot_zero_dot_zero_ts
                        )),
                        generate_greater_than_one_less_ts(&generate_minus_one_is_finite_then_some_ts(
                            F32OrF64::F64,
                            &create_dot_zero_dot_zero_ts
                        )),
                    )
                };
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => int_greater_than_one_less_ts,
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => float_32_greater_than_one_less_ts,
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => float_64_greater_than_one_less_ts,
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::StdStringStringAsJsonbString |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts = if matches!(&postgresql_json_type_pattern, PostgresqlJsonTypePattern::Standart) &&
                matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
            {
                let (
                    between_one_less_and_one_more_int_ts,
                    between_one_less_and_one_more_float_ts
                ) = {
                    let generate_between_one_less_and_one_more_ts = |
                        less_ts: &dyn quote::ToTokens,
                        more_ts: &dyn quote::ToTokens
                    |quote!{
                        if let (Some(start), Some(end)) = (#less_ts, #more_ts) {
                            match where_filters::Between::try_new(
                                #ident_table_type_declaration_ucc::new(start),
                                #ident_table_type_declaration_ucc::new(end)
                            ) {
                                Ok(value_cdde02cc) => match postgresql_crud_common::NotEmptyUniqueVec::try_new(vec![
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_ucc::Between(
                                            where_filters::PostgresqlJsonTypeWhereBetween {
                                                logical_operator: postgresql_crud_common::LogicalOperator::Or,
                                                value: value_cdde02cc,
                                            }
                                        )
                                    )
                                ]) {
                                    Ok(value_41af48fb) => Some(value_41af48fb),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("5edabfcc-fcbe-419c-a2be-5bd7e961e6e0")
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
                            let generate_content_ts = |content_ts: &dyn quote::ToTokens|quote!{create.0.0.#content_ts(1)};
                            generate_between_one_less_and_one_more_ts(
                                &generate_content_ts(&quote!{checked_sub}),
                                &generate_content_ts(&quote!{checked_add})
                            )
                        },
                        {
                            let generate_content_ts = |content_ts: &dyn quote::ToTokens|quote!{{
                                let value = create.0.0 #content_ts 1.0;
                                value.is_finite().then_some(value)
                            }};
                            generate_between_one_less_and_one_more_ts(
                                &generate_content_ts(&quote!{-}),
                                &generate_content_ts(&quote!{+})
                            )
                        }
                    )
                };
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => between_one_less_and_one_more_int_ts,
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => between_one_less_and_one_more_float_ts,
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::StdStringStringAsJsonbString |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_ts.clone()
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts = if matches!(&postgresql_json_type_pattern, PostgresqlJsonTypePattern::Standart) &&
                matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
            {
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => {
                        //todo additional variants to test
                        quote!{
                            match #import_path::NotEmptyUniqueVec::try_new(vec![
                                #import_path::SingleOrMultiple::Single(
                                    #ident_where_ucc::In(
                                        where_filters::PostgresqlJsonTypeWhereIn {
                                            logical_operator: #import_path::LogicalOperator::Or,
                                            value: where_filters::PostgresqlJsonTypeNotEmptyUniqueVec::try_new(
                                                vec![#ident_table_type_declaration_ucc::new(#create_sc.0.0)]
                                            ).expect("2737c0ed-cf4a-4aba-b749-dc7c4e37ff2e")
                                        }
                                    ),
                                )
                            ]) {
                                Ok(value_1c4f89a4) => Some(value_1c4f89a4),
                                Err(error) => match error {
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("16ae359d-3869-421b-b37d-85b0b24835bd")
                                }
                            }
                        }
                    },
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::StdStringStringAsJsonbString |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_ts.clone()
                }
            }
            else {
                none_ts.clone()
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts = if matches!(&postgresql_json_type_pattern, PostgresqlJsonTypePattern::Standart) &&
                matches!(&not_null_or_nullable, NotNullOrNullable::NotNull)
            {
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                    PostgresqlJsonType::StdStringStringAsJsonbString => quote!{
                        match #import_path::NotEmptyUniqueVec::try_new(vec![
                            #import_path::SingleOrMultiple::Single(
                                #ident_where_ucc::RegularExpression(
                                    where_filters::PostgresqlJsonTypeWhereRegularExpression {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        regular_expression_case: where_filters::RegularExpressionCase::Sensitive,
                                        value: where_filters::RegexRegex(regex::Regex::new(&format!("^{}$", regex::escape(&#create_sc.0.0))).expect("3814ff38-0e4d-4173-bf0e-971372b888f6")),
                                    }
                                ),
                            )
                        ]) {
                            Ok(value_75ae8964) => Some(value_75ae8964),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("b9713787-7160-4a8b-b82c-72617d446184")
                            }
                        }
                    },
                }
            }
            else {
                none_ts.clone()
            };
            //todo add contains_el_greater_than for dimension 2,3,4
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    if matches!((&not_null_or_nullable, &dimension1_not_null_or_nullable), (
                        NotNullOrNullable::NotNull,
                        NotNullOrNullable::NotNull
                    )) {
                        let (
                            int_greater_than_one_less_ts,
                            float_32_greater_than_one_less_ts,
                            float_64_greater_than_one_less_ts,
                        ) = {
                            let generate_greater_than_one_less_ts = |content_ts: &dyn quote::ToTokens|quote!{
                                match #import_path::NotEmptyUniqueVec::try_new({
                                    let mut acc_f95ec4f2 = vec![];
                                    for el_ba78af60 in create.0.0 {
                                        let value_027d0d3a = #content_ts;
                                        match value_027d0d3a {
                                            Some(value_0cd93c25) => {
                                                acc_f95ec4f2.push(
                                                    #import_path::SingleOrMultiple::Single(
                                                        #ident_where_ucc::ContainsElGreaterThan(
                                                            where_filters::PostgresqlJsonTypeWhereContainsElGreaterThan {
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
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("47e44ecd-d1c9-4d2b-9d9e-4191cad34be9")
                                    }
                                }
                            };
                            let el_dot_zero_ts = quote!{el_ba78af60.0};
                            (
                                generate_greater_than_one_less_ts(&generate_dot_checked_sub_one_ts(
                                    &el_dot_zero_ts
                                )),
                                generate_greater_than_one_less_ts(&generate_minus_one_is_finite_then_some_ts(
                                    F32OrF64::F32,
                                    &el_dot_zero_ts
                                )),
                                generate_greater_than_one_less_ts(&generate_minus_one_is_finite_then_some_ts(
                                    F32OrF64::F64,
                                    &el_dot_zero_ts
                                )),
                            )
                        };
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => int_greater_than_one_less_ts,
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => float_32_greater_than_one_less_ts,
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => float_64_greater_than_one_less_ts,
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                            PostgresqlJsonType::StdStringStringAsJsonbString |
                            PostgresqlJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                        }
                    }
                    else {
                        none_ts.clone()
                    }
                },
                PostgresqlJsonTypePattern::Standart |
                PostgresqlJsonTypePattern::ArrayDimension2 {..} |
                PostgresqlJsonTypePattern::ArrayDimension3 {..} |
                PostgresqlJsonTypePattern::ArrayDimension4 {..} => none_ts.clone()
            };
            //todo add contains_el_regular_expression for dimension 2,3,4
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    if matches!((&not_null_or_nullable, &dimension1_not_null_or_nullable), (
                        NotNullOrNullable::NotNull,
                        NotNullOrNullable::NotNull
                    )) {
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                            PostgresqlJsonType::UuidUuidAsJsonbString => none_ts.clone(),
                            PostgresqlJsonType::StdStringStringAsJsonbString => quote!{
                                match #import_path::NotEmptyUniqueVec::try_new(create.0.0.into_iter().map(|el_590fca71| {
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_ucc::ContainsElRegularExpression(
                                            where_filters::PostgresqlJsonTypeWhereContainsElRegularExpression {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                regular_expression_case:
                                                    where_filters::RegularExpressionCase::Sensitive,
                                                value: where_filters::RegexRegex(
                                                    regex::Regex::new(&format!(
                                                        "^{}$",
                                                        regex::escape(&el_590fca71.0)
                                                    ))
                                                    .expect("7d01653a-e82f-4615-bbef-a8c899491f34"),
                                                ),
                                            },
                                        ),
                                    )
                                })
                                .collect()) {
                                    Ok(value_0363f494) => Some(value_0363f494),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("415a73d9-3665-4dde-9120-662a51626586")
                                    }
                                }
                            }
                        }
                    }
                    else {
                        none_ts.clone()
                    }
                },
                PostgresqlJsonTypePattern::Standart |
                PostgresqlJsonTypePattern::ArrayDimension2 {..} |
                PostgresqlJsonTypePattern::ArrayDimension3 {..} |
                PostgresqlJsonTypePattern::ArrayDimension4 {..} => none_ts.clone()
            };
            postgresql_crud_macros_common::generate_impl_postgresql_json_type_test_cases_for_ident_ts(
                &quote! {#[cfg(feature = "test-utils")]},
                &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                &ident_read_inner_ucc,
                &ident,
                &option_vec_create_ts,
                &read_only_ids_to_two_dimensional_vec_read_inner_ts,
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
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts,
                &create_into_postgresql_json_type_option_vec_where_length_equal_ts,
                &create_into_postgresql_json_type_option_vec_where_length_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts,
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
            #impl_postgresql_json_type_for_ident_ts
            #maybe_impl_postgresql_json_type_object_vec_el_id_for_ident_origin_ts
            #impl_postgresql_json_type_test_cases_for_ident_ts
        };
        (
            {
                let field_ident = format!("field_{index}").parse::<proc_macro2::TokenStream>().expect("f992f797-a4df-40d0-9984-3a3a3ad439d7");
                quote! {pub #field_ident: #ident,}.to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_json_types_config.postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_types,
        "postgresql_table_columns_using_postgresql_json_types",
        &{
            let fields_content_ts = fields_ts
                .into_iter()
                .map(|el_7ec253fa| el_7ec253fa.parse::<proc_macro2::TokenStream>().expect("1d8cd8e4-5f51-4aed-a626-79d759d86ebf"))
                .collect::<Vec<proc_macro2::TokenStream>>();
            quote! {
                pub struct PostgresqlTableColumnsUsingPostgresqlJsonTypes {
                    #(#fields_content_ts)*
                }
            }
        },
        &macros_helpers::FormatWithCargofmt::True
    );
    let generated = {
        let generate_postgresql_json_types_mod_sc = GeneratePostgresqlJsonTypesModSc;
        let content_ts = postgresql_json_type_array
            .into_iter()
            .map(|el_af9caefa| {
                el_af9caefa
                    .parse::<proc_macro2::TokenStream>()
                    .expect("84e21b40-b5a4-4f4c-86d3-8f6ecfbe1f6e")
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #generate_postgresql_json_types_mod_sc {
                #(#content_ts)*
            }
            pub use #generate_postgresql_json_types_mod_sc::*;
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_json_types_config
            .whole_content_write_into_generate_postgresql_json_types,
        "generate_postgresql_json_types",
        &generated,
        &macros_helpers::FormatWithCargofmt::True,
    );
    generated
}
